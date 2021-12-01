use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind, Read},
};

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn count_increases(vec: Vec<i64>) -> i32 {
    let mut count = 0;
    let mut prev = vec[0];

    for i in 1..vec.len() {
        if vec[i] > prev {
            count += 1;
        }

        prev = vec[i];
    }

    count
}

fn count_increases_triplets(vec: Vec<i64>) -> i32 {
    let mut count = 0;
    let mut prev = vec[0] + vec[1] + vec[2];

    for i in 1..(vec.len() - 2) {
        let current = vec[i] + vec[i + 1] + vec[i + 2];
        if current > prev {
            count += 1;
        }
        prev = current;
    }

    count
}

fn main() -> Result<(), Error> {
    let vec = read(File::open("input.txt")?)?;

    // let count = count_increases(vec);
    let triple_count = count_increases_triplets(vec);

    println!("Times a depth measurement increases: {}", triple_count);

    Ok(())
}
