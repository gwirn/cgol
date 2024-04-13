use core::f32;
use std::{thread, time};

static KERNEL: [[i32; 2]; 8] = [
    [-1, -1],
    [0, -1],
    [1, -1],
    [-1, 0],
    [1, 0],
    [-1, 1],
    [0, 1],
    [1, 1],
];
fn wrap(coord: i32, limit: usize) -> usize {
    if coord > limit.try_into().unwrap() {
        return 0;
    }
    if coord < 0 {
        return limit;
    }
    coord.try_into().unwrap()
}
fn count_neighbors(in_arr: &[Vec<bool>], y: i32, x: i32, y_lim: usize, x_lim: usize) -> u8 {
    let mut n_neighbors = 0;
    for i in KERNEL.iter() {
        // add wrap around and then as usize
        let check_y = wrap(y + i[0], y_lim);
        let check_x = wrap(x + i[1], x_lim);
        if in_arr[check_y][check_x] {
            n_neighbors += 1;
        }
    }
    n_neighbors
}
fn draw_pattern(
    old_world: &mut [Vec<bool>],
    pattern_mat_side_len: &usize,
    start_x: &usize,
    start_y: &usize,
    start_config: &[bool],
) {
    for (i, _) in start_config
        .iter()
        .enumerate()
        .take(pattern_mat_side_len * pattern_mat_side_len)
    {
        let mut ix = i % pattern_mat_side_len;
        let mut cur_y = (i / pattern_mat_side_len) as f32;
        cur_y = cur_y.floor();
        let mut iy = cur_y as usize;
        ix += start_x;
        iy += start_y;
        if start_config[i] {
            old_world[iy][ix] = true;
        };
    }
}
fn evolve(
    old_world: &mut Vec<Vec<bool>>,
    new_world: &mut [Vec<bool>],
    x_len: &usize,
    y_len: &usize,
    y_lim: &usize,
    x_lim: &usize,
    epochs: &i32,
    show_out: bool,
) {
    let start_idx: usize = 0;
    for _ in 0..*epochs {
        for x_ in start_idx..*x_len {
            for y_ in start_idx..*y_len {
                let cell_neigh = count_neighbors(
                    old_world,
                    y_.try_into().unwrap(),
                    x_.try_into().unwrap(),
                    *y_lim,
                    *x_lim,
                );
                if old_world[y_][x_] {
                    if cell_neigh < 2 {
                        new_world[y_][x_] = false;
                    }
                    if cell_neigh == 2 || cell_neigh == 3 {
                        new_world[y_][x_] = true;
                    }
                    if cell_neigh > 3 {
                        new_world[y_][x_] = false;
                    }
                } else if cell_neigh == 3 {
                    new_world[y_][x_] = true;
                }
            }
        }
        *old_world = (*new_world.to_owned()).to_vec();
        if show_out {
            let sleep_time = time::Duration::from_millis(100);
            for i in old_world.iter() {
                for y in i {
                    if *y {
                        print!(".")
                    } else {
                        print!(" ")
                    }
                }
                println!();
            }
            thread::sleep(sleep_time);
            print!("{esc}c", esc = 27 as char);
        }
    }
}
/*
Any live cell with fewer than two live neighbors dies, as if by underpopulation.
Any live cell with two or three live neighbors lives on to the next generation.
Any live cell with more than three live neighbors dies, as if by overpopulation.
Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
*/
fn main() {
    let x_len = 5;
    let y_len = 5;
    let x_limit = x_len - 1;
    let y_limit = y_len - 1;
    let n_epochs = 1;
    let pmsl = 3;
    let mut old_state = vec![vec![false; x_len]; y_len];
    let mut new_state = vec![vec![false; x_len]; y_len];
    /*
    old_state[2][1] = true;
    old_state[2][2] = true;
    old_state[2][3] = true;
    old_state[1][3] = true;
    old_state[0][2] = true;
    */
    // old_state[2][0] = true;
    evolve(
        &mut old_state,
        &mut new_state,
        &x_len,
        &y_len,
        &y_limit,
        &x_limit,
        &n_epochs,
        false,
    );

    let start_config = vec![true, false, false, true, false, false, true, false, false];
    let (start_x, start_y) = (1, 2);
    draw_pattern(&mut old_state, &pmsl, &start_x, &start_y, &start_config);
    for i in old_state.iter() {
        println!("{:?}", i);
    }
}
