const N: i32 = 100;

fn parse_rotations() -> impl Iterator<Item = i32> {
    std::io::stdin().lines()
        .filter_map(|line| {
            match line {
                Err(_) => None,
                Ok(line) => Some({
                    let sign = match line.chars().nth(0).unwrap() {
                        'L' => -1,
                        'R' => 1,
                        _ => return None,
                    };
                    sign * line[1..].parse::<i32>().unwrap()
                }),
            }
        })
}

pub fn part1() -> i64 {
    let mut curr = 50i32;
    let mut password = 0i64;

    for delta in parse_rotations() {
        curr = (curr + delta).rem_euclid(N);

        // check if the dial points to zero as a result of current rotation.
        if curr == 0 {
            password += 1;
        }
    }

    password
}

pub fn part2() -> i64 {
    let mut curr = 50i32;
    let mut next;
    let mut password = 0i64;

    for delta in parse_rotations() {
        // if `N * m <= amount < N * (m + 1)`, then the dial shall point to
        // zero for `m` times.
        password += (delta.abs() / N) as i64;

        next = curr + delta % N;

        // check if remaining clicks cause an underflow or overflow
        if curr * next < 0 || next > N {
            password += 1;
        }

        curr = next.rem_euclid(N);

        // check if the dial points to zero as a result of current rotation.
        if curr == 0 {
            password += 1;
        }
    }

    password
}
