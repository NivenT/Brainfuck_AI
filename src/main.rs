mod interpreter;

use std::io;
use std::io::prelude::*;
use interpreter::*;

fn main() {
    let mut iptr = Interpreter::new();

    let mut prog = String::new();
    println!("Enter program:");
    match io::stdin().read_line(&mut prog) {
    	Err(why) => panic!("Could not reading program because {}", why),
    	_ => {}
    }
    match iptr.run(Interpreter::get_tokens(&mut prog)) {
    	Ok(res) => println!("\nProgram output: {}", res),
    	Err(msg) => println!("Error: {}", msg)
    }
}
