use std::fs::File;
use std::io::{stdout, BufRead, BufReader};

use crate::{BOARD_POSITION, compress_qu, expand_qu};
use rand;
use crossterm::{style::Stylize, execute, cursor::MoveTo};

pub struct Board {
    pub size: usize,
    pub chars: Vec<char>,  // len = size * size
    pub min_len: usize,
    pub words: Vec<String>,
}

fn load_words() -> Vec<String> {
    let file = File::open("word_list.txt").expect("Unable to load words file");
    let reader = BufReader::new(file);
    reader.lines().map(|w|
        compress_qu(w.unwrap())
    ).collect()
}

// fn fast_filter_words<'a>(min_len: usize, words: Vec<&'a String>) -> Vec<&'a String> {
//     // takes advantage of fact that words are sorted in ascending length
//     for (i, w) in words.iter().enumerate() {
//         if w.len() >= min_len {
//             return Vec::from(&words[i..]);
//         }
//     }
//     Vec::new()
// }

fn filter_words(min_len: usize, words: Vec<String>) -> Vec<String> {
    words.into_iter().filter(|w|
        w.len() >= min_len
    ).collect()
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

impl Board {
    pub fn random(size: usize, min_len: usize) -> Board {
        let words = filter_words(min_len, load_words());
        let char_dist = get_char_dist(&words);
        let mut chars = Vec::with_capacity(size * size);
        for _i in 0..size * size {
            // draw random char based on char_dist
            let mut r = rand::random::<f64>();
            for (j, f) in char_dist.iter().enumerate() {
                if r < *f {
                    chars.push((97 + j as u8) as char);
                    break;
                }
                r -= f;
            }
        }
        Board { size, chars, min_len, words }
    }

    fn to_index(&self, x: usize, y: usize) -> usize {
        x + y * self.size
    }

    fn get_neighbors(&self, i: usize, visited: &Vec<usize>) -> Vec<usize> {
        let x = (i % self.size) as i32;
        let y = (i / self.size) as i32;
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
            |(x, y)| *x >= 0 && *x < (self.size as i32) && *y >= 0 && *y < (self.size as i32)
        ).map(
            |(x, y)| self.to_index(x as usize, y as usize)
        ).filter(
            |i| !visited.contains(i)
        ).collect()
    }

    fn has_word_from(&self, word: &[char], i: usize, mut visited: Vec<usize>) -> bool {
        if word.len() == 0 {
            return true;
        }
        if word[0] == self.chars[i] {
            let tail = &word[1..];
            visited.push(i);
            for j in self.get_neighbors(i, &visited) {
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
        for i in 0..self.size * self.size {
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
            for j in self.get_neighbors(i, &visited) {
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
        for i in 0..self.size * self.size {
            let path = self.get_path_from(&word, i, vec![]);
            if path.is_ok() {
                return path;
            }
        }
        Err("Word not found".to_string())
    }

    // pub fn find_words_from(
    //     &self,
    //     i: usize,
    //     word_head: String,
    //     visited: Vec<usize>,
    //     words_remaining: Vec<&String>,
    // ) -> Vec<String> {
    //     let words_remaining = fast_filter_words(word_head.len() + 1, words_remaining);
    //     if words_remaining.is_empty() {
    //         return vec![];
    //     }
    //     let mut new_words = Vec::new();
    //     for j in self.get_neighbors(i, &visited) {
    //         let candidate_words = words_remaining.clone().into_iter().filter(|w|
    //             w.starts_with(&word_head)
    //         ).collect::<Vec<&String>>();
    //         if candidate_words.is_empty() {
    //             continue;
    //         }
    //         let mut new_head = word_head.clone();
    //         new_head.extend([self.chars[j]].iter());
    //         let mut new_visited = visited.clone();
    //         new_visited.push(j);
    //         let mut to_add = self.find_words_from(
    //             j, new_head, new_visited, candidate_words
    //         );
    //         new_words.append(&mut to_add);
    //     }
    //     new_words
    // }

    // pub fn find_all_words(&self) -> Vec<String> {
    //     let mut found = vec![];
    //     let vec_refs = self.words.iter().collect::<Vec<&String>>();
    //     for i in 0..self.size * self.size {
    //         let mut new_words = self.find_words_from(i, "".to_string(), vec![], vec_refs.clone());
    //         found.append(&mut new_words);
    //     }
    //     found
    // }

    pub fn find_all_words(&self) -> Vec<String> {
        let mut found = vec![];
        for word in &self.words {
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
        execute!(stdout(), MoveTo(BOARD_POSITION.0, BOARD_POSITION.1)).unwrap();
        for i in 0..self.size * self.size {
            let l = self.chars[i].to_uppercase().to_string();
            let l_fmt = if i % self.size < self.size - 1 {
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
            if i % self.size == self.size - 1 {
                print!("\n");
            }
        }
    }

    pub fn check_word(&self, word: &String) -> bool {
        if word.len() < self.min_len {
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
