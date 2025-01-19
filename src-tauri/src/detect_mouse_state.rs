use mouse_position::mouse_position::Mouse;
use std::thread::sleep;
use std::time::Duration;

pub enum MouseState {
    SHAKE,
}

pub fn detect_mouse_shake(on_shake: Box<dyn Fn(MouseState) + Send + 'static>) {
    const MAX_SHAKE_WINDOW: usize = 30;

    tauri::async_runtime::spawn(async move {
        // Use a simple Vec with capacity instead of VecDeque
        let mut shake_window = Vec::with_capacity(MAX_SHAKE_WINDOW);

        loop {
            let position = Mouse::get_mouse_position();
            match position {
                Mouse::Position { x, y: _ } => {
                    if shake_window.len() >= MAX_SHAKE_WINDOW {
                        // Remove the oldest (front) element if we're at capacity
                        shake_window.remove(0);
                    }
                    shake_window.push(x);

                    // Pass the Vec by slice instead of creating a new Vec
                    if is_mouse_shaking(&shake_window, 10, 3) {
                        on_shake(MouseState::SHAKE);
                        shake_window.clear();
                    }
                }
                Mouse::Error => {
                    // Do nothing or handle the error
                }
            }

            sleep(Duration::from_millis(50));
        }
    });
}

fn is_mouse_shaking(y_coords: &[i32], min_distance: i32, min_direction_changes: usize) -> bool {
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
                    increasing = Some(is_increasing); // Update direction
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
