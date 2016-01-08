use interpreter::*;
use program::*;
use rand::{Rng,thread_rng,random};
use time::SteadyTime;

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

fn first_n(s: String, n: usize) ->  String {
	if s.len() <= n {
		s
	} else {
		s.chars().take(n).collect()
	}
}

pub struct GAlgo {
	pub generation: usize,
	pub max_fit: f64,
	pub min_fit: f64,

	pop: Vec<BFProgram>,
	mut_rate: f64,
	cross_rate: f64,
	target: String
}

impl GAlgo {
	pub fn new(size: usize, mut_rate: f64, cross_rate: f64, target: String) -> GAlgo {
	    let mut pop = vec![];
	    for _ in 0..size {
	    	pop.push(BFProgram::new(thread_rng().gen_range(5,15)));
	    }
	    GAlgo{pop: pop, mut_rate: mut_rate, target: target,
	    		cross_rate: cross_rate, generation: 0,
	    		max_fit: 0f64, min_fit: 0f64}
	}
	pub fn rand_prog(&self) -> BFProgram {
		self.pop[thread_rng().gen_range(0,self.pop.len())].clone()
	}
	pub fn get_fitness(&self, prog: &BFProgram) -> f64 {
		let mut iptr = Interpreter::new();
		let mut prog = prog.code.clone();
		let _ = iptr.run(Interpreter::get_tokens(&mut prog), false, SteadyTime::now());
		iptr.stdout = first_n(iptr.stdout.clone(), self.target.len());
		//iptr.stdout.chars().zip(self.target.chars()).fold(0f64, |accum, (c1,c2)|
		//	accum + 256f64 - (c1 as i32 - c2 as i32).abs() as f64)
		let edit_dist = levenshtein(&iptr.stdout, &self.target);
		1f64/(1f64 + edit_dist as f64)
	}
	pub fn step_pop(&mut self) -> Option<BFProgram> {
		let mut fitness = vec![];
		let mut tot_fit = 0f64;
		self.max_fit = 0f64;
		self.min_fit = 2f64;
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
		let mut new_pop = vec![];
		while new_pop.len() < self.pop.len() {
			let mut mom = BFProgram::new(0);
			let mut dad = BFProgram::new(0);

			let mut rand = thread_rng().gen_range(0f64,tot_fit);
			for i in 0.. {
				if rand > fitness[i] {
					rand -= fitness[i];
				} else {
					mom = self.pop[i].clone();
					break
				}
			}

			rand = thread_rng().gen_range(0f64,tot_fit);
			for i in 0.. {
				if rand > fitness[i] {
					rand -= fitness[i];
				} else {
					dad = self.pop[i].clone();
					break
				}
			}

			if thread_rng().gen_range(0f64,1f64) < self.cross_rate {
				let mut child = if random() {mom.cross(&mut dad)} else {dad.cross(&mut mom)};
				if thread_rng().gen_range(0f64,1f64) < q {
					child.mutate_ins()
				} else if thread_rng().gen_range(0f64,1f64) < q {
					child.mutate_del()
				} else if thread_rng().gen_range(0f64,1f64) < q {
					child.mutate_sub()
				} else if thread_rng().gen_range(0f64,1f64) < q {
					child.mutate_trn()
				} else if thread_rng().gen_range(0f64,1f64) < q {
					child.mutate_shl()
				} else if thread_rng().gen_range(0f64,1f64) < q {
					child.mutate_shr()
				}
				new_pop.push(child);
			}
		}
		self.pop = new_pop;
		self.generation += 1;
		None
	}
}