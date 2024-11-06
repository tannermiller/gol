use std::mem;

pub struct Game {
    x_size: usize,
    y_size: usize,

    // previous and current are the game boards. In order to make (x,y) coordinates make sense with
    // the nested vectors as vec[x][y] then the outer vec represents the columns (x values) and the
    // inner vec represents the rows (y values).
    previous: Vec<Vec<bool>>,
    current: Vec<Vec<bool>>,
}

impl Game {
    // Build a new, empty game board.
    pub fn new(x_size: usize, y_size: usize) -> Self {
        Game {
            x_size,
            y_size,
            previous: make_board(x_size, y_size),
            current: make_board(x_size, y_size),
        }
    }

    // Run the simulation for a set number of iterations.
    pub fn run(&mut self, iters: usize) {
        println!("running with {iters} iterations!");
        for _ in 0..iters {
            self.iterate()
        }
    }

    // Run a single iteration of the game.
    fn iterate(&mut self) {
        // Swap the boards so we can work off the current coming into this iteration.
        mem::swap(&mut self.current, &mut self.previous);

        // Clear the current board (now that we've saved the current state to previous).
        clear_board(&mut self.current);

        for x in 0..self.x_size {
            for y in 0..self.y_size {
                self.current[x][y] = self.is_live(x, y);
            }
        }
    }

    fn is_live(&self, x: usize, y: usize) -> bool {
        let mut live_neighbors = 0;

        // translate (-1, -1)
        let top_left_x = if x == 0 { self.x_size - 1 } else { x - 1 };
        let top_left_y = if y == 0 { self.y_size - 1 } else { y - 1 };
        if self.previous[top_left_x][top_left_y] {
            live_neighbors += 1;
        }

        // translate (0, -1)
        let top_x = x;
        let top_y = if y == 0 { self.y_size - 1 } else { y - 1 };
        if self.previous[top_x][top_y] {
            live_neighbors += 1;
        }

        // translate (1, -1)
        let top_right_x = if x == self.x_size - 1 { 0 } else { x + 1 };
        let top_right_y = if y == 0 { self.y_size - 1 } else { y - 1 };
        if self.previous[top_right_x][top_right_y] {
            live_neighbors += 1;
        }

        // translate (-1, 0)
        let left_x = if x == 0 { self.x_size - 1 } else { x - 1 };
        let left_y = y;
        if self.previous[left_x][left_y] {
            live_neighbors += 1;
        }

        // translate (1, 0)
        let right_x = if x == self.x_size - 1 { 0 } else { x + 1 };
        let right_y = y;
        if self.previous[right_x][right_y] {
            live_neighbors += 1;
        }

        // translate (-1, 1)
        let below_left_x = if x == 0 { self.x_size - 1 } else { x - 1 };
        let below_left_y = if y == self.y_size - 1 { 0 } else { y + 1 };
        if self.previous[below_left_x][below_left_y] {
            live_neighbors += 1;
        }

        // translate (0, 1)
        let below_x = x;
        let below_y = if y == self.y_size - 1 { 0 } else { y + 1 };
        if self.previous[below_x][below_y] {
            live_neighbors += 1;
        }

        // translate (1, 1)
        let below_right_x = if x == self.x_size - 1 { 0 } else { x + 1 };
        let below_right_y = if y == self.y_size - 1 { 0 } else { y + 1 };
        if self.previous[below_right_x][below_right_y] {
            live_neighbors += 1;
        }

        let is_live = self.previous[x][y];
        match (is_live, live_neighbors) {
            (true, 0 | 1) => false, // live with <2 neighbors dies
            (true, 2 | 3) => true,  // live with 2 or 3 neighbors lives
            (false, 3) => true,     // dead with 3 neighbors lives
            _ => false,             // everything else dies
        }
    }

    // Set up a fresh state, clearing any previous state from the game board.
    pub fn set<I: Iterator<Item = (usize, usize)>>(&mut self, pairs: I) {
        clear_board(&mut self.current);
        clear_board(&mut self.previous);

        for (x, y) in pairs {
            if x > self.x_size || y > self.y_size {
                panic!("unexpected input coordinate");
            }

            self.current[x][y] = true
        }
    }
}

fn make_board(x_size: usize, y_size: usize) -> Vec<Vec<bool>> {
    let mut board = Vec::with_capacity(x_size);
    for _ in 0..x_size {
        board.push(vec![false; y_size]);
    }
    board
}

fn clear_board(board: &mut [Vec<bool>]) {
    for col in board {
        for cell in col {
            *cell = false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_make_board() {
        let board = make_board(1, 1);
        assert_eq!(1, board.len());
        assert_eq!(1, board[0].len());
    }

    #[test]
    fn test_clear_board() {
        let mut board = make_board(1, 1);
        board[0][0] = true;
        clear_board(&mut board);
        assert!(!board[0][0]);
    }

    #[test]
    fn test_board_set() {
        let mut game = Game::new(1, 1);
        game.set([(0, 0)].into_iter());
        assert!(!game.previous[0][0]);
        assert!(game.current[0][0]);
    }

    #[test]
    fn test_iterate_block() {
        let mut game = Game::new(4, 4);
        game.set([(1, 1), (2, 1), (1, 2), (2, 2)].into_iter());
        game.iterate();
        assert_eq!(
            &vec![
                // NOTE: this is rotated from the actual board
                vec![false, false, false, false],
                vec![false, true, true, false],
                vec![false, true, true, false],
                vec![false, false, false, false]
            ],
            &game.current
        );
    }

    #[test]
    fn test_iterate_blinker() {
        let mut game = Game::new(5, 5);
        game.set([(2, 1), (2, 2), (2, 3)].into_iter());
        game.iterate();
        assert_eq!(
            &vec![
                vec![false, false, false, false, false],
                vec![false, false, true, false, false],
                vec![false, false, true, false, false],
                vec![false, false, true, false, false],
                vec![false, false, false, false, false]
            ],
            &game.current
        );
    }
}
