use std::cmp;

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

    fn amount_of_wrapping_paper_needed(&self) -> u32 {
        self.surface_area() + self.area_of_smallest_side()
    }
}

fn main() {
    let present1 =  Present { length: 2, width: 3, height: 4 };
    let present2 = Present { length: 1, width: 1, height: 10 };

    println!("Volume of present1 is: {}", present1.volume());
    println!("Surface area of present1 is: {}", present1.surface_area());
    println!("Surface area of smallest side of present1 is: {}", present1.area_of_smallest_side());
    println!("Amount of wrapping paper needed for present1 is: {}", present1.amount_of_wrapping_paper_needed());

    println!("Volume of present2 is: {}", present2.volume());
    println!("Surface area of present2 is: {}", present2.surface_area());
    println!("Surface area of smallest side of present2 is: {}", present2.area_of_smallest_side());
    println!("Amount of wrapping paper needed for present2 is: {}", present2.amount_of_wrapping_paper_needed());
}
