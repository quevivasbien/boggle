use std::io::stdout;

use crate::{BOARD_SIZE, MIN_LENGTH, expand_qu, get_char_dist};
use rand;
use crossterm::{style::Stylize, execute, cursor::MoveTo};

pub struct Board<'a> {
    pub chars: [char; BOARD_SIZE * BOARD_SIZE],
    pub words: &'a Vec<String>,
}

fn to_index(x: usize, y: usize) -> usize {
    x + y * BOARD_SIZE
}

fn get_neighbors(i: usize, visited: &Vec<usize>) -> Vec<usize> {
    let x = (i % BOARD_SIZE) as i32;
    let y = (i / BOARD_SIZE) as i32;
    [
        (x + 1, y + 1),
        (x + 1, y),
        (x + 1, y - 1),
        (x, y + 1),
        (x, y),
        (x, y - 1),
        (x - 1, y + 1),
        (x - 1, y),
        (x - 1, y - 1),
    ].into_iter().filter(
        |(x, y)| *x >= 0 && *x < (BOARD_SIZE as i32) && *y >= 0 && *y < (BOARD_SIZE as i32)
    ).map(
        |(x, y)| to_index(x as usize, y as usize)
    ).filter(
        |i| !visited.contains(i)
    ).collect()
}

impl<'a> Board<'a> {
    pub fn random(words: &'a Vec<String>) -> Board<'a> {
        let char_dist = get_char_dist(words);
        let mut chars = ['a'; BOARD_SIZE * BOARD_SIZE];
        for i in 0..BOARD_SIZE * BOARD_SIZE {
            // draw random char based on char_dist
            let mut r = rand::random::<f64>();
            for (j, f) in char_dist.iter().enumerate() {
                if r < *f {
                    chars[i] = (97 + j as u8) as char;
                    break;
                }
                r -= f;
            }
        }
        Board { chars, words }
    }
    pub fn from(x: &str, words: &'a Vec<String>) -> Board<'a> {
        let mut chars = ['a'; BOARD_SIZE * BOARD_SIZE];
        for (i, c) in x.chars().enumerate() {
            if i >= BOARD_SIZE * BOARD_SIZE {
                break;
            }
            chars[i] = c;
        }
        Board { chars, words }
    }
    
    pub fn get(&self, x: usize, y: usize) -> char {
        self.chars[to_index(x, y)]
    }

    fn has_word_from(&self, word: &Vec<char>, i: usize, mut visited: Vec<usize>) -> bool {
        if word.len() == 0 {
            return true;
        }
        if word[0] == self.chars[i] {
            let tail = word[1..].to_vec();
            visited.push(i);
            for j in get_neighbors(i, &visited) {
                if self.has_word_from(&tail, j, visited.clone()) {
                    return true;
                }
            }
        }
        false
    }

    // searches for word, doesn't keep track of path
    pub fn has_word(&self, word: &str) -> bool {
        let word = word.to_lowercase().chars().collect::<Vec<char>>();
        for i in 0..BOARD_SIZE * BOARD_SIZE {
            if self.has_word_from(&word, i, vec![]) {
                return true;
            }
        }
        false
    }

    fn get_path_from(&self, word: &Vec<char>, i: usize, mut visited: Vec<usize>) -> Result<Vec<usize>, String> {
        if word.len() == 0 {
            return Ok(visited);
        }
        if word[0] == self.chars[i] {
            let tail = word[1..].to_vec();
            visited.push(i);
            for j in get_neighbors(i, &visited) {
                let path = self.get_path_from(&tail, j, visited.clone());
                if path.is_ok() {
                    return path;
                }
            }
        }
        Err("Word not found".to_string())
    }

    // attempts to find path for word, returns error if not found
    pub fn get_path(&self, word: &String) -> Result<Vec<usize>, String> {
        let word = word.to_lowercase().chars().collect::<Vec<char>>();
        for i in 0..BOARD_SIZE * BOARD_SIZE {
            let path = self.get_path_from(&word, i, vec![]);
            if path.is_ok() {
                return path;
            }
        }
        Err("Word not found".to_string())
    }

    pub fn find_all_words(&self) -> Vec<String> {
        let mut found = vec![];
        for word in self.words {
            if self.has_word(word) {
                found.push(expand_qu(word.clone()));
            }
        }
        found
    }

    pub fn display(&self) {
        self.display_with_highlights(vec![]);
    }

    pub fn display_with_highlights(&self, highlights: Vec<usize>) {
        execute!(stdout(), MoveTo(0, 2)).unwrap();
        for i in 0..BOARD_SIZE * BOARD_SIZE {
            let l = self.chars[i].to_uppercase().to_string();
            let l_fmt = if i % BOARD_SIZE < BOARD_SIZE - 1 {
                let letter = {
                    if l == "Q" {
                        "Qu ".to_string()
                    } else {
                        format!("{}  ", l)
                    }
                };
                format!("{}", letter)
            } else {
                let letter = {
                    if l == "Q" {
                        "Qu".to_string()
                    } else {
                        format!("{}", l)
                    }
                };
                format!("{}", letter)
            };
            print!("{}", if highlights.contains(&i) { l_fmt.green().bold() } else { l_fmt.white() });
            if i % BOARD_SIZE == BOARD_SIZE - 1 {
                print!("\n");
            }
        }
    }

    pub fn check_word(&self, word: &String) -> bool {
        if word.len() < MIN_LENGTH {
            execute!(stdout(), MoveTo(0, 1)).unwrap();
            println!("{}: {}" , "Word too short".red(), expand_qu(word.clone()).red());
            return false;
        }
        if !self.words.contains(word) {
            execute!(stdout(), MoveTo(0, 1)).unwrap();
            println!("{}: {}" , "Word not in dictionary".red(), expand_qu(word.clone()).red());
            return false;
        }
        if !self.has_word(word) {
            execute!(stdout(), MoveTo(0, 1)).unwrap();
            println!("{}: {}" , "Word not found".red(), expand_qu(word.clone()).red());
            return false;
        }
        true
    }

}
