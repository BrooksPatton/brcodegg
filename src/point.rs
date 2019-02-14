extern crate ggez;

use ggez::graphics::Point2 as GPoint2;
use std::clone::Clone;
use std::cmp::PartialEq;
use std::ops::{AddAssign};

#[derive(Serialize, Deserialize, Debug, Eq, Hash)]
pub struct Point {
    pub x: u16,
    pub y: u16
}

impl Point {
    pub fn new(x: u16, y: u16) -> Point {
        Point { x, y }
    }

    pub fn to_ggez_point2(&self) -> GPoint2 {
        GPoint2::new(self.x.into(), self.y.into())
    }

    pub fn multiply(&self, other_point: &Point) -> Point {
        Point::new(self.x * other_point.x, self.y * other_point.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other_point: Point) {
        self.x += other_point.x;
        self.y += other_point.y;
    }
}

impl Clone for Point {
    fn clone(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, point: &Point) -> bool {
        self.x == point.x && self.y == point.y
    }
}

#[cfg(test)]
#[test]
fn new_point() {
    let x = 5;
    let y = 10;
    let point = Point::new(x, y);

    assert_eq!(point.x, x);
    assert_eq!(point.y, y);
}

#[test]
fn add_assign_points() {
    let mut point_1 = Point::new(1, 2);
    let point_2 = Point::new(5, 6);

    point_1 += point_2;

    assert_eq!(point_1.x, 6);
    assert_eq!(point_1.y, 8);
}

#[test]
fn get_ggez_point2() {
    let point = Point::new(1, 2);
    let ggez_point2 = GPoint2::new(1.0, 2.0);

    assert_eq!(point.to_ggez_point2(), ggez_point2);
}

#[test]
fn clone() {
    let point_1 = Point::new(1, 2);
    let cloned_point = point_1.clone();

    assert_eq!(point_1, cloned_point);
}

#[test]
fn partial_eq() {
    let point_1 = Point::new(1, 2);
    let point_2 = Point::new(1, 2);

    assert_eq!(point_1, point_2);
}

#[test]
fn multiplication() {
    let point_1 = Point::new(2, 4);
    let point_2 = Point::new(3, 6);
    let expected_result = Point::new(6, 24);
    
    assert_eq!(point_1.multiply(&point_2), expected_result);
}