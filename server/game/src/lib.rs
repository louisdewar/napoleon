pub mod deck;

pub use deck::{Card, Deck, Suit};

use serde::Serialize;

#[derive(Clone)]
pub struct Napoleon {
    pub bid: u32,
    pub player_id: usize,
}

enum GameState {
    Bidding {
        current_player: usize,
        current_napoleon: Option<Napoleon>,
    },
    PostBidding {
        napoleon: Napoleon,
    },
    Playing {
        napoleon: Napoleon,
        allies: Vec<usize>,
        trump_suit: Suit,
        current_player: usize,
        played_cards: Vec<Card>,
        required_suit: Option<Suit>,
    },
}

pub enum BiddingEvent {
    NextBidder { player_id: usize },
    BiddingFinished { napoleon: Napoleon },
}

pub enum BiddingError {
    BidTooLow { min: u32 },
    BidTooHigh { max: u32 },
    NotCurrentPlayer { current_player: usize },
    InvalidGameState,
    NoBids,
}

pub enum PostBiddingEvent {
    AlliesChosen { allies: Vec<usize> },
}

pub enum PostBiddingError {
    NotCurrentPlayer { current_player: usize },
    IncorrectAllyCount { expected: usize, received: usize },
    InvalidGameState,
}

pub enum PlayingEvent {
    NextPlayer {
        player_id: usize,
        required_suit: Suit,
    },
    RoundEnded {
        winner: usize,
        next_player: usize,
    },
    GameEnded {
        combined_napoleon_score: u32,
        napoleon: Napoleon,
        allies: Vec<usize>,
    },
}

pub enum PlayingError {
    NotCurrentPlayer { current_player: usize },
    InvalidGameState,
    InvalidSuit,
    CardNotInHand,
}

pub struct Game {
    players: usize,
    hands: Vec<Deck>,
    score: Vec<u32>,
    state: GameState,
    settings: GameSettings,
}

#[derive(Clone, Serialize)]
pub struct GameSettings {
    pub ally_count: usize,
    pub hand_size: u32,
}

impl Game {
    pub fn new(players: usize, settings: GameSettings) -> Game {
        let mut deck = Deck::new(1);
        deck.shuffle();

        let mut hands = Vec::new();
        for _ in 0..players {
            let mut hand = Deck::new_empty();

            for _ in 0..settings.hand_size {
                hand.push(deck.pop().expect("Deck should have enough elements"));
            }

            hands.push(hand);
        }

        Game {
            players,
            hands,
            score: vec![0; players],
            state: GameState::Bidding {
                current_player: 0,
                current_napoleon: None,
            },
            settings,
        }
    }

    pub fn get_settings(&self) -> &GameSettings {
        &self.settings
    }

    pub fn get_hands(&self) -> &[Deck] {
        &self.hands
    }

    pub fn get_score(&self) -> &[u32] {
        &self.score
    }

    pub fn bid(
        &mut self,
        player_id: usize,
        bid: Option<u32>,
    ) -> Result<BiddingEvent, BiddingError> {
        if let GameState::Bidding {
            current_player,
            current_napoleon,
        } = &mut self.state
        {
            if *current_player != player_id {
                return Err(BiddingError::NotCurrentPlayer {
                    current_player: *current_player,
                });
            }

            match bid {
                Some(bid) => {
                    if bid > self.settings.hand_size {
                        return Err(BiddingError::BidTooHigh {
                            max: self.settings.hand_size,
                        });
                    }

                    if let Some(napoleon) = current_napoleon {
                        if bid <= napoleon.bid {
                            return Err(BiddingError::BidTooLow {
                                min: napoleon.bid + 1,
                            });
                        }
                    }

                    *current_napoleon = Some(Napoleon { player_id, bid });
                }
                None => {}
            }

            if player_id == self.players - 1 {
                if let Some(napoleon) = current_napoleon {
                    let napoleon = napoleon.clone();

                    self.state = GameState::PostBidding {
                        napoleon: napoleon.clone(),
                    };

                    return Ok(BiddingEvent::BiddingFinished { napoleon });
                } else {
                    return Err(BiddingError::NoBids);
                }
            }

            *current_player += 1;
            return Ok(BiddingEvent::NextBidder {
                player_id: *current_player,
            });
        } else {
            return Err(BiddingError::InvalidGameState);
        }
    }

    pub fn pick_allies(
        &mut self,
        player_id: usize,
        ally_cards: Vec<Card>,
        trump_suit: Suit,
    ) -> Result<PostBiddingEvent, PostBiddingError> {
        if let GameState::PostBidding { napoleon } = &self.state {
            if napoleon.player_id != player_id {
                return Err(PostBiddingError::NotCurrentPlayer {
                    current_player: napoleon.player_id,
                });
            }

            if ally_cards.len() != self.settings.ally_count {
                return Err(PostBiddingError::IncorrectAllyCount {
                    expected: self.settings.ally_count,
                    received: ally_cards.len(),
                });
            }

            let mut allies = Vec::new();

            'outer: for (id, hand) in self.hands.iter().enumerate() {
                // Napoleon can't pick themselves as an ally. Not an error just skip adding
                // napoleon to allies vector
                if id == napoleon.player_id {
                    continue 'outer;
                }

                for ally_card in &ally_cards {
                    if hand.contains(ally_card) {
                        allies.push(id);

                        continue 'outer;
                    }
                }
            }

            self.state = GameState::Playing {
                napoleon: napoleon.clone(),
                allies: allies.clone(),
                required_suit: Some(trump_suit.clone()),
                trump_suit,
                current_player: napoleon.player_id,
                played_cards: Vec::with_capacity(self.players),
            };

            Ok(PostBiddingEvent::AlliesChosen { allies })
        } else {
            Err(PostBiddingError::InvalidGameState)
        }
    }

    pub fn play_card(
        &mut self,
        player_id: usize,
        card: Card,
    ) -> Result<PlayingEvent, PlayingError> {
        if let GameState::Playing {
            allies,
            current_player,
            played_cards,
            trump_suit,
            napoleon,
            required_suit,
            ..
        } = &mut self.state
        {
            if player_id != *current_player {
                return Err(PlayingError::NotCurrentPlayer {
                    current_player: *current_player,
                });
            }

            if self.hands[player_id].remove(&card).is_some() {
                return Err(PlayingError::CardNotInHand);
            }

            // Ensure that the required suit is played if they have a card of that suit and if
            // there is in fact a required suit (there isn't for the first player of a round except
            // for the first round)
            if let Some(required_suit) = required_suit {
                if &card.suit != required_suit && self.hands[player_id].contains_suit(required_suit)
                {
                    return Err(PlayingError::InvalidSuit);
                }
            } else {
                // If there wasn't a required suit then all next cards should have the same suit as
                // the current (since it's the first of a round)
                *required_suit = Some(card.suit.clone());
            }

            played_cards.push(card);

            if played_cards.len() == self.players {
                // TODO: Even though deck supports multiple packs of cards, scoring does not.
                // It is unclear what to do when two players both have the exact same card that is
                // the highest number + trump suit.
                let (winner, _card) = played_cards
                    .iter()
                    .enumerate()
                    .filter(|(_, card)| &card.suit == trump_suit)
                    .max_by_key(|(_, card)| Into::<u8>::into(&card.number))
                    .unwrap_or_else(|| {
                        played_cards
                            .iter()
                            .enumerate()
                            .filter(|(_, card)| card.suit == played_cards[0].suit)
                            .max_by_key(|(_, card)| Into::<u8>::into(&card.number))
                            .expect("The first player's card trivially must exist as a possible solution")
                    });

                self.score[winner] += 1;

                played_cards.clear();

                if self.hands[0].len() == 0 {
                    let combined_napoleon_score = self.score[napoleon.player_id]
                        + self
                            .score
                            .iter()
                            .enumerate()
                            .filter_map(|(id, score)| {
                                if allies.contains(&id) {
                                    Some(score)
                                } else {
                                    None
                                }
                            })
                            .sum::<u32>();
                    return Ok(PlayingEvent::GameEnded {
                        combined_napoleon_score,
                        napoleon: napoleon.clone(),
                        allies: allies.clone(),
                    });
                } else {
                    // Now that the round has ended the next player has no required_suit
                    *required_suit = None;
                    return Ok(PlayingEvent::RoundEnded {
                        next_player: winner,
                        winner,
                    });
                }
            } else {
                *current_player = (*current_player + 1) % self.players;
                return Ok(PlayingEvent::NextPlayer {
                    player_id: *current_player,
                    // Can never fail since earlier if required_suit was None is was set to Some.
                    required_suit: required_suit.clone().unwrap(),
                });
            }
        } else {
            Err(PlayingError::InvalidGameState)
        }
    }
}
