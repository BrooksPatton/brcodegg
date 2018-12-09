extern crate ggez;

use std::ops::AddAssign;
use ggez::graphics::Point2 as GPoint2;
use std::clone::Clone;

#[derive(Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point {
            x,
            y
        }
    }

    pub fn get_ggez_point2(&self) -> GPoint2 {
        GPoint2::new(self.x, self.y)
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
            y: self.y
        }
    }
}

#[cfg(test)]
#[test]
fn new_point() {
    let x = 5.0;
    let y = 10.0;
    let point = Point::new(x, y);

    assert_eq!(point.x, x);
    assert_eq!(point.y, y);
}

#[test]
fn add_assign_points() {
    let mut point_1 = Point::new(1.0, 2.0);
    let point_2 = Point::new(5.0, 6.0);

    point_1 += point_2;

    assert_eq!(point_1.x, 6.0);
    assert_eq!(point_1.y, 8.0);
}

#[test]
fn get_ggez_point2() {
    let point = Point::new(1.0, 2.0);
    let ggez_point2 = GPoint2::new(1.0, 2.0);

    assert_eq!(point.get_ggez_point2(), ggez_point2);
}