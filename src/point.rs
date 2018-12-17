extern crate ggez;

use ggez::graphics::Point2 as GPoint2;
use std::clone::Clone;
use std::cmp::PartialEq;
use std::ops::AddAssign;

#[derive(Serialize, Deserialize, Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

// impl Point<f32> {
//     pub fn new(x: f32, y: f32) -> Point<f32> {
//         Point { x, y }
//     }

//     pub fn get_ggez_point2(&self) -> GPoint2 {
//         GPoint2::new(self.x, self.y)
//     }
// }

impl Point<u16> {
    pub fn new(x: u16, y: u16) -> Point<u16> {
        Point { x, y }
    }

    pub fn to_ggez_point2(&self) -> GPoint2 {
        GPoint2::new(self.x.into(), self.y.into())
    }
}

impl AddAssign for Point<u16> {
    fn add_assign(&mut self, other_point: Point<u16>) {
        self.x += other_point.x;
        self.y += other_point.y;
    }
}

impl Clone for Point<u16> {
    fn clone(&self) -> Point<u16> {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

impl PartialEq for Point<u16> {
    fn eq(&self, point: &Point<u16>) -> bool {
        self.x == point.x && self.y == point.y
    }
}

#[cfg(test)]
#[test]
fn new_point() {
    let x = 5;
    let y = 10;
    let point = Point::<u16>::new(x, y);

    assert_eq!(point.x, x);
    assert_eq!(point.y, y);
}

#[test]
fn add_assign_points() {
    let mut point_1 = Point::<u16>::new(1, 2);
    let point_2 = Point::<u16>::new(5, 6);

    point_1 += point_2;

    assert_eq!(point_1.x, 6);
    assert_eq!(point_1.y, 8);
}

#[test]
fn get_ggez_point2() {
    let point = Point::<u16>::new(1, 2);
    let ggez_point2 = GPoint2::new(1.0, 2.0);

    assert_eq!(point.to_ggez_point2(), ggez_point2);
}

#[test]
fn clone() {
    let point_1 = Point::<u16>::new(1, 2);
    let cloned_point = point_1.clone();

    assert_eq!(point_1, cloned_point);
}

#[test]
fn partial_eq() {
    let point_1 = Point::<u16>::new(1, 2);
    let point_2 = Point::<u16>::new(1, 2);

    assert_eq!(point_1, point_2);
}
