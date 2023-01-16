use std::fs::File;
use std::io::{stdout, BufRead, BufReader};
use std::{thread, time};
use crossterm::{execute, Result, terminal, cursor, style::Stylize};

pub mod board;
use board::Board;


const BOARD_SIZE: usize = 5;  // must be > 1
const MIN_LENGTH: usize = 4;  // must be > 1

// constants determining how found words are arranged
const WORDS_IN_COLUMN: usize = 12;  // must be > 0
const N_COLUMNS: usize = 3;
const COL_SPACING: u16 = 10;

const HELP_MESSAGE: &str = r#"
Commands:
    !help    Show this message
    !quit    Quit the game
    !check   See all words in the board and display your score (ends round)

    [word]   Check if the word is in the board
"#;

fn compress_qu(word: String) -> String {
    word.replace("qu", "q")
}

fn expand_qu(word: String) -> String {
    word.replace("q", "qu")
}

fn has_vowel(word: &String) -> bool {
    for c in word.chars() {
        if "aeiouy".contains(c) {
            return true;
        }
    }
    false
}

fn get_words() -> Vec<String> {
    let mut words = vec![];
    let file = File::open("words_alpha.txt").expect("Unable to load words file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let word = compress_qu(line.unwrap().trim().to_lowercase());
        // filter out invalid "words"
        if word.len() >= MIN_LENGTH && has_vowel(&word) {
            words.push(word);
        }
    }
    words
}

fn get_char_dist(words: &Vec<String>) -> Vec<f64> {
    let mut counts = vec![0; 26];
    for word in words {
        for c in word.chars() {
            counts[(c as u8 - 97) as usize] += 1;
        }
    }
    let total = counts.iter().sum::<i32>() as f64;
    counts.into_iter().map(|x| x as f64 / total).collect()
}

pub fn start_screen(words: &Vec<String>) {
    println!("Welcome to {}{}{}{}{}{}!" , "B".red().bold(), "O".green().bold(), "G".blue().bold(), "G".yellow().bold(), "L".magenta().bold(), "E".cyan().bold());

    println!("Find as many words as you can in the board. Words must be at least {} letters long.", MIN_LENGTH);

    println!("Enter a word to check if it's in the board. Enter a command to do something else.");
    println!("{}", HELP_MESSAGE);

    println!("Press enter to start...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim() == "!quit" {
        quit();
    }
    new_game(words);
}

fn quit() {
    execute!(stdout(), terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0, 0)).unwrap();
    std::process::exit(0);
}

fn process_command(command: &str, board: Board, words_found: Vec<String>) {
    match command {
        "help" => help_screen(board, words_found),
        "check" => {
            print_all_words(&board, &words_found);
            thread::sleep(time::Duration::from_secs(2));
            print!("\n\n");
            start_screen(board.words);
        },
        "quit" => quit(),
        _ => { println!("Unknown command: {} -- type !help for list of commands", command); },
    }
}

// prints all possible words in the board & number + percentage of words found
fn print_all_words(board: &Board, words_found: &Vec<String>) {
    execute!(stdout(), terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0, 0)).unwrap();
    let found = board.find_all_words();
    println!("{} total words in puzzle:", found.len());
    for (i, word) in found.iter().enumerate() {
        thread::sleep(time::Duration::from_millis(100));
        // calculate position
        let col = ((i / WORDS_IN_COLUMN) % N_COLUMNS) as u16;
        let row = (i % WORDS_IN_COLUMN) as u16 + 1;
        // set line cursor
        execute!(stdout(), cursor::SetCursorShape(cursor::CursorShape::Line)).unwrap();
        execute!(stdout(), cursor::MoveTo(col * 2 * COL_SPACING, row)).unwrap();
        print!("{} ", " ".repeat(2 * COL_SPACING as usize - 1));
        // write word
        execute!(stdout(), cursor::MoveTo(col * 2 * COL_SPACING, row)).unwrap();
        println!("{}. {}", i + 1, if words_found.contains(word) { word.clone().green() } else { word.clone().red() });
    }
    // move cursor to bottom and print score
    execute!(stdout(), cursor::MoveTo(0, WORDS_IN_COLUMN as u16 + 2)).unwrap();
    println!("You found {} words ({:.1}%)", words_found.len(), words_found.len() as f64 / found.len() as f64 * 100.0);
    // reset cursor
    execute!(stdout(), cursor::SetCursorShape(cursor::CursorShape::Block)).unwrap();
}

fn help_screen(board: Board, words_found: Vec<String>) {
    // clear screen
    execute!(stdout(), terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0, 0)).unwrap();
    println!("{}", HELP_MESSAGE);
    println!("Press enter to continue...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim() == "!quit" {
        quit();
    }
    // re-enter game loop
    game_loop(board, words_found)
}

fn display_words_found(words_found: &Vec<String>) {
    let vert_offset = (BOARD_SIZE + 3) as u16;
    execute!(stdout(), cursor::MoveTo(0, vert_offset)).unwrap();
    let n_found = words_found.len();
    println!("Words found ({}):", n_found);
    if n_found == 0 {
        return;
    }
    let n_to_display = n_found.min(WORDS_IN_COLUMN * N_COLUMNS - 1);
    for i in 0..n_to_display {
        let column = (i / WORDS_IN_COLUMN) as u16;
        let row = (i % WORDS_IN_COLUMN) as u16 + 1;
        execute!(stdout(), cursor::MoveTo(column * COL_SPACING, row + vert_offset)).unwrap();
        let j = n_found - i - 1;  // index in list of words found, counts backward
        print!("{} ", words_found[j].clone().green());
    }
    if n_found > n_to_display {
        execute!(stdout(), cursor::MoveTo((N_COLUMNS as u16 - 1) * COL_SPACING, WORDS_IN_COLUMN as u16 + vert_offset)).unwrap();
        println!("...");
    }
}

fn game_loop(board: Board, mut words_found: Vec<String>) {
    execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
    board.display();
    loop {
        display_words_found(&words_found);
        // set cursor to origin
        execute!(stdout(), cursor::MoveTo(0, 0)).unwrap();
        print!("{} ", ">".red().bold());
        // board starts 2 lines down
        execute!(stdout(), cursor::MoveTo(2, 0)).unwrap();
        // get user input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        // process input
        if input.starts_with('!') {
            return process_command(input[1..].trim(), board, words_found);
        }
        else if input.trim() == "" {
            println!("Enter a word or a command (type !help for a list of commands).");
        }
        else {
            execute!(stdout(), terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0, 0)).unwrap();
            let word = compress_qu(input.trim().to_lowercase());
            if board.check_word(&word) {
                board.display_with_highlights(board.get_path(&word).unwrap());
                let word = expand_qu(word);
                if !words_found.contains(&word) {
                    words_found.push(word);
                }
            }
            else {
                board.display();
            }
        }
    }
}

fn new_game(words: &Vec<String>) {
    let board = Board::random(words);
    let words_found = Vec::new();
    game_loop(board, words_found);
}


fn main() -> Result<()> {
    let words = get_words();
    // // println!("{} words loaded", words.len());

    execute!(stdout(), terminal::EnterAlternateScreen, cursor::MoveTo(0, 0))?;
    // // set indigo background
    // execute!(stdout(), style::SetBackgroundColor(style::Color::Rgb { r: 75, g: 0, b: 130 }))?;
    start_screen(&words);
    execute!(stdout(), terminal::LeaveAlternateScreen)
}
