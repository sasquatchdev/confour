use std::{fmt, ops::{Index, IndexMut}};

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
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct State {
    pub data: Vec<Vec<Cell>>,
}

impl State {
    /// Create a new state with an empty board
    pub fn new() -> State {
        State {
            data: vec![vec![Cell::Empty; COLS]; ROWS]
        }
    }

    /// Create a new state from a 2D vector of cells
    pub fn from_data(data: Vec<Vec<Cell>>) -> State {
        State {
            data
        }
    }

    /// Returns true if the board is full
    pub fn is_full(&self) -> bool {
        self.data.iter().all(|row| row.iter().all(|cell| *cell != Cell::Empty))
    }

    /// Returns true if dropping a piece into the given
    /// column is a valid move
    pub fn is_valid(&self, col: usize) -> bool {
        col < COLS && self[(0, col)] == Cell::Empty
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

/// The state of any given cell (or "position") on the board
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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