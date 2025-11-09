use std::{path::PathBuf, time::Instant};

use chrono::Local;
use serde::Serialize;

mod upgrades;

/// Holds the state for a game
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Game {
    /// The probability of flipping heads
    p_heads: f64,

    /// How long does a flip take in game time?
    flip_time: f64,

    /// How long has this game been running?
    total_time: f64,

    /// How many times have we flipped the coin this game?
    n_flips: usize,

    /// How many heads in a row? You win if you reach
    n_heads_in_a_row: usize,

    /// Current value of the coin
    coin_val: f64,

    /// Current score multiplier
    multiplier: f64,

    /// Current cash
    cash: f64,

    /// Current status of upgrades
    upgrades: upgrades::PHeadsUpgradeState,
}

impl Game {
    /// Start a new game
    fn new() -> Self {
        Game {
            p_heads: 0.3,
            flip_time: 2.0,
            total_time: 0.0,
            n_flips: 0,
            n_heads_in_a_row: 0,
            coin_val: 0.01,
            multiplier: 1.5,
            cash: 0.0,
            upgrades: upgrades::PHeadsUpgradeState::new(),
        }
    }

    /// Flip the coin, properly managing state
    fn flip(&mut self) {
        // Update `total_time`
        self.total_time += self.flip_time;

        // Update `n_flips`
        self.n_flips += 1;

        let rand_val = fastrand::f64();
        if rand_val < self.p_heads {
            // Heads!
            // Update `n_heads_in_a_row`
            self.n_heads_in_a_row += 1;

            // Update the amount of cash
            self.cash += Game::calc_reward(self.coin_val, self.multiplier, self.n_heads_in_a_row);
        } else {
            // Tails
            // Reset `n_heads_in_a_row`
            self.n_heads_in_a_row = 0;
        }
    }

    /// Flip the coin until you reach `n_win` heads in a row. Also
    /// allows setting a maximum number of iterations. Returns the final
    /// game state, regardless of ending. If the game did not complete in
    /// `max_iters`, then `self.n_flips` will be set to `usize::MAX`.
    fn play(&mut self, n_win: usize, max_iters: usize) -> Self {
        for _ in 0..max_iters {
            // Check for game completion
            if self.n_heads_in_a_row >= n_win {
                return *self;
            }

            // Flip the coin
            self.flip();
        }

        // Did not complete the game in `max_iters`, so set `self.n_flips`
        // to `usize::MAX`, and return.
        self.n_flips = usize::MAX;
        *self
    }

    /// A stateless function for calculating the reward given the current reward, the
    /// current multiplier, and the number of heads in a row. For the first head in a
    /// streak, the `current_reward` is the `coin_value`; after that, `current_reward`
    /// is the reward we got last time.
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    fn calc_reward(coin_value: f64, multiplier: f64, n_heads_in_a_row: usize) -> f64 {
        multiplier.powi((n_heads_in_a_row as i32) - 1).ceil() * coin_value
    }
}

/// Saves `games` to a TSV file named `<YYYY-MM-DDTHH-MM-SS>.csv` in the current directory.
/// Returns the created file path.
///
/// # Errors
///
/// Will error out if the file cannot be created, or if there is an error during writing,
/// or if the file cannot be flushed.
fn save_game_states_tsv(games: &[Game]) -> std::io::Result<PathBuf> {
    // Format current local time as a readable timestamp
    let timestamp = Local::now().format("%Y-%m-%dT%H-%M-%S").to_string();
    let filename = format!("{timestamp}.tsv");
    let path = PathBuf::from(&filename);

    // Create the writer, using tabs, and writing to the created path
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .from_path(&path)?;

    // Write each game
    for &g in games {
        wtr.serialize(g)?;
    }

    wtr.flush()?;
    Ok(path)
}

#[allow(clippy::cast_precision_loss)]
fn main() {
    let max_iters = 2_000_000;
    let n_games = 5_000;
    let mut results: Vec<Game> = Vec::with_capacity(n_games);

    let start_time = Instant::now();

    for _ in 0..n_games {
        let mut game_state = Game::new();

        let end_game = game_state.play(10, max_iters);

        results.push(end_game);
    }

    let run_time = start_time.elapsed();
    #[allow(clippy::cast_possible_wrap)]
    let avg_run_time = run_time.div_f64(n_games as f64);
    println!("Ran in {run_time:?}, at about {avg_run_time:?} per game");

    match save_game_states_tsv(&results) {
        Ok(path) => println!("Saved the data to {}", path.display()),
        Err(_) => println!("Failed to save the data to file. Printing here.\n\n{results:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use rstest::rstest;

    #[rstest]
    #[case(1.0, 3.0, 1, 1.0)]
    #[case(1.0, 3.0, 2, 3.0)]
    #[case(1.0, 3.0, 3, 9.0)]
    #[case(1.0, 3.0, 4, 27.0)]
    #[case(1.0, 3.0, 5, 81.0)]
    #[case(0.1, 3.0, 1, 0.1)]
    #[case(0.1, 3.0, 2, 0.3)]
    #[case(0.1, 3.0, 3, 0.9)]
    #[case(0.1, 3.0, 4, 2.7)]
    #[case(0.1, 3.0, 5, 8.1)]
    #[case(1.0, 2.5, 1, 1.0)]
    #[case(1.0, 2.5, 2, 3.0)]
    #[case(1.0, 2.5, 3, 7.0)]
    #[case(1.0, 2.5, 4, 16.0)]
    #[case(1.0, 2.5, 5, 40.0)]
    #[case(0.25, 2.5, 1, 0.25)]
    #[case(0.25, 2.5, 2, 0.75)]
    #[case(0.25, 2.5, 3, 1.75)]
    #[case(0.25, 2.5, 4, 4.0)]
    #[case(0.25, 2.5, 5, 10.0)]
    #[case(1.0, 1.0, 1, 1.0)]
    #[case(1.0, 1.0, 2, 1.0)]
    #[case(1.0, 1.0, 3, 1.0)]
    #[case(1.0, 1.0, 4, 1.0)]
    #[case(1.0, 1.0, 5, 1.0)]
    fn test_combo_mult(
        #[case] coin_value: f64,
        #[case] multiplier: f64,
        #[case] n_heads_in_a_row: usize,
        #[case] expected: f64,
    ) {
        let got = Game::calc_reward(coin_value, multiplier, n_heads_in_a_row);
        assert_relative_eq!(expected, got);
    }
}
