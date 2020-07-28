use game::*;

fn str_to_card(s: &str) -> Card {
    let mut chars = s.chars();
    let n = chars.next().unwrap();
    let s = chars.next().unwrap();
    Card::from_chars(s, n).unwrap()
}

#[test]
/// A non-specific test for testing the happy path of a game
fn it_plays_the_game() {
    let hands: Vec<Deck> = vec![vec!["8C", "AS"], vec!["4C", "AC"]]
        .into_iter()
        .map(|hand| hand.into_iter().map(str_to_card).collect())
        .collect();

    let mut game = Game::new_with_hands(
        2,
        GameSettings {
            ally_count: 0,
            hand_size: 5,
        },
        hands.clone(),
    );

    assert_eq!(hands.as_slice(), game.get_hands());

    assert_eq!(
        game.bid(0, None),
        Ok(BiddingEvent::NextBidder { player_id: 1 })
    );

    assert_eq!(
        game.bid(1, Some(2)),
        Ok(BiddingEvent::BiddingFinished {
            napoleon: Napoleon {
                player_id: 1,
                bid: 2
            }
        })
    );

    assert_eq!(
        game.pick_allies(1, Vec::new(), Suit::Clubs),
        Ok(PostBiddingEvent::AlliesChosen { allies: Vec::new() })
    );

    assert_eq!(
        game.play_card(1, str_to_card("4C")),
        Ok(PlayingEvent::NextPlayer {
            player_id: 0,
            required_suit: Suit::Clubs
        })
    );

    assert_eq!(
        game.play_card(0, str_to_card("8C")),
        Ok(PlayingEvent::RoundEnded {
            winner: 0,
            next_player: 0
        })
    );

    assert_eq!(
        game.play_card(0, str_to_card("AS")),
        Ok(PlayingEvent::NextPlayer {
            player_id: 1,
            required_suit: Suit::Spades
        })
    );

    assert_eq!(
        game.play_card(1, str_to_card("AC")),
        Ok(PlayingEvent::GameEnded {
            final_winner: 1,
            combined_napoleon_score: 1,
            napoleon: Napoleon {
                player_id: 1,
                bid: 2
            },
            allies: Vec::new(),
        })
    );
}
