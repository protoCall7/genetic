extern crate rand;

const CHROMO_LENGTH: usize = 300;
const GENE_LENGTH: usize = 4;
const POPULATION_SIZE: u32 = 100;
const CROSSOVER_RATE: f32 = 0.7;
const MUTATE_RATE: f32 = 0.001;

#[derive(Debug)]
struct ChromosomeType {
	chromosome: String,
	parsed_genes: Vec<i32>,
	fitness: f32,
}

impl ChromosomeType {
	// generates a new chromosome
	fn new(generate: bool) -> ChromosomeType {
		// initialize the chromosome
		let mut chromo = ChromosomeType {
			chromosome: String::new(),
			parsed_genes: Vec::new(),
			fitness: 0.0,
		};

		if generate {
			// generate a random number between 0 and 1
			// if it is over 0.5 push a 1 to the string
			// othewise push a 0
			for _ in 0..CHROMO_LENGTH {
				let tmp = rand::random::<f32>();
				if tmp > 0.5 {
   	            	chromo.chromosome.push('1');
				} else {
					chromo.chromosome.push('0');
				}
			}
		}

		chromo
	}

	fn parse_genes(&mut self) {
		let mut offset = 0;
		let mut operator = false;

		// parse each gene, and push valid instructions to parsed_genes
		for _ in 0..(CHROMO_LENGTH / GENE_LENGTH) {
			let gene = &self.chromosome[offset..(offset + GENE_LENGTH)];
            let gene = bin_to_dec(gene);

			// valid genes range from 0..13
			if gene < 14 {
				if operator == false {    // looking for an operand
					if gene < 10 {        // we've got one
						self.parsed_genes.push(gene);
						operator = true;  // look for an operator next
					} else {              // this is an operator
						offset += GENE_LENGTH;
						continue;
					}
				} else {                  // looking for an operator
					if gene > 9 {         // we've got one
						self.parsed_genes.push(gene);
					    operator = false; // look for an operand next
					} else {              // this is an operand
						offset += GENE_LENGTH;
						continue;
					}
				}
			}
			offset += GENE_LENGTH;
		}

		// convert n / 0 to n + 0 for sanity
		for i in 0..(self.parsed_genes.len() - 1) {
			if (self.parsed_genes[i] == 13) && (self.parsed_genes[i + 1] == 0) {
				// this is a divide by 0
				self.parsed_genes[i] = 10;
			} 
		}

		let parsed_len = self.parsed_genes.len();
		match self.parsed_genes[parsed_len - 1] {
            10...13 => self.parsed_genes.truncate(parsed_len - 1),
			_       => {},
		}
	}

	/// Assigns fitness to a ChromosomeType
	///
	/// # Examples
	/// ```
	/// let mychromosome = ChromosomeType::new();
	/// mychromosome.parse_genes();
	/// mychromosome.assign_fitness();
    /// ```
	fn assign_fitness(&mut self, target: f32) {
		let mut result = 0.0;

		for i in 0..(self.parsed_genes.len() - 1) {
			match i {
				// seed result with the first operand
				0 => result = self.parsed_genes[0] as f32,
				_ => match self.parsed_genes[i] {
					10 => result += self.parsed_genes[i + 1] as f32,
					11 => result -= self.parsed_genes[i + 1] as f32,
					12 => result *= self.parsed_genes[i + 1] as f32,
					13 => result /= self.parsed_genes[i + 1] as f32,
					_  => continue,
				}
			}
		}

		if result == target {
			self.fitness = 999.9;
		} else {
			self.fitness = 1.0/(target - result).abs();
		}
	}

	fn print_chromosome(&self) {
		for i in &self.parsed_genes {
			match i {
				&0...9 => print!("{} ", i),
				&10    => print!("+ "),
				&11    => print!("- "),
				&12    => print!("* "),
				&13    => print!("/ "),
				_     => continue,
			}
		}
		println!("");
	}
}

fn main() {
	let mut population: Vec<ChromosomeType> = Vec::new();
	let mut generations_to_solution = 1;
	let found = false;
	println!("Enter Target");
	let mut target_number = String::new();

	std::io::stdin().read_line(&mut target_number).unwrap();

	let target_len = target_number.len();
	target_number.truncate(target_len - 1);

	let target_number: f32 = target_number.parse().unwrap();

	'outer: while found == false {
		let mut total_fitness = 0.0;

		// generate a population
		for _ in 0..POPULATION_SIZE {
			population.push(ChromosomeType::new(true));
		}

		// parse the genes for the population
		for mut individual in &mut population {
			individual.parse_genes();
			individual.assign_fitness(target_number);

			if individual.fitness == 999.9 {
				println!("Solution found in {} generations!", generations_to_solution);
				individual.print_chromosome();
				break 'outer;
			}

			total_fitness += individual.fitness;
		} // no solution if we got here. Create the next generation

		let mut tmp_pop: Vec<ChromosomeType> = Vec::new();

		for _ in 0..(POPULATION_SIZE) {
			tmp_pop.push(ChromosomeType::new(false));
		}

		for i in 0..tmp_pop.len() {
			// crossover mates
			let offspring1 = roulette(total_fitness, &population);
			let offspring2 = roulette(total_fitness, &population);

			let (newoffspring1, newoffspring2) = crossover(offspring1.to_string(), offspring2.to_string());

			if i % 2 == 0 {
				tmp_pop[i].chromosome = newoffspring1.to_string();
			} else {
				tmp_pop[i].chromosome = newoffspring2.to_string();
			}
		}

		population.truncate(0);

		for individual in tmp_pop {
			population.push(individual);
		}

		for mut individual in &mut population {
			individual.chromosome = mutate(&mut individual.chromosome);
		}

		generations_to_solution += 1;
	}
}

// helper to convert string representing binary nibble to decimal
fn bin_to_dec(gene: &str) -> i32 {
	let mut to_add = 1;
	let mut result = 0;

	for i in gene.chars().rev() {
		if i == '1' {
			result += to_add;
		} 

		to_add *= 2;
	}
	result
}

// roulette selection function takes population and total population fitness
// and returns a chromosome as a string reference.
fn roulette(total_fitness: f32, population: &Vec<ChromosomeType>) -> &str {
    // select random value between 0 and total fitness
	let slice = rand::random::<f32>() * total_fitness;
	let mut fitness_so_far = 0.0;

	for individual in population {
		fitness_so_far += individual.fitness;
		if fitness_so_far > slice {
			return &individual.chromosome;
		}
	}
	
	""
}

fn crossover(offspring1: String, offspring2: String) -> (String, String) {
	let mut tmp_random = rand::random::<f32>();
	let retstr1: String;
	let retstr2: String;

	if tmp_random < CROSSOVER_RATE {
	    tmp_random = rand::random::<f32>();	
		let crossover = ( tmp_random * CHROMO_LENGTH as f32) as usize;

		let slice1 = &offspring1[..crossover];
		let slice2 = &offspring2[crossover..];

		retstr1 = slice2.to_string() + slice1;
		retstr2 = slice1.to_string() + slice2;
	} else {
		retstr1 = offspring1;
		retstr2 = offspring2;
	}

	return (retstr1, retstr2);
}

fn mutate(chromosome: &mut str) -> String {
	let mut ret = String::new();
	
	for i in chromosome.chars() {
		let tmp_random = rand::random::<f32>();
		if tmp_random < MUTATE_RATE {
			match i {
				'0' => ret.push('1'),
				'1' => ret.push('0'),
				_   => continue,
			}
		} else {
			match i {
				'0' => ret.push('0'),
				'1' => ret.push('1'),
				_   => continue,
			}
		}
	}
	ret
}
