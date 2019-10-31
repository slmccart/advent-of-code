use std::cmp;
use std::fs;

struct Present {
    length: u32,
    width: u32,
    height: u32
}

impl Present {
    fn volume(&self) -> u32 {
        self.length * self.width * self.height
    }

    fn surface_area(&self) -> u32 {
        (2 * self.length * self.width) + (2 * self.width * self.height) + (2 * self.height * self.length)
    }

    fn area_of_smallest_side(&self) -> u32 {
        cmp::min(self.length * self.width, cmp::min(self.width * self.height, self.height * self.length))
    }

    fn perimeter_of_smallest_side(&self) -> u32 {
        cmp::min(2 * self.length + 2 * self.width, cmp::min(2 * self.width + 2 * self.height, 2 * self.height + 2 * self.length))
    }

    fn amount_of_wrapping_paper_needed(&self) -> u32 {
        self.surface_area() + self.area_of_smallest_side()
    }

    fn length_of_ribbon_needed(&self) -> u32 {
        self.perimeter_of_smallest_side() + self.volume()
    }

    fn new(input: &str) -> Present {
        let v: Vec<&str> = input.splitn(3, 'x').collect();
        Present {
            width:  v[0].parse::<u32>().unwrap(), 
            length: v[1].parse::<u32>().unwrap(), 
            height: v[2].parse::<u32>().unwrap()
        }
    }
}

fn main() {
    let present1 = Present::new("2x3x4");
    let present2 = Present::new("1x1x10");

    println!("Ribbon for present1 - Smallest side: {} - Bow: {} - Total: {}", 
        present1.perimeter_of_smallest_side(), 
        present1.volume(), 
        present1.length_of_ribbon_needed());

    println!("Ribbon for present2 - Smallest side: {} - Bow: {} - Total: {}", 
        present2.perimeter_of_smallest_side(), 
        present2.volume(), 
        present2.length_of_ribbon_needed());


    let presents = read_input("input.txt");
    let presents: Vec<&str> = presents.split('\n').collect();

    let mut wrapping_paper_needed = 0;

    for present in &presents {
        wrapping_paper_needed += Present::new(present).amount_of_wrapping_paper_needed();
    }

    println!("I have {} presents!", presents.len());
    println!("I need {} square feet of wrapping paper!", wrapping_paper_needed);
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
}