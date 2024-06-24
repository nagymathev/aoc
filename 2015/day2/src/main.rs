use std::fs;
use std::env;

struct Rect(u32, u32, u32);

impl Rect {
    fn area(&self) -> u32 {
        2 * (self.0 * self.1 + self.0 * self.2 + self.1 * self.2)
    }

    fn extra(&self) -> u32 {
        (self.0 * self.1).min(self.0 * self.2).min(self.1 * self.2)
    }

    fn ribbon(&self) -> u32 {
        let mut dims = [self.0, self.1, self.2];
        dims.sort();

        2*dims[0] + 2*dims[1]
    }

    fn ribbon_bow(&self) -> u32 {
        self.0 * self.1 * self.2
    }

    fn total(&self) -> u32 {
        self.area() + self.extra()
    }

    fn total_ribbon(&self) -> u32 {
        self.ribbon() + self.ribbon_bow()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = match fs::read_to_string(&args[1]) {
        Ok(s) => {
            println!("[DONE] File successfully read: [{}]", &args[1]);
            s.trim().to_owned()
        },
        Err(_) => {
            println!("[ERROR] File could not be read: [{}]", &args[1]);
            return;
        }
    };

    let mut rects: Vec<Rect> = Vec::new();
    let mut total_paper_needed = 0;
    let mut total_ribbon = 0;

    let split: Vec<&str> = input.split("\n").collect();
    for eq in split {
        let dims: Vec<&str> = eq.split("x").collect();
        let dims: Vec<u32> = dims.iter().map(|x| x.parse().unwrap()).collect();

        rects.push(Rect(dims[0], dims[1], dims[2]));
    }

    for r in rects {
        total_paper_needed += r.total();
        total_ribbon += r.total_ribbon();
    }

    println!("[DONE] Square feet of paper needed: [{total_paper_needed}]");
    println!("[DONE] Total feet of ribbon needed: [{total_ribbon}]");


}
