use super::Room;
use crate::{game_server::PlayerJoined, WebsocketMessage};
use actix::prelude::*;

use game::*;

impl Room {
    fn handle_message(&mut self, ws_message: WebsocketMessage) -> Result<(), String> {
        let session_id = ws_message.session_id;
        let content = ws_message.content;
        let mut message = content.chars().peekable();

        let first_char = if let Some(first_char) = message.next() {
            first_char
        } else {
            todo!("log error and return");
        };

        match first_char {
            's' => self.start_game(session_id, game::GameSettings { ally_count: 1 }),
            'b' => {
                let bid: Option<usize> = if content.len() > 1 {
                    Some(
                        content[1..]
                            .parse()
                            .map_err(|_| format!("Couldn't parse bid"))?,
                    )
                } else {
                    None
                };

                self.bid(session_id, bid);
            }
            'a' => {
                let trump_suit = Suit::from_char(
                    message
                        .next()
                        .ok_or_else(|| format!("pick allies had no trump suit"))?,
                )
                .map_err(|_| format!("invalid trump suit char"))?;
                let mut ally_cards = Vec::new();

                while let (Some(','), Some(n), Some(s)) =
                    (message.next(), message.next(), message.next())
                {
                    let card = Card::from_chars(s, n)
                        .map_err(|_| format!("invalid card format in pick allies"))?;
                    ally_cards.push(card);
                }

                self.pick_allies(session_id, ally_cards, trump_suit);
            }
            'p' => {
                let n = message.next().ok_or_else(|| "no card number in play card".to_string())?;
                let s = message.next().ok_or_else(|| "no card suit in play card".to_string())?;
                
                let card = Card::from_chars(s, n).map_err(|_| format!("invalid card format in play card"))?;
                self.play_card(session_id, card);
            }
            _ => {}
        }

        Ok(())
    }
}

impl Handler<WebsocketMessage> for Room {
    type Result = ();

    fn handle(&mut self, ws_message: WebsocketMessage, _context: &mut Self::Context) {
        if let Err(error) = self.handle_message(ws_message) {
            println!("ERROR: {}", error);
        }
    }
}

impl Handler<PlayerJoined> for Room {
    type Result = ();

    fn handle(&mut self, msg: PlayerJoined, _context: &mut Self::Context) {
        self.join(msg.session_id, msg.username, msg.recipient.clone());
    }
}
