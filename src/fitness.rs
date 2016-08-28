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

		let mut target = self.clone();
		let mut output = env.out_stream.clone();
		if output.len() > target.len() {
			for _ in 0..(output.len()-target.len()) {
				target = target + "\0";
			}
		} else {
			for _ in 0..(target.len()-output.len()) {
				output = output + "\0";
			}
		}

		let fit = target.chars()
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

pub struct Adder;

impl Fitness for Adder {
    fn fitness(&self, prog: &BFProgram) -> f64 {
    	let mut env = Interpreter::new().print(false).limit(true);
    	let tokens = Interpreter::get_tokens(&mut prog.code.clone());

    	let mut fit = 0f64;
    	for _ in 0..10 {
    		let mut input = InputTape::random(2);
    		let _ = env.run(tokens.clone(), &mut input.clone(), &mut None);

    		let target = input.read();
    		let target = target.wrapping_add(input.read());
    		let output = env.return_val();

    		let diff = if target > output {target - output} else {output - target};
    		fit += diff as f64;
    	}
    	return 1./(1. + fit);
    }
    fn max_fitness(&self) -> f64 {
    	1.
    }
    fn min_fitness(&self) -> f64 {
    	0.
    }
}