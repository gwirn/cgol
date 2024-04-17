use crate::utils::softmax;
use crate::world::{draw_pattern, live_life, reset_world};
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::Rng;

#[derive(Debug)]
pub struct Population {
    pub genome_size: usize,
    pub genomes: Vec<bool>,
    pub fitness: Vec<i32>,
}
impl Population {
    /// low to high
    pub fn fitness_sorted(&self) -> Vec<usize> {
        let mut idxs = (0..self.fitness.len()).collect::<Vec<_>>();
        idxs.sort_by_key(|i| &self.fitness[*i]);
        idxs
    }
}
impl Population {
    pub fn calc_fitness(&mut self) {
        // let x_len = 30;
        // let y_len = 15;
        let x_len = 5;
        let y_len = 5;
        let x_limit = x_len - 1;
        let y_limit = y_len - 1;
        let n_epochs = 100;
        let pmsl = (self.genome_size as f32).sqrt() as usize;

        let mut old_state = vec![vec![false; x_len]; y_len];
        let mut new_state = vec![vec![false; x_len]; y_len];
        let individuums = self.genomes.chunks_exact(self.genome_size);
        if !individuums.remainder().is_empty()
            || self.genomes.len() / self.genome_size != self.fitness.len()
        {
            panic!("Genome size does not match genomes and fitness")
        }
        for (ci, i) in individuums.enumerate() {
            if ci > 0 {
                reset_world(&mut old_state, &x_len, &y_len);
                reset_world(&mut new_state, &x_len, &y_len);
            }
            draw_pattern(&mut old_state, &pmsl, i);
            let n_living = live_life(
                &mut old_state,
                &mut new_state,
                &x_len,
                &y_len,
                &y_limit,
                &x_limit,
                &n_epochs,
                false,
            );
            self.fitness[ci] = n_living;
        }
    }
}
impl Population {
    pub fn evolve(&mut self, mutation_prop: &f32) {
        assert!(
            self.fitness.len() > 3,
            "There must be more than 3 individuums in the population to evolve"
        );
        let n_indi = self.fitness.len();
        let ind_idx: Vec<usize> = (0..n_indi).collect();
        let crossover_prob = softmax(&self.fitness);
        let mut new_pop: Vec<bool> = Vec::new();
        let mut rng = thread_rng();
        for _ in 0..n_indi {
            let dist0 = WeightedIndex::new(&crossover_prob).unwrap();
            let ind0 = ind_idx[dist0.sample(&mut rng)];
            let dist = WeightedIndex::new(
                crossover_prob
                    .iter()
                    .enumerate()
                    .filter(|&(cx, _)| cx != ind0)
                    .map(|(_, x)| x)
                    .collect::<Vec<_>>(),
            )
            .unwrap();
            let ind1 =
                ind_idx.iter().filter(|&x| *x != ind0).collect::<Vec<_>>()[dist.sample(&mut rng)];
            let split_border = rng.gen_range(0..self.genome_size);
            new_pop.append(
                &mut self.genomes[self.genome_size * ind0..self.genome_size * (ind0 + 1)]
                    [0..split_border]
                    .to_vec(),
            );
            new_pop.append(
                &mut self.genomes[self.genome_size * ind1..self.genome_size * (ind1 + 1)]
                    [split_border..self.genome_size]
                    .to_vec(),
            );
            if rng.gen::<f32>() > *mutation_prop {
                let mut mut_position = rng.gen_range(0..self.genome_size);
                mut_position = new_pop.len() - self.genome_size + mut_position;
                new_pop[mut_position] = !new_pop[mut_position];
            };
        }
        self.genomes = new_pop;
    }
}
impl Population {
    pub fn init_rand_gen(&mut self, n_indi: &i32, prob_alive: &f32) {
        let mut rng = thread_rng();
        for _ in 0..*n_indi * (self.genome_size as i32) {
            self.genomes.push(rng.gen::<f32>() < *prob_alive);
        }
    }
}
