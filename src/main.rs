use ::std::io::ErrorKind;
use std::{fs::File, io::Write};
fn main() {
    fn average_func(selection: &mut Vec<i32>) -> i32 {
        let selection_length = selection.len() as i32;

        let mut total: i32 = 0;

        for val in selection {
            total += *val;
        }

        total /= selection_length;

        total
    }

    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

    let average = average_func(&mut numbers);

    println!("Average Of selection Is {}", average);

    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Cannot Create File {:?}", e),
            },

            other_error => {
                panic!("Cannot Open File {:?}", other_error)
            }
        },
    };

    match f.write_all(b"HI") {
        Ok(f) => f,
        Err(e) => panic!("ERROR {:?}", e),
    };
}
