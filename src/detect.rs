use crate::board::{Cell, Player, State, COLS, ROWS};

/// All directions that we can check for a win,
/// in form of (dr, dc) tuples.
pub const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (1, 1), (1, -1)];  // right, down, down-right, down-left

/// The number of pieces in a row needed to win
pub const WIN_LENGTH: usize = 4;

pub type Direction = (i32, i32);
pub type Sequence = Vec<(usize, usize)>;

pub fn sequences_all(state: &State, player: Player) -> Vec<Sequence> {
    let mut sequences = Vec::new();
    
    for row in 0 .. ROWS {
        for col in 0 .. COLS {
            let seqs = sequences_from_position(state, player, row, col);
            sequences.extend(seqs);
        }
    }

    sequences.sort();
    sequences.dedup();

    sequences
}

pub fn sequences_from_position(state: &State, player: Player, row: usize, col: usize) -> Vec<Sequence> {
    let mut sequences = Vec::new();

    if let Cell::Player { player: p } = state[(row, col)] {
        if p == player {
            for direction in DIRECTIONS.iter() {
                let s1 = sequence_in_direction(state, player, row, col, (direction.0, direction.1));
                let s2 = sequence_in_direction(state, player, row, col, (-direction.0, -direction.1));

                let mut seq = [s1, s2].concat();
                seq.dedup();

                sequences.push(seq);
            }
        }
    }

    let mut sequences = sequences
        .iter()
        .map(|seq| {
            let mut seq = seq.clone();
            seq.sort();
            seq.dedup();
            seq
        })
        .filter(|seq| seq.len() > 1)
        .collect::<Vec<_>>();

    sequences.sort();
    sequences.dedup();
    sequences
}

pub fn sequence_in_direction(state: &State, player: Player, row: usize, col: usize, direction: Direction) -> Sequence {
    let mut sequence = Vec::new();
    
    let mut r = row as i32;
    let mut c = col as i32;

    let (dr, dc) = direction;

    while
        r >= 0 && r < ROWS as i32 &&
        c >= 0 && c < COLS as i32
    {
        if let Cell::Player { player: p } = state[(r as usize, c as usize)] {
            if p == player {
                sequence.push((r as usize, c as usize));
            } else {
                break;
            }
        } else {
            break;
        }

        r += dr;
        c += dc;
    }

    sequence
}

pub fn get_winner(state: &State) -> Option<Player> {
    for player in [Player::Red, Player::Yellow].iter() {
        let sequences = sequences_all(state, *player);

        for seq in sequences.iter() {
            if seq.len() >= WIN_LENGTH {
                return Some(*player);
            }
        }
    }

    None
}

#[cfg(test)]
pub mod tests {
    use crate::board::{Player, State};
    use super::*;

    #[test]
    /// Tests direction (dx = 1, dy = 0) or towards the right
    fn test_sequence_in_direction_1() {
        let mut state = State::new();
        let fields = vec![(5, 0), (5, 1), (5, 2), (5, 3)];

        state[(5, 0)] = Cell::Player { player: Player::Red };
        state[(5, 1)] = Cell::Player { player: Player::Red };
        state[(5, 2)] = Cell::Player { player: Player::Red };
        state[(5, 3)] = Cell::Player { player: Player::Red };

        let seq = sequence_in_direction(&state, Player::Red, 5, 0, DIRECTIONS[0]);
        
        assert_eq!(
            seq.len(), 4,
            "Expected sequence of length 4, got {}",
            seq.len()
        );

        assert_eq!(
            seq, fields,
            "Expected sequence {:?}, got {:?}",
            fields,
            seq
        );
    }

    #[test]
    /// Tests direction (0, 1) or downwards
    fn test_sequence_in_direction_2() {
        let mut state = State::new();
        let fields = vec![(2, 0), (3, 0), (4, 0), (5, 0)];

        state[(5, 0)] = Cell::Player { player: Player::Red };
        state[(4, 0)] = Cell::Player { player: Player::Red };
        state[(3, 0)] = Cell::Player { player: Player::Red };
        state[(2, 0)] = Cell::Player { player: Player::Red };

        let seq = sequence_in_direction(&state, Player::Red, 2, 0, DIRECTIONS[1]);
        
        assert_eq!(
            seq.len(), 4,
            "Expected sequence of length 4, got {}",
            seq.len()
        );

        assert_eq!(
            seq, fields,
            "Expected sequence {:?}, got {:?}",
            fields,
            seq
        );
    }

    #[test]
    /// Tests direction (1, -1) or diagonally towards the bottom right
    fn test_sequence_in_direction_3() {
        let mut state = State::new();
        let fields = vec![(2, 0), (3, 1), (4, 2), (5, 3)];

        state[(2, 0)] = Cell::Player { player: Player::Red };
        state[(3, 1)] = Cell::Player { player: Player::Red };
        state[(4, 2)] = Cell::Player { player: Player::Red };
        state[(5, 3)] = Cell::Player { player: Player::Red };

        println!("{}", state);

        let seq = sequence_in_direction(&state, Player::Red, 2, 0, DIRECTIONS[2]);

        assert_eq!(
            seq.len(), 4,
            "Expected sequence of length 4, got {}",
            seq.len()
        );

        assert_eq!(
            seq, fields,
            "Expected sequence {:?}, got {:?}",
            fields,
            seq
        );
    }

    #[test]
    /// Tests direction (1, -1) or towards the bottom left
    fn test_sequence_in_direction_4() {
        let mut state = State::new();
        let fields = vec![(2, 3), (3, 2), (4, 1), (5, 0)];

        state[(2, 3)] = Cell::Player { player: Player::Red };
        state[(3, 2)] = Cell::Player { player: Player::Red };
        state[(4, 1)] = Cell::Player { player: Player::Red };
        state[(5, 0)] = Cell::Player { player: Player::Red };

        println!("{}", state);

        let seq = sequence_in_direction(&state, Player::Red, 2, 3, DIRECTIONS[3]);

        assert_eq!(
            seq.len(), 4,
            "Expected sequence of length 4, got {}",
            seq.len()
        );

        assert_eq!(
            seq, fields,
            "Expected sequence {:?}, got {:?}",
            fields,
            seq
        );
    }

    #[test]
    fn test_sequence_from_position_1() {
        let mut state = State::new();
        let fields = vec![(2, 3), (3, 2), (4, 1), (5, 0)];

        state[(2, 3)] = Cell::Player { player: Player::Red };
        state[(3, 2)] = Cell::Player { player: Player::Red };
        state[(4, 1)] = Cell::Player { player: Player::Red };
        state[(5, 0)] = Cell::Player { player: Player::Red };
    
        let seqs1 = sequences_from_position(&state, Player::Red, 2, 3);
        let seqs2 = sequences_from_position(&state, Player::Red, 5, 0);

        println!("{:?}", seqs1);
        println!("{:?}", seqs2);

        assert_eq!(
            seqs1.len(), 1,
            "Expected 2 sequences, got {}",
            seqs1.len()
        );

        assert_eq!(
            seqs2.len(), 1,
            "Expected 2 sequences, got {}",
            seqs2.len()
        );

        assert_eq!(
            seqs1[0], fields,
            "Expected sequence {:?}, got {:?}",
            fields,
            seqs1[0]
        );

        assert_eq!(
            seqs2[0], fields,
            "Expected {:?}, got {:?}",
            fields,
            seqs2[0]
        );
    }

    #[test]
    fn test_sequence_from_position_2() {
        let mut state = State::new();
        let mut fields = vec![(5, 0), (4, 0), (3, 0), (2, 0)];
        fields.sort();

        state[(2, 0)] = Cell::Player { player: Player::Red };
        state[(3, 0)] = Cell::Player { player: Player::Red };
        state[(4, 0)] = Cell::Player { player: Player::Red };
        state[(5, 0)] = Cell::Player { player: Player::Red };
    
        let seqs1 = sequences_from_position(&state, Player::Red, 5, 0);
        let seqs2 = sequences_from_position(&state, Player::Red, 2, 0);

        println!("{:?}", seqs1);
        println!("{:?}", seqs2);

        assert_eq!(
            seqs1.len(), 1,
            "Expected 2 sequences, got {}",
            seqs1.len()
        );

        assert_eq!(
            seqs2.len(), 1,
            "Expected 2 sequences, got {}",
            seqs2.len()
        );

        assert_eq!(
            seqs1[0], fields,
            "Expected sequence {:?}, got {:?}",
            fields,
            seqs1[0]
        );

        assert_eq!(
            seqs2[0], fields,
            "Expected {:?}, got {:?}",
            fields,
            seqs2[0]
        );
    }

    #[test]
    fn test_sequences_all() {
        let mut state = State::new();
        let fields = vec![(2, 3), (3, 2), (4, 1), (5, 0)];

        state[(2, 3)] = Cell::Player { player: Player::Red };
        state[(3, 2)] = Cell::Player { player: Player::Red };
        state[(4, 1)] = Cell::Player { player: Player::Red };
        state[(5, 0)] = Cell::Player { player: Player::Red };

        let seqs = sequences_all(&state, Player::Red);
        println!("{:#?}", seqs);

        assert_eq!(
            seqs.len(), 1,
            "Expected 1 sequence, got {}",
            seqs.len()
        );

        assert_eq!(
            seqs[0], fields,
            "Expected sequence {:?}, got {:?}",
            fields,
            seqs[0]
        );
    }

    #[test]
    fn test_get_winner_1() {
        let mut state = State::new();

        state[(2, 3)] = Cell::Player { player: Player::Red };
        state[(3, 2)] = Cell::Player { player: Player::Red };
        state[(4, 1)] = Cell::Player { player: Player::Red };
        state[(5, 0)] = Cell::Player { player: Player::Red };

        let winner = get_winner(&state);

        assert_eq!(
            winner, Some(Player::Red),
            "Expected winner to be Red, got {:?}",
            winner
        );
    }

    #[test]
    fn test_get_winner_2() {
        let mut state = State::new();

        state[(2, 3)] = Cell::Player { player: Player::Yellow };
        state[(3, 2)] = Cell::Player { player: Player::Yellow };
        state[(4, 1)] = Cell::Player { player: Player::Yellow };
        state[(5, 0)] = Cell::Player { player: Player::Yellow };

        let winner = get_winner(&state);

        assert_eq!(
            winner, Some(Player::Yellow),
            "Expected winner to be Yellow, got {:?}",
            winner
        );
    }

    #[test]
    fn test_get_winner_3() {
        let mut state = State::new();

        state[(2, 3)] = Cell::Player { player: Player::Red };
        state[(3, 2)] = Cell::Player { player: Player::Red };
        state[(4, 1)] = Cell::Player { player: Player::Red };

        state[(2, 2)] = Cell::Player { player: Player::Yellow };
        state[(3, 1)] = Cell::Player { player: Player::Yellow };
        state[(4, 0)] = Cell::Player { player: Player::Yellow };

        let winner = get_winner(&state);

        assert_eq!(
            winner, None,
            "Expected no winner, got {:?}",
            winner
        );
    }
}
