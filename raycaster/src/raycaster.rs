use crate::{map::Map, math::numerics::float2::Float2};

const MAX_RAY_DISTANCE: f32 = 50.0;
const MAX_DDA_STEPS: usize = 100;

pub fn cast_rays(pos: Float2, view_angle: f32, fov: f32, resolution: u32, map: &Map) -> Vec<f32> {
    let mut rays = Vec::with_capacity(resolution as usize);
    
    let projection_dist = 1.0;
    let half_fov = fov / 2.0;
    let projection_plane_width = 2.0 * projection_dist * half_fov.tan();
    
    for i in 0..resolution {
        let screen_x = (i as f32 / resolution as f32) - 0.5;
        let plane_x = screen_x * projection_plane_width;
        
        let ray_angle = plane_x.atan2(projection_dist) + view_angle;
        
        let distance = cast_single_ray(pos, ray_angle, map);
        
        let relative_angle = ray_angle - view_angle;
        let corrected_distance = distance * relative_angle.cos();
        
        rays.push(corrected_distance);
    }
    
    rays
}

fn cast_single_ray(pos: Float2, angle: f32, map: &Map) -> f32 {
    let dir_x = angle.cos();
    let dir_y = angle.sin();
    
    let mut map_x = pos.x as i32;
    let mut map_y = pos.y as i32;
    
    let delta_dist_x = if dir_x == 0.0 { f32::MAX } else { (1.0 / dir_x).abs() };
    let delta_dist_y = if dir_y == 0.0 { f32::MAX } else { (1.0 / dir_y).abs() };
    
    let step_x: i32;
    let step_y: i32;
    
    let mut side_dist_x: f32;
    let mut side_dist_y: f32;
    
    if dir_x < 0.0 {
        step_x = -1;
        side_dist_x = (pos.x - map_x as f32) * delta_dist_x;
    } else {
        step_x = 1;
        side_dist_x = (map_x as f32 + 1.0 - pos.x) * delta_dist_x;
    }
    
    if dir_y < 0.0 {
        step_y = -1;
        side_dist_y = (pos.y - map_y as f32) * delta_dist_y;
    } else {
        step_y = 1;
        side_dist_y = (map_y as f32 + 1.0 - pos.y) * delta_dist_y;
    }
    
    let mut hit = false;
    let mut side = 0;
    
    for _ in 0..MAX_DDA_STEPS {
        if side_dist_x < side_dist_y {
            side_dist_x += delta_dist_x;
            map_x += step_x;
            side = 0;
        } else {
            side_dist_y += delta_dist_y;
            map_y += step_y;
            side = 1;
        }
        
        if map.is_wall(map_x, map_y) {
            hit = true;
            break;
        }
    }
    
    if !hit {
        return MAX_RAY_DISTANCE;
    }
    
    let distance = if side == 0 {
        (map_x as f32 - pos.x + (1.0 - step_x as f32) / 2.0) / dir_x
    } else {
        (map_y as f32 - pos.y + (1.0 - step_y as f32) / 2.0) / dir_y
    };
    
    distance.abs()
}
