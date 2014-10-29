#![feature(phase)]
#[phase(plugin)] extern crate scan;
extern crate scan_util;

use std::cmp;

const MAX_D:uint = 1000000000000;

fn main() {
    let t = scanln! { t:uint => t }.unwrap();
    for _ in range(0, t) {
        let (n, k, m) = scanln! { n:uint k:uint m:uint => (n, k, m) }.unwrap();
        solve_case(n, k, m);
    }
}

// First we read the road edges into an adjacency matrix M and run the
// Floyd-Warshall algorithm create a new adjacency matrix M' with extra edges
// that represent the shortest distance between and pair of vertices.
// This means that the optimal path between two schools in M' will have
// vertices only at the stops, which is not necessarily true of the optimal
// path in M.
//
// We proceed by doing a binary search for the minimal, valid battery range in
// the range (1, longest distance in M'). For each candidate battery range R we
// consider the subgraph G of M' whose edges have distance <= R. The diameter
// of the unweighted version of G is precisely the minimal number of stops
// required to get between any two schools. If the diameter is <= K, then the
// range is sufficient and we continue the binary search for smaller ranges.
// Otherwise, the range is insufficient and we continue the binary search in
// the larger range.

fn solve_case(n:uint, k:uint, m:uint) {
    let mut adj:Vec<uint> = Vec::from_elem(n*n, MAX_D);
    let mut min_stops:Vec<uint> = Vec::from_elem(n*n, 0);

    for _ in range(0, m) {
        let (n1, n2, d) = scanln! { n1:uint n2:uint d:uint => (n1, n2, d) }.unwrap();
        adj[n1*n+n2] = d;
        adj[n2*n+n1] = d;
    }

    floyd_warshall(&mut adj, n);

    let mut min = 1u;
    let mut max = *adj.iter().max_by(|x| *x).unwrap();
    let mut best_range = MAX_D;

    while min <= max {
        let mid = (min + max) / 2;

        for i in range(0, n) {
            for j in range(0, n) {
                let is_close = adj[i*n+j] <= mid;
                min_stops[i*n+j] = if is_close { 1 } else { n+1 };
            }
        }

        floyd_warshall(&mut min_stops, n);

        let diam = *min_stops.iter().max_by(|x| *x).unwrap();
        if diam <= k {
            best_range = mid;
            max = mid - 1;
        } else {
            min = mid + 1;
        }
    }

    println!("{}", best_range);
}

fn floyd_warshall(adj:&mut Vec<uint>, n:uint) {
    for k in range(0, n) {
        for i in range(0, n) {
            for j in range(0, n) {
                let a_ij = (*adj)[i*n+j];
                let a_ik = (*adj)[i*n+k];
                let a_kj = (*adj)[k*n+j];
                (*adj)[i*n+j] = cmp::min(a_ij, a_ik + a_kj);
            }
        }
    }
}
