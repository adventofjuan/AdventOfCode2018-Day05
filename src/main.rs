extern crate char_iter;
use std::fs::File;
use std::io::prelude::*;
use std::cmp;

fn part01(list: &String) {
    let mut last_c;
    let mut new_string = String::new();

    for c in list.chars() {
        if new_string.is_empty() {
            new_string.push(c);
        } else {
            last_c = new_string.chars().last().unwrap();
            if eq_opposite_case(c, last_c) {
                new_string.pop();
            } else {
                if c != '\n' {
                    new_string.push(c);
                }
            }
        }
    }

    println!("Part01 result = {}", new_string.len());
}

fn remove_char(list: &String, fc: char) -> String {

    let mut res = String::new();
    for c in list.chars() {
        if c.to_ascii_lowercase() != fc {
            res.push(c);
        }
    }
    res
}

fn react_full(list: &String) -> String {
    let mut last_c;
    let mut new_string = String::new();

    for c in list.chars() {
        if new_string.is_empty() {
            new_string.push(c);
        } else {
            last_c = new_string.chars().last().unwrap();
            if eq_opposite_case(c, last_c) {
                new_string.pop();
            } else {
                if c != '\n' {
                    new_string.push(c);
                }
            }
        }
    }
    new_string
}

fn part02(list: &String) {
    let mut min = std::usize::MAX;

    for c in char_iter::new('a', 'z') {
        let removed = remove_char(&list, c);
        min = cmp::min(min, react_full(&removed).len());
    }
    println!("Part02 result = {}", min);
}

fn eq_opposite_case(this: char, that: char) -> bool {
    this.eq_ignore_ascii_case(&that) && (
        (this.is_lowercase() && that.is_uppercase()) ||
        (this.is_uppercase() && that.is_lowercase())
        )
}

fn main() {
    let mut f = File::open("input.txt").expect("file not found");
    let mut line = String::new();
    f.read_to_string(&mut line).expect("Could not read file");

    part01(&line);
    part02(&line);
}
