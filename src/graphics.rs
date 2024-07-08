use minifb::{Key, MouseButton, Window, WindowOptions, Scale};
use crate::board::Board;

pub fn run_app(width: usize, height: usize, cell_size: usize, top_bar_height: usize, mut my_board: Board) {
    let mut window = Window::new(
        "Conway's Game of Life",
        width * cell_size,
        height * cell_size + top_bar_height,
        WindowOptions {
            scale: Scale::X1,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut buffer: Vec<u32> = vec![0; width * cell_size * (height * cell_size + top_bar_height)];
    let mut running = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Draw the controls
        for x in 0..width * cell_size {
            for y in 0..top_bar_height {
                let color = 0xAAAAAA; // Gray
                buffer[y * width * cell_size + x] = color;
            }
        }

        // Labels for buttons
        let labels = vec!["Start", "Stop", "Step"];
        for (i, &label) in labels.iter().enumerate() {
            let x = i * 60;
            for dx in 0..50 {
                for dy in 0..20 {
                    buffer[(5 + dy) * width * cell_size + (10 + x + dx)] = 0xFFFFFF; // White
                }
            }
        }

        // Check for mouse clicks
        if let Some((mx, my)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
            if window.get_mouse_down(MouseButton::Left) {
                if my < top_bar_height as f32 {
                    if mx < 60.0 { // Start
                        running = true;
                    } else if mx < 120.0 { // Stop
                        running = false;
                    } else if mx < 180.0 { // Step
                        my_board = my_board.next_generation();
                    }
                }
            }
        }

        // Update the buffer with the board state
        for y in 0..height {
            for x in 0..width {
                let color = if my_board.get(x, y) {
                    0x00FF00 // Green
                } else {
                    0x000000 // Black
                };

                for dy in 0..cell_size {
                    for dx in 0..cell_size {
                        buffer[(top_bar_height + y * cell_size + dy) * width * cell_size + (x * cell_size + dx)] = color;
                    }
                }
            }
        }

        window.update_with_buffer(&buffer, width * cell_size, height * cell_size + top_bar_height).unwrap();

        if running {
            my_board = my_board.next_generation();
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
