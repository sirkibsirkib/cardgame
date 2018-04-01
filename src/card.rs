use std::rc::Rc;
use rand::{Rng, thread_rng};
use screen::Screen;


const CARD_SHORTER: usize = 5;
const CARD_LONGER: usize = 8;

#[derive(Debug)]
pub struct Database {
	cards: Vec<Card>,
}

impl Database {
	pub fn new() -> Self {
		Database{ cards: vec![] } 
	}

	pub fn create(&mut self, card: Card) -> usize {
		self.cards.push(card);
		self.cards.len()
	}

	pub fn get_random<R: Rng>(&self, rng: &mut R) -> Option<CardFlyweight> {
		if self.cards.is_empty() {
			None
		} else {
			let which: usize = rng.gen::<usize>() % self.cards.len();
			Some(CardFlyweight{
				visible: false,
				card: which,
				direction: Direction::Right,
			})
		}
	}

	#[inline] 
	pub fn get(&self, key: usize) -> &Card {
		assert!(self.cards.len() > key);
		& self.cards[key]
	}
}

#[derive(Debug)]
pub struct Card {
	name: String,
}

impl Card {
	pub fn new(name: String) -> Card {
		Card {
			name: name,
		}
	}
}

#[derive(Debug)]
pub struct CardFlyweight {
	visible: bool,
	card: usize,
	direction: Direction,
}

impl CardFlyweight {
	pub fn draw_to(&self, screen: &mut Screen, coord: (usize, usize)) {
		let vert = self.direction.vertical();
		for y in 0..(if vert {CARD_LONGER} else {CARD_SHORTER}) {
			for x in 0..(if vert {CARD_SHORTER} else {CARD_LONGER}) {
				let coord_offset = (coord.0 + x, coord.1 + y);
				screen.put('*', coord_offset);
			}
		}
	}
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
	Right, Up, Left, Down,
}

impl Direction {
	pub fn vertical(self) -> bool {
		use self::Direction::*;
		match self {
			Right => false,
			Left => false,
			Up => true,
			Down => true,
		}
	}
}