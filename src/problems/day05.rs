use super::super::utils::read_strings_from_file;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct BoardingPass {
    encoded: String,
    binary: String,
    seat_id: isize,
}

impl BoardingPass {
    fn from_str(str: String) -> Self {
        let binary = str
            .chars()
            .map(|c| match c {
                'F' => '0',
                'B' => '1',
                'R' => '1',
                'L' => '0',
                _ => '0',
            })
            .join("");
        let seat_id = isize::from_str_radix(&binary, 2).unwrap();
        Self {
            encoded: str,
            binary,
            seat_id,
        }
    }
}

pub fn solve() {
    let strings = read_strings_from_file("./inputs/day05_1").expect("Failed to read inputs");
    let passes: Vec<BoardingPass> = strings
        .iter()
        .map(|s| BoardingPass::from_str(s.to_string()))
        .collect();
    let highest_seat_id = passes.iter().reduce(|state, pass| {
        if pass.seat_id > state.seat_id {
            pass
        } else {
            state
        }
    });
    let mut sorted_passes = passes.to_vec();
    sorted_passes.sort_by(|a, b| a.seat_id.cmp(&b.seat_id));
    println!("{:?}", passes);
    println!("{:?}", highest_seat_id);
    println!("{:?}, {:?}", sorted_passes.first(), sorted_passes.last());
    let my_seat_window = sorted_passes
        .iter()
        .tuple_windows::<(_, _, _)>()
        .find(|w| (w.0.seat_id < w.1.seat_id - 1) || (w.1.seat_id < w.2.seat_id - 1))
        .unwrap();
    println!("{:?}", my_seat_window);
}
