use interpreter::*;
use program::*;

pub trait Fitness {
	fn fitness(&self, &BFProgram) -> f64;
	fn max_fitness(&self) -> f64;
	fn min_fitness(&self) -> f64;
}

impl Fitness for String {
	fn fitness(&self, prog: &BFProgram) -> f64 {
		let mut env = Interpreter::new().print(false).limit(true);
		let tokens = Interpreter::get_tokens(&mut prog.code.clone());
		let _ = env.run(tokens, &mut InputTape::new(), &mut None);

		let mut output = env.out_stream.clone();
		for _ in 0..self.len() {
			output = output + "\0";
		}

		let fit = self.chars()
				  	  .zip(output.chars())
					  .map(|(a, b)| {
					  	let (c, d) = (a as u8, b as u8);
						if c > d {c - d} else {d - c}
				      })
					  .fold(0f64, |accum, x| accum + x as f64);
		return 1./(1. + fit);
	}
	fn max_fitness(&self) -> f64 {
		1.
	}
	fn min_fitness(&self) -> f64 {
		0.
	}
}