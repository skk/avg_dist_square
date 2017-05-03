use std::ops::Sub;
use std::fmt;


extern crate rand;
use rand::distributions::{IndependentSample, Range};

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}


impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


fn rand_point() -> Point {
    let between = Range::new(0f64, 1f64);
    let mut rng = rand::thread_rng();
    let x = between.ind_sample(&mut rng) as f64;

    rng = rand::thread_rng();
    let y = between.ind_sample(&mut rng) as f64;
    // println!("x {}, y {}", x, y);
    let point = Point { x: x, y: y };
    return point;
}

fn main() {
    let count = 1000;
    let mut seq = 0;
    let mut sum = 0.0f64;;

    while seq <= count {
        let p1 = rand_point();
        let p2 = rand_point();
        let point_diff = p1 - p2;
        let diff = (point_diff.x.powi(2) + point_diff.y.powi(2)).abs().sqrt();
        sum += diff;
        seq += 1;
    }

    let estimate_dist = sum / (count as f64);

    let two: f64 = 2.0;
    let sqrt2: f64 = two.sqrt();

    let exact_dist = (2.0 + sqrt2 + ((sqrt2 + 1.0).ln())*5.0) / 15.0;
    println!("estimate_dist {}", estimate_dist);
    println!("   exact_dist {}", exact_dist);
}
