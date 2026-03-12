pub struct Map {
    pub grid: Vec<Vec<u8>>,
}

impl Map {
    pub fn is_wall(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || y >= self.grid.len() as i32 || x >= self.grid[0].len() as i32 {
            return true;
        }
        self.grid[y as usize][x as usize] == 1
    }
}