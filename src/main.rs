use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
    time::Instant,
};

use chrono::Local;

/// Holds the state for a game
#[derive(Debug, Clone, Copy)]
struct Game {
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
        } else {
            // Tails
            // Reset `n_heads_in_a_row`
            self.n_heads_in_a_row = 0;
        }
    }

    /// Flip the coin until you reach `n_win` heads in a row. Also
    /// allows setting a maximum number of iterations. If successful within
    /// the `max_iters` then return the game state, otherwise `None`.
    fn play(&mut self, n_win: usize, max_iters: usize) -> Option<Self> {
        for _ in 0..max_iters {
            // Check for game completion
            if self.n_heads_in_a_row >= n_win {
                return Some(*self);
            }

            // Flip the coin
            self.flip();
        }
        None
    }
}

/// Saves `data` to a CSV file named `<YYYY-MM-DDTHH-MM-SS>.csv` in the current directory.
/// Each value is written on its own line (single-column CSV).
/// Returns the created file path.
///
/// # Errors
///
/// Will error out if the file cannot be created, or if there is an error during writing,
/// or if the file cannot be flushed.
fn save_usize_csv(data: &[usize]) -> std::io::Result<PathBuf> {
    // Format current local time as a readable timestamp
    let timestamp = Local::now().format("%Y-%m-%dT%H-%M-%S").to_string();
    let filename = format!("{timestamp}.csv");
    let path = PathBuf::from(&filename);

    let file = File::create(&path)?;
    let mut w = BufWriter::new(file);

    // Write the header
    writeln!(w, "n_flips")?;

    for &value in data {
        writeln!(w, "{value}")?;
    }
    w.flush()?;
    Ok(path)
}

fn main() {
    let max_iters = 2_000_000;
    let n_games = 1_000;
    let mut results: Vec<usize> = Vec::with_capacity(n_games);

    let start_time = Instant::now();

    for _ in 0..n_games {
        let mut game_state = Game::new();

        let end_game = game_state.play(10, max_iters);

        match end_game {
            Some(Game {
                p_heads: _,
                flip_time: _,
                total_time: _,
                n_flips,
                n_heads_in_a_row: _,
            }) => {
                results.push(n_flips);
            }
            None => results.push(max_iters),
        }
    }

    let run_time = start_time.elapsed();
    let avg_run_time = run_time.div_f64(n_games as f64);
    println!("Ran in {run_time:?}, at about {avg_run_time:?} per game");

    match save_usize_csv(&results) {
        Ok(path) => println!("Saved the data to {}", path.display()),
        Err(_) => println!("Failed to save the data to file. Printing here.\n\n{results:?}"),
    }
}
