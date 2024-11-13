use std::mem;

pub struct Game {
    size: BoardSize,

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
            size: BoardSize { x_size, y_size },
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
    pub fn iterate(&mut self) {
        // Swap the boards so we can work off the current coming into this iteration.
        mem::swap(&mut self.current, &mut self.previous);

        // Clear the current board (now that we've saved the current state to previous).
        clear_board(&mut self.current);

        for x in 0..self.size.x_size {
            for y in 0..self.size.y_size {
                self.current[x][y] = self.is_live(x, y);
            }
        }
    }

    fn is_live(&self, x: usize, y: usize) -> bool {
        let mut live_neighbors = 0;

        let (check_x, check_y) = self.size.top_left(x, y);
        if self.previous[check_x][check_y] {
            live_neighbors += 1;
        }

        let (check_x, check_y) = self.size.top(x, y);
        if self.previous[check_x][check_y] {
            live_neighbors += 1;
        }

        let (check_x, check_y) = self.size.top_right(x, y);
        if self.previous[check_x][check_y] {
            live_neighbors += 1;
        }

        let (check_x, check_y) = self.size.left(x, y);
        if self.previous[check_x][check_y] {
            live_neighbors += 1;
        }

        let (check_x, check_y) = self.size.right(x, y);
        if self.previous[check_x][check_y] {
            live_neighbors += 1;
        }

        let (check_x, check_y) = self.size.bottom_left(x, y);
        if self.previous[check_x][check_y] {
            live_neighbors += 1;
        }

        let (check_x, check_y) = self.size.bottom(x, y);
        if self.previous[check_x][check_y] {
            live_neighbors += 1;
        }

        let (check_x, check_y) = self.size.bottom_right(x, y);
        if self.previous[check_x][check_y] {
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

    pub fn clear(&mut self) {
        clear_board(&mut self.current);
        clear_board(&mut self.previous);
    }

    // Set up a fresh state, clearing any previous state from the game board.
    pub fn clear_and_set<I: Iterator<Item = (usize, usize)>>(&mut self, pairs: I) {
        clear_board(&mut self.current);
        clear_board(&mut self.previous);

        for (x, y) in pairs {
            if x > self.size.x_size || y > self.size.y_size {
                panic!("unexpected input coordinate");
            }

            self.current[x][y] = true
        }
    }

    pub fn set<I: Iterator<Item = (usize, usize)>>(&mut self, pairs: I) {
        for (x, y) in pairs {
            if x > self.size.x_size || y > self.size.y_size {
                panic!("unexpected input coordinate");
            }

            self.current[x][y] = true
        }
    }

    pub fn cell(&mut self, x: usize, y: usize) -> &mut bool {
        &mut self.current[x][y]
    }

    pub fn x_size(&self) -> usize {
        self.size.x_size
    }

    pub fn y_size(&self) -> usize {
        self.size.y_size
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

struct BoardSize {
    x_size: usize,
    y_size: usize,
}

impl BoardSize {
    fn top_left(&self, x: usize, y: usize) -> (usize, usize) {
        // translate (-1, -1)
        (
            if x == 0 { self.x_size - 1 } else { x - 1 },
            if y == 0 { self.y_size - 1 } else { y - 1 },
        )
    }

    fn top(&self, x: usize, y: usize) -> (usize, usize) {
        // translate (0, -1)
        (x, if y == 0 { self.y_size - 1 } else { y - 1 })
    }

    fn top_right(&self, x: usize, y: usize) -> (usize, usize) {
        // translate (1, -1)
        (
            if x == self.x_size - 1 { 0 } else { x + 1 },
            if y == 0 { self.y_size - 1 } else { y - 1 },
        )
    }

    fn left(&self, x: usize, y: usize) -> (usize, usize) {
        // translate (-1, 0)
        (if x == 0 { self.x_size - 1 } else { x - 1 }, y)
    }

    fn right(&self, x: usize, y: usize) -> (usize, usize) {
        // translate (1, 0)
        (if x == self.x_size - 1 { 0 } else { x + 1 }, y)
    }

    fn bottom_left(&self, x: usize, y: usize) -> (usize, usize) {
        // translate (-1, 1)
        (
            if x == 0 { self.x_size - 1 } else { x - 1 },
            if y == self.y_size - 1 { 0 } else { y + 1 },
        )
    }

    fn bottom(&self, x: usize, y: usize) -> (usize, usize) {
        // translate (0, 1)
        (x, if y == self.y_size - 1 { 0 } else { y + 1 })
    }

    fn bottom_right(&self, x: usize, y: usize) -> (usize, usize) {
        // translate (1, 1)
        (
            if x == self.x_size - 1 { 0 } else { x + 1 },
            if y == self.y_size - 1 { 0 } else { y + 1 },
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_board_size_top_left() {
        let size = BoardSize {
            x_size: 2,
            y_size: 2,
        };
        assert_eq!((1, 1), size.top_left(0, 0));
        assert_eq!((0, 0), size.top_left(1, 1));
        assert_eq!((0, 1), size.top_left(1, 0));
        assert_eq!((1, 0), size.top_left(0, 1));
    }

    #[test]
    fn test_board_size_top() {
        let size = BoardSize {
            x_size: 2,
            y_size: 2,
        };
        assert_eq!((0, 1), size.top(0, 0));
        assert_eq!((1, 0), size.top(1, 1));
        assert_eq!((1, 1), size.top(1, 0));
        assert_eq!((0, 0), size.top(0, 1));
    }

    #[test]
    fn test_board_size_top_right() {
        let size = BoardSize {
            x_size: 2,
            y_size: 2,
        };
        assert_eq!((1, 1), size.top_right(0, 0));
        assert_eq!((0, 0), size.top_right(1, 1));
        assert_eq!((0, 1), size.top_right(1, 0));
        assert_eq!((1, 0), size.top_right(0, 1));
    }

    #[test]
    fn test_board_size_left() {
        let size = BoardSize {
            x_size: 2,
            y_size: 2,
        };
        assert_eq!((1, 0), size.left(0, 0));
        assert_eq!((0, 1), size.left(1, 1));
        assert_eq!((0, 0), size.left(1, 0));
        assert_eq!((1, 1), size.left(0, 1));
    }

    #[test]
    fn test_board_size_right() {
        let size = BoardSize {
            x_size: 2,
            y_size: 2,
        };
        assert_eq!((1, 0), size.left(0, 0));
        assert_eq!((0, 1), size.left(1, 1));
        assert_eq!((0, 0), size.left(1, 0));
        assert_eq!((1, 1), size.left(0, 1));
    }

    #[test]
    fn test_board_size_bottom_left() {
        let size = BoardSize {
            x_size: 2,
            y_size: 2,
        };
        assert_eq!((1, 1), size.bottom_left(0, 0));
        assert_eq!((0, 0), size.bottom_left(1, 1));
        assert_eq!((0, 1), size.bottom_left(1, 0));
        assert_eq!((1, 0), size.bottom_left(0, 1));
    }

    #[test]
    fn test_board_size_bottom() {
        let size = BoardSize {
            x_size: 2,
            y_size: 2,
        };
        assert_eq!((0, 1), size.bottom(0, 0));
        assert_eq!((1, 0), size.bottom(1, 1));
        assert_eq!((1, 1), size.bottom(1, 0));
        assert_eq!((0, 0), size.bottom(0, 1));
    }

    #[test]
    fn test_board_size_bottom_right() {
        let size = BoardSize {
            x_size: 2,
            y_size: 2,
        };
        assert_eq!((1, 1), size.bottom_right(0, 0));
        assert_eq!((0, 0), size.bottom_right(1, 1));
        assert_eq!((0, 1), size.bottom_right(1, 0));
        assert_eq!((1, 0), size.bottom_right(0, 1));
    }

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
