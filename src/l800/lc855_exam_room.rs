use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap},
};

#[allow(unused)]
struct ExamRoom {
    n: i32,
    tree: BTreeSet<Seat>,
    left: HashMap<i32, i32>,
    right: HashMap<i32, i32>,
}

#[derive(Clone)]
struct Seat {
    x: i32,
    y: i32,
    n: i32,
}

impl Seat {
    fn dist(&self) -> i32 {
        if self.x == -1 {
            return self.y;
        }
        if self.y == self.n {
            return self.n - 1 - self.x;
        }
        (self.y - self.x) / 2
    }
}

impl PartialEq for Seat {
    fn eq(&self, other: &Self) -> bool {
        self.dist() == other.dist() && self.x == other.x
    }
}

impl Eq for Seat {}

impl PartialOrd for Seat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Seat {
    fn cmp(&self, other: &Self) -> Ordering {
        let dist_a = self.dist();
        let dist_b = other.dist();
        if dist_a == dist_b {
            return self.x.cmp(&other.x);
        }
        dist_b.cmp(&dist_a)
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(unused)]
impl ExamRoom {
    fn new(n: i32) -> Self {
        let mut room = ExamRoom {
            n,
            tree: BTreeSet::new(),
            left: HashMap::new(),
            right: HashMap::new(),
        };
        room.add(-1, n);
        room
    }

    fn seat(&mut self) -> i32 {
        let seat = self.tree.pop_first().unwrap();
        let p = if seat.x == -1 {
            0
        } else if seat.y == self.n {
            self.n - 1
        } else {
            (seat.x + seat.y) / 2
        };

        self.del(seat.x, seat.y);
        self.add(seat.x, p);
        self.add(p, seat.y);
        p
    }

    fn leave(&mut self, p: i32) {
        let l = *self.left.get(&p).unwrap();
        let r = *self.right.get(&p).unwrap();
        self.del(l, p);
        self.del(p, r);
        self.add(l, r);
    }

    fn add(&mut self, x: i32, y: i32) {
        let seat = Seat { x, y, n: self.n };
        self.tree.insert(seat);
        self.left.insert(y, x);
        self.right.insert(x, y);
    }

    fn del(&mut self, x: i32, y: i32) {
        let seat = Seat { x, y, n: self.n };
        self.tree.remove(&seat);
        self.left.remove(&y);
        self.right.remove(&x);
    }
}
