use std::fs::File;
use std::io::prelude::*;

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
}
