use std::io::stdout;
use crossterm::{execute, Result, terminal, cursor, style::Stylize};

mod board;
mod game;

const BOARD_POSITION: (u16, u16) = (0, 2);

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

fn get_input(prompt: Option<&str>) -> String {
    if let Some(prompt) = prompt {
        print!("{} ", prompt.red().bold());
        execute!(stdout()).unwrap();
    }
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

fn await_enter() {
    if get_input(None).trim() == "!quit" {
        quit();
    }
}

fn quit() {
    execute!(stdout(), terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0, 0)).unwrap();
    std::process::exit(0);
}


fn main() -> Result<()> {
    execute!(stdout(), terminal::EnterAlternateScreen, cursor::MoveTo(0, 0))?;
    game::start_screen();
    execute!(stdout(), terminal::LeaveAlternateScreen)
}
