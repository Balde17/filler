use std::io;
use std::io::Write;
use std::io::BufRead;

mod algorithme;
use algorithme::*;

fn main() {
    let stdin = io::stdin();
    let mut anfield: Vec<Vec<char>> = Vec::new();
    let mut piece: Vec<Vec<char>> = Vec::new();
    let mut p: String = String::new();

    let mut lines = stdin.lock().lines();
    loop {
        while let Some(result) = lines.next() {
            let line = result.expect("Failed to read line");

            if line.starts_with("$$$") {
                let p_parts: Vec<&str> = line.split_whitespace().collect();
                let player: String = p_parts[2].parse().expect("Failed to parse player");
                p = player;
            }

            if line.starts_with("Anfield") {
                let an_parts: Vec<&str> = line.split_whitespace().collect();
                let an_height: usize = an_parts[2]
                    .trim_end_matches(':')
                    .parse()
                    .expect("Failed to parse height");

                //Sauter la ligne qui contient que des nombres
                let _ = lines.next().expect("Failed to skip numbers line");

                for _ in 0..an_height {
                    if let Some(board_line_result) = lines.next() {
                        let board_line = board_line_result.expect("Failed to read board line");
                        let chars: Vec<char> = board_line[4..].chars().collect();
                        anfield.push(chars);
                    }
                }
            }

            if line.starts_with("Piece") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let height: usize = parts[2]
                    .trim_end_matches(':')
                    .parse()
                    .expect("Failed to parse height");

                for _ in 0..height {
                    if let Some(piece_line_result) = lines.next() {
                        let piece_line = piece_line_result.expect("Failed to read piece line");
                        let chars: Vec<char> = piece_line.chars().collect();
                        piece.push(chars);
                    }
                }

                let enemy: char;
                let enemy2: char;
                if p == "p1" {
                    enemy = 's';
                    enemy2 = '$';
                } else {
                    enemy = 'a';
                    enemy2 = '@';
                }

                let valid_positions = place_piece_on_board(&anfield, &piece, &p);
                let enemy_positions: Vec<_> = anfield
                    .iter()
                    .enumerate()
                    .flat_map(|(i, row)| {
                        row.iter()
                            .enumerate()
                            .filter(|&(_, &ch)| (ch == enemy || ch == enemy2))
                            .map(move |(j, _)| (i, j))
                    })
                    .collect();

                if !valid_positions.is_empty() {
                    let (chosen_x, chosen_y) = valid_positions
                        .into_iter()
                        .min_by(|&(x1, y1), &(x2, y2)| {
                            let min_distance1 = enemy_positions
                                .iter()
                                .map(|&(ex, ey)| distance(x1, y1, ex, ey))
                                .min_by(|&a, &b|
                                    a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal)
                                )
                                .unwrap_or(f64::INFINITY);
                            let min_distance2 = enemy_positions
                                .iter()
                                .map(|&(ex, ey)| distance(x2, y2, ex, ey))
                                .min_by(|&a, &b|
                                    a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal)
                                )
                                .unwrap_or(f64::INFINITY);
                            min_distance1
                                .partial_cmp(&min_distance2)
                                .unwrap_or(std::cmp::Ordering::Equal)
                        })
                        .unwrap();

                    println!("{} {}", chosen_y, chosen_x);
                    io::stdout().flush().unwrap();
                    piece.clear();
                    anfield.clear();
                } else {
                    println!("0 0");
                    io::stdout().flush().unwrap();
                    piece.clear();
                    anfield.clear();
                }
            }
        }
    }
}
