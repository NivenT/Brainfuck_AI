/*
fn find_closing_bracket(prog: &Vec<char>, pos: usize) -> Result<usize, &'static str> {
	let mut stack = 1;
	let mut pos = pos + 1;
	if pos >= prog.len() {return Err("[ without a closing ]");}
	while stack != 0 {
		match prog[pos] {
			'[' => {stack += 1},
			']' => {stack -= 1},
			_ => {}
		}
		pos += 1;
		if pos >= prog.len() {
			return Err("[ without a closing ]");
		}
	}
	return Ok(pos-1);
}

fn find_opening_bracket(prog: &Vec<char>, pos: usize) -> Result<usize, &'static str> {
	let mut stack = 1;
	if pos == 0 {return Err("] without an opening [");}
	let mut pos = pos;
	while stack != 0 {
		if pos == 0 {
			return Err("] without an opening [");
		}
		pos -= 1;
		match prog[pos] {
			'[' => {stack -= 1},
			']' => {stack += 1},
			_ => {}
		}
	}
	return Ok(pos);
}
*/

#[allow(dead_code)]
#[allow(non_camel_case_types)]
enum Token {
	//original 8 tokens
	PLUS,
	MINUS,
	FORWARD,
	BACKWARDS,
	OPEN_LOOP,
	CLOSE_LOOP,
	GET_CHAR,
	PUT_CHAR,
	//new tokens for extended language
	INT(i64), //stores integer literal in current cell
	GET_INT,  //prompts user to input an integer
	PUT_INT,  //prints value @ cell as an integer
	GET_MSG   //prompts user to input a string (uses |msg| cells)
}

#[allow(dead_code)]
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
	/* Original function for running BF code
	pub fn run(&mut self, prog: String) -> Result<i64, &str> {
		let mut table = &mut self.table;
		let mut dp = &mut self.dp;

		let chars: Vec<_> = prog.chars().collect();
		let mut pos = 0;

		while pos < chars.len() {
			match chars[pos] {
				'+' => {table[*dp] += 1},
				'-' => {table[*dp] -= 1},
				'>' => {*dp += 1},
				'<' => {*dp = if *dp == 0 {0} else {*dp - 1}},
				'[' => {
					if table[*dp] == 0 {
						match find_closing_bracket(&chars, pos) {
							Ok(i) => {pos = i},
							Err(msg) => {return Err(msg)}
						}
					}
				},
				']' => {
					match find_opening_bracket(&chars, pos) {
						Ok(i) => {pos = i-1},
						Err(msg) => {return Err(msg)}
					}
				},
				'.' => {
					print!("{}", (table[*dp]%256) as u8 as char);
					io::stdout().flush().ok().expect("")
				},
				_ => {}
			}
			pos += 1;
		}
		Ok(table[*dp])
	}
	*/
}