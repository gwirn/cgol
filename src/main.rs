mod genetic;
mod utils;
mod world;

use world::live_life;

use crate::{genetic::*, world::draw_pattern};

/*
Any live cell with fewer than two live neighbors dies, as if by underpopulation.
Any live cell with two or three live neighbors lives on to the next generation.
Any live cell with more than three live neighbors dies, as if by overpopulation.
Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
*/
fn start_with_genome(genome: &[bool], x_len: &usize, y_len: &usize) {
    let pmsl = (genome.len() as f32).sqrt() as usize;
    let mut old_state = vec![vec![false; *x_len]; *y_len];
    let mut new_state = vec![vec![false; *x_len]; *y_len];
    draw_pattern(&mut old_state, &pmsl, genome);
    live_life(&mut old_state, &mut new_state, x_len, y_len, &100, true);
}
fn main() {
    let n_indi = 50;
    let p_alive = 0.5;
    let p_mut = 0.6;
    let x_len = 40;
    let y_len = 40;
    let mut pop = Population {
        genome_size: 9,
        genomes: vec![],
        fitness: vec![0; n_indi],
    };
    pop.init_rand_gen(&(n_indi as i32), &p_alive);
    // println!("{:?}", pop.genomes);
    for _ in 0..9 {
        pop.calc_fitness(&x_len, &y_len);
        pop.evolve(&p_mut);
    }
    println!("{:?}", pop.fitness);
    // println!("{:?}", pop.genomes)
    let fitnesses = pop.fitness_sorted();
    let split_genomes: Vec<&[bool]> = pop.genomes.chunks_exact(pop.genome_size).collect();
    for i in 0..5 {
        start_with_genome(split_genomes[fitnesses.len() - (i + 1)], &x_len, &y_len)
    }
    println!("{:?}", pop.fitness)
}
