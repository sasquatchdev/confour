use std::{fmt, hash::{DefaultHasher, Hash, Hasher}, ops::{Index, IndexMut}};

/// Defines which player (in the minimax algorithm)
/// is favored by a high (positive) score.
pub const MAXIMIZER: Player = Player::Red;

/// Defines which player (in the minimax algorithm)
/// is favored by a low (negative) score.
pub const MINIMIZER: Player = Player::Yellow;

pub const ROWS: usize = 6;
pub const COLS: usize = 7;

/// A shallow wrapper around the State struct
/// for the current/active/etc. state of the board
pub struct Board {
    state: State
}

impl Board {
    /// Creates a new (empty) board state and wraps it
    /// in a Board struct
    pub fn new() -> Board {
        Board {
            state: State {
                data: vec![vec![Cell::Empty; COLS]; ROWS]
            }
        }
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut State {
        &mut self.state
    }
}

impl fmt::Display for Board {
    /// Display the board as a grid of cells
    /// (delegates to the State's Display impl)
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.state)
    }
}

/// The state of any connect four board/game. Not necessarily 
/// valid or current
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct State {
    pub data: Vec<Vec<Cell>>
}

impl State {
    /// Create a new state with an empty board
    pub fn new() -> State {
        State {
            data: vec![vec![Cell::Empty; COLS]; ROWS]
        }
    }

    /// A simple wrapper around the `hash` method
    pub fn hash_value(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        return hasher.finish();
    }

    /// Create a new state from a 2D vector of cells
    pub fn from_data(data: Vec<Vec<Cell>>) -> State {
        State {
            data
        }
    }

    /// Returns the playe whose turn it is
    pub fn player(&self) -> Player {
        // Designates the first player as Player::Red
        if self.moves() % 2 == 0 {
            Player::Red
        } else {
            Player::Yellow
        }
    }

    /// Returns the number of moves made so far
    pub fn moves(&self) -> usize {
        self.data.iter().flatten().filter(|cell| !cell.is_empty()).collect::<Vec<_>>().len()
    }

    /// Returns true if the board is full
    pub fn is_full(&self) -> bool {
        self.data.iter().all(|row| row.iter().all(|cell| *cell != Cell::Empty))
    }

    /// Retrusn whether the game is over
    /// (i.e. the board is full or there is a winner)
    pub fn is_terminal(&self) -> bool {
        return self.is_full() || self.get_winner().is_some();
    }

    /// Returns true if dropping a piece into the given
    /// column is a valid move
    pub fn is_valid(&self, col: usize) -> bool {
        col < COLS && self[(0, col)] == Cell::Empty
    }

    /// Returns all the possible columns that a piece
    /// can be dropped into
    pub fn get_valid(&self) -> Vec<usize> {
        (0..COLS).filter(|&col| self.is_valid(col)).collect()
    }

    /// Drop a piece into the given column and update
    /// &mut self state
    pub fn drop(&mut self, col: usize, player: Player) {
        for row in (0..ROWS).rev() {
            if self[(row, col)] == Cell::Empty {
                self[(row, col)] = Cell::Player { player };
                break;
            }
        }
    }

    /// Create a new state with the given column dropped
    /// and return it
    pub fn dropped(&self, col: usize, player: Player) -> State {
        let mut new_state = self.clone();
        new_state.drop(col, player);
        new_state
    }
}

impl Index<(usize, usize)> for State {
    type Output = Cell;

    /// Useful for getting the value of a cell
    /// Note: Unusual syntax, `state[(row, col)]` instead
    ///       of `state[row][col]`
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl IndexMut<(usize, usize)> for State {
    /// Useful for setting the value of a cell
    /// Note: Unusual syntax, `state[(row, col)]` instead
    ///       of `state[row][col]`
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}

impl fmt::Display for State {
    /// Display the board as a grid of cells
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for row in self.data.iter() {
            for cell in row.iter() {
                s.push_str(&format!("{} ", cell));
            }
            s.push_str("\n");
        }
        write!(f, "{}", s)
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        let mut i = 0;
        for row in self.data.iter() {
            for cell in row.iter() {
                let fmt_cell = match cell {
                    Cell::Empty => &format!("{}", i),
                    Cell::Player { player } => match player {
                        Player::Red => "R",
                        Player::Yellow => "y"
                    }
                };

                s.push_str(fmt_cell);
            }
            s.push_str("\n");
            i += 1;
        }
        write!(f, "{}", s)
    }
}

/// The state of any given cell (or "position") on the board
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Cell {
    Empty,                      // No player occupies the cell
    Player { player: Player }   // A player occupies the cell
}

impl Cell {
    /// Returns the player occupying the cell, if any
    pub fn player(&self) -> Option<Player> {
        match self {
            Cell::Empty => None,
            Cell::Player { player } => Some(*player)
        }
    }

    /// Returns true if the cell is empty
    pub fn is_empty(&self) -> bool {
        match self {
            Cell::Empty => true,
            _ => false
        }
    }
}

impl fmt::Display for Cell {
    /// Display the cell as a single character for
    /// visualizing the board in a CLI
    /// "·" or "R" or "y"
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, "·"),
            Cell::Player { player } => match player {
                Player::Red => write!(f, "R"),
                Player::Yellow => write!(f, "y")
            }
        }
    }
}

/// An enum representing all possible players
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Player {
    Red,        // Player 1
    Yellow      // Player 2
}

impl Player {
    /// Return the "other player" (i.e. the opponent)
    pub fn other(&self) -> Player {
        match self {
            Player::Red => Player::Yellow,
            Player::Yellow => Player::Red
        }
    }

    /// Returns true if the given player is the opponent
    /// of the &self player
    pub fn is_opponent(&self, other: Player) -> bool {
        self.other() == other
    }

    /// Returns true if the player is the maximizer
    /// defined in the `MAXIMIZER` constant
    pub fn is_maximizer(&self) -> bool {
        *self == MAXIMIZER
    }

    /// Returns true if the player is the minimizer
    /// defined in the `MINIMIZER` constant
    pub fn is_minimizer(&self) -> bool {
        *self == MINIMIZER
    }
}

impl fmt::Display for Player {
    /// Display the full player string (e.g. "P1 - Red")
    /// P{index} - {color}
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::Red => write!(f, "P1 - Red"),
            Player::Yellow => write!(f, "P2 - Yellow")
        }
    }
}

/// Unit and integration tests for logic
/// concerning the board and its state
#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    /// Tests the `from_data` method of the State struct,
    /// kind of a trivial test but it's a start
    fn test_from_data() {
        let data = vec![vec![Cell::Empty; COLS]; ROWS];
        let state = State::from_data(data.clone());
        
        assert_eq!(
            state.data, data,
            "Expected the data to be the same, but it wasn't",
        );
        
        assert_eq!(
            ROWS, state.data.len(),
            "Expected the number of rows to be {}, but it wasn't",
            ROWS,
        );
        
        assert_eq!(
            COLS, state.data[0].len(),
            "Expected the number of columns to be {}, but it wasn't",
            COLS,
        );
    }

    #[test]
    /// Tests the `is_full` method of the State struct
    /// both positive and negative cases
    fn test_is_full() {
        let mut state = State::new();
        
        assert!(
            !state.is_full(),
            "Expected an empty board to not be full, but it was",
        );
        
        for col in 0..COLS {
            for row in 0..ROWS {
                state[(row, col)] = Cell::Player { player: Player::Red };
            }
        }
        assert!(
            state.is_full(),
            "Expected a full board to be full, but it wasn't",
        );
    }

    #[test]
    /// Tests the `is_valid` method of the State struct
    /// by filling up the board and checking if the columns
    /// are still valid
    fn test_is_valid() {
        let mut state = State::new();
        
        for col in 0..COLS {
            assert!(
                state.is_valid(col),
                "Expected an empty board to result in all valid columns, but it didn't",
            );
        }
        
        for row in 0..ROWS {
            for col in 0..COLS {
                state[(row, col)] = Cell::Player { player: Player::Red };
            }
        }
        for col in 0..COLS {
            assert!(
                !state.is_valid(col),
                "Expected a full board to result in no valid columns, but it didn't",
            );
        }
    }

    #[test]
    /// Tests the `drop` method of the State struct
    /// by dropping a piece into a column and checking
    /// if it landed in the correct row
    fn test_drop() {
        let mut state = State::new();
        
        state.drop(0, Player::Red);
        assert_eq!(
            state[(ROWS - 1, 0)], Cell::Player { player: Player::Red },
            "Expected player 1 to be in the last row of the column, but it wasn't",
        );
        
        state.drop(0, Player::Yellow);
        assert_eq!(
            state[(ROWS - 2, 0)], Cell::Player { player: Player::Yellow },
            "Expected player 2 to be in the second to last row of the column, but it wasn't",
        );
    }

    #[test]
    fn test_dropped() {
        let state = State::new();
        let new_state = state.dropped(0, Player::Red);

        assert_eq!(
            new_state[(ROWS - 1, 0)], Cell::Player { player: Player::Red },
            "Expected player 1 to be in the last row of the column, but it wasn't",
        );
    }

    #[test]
    fn test_index() {
        let state = State::new();
        assert_eq!(
            state[(0, 0)], Cell::Empty,
            "Expected the cell at (0, 0) to be accessible yet empty, but it wasn't",
        );
    }

    #[test]
    fn test_index_mut() {
        let mut state = State::new();
        state[(0, 0)] = Cell::Player { player: Player::Red };
        assert_eq!(
            state[(0, 0)], Cell::Player { player: Player::Red },
            "Expected the cell at (0, 0) to be player 1, but it wasn't",
        );
    }

    #[test]
    fn test_cell_player() {
        let player_cell = Cell::Player { player: Player::Red };
        let empty_cell = Cell::Empty;

        assert!(
            player_cell.player().is_some(),
            "Expected a player cell to have a player, but it didn't",
        );

        assert_eq!(
            player_cell.player().unwrap(), Player::Red,
            "Expected the player to be player 1 / red, but it wasn't",
        );

        assert!(
            empty_cell.player().is_none(),
            "Expected an empty cell to not have a player, but it did",
        );

        assert!(
            empty_cell.is_empty(),
            "Expected an empty cell to be empty, but it wasn't",
        );
    }

    #[test]
    fn test_other() {
        assert_eq!(
            Player::Red.other(), Player::Yellow,
            "Expected player 1's opponent to be player 2, but it wasn't",
        );

        assert_eq!(
            Player::Yellow.other(), Player::Red,
            "Expected player 2's opponent to be player 1, but it wasn't",
        );
    }

    #[test]
    fn test_is_opponent() {
        assert!(
            Player::Red.is_opponent(Player::Yellow),
            "Expected player 1 to be player 2's opponent, but it wasn't",
        );

        assert!(
            Player::Yellow.is_opponent(Player::Red),
            "Expected player 2 to be player 1's opponent, but it wasn't",
        );
    }

    #[test]
    fn test_is_maximizer() {
        assert!(
            Player::Red.is_maximizer(),
            "Expected player 1 to be the maximizer, but it wasn't",
        );

        assert!(
            !Player::Yellow.is_maximizer(),
            "Expected player 2 to not be the maximizer, but it was",
        );
    }

    #[test]
    fn test_is_minimizer() {
        assert!(
            !Player::Red.is_minimizer(),
            "Expected player 1 to not be the minimizer, but it was",
        );

        assert!(
            Player::Yellow.is_minimizer(),
            "Expected player 2 to be the minimizer, but it wasn't",
        );
    }

    #[test]
    fn test_moves() {
        let mut state = State::new();
        assert_eq!(
            state.moves(), 0,
            "Expected the number of moves to be 0, but it wasn't",
        );

        state.drop(0, Player::Red);
        assert_eq!(
            state.moves(), 1,
            "Expected the number of moves to be 1, but it wasn't",
        );

        state.drop(0, Player::Yellow);
        assert_eq!(
            state.moves(), 2,
            "Expected the number of moves to be 2, but it wasn't",
        );
    }

    #[test]
    fn test_state_player() {
        let state = State::new();
        assert_eq!(
            state.player(), Player::Red,
            "Expected player 1 to be the first player, but it wasn't",
        );

        let mut state = State::new();
        state.drop(0, Player::Red);
        assert_eq!(
            state.player(), Player::Yellow,
            "Expected player 2 to be the second player, but it wasn't",
        );
    }
}
