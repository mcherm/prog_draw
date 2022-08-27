use std::collections::LinkedList;

use super::super::geometry::Coord;

pub struct LinkedYList {
    pub index: usize,
    y: Coord,
    next: Option<Box<LinkedYList>>,
}

impl LinkedYList {
    pub fn new(index: usize, y: Coord) -> Self {
        LinkedYList {
            index,
            y,
            next: None,
        }
    }

    pub fn bottom(&self) -> Coord {
        self.y
    }

    pub fn update(self, index: usize, y: Coord) -> Self {
        let mut node = self;
        while node.y <= y {
            if let Some(next) = node.next.take() {
                node = *next;
            } else {
                return LinkedYList {
                    index,
                    y,
                    next: None,
                };
            }
        }

        LinkedYList {
            index,
            y,
            next: Some(Box::new(node)),
        }
    }

    pub fn pop(mut self) -> Option<Self> {
        if let Some(next) = self.next.take() {
            return Some(*next);
        } else {
            return None;
        }
    }
}
