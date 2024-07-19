#[cfg(test)]
mod tests {
    use super::super::board::Board;
    use std::time::{Duration, Instant};


    fn test_board(width: usize, height: usize, _threading: bool) -> Duration {
        let board = Board::new(width, height);
        let start = Instant::now();
        board.next_generation();
        start.elapsed()
    }

    #[test]
    fn test_small_board_single_thread() {
        let elapsed = test_board(10, 10, false);
        println!("Single-threaded 10x10: {:?}", elapsed);
    }

    #[test]
    fn test_small_board_multi_thread() {
        let elapsed = test_board(10, 10, true);
        println!("Multi-threaded 10x10: {:?}", elapsed);
    }

    #[test]
    fn test_large_board_single_thread() {
        let elapsed = test_board(1000, 1000, false);
        println!("Single-threaded 1000x1000: {:?}", elapsed);
    }

    #[test]
    fn test_large_board_multi_thread() {
        let elapsed = test_board(1000, 1000, true);
        println!("Multi-threaded 1000x1000: {:?}", elapsed);
    }

    #[test]
    fn compare_threading_on_various_sizes() {
        let sizes = vec![(10, 10), (100, 100), (500, 500), (1000, 1000)];
        
        for (width, height) in sizes {
            let elapsed_single = test_board(width, height, false);
            let elapsed_multi = test_board(width, height, true);

            println!(
                "Size: {}x{} | Single-threaded: {:?} | Multi-threaded: {:?}",
                width, height, elapsed_single, elapsed_multi
            );
        }
    }

}

