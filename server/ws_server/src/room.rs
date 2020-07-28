use actix::prelude::*;
use game::*;
use std::collections::HashMap;

use slog::{debug, error, trace, warn, Logger};

mod message_handling;

#[derive(Message, Clone)]
#[rtype("()")]
pub enum RoomEvent {
    /// Sent to the player who joins a room (sent in game_server)
    JoinedRoom {
        key: String,
        host: usize,
        address: Addr<Room>,
        players: Vec<(String, usize)>,
    },
    /// Sent when a player joins the room
    PlayerJoined {
        player_id: usize,
        username: String,
    },
    GameStarted {
        player_order: Vec<usize>,
        game_settings: GameSettings,
    },
    PlayerHand {
        hand: Deck,
    },
    PlayerBid {
        player_id: usize,
        bid: Option<u32>,
    },
    NextBidder {
        player_id: usize,
    },
    BiddingOver {
        napoleon_id: usize,
        bid: u32,
    },
    NoBids,
    BecomeAlly,
    AlliesChosen {
        allies: Vec<Card>,
        trump_suit: Suit,
    },
    CardPlayed {
        player_id: usize,
        card: Card,
    },
    NextPlayer {
        player_id: usize,
        required_suit: Option<Suit>,
    },
    RoundOver {
        winner: usize,
    },
    GameOver {
        allies: Vec<usize>,
        napoleon_score_delta: i32,
        player_score_delta: i32,
        napoleon_bet: u32,
        combined_napoleon_score: u32,
    },
}

pub enum RoomState {
    Lobby {},
    /// id_map: Index of the vector is the player id from the game, and the value is the session_id
    /// of the player
    InGame {
        game: Game,
        id_map: Vec<usize>,
    },
}

struct Occupant {
    recipient: Recipient<RoomEvent>,
    username: String,
}

pub struct Room {
    players: HashMap<usize, Occupant>,
    state: RoomState,
    host: usize,
    logger: Logger,
}

impl Room {
    pub fn new(
        session_id: usize,
        username: String,
        session: Recipient<RoomEvent>,
        logger: Logger,
    ) -> Room {
        let mut players = HashMap::new();
        players.insert(
            session_id,
            Occupant {
                recipient: session,
                username,
            },
        );

        Room {
            players,
            state: RoomState::Lobby {},
            host: session_id,
            logger,
        }
    }

    fn send_recipient_event(recipient: &Recipient<RoomEvent>, event: RoomEvent) {
        let _ = recipient.do_send(event);
    }

    fn send_event(&self, session_id: &usize, event: RoomEvent) {
        if let Some(Occupant { recipient, .. }) = self.players.get(session_id) {
            Self::send_recipient_event(recipient, event);
        } else {
            debug!(self.logger, "Tried to send event to id which was no in the room"; "session_id" => session_id);
        }
    }

    fn broadcast(&self, event: RoomEvent) {
        for occupant in self.players.values() {
            Self::send_recipient_event(&occupant.recipient, event.clone());
        }
    }

    fn join(
        &mut self,
        session_id: usize,
        username: String,
        session: Recipient<RoomEvent>,
        room_key: String,
        room_addr: Addr<Room>,
    ) {
        self.broadcast(RoomEvent::PlayerJoined {
            player_id: session_id,
            username: username.clone(),
        });
        self.players.insert(
            session_id,
            Occupant {
                username,
                recipient: session.clone(),
            },
        );
        let _ = session.do_send(super::RoomEvent::JoinedRoom {
            address: room_addr,
            key: room_key,
            host: self.host,
            players: self
                .players
                .iter()
                .map(|(session_id, occ)| (occ.username.clone(), *session_id))
                .collect(),
        });
    }

    fn start_game(&mut self, session_id: usize, settings: GameSettings) {
        if let RoomState::Lobby {} = &self.state {
            if session_id != self.host {
                warn!(self.logger, "Non-host tried to start game"; "session_id" => session_id, "host_id" => self.host);
                return;
            }
            let id_map = self.players.keys().cloned().collect();

            self.new_game(settings, id_map);
        } else {
            warn!(
                self.logger,
                "Session tried to start game when the room state wasn't lobby";
                "session_id" => session_id
            );
        }
    }

    fn new_game(&mut self, game_settings: GameSettings, id_map: Vec<usize>) {
        if id_map.len() == 0 {
            error!(self.logger, "Tried to start a game with 0 people");
            return;
        }

        let game = Game::new(id_map.len(), game_settings.clone());
        self.broadcast(RoomEvent::GameStarted {
            player_order: id_map.clone(),
            game_settings,
        });

        for (hand, session_id) in game.get_hands().iter().zip(id_map.iter()) {
            self.send_event(session_id, RoomEvent::PlayerHand { hand: hand.clone() });
        }

        self.broadcast(RoomEvent::NextBidder {
            player_id: id_map[0],
        });

        trace!(self.logger, "New game started"; "players" => id_map.len());

        self.state = RoomState::InGame { game, id_map };
    }

    fn bid(&mut self, session_id: usize, bid: Option<u32>) {
        use BiddingError::*;
        use BiddingEvent::*;

        if let RoomState::InGame {
            ref mut game,
            ref id_map,
        } = self.state
        {
            if let Some(player_id) = id_map.iter().position(|id| session_id == *id) {
                match game.bid(player_id, bid) {
                    Ok(event) => {
                        self.broadcast(RoomEvent::PlayerBid {
                            player_id: session_id,
                            bid,
                        });
                        match event {
                            NextBidder { player_id } => {
                                self.broadcast(RoomEvent::NextBidder {
                                    player_id: id_map[player_id],
                                });
                            }
                            BiddingFinished { napoleon } => {
                                let napoleon_id = id_map[napoleon.player_id];
                                self.broadcast(RoomEvent::BiddingOver {
                                    bid: napoleon.bid,
                                    napoleon_id,
                                });
                            }
                        }
                    }
                    Err(error) => match error {
                        InvalidGameState => warn!(
                            self.logger,
                            "Session tried to bid when game state wasn't bidding";
                            "session_id" => session_id
                        ),
                        BidTooLow { min } => warn!(
                            self.logger,
                            "Session tried to bid below the minimum";
                            "session_id" => session_id,
                            "minimum" => min,
                            "bid" => bid
                        ),
                        BidTooHigh { max } => warn!(
                            self.logger,
                            "Session tried to bid above the maximum";
                            "session_id" => session_id,
                            "maximum" => max,
                            "bid" => bid
                        ),
                        NoBids => {
                            trace!(self.logger, "No bids so starting a new game");
                            let settings = game.get_settings().clone();
                            let id_map = id_map.clone();
                            self.broadcast(RoomEvent::NoBids);
                            self.new_game(settings, id_map);
                        }
                        NotCurrentPlayer { current_player } => warn!(
                            self.logger,
                            "Session tried to bid when they weren't the current bidder";
                            "session_id" => session_id,
                            "current_bidder" => current_player
                        ),
                    },
                }
            } else {
                warn!(
                    self.logger,
                    "Non-player tried to bid (was spectator)";
                    "session_id" => session_id
                )
            }
        } else {
            warn!(
                self.logger,
                "Session tried to bid when the room state wasn't in game";
                "session_id" => session_id
            );
        }
    }

    fn pick_allies(&mut self, session_id: usize, ally_cards: Vec<Card>, trump_suit: Suit) {
        use PostBiddingError::*;
        use PostBiddingEvent::*;

        if let RoomState::InGame {
            ref mut game,
            ref id_map,
        } = self.state
        {
            if let Some(player_id) = id_map.iter().position(|id| session_id == *id) {
                match game.pick_allies(player_id, ally_cards.clone(), trump_suit.clone()) {
                    Ok(event) => match event {
                        AlliesChosen { allies } => {
                            self.broadcast(RoomEvent::AlliesChosen {
                                allies: ally_cards,
                                trump_suit: trump_suit.clone(),
                            });

                            for ally in allies {
                                self.send_event(&id_map[ally], RoomEvent::BecomeAlly);
                            }

                            // Session_id here must be napoleon
                            self.broadcast(RoomEvent::NextPlayer {
                                player_id: session_id,
                                required_suit: Some(trump_suit),
                            });
                        }
                    },
                    Err(error) => match error {
                        NotCurrentPlayer { current_player } => warn!(
                            self.logger,
                            "Session tried to pick allies when they weren't the napoleon";
                            "session_id" => session_id,
                            "napoleon" => current_player,
                        ),
                        InvalidGameState => warn!(
                            self.logger,
                            "Session tried to pick allies when game state wasn't pick_allies";
                            "session_id" => session_id
                        ),
                        IncorrectAllyCount { expected, received } => warn!(
                            self.logger,
                            "Session picked an incorrect number of allies";
                            "session_id" => session_id,
                            "expected" => expected,
                            "received" => received,
                        ),
                    },
                }
            } else {
                warn!(
                    self.logger,
                    "Non player tried to pick allies (was spectator)";
                    "session_id" => session_id
                );
            }
        } else {
            warn!(
                self.logger,
                "Session tried to pick allies when the room state wasn't in game";
                "session_id" => session_id
            );
        }
    }

    fn play_card(&mut self, session_id: usize, card: Card, context: &mut Context<Self>) {
        use PlayingError::*;
        use PlayingEvent::*;

        if let RoomState::InGame {
            ref mut game,
            ref id_map,
        } = self.state
        {
            if let Some(player_id) = id_map.iter().position(|id| session_id == *id) {
                match game.play_card(player_id, card.clone()) {
                    Ok(event) => {
                        self.broadcast(RoomEvent::CardPlayed {
                            player_id: session_id,
                            card,
                        });
                        match event {
                            NextPlayer {
                                player_id,
                                required_suit,
                            } => {
                                self.broadcast(RoomEvent::NextPlayer {
                                    player_id: id_map[player_id],
                                    required_suit: Some(required_suit),
                                });
                            }
                            RoundEnded {
                                winner,
                                next_player,
                            } => {
                                let next_player_id = id_map[next_player];

                                self.broadcast(RoomEvent::RoundOver {
                                    winner: id_map[winner],
                                });
                                context.run_later(
                                    std::time::Duration::new(5, 0),
                                    move |room, _context| {
                                        room.broadcast(RoomEvent::NextPlayer {
                                            player_id: next_player_id,
                                            required_suit: None,
                                        });
                                    },
                                );
                            }
                            GameEnded {
                                combined_napoleon_score,
                                napoleon,
                                mut allies,
                                final_winner,
                            } => {
                                self.broadcast(RoomEvent::RoundOver {
                                    winner: final_winner,
                                });

                                // TODO: decide scoring
                                // TODO: implement room wide score
                                let (napoleon_score_delta, player_score_delta) =
                                    if napoleon.bid == combined_napoleon_score {
                                        (15, -10)
                                    } else {
                                        (-10, 15)
                                    };

                                for ally in &mut allies {
                                    *ally = id_map[*ally];
                                }

                                context.run_later(
                                    std::time::Duration::new(5, 0),
                                    move |room, _context| {
                                        room.broadcast(RoomEvent::GameOver {
                                            napoleon_score_delta,
                                            player_score_delta,
                                            allies,
                                            combined_napoleon_score,
                                            napoleon_bet: napoleon.bid,
                                        })
                                    },
                                );
                            }
                        }
                    }
                    Err(error) => match error {
                        InvalidGameState => warn!(
                            self.logger,
                            "Session tried to play card when game state wasn't in round";
                            "session_id" => session_id
                        ),
                        NotCurrentPlayer { current_player } => warn!(
                            self.logger,
                            "Session tried to play card when they weren't the current player";
                            "session_id" => session_id,
                            "player_id" => player_id,
                            "current_player" => current_player
                        ),
                        CardNotInHand => warn!(
                            self.logger,
                            "Session tried to play a card that they didn't have";
                            "session_id" => session_id,
                        ),
                        InvalidSuit => warn!(
                            self.logger,
                            "Session tried to play a card of the wrong suit";
                            "session_id" => session_id,
                        ),
                    },
                }
            } else {
                warn!(
                    self.logger,
                    "Non player tried to pick allies (was spectator)";
                    "session_id" => session_id
                );
            }
        } else {
            warn!(
                self.logger,
                "Session tried to play card when the room state wasn't in game";
                "session_id" => session_id
            );
        }
    }
}

impl Actor for Room {
    type Context = Context<Self>;
}
