use tui::{style::Color, widgets::canvas::Shape};

pub struct GameOfLife {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    rule: fn(&GameOfLife, u32, u32) -> bool,
    color: Color,
}

pub struct Cell {
    alive: bool,
}

fn default_rule(game: &GameOfLife, x: u32, y: u32) -> bool {
    let neighbours = game.get_neighbours(x, y);
    let alive = game.get_cell(x, y);
    match (alive, neighbours) {
        (true, 2) | (true, 3) => true,
        (false, 3) => true,
        _ => false,
    }
}

impl GameOfLife {
    pub fn new(width: u32, height: u32) -> GameOfLife {
        let mut cells = Vec::new();
        for _ in 0..width * height {
            cells.push(Cell {
                alive: rand::random(),
            });
        }
        GameOfLife {
            width,
            height,
            cells,
            rule: default_rule,
            color: Color::White,
        }
    }

    pub fn new_with_presets(width: u32, height: u32, presets: Vec<(u32, u32)>) -> GameOfLife {
        let mut map = GameOfLife::new(width, height);
        for (x, y) in presets {
            if x >= width || y >= height {
                continue;
            }
            map.set_cell(x, y, true);
        }
        map
    }

    pub fn set_cell(&mut self, x: u32, y: u32, arg: bool) -> () {
        let index = (y * self.width + x) as usize;
        self.cells[index].alive = arg;
    }

    pub fn get_cell(&self, x: u32, y: u32) -> bool {
        let index = (y * self.width + x) as usize;
        self.cells[index].alive
    }

    fn get_neighbours(&self, x: u32, y: u32) -> u32 {
        let mut neighbours = 0;
        let x = x as i32;
        let y = y as i32;
        for i in x - 1..x + 2 {
            for j in y - 1..y + 2 {
                if i == x && j == y {
                    continue;
                }
                if i < 0 || j < 0 {
                    continue;
                }
                if i as u32 >= self.width || j as u32 >= self.height {
                    continue;
                }
                if self.get_cell(i as u32, j as u32) {
                    neighbours += 1;
                }
            }
        }
        neighbours
    }

    pub fn iterate(&mut self) -> () {
        let mut new_cells = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                new_cells.push(Cell {
                    alive: (self.rule)(self, x, y),
                });
            }
        }
        self.cells = new_cells;
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }
}

impl Shape for GameOfLife {
    fn draw(&self, painter: &mut tui::widgets::canvas::Painter) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get_cell(x, y) {
                    painter.paint(x as usize, y as usize, self.color);
                }
            }
        }
    }
}
