#[derive(Clone, Debug, PartialEq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

impl Suit {
    pub fn to_char(&self) -> char {
        use Suit::*;
        match &self {
            Hearts => 'H',
            Diamonds => 'D',
            Spades => 'S',
            Clubs => 'C',
        }
    }

    pub fn from_char(c: char) -> Result<Suit, ()> {
        use Suit::*;

        Ok(match c {
            'H' => Hearts,
            'D' => Diamonds,
            'S' => Spades,
            'C' => Clubs,
            _ => return Err(()),
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Number {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<&Number> for u8 {
    fn from(num: &Number) -> u8 {
        use Number::*;

        match num {
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
            Seven => 7,
            Eight => 8,
            Nine => 9,
            Ten => 10,
            Jack => 11,
            Queen => 12,
            King => 13,
            Ace => 14,
        }
    }
}

impl Number {
    pub fn from_char(c: char) -> Result<Number, ()> {
        use Number::*;

        Ok(match c {
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'T' => Ten,
            'J' => Jack,
            'Q' => Queen,
            'K' => King,
            'A' => Ace,
            _ => return Err(()),
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub number: Number,
}

impl Card {
    pub fn new(suit: Suit, number: Number) -> Card {
        Card { suit, number }
    }

    pub fn from_chars(s: char, n: char) -> Result<Card, ()> {
        let suit = Suit::from_char(s)?;
        let number = Number::from_char(n)?;

        Ok(Card { suit, number })
    }
}

#[derive(Clone, Debug)]
pub struct Deck {
    inner: Vec<Card>,
}

impl Deck {
    pub fn new(pack_count: usize) -> Deck {
        let mut deck = Self::new_empty();

        for _ in 0..pack_count {
            deck.join(&mut Self::new_full());
        }

        deck
    }

    pub fn new_full() -> Deck {
        use Number::*;
        use Suit::*;

        let mut deck = Self::new_empty();

        for suit in &[Hearts, Diamonds, Spades, Clubs] {
            for number in &[
                Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
            ] {
                deck.inner.push(Card::new(suit.clone(), number.clone()));
            }
        }

        deck
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn new_empty() -> Deck {
        Deck { inner: Vec::new() }
    }

    fn join(&mut self, other: &mut Deck) {
        self.inner.append(&mut other.inner);
    }

    pub fn shuffle(&mut self) {
        use rand::prelude::*;
        let mut rng = thread_rng();
        self.inner.shuffle(&mut rng);
    }

    pub fn push(&mut self, card: Card) {
        self.inner.push(card);
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.inner.pop()
    }

    pub fn contains(&self, card: &Card) -> bool {
        self.inner.contains(card)
    }

    pub fn contains_suit(&self, suit: &Suit) -> bool {
        self.inner.iter().find(|card| &card.suit == suit).is_some()
    }

    pub fn remove(&mut self, card: &Card) -> Option<Card> {
        let index = self.inner.iter().position(|c| c == card)?;
        Some(self.inner.remove(index))
    }

    pub fn into_iter(self) -> impl Iterator<Item = Card> {
        self.inner.into_iter()
    }
}

impl std::fmt::Display for Suit {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        use Suit::*;

        match self {
            Hearts => write!(fmt, "♥"),
            Diamonds => write!(fmt, "♦"),
            Spades => write!(fmt, "♠"),
            Clubs => write!(fmt, "♣"),
        }
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        use Number::*;

        match self {
            Two => write!(fmt, "2"),
            Three => write!(fmt, "3"),
            Four => write!(fmt, "4"),
            Five => write!(fmt, "5"),
            Six => write!(fmt, "6"),
            Seven => write!(fmt, "7"),
            Eight => write!(fmt, "8"),
            Nine => write!(fmt, "9"),
            Ten => write!(fmt, "T"),
            Jack => write!(fmt, "J"),
            Queen => write!(fmt, "Q"),
            King => write!(fmt, "K"),
            Ace => write!(fmt, "A"),
        }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}{}", self.number, self.suit)
    }
}

impl std::fmt::Display for Deck {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "[")?;

        let mut cards = self.inner.iter();

        if let Some(card) = cards.next() {
            write!(fmt, "{}", card)?;
        }

        for card in cards {
            write!(fmt, ", {}", card)?;
        }

        write!(fmt, "]")
    }
}
