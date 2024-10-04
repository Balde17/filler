
pub fn can_place_piece(board: &Vec<Vec<char>>, piece: &Vec<Vec<char>>, x: usize, y: usize, enemy: char, enemy2: char, p1: char, p2: char) -> bool {
    let mut overlap_count = 0;

    for i in 0..piece.len() {
        for j in 0..piece[i].len() {
            let board_x = x + i;
            let board_y = y + j;

            if board_x >= board.len() || board_y >= board[0].len() {
                if piece[i][j] != '.' {
                    return false;
                }
                continue;
            }
            if piece[i][j] != '.' && (board[board_x][board_y] == enemy || board[board_x][board_y] == enemy2)
            {
                return false;
            }
            if piece[i][j] != '.' && (board[board_x][board_y] == p1 || board[board_x][board_y] == p2)
            {
                overlap_count += 1;
            }
        }
    }
    overlap_count == 1
}
//Placement de la piece
pub fn place_piece_on_board( board: &Vec<Vec<char>>, piece: &Vec<Vec<char>>, player: &String) -> Vec<(usize, usize)> {
    let enemy: char;
    let enemy2: char;
    let p1: char;
    let p2: char;
    let mut valid_positions = Vec::new();

    if player == "p1" {
        enemy = 's';
        enemy2 = '$';
        p1 = '@';
        p2 = 'a';
    } else {
        enemy = 'a';
        enemy2 = '@';
        p1 = 's';
        p2 = '$';
    }

    for x in 0..board.len() {
        for y in 0..board[0].len() {
            if can_place_piece(&board, &piece, x, y, enemy, enemy2, p1, p2) {
                valid_positions.push((x, y));
            }
        }
    }
    valid_positions
}

//La distance entre deux points pour trouver la plus petite possible
pub fn distance(x1: usize, y1: usize, x2: usize, y2: usize) -> f64 {
    let dx = (x2 as f64) - (x1 as f64);
    let dy = (y2 as f64) - (y1 as f64);
    (dx * dx + dy * dy).sqrt()
}
