use std::collections::HashMap;
use std::fs;

fn main() {
    let directions = read_input("input.txt");
    let map = deliver_to_list_of_directions(&directions);

    //println!("{:?}", map);
    println!("\nmap has {} houses with at least one present\n", map.len());
}

fn move_and_deliver_present(map: &mut HashMap<(i32, i32), u32>, current_location: (i32, i32), direction: char) -> (i32, i32) {
    let new_location = 
        match direction {
            '^' => (current_location.0 + 1, current_location.1),
            'v' => (current_location.0 - 1, current_location.1),
            '<' => (current_location.0, current_location.1 -1),
            '>' => (current_location.0, current_location.1 + 1),
            _ => current_location
        };

    let presents = map.entry(new_location).or_insert(0);
    *presents += 1;

    new_location
}

fn deliver_to_list_of_directions(list_of_directions: &str) -> HashMap<(i32, i32), u32> {
    let mut location = (0,0);
    let mut map: HashMap<(i32, i32), u32> =  HashMap::new();

    //Deliver present to initial house
    map.insert(location, 1);

    for direction in list_of_directions.chars() {
        location = move_and_deliver_present(&mut map, location, direction);
    }
    
    map
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
}