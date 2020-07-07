pub mod deck;

pub use deck::{Card, Deck, Suit};

#[derive(Clone)]
pub struct Napoleon {
    pub bet: usize,
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
    },
}

pub enum BiddingEvent {
    NextBidder { player_id: usize },
    BiddingFinished { napoleon: Napoleon },
}

pub enum BiddingError {
    InvalidRange,
    NotCurrentPlayer,
    InvalidGameState,
    NoBids,
}

pub enum PostBiddingEvent {
    AlliesChosen { allies: Vec<usize> },
}

pub enum PostBiddingError {
    NotCurrentPlayer,
    InvalidGameState,
}

pub enum PlayingEvent {
    NextPlayer {
        player_id: usize,
    },
    RoundEnded {
        winner: usize,
        next_player: usize,
    },
    GameEnded {
        combined_napoleon_score: usize,
        napoleon: Napoleon,
        allies: Vec<usize>,
    },
}

pub enum PlayingError {
    InvalidGameState,
}

pub struct Game {
    deck: Deck,
    players: usize,
    hands: Vec<Deck>,
    score: Vec<usize>,
    state: GameState,
    settings: GameSettings,
}

#[derive(Clone)]
pub struct GameSettings {
    pub ally_count: usize,
}

impl Game {
    pub fn new(players: usize, settings: GameSettings) -> Game {
        let mut deck = Deck::new(1);
        deck.shuffle();

        let mut hands = Vec::new();
        for _ in 0..players {
            let mut hand = Deck::new_empty();

            for _ in 0..5 {
                hand.push(deck.pop().expect("Deck should have enough elements"));
            }

            hands.push(hand);
        }

        Game {
            deck,
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

    pub fn get_score(&self) -> &[usize] {
        &self.score
    }

    pub fn bid(
        &mut self,
        player_id: usize,
        bet: Option<usize>, // TODO: rename bet as bid
    ) -> Result<BiddingEvent, BiddingError> {
        if let GameState::Bidding {
            current_player,
            current_napoleon,
        } = &mut self.state
        {
            assert_eq!(
                *current_player, player_id,
                "{} tried to bet when it was {}'s turn",
                player_id, current_player
            );

            match bet {
                Some(bet) => {
                    assert!(
                        bet <= 5,
                        "Tried to bet {} which was higher than trick count ({})",
                        bet,
                        5
                    );

                    if let Some(napoleon) = current_napoleon {
                        assert!(
                            bet > napoleon.bet,
                            "Tried to bet lower than or equal to current"
                        );
                    }

                    *current_napoleon = Some(Napoleon { player_id, bet });
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
            // panic!("Tried to bet even though game state was not betting");
        }
    }

    // TODO: Error handling
    pub fn pick_allies(
        &mut self,
        player_id: usize,
        ally_cards: Vec<Card>,
        trump_suit: Suit,
    ) -> Result<PostBiddingEvent, PostBiddingError> {
        if let GameState::PostBidding { napoleon } = &self.state {
            assert_eq!(
                napoleon.player_id, player_id,
                "Non-napoleon tried to pick allies"
            );

            assert_eq!(
                ally_cards.len(),
                self.settings.ally_count,
                "Napoleon tried to pick incorrect number of allies"
            );

            let mut allies = Vec::new();

            // TODO: Napoleon can't pick themselves as an ally. Not an error just don't add
            // napoleon to allies vector
            'outer: for (id, hand) in self.hands.iter().enumerate() {
                for ally_card in &ally_cards {
                    if hand.contains(ally_card) {
                        allies.push(id);

                        continue 'outer;
                    }
                }
            }

            // TODO: napeolon must start game with a card from the trump suit
            self.state = GameState::Playing {
                napoleon: napoleon.clone(),
                allies: allies.clone(),
                trump_suit,
                current_player: napoleon.player_id,
                played_cards: Vec::with_capacity(self.players),
            };

            Ok(PostBiddingEvent::AlliesChosen { allies })
        } else {
            Err(PostBiddingError::InvalidGameState)
            //            panic!("Tried to pick allies when game state wasn't PostBidding");
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
            ..
        } = &mut self.state
        {
            assert_eq!(
                player_id, *current_player,
                "Player tried to play a card in another's turn"
            );
            assert!(
                self.hands[player_id].remove(&card).is_some(),
                "Player tried to play card that they did not have"
            );

            if let Some(first_card) = played_cards.first() {
                assert!(
                    card.suit == first_card.suit
                        || !self.hands[player_id].contains_suit(&first_card.suit)
                );
            }

            played_cards.push(card);

            if played_cards.len() == self.players {
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
                            .sum::<usize>();
                    return Ok(PlayingEvent::GameEnded {
                        combined_napoleon_score,
                        napoleon: napoleon.clone(),
                        allies: allies.clone(),
                    });
                } else {
                    return Ok(PlayingEvent::RoundEnded {
                        next_player: winner,
                        winner,
                    });
                }
            } else {
                *current_player = (*current_player + 1) % self.players;
                return Ok(PlayingEvent::NextPlayer {
                    player_id: *current_player,
                });
            }
        } else {
            //            panic!("Tried to play card when game state wasn't Playing");
            Err(PlayingError::InvalidGameState)
        }
    }
}
