use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
	cards: Vec<String>
}

impl Deck {
	fn new() -> Self {
		// List of suits
		let suits: [&str; 4] = [
			"Spades"
			,"Diamonds"
			,"Clubs"
			,"Hearts"
		];

		// List of values
		let values: [&str; 13] = [
			"Ace"
			,"Two"
			,"Three"
			,"Four"
			,"Five"
			,"Six"
			,"Seven"
			,"Eight"
			,"Nine"
			,"Ten"
			,"Jack"
			,"Queen"
			,"King"
		];

		let mut cards: Vec<String> = vec![];

		for suit in suits {
			for value in values {
				let card: String = format!("{} of {}", value, suit);
				cards.push(card);
			}
		}
		Deck{ cards }
	}

	fn shuffle(&mut self) {
		let mut rng = thread_rng();
		self.cards.shuffle(&mut rng);
	}

	fn deal(&mut self, num_cards: usize) -> Vec<String> {
		self.cards.split_off(
			self.cards.len() - num_cards
		)
	}
}

fn main() {
	let mut deck = Deck::new();

	deck.shuffle();

	// need to add error handling
	let cards = deck.deal(3);

	println!("Here's your deck: {:#?}", deck);
	println!("Here's your hand: {:#?}", cards);
}