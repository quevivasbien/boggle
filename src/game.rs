use std::io::stdout;

use crate::{
    HELP_MESSAGE, WORDS_IN_COLUMN, N_COLUMNS, COL_SPACING,
    compress_qu, expand_qu,
    await_enter, quit, get_input
};
use crate::board::Board;
use crossterm::{execute, terminal, cursor, style::Stylize};


pub fn start_screen() {
    execute!(stdout(), terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0, 0)).unwrap();
    println!("Welcome to {}{}{}{}{}{}!" , "B".red().bold(), "O".green().bold(), "G".blue().bold(), "G".yellow().bold(), "L".magenta().bold(), "E".cyan().bold());

    println!("Find as many words as you can using consecutive letters on the board.");
    println!("\n{}", HELP_MESSAGE);

    let board_size = get_board_size();
    let min_len = get_min_len();
    println!("Ok! Playing with board size {} and minimum word length {}.\nPress enter to start...", &board_size, &min_len);
    await_enter();
    new_game(board_size, min_len);
}

fn get_board_size() -> usize {
    loop {
        let input = get_input(Some("Choose a board size (default is 5):"));
        if input.trim() == "" {
            return 5;
        }
        if let Ok(x) = usize::from_str_radix(input.trim(), 10) {
            return x;
        }
        println!("Choose a whole number greater than 1.");
    }
}

fn get_min_len() -> usize {
    loop {
        let input = get_input(Some("Choose a minimum word length (default is 3):"));
        if input.trim() == "" {
            return 3;
        }
        if let Ok(x) = usize::from_str_radix(input.trim(), 10) {
            return x;
        }
        println!("Choose a whole number greater than 1.");
    }
}

fn process_command(command: &str, board: Board, words_found: Vec<String>) {
    match command {
        "help" => help_screen(board, words_found),
        "check" => show_score(board, words_found),
        "quit" => quit(),
        _ => { println!("Unknown command: {} -- type !help for list of commands", command); },
    }
}

// prints all possible words in the board & number + percentage of words found
fn show_score(board: Board, words_found: Vec<String>) {
    execute!(stdout(), terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0, 0)).unwrap();
    let found = board.find_all_words();
    println!("{} total words in puzzle:", found.len());
    for (i, word) in found.iter().enumerate() {
        std::thread::sleep(std::time::Duration::from_millis(100));
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

    std::thread::sleep(std::time::Duration::from_millis(500));
    println!("\nPress enter to return to the main screen...");
    await_enter();
    start_screen();
}

fn help_screen(board: Board, words_found: Vec<String>) {
    // clear screen
    execute!(stdout(), terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0, 0)).unwrap();
    println!("{}", HELP_MESSAGE);
    println!("Press enter to continue...");
    await_enter();
    // re-enter game loop
    game_loop(board, words_found);
}

fn display_words_found(board_size: usize, words_found: &Vec<String>) {
    let vert_offset = (board_size + 3) as u16;
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
        display_words_found(board.size, &words_found);
        // get user input
        execute!(stdout(), cursor::MoveTo(0, 0)).unwrap();
        let input = get_input(Some(">"));
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

fn new_game(board_size: usize, min_len: usize) {
    let board = Board::random(board_size, min_len);
    let words_found = Vec::new();
    game_loop(board, words_found);
}
