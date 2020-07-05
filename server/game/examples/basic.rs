use game::deck::Deck;

fn main() {
    let mut deck = Deck::new(2);
    deck.shuffle();

    println!("There are {} cards, they are: {}", deck.len(), deck);
}
