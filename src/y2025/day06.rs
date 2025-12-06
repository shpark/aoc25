#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

impl TryFrom<char> for Op {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '*' => Ok(Op::Mul),
            '+' => Ok(Op::Add),
            _ => Err(()),
        }
    }
}

// NOTE: turns out builder pattern was not very helpful for this problem,
// but won't fix :)
struct Problem {
    op: Op,
    operands: Vec<u64>,
}

impl Problem {
    fn eval(&self) -> u64 {
        match self.op {
            Op::Add => self.operands.iter().sum(),
            Op::Mul => self.operands.iter().product()
        }
    }
}

struct ProblemBuilder {
    op: Option<Op>,
    operands: Vec<u64>,
}

impl ProblemBuilder {
    fn new() -> Self {
        Self {
            op: None,
            operands: vec![],
        }
    }

    fn push_operand(&mut self, operand: u64) {
        self.operands.push(operand);
    }

    fn set_op(&mut self, op: Op) {
        self.op.replace(op);
    }

    fn build(self) -> Problem {
        Problem {
            op: self.op.unwrap(),
            operands: self.operands,
        }
    }
}

pub fn part1() -> i64 {
    let mut builders = vec![];

    std::io::stdin().lines().flatten()
        .for_each(|line| {
            line.split_whitespace().enumerate().for_each(|(i, part)| {

                if builders.len() < i + 1 {
                    builders.push(ProblemBuilder::new());
                }

                if let Some(builder) = builders.get_mut(i) {
                    if let Ok(operand) = part.parse::<u64>() {
                        builder.push_operand(operand);
                    } else if let Ok(op) = Op::try_from(part.chars().take(1).collect::<Vec<_>>()[0]) {
                        builder.set_op(op);
                    }
                }
            })
        });

    builders.into_iter()
        .map(|builder| {
            builder.build().eval()
        })
        .sum::<u64>() as i64
}

fn get_operand(s: &str) -> Option<u64> {
    let operands = s.split_whitespace()
        .filter_map(|part| part.parse::<u64>().ok())
        .collect::<Vec<_>>();

    if operands.len() > 0 {
        Some(operands[0])
    } else {
        None
    }
}

fn step(rows: &mut Vec<String>) -> Result<(Option<u64>, Option<Op>), ()> {
    let mut s = rows.iter_mut().filter_map(|row| row.pop()).collect::<String>();

    if s.len() < rows.len() {
        return Err(());
    }

    let mut op = s.split_off(s.len() - 1);

    Ok((
        get_operand(&s),
        Op::try_from(op.pop().unwrap()).ok(),
    ))
}

pub fn part2() -> i64 {
    let mut rows = std::io::stdin().lines().flatten().collect::<Vec<_>>();

    let mut operands: Vec<u64> = vec![];
    let mut res = 0i64;

    while let Ok((operand, op)) = step(&mut rows) {
        match (operand, op) {
            (Some(operand), None) => {
                operands.push(operand);
            },
            (Some(operand), Some(op)) => {
                operands.push(operand);
                res += match op {
                    Op::Add => operands.iter().sum::<u64>(),
                    Op::Mul => operands.iter().product::<u64>(),
                } as i64;
            },
            (None, _) => {
                operands.clear();
            }
        }
    }

    res
}

#[cfg(test)]
mod test {
    use crate::y2025::day06::get_operand;

    #[test]
    fn test_day6_get_operand() {
        assert_eq!(get_operand("  3 "), Some(3));
        assert_eq!(get_operand(" 23 "), Some(23));
        assert_eq!(get_operand("123 "), Some(123));
    }
}