use std::ops::{Add, Sub};

use super::map::TPoint;

pub trait Movement<T: Sized + Add + Sub> {
    fn get_speed(&self) -> T;

    fn left(&mut self);
    fn right(&mut self);
    fn up(&mut self);
    fn down(&mut self);
}

pub trait NextPos<T: Add<Output = T> + Sub<Output = T>>: Movement<T> + TPoint<T> {
    fn left_point(&self) -> T {
        self.get_x() - self.get_speed()
    }
    fn right_point(&self) -> T {
        self.get_x() + self.get_speed()
    }
    fn up_point(&self) -> T {
        self.get_y() - self.get_speed()
    }
    fn down_point(&self) -> T {
        self.get_y() + self.get_speed()
    }
}
