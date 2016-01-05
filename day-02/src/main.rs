use std::io;
use std::io::prelude::*;

struct GiftBox {
    l: i32,
    w: i32,
    h: i32,
}

impl GiftBox {
    fn from_string(params: &str) -> Result<GiftBox, &str> {
        let params: Vec<_> = params.trim().split("x").map(|dim| { dim.parse::<i32>().unwrap() }).collect();

        let gift_box = GiftBox { l: params[0], w: params[1], h: params[2] };
        Ok(gift_box)
    }

    fn paper_required(&self) -> i32 {
        self.area() + self.smallest_side_area()
    }

    fn area(&self) -> i32 {
        (2 * self.l * self.w) + (2 * self.w * self.h) + (2 * self.h * self.l)
    }

    fn smallest_side_area(&self) -> i32 {
        *([self.l * self.w, self.w * self.h, self.h * self.l].iter().min().unwrap())
    }
}

fn main() {
    let stdin = io::stdin();

    let paper_required = stdin.lock().lines().map(|line| {
        let line = line.unwrap();
        GiftBox::from_string(&line).unwrap().paper_required()
    }).fold(0, std::ops::Add::add);

    println!("Paper required: {}", paper_required);
}
