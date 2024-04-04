use std::{thread, time};

const Y: usize = 15;
const X: usize = 30;
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
fn count_neighbors(in_arr: &[[bool; X]; Y], y: i32, x: i32, y_lim: usize, x_lim: usize) -> u8 {
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
/*
Any live cell with fewer than two live neighbors dies, as if by underpopulation.
Any live cell with two or three live neighbors lives on to the next generation.
Any live cell with more than three live neighbors dies, as if by overpopulation.
Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
*/
fn main() {
    let sleep_time = time::Duration::from_millis(100);
    let x_limit = X - 1;
    let y_limit = Y - 1;
    let mut old_state = [[false; X]; Y];
    let mut new_state = [[false; X]; Y];
    old_state[2][1] = true;
    old_state[2][2] = true;
    old_state[2][3] = true;
    old_state[1][3] = true;
    old_state[0][2] = true;
    // old_state[2][0] = true;
    for _ in 0..100 {
        for x_ in 0..X {
            for y_ in 0..Y {
                let cell_neigh = count_neighbors(
                    &old_state,
                    y_.try_into().unwrap(),
                    x_.try_into().unwrap(),
                    y_limit,
                    x_limit,
                );
                if old_state[y_][x_] {
                    if cell_neigh < 2 {
                        new_state[y_][x_] = false;
                    }
                    if cell_neigh == 2 || cell_neigh == 3 {
                        new_state[y_][x_] = true;
                    }
                    if cell_neigh > 3 {
                        new_state[y_][x_] = false;
                    }
                } else if cell_neigh == 3 {
                    new_state[y_][x_] = true;
                }
            }
        }
        old_state = new_state;
        for i in old_state.iter() {
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
