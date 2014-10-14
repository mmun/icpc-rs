#![feature(phase)]
#[phase(plugin)] extern crate scan;
extern crate scan_util;

fn main() {
    loop {
        match scanln! { n:uint => n } {
            Ok(n) if n > 0 => solve_case(n),
            _ => return
        }
    }
}

// This solution is based on the observeration that sums of the form
// w_1 + ... + w_n where w_i = a_i * v_i and a_i = +1 or -1 that are maximal
// (in length) will have the property that all w_i must lie in a single
// half-plane. Indeed, otherwise the sum could be increased by negating the
// a_i coefficient of any vector not in the same half-plane as the sum itself.
//
// This insight allows us to decrease the search space from 2 ^ n possible
// configurations of the a_i's to 2 * n configurations.
// 
// The algorithm is as follows and runs in O(n log n):
//
//      1. Construct a list, dirs (of size 2 * n), containing all v_i and -v_i.
//      2. Sort dirs by the angle each vector makes with the x-axis.
//      3. Loop over the n slices of n consecutive vectors. That is, the slices
//         dirs[0..n], dirs[1..n+1], dirs[n..2*n]. These slices correspond to
//         each of the possible partitions of the vectors by a half-plane.
//         i. Calculate the sum of the vectors in the slice.
//        ii. Keep track of the length of the longest sum seen so far.
//      4. Report the maximal length.

fn solve_case(n:uint) {
    let mut dirs: Vec<Dir> = Vec::with_capacity(2*n);

    for _ in range(0, n) {
        let (x, y) = scanln! { x:int y:int => (x, y) }.unwrap();

        dirs.push(Dir { x: x, y: y });
        dirs.push(Dir { x: -x, y: -y });
    }

    dirs.sort_by(|a, b| {
        a.theta().partial_cmp(&b.theta()).unwrap_or(Equal)
    });

    let mut curr_dir = Dir { x: 0, y: 0 };

    for i in range(0, n) {
        curr_dir.add_mut(dirs[i].x, dirs[i].y);
    }

    let mut max_dist = curr_dir.length_squared();

    for i in range(0, n) {
        curr_dir.sub_mut(dirs[i].x, dirs[i].y);
        curr_dir.add_mut(dirs[i+n].x, dirs[i+n].y);

        max_dist = std::cmp::max(max_dist, curr_dir.length_squared());
    }

    println!("Maximum distance = {:.3} meters.", (max_dist as f64).sqrt());
}

struct Dir { x: int, y: int }

impl Dir {
    fn add_mut(&mut self, dx:int, dy:int) {
        self.x += dx;
        self.y += dy;
    }

    fn sub_mut(&mut self, dx:int, dy:int) {
        self.x -= dx;
        self.y -= dy;
    }

    fn theta(&self) -> f64 {
        (self.y as f64).atan2(self.x as f64)
    }

    fn length_squared(&self) -> uint {
        let x = self.x as uint;
        let y = self.y as uint;

        x*x + y*y
    }
}
