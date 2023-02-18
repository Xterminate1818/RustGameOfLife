use raylib::prelude::*;

type Index = (i32, i32);
type UIndex = (usize, usize);

struct Grid {
    cell: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let cell: Vec<Vec<bool>> = vec![vec![false; width]; height]; 
        Grid{cell, width, height}
    } 

    pub fn normalize(&self, pos: Index) -> UIndex {
        (
            (pos.0 % self.width as i32) as usize, 
            (pos.1 % self.height as i32) as usize
        )
    }

    pub fn get(&self, pos: Index) -> bool {
        let pos = self.normalize(pos); 
        let ret = self.cell.get(pos.1).unwrap();
        *ret.get(pos.0).unwrap()
    }

    pub fn set(&mut self, pos: Index, val: bool) {
        let pos = self.normalize(pos);     
        let b = self.cell.get_mut(pos.1).unwrap();
        let b = b.get_mut(pos.0).unwrap(); 
        *b = val;
    }

    pub fn neighbors(&self, pos: Index) -> i32 {
        let mut nb = 0;
        for x in -1..=1_i32 {
            for y in -1..=1_i32 {
                if (x == 1) && (y == 1) {
                    continue;
                }
                if self.get((pos.0 + x, pos.1 + y)) {
                   nb += 1; 
                }
            }
        }
        nb
    }

    pub fn step(old: &Grid) -> Self {
       let mut new: Grid = Grid::new(old.width, old.height); 
        for x in 0..new.width {
            for y in 0..new.height {
                let pos = (x as i32, y as i32);
                let mut alive = old.get(pos);
                let nb = old.neighbors(pos);

                if alive && !(2..=3).contains(&nb) {
                    alive = false;
                }
                else if !alive && nb == 3 {
                    alive = true;
                }
                else {
                    alive = false;
                }
                new.set(pos, alive);
            }
        }
        new
    }
}

fn main() {
    const WIDTH: i32 = 600;
    const HEIGHT: i32 = 600;
    const TILE: i32 = 20;
    const TILE_LEN: i32 = WIDTH / TILE;

    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Hello, World")
        .build();
    rl.set_target_fps(60);


    let mut grid = Grid::new(TILE_LEN as usize, TILE_LEN as usize);
    grid.set((0, 0), true);
    grid.set((1, 0), true);
    grid.set((2, 0), true);
    grid.set((3, 0), true);

    while !rl.window_should_close() {
        let mut draw = rl.begin_drawing(&thread);
        
        let grid = Grid::step(&grid); 

        draw.clear_background(Color::BLACK);
        let red = Color::RED;
        for x in 0..TILE_LEN {
            for y in 0..TILE_LEN {
                if grid.get((x, y)) {
                    draw.draw_rectangle(x * TILE, y * TILE, TILE, TILE, red);
                }
            }
        }
    }
}
