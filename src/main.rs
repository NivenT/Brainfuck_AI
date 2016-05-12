extern crate rand;
extern crate time;

mod interpreter;
mod program;
mod algorithm;

use time::SteadyTime;
use interpreter::*;
use algorithm::*;

/* Produced programs:
 * hi: 
 	* -]362--[.-+[[[[+.70<
 		hi
 	* +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++[+++++++++++++++++++++++++++++++++++.----------------------------------]
 		hijklmnop...
 * hello:
 	* 1][[3[[3---1111181488[[[[118[1[1893[[+++.-[--.[[+++][+[+++..+++.4+++++++++++[[+++.+....-..+..4..+<...9299999[229[[[[[>[]>]]>[]>>]>]]>[>[>]][][[-[-[7]]777>---7>7.------[[-[[[[-[[---]-.[.--.-.]-.-.-][..]]][[[[[[]6]1]][]][]]]-]]]][]][[[[[[[[[[][[[[][[[[[[[+[[[-[[[[][-][-][][-][[[[][[][[[[[][<]77<7[[<>33<<77]77777]]7337>77]]]]777][>7]]]]]]]]]]]]]]]]][]][]]]]]][[]][]]]]]77]]]]]]]][]]]][]]][[>>-]]]-]-]]--]----]-][<]-[]]][]][]]]--+]--]]]]+-]>>]]]]]]]]>>[]>-]>]>]>>]]>]]>]>]>>]>]>>]]][[]][]]]]]]]]]]]+
 		hello...
*/

fn main() {
	let start = SteadyTime::now();
	let mut ga = GAlgo::new(200, 0.1, 0.8, 0.9, 0.08, "hey".to_string());
	loop {
		if ga.generation%50 == 0 {print!("Generation {}", ga.generation);}
		let ind = ga.rand_prog();
		let res = ga.step_pop();
		if ga.generation%50 == 1 {
			println!("(max fitness = {:.*}/min fitness = {:.*})\n\tRandom program: {}", 
				3, ga.max_fit, 3, ga.min_fit, ind);
		}
		match res {
			Some(mut prog) => {
				if ga.generation%50 != 0 {println!("Generation {}", ga.generation);}
					else {println!("");}
				println!("\tMost fit program: {}", prog);
				let mut iptr = Interpreter::new();
				//print!("\toutput:");
				let _ = iptr.run(Interpreter::get_tokens(&mut prog.code), true, false, SteadyTime::now());
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
