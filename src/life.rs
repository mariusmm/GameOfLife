use rand::distributions::{Uniform, Distribution};
extern crate rand;

pub fn gen_board(size: i32) -> Vec<u8>
{
    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..2);
    (0..size*size).map(|_| die.sample(&mut rng)).collect()
}

fn simulate_cell(grid: &[u8], cx: i32, cy: i32, size: i32) -> u8 {
    let mut sum = 0;
    for y in cy-1..cy+2 {
        for x in cx-1..cx+2 {
            if !(cx == x && cy == y) {
                if x >= 0 && x < size && y >= 0 && y < size {
                    sum += grid[(size*y + x) as usize];
                }
            }
        }
    }

    let neigbours = sum;
    if grid[(size*cy + cx) as usize] == 1 && neigbours < 2 || neigbours > 3 {
        0
    } else if neigbours == 3 {
        1
    } else {
        grid[(size*cy + cx) as usize]
    }
}


pub fn life(grid: &[u8], next: &mut [u8], size: i32)
{
    assert!(size % 2 == 0);
    let (first, second) = next.split_at_mut((size*size/2) as usize);
    std::thread::scope(|s| {
        let thread = s.spawn(|| {
            for cy in 0..size/2 {
                for cx in 0..size {
                    first[(size*cy + cx) as usize] = simulate_cell(&grid, cx, cy, size);
                }
            }
        });

        let other = s.spawn(|| {
            for cy in 0..size/2 {
                for cx in 0..size {
                    second[(size*cy + cx) as usize] = simulate_cell(&grid, cx, cy + size/2, size);
                }
            }
        });

        let _ = thread.join();
        let _ = other.join();
    });
}

// viva:
// < 2: muerte
// 2,3: vive
// > 3, muerte

// muerta:
// 3: revive

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let size = 4;
        let mut board: Vec<u8> = vec![0; size*size];
        board[0*size + 1] = 1;
        board[1*size + 1] = 1;
        board[2*size + 1] = 1;
        let mut next: Vec<u8> = vec![0; size*size];
        let mut board_expected: Vec<u8> = vec![0; size*size];
        board_expected[1*size + 0] = 1;
        board_expected[1*size + 1] = 1;
        board_expected[1*size + 2] = 1;
        life(&board[..], &mut next[..], size as i32);
        assert_eq!(board_expected, next);
    }
}

