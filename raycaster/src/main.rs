use crossterm::event::KeyCode;
use crate::{canvas::{Canvas, color::Color, input::Input}, map::Map, math::{mathf, numerics::float2::Float2}};

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
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 1, 0, 0, 1, 1, 0],
            vec![0, 1, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 1, 1, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
        ],
    };
    
    let mut player_pos = Float2::new(3.5, 2.5);
    let mut view_dir = Float2::new(0.0, 1.0);
    let fov: f32 = 90.0;
    let move_speed = 0.05_f32;
    let rotation_speed = 0.05_f32;
    
    loop {
        let _ = input.update();
        if input.is_key_down(KeyCode::Esc) { break; }
        
        canvas.clear();
        
        if input.is_key_pressed(KeyCode::Char('a')) {
            let cos_a = (-rotation_speed).cos();
            let sin_a = (-rotation_speed).sin();
            let new_x = view_dir.x * cos_a - view_dir.y * sin_a;
            let new_y = view_dir.x * sin_a + view_dir.y * cos_a;
            view_dir = Float2::new(new_x, new_y);
        }
        if input.is_key_pressed(KeyCode::Char('d')) {
            let cos_a = rotation_speed.cos();
            let sin_a = rotation_speed.sin();
            let new_x = view_dir.x * cos_a - view_dir.y * sin_a;
            let new_y = view_dir.x * sin_a + view_dir.y * cos_a;
            view_dir = Float2::new(new_x, new_y);
        }
        
        if input.is_key_pressed(KeyCode::Char('w')) {
            player_pos.x += view_dir.x * move_speed;
            player_pos.y += view_dir.y * move_speed;
        }
        if input.is_key_pressed(KeyCode::Char('s')) {
            player_pos.x -= view_dir.x * move_speed;
            player_pos.y -= view_dir.y * move_speed;
        }
        
        let distances = raycaster::cast_rays(player_pos, view_dir, fov.to_radians(), canvas.width(), &map);
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

            let wall_height = (projection_dist / distance).min(screen_height * 2.0);
            let wall_start = (screen_center - wall_height / 2.0).max(0.0) as u32;
            let wall_end = (screen_center + wall_height / 2.0).min(screen_height) as u32;

            let max_distance = 10.0;
            let brightness = (1.0 - (distance / max_distance).min(1.0)).max(0.0);
            let base_color = 200.0;

            let r = (base_color * brightness) as u8;
            let g = (base_color * brightness) as u8;
            let b = (base_color * brightness) as u8;
            for y in wall_start..wall_end {
                canvas.set_pixel(x, y, Color::new(r, g, b));
            }
        }
        
        canvas.render();
    }
}