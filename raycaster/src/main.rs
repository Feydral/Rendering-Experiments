use crossterm::event::KeyCode;

use crate::{canvas::{Canvas, color::Color, input::Input}, math::numerics::float2::Float2, raycaster::Map};

mod math;
mod canvas;
mod raycaster;

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
        
        let distances = raycaster::cast_rays(player_pos, view_angle, 80.0_f32.to_radians(), canvas.width(), &map);

        for x in 0..canvas.width() {
            let distance = distances[x as usize];
            let wall_height = (1.0 / distance * 60.0).min(canvas.height() as f32) as u32;

            let canvas_center = canvas.height() / 2;
            let wall_start = canvas_center - (wall_height / 2);
            let wall_end = wall_start + wall_height;
                
            for y in wall_start..wall_end {
                canvas.set_pixel(x, y, Color::GRAY);
            }
        }
        
        canvas.render();
    }
}