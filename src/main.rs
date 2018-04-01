extern crate rand;

mod card;
mod card_type;
mod stack;
mod combat;
mod screen;

use screen::Screen;

use stack::Stack;
use card::{Database, Card};
use rand::{Rng, thread_rng};

fn cards() -> Database {
	let mut db = Database::new();
	for s in vec!["a", "b", "c", "d", "e", "f", "g", "h"].iter() {
		let c = Card::new(s.to_string());
		db.create(c);
	}
	db
}


fn stack(db: & Database) -> Stack {
	let mut stack = Stack::new();
	let mut rng = thread_rng();
	for i in 0..5 {
		stack.push(db.get_random(&mut rng).unwrap());
	}
	stack
}

fn main() {
    let cards = cards();
    let mut screen = Screen::new('-');

    let mut stacks = vec![stack(&cards), stack(&cards), stack(&cards)];
    // println!("{:#?}", & stacks);
    stacks[0].draw_to(&mut screen, (21,7));
    stacks[1].draw_to(&mut screen, (41,7));
    stacks[2].draw_to(&mut screen, (61,7));
    screen.print();
}
