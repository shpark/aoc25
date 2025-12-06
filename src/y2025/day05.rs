use crate::utils::interval_tree::IntervalTree;

pub fn part1() -> i64 {
    let mut itree = IntervalTree::<u64>::new();
    let mut ans = 0i64;

    std::io::stdin().lines().flatten().for_each(|line| {
        if line.contains('-') {
            let (lower, upper) = {
                let parts = line.split('-')
                    .filter_map(|part| part.parse::<u64>().ok())
                    .collect::<Vec<_>>();
                (parts[0], parts[1] + 1) // convert ranges to [lower, upper)
            };

         _ = itree.insert(lower, upper);
        }
        else {
            if let Ok(id) = line.parse::<u64>() {
                if itree.query(id) > 0 {
                    ans += 1;
                }
            }
        }
    });

    ans
}

pub fn part2() -> i64 {
    let mut ranges = std::io::stdin().lines().flatten()
        .filter_map(|line| {
            if line.contains('-') {
                let (lower, upper) = {
                    let parts = line.split('-')
                        .filter_map(|part| part.parse::<u64>().ok())
                        .collect::<Vec<_>>();
                    (parts[0], parts[1] + 1) // convert ranges to [lower, upper)
                };

                Some((lower, upper))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    ranges.sort_unstable_by_key(|&(start, _)| start);

    // NOTE: ranges are left-inclusive, i.e., [start, end)
    ranges.into_iter().fold((0u64, 0u64), |(mut acc, max_end), (start, end)| {
        if end > max_end {
            acc += end - std::cmp::max(start, max_end);
        }

        (acc, std::cmp::max(max_end, end))
    }).0 as i64
}