use game::deck::Deck;

fn main() {
    let mut deck = Deck::new(1);
    deck.shuffle();

    println!("There are {} cards", deck.len());
}
