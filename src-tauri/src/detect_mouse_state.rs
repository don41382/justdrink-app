use mouse_position::mouse_position::Mouse;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::collections::VecDeque;
use std::thread::sleep;
use std::time::Duration;
use tauri::Runtime;
use tauri_specta::Event;

pub enum MouseState {
    SHAKE
}


pub fn init(on_shake: Box<dyn Fn(MouseState) + Send + 'static>) {
    const MAX_WINDOW: usize = 30;
    tauri::async_runtime::spawn(async move {
        let mut window = VecDeque::with_capacity(MAX_WINDOW);
        loop {
            let position = Mouse::get_mouse_position();
            match position {
                Mouse::Position { x, y: _ } => {
                    if window.len() >= MAX_WINDOW {
                        window.pop_front();
                    }
                    window.push_back(x);

                    let vec = window.iter().map(|&x| x).collect::<Vec<i32>>();
                    if is_mouse_shaking(vec, 5, 4) {
                        on_shake(MouseState::SHAKE);
                        window.clear();
                    }
                }
                Mouse::Error => {}
            }
            sleep(Duration::from_millis(50));
        }
    });
}

fn is_mouse_shaking(y_coords: Vec<i32>, min_distance: i32, min_direction_changes: usize) -> bool {
    if y_coords.len() < 3 {
        // Need at least 3 points to detect a back-and-forth pattern
        return false;
    }

    let mut increasing = None; // None at the start, then Some(true) or Some(false)
    let mut direction_changes = 0;

    for window in y_coords.windows(2) {
        let (prev, current) = (window[0], window[1]);

        let diff = (current - prev).abs();
        if diff < min_distance {
            // Ignore movements smaller than the minimum distance
            continue;
        }

        let is_increasing = current > prev;

        match increasing {
            Some(last_direction) => {
                if last_direction != is_increasing {
                    // Direction change detected
                    direction_changes += 1;
                    increasing = Some(is_increasing); // Update the direction
                }
            }
            None => {
                // Set the initial movement direction
                increasing = Some(is_increasing);
            }
        }

        // Early exit if we have already detected enough direction changes
        if direction_changes >= min_direction_changes {
            return true;
        }
    }

    // Check if the number of direction changes meets the required minimum
    direction_changes >= min_direction_changes
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn test_vec() {
        let mut vec = Vec::with_capacity(5);
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        vec.push(5);
        vec.push(6);
        vec.truncate(5);
        println!("{:?}", vec);
    }
}