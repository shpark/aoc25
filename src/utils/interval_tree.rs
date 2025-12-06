struct Node<T>
    where T: PartialEq +  PartialOrd + Ord + Copy
{
    start: T,
    end: T,
    max_end: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: PartialEq + PartialOrd + Ord + Copy> Node<T> {
    fn new(start: T, end: T) -> Node<T> {
        Self {
            start,
            end,
            max_end: end,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, start: T, end: T) -> Result<(), ()> {
        self.max_end = std::cmp::max(self.max_end, end);

        let cursor = if start >= self.start {
            &mut self.right
        } else {
            &mut self.left
        };

        match cursor {
            Some(node) => {
                node.insert(start, end)
            }
            _ => {
                *cursor = Some(Box::new(Self::new(start, end)));
                Ok(())
            }
        }
    }

    fn query(&self, point: T) -> usize {
        if point >= self.max_end {
            return 0;
        }

        let mut num_matches = 0usize;

        if let Some(left) = &self.left {
            num_matches += left.query(point);
        }

        if self.start <= point && point < self.end {
            num_matches += 1;
        }

        if let Some(right) = &self.right {
            num_matches += right.query(point);
        }

        num_matches
    }

    fn count(&self) -> usize {
        let mut res = 0usize;

        if let Some(left) = &self.left {
            res += left.count();
        }

        res += 1;

        if let Some(right) = &self.right {
            res += right.count();
        }

        res
    }
}

pub struct IntervalTree<T>
    where T: PartialEq + PartialOrd + Ord + Copy
{
    root: Option<Box<Node<T>>>,
}

impl<T: PartialEq + PartialOrd + Ord + Copy> IntervalTree<T> {
    pub fn new() -> Self {
        Self {
            root: None,
        }
    }

    pub fn insert(&mut self, start: T, end: T) -> Result<(), ()> {
        match &mut self.root {
            Some(node) => node.insert(start, end),
            None => {
                self.root = Some(Box::new(Node::new(start, end)));
                Ok(())
            },
        }
    }

    pub fn query(&self, point: T) -> usize {
        if let Some(root) = &self.root {
            root.query(point)
        } else {
            0
        }
    }

    pub fn count(&self) -> usize {
        if let Some(root) = &self.root {
            root.count()
        } else {
            0
        }
    }
}

#[cfg(test)]
mod test {
    use crate::utils::interval_tree::IntervalTree;

    #[test]
    fn test_itree_simple() {
        let mut itree = IntervalTree::<i32>::new();

        _ = itree.insert(5, 10); // 5, 6, 7, 8, 9
        _ = itree.insert(3, 6); // 3, 4, 5
        _ = itree.insert(7, 13); // 7, 8, 9, 10, 11, 12
        _ = itree.insert(14, 18); // 14, 15, 16, 17
        _ = itree.insert(1, 4); // 1, 2, 3

        assert_eq!(itree.query(2), 1);
        assert_eq!(itree.query(3), 2);
        assert_eq!(itree.query(5), 2);
        assert_eq!(itree.query(8), 2);
        assert_eq!(itree.query(11), 1);
    }
}