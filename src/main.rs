use anyhow::Result;
use promptly::prompt;
use rand::{seq::SliceRandom, thread_rng};
use std::{fs, fmt::Display};
use comfy_table::Table;

type Row = Vec<Space>;

type Board = Vec<Row>;

#[derive(Debug)]
enum SpaceStatus {
    Correct,
    Present,
    Absent,
}

#[derive(Debug)]
struct Space {
    guessed_letter: char,
    letter: char,
    status: SpaceStatus,
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{}", self.letter)
    }
}

fn create_board() -> Board {
    (0..6)
        .map(|_| -> Row {
            (0..5)
                .map(|_| Space {
                    guessed_letter: ' ',
                    letter: ' ',
                    status: SpaceStatus::Absent,
                })
                .collect()
        })
        .collect()
}

fn check_row(row: Row) -> Row {
   row.iter().map(|&space| {
        if space.letter == space.guessed_letter {
            return Space {
                letter: space.letter,
                guessed_letter: space.guessed_letter,
                status: SpaceStatus::Correct
            }
        }
        return space
    }).collect()
}

fn render_board(board: Board) {
    let mut table = Table::new();
    for row in board {
        table.add_row(row);
    }
    println!("{table}");
}

fn start_game() -> Result<()> {
    let word_file = fs::read_to_string("words")?;
    let mut rng = thread_rng();
    let words: Vec<String> = word_file.lines().map(|x| x.to_string()).collect();
    let random_word = words.choose(&mut rng).expect("Couldn't find a random word");

    let board: Board = create_board();
    render_board(board);

    loop {
        let mut guesses = 0;
        if guesses >= 6 {
            break;
        }
        let guessed_word: String = prompt("Guess a five letter word")?;
        if guessed_word.len() != 5 {
            println!("That word isn't five letters.");
            continue;
        }
        if !words.contains(&guessed_word) {
            println!("That isnt' a valid word!");
        }
        


        guesses += 1;
    }
    Ok(())
}

fn main() -> Result<()> {
    start_game()?;
    Ok(())
}
