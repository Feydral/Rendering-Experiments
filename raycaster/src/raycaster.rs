use crate::math::numerics::float2::Float2;

pub fn cast_rays(pos: Float2, view_angle: f32, fov: f32, resolution: u32, map: &Map) -> Vec<f32> {
    let mut rays = Vec::with_capacity(resolution as usize);
    
    let start_angle = view_angle - (fov / 2.0);
    let angle_step = fov / resolution as f32;
    
    for i in 0..resolution {
        let ray_angle = start_angle + (i as f32 * angle_step);
        let distance = cast_single_ray(pos, ray_angle, map);
        
        rays.push(distance);
    }
    
    rays
}

fn cast_single_ray(pos: Float2, angle: f32, map: &Map) -> f32 {
    let dx = angle.cos();
    let dy = angle.sin();
    
    let mut distance = 0.0;
    let step = 0.1;
    let max_distance = 50.0;
    
    while distance < max_distance {
        let check_x = pos.x + dx * distance;
        let check_y = pos.y + dy * distance;
        
        if map.is_wall(check_x as i32, check_y as i32) {
            return distance;
        }
        
        distance += step;
    }
    
    max_distance
}

pub struct Map {
    pub grid: Vec<Vec<u8>>,
}

impl Map {
    fn is_wall(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || y >= self.grid.len() as i32 || x >= self.grid[0].len() as i32 {
            return true;
        }
        self.grid[y as usize][x as usize] == 1
    }
}