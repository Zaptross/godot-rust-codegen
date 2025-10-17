#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Physics2d {
    COLLISIONS = 1,
    NONCOLLIDING = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Render2d {
    GHOSTS = 1,
}
