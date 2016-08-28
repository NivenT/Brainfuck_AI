use rand::{Rng, thread_rng, random};

use program::*;
use fitness::*;

pub struct GAlgo {
	pub generation: 	usize,
	pub max_fit: 		f64,
	pub min_fit: 		f64,

	fit_func: 			Box<Fitness>,
	pop: 				Vec<BFProgram>,
	mut_rate: 			f64,
	cross_rate: 		f64,
	percent_bred: 		f64,
	percent_kept: 		f64
}

impl GAlgo {
	pub fn new(size: usize, mut_rate: f64, cross_rate: f64, bred: f64, kept: f64, fit: Box<Fitness>) -> GAlgo {
	    let mut pop = vec![];
	    for _ in 0..size {
	    	pop.push(BFProgram::new(thread_rng().gen_range(0,10)));
	    }
	    GAlgo {
	    	fit_func: fit,
	    	pop: pop, 
	    	mut_rate: mut_rate,
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
	pub fn step_pop(&mut self) -> Option<BFProgram> {
		let mut fitness = vec![];
		let mut tot_fit = 0f64;
		self.max_fit = self.fit_func.min_fitness() - 1f64;
		self.min_fit = self.fit_func.max_fitness() + 1f64;
		for i in 0..self.pop.len() {
			let fit = self.fit_func.fitness(&self.pop[i]);

			if fit > self.max_fit {
				self.max_fit = fit
			} else if fit < self.min_fit {
				self.min_fit = fit
			}

			if (fit - self.fit_func.max_fitness()).abs() < 0.001 {
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