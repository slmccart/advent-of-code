use md5lib::find_smallest_number;
use md5lib::find_smallest_number_with_six_zeroes;

fn main() {
    let input = "iwrupvqb";

    let n = find_smallest_number(input);
    println!("{}", n);

    let n = find_smallest_number_with_six_zeroes(input);
    println!("{}", n);
}