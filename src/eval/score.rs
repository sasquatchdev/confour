use crate::{board::{State, COLS, MAXIMIZER, MINIMIZER, ROWS}, detect::sequences_all};

const WEIGHTS: [[i32; COLS]; ROWS] = [
    [ 1, 1, 2, 2, 2, 1, 1 ],
    [ 1, 2, 3, 3, 3, 2, 1 ],
    [ 1, 2, 3, 4, 3, 2, 1 ],
    [ 1, 3, 4, 5, 4, 3, 1 ],
    [ 1, 2, 3, 4, 3, 2, 1 ],
    [ 1, 1, 2, 3, 2, 1, 1 ],
];

const fn evaluate_streak(len: usize) -> f32 {
    (1 << len) as f32
}

impl State {
    /// Returns the static evaluation of the current state
    /// from the perspective of the current player.
    /// i.e.
    ///     MAXIMIZER's turn + MAXIMIZER wins => 100.0
    ///     MAXIMIZER's turn + MINIMIZER wins => -100.0
    ///     MINIMIZER's turn + MAXIMIZER wins => -100.0
    ///     MINIMIZER's turn + MINIMIZER wins => 100.0
    pub fn evaluate(
        &self
    ) -> f32 {
        if let Some(MAXIMIZER) = self.get_winner() {
            return if self.player() == MAXIMIZER { 100.0 } else { -100.0 };
        } else if Some(MINIMIZER) == self.get_winner() {
            return if self.player() == MINIMIZER { 100.0 } else { -100.0 };
        } else if self.is_full() {
            return 0.0;
        }

        let mut score = 0.0;

        for sequence in sequences_all(self, MAXIMIZER) {
            for cell in &sequence {
                score += WEIGHTS[cell.0 as usize][cell.1 as usize] as f32 * 0.2;
            }

            score += evaluate_streak(sequence.len());
        }

        score
    }
}
