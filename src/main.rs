#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod genetic;
mod utils;
mod world;

use crate::genetic::*;

/*
Any live cell with fewer than two live neighbors dies, as if by underpopulation.
Any live cell with two or three live neighbors lives on to the next generation.
Any live cell with more than three live neighbors dies, as if by overpopulation.
Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
*/
fn main() {
    let n_indi = 5;
    let p_alive = 0.5;
    let p_mut = 0.4;
    let mut pop = Population {
        genome_size: 9,
        genomes: vec![],
        fitness: vec![0; n_indi],
    };
    pop.init_rand_gen(&(n_indi as i32), &p_alive);
    // println!("{:?}", pop.genomes);
    for _ in 0..9 {
        pop.calc_fitness();
        pop.evolve(&p_mut);
    }
    println!("{:?}", pop.fitness);
    // println!("{:?}", pop.genomes)
    for i in pop.genomes.chunks_exact(pop.genome_size) {
        println!("{:?}", i)
    }
}
