use std::collections::LinkedList;

pub enum Collider {
    World((usize, usize)),
    Apple((usize, usize)),
    Snake(LinkedList<(usize, usize)>),
}
