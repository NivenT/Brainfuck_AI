use std::fmt;
use rand::{Rng,thread_rng};

pub const NUM_MUTATIONS: usize = 6;

pub fn min(a: usize, b: usize) -> usize {
	if a > b {b} else {a}
}

fn random_char() -> &'static str {
	let choices = vec!["+", "-", ">", "<", "[", "]", ",", ".",
	                   "0", "1", "2", "3", "4", "5", "6", "7",
					   "8", "9", "A", "B", "C", "D", "E", "F",
					   "!", "@", "*"];
	choices[thread_rng().gen_range(0,choices.len())]
}

#[derive(Clone)]
pub struct BFProgram {
	pub code: String
}

impl fmt::Display for BFProgram {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.code)
	}
}

impl BFProgram {
	pub fn new(len: usize) -> BFProgram {
	    let mut code = String::new();
	    for _ in 0..len {
	    	code = code + random_char();
	    }
	    BFProgram{code: code}
	}
	pub fn mutate_ins(&mut self) {
		if self.code.len() == 0 {
			self.code = random_char().to_string();
		} else {
			let ins = thread_rng().gen_range(0,self.code.len());
		    self.code = self.code[0..ins].to_string()
		    			 + random_char() + &self.code[ins..];
		}
	}
	pub fn mutate_del(&mut self) {
		if self.code.len() != 0 {
			let del = thread_rng().gen_range(0, self.code.len());
		    self.code = self.code[0..del].to_string() 
		    			+ &self.code[del+1..];
		}
	}
	pub fn mutate_sub(&mut self) {
		if self.code.len() != 0 {
			let sub = thread_rng().gen_range(0,self.code.len());
		    self.code = self.code[0..sub].to_string()
		    			+ random_char() + &self.code[sub+1..];
		}
	}
	pub fn mutate_trn(&mut self) {
		if self.code.len() > 1 {
			let trn = thread_rng().gen_range(0,self.code.len()-1);
		    self.code = self.code[0..trn].to_string()
		    			+ &self.code[trn+1..trn+2] 
		    			+ &self.code[trn..trn+1]
		    			+ &self.code[trn+2..];
		}
	}
	pub fn mutate_shr(&mut self) {
		if self.code.len() > 1 {
			self.code = self.code.pop().unwrap().to_string() + &self.code[0..];
		}
	}
	pub fn mutate_shl(&mut self) {
		if self.code.len() > 1 {
			self.code = self.code[1..].to_string() + &self.code.remove(0).to_string();
		}
	}
	pub fn cross(&self, other: &BFProgram) -> BFProgram {
		let (mom, dad) = (self.code.clone(), other.code.clone());
		let pivot = if mom.len() == 0 || dad.len() == 0 {0} else {thread_rng().gen_range(0,min(mom.len(),dad.len()))};
		BFProgram{code: mom[..pivot].to_string() + &dad[pivot..]}
	}
}