use time::{Duration, SteadyTime};
use rand::random;
use std::io;
use std::io::prelude::*;

fn has_matched_brackets(prog: &String) -> bool {
	prog.chars().fold(0, |accum, c| match c {'['=>accum+1,']'=>accum-1,_=>accum})==-1
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum Token {
	//original Brainfuck
	INC, // increments value @ cell						(+)
	DEC, //	decrements value @ cell						(-)
	RIGHT, // increments data pointer					(>)
	LEFT, // decrements data pointer					(<)
	LOOP(Vec<Token>), // executes body while cell!=0	([..])
	GET_CHAR, // prompts user to input a char			(,)
	PUT_CHAR, // prints value @ cell as a char			(.)
	//new tokens for extended language
	CONST0, 	//stores 0 at current cell  			(0)
	CONST1, 	//stores 16 at current cell  			(1)
	CONST2, 	//stores 32 at current cell  			(2)
	CONST3, 	//stores 48 at current cell  			(3)
	CONST4, 	//stores 64 at current cell  			(4)
	CONST5, 	//stores 80 at current cell  			(5)
	CONST6, 	//stores 96 at current cell  			(6)
	CONST7, 	//stores 112 at current cell  			(7)
	CONST8, 	//stores 128 at current cell  			(8)
	CONST9, 	//stores 144 at current cell  			(9)
	CONSTA, 	//stores 160 at current cell  			(A)
	CONSTB, 	//stores 176 at current cell  			(B)
	CONSTC, 	//stores 192 at current cell  			(C)
	CONSTD, 	//stores 208 at current cell  			(D)
	CONSTE, 	//stores 224 at current cell  			(E)
	CONSTF, 	//stores 240 at current cell  			(F)
	GET_INT,  	//prompts user to input an integer		(#)
	PUT_INT,  	//prints value @ cell as an integer   	(!)
	GET_MSG,  	//prompts user to input a string      	(?)
	SAVE, 	  	//stores value @ cell in memory 		(@)
	LOAD,	  	//stores value from memory in cell    	(*)
}

#[derive(Clone,Copy)]
pub enum Input {
	CHAR(char),
	INT(u8)
}

pub struct Interpreter {
	pub stdout: String,
	pub stdin: Vec<Input>,

	mem: u8,
	table: Vec<u8>,
	dp: usize
}

#[allow(dead_code)]
impl Interpreter {
	pub fn new() -> Interpreter {
	    Interpreter{stdout: String::new(), stdin: vec![], table: vec![0; 1000], dp: 0, mem: 0}
	}
	pub fn reset(&mut self) {
		self.table = vec![0; self.table.len()];
		self.dp = 0;
	}
	pub fn run(&mut self, prog: Vec<Token>, print: bool, algo: bool, start: SteadyTime) -> Result<u8,String> {
		for tkn in prog {
			if SteadyTime::now() - start > Duration::milliseconds(2) && algo {
				return Err("Computation took too long".to_string());
			}
			match tkn {
				Token::INC => {
					if self.table[self.dp] < 255 {self.table[self.dp] += 1} else {self.table[self.dp] = 0}
				}, Token::DEC => {
					if self.table[self.dp] > 0 {self.table[self.dp] -= 1} 
				}, Token::RIGHT => {
					self.dp += 1
				}, Token::LEFT => {
					if self.dp != 0 {self.dp -= 1}
				}, Token::GET_CHAR => {
					let mut c: [u8; 1] = [0];
					if algo {
						c[0] = random();
						self.table[self.dp] = c[0];
					} else {
						print!("Please enter a char: ");
						io::stdout().flush().ok().expect("");
						match io::stdin().read(&mut c) {
							Ok(_) => {self.table[self.dp] = c[0]},
							Err(msg) => {return Err(format!("{}", msg))}
						}
					}
					self.stdin.push(Input::CHAR(c[0] as char))
				}, Token::PUT_CHAR => {
					let c = self.table[self.dp] as char;
					self.stdout = self.stdout.clone() + &c.to_string();
					if print {print!("{}", c)}
				}, Token::GET_INT => {
					if algo {
						self.table[self.dp] = random();
					} else {
						print!("Please enter a number: ");
						io::stdout().flush().ok().expect("");
						let mut s = String::new();
						io::stdin().read_line(&mut s).ok().expect("");
						match s.trim().parse::<u8>() {
							Ok(num) => {self.table[self.dp] = num},
							Err(msg) => {return Err(format!("{}", msg))}
						}
					}
					self.stdin.push(Input::INT(self.table[self.dp]))
				}, Token::PUT_INT => {
					self.stdout = self.stdout.clone() + &self.table[self.dp].to_string();
					if print {print!("{}", self.table[self.dp])}
				}, Token::GET_MSG => {
					if !algo {
						print!("Please enter a string: ");
						io::stdout().flush().ok().expect("");
						let mut s = String::new();
						io::stdin().read_line(&mut s).ok().expect("");
						for c in s.chars() {
							self.table[self.dp] = c as u8;
							self.dp += 1;
						}
						self.dp -= s.chars().count();
					}
				}, Token::LOOP(body) => {
					while self.table[self.dp] != 0 {
						match self.run(body.clone(), print, algo, start) {
							Ok(_) => {},
							Err(msg) => {return Err(msg)}
						}
					}
				}, Token::SAVE => {
					self.mem = self.table[self.dp];
				}, Token::LOAD => {
					self.table[self.dp] = self.mem;
				}, Token::CONST0 => {
					self.table[self.dp] = 0x0*15;
				}, Token::CONST1 => {
					self.table[self.dp] = 0x1*15;
				}, Token::CONST2 => {
					self.table[self.dp] = 0x2*15;
				}, Token::CONST3 => {
					self.table[self.dp] = 0x3*15;
				}, Token::CONST4 => {
					self.table[self.dp] = 0x4*15;
				}, Token::CONST5 => {
					self.table[self.dp] = 0x5*15;
				}, Token::CONST6 => {
					self.table[self.dp] = 0x6*15;
				}, Token::CONST7 => {
					self.table[self.dp] = 0x7*15;
				}, Token::CONST8 => {
					self.table[self.dp] = 0x8*15;
				}, Token::CONST9 => {
					self.table[self.dp] = 0x9*15;
				}, Token::CONSTA => {
					self.table[self.dp] = 0xA*15;
				}, Token::CONSTB => {
					self.table[self.dp] = 0xB*15;
				}, Token::CONSTC => {
					self.table[self.dp] = 0xC*15;
				}, Token::CONSTD => {
					self.table[self.dp] = 0xD*15;
				}, Token::CONSTE => {
					self.table[self.dp] = 0xE*15;
				}, Token::CONSTF => {
					self.table[self.dp] = 0xF*15;
				}
			}
			if self.dp >= self.table.len() {
				self.table.extend(vec![0; 1000]);
			}
		}
		Ok(self.table[self.dp])
	}
	fn tokenize(prog: &mut String) -> Option<Token> {
		match prog.remove(0) {
			'+' => Some(Token::INC),
			'-' => Some(Token::DEC),
			'>' => Some(Token::RIGHT),
			'<' => Some(Token::LEFT),
			',' => Some(Token::GET_CHAR),
			'.' => Some(Token::PUT_CHAR),
			'#' => Some(Token::GET_INT),
			'!' => Some(Token::PUT_INT),
			'?' => Some(Token::GET_MSG),
			'@' => Some(Token::SAVE),
			'*' => Some(Token::LOAD),
			'0' => Some(Token::CONST0),
			'1' => Some(Token::CONST1),
			'2' => Some(Token::CONST2),
			'3' => Some(Token::CONST3),
			'4' => Some(Token::CONST4),
			'5' => Some(Token::CONST5),
			'6' => Some(Token::CONST6),
			'7' => Some(Token::CONST7),
			'8' => Some(Token::CONST8),
			'9' => Some(Token::CONST9),
			'A' => Some(Token::CONSTA),
			'B' => Some(Token::CONSTB),
			'C' => Some(Token::CONSTC),
			'D' => Some(Token::CONSTD),
			'E' => Some(Token::CONSTE),
			'F' => Some(Token::CONSTF),
			'[' if has_matched_brackets(&prog) => {
				let mut body = vec![];
				while !prog.starts_with(']') && !prog.is_empty() {
					match Interpreter::tokenize(prog) {
						Some(tkn) => {body.push(tkn)},
						None => {}
					}
				}
				prog.remove(0); // get rid of ']'
				if body.is_empty() {None} else {Some(Token::LOOP(body))}
			}
			_ => None
		}
	}
	pub fn get_tokens(prog: &mut String) -> Vec<Token> {
		let mut res = vec![];
		while prog.len() > 0 {
			match Interpreter::tokenize(prog) {
				Some(tkn) => {res.push(tkn)},
				None => {}
			}
		}
		return res;
	}
	pub fn return_val(&self) -> u8 {
		self.table[self.dp]
	}
}