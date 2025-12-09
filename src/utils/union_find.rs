use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Rc<RefCell<UfNode<T>>>;

pub struct UnionFind<T>(pub Link<T>);

pub struct UfNode<T> {
    data: T,
    parent: Option<Link<T>>,
    rank: u64,
}

impl<T> UnionFind<T> {
    pub fn new(data: T) -> Self {
        Self(Rc::new(RefCell::new(UfNode {
            data,
            parent: None,
            rank: 0,
        })))
    }

    pub fn find(&self) -> Self {
        let parent = self.0.borrow().parent.clone();
        match parent {
            None => Self(Rc::clone(&self.0)),
            Some(p) => {
                let root = UnionFind(p).find();
                self.0.borrow_mut().parent = Some(Rc::clone(&root.0));
                root
            }
        }
    }

    pub fn union(&mut self, other: &mut UnionFind<T>) -> bool {
        let px = self.find();
        let py = other.find();

        if Rc::ptr_eq(&px.0, &py.0) {
            return false;
        }

        let rank_x = px.0.borrow().rank;
        let rank_y = py.0.borrow().rank;

        match rank_x.cmp(&rank_y) {
            std::cmp::Ordering::Less => px.0.borrow_mut().parent = Some(py.0),
            std::cmp::Ordering::Greater => py.0.borrow_mut().parent = Some(px.0),
            std::cmp::Ordering::Equal => {
                py.0.borrow_mut().parent = Some(Rc::clone(&px.0));
                px.0.borrow_mut().rank += 1;
            },
        }

        true
    }

    pub fn data(&self) -> std::cell::Ref<'_, T> {
        std::cell::Ref::map(self.0.borrow(), |node| &node.data)
    }
}


#[cfg(test)]
mod test {
    use std::rc::Rc;

    use crate::utils::union_find::UnionFind;

    #[test]
    fn test_union_find_simple() {
        let mut a = UnionFind::new('a');
        let mut b = UnionFind::new('b');
        let mut c = UnionFind::new('b');

        assert_eq!(a.union(&mut b), true);

        assert_eq!(a.0.borrow().rank, 1);
        assert_eq!(b.0.borrow().rank, 0);
        assert!(Rc::ptr_eq(&a.find().0, &a.0));
        assert!(Rc::ptr_eq(&b.find().0, &a.0));

        assert_eq!(b.union(&mut c), true);

        assert_eq!(a.0.borrow().rank, 1);
        assert_eq!(b.0.borrow().rank, 0);
        assert_eq!(c.0.borrow().rank, 0);

        assert!(Rc::ptr_eq(&a.find().0, &a.0));
        assert!(Rc::ptr_eq(&b.find().0, &a.0));
        assert!(Rc::ptr_eq(&c.find().0, &a.0));
    }
}