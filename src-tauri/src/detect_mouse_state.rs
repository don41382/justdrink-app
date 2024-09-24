use std::cmp::{max, min};
use mouse_position::mouse_position::{Mouse, Position};
use std::collections::VecDeque;
use std::thread::sleep;
use std::time::Duration;
use log::info;

pub enum MouseState {
    SHAKE,
}

pub fn detect_mouse_shake(on_shake: Box<dyn Fn(MouseState) + Send + 'static>) {
    const MAX_SHAKE_WINDOW: usize = 30;
    tauri::async_runtime::spawn(async move {
        let mut shake_window = VecDeque::with_capacity(MAX_SHAKE_WINDOW);
        loop {
            let position = Mouse::get_mouse_position();
            match position {
                Mouse::Position { x, y: _ } => {
                    if shake_window.len() >= MAX_SHAKE_WINDOW {
                        shake_window.pop_front();
                    }
                    shake_window.push_back(x);

                    let vec = shake_window.iter().map(|&x| x).collect::<Vec<i32>>();
                    if is_mouse_shaking(vec, 5, 3) {
                        on_shake(MouseState::SHAKE);
                        shake_window.clear();
                    }
                }
                Mouse::Error => {}
            }
            sleep(Duration::from_millis(50));
        }
    });
}

pub enum Mode {
    Idle,
    Working,
}

pub fn detect_mouse_idl(idle_duration_s: usize, working_duration_s: usize, on_mode_switch: Box<dyn Fn(Mode) + Send + 'static>) {
    let idle_window_size: usize = 60 * max(idle_duration_s, working_duration_s);
    tauri::async_runtime::spawn(async move {
        let mut idle_window: VecDeque<Position> = VecDeque::with_capacity(idle_window_size);
        let mut mode = Mode::Working;
        loop {
            let position = Mouse::get_mouse_position();
            match position {
                Mouse::Position { x, y } => {
                    if idle_window.len() >= idle_window_size {
                        idle_window.pop_front();
                    }
                    idle_window.push_back(Position { x, y });
                }
                Mouse::Error => {}
            }

            match mode {
                Mode::Working => {
                    let positions: Vec<&Position> = idle_window.iter().rev().take(idle_duration_s).collect();
                    let distance = max_distance(&positions);
                    if distance < 100.0 && (positions.len() >= idle_duration_s) {
                        println!("switch to idle: {:.2}", &distance);
                        mode = Mode::Idle;
                        on_mode_switch(Mode::Idle);
                    }
                }
                Mode::Idle => {
                    let positions: Vec<&Position> = idle_window.iter().rev().take(working_duration_s).collect();
                    let distance = max_distance(&positions);
                    if distance > 100.0 && (positions.len() >= working_duration_s) {
                        println!("switch to working: {:.2}", &distance);
                        mode = Mode::Working;
                        on_mode_switch(Mode::Working);
                    }
                }
            }
            sleep(Duration::from_secs(1));
        }
    });
}

fn max_distance(positions: &Vec<&Position>) -> f64 {
    let mut max_diff = 0.0;

    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            let dx = (positions[i].x - positions[j].x).pow(2) as f64;
            let dy = (positions[i].y - positions[j].y).pow(2) as f64;
            let distance = (dx + dy).sqrt();

            if distance > max_diff {
                max_diff = distance;
            }
        }
    }

    max_diff
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
    use std::thread::sleep;
    use std::time::Duration;
    use tauri::command;
    use crate::detect_mouse_state::{detect_mouse_idl, Mode};

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
