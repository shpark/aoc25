use std::{cell::RefCell, collections::HashMap, rc::Rc};

use itertools::Itertools;

// WARNING: Current implementation is too slow :(

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Coord {
    i: i64,
    j: i64,
}

impl Coord {
    fn area(&self, other: &Self) -> usize {
        let Coord { i: i1, j: j1 } = self;
        let Coord { i: i2, j: j2 } = other;

        ((i2 - i1).abs() as usize + 1) * ((j2 - j1).abs() as usize + 1)
    }
}

impl TryFrom<&str> for Coord {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts = s.split(',').collect::<Vec<_>>();

        if parts.len() < 2 {
            Err(())
        } else {
            let x = parts[0].parse::<i64>();
            let y = parts[1].parse::<i64>();
            match (x, y) {
                (Ok(x), Ok(y)) => Ok(Coord { i: x, j: y }),
                _ => Err(()),
            }
        }
    }
}

fn parse_coords() -> impl Iterator<Item = Coord> {
    std::io::stdin().lines().flatten()
        .filter_map(|line| {
            let parts = line.split(',').collect::<Vec<_>>();

            if parts.len() < 2 {
                None
            } else {
                Coord::try_from(line.as_str()).ok()
            }
        })
}

pub fn part1() -> i64 {
    parse_coords()
        .combinations(2)
        .map(|points| { points[0].area(&points[1]) as i64 })
        .max()
        .unwrap()
}

struct Floor {
    vertices: Vec<Coord>,
    cache: Rc<RefCell<HashMap<Coord, bool>>>,
}

impl Floor {
    fn new<'a>(vertices: impl Iterator<Item = Coord>) -> Self {
        let vertices = vertices.collect::<Vec<_>>();

        let cache = Rc::new(RefCell::new(HashMap::new()));

        for coord in vertices.iter() {
            cache.borrow_mut().insert(coord.clone(), true);
        }

        Self {
            vertices,
            cache,
        }
    }

    fn rect_edges(
        &self,
        p: Coord,
        q: Coord
    ) -> Vec<Coord> {
        let r = Coord {
            i: std::cmp::min(p.i, q.i),
            j: std::cmp::min(p.j, q.j)
        };
        let s = Coord {
            i: std::cmp::max(p.i, q.i),
            j: std::cmp::max(p.j, q.j)
        };

        let mut coords = Vec::new();

        (r.j + 1..s.j)
            .for_each(|j| {
                coords.push(Coord { i: r.i + 1, j });
                coords.push(Coord { i: s.i - 1, j });
            });

        (r.i + 1..s.i)
            .for_each(|i| {
                coords.push(Coord { i, j: r.j + 1 });
                coords.push(Coord { i, j: s.j - 1 });
            });

        coords
    }

    fn polygon_edges(&self) -> impl Iterator<Item = (Coord, Coord)> {
        self.vertices.iter().cloned().circular_tuple_windows::<(_, _)>()
    }

    fn is_coord_inside_polygon(&self, coord: Coord) -> bool {
        if let Some(&res) = self.cache.borrow().get(&coord) {
            return res;
        }

        let inside = {
            let mut inside = false;
            let Coord { i, j } = coord;

            // ray casting
            for (u, v) in self.polygon_edges() {
                let Coord { i: ui, j: uj } = u;
                let Coord { i: vi, j: vj } = v;

                if ui == vi {
                    // Handle horizontal edges.
                    let (min_j, max_j) = (std::cmp::min(uj, vj), std::cmp::max(uj, vj));

                    if i == ui && min_j <= j && j <= max_j {
                        inside = true;
                        break;
                    }
                } else if uj == vj {
                    // Handle vertical edges.
                    let (min_i, max_i) = (std::cmp::min(ui, vi), std::cmp::max(ui, vi));

                    // NOTE: test against half-open range to avoid double-counting...
                    if i < min_i || i >= max_i {
                        continue;
                    }

                    if j == uj {
                        inside = true;
                        break;
                    } else if j > uj {
                        inside = !inside;
                    }
                }
            }

            inside
        };

        self.cache.borrow_mut().insert(coord, inside);
        inside
    }
}

pub fn part2() -> i64 {
    let floor = Floor::new(parse_coords());

    let mut max_area = 0usize;

    let mut rectangles = floor.vertices.iter().cloned()
        .combinations(2).collect::<Vec<_>>();

    rectangles.sort_by(|rect1, rect2| {
        let area1 = rect1[0].area(&rect1[1]);
        let area2 = rect2[0].area(&rect2[1]);
        area2.cmp(&area1)
    });

    for i in 0..rectangles.len() {
        let (p, q) = (rectangles[i][0], rectangles[i][1]);

        let all_coords_are_inside_polygon = floor
            .rect_edges(p, q)
            .into_iter()
            .all(|coord| floor.is_coord_inside_polygon(coord));

        if all_coords_are_inside_polygon {
            max_area = std::cmp::max(max_area, p.area(&q));
            break;
        }
    }

    max_area as i64
}

#[cfg(test)]
mod test {
    use crate::y2025::day09::{Coord, Floor};

    #[test]
    fn test_floor() {
        let coords = vec![
            Coord { i: 7, j: 1 },
            Coord { i: 11, j: 1 },
            Coord { i: 11, j: 7 },
            Coord { i: 9, j: 7 },
            Coord { i: 9, j: 5 },
            Coord { i: 2, j: 5 },
            Coord { i: 2, j: 3 },
            Coord { i: 7, j: 3 },
        ];

        let floor = Floor::new(coords.iter().cloned());

        let edges = floor.polygon_edges().collect::<Vec<_>>();

        assert_eq!(edges.len(), coords.len())
    }
}
