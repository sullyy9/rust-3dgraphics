//! Trait Providing an interface for Polygon types.
//! 

use super::geometry::{Point, Vector};

pub trait Polygonal {
    fn vertex_count(&self) -> usize;
    fn verticies(&self) -> &[&Point<4>];
    fn normal(&self) -> Vector<3>;
}