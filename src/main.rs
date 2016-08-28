extern crate rand;
extern crate time;

mod interpreter;
mod program;
mod algorithm;
mod fitness;

use std::io;
use std::io::prelude::*;
use std::str::FromStr;

use time::SteadyTime;

use interpreter::*;
use algorithm::*;
use fitness::*;

fn prompt_for_val<T: FromStr>(prompt: &str) -> Result<T, T::Err> {
    print!("{}", prompt);

    let mut input = String::new();
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut input);

    input.lines().last().unwrap().trim().parse()
}

fn main() {
	let start = SteadyTime::now();
	let mut ga = GAlgo::new(200, 0.3, 0.8, 0.9, 0.08, Box::new(Adder));

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

			let mut input = InputTape::random(20);
			println!("\tInput: {}", input);

			let mut interpreter = Interpreter::new().print(false).limit(true);
			let _ = interpreter.run(Interpreter::get_tokens(&mut individual.code),
									&mut input,
									&mut None);
			println!("\tOutput: {}", interpreter.out_stream);
			println!("\tReturn Value: {}\n", interpreter.return_val());
		}

		match solution {
			Some(mut prog) => {
				if ga.generation%generations_per_print != 0 {
					println!("Generation {}", ga.generation);
				} else {
					println!("");
				}

				println!("\tMost fit program: {}\n", prog);

				let input = prompt_for_val::<String>("Enter input stream: ").unwrap();
				println!("Running program...");

				let mut iptr = Interpreter::new().limit(true);
				let _ = iptr.run(Interpreter::get_tokens(&mut prog.code), 
								   &mut InputTape::from_string(&input), 
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