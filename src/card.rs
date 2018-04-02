use std::rc::Rc;
use rand::{Rng, thread_rng};
use screen::Screen;


const CARD_SHORTER: usize = 10;
const CARD_LONGER: usize = 12;

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
				direction: Direction::Up,
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
	abilities: Vec<String>,
}

impl Card {
	pub fn new(name: String, abilities: Vec<String>) -> Card {
		Card {
			name: name,
			abilities: abilities,
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
	pub fn rot_cw(&mut self, block_rot: bool) {
		if self.direction != Direction::Right || !block_rot {
			self.direction = self.direction.rot_cw();
		}
	}

	pub fn rot_ccw(&mut self, block_rot: bool) {
		println!("did");
		if self.direction != Direction::Down || !block_rot {
			self.direction = self.direction.rot_ccw();
		}
	}

	pub fn set_vis(&mut self, vis: bool) {
		self.visible = vis;
	}

	pub fn visible(&self) -> bool {
		self.visible
	}

	pub fn draw_to(&self, db: &Database, screen: &mut Screen, coord: (usize, usize)) {
		let xo = if self.direction == Direction::Left {
			coord.0 + CARD_SHORTER - CARD_LONGER 
		} else {coord.0};
		let yo = if self.direction == Direction::Up {
			coord.1 + CARD_SHORTER - CARD_LONGER 
		} else {coord.1};
		let is_vert = self.direction.vertical();
		let hors = if is_vert {CARD_SHORTER} else {CARD_LONGER};
		let verts = if is_vert {CARD_LONGER} else {CARD_SHORTER};
		let filler = if self.visible {' '} else {
			use self::Direction::*;
			match self.direction {
				Up => '^',
				Down => 'v',
				Left => '<',
				Right => '>', 
			}
		};
		for y in 0..verts {
			for x in 0..hors {
				let coord_offset = (xo + x, yo + y);
				screen.put(filler, coord_offset);
			}
			screen.put('│', (xo,yo+y));
			screen.put('│', (xo+hors-1,yo+y));
		}
		for x in 0..hors {
			screen.put('━', (xo+x,yo));
			screen.put('━', (xo+x,yo+verts-1));
		}
		screen.put('┍',(xo,yo));
		screen.put('┑',(xo+hors-1,yo));
		screen.put('┕',(xo,yo+verts-1));
		screen.put('┙',(xo+hors-1,yo+verts-1));
		let real_card = db.get(self.card);
		if self.visible {
			use self::Direction::*;
			let mut pos = match self.direction {
				Up => 		(xo+1,		yo+1),
				Right => 	(xo+hors-2,	yo+1),
				Down => 	(xo+hors-2,	yo+verts-2),
				Left => 	(xo+1,		yo+verts-2),
			};
			let dir_rot_cw = self.direction.rot_cw();
			write_string(
				screen,
				&real_card.name[0..real_card.name.len().min(CARD_SHORTER-2)],
				dir_rot_cw,
				pos
			);
			for (i,a) in real_card.abilities.iter().enumerate() {
			// too many abilities may crash! avoid allowing too many to begin with

				match self.direction {
					Up => pos.1 += 1,
					Down => pos.1 -= 1,
					Left => pos.0 += 1,
					Right => pos.0 -= 1,
				};
				write_string(
					screen,
					& format!("{} {}", i+1, &a[0..a.len().min(CARD_SHORTER-4)]),
					dir_rot_cw,
					pos,
				);
			}
		}
	}
}

fn write_string(screen: &mut Screen, msg: &str, dir: Direction, mut pos: (usize, usize)) {
	for c in msg.chars() {
		screen.put(c, pos);
		use self::Direction::*;
		match dir {
			Up => 		{ pos.1 -= 1 },
			Down => 	{ pos.1 += 1 },
			Left => 	{ pos.0 -= 1 },
			Right => 	{ pos.0 += 1 },
		}
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

	pub fn rot_ccw(self) -> Self {
		use self::Direction::*;
		match self {
			Right => Up,
			Left => Down,
			Up => Left,
			Down => Right,
		}
	}

	pub fn rot_cw(self) -> Self {
		use self::Direction::*;
		match self {
			Right => Down,
			Left => Up,
			Up => Right,
			Down => Left,
		}
	}
}

