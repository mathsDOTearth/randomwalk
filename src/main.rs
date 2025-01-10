// 8 way random walk with minifb and a 5px dot
// Now with added wind!
// written for maths.earth 20250110 

use minifb::{Key, MouseButton, Window, WindowOptions};
use rand::Rng;

// Define the size of the window.
const WIDTH: usize = 800;
const HEIGHT: usize = 600;

// Define the size of the dot as an odd number so the "centre" aligns with the walker's (x, y).
const DOT_SIZE: usize = 5;

// Define fade speed.
const FADE_SPEED: u16 = 15;

// Define struct to store the walker's position.
struct Walker {
    x: f32,
    y: f32,
}

// Define methods for the Walker.
impl Walker {
    fn new() -> Self {
        Walker {
            x: (WIDTH / 2) as f32,
            y: (HEIGHT / 2) as f32,
        }
    }

    fn step(&mut self) {
        // 8 random directions.
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0_f32..360.0_f32).to_radians();
        let dx = angle.cos();
        let dy = angle.sin();

        self.x = (self.x + dx).clamp(0.0, (WIDTH - 1) as f32);
        self.y = (self.y + dy).clamp(0.0, (HEIGHT - 1) as f32);
    }

    fn blow_away(&mut self, mouse_x: f32, mouse_y: f32) {
        // Calculate direction vector from the mouse pointer to the walker.
        let delta_x = self.x - mouse_x;
        let delta_y = self.y - mouse_y;

        // Normalise the vector to calculate the unit direction vector.
        let distance = (delta_x.powi(2) + delta_y.powi(2)).sqrt();
        if distance > 0.0 {
            let unit_x = delta_x / distance;
            let unit_y = delta_y / distance;

            // Move in the direction away from the mouse.
            self.x = (self.x + unit_x).clamp(0.0, (WIDTH - 1) as f32);
            self.y = (self.y + unit_y).clamp(0.0, (HEIGHT - 1) as f32);
        }
    }

    fn show(&self, buffer: &mut [u32]) {
        // Plot a small DOT_SIZE x DOT_SIZE block around (x, y).
        let half = (DOT_SIZE / 2) as isize;

        for dy in -half..=half {
            for dx in -half..=half {
                let px = self.x as isize + dx;
                let py = self.y as isize + dy;

                if px >= 0 && px < WIDTH as isize && py >= 0 && py < HEIGHT as isize {
                    buffer[py as usize * WIDTH + px as usize] = 0x000000; // black
                }
            }
        }
    }
}

fn main() {
    // Create a window with default options.
    let mut window = Window::new(
        "Random Walk with Mouse Interaction (Away Direction)",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Failed to create window: {}", e);
    });

    let mut temp_pixel: u32;
    let mut fade_speed: u16 = FADE_SPEED;

    // Create a buffer for the entire screen, initialised to white.
    let mut buffer = vec![0xFFFFFF; WIDTH * HEIGHT];

    // Create our Walker.
    let mut walker = Walker::new();

    // Main event loop.
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if fade_speed == 0 {
            // Fade the buffer to white.
            for pixel in buffer.iter_mut() {
                if *pixel != 0xFFFFFF {
                    temp_pixel = *pixel;
                    temp_pixel += 0x010101;
                    if temp_pixel > 0xFFFFFF {
                        *pixel = 0xFFFFFF;
                    } else {
                        *pixel = temp_pixel;
                    }
                }
            }

            fade_speed = FADE_SPEED;
        } else {
            fade_speed -= 1;
        }

        // Check mouse position and button state.
        if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
            let mouse_x = mouse_x as f32;
            let mouse_y = mouse_y as f32;

            if window.get_mouse_down(MouseButton::Left) {
                walker.blow_away(mouse_x, mouse_y);
            } else {
                walker.step();
            }
        } else {
            walker.step();
        }

        // Draw the new position with a larger dot.
        walker.show(&mut buffer);

        // Render the updated buffer.
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
