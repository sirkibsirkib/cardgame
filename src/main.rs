extern crate rand;

use std::io::{stdin, stdout, Write};

mod card;
mod card_type;
mod stack;
mod combat;
mod screen;

use screen::Screen;

use stack::Stack;
use card::{Database, Card};
use rand::{Rng, thread_rng};

const abs: [&'static str; 4] = [
	"^:-,~",
	"<:+",
	"v:+",
	">:-,Ee-",
];

const MAX_ABILITIES: usize = 4;


fn cards() -> Database {
	let mut rng = thread_rng();
	let mut db = Database::new();
	for s in vec![
		"slave", "plague", "cow", "hat", "be", "zoop", "wow", "bow"
	].iter() {
		let mut abilties = vec![];
		while abilties.len() < MAX_ABILITIES && (rng.gen::<bool>() || rng.gen::<bool>()) {
			let i = rng.gen::<usize>() % abs.len();
			abilties.push(abs[i].to_owned());
		} 
		let c = Card::new(s.to_string(), abilties);
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
    let db = cards();
    let mut screen = Screen::new('.');

    let mut stacks = vec![stack(&db), stack(&db), stack(&db)];
    // println!("{:#?}", & stacks);
    
    let mut s = String::new();
	loop {
		screen.clear(' ');
		stacks[0].draw_to(&db, &mut screen, (10,14));
	    stacks[1].draw_to(&db, &mut screen, (25,14));
	    stacks[2].draw_to(&db, &mut screen, (40,14));
	    screen.print();
	    s.clear();
		stdin().read_line(&mut s);
		let _ = stdout().flush();
		if let Some('\n') = s.chars().next_back() {
			s.pop();
		}
		if let Some('\r') = s.chars().next_back() {
			s.pop();
		}
		let parsed = s.split(" ")
		.filter(|x| !x.is_empty())
		.collect::<Vec<_>>();
		let mut worked = true;
		if let Some(Ok(index)) = parsed.get(0).map(|x| x.parse::<usize>()) {
			println!("!!{:?}", index);
			match parsed.get(1) {
				Some(&"cw") => {
					stacks[index].top_mut().map(|x| x.rot_cw(false));
				},
				Some(&"ccw") => {
					stacks[index].top_mut().map(|x| x.rot_ccw(false));
				},
				Some(&"vis") => {
					stacks[index].top_mut().map(|x| x.set_vis(true));
				},

				Some(&"!vis") => {
					stacks[index].top_mut().map(|x| x.set_vis(false));
				},
				Some(&"l") => {
					let dest = index - 1;
					if index > 0
					&& index < stacks.len()
					&& !stacks[dest].is_empty() {
						if let Some(c) = stacks[index].pop() {
							stacks[dest].push(c);
						
						} else { worked = false; }
					} else { worked = false; }
				}
				Some(&"r") => {
					let dest = index + 1;
					if index < stacks.len() - 1
					&& !stacks[dest].is_empty() {
						if let Some(c) = stacks[index].pop() {
							stacks[dest].push(c);
						} else { worked = false; } 
					} else { worked = false; }
				}
				x => { worked = false; },
			}
		}
		if !worked {
			println!("this didnt work: {:?}, fam.", s);
		}
	}
}