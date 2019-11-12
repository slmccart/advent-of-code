fn main() {
    let input = "iwrupvqb";

    let n = find_smallest_number(input, "00000");

    println!("{}", n);
}

fn find_smallest_number(input: &str, desired: &str) -> u64 {
    for n in 1..std::u64::MAX {
        let test = format!("{}{}", input, n);
        let md5 = md5::compute(test);
        let md5 = format!("{:x}", md5);
        let md5 = &md5[..5];

        if md5 == desired {
            return n;
        }
    }

    0
}