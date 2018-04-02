use card::{Card, CardFlyweight};
use screen::Screen;
use card::Database;

#[derive(Debug)]
pub struct Stack {
	cards: Vec<CardFlyweight>,
}

impl Stack {
	pub fn new() -> Stack {
		Stack { cards: vec![] }
	}

	pub fn push(&mut self, card: CardFlyweight) {
		self.cards.push(card);
	}

	pub fn pop(&mut self) -> Option<CardFlyweight> {
		self.cards.pop()
	}

	pub fn draw_to(&self, db: &Database, screen: &mut Screen, coord: (usize, usize)) {
		for (i, c) in self.cards.iter().enumerate() {
			c.draw_to(db, screen, (coord.0, coord.1-i));
		}
	}

	pub fn top_mut(&mut self) -> Option<&mut CardFlyweight> {
		let x = self.cards.len() - 1;
		println!("c: {:?}", x);
		self.cards.get_mut(x)
	}

	pub fn is_empty(&self) -> bool {
		self.cards.is_empty()
	}
}

