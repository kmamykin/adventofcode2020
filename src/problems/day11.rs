use super::super::utils::{read_strings_from_file};
use nalgebra::{DMatrix};
use std::fmt;
use itertools::Itertools;
use std::collections::HashSet;

pub fn solve() {
    let strings = read_strings_from_file("./inputs/day11_1").expect("Failed to read inputs");
    println!("Problem 1: {:?}", problem_1(&strings));
    println!("Problem 2: {:?}", problem_2(&strings));
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
enum Seat {
    Floor = 0,
    Empty = 1,
    Occupied = 2,
}

type SeatGrid = DMatrix<Seat>;

#[derive(Debug)]
struct SeatLayout {
    grid: SeatGrid,
}

impl SeatLayout {
    fn from_input(strings: &Vec<String>) -> Self {
        let first_string = strings.first().unwrap();
        let mut grid = SeatGrid::from_element(strings.len(), first_string.len(), Seat::Floor);
        for (r, row) in strings.iter().enumerate() {
            for (c, ch) in row.chars().enumerate() {
                grid[(r, c)] = match ch {
                    'L' => Seat::Empty,
                    '#' => Seat::Occupied,
                    _ => Seat::Floor,
                }
            }
        }
        Self { grid }
    }

    // fn cells(&self) -> impl Iterator<Item=(usize, usize, Seat)>  {
    //     self.grid.row_iter()
    //         .enumerate()
    //         .flat_map(|(r, row)|
    //             row.iter().enumerate().map(move |(c, seat)| (r, c, seat)).cloned()
    //         )
    // }

    fn number_of_occupied_seats_around(&self, r: usize, c: usize) -> usize {
        let (nrows, ncols) = self.grid.shape();
        let mut directions: HashSet<(i8, i8)> = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ].iter().cloned().collect();
        if r == 0 {
            directions.remove(&(-1, -1));
            directions.remove(&(-1, 0));
            directions.remove(&(-1, 1));
        }
        if r == nrows - 1 {
            directions.remove(&(1, -1));
            directions.remove(&(1, 0));
            directions.remove(&(1, 1));
        }
        if c == 0 {
            directions.remove(&(-1, -1));
            directions.remove(&(0, -1));
            directions.remove(&(1, -1));
        }
        if c == ncols - 1 {
            directions.remove(&(-1, 1));
            directions.remove(&(0, 1));
            directions.remove(&(1, 1));
        }
        let n: usize  = directions.into_iter()
            .map(|d| ((r as i8 + d.0) as usize, (c as i8 + d.1) as usize))
            .map(|d| self.grid[d] )
            .map(|s| match s { Seat::Occupied => 1usize, _ => 0usize })
            .sum();
        n
    }

    fn next_generation_cell(&self, r: usize, c: usize) -> Seat {
        match self.grid[(r, c)] {
            Seat::Empty => {
                if self.number_of_occupied_seats_around(r, c) == 0 { Seat::Occupied } else { Seat::Empty }
            },
            Seat::Occupied => {
                if self.number_of_occupied_seats_around(r, c) >= 4 { Seat::Empty } else { Seat::Occupied }
            },
            _ => {
                self.grid[(r, c)]
            },
        }
    }

    fn next_generation(&self) -> Self {
        let mut other = Self { grid: self.grid.clone() };
        let (nrows, ncols) = self.grid.shape();
        for r in 0..nrows {
            for c in 0..ncols {
                other.grid[(r, c)] = self.next_generation_cell(r, c);
            }
        }
        other
    }

    fn invert(&self) -> Self {
        let mut other = Self { grid: self.grid.clone() };
        let (nrows, ncols) = self.grid.shape();
        for r in 0..nrows {
            for c in 0..ncols {
                other.grid[(r, c)] = match self.grid[(r, c)] {
                    Seat::Empty => Seat::Occupied ,
                    Seat::Occupied => Seat::Empty,
                    _ => self.grid[(r, c)],
                }
            }
        }
        other
    }

    fn number_of_occupied_seats(&self) -> usize {
        let mut n = 0;
        let (nrows, ncols) = self.grid.shape();
        for r in 0..nrows {
            for c in 0..ncols {
                match self.grid[(r, c)] {
                    Seat::Occupied => { n += 1; },
                    _ => {},
                }
            }
        }
        n
    }
}

impl fmt::Display for SeatLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.grid.row_iter() {
            let line = row.iter()
                .map(|s| match s {
                    Seat::Floor => '.',
                    Seat::Empty => 'L',
                    Seat::Occupied => '#',
                })
                .join("");
            write!(f, "{:}\n", line)?;
        }
        Ok(())
    }
}

pub fn problem_1(strings: &Vec<String>) -> usize {
    println!("{:?}", strings);
    let mut layout = SeatLayout::from_input(strings);
    // println!("{:}", layout);
    loop {
        let other = layout.next_generation();
        // println!("{:}", other);
        if layout.to_string() == other.to_string() {
            break;
        }
        layout = other;
    }
    layout.number_of_occupied_seats()
}

pub fn problem_2(strings: &Vec<String>) -> u64 {
    println!("{:?}", strings);
    2
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_of_occupied_seats_around() {
        let strings: Vec<String> = vec![
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let layout = SeatLayout::from_input(&strings);
        assert_eq!(0, layout.number_of_occupied_seats_around(0, 0));
        assert_eq!(0, layout.number_of_occupied_seats_around(9, 9));
        let other = layout.invert();
        assert_eq!(2, other.number_of_occupied_seats_around(0, 0));
        assert_eq!(2, other.number_of_occupied_seats_around(9, 9));
        assert_eq!(3, other.number_of_occupied_seats_around(0, 9));
        assert_eq!(1, other.number_of_occupied_seats_around(9, 0));
        assert_eq!(6, other.number_of_occupied_seats_around(4, 5));
    }

    #[test]
    fn step2() {
        let strings: Vec<String> = vec![
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let layout = SeatLayout::from_input(&strings);
        let step1 = layout.next_generation();
        println!("step1\n{:}", step1);
        let step2 = step1.next_generation();
        println!("step2\n{:}", step2);
        assert_eq!(1, step2.number_of_occupied_seats_around(0, 0));
        assert_eq!(1, step2.number_of_occupied_seats_around(9, 9));
        assert_eq!(2, step2.number_of_occupied_seats_around(0, 9));
        assert_eq!(1, step2.number_of_occupied_seats_around(9, 0));
        assert_eq!(1, step2.number_of_occupied_seats_around(4, 5));
    }

    #[test]
    fn example_1() {
        let strings: Vec<String> = vec![
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(37, problem_1(&strings));
    }
}
