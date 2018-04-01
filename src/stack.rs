use card::{Card, CardFlyweight};
use screen::Screen;

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

	pub fn draw_to(&self, screen: &mut Screen, coord: (usize, usize)) {
		for (i, c) in self.cards.iter().enumerate() {
			c.draw_to(screen, (coord.0, coord.1-i));
		}
	}
}

