use super::Room;
use crate::{game_server::PlayerJoined, WebsocketMessage};
use actix::prelude::*;

use game::*;

use slog::warn;

impl Handler<WebsocketMessage> for Room {
    type Result = ();

    fn handle(&mut self, ws_message: WebsocketMessage, context: &mut Self::Context) {
        let session_id = ws_message.session_id;
        let content = ws_message.content;
        let mut message = content.chars().peekable();

        let first_char = if let Some(first_char) = message.next() {
            first_char
        } else {
            warn!(self.logger, "Session sent empty message"; "session_id" => session_id);
            return;
        };

        match first_char {
            's' => self.start_game(
                session_id,
                game::GameSettings {
                    ally_count: 1,
                    hand_size: 5,
                },
            ),
            'b' => {
                let bid: Option<u32> = if content.len() > 1 {
                    if let Ok(bid) = content[1..].parse() {
                        Some(bid)
                    } else {
                        warn!(self.logger, "Couldn't parse bid from session"; "session_id" => session_id, "bid" => &content[1..]);
                        return;
                    }
                } else {
                    None
                };

                self.bid(session_id, bid);
            }
            'a' => {
                let trump_suit = if let Some(c) = message.next() {
                    if let Ok(suit) = Suit::from_char(c) {
                        suit
                    } else {
                        warn!(self.logger, "Invalid suit character for trump suit"; "session_id" => session_id, "suit_char" => c);
                        return;
                    }
                } else {
                    warn!(self.logger, "Pick allies message had no trump suit"; "session_id" => session_id);
                    return;
                };

                let mut ally_cards = Vec::new();

                while let (Some(','), Some(n), Some(s)) =
                    (message.next(), message.next(), message.next())
                {
                    let card = if let Ok(card) = Card::from_chars(s, n) {
                        card
                    } else {
                        warn!(self.logger, "Couldn't parse card from chars in pick allies"; "session_id" => session_id, "suit_char" => s, "number_char" => n);
                        return;
                    };
                    ally_cards.push(card);
                }

                self.pick_allies(session_id, ally_cards, trump_suit);
            }
            'p' => {
                let n = if let Some(n) = message.next() {
                    n
                } else {
                    warn!(self.logger, "No number in play card"; "session_id" => session_id);
                    return;
                };

                let s = if let Some(s) = message.next() {
                    s
                } else {
                    warn!(self.logger, "No suit in play card"; "session_id" => session_id);
                    return;
                };

                let card = if let Ok(card) = Card::from_chars(s, n) {
                    card
                } else {
                    warn!(self.logger, "Couldn't parse card from chars in play card"; "session_id" => session_id, "suit_char" => s, "number_char" => n);
                    return;
                };
                self.play_card(session_id, card, context);
            }
            _ => {}
        }   
    }
}

impl Handler<PlayerJoined> for Room {
    type Result = ();

    fn handle(&mut self, msg: PlayerJoined, _context: &mut Self::Context) {
        self.join(
            msg.session_id,
            msg.username,
            msg.recipient.clone(),
            msg.room_key,
            msg.room_addr,
        );
    }
}
