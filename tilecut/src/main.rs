fn main() {
    let mut stdin = std::io::stdin();

    loop {
        let mut tile_grid = Vec::new();

        loop {
            let line = stdin.read_line().unwrap();
            let mut tile_row = unsafe { line.into_ascii_nocheck() };
            tile_row.pop();

            match tile_row.len() {
                0 => break,
                _ => tile_grid.push(tile_row)
            }
        }

        match tile_grid.len() {
            0 => break,
            _ => solve_case(tile_grid)
        }
    }
}

// This problem can be solved with a direct application of the
// Ford-Fulkerson maximum flow algorithm. For each tile we create
// an in-node, an out-node and an edge with capacity one that
// joins them. Flow through this edge represents choosing the
// associated tile as part of a triominoe.
//
// The source is connected to the in-node of each of the W-tiles
// while the out-node of each N-tiles is connected to the sink.
// Adjacent W-I and I-N pairs on the grid are connected accordingly.
// The resultant maximum flow from the source to the sink is then
// the maximum number of triominoes that can be cut from the grid.
//
// For simplicity, this implementation uses an adjacency matrix
// and selects augmenting paths through a depth-first search, but
// it could be sped up using adjacency lists and an alternative
// augmenting path selection algorithm.

fn solve_case(tile_grid:Vec<Vec<Ascii>>) {
    let w = tile_grid[0].len();
    let h = tile_grid.len();
    let ds:[(uint, uint), ..4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];

    let mut flow_network = FlowNetwork::new(2*w*h + 2);

    let source = 2*w*h;
    let sink = 2*w*h + 1;
    let out = w*h;

    for y in range(0, h) {
        for x in range(0, w) {
            flow_network.capacity[y*w+x][out+y*w+x] = 1;
            match tile_grid[y][x].to_char() {
                'W' => flow_network.capacity[source][y*w+x] = 1,
                'N' => flow_network.capacity[out+y*w+x][sink] = 1,
                'I' => for &(dx, dy) in ds.iter() {
                    let (nx, ny) = (x + dx, y + dy);
                    if nx == -1 || nx == w || ny == -1 || ny == h { continue }
                    match tile_grid[ny][nx].to_char() {
                        'W' => flow_network.capacity[out+ny*w+nx][y*w+x] = 1,
                        'N' => flow_network.capacity[out+y*w+x][ny*w+nx] = 1,
                        _ => {}
                    }
                },
                _ => {}
            }
        }
    }

    println!("{}", flow_network.get_max_flow(source, sink));
}

struct FlowNetwork {
    size: uint,
    capacity: Vec<Vec<uint>>
}

impl FlowNetwork {
    fn new(size:uint) -> FlowNetwork {
        FlowNetwork {
            size: size,
            capacity: Vec::from_fn(size, |_| { Vec::from_elem(size, 0) })
        }
    }

    fn augment(self:&mut FlowNetwork, seen: &mut Vec<bool>, current: uint, sink: uint, flow: uint) -> uint {
        if current == sink { return flow }

        seen[current] = true;
        for i in range(0, self.size) {
            if !(*seen)[i] && self.capacity[current][i] > 0 {
                let min_capacity = std::cmp::min(flow, self.capacity[current][i]);
                let flow = self.augment(seen, i, sink, min_capacity);
                if flow > 0 {
                    self.capacity[current][i] -= flow;
                    self.capacity[i][current] += flow;
                    return flow
                }
            }
        }

        return 0
    }

    fn get_max_flow(self:&mut FlowNetwork, source:uint, sink:uint) -> uint {
        let mut max_flow = 0u;
        loop {
            let mut seen = Vec::from_elem(self.size, false);
            match self.augment(&mut seen, source, sink, -1) {
                0 => return max_flow,
                f => max_flow += f
            }
        }
    }
}