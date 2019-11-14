use ndarray::{Array2, s};
use std::fs;

fn main() {
    let mut grid = Array2::<usize>::zeros((1000, 1000));

    let commands = read_input("input.txt");
    let commands: Vec<&str> = commands.split('\n').collect();

    for command in commands {
        execute_command(&mut grid, command);
    }

    println!("There are {} lights on", grid.sum());
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
}

// Commands come in the form of 'command N,N through N,N'
//  ex. toggle 461,550 through 564,900
//      turn off 370,39 through 425,839
//      turn on 599,989 through 806,993
fn execute_command(grid: &mut Array2::<usize>, s: &str) {
    let tokens: Vec<&str> = s.split_ascii_whitespace().collect();

    match tokens[0] {
        "toggle" => toggle_range(grid, tuple_from_comma_separated_numbers(tokens[1]), tuple_from_comma_separated_numbers(tokens[3])),
        "turn" => {
            match tokens[1] {
                "on" => turn_on_range(grid, tuple_from_comma_separated_numbers(tokens[2]), tuple_from_comma_separated_numbers(tokens[4])),
                "off" => turn_off_range(grid, tuple_from_comma_separated_numbers(tokens[2]), tuple_from_comma_separated_numbers(tokens[4])),
                _ => (),
            }
        }
        _ => (),
    }
}

fn tuple_from_comma_separated_numbers(s: &str) -> (usize, usize) {
    let tokens: Vec<&str> = s.split(',').collect();
    assert_eq!(tokens.len(), 2);
    (tokens[0].parse::<usize>().unwrap(), tokens[1].parse::<usize>().unwrap())
}

fn toggle_range(grid: &mut Array2::<usize>, corner1: (usize, usize), corner2: (usize, usize)) {
    println!("Toggling from {:?} through {:?}", corner1, corner2);
    grid
        .slice_mut(s![corner1.0..corner2.0+1, corner1.1..corner2.1+1])
        .mapv_inplace(|v| 1-v);
}

fn turn_on_range(grid: &mut Array2::<usize>, corner1: (usize, usize), corner2: (usize, usize)) {
    println!("Turning on from {:?} through {:?}", corner1, corner2);
    grid
        .slice_mut(s![corner1.0..corner2.0+1, corner1.1..corner2.1+1])
        .mapv_inplace(|_v| 1);
}

fn turn_off_range(grid: &mut Array2::<usize>, corner1: (usize, usize), corner2: (usize, usize)) {
    println!("Turning off from {:?} through {:?}", corner1, corner2);
    grid
        .slice_mut(s![corner1.0..corner2.0+1, corner1.1..corner2.1+1])
        .mapv_inplace(|_v| 0);
}

#[cfg(test)]
mod tests {

    #[test]
    fn tuple_from_comma_separated_numbers() {
        assert_eq!(super::tuple_from_comma_separated_numbers("1,2"), (1,2));
        assert_eq!(super::tuple_from_comma_separated_numbers("11,22"), (11,22));
        assert_eq!(super::tuple_from_comma_separated_numbers("333,444"), (333,444));
    }
}