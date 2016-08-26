use time::{Duration, SteadyTime};
use rand::random;

fn has_matched_brackets(prog: &String) -> bool {
	prog.chars().fold(0, |accum, c| match c {'['=>accum+1,']'=>accum-1,_=>accum})==-1
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum Token {
	//original Brainfuck
	INC, 		//increments value @ cell				(+)
	DEC, 		//decrements value @ cell				(-)
	RIGHT, 		//increments data pointer				(>)
	LEFT, 		//decrements data pointer				(<)
	LOOP(Vec<Token>), //executes body while cell!=0		([..])
	GET, 		//reads value from input tape			(,)
	PUT_CHAR, 	//prints value @ cell as a char			(.)
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
	PUT_INT,  	//prints value @ cell as an integer   	(!)
	SAVE, 	  	//stores value @ cell in memory 		(@)
	LOAD,	  	//stores value from memory in cell    	(*)
}

#[derive(Clone)]
pub struct InputTape {
	stream: String
}

#[allow(dead_code)]
impl InputTape {
	pub fn new() -> InputTape {
		InputTape{stream: String::new()}
	}
	pub fn from_string(s: &str) -> InputTape {
		InputTape{stream: s.to_string()}
	}
	pub fn random(len: usize) -> InputTape {
		let mut stream = String::new();
	    for _ in 0..len {
	    	stream = stream + &random::<u8>().to_string();
	    }
	    InputTape{stream: stream}
	}

	fn read(&mut self) -> u8 {
		if self.stream.len() > 0 {
			self.stream.remove(0) as u8
		} else {
			0
		}
	}
}

#[derive(Clone)]
pub struct Interpreter {
	pub out_stream: 	String,
	pub in_stream: 		InputTape,

	mem: 				u8,
	table: 				Vec<u8>,
	dp: 				usize,		//Data pointer
	print_output:		bool,
	time_limit:			bool 
}

#[allow(dead_code)]
impl Interpreter {
	pub fn new() -> Interpreter {
	    Interpreter {
	    	out_stream: String::new(),
	    	in_stream: InputTape::new(),
	    	mem: 0,
	    	table: vec![0; 1000],
	    	dp:	0,
	    	print_output: true,
	    	time_limit:	false  
	    }
	}
	pub fn print(&self, p: bool) -> Interpreter {
		let mut ret = self.clone();
		ret.print_output = p;
		ret
	}
	pub fn limit(&self, p: bool) -> Interpreter {
		let mut ret = self.clone();
		ret.time_limit = p;
		ret
	}
	pub fn reset(&mut self) {
		self.out_stream = String::new();
		for i in 0..self.table.len() {
			self.table[i] = 0;
		}
		self.dp = 0;
		self.mem = 0;
	}
	pub fn run(&mut self, prog: Vec<Token>, input: &mut InputTape, start: &mut Option<SteadyTime>) -> Result<u8, &'static str> {
		if start.is_none() && self.time_limit {
			*start = Some(SteadyTime::now());
		}
		for tkn in prog {
			if self.time_limit {
				if SteadyTime::now() - start.unwrap() > Duration::milliseconds(2) {
					return Err("Computation took too long");
				}
			}

			match tkn {
				Token::INC => {
					self.table[self.dp] = self.table[self.dp].wrapping_add(1);
				}, Token::DEC => {
					self.table[self.dp] = self.table[self.dp].wrapping_sub(1);
				}, Token::RIGHT => {
					self.dp += 1;
				}, Token::LEFT => {
					self.dp = self.dp.saturating_sub(1);
				}, Token::GET => {
					self.table[self.dp] = input.read();
				}, Token::PUT_CHAR => {
					let c = self.table[self.dp] as char;
					self.out_stream = self.out_stream.clone() + &c.to_string();
					if self.print_output {print!("{}", c)}
				}, Token::PUT_INT => {
					self.out_stream = self.out_stream.clone() + &self.table[self.dp].to_string();
					if self.print_output {print!("{}", self.table[self.dp])}
				}, Token::LOOP(body) => {
					while self.table[self.dp] != 0 {
						match self.run(body.clone(), input, start) {
							Ok(_) => {},
							Err(msg) => {return Err(msg)}
						}
					}
				}, Token::SAVE => {
					self.mem = self.table[self.dp];
				}, Token::LOAD => {
					self.table[self.dp] = self.mem;
				}, Token::CONST0 => {
					self.table[self.dp] = 0x0*16;
				}, Token::CONST1 => {
					self.table[self.dp] = 0x1*16;
				}, Token::CONST2 => {
					self.table[self.dp] = 0x2*16;
				}, Token::CONST3 => {
					self.table[self.dp] = 0x3*16;
				}, Token::CONST4 => {
					self.table[self.dp] = 0x4*16;
				}, Token::CONST5 => {
					self.table[self.dp] = 0x5*16;
				}, Token::CONST6 => {
					self.table[self.dp] = 0x6*16;
				}, Token::CONST7 => {
					self.table[self.dp] = 0x7*16;
				}, Token::CONST8 => {
					self.table[self.dp] = 0x8*16;
				}, Token::CONST9 => {
					self.table[self.dp] = 0x9*16;
				}, Token::CONSTA => {
					self.table[self.dp] = 0xA*16;
				}, Token::CONSTB => {
					self.table[self.dp] = 0xB*16;
				}, Token::CONSTC => {
					self.table[self.dp] = 0xC*16;
				}, Token::CONSTD => {
					self.table[self.dp] = 0xD*16;
				}, Token::CONSTE => {
					self.table[self.dp] = 0xE*16;
				}, Token::CONSTF => {
					self.table[self.dp] = 0xF*16;
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
			',' => Some(Token::GET),
			'.' => Some(Token::PUT_CHAR),
			'!' => Some(Token::PUT_INT),
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