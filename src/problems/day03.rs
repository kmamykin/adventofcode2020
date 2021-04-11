use super::super::utils::read_strings_from_file;
use std::io::Empty;
use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Coordinates {
    x: usize,
    y: usize,
}

impl Coordinates {
    fn wrap_around_x(&self, size: usize) -> Coordinates {
        Coordinates {
            x: self.x % size,
            y: self.y,
        }
    }
}
impl ops::Add<Coordinates> for Coordinates {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

fn P(x: usize, y: usize) -> Coordinates {
    Coordinates { x, y }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Cell {
    Empty,
    Tree,
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
}

impl Map {
    fn empty(width: usize, height: usize) -> Map {
        let cells = vec![vec![Cell::Empty; width]; height];
        Map {
            width,
            height,
            cells,
        }
    }

    fn put(&mut self, c: Coordinates, cell: Cell) {
        self.cells[c.y][c.x] = cell;
    }

    fn get(&self, c: Coordinates) -> Cell {
        let p = c.wrap_around_x(self.width);
        self.cells[p.y][p.x]
    }

    fn from_strings(strings: &Vec<String>) -> Map {
        let width = strings[0].len();
        let height = strings.len();
        let mut map = Map::empty(width, height);
        for (y, s) in strings.iter().enumerate() {
            for (x, c) in s.chars().enumerate() {
                let content = match c {
                    '.' => Cell::Empty,
                    '#' => Cell::Tree,
                    _ => Cell::Empty,
                };
                map.put(P(x, y), content);
            }
        }
        map
    }
}

fn count_trees_on_the_way_down_slope(map: &Map, slope: Coordinates) -> usize {
    let mut position = P(0, 0);
    let mut count = 0;
    while position.y < map.height {
        match map.get(position) {
            Cell::Tree => count = count + 1,
            _ => (),
        }
        position = position + slope;
    }
    return count;
}
pub fn day03() {
    let strings = read_strings_from_file("./inputs/day03_1").expect("Failed to read inputs");
    let mut map = Map::from_strings(&strings);
    println!("{:?}", strings);
    println!("{:?}", map);
    let count1 = count_trees_on_the_way_down_slope(&map, P(1, 1));
    let count2 = count_trees_on_the_way_down_slope(&map, P(3, 1));
    let count3 = count_trees_on_the_way_down_slope(&map, P(5, 1));
    let count4 = count_trees_on_the_way_down_slope(&map, P(7, 1));
    let count5 = count_trees_on_the_way_down_slope(&map, P(1, 2));
    println!("{:?}", count1 * count2 * count3 * count4 * count5);
}
