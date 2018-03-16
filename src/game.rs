/*
1. Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
2. Any live cell with two or three live neighbours lives on to the next generation.
3. Any live cell with more than three live neighbours dies, as if by overpopulation.
4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
 */

struct Board {
    board: Vec<Vec<Cell>>,
    x: usize,
    y: usize,
}

struct Cell {
    living: bool,
    x: usize,
    y: usize,
    neighbors: Vec<Cell>,
}

fn populate_board (x: usize, y: usize) -> Board {
    let mut board: Vec<Vec<Cell>>;

    for x_coord in 0..x {
        for y_coord in 0..y {
            // let neighbors = vec![ // As a cricle, starting at 0, go around positive
            //     board[x_coord + 1][y_coord + 0],
            //     board[x_coord + 1][y_coord + 1],
            //     board[x_coord + 0][y_coord + 1],
            //     board[x_coord - 1][y_coord + 1],
            //     board[x_coord - 1][y_coord + 0],
            //     board[x_coord - 1][y_coord - 1],
            //     board[x_coord + 0][y_coord - 1],
            //     board[x_coord + 1][y_coord - 1],
            // ];

            board[x_coord][y_coord] = Cell {
                living: false,
                x: x_coord,
                y: y_coord,
                neighbors: vec![],
            };
        }
    }

    return Board {
        board: board,
        x: x,
        y: y,
    };
}

fn main() {
    let board: Board = populate_board(16, 16);
}
