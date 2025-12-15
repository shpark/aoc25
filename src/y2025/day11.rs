use std::collections::HashMap;

use crate::utils::dag::Dag;

struct Graph {
    you: String,
    out: String,
    svr: String,
    fft: String,
    dac: String,
    dag: Dag<String>,
}

fn parse_graph() -> Graph {
    let mut dag = Dag::new();

    std::io::stdin().lines().flatten()
        .for_each(|line| {
            match line.clone().split_once(": ") {
                Some((src, dst_edges)) => {
                    dst_edges.split(' ')
                        .for_each(|dst| {
                            dag.add_edge(&src.to_string(), &dst.to_string());
                        });
                },
                _ => {},
            }
        });

    Graph {
        you: String::from("you"),
        out: String::from("out"),
        svr: String::from("svr"),
        fft: String::from("fft"),
        dac: String::from("dac"),
        dag,
    }
}

impl Graph {
    fn dfs(
        &self,
        src: &String,
        dst: &String,
        node: &String,
        cache: &mut HashMap<String, usize>,
    ) -> usize {
        if node == dst {
            return 1;
        }

        if let Some(&num_paths) = cache.get(node) {
            return num_paths;
        }

        if node == src {
            cache.insert(node.clone(), 1);
        }

        let mut num_paths = 0usize;

        for neighbor in self.dag.neighbors(node) {
            num_paths += self.dfs(src, dst, &neighbor, cache);
        }

        cache.insert(node.clone(), num_paths);

        num_paths
    }

    fn num_unique_paths(
        &self,
        src: &String,
        dst: &String
    ) -> usize {
        let mut num_paths = HashMap::new();

        self.dfs(
            src,
            dst,
            src,
            &mut num_paths
        )
    }
}

pub fn part1() -> i64 {
    let g = parse_graph();

    g.num_unique_paths(&g.you, &g.out) as i64
}

pub fn part2() -> i64 {
    let g = parse_graph();

    let svr_to_fft = g.num_unique_paths(&g.svr, &g.fft);

    let fft_to_dac = g.num_unique_paths(&g.fft, &g.dac);

    let dac_to_out = g.num_unique_paths(&g.dac, &g.out);

    let svr_to_dac = g.num_unique_paths(&g.svr, &g.dac);

    let dac_to_fft = g.num_unique_paths(&g.dac, &g.fft);

    let fft_to_out = g.num_unique_paths(&g.fft, &g.out);

    (svr_to_fft * fft_to_dac * dac_to_out + svr_to_dac * dac_to_fft * fft_to_out) as i64
}