use std::{fs, path::Path};

fn main() {
    let inputs = read_inputs("input.txt");

    let new_arr: Vec<i16> = (0..inputs.len() - 2).map(|i| inputs.iter().skip(i).take(3).sum()).collect();

    let mut increased = 0;

    new_arr.iter().reduce(|last, new| {
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
