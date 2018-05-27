#![allow(dead_code)]

use tiny_keccak::sha3_256;
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread::{self, JoinHandle};
use std::sync::{Mutex, Arc};
use std::sync::mpsc::{Sender, Receiver, channel, TryRecvError};
use std::ops::{Deref, DerefMut};
//use std::rc::{self, Rc};
//use std::string::ToString;
//use std::str::from_utf8;

static NUM_THREADS: u32 = 4;

#[derive(Clone, Copy)]
pub struct Block {
	pub hash: [u8; 32],
	prev_hash: [u8; 32],
	pub data: [char; 128],
	pub time_stamp: u64,
	pub nonce: u64
}

impl Block {
	pub fn new(data: [char; 128], prev_hash: [u8; 32]) -> Block {
		let time = SystemTime::now();
		let since_epoch = time.duration_since(UNIX_EPOCH).expect("Error with time");
		let time_stamp = since_epoch.as_secs() * 1000 + since_epoch.subsec_nanos() as u64 / 1_000_000;
		//println!("Prev Hash: {:?}", prev_hash);
		let mut b = Block {
			data,
			prev_hash,
			time_stamp,
			hash: [0; 32],
			nonce: 0
		};

		b.hash = b.calculate_hash();

		//println!("b.hash: ");
		//println!("{}", b.hash.iter().map(|var| var.to_string() + " ").collect::<String>());
		b

	}

	fn calc_and_set_hash(&mut self) {
		self.hash = self.calculate_hash()
	}

	pub fn calculate_hash(&self) -> [u8; 32] {

		let mut whole_string: String = self.prev_hash.iter().map(|var| var.to_string()).collect();

		//adding other stuff
		//println!("{}", self.nonce.to_string());
		whole_string = whole_string + &self.time_stamp.to_string() + &self.nonce.to_string() + &self.data.iter().collect::<String>();
		//println!("Word: {}", whole_string);
		sha3_256(whole_string.as_bytes())
	}

	pub fn mine_block(&mut self, difficulty: u64) {

		//let (tx, rx) = channel();
		//tx.send(self.nonce).expect("mine_block: unable to send first nonce message");
		let starting_nonce = Mutex::new(self.nonce);
		let starting_nonce_perthread_increase: u64 = 18446744073709551614 as u64 / (NUM_THREADS + 1) as u64; // could be better
		let (tx, rx) = channel();
		let nonce_found = Arc::new(Mutex::new(false));
		let difficulty = Arc::new(difficulty);

		for id in 0..NUM_THREADS {

			let starting_nonce_thread = *starting_nonce.lock().expect("unable to set starting_nonce_th (locking)").deref();
			let mut block_thread: Block = *self;
			let tx_thread = tx.clone();

			let nonce_found = Arc::clone(&nonce_found);
			let difficulty = *Arc::clone(&difficulty);

			thread::spawn(move || {
				block_thread.nonce = starting_nonce_thread;
				println!("{} thread started", id);
				for _ in 0..starting_nonce_perthread_increase {
					block_thread.calc_and_set_hash();

					//println!("{} Hahs: {}", block_thread.nonce, block_thread.hash.iter().map(|a| a.to_string() + " ").collect::<String>());
					let num_matching_digits = block_thread.hash.iter().take_while(|b| **b == 0).count() as u64;

					if num_matching_digits == difficulty {
						println!("Thread {} found 4 zeros. Nonce: {}, hash {:?}", id, block_thread.nonce, block_thread.hash);
						tx_thread.send(block_thread.nonce).unwrap();
						break;
					}
					if *nonce_found.lock().expect("unable to get nonce_found_th (locking)") { break; }
					block_thread.nonce += 1;
				}
			});

			*starting_nonce.lock().expect("starting nonce").deref_mut() += starting_nonce_perthread_increase;

		}

		//main thread won't have much to do
		thread::yield_now();
		self.nonce = rx.recv().expect("Unable to unpack received message");
		*nonce_found.lock().expect("unable to set nonce_found_th (locking)") = true;

		//don't really need join

		self.calc_and_set_hash();
	}
}