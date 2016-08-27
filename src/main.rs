extern crate rand;
extern crate time;

mod interpreter;
mod program;
mod algorithm;
mod fitness;

use time::SteadyTime;

use interpreter::*;
use algorithm::*;

fn main() {
	let start = SteadyTime::now();
	let mut ga = GAlgo::new(200, 0.1, 0.8, 0.9, 0.08, Box::new("looplooploop".to_string()));

	let generations_per_print = 100;
	loop {
		if ga.generation%generations_per_print == 0 {
			print!("Generation {} ", ga.generation);
		}

		let mut individual = ga.rand_prog();
		let solution = ga.step_pop();

		if ga.generation%generations_per_print == 1 {
			println!("(max fitness = {:.3} | min fitness = {:.3})", ga.max_fit, ga.min_fit);
			println!("\tRandom program: {}", individual);

			let mut interpreter = Interpreter::new().print(false).limit(true);
			let _ = interpreter.run(Interpreter::get_tokens(&mut individual.code),
									&mut InputTape::new(),
									&mut None);
			println!("\tOutput: {}\n", interpreter.out_stream);
		}

		match solution {
			Some(mut prog) => {
				if ga.generation%generations_per_print != 0 {
					println!("Generation {}", ga.generation);
				} else {
					println!("");
				}

				println!("\tMost fit program: {}", prog);
				let mut iptr = Interpreter::new().limit(true);
				let _ = iptr.run(Interpreter::get_tokens(&mut prog.code), 
								   &mut InputTape::new(), 
								   &mut None);
				println!("\n\tReturn value: {}", iptr.return_val());

				let durr = SteadyTime::now() - start;
				println!("\nTook {} hours, {} minutes, and {} seconds",
					durr.num_hours(), durr.num_minutes()%60, durr.num_seconds()%60);
				break
			},
			None => {}
		}
	}
}

#[cfg(test)]
mod test {
	use interpreter::*;

	#[test]
	fn sum_program() {
		let mut iptr = Interpreter::new();

		let mut prog = ",!>3-----.,!>4---.0<<[->>+<<]>[->+<]>!1------.".to_string();
		let mut input = InputTape::from_string("foo"); //102, 111, 111

		let ret = iptr.run(Interpreter::get_tokens(&mut prog), &mut input, &mut None);

		assert_eq!(ret, Ok(10));
		assert_eq!(iptr.out_stream, "102+111=213\n".to_string());
	}
}