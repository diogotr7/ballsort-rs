use crate::constants::BallType;
use rand::Rng;

#[derive(Debug)]
pub struct Puzzle {
    pub vials: Vec<Vec<BallType>>,
    pub n_colors: usize,
    pub n_empty_vials: usize,
    pub n_volume: usize,
}

fn char_to_ball_type(c: char) -> BallType {
    match c {
        '_' => 0,
        'A'.. => c.to_digit(36).unwrap() as BallType,
        '0'..='9' => c.to_digit(10).unwrap() as BallType,
        _ => panic!("Invalid char"),
    }
}

impl Puzzle {
    pub fn new_parse(file_name: &str) -> Puzzle {
        let binding =
            std::fs::read_to_string(file_name).expect("Something went wrong reading the file");
        let lines = binding.lines().collect::<Vec<&str>>();

        let number_of_lines = lines.len();
        let number_of_columns = lines[0].len();

        let mut vials: Vec<Vec<BallType>> = vec![vec![0; number_of_columns]; number_of_lines];
        for i in 0..number_of_lines {
            for j in 0..number_of_columns {
                vials[i][j] = char_to_ball_type(lines[i].chars().nth(j).unwrap());
            }
        }

        let empty_vials = vials.iter().filter(|v| v.iter().all(|b| *b == 0)).count();

        Puzzle {
            vials,
            n_colors: number_of_lines - empty_vials,
            n_empty_vials: empty_vials,
            n_volume: number_of_columns,
        }
    }

    pub fn new_rand(vial_count_full: u8, vial_count_empty: u8, vial_depth: u8) -> Puzzle {
        let vial_count = vial_count_full + vial_count_empty;
        let mut vials: Vec<Vec<BallType>> = Vec::new();
        for i in 0..vial_count_full {
            let mut vial: Vec<BallType> = Vec::new();
            for _ in 0..vial_depth {
                vial.push((i + 1).into());
            }
            vials.push(vial);
        }

        for _ in vial_count_full..vial_count {
            vials.push(vec![0; vial_depth as usize]);
        }

        for i in 1..vial_count_full as usize * vial_depth as usize - 1 {
            let j = rand::thread_rng().gen_range(0..i);

            (
                vials[j / vial_depth as usize][j % vial_depth as usize],
                vials[i / vial_depth as usize][i % vial_depth as usize],
            ) = (
                vials[i / vial_depth as usize][i % vial_depth as usize],
                vials[j / vial_depth as usize][j % vial_depth as usize],
            );
        }

        Puzzle {
            vials,
            n_colors: vial_count_full as usize,
            n_empty_vials: vial_count_empty as usize,
            n_volume: vial_depth as usize,
        }
    }
}
