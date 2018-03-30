use std::rc::Rc;

pub #[derive(Debug)]
struct Database {
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

	#[inline] 
	pub fn get(&self, key: usize) -> &Card {
		assert!(self.cards.len() > key);
		& self.cards[key]
	}
}

#[derive(Debug)]
pub struct Card {
	// 
}


pub struct CardFlyweight {
	card: Rc<Card>,
	direction: u32, // [0,3]
}