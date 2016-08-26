use interpreter::*;
use program::*;
use rand::{Rng,thread_rng,random};

#[allow(dead_code)]
pub fn levenshtein(s1: &String, s2: &String) -> usize {
	let (len1, len2) = (s1.chars().count(), s2.chars().count());
	let mut table = vec![vec![0; len2+1]; len1+1];
	table[0][0] = 0;
	for i in 1..len1+1 {
		table[i][0] = table[i-1][0] + s1.chars().skip(i-1).next().unwrap() as usize;
	}
	for j in 1..len2+1 {
		table[0][j] = table[0][j-1] + s2.chars().skip(j-1).next().unwrap() as usize;
	}
	for i in 1..len1+1 {
		for j in 1..len2+1 {
			let (c1, c2) = (s1.chars().skip(i-1).next().unwrap() as i32,
							s2.chars().skip(j-1).next().unwrap() as i32);
			table[i][j] = *[table[i-1][j-1]+if c1==c2 {0} else {(c1-c2).abs() as usize},
							table[i-1][j]+c1 as usize,
							table[i][j-1]+c2 as usize
							].into_iter().min().unwrap();
		}
	}
	table[len1][len2]
}

#[allow(dead_code)]
fn first_n(s: String, n: usize) -> String {
	if s.len() <= n {
		s
	} else {
		s.chars().take(n).collect()
	}
}

#[allow(dead_code)]
fn diff(a: u8, b: u8) -> u8 {
	if a > b {a-b} else {b-a}
}

pub struct GAlgo {
	pub generation: usize,
	pub max_fit: f64,
	pub min_fit: f64,

	pop: Vec<BFProgram>,
	mut_rate: f64,
	cross_rate: f64,
	target: String,
	percent_bred: f64,
	percent_kept: f64
}

impl GAlgo {
	pub fn new(size: usize, mut_rate: f64, cross_rate: f64, bred: f64, kept: f64, target: &str) -> GAlgo {
	    let mut pop = vec![];
	    for _ in 0..size {
	    	pop.push(BFProgram::new(thread_rng().gen_range(5,15)));
	    }
	    GAlgo {
	    	pop: pop, 
	    	mut_rate: mut_rate, 
	    	target: target.to_string(),
	    	cross_rate: cross_rate, 
	    	percent_bred: bred,
	    	percent_kept: kept,
	    	generation: 0, 
	    	max_fit: 0f64, 
	    	min_fit: 0f64
	    }
	}
	pub fn rand_prog(&self) -> BFProgram {
		self.pop[thread_rng().gen_range(0,self.pop.len())].clone()
	}
	pub fn get_fitness(&self, prog: &BFProgram) -> f64 {
		let mut iptr = Interpreter::new().print(false).limit(true);
		let mut prog = prog.code.clone();
		let mut input = InputTape::new();

		let _ = iptr.run(Interpreter::get_tokens(&mut prog), &mut input, &mut None);
		
		/* String fitness function */
		//iptr.stdout = first_n(iptr.stdout.clone(), self.target.len());
		//iptr.stdout.chars().zip(self.target.chars()).fold(0f64, |accum, (c1,c2)|
		//	accum + 256f64 - (c1 as i32 - c2 as i32).abs() as f64)
		let edit_dist = levenshtein(&iptr.out_stream, &self.target);
		1f64/(1f64 + edit_dist as f64)
	}
	pub fn step_pop(&mut self) -> Option<BFProgram> {
		let mut fitness = vec![];
		let mut tot_fit = 0f64;
		self.max_fit = 0f64;
		self.min_fit = 3f64;
		for i in 0..self.pop.len() {
			let fit = self.get_fitness(&self.pop[i]);

			if fit > self.max_fit {
				self.max_fit = fit
			} else if fit < self.min_fit {
				self.min_fit = fit
			}
			//if (fit - 256f64*self.target.len() as f64).abs() < 0.001 {
			if (fit - 1f64).abs() < 0.001 {
				return Some(self.pop[i].clone())
			} else {
				tot_fit += fit;
				fitness.push(fit)
			}
		}

		let q = 1f64 - (1f64-self.mut_rate).powf(1./NUM_MUTATIONS as f64); 
		let num_bred = (self.percent_bred*self.pop.len() as f64).floor() as usize;
		let mut new_pop = Vec::with_capacity(self.pop.len());
		while new_pop.len() < num_bred {
			let mom: &BFProgram;
			let dad: &BFProgram;

			let mut rand = thread_rng().gen_range(0f64, tot_fit);
			let mut index = 0;
			while rand > fitness[index] {
				rand -= fitness[index];
				index += 1;
			}
			mom = &self.pop[index];

			rand = thread_rng().gen_range(0f64, tot_fit);
			index = 0;
			while rand > fitness[index] {
				rand -= fitness[index];
				index += 1;
			}
			dad = &self.pop[index];

			if thread_rng().gen_range(0f64, 1f64) < self.cross_rate {
				let mut child = if random() {mom.cross(dad)} else {dad.cross(mom)};
				if thread_rng().gen_range(0f64, 1f64) < q {
					child.mutate_ins()
				} else if thread_rng().gen_range(0f64, 1f64) < q {
					child.mutate_del()
				} else if thread_rng().gen_range(0f64, 1f64) < q {
					child.mutate_sub()
				} else if thread_rng().gen_range(0f64, 1f64) < q {
					child.mutate_trn()
				} else if thread_rng().gen_range(0f64, 1f64) < q {
					child.mutate_shl()
				} else if thread_rng().gen_range(0f64, 1f64) < q {
					child.mutate_shr()
				}
				new_pop.push(child);
			}
		}
		let num_kept = (self.percent_kept*self.pop.len() as f64)
						.min((self.pop.len()-new_pop.len()) as f64)
						.floor() as usize;
		let mut n_best: Vec<_> = self.pop.clone()
								 	 .into_iter()
									 .enumerate()
									 .collect();
		n_best.sort_by(|&(i, _), &(j, _)|  fitness[i].partial_cmp(&fitness[j]).unwrap());
		new_pop.extend(n_best.into_iter()
						     .take(num_kept)
						     .map(|(_, a)| a)
						     .collect::<Vec<_>>());

		while new_pop.len() < self.pop.len() {
			new_pop.push(BFProgram::new(thread_rng().gen_range(5,15)));
		}

		self.pop = new_pop;
		self.generation += 1;
		None
	}
}