use std::{fs, path::Path};

fn main() {
    let inputs = read_inputs("input.txt");

    let mut increased = 0;

    inputs.iter().reduce(|last, new| {
        if new > last {
            increased += 1;
        }
        new
    });

    println!("{:?}", increased);
}

fn read_inputs<T: AsRef<Path>>(path: T) -> Vec<i16> {
    let file_contents = fs::read_to_string(path).expect("Input file not found.");

    file_contents.split("\n").map(|n|
        n.parse::<i16>().unwrap()
    ).collect()
}
