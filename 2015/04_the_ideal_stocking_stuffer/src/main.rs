use md5lib::find_smallest_number;

fn main() {
    let input = "iwrupvqb";

    let n = find_smallest_number(input, "00000");

    println!("{}", n);
}