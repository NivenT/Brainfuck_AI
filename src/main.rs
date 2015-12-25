mod interpreter;

use std::io;
use std::io::prelude::*;
use interpreter::*;

#[allow(dead_code)]
fn main() {
    let mut iptr = Interpreter::new();
    iptr.reset(); // so warnings stay away

    let mut prog = String::new();
    println!("Enter program:");
    match io::stdin().read_line(&mut prog) {
    	Err(why) => panic!("Could not reading program because {}", why),
    	_ => {}
    }
    println!("{:?}", Interpreter::get_tokens(&mut prog));
    /*
    match iptr.run(prog) {
    	Ok(val) => {println!("\nOutput: {}", val)},
    	Err(why) => {println!("\nError: {}", why)}
    }
    println!("");
    */
}
