use crossterm::event::KeyCode;

use crate::{canvas::{Canvas, color::Color, input::Input}, map::Map, math::numerics::float2::Float2};

mod math;
mod canvas;
mod raycaster;
mod map;

fn main() {
    let mut canvas = Canvas::new();
    let mut input = Input::new();
    
    let map = Map {
        grid: vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 1, 1, 0, 0, 1, 1, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 1, 1, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
        ],
    };
    
    let mut player_pos = Float2::new(3.5, 2.5);
    let mut view_angle = 0.0;
    let fov: f32 = 60.0;
    
    loop {
        let _ = input.update();
        if input.is_key_down(KeyCode::Esc) { break; }
        
        canvas.clear();
        
        if input.is_key_pressed(KeyCode::Char('a')) {
            view_angle -= 0.05
        }
        if input.is_key_pressed(KeyCode::Char('d')) {
            view_angle += 0.05
        }

        if input.is_key_pressed(KeyCode::Up) {
            player_pos.y += 0.05;
        }
        if input.is_key_pressed(KeyCode::Down) {
            player_pos.y -= 0.05;
        }
        if input.is_key_pressed(KeyCode::Left) {
            player_pos.x += 0.05;
        }
        if input.is_key_pressed(KeyCode::Right) {
            player_pos.x -= 0.05;
        }

        let distances = raycaster::cast_rays(player_pos, view_angle, fov.to_radians(), canvas.width(), &map);

        let projection_dist = (canvas.width() as f32 / 2.0) / (fov / 2.0 * std::f32::consts::PI / 180.0).tan();
        let screen_height = canvas.height() as f32;
        let screen_center = screen_height / 2.0;

        for x in 0..canvas.width() {
            for y in 0..canvas.height() {
                if y > canvas.height() / 2 {
                    canvas.set_pixel(x, y, Color::new(230, 230, 230));
                } else {
                    canvas.set_pixel(x, y, Color::new(30, 30, 30));
                }
            }

            let distance = distances[x as usize].max(0.05);

            let wall_height_f = projection_dist / distance;
            let wall_height_f = wall_height_f.min(screen_height * 2.0);

            let wall_start_f = screen_center - (wall_height_f / 2.0);
            let wall_end_f = wall_start_f + wall_height_f;

            let wall_start = wall_start_f.max(0.0) as u32;
            let wall_end = wall_end_f.min(screen_height) as u32;

            for y in wall_start..wall_end {
                canvas.set_pixel(x, y, Color::GRAY);
            }
        }
        
        canvas.render();
    }
}