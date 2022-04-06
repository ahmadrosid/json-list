use std::fs;
use std::io::Read;

fn main() {
    let inputs: Vec<String> = std::env::args().collect();
    if inputs.len() == 1 {
        let mut stdin = std::io::stdin();
        let mut buffer = String::new();
        let res = stdin.read_to_string(&mut buffer);
        if res.is_ok() {
            parse_input(&buffer);
            return;
        }
    }

    for item in &inputs[1..] {
        if item.contains("[") {
            parse_input(item);
        } else {
            parse_input(
                &fs::read_to_string(item).expect("Failed to read file.")
            );
        }
    }
}

pub fn parse_input(data: &str) {
    let mut result = vec![];
    for item in data.split(',') {
        let line = item
            .chars()
            .filter(|c| !c.eq(&'"') && !c.eq(&'[') && !c.eq(&']') && !c.eq(&','))
            .collect::<String>()
            .trim()
            .to_string();
        if !&line.is_empty() {
            result.push(line);
        }
    }
    print!("{}", result.join("\n"));
}
