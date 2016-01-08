use time::{Duration, SteadyTime};
use std::io;
use std::io::prelude::*;

fn has_matched_brackets(prog: &String) -> bool {
	prog.chars().fold(0, |accum, c| match c {'['=>accum+1,']'=>accum-1,_=>accum})==-1
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum Token {
	//original Brainfuck
	INC, // increments value @ cell					(+)
	DEC, //	decrements value @ cell					(-)
	RIGHT, // increments data pointer				(>)
	LEFT, // decrements data pointer				(<)
	LOOP(Vec<Token>), // executes body while cell!=0([..])
	GET_CHAR, // prompts user to input a char		(,)
	PUT_CHAR, // prints value @ cell as a char		(.)
	//new tokens for extended language
	INT(i64), //stores integer literal in current cell
	GET_INT,  //prompts user to input an integer	(#)
	PUT_INT,  //prints value @ cell as an integer   (!)
	GET_MSG   //prompts user to input a string      (?)
}

#[allow(dead_code)]
impl Token {
	pub fn to_string(&self) -> String {
	    match *self {
	    	Token::INC => "+".to_string(),
	    	Token::DEC => "-".to_string(),
	    	Token::RIGHT => ">".to_string(),
	    	Token::LEFT => "<".to_string(),
	    	Token::GET_CHAR => ",".to_string(),
	    	Token::PUT_CHAR => ".".to_string(),
	    	Token::GET_INT => "#".to_string(),
	    	Token::PUT_INT => "!".to_string(),
	    	Token::GET_MSG => "?".to_string(),
	    	Token::INT(n) => n.to_string(),
	    	Token::LOOP(ref body) => {
	    		let mut s = "[".to_string();
	    		for tkn in body {
	    			s = s + &tkn.to_string();
	    		}
	    		s + "]"
	    	}
	    }
	}
}

pub struct Interpreter {
	pub stdout: String,
	table: Vec<i64>,
	dp: usize
}

#[allow(dead_code)]
impl Interpreter {
	pub fn new() -> Interpreter {
	    Interpreter{stdout: String::new(), table: vec![0; 1000], dp: 0}
	}
	pub fn reset(&mut self) {
		self.table = vec![0; self.table.len()];
		self.dp = 0;
	}
	pub fn run(&mut self, prog: Vec<Token>, print: bool, start: SteadyTime) -> Result<i64,String> {
		for tkn in prog {
			if SteadyTime::now() - start > Duration::milliseconds(10) {
				return Err("Computation took too long".to_string());
			}
			match tkn {
				Token::INC => {
					self.table[self.dp] += 1
				}, Token::DEC => {
					self.table[self.dp] -= 1
				}, Token::RIGHT => {
					self.dp += 1
				}, Token::LEFT => {
					if self.dp != 0 {self.dp -= 1}
				}, Token::GET_CHAR => {
					print!("Please enter a char: ");
					io::stdout().flush().ok().expect("");
					let mut c: [u8; 1] = [0];
					match io::stdin().read(&mut c) {
						Ok(_) => {self.table[self.dp] = c[0] as i64},
						Err(msg) => {return Err(format!("{}", msg))}
					}
				}, Token::PUT_CHAR => {
					let c = (self.table[self.dp]%256) as u8 as char;
					self.stdout = self.stdout.clone() + &c.to_string();
					if print {print!("{}", c)}
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
					self.stdout = self.stdout.clone() + &self.table[self.dp].to_string();
					if print {print!("{}", self.table[self.dp])}
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
						match self.run(body.clone(), print, start) {
							Ok(_) => {},
							Err(msg) => {return Err(msg)}
						}
					}
				}
			}
			if self.dp >= self.table.len() {
				self.table.extend(vec![0; 1000]);
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
					if prog.chars().next().unwrap().is_numeric() {
						num = num + &prog.remove(0).to_string();
					} else {break}
				}
				match num.parse::<i64>() {
					Ok(n) => Some(Token::INT(n)),
					Err(_) => None
				}
			},
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
}