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
pub fn reset_world(world: &mut [Vec<bool>], x_len: &usize, y_len: &usize) {
    for x in 0..*x_len {
        for y in world.iter_mut().take(*y_len) {
            y[x] = false;
        }
    }
}
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
pub fn draw_pattern(
    old_world: &mut [Vec<bool>],
    pattern_mat_side_len: &usize,
    start_config: &[bool],
) {
    assert!(
        start_config.len() == pattern_mat_side_len * pattern_mat_side_len,
        "Start config size doesn't match pattern_mat_side_len size"
    );
    for (i, _) in start_config
        .iter()
        .enumerate()
        .take(pattern_mat_side_len * pattern_mat_side_len)
    {
        let ix = i % pattern_mat_side_len;
        let mut cur_y = (i / pattern_mat_side_len) as f32;
        cur_y = cur_y.floor();
        let iy = cur_y as usize;
        if start_config[i] {
            old_world[iy][ix] = true;
        };
    }
}

pub fn live_life(
    old_world: &mut Vec<Vec<bool>>,
    new_world: &mut [Vec<bool>],
    x_len: &usize,
    y_len: &usize,
    y_lim: &usize,
    x_lim: &usize,
    epochs: &i32,
    show_out: bool,
) -> i32 {
    let start_idx: usize = 0;
    let mut live_cells = 0;
    for i in 0..*epochs {
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
                    live_cells += 1;
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
        if live_cells == 0 {
            return 0;
        }
        if i < (*epochs - 1) {
            live_cells = 0;
        }
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
    live_cells
}
