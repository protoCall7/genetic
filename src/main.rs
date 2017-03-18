extern crate rand;

const CHROMO_LENGTH: usize = 300;
const GENE_LENGTH: usize = 4;
const POPULATION_SIZE: u32 = 100;

#[derive(Debug)]
struct ChromosomeType {
    chromosome: String,
	parsed_genes: Vec<i32>,
    fitness: f32,
}

impl ChromosomeType {
	// generates a new chromosome
	fn new() -> ChromosomeType {
		// initialize the chromosome
		let mut chromo = ChromosomeType {
			chromosome: String::new(),
			parsed_genes: Vec::new(),
			fitness: 0.0,
		};

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
	}
}

fn main() {
	let mut population: Vec<ChromosomeType> = Vec::new();

	// generate a population
	for _ in 0..POPULATION_SIZE {
		population.push(ChromosomeType::new());
	}

	// parse the genes for the population
	for mut individual in population {
		individual.parse_genes();
		println!("{:?}", individual);
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
