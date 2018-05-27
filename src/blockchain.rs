use block::Block;

pub struct Blockchain {
	pub chain: Vec<Block>,
	pub prev_hash: [u8; 32]
}

impl Blockchain {
	pub fn new() -> Blockchain {
		let mut chain = Vec::<Block>::with_capacity(50);
		chain.push(Block::new(['A'; 128], [0; 32]));

		Blockchain {
			chain,
			prev_hash: [0; 32]
		}
	}

	pub fn add_block(&mut self, data: [char; 128]) {
		self.chain.push(Block::new(data, self.prev_hash));
	}

	pub fn is_valid(&self) -> bool {
		let current = self.chain.last().expect("is_chain_valid: unable to get current block");
		let previous = self.chain.get(self.chain.len() -1).expect("is_chain_valid: unable to get previous block");

		if current.hash != current.calculate_hash() {
			println!("Current hash invalid");
			return false;
		}
		
		if previous.hash != previous.calculate_hash() {
			println!("Previous hash invalid");
			return false;
		}

		true
	}
}