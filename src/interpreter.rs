use std::io;
use std::io::prelude::*;

fn isnum(c: char) -> bool {
    match c {
    	'0' ... '9' => true,
    	_ => false
    }
}


#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum Token {
	//original Brainfuck
	INC,
	DEC,
	RIGHT,
	LEFT,
	LOOP(Vec<Token>),
	GET_CHAR,
	PUT_CHAR,
	//new tokens for extended language
	INT(i64), //stores integer literal in current cell
	GET_INT,  //prompts user to input an integer     (#)
	PUT_INT,  //prints value @ cell as an integer    (!)
	GET_MSG   //prompts user to input a string       (?)
}

pub struct Interpreter {
	table: Vec<i64>,
	dp: usize
}

#[allow(dead_code)]
impl Interpreter {
	pub fn new() -> Interpreter {
	    Interpreter{table: vec![0; 1000], dp: 0}
	}
	pub fn reset(&mut self) {
		for i in 0..self.table.len() {
			self.table[i] = 0;
		}
		self.dp = 0;
	}
	pub fn run(&mut self, prog: Vec<Token>) -> Result<i64,String> {
		for tkn in prog {
			match tkn {
				Token::INC => {
					self.table[self.dp] += 1
				}, Token::DEC => {
					self.table[self.dp] -= 1
				}, Token::RIGHT => {
					self.dp += 1
				}, Token::LEFT => {
					self.dp -= 1
				}, Token::GET_CHAR => {
					print!("Please enter a char: ");
					io::stdout().flush().ok().expect("");
					let mut c: [u8; 1] = [0];
					match io::stdin().read(&mut c) {
						Ok(_) => {self.table[self.dp] = c[0] as i64},
						Err(msg) => {return Err(format!("{}", msg))}
					}
				}, Token::PUT_CHAR => {
					print!("{}", (self.table[self.dp]%256) as u8 as char)
				}, Token::GET_INT => {
					print!("Please enter a number: ");
					io::stdout().flush().ok().expect("");
					let mut s = String::new();
					io::stdin().read_line(&mut s).ok().expect("");
					match s.trim().parse::<i64>() {
						Ok(num) => {self.table[self.dp] = num},
						Err(msg) => {return Err(format!("{}", msg))}
					}
				}, Token::PUT_INT => {
					print!("{}", self.table[self.dp]);
				}, Token::GET_MSG => {
					print!("Please enter a string: ");
					io::stdout().flush().ok().expect("");
					let mut s = String::new();
					io::stdin().read_line(&mut s).ok().expect("");
					for c in s.chars() {
						self.table[self.dp] = c as i64;
						self.dp += 1;
					}
					self.dp -= s.chars().count();
				}, Token::INT(n) => {
					self.table[self.dp] = n
				}, Token::LOOP(body) => {
					while self.table[self.dp] != 0 {
						match self.run(body.clone()) {
							Ok(_) => {},
							Err(msg) => {return Err(msg)}
						}
					}
				}
			}
		}
		Ok(self.table[self.dp])
	}
	pub fn tokenize(prog: &mut String) -> Option<Token> {
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
			n @ '0' ... '9' => {
				let mut num = n.to_string();
				while prog.len() > 0 {
					if isnum(prog.chars().next().unwrap()) {
						num = num + &prog.remove(0).to_string();
					} else {
						break
					}
				}
				Some(Token::INT(num.parse::<i64>().unwrap()))
			},
			'[' => {
				let mut body = vec![];
				while !prog.starts_with(']') {
					if prog.len() == 0 {
						return None;
					}
					match Interpreter::tokenize(prog) {
						Some(tkn) => {body.push(tkn)},
						None => {}
					}
				}
				prog.remove(0); //get rid of ']'
				Some(Token::LOOP(body))
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
}