use std::cmp;

#[derive(PartialEq)]
#[derive(Eq)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}

impl Point {
  pub fn min(&self, other: &Point) -> Point {
    Point {
      x: cmp::min(self.x, other.x),
      y: cmp::min(self.y, other.y),
    }
  }

  pub fn max(&self, other: &Point) -> Point {
    Point {
      x: cmp::max(self.x, other.x),
      y: cmp::max(self.y, other.y),
    }
  }
}

#[derive(PartialEq)]
#[derive(Eq)]
pub struct Dimensions {
  pub w: u32,
  pub h: u32,
}

impl Dimensions {
  pub fn min(&self, other: &Dimensions) -> Dimensions {
    Dimensions {
      w: cmp::min(self.w, other.w),
      h: cmp::min(self.h, other.h),
    }
  }

  pub fn max(&self, other: &Dimensions) -> Dimensions {
    Dimensions {
      w: cmp::max(self.w, other.w),
      h: cmp::max(self.h, other.h),
    }
  }
}

pub struct Geometry {
  pub origin: Point,
  pub size: Dimensions,
}

impl Geometry {
  pub fn contains(&self, other: &Geometry) -> bool {
    self.origin.x <= other.origin.y && self.origin.y <= other.origin.y &&
    (self.origin.x + self.size.w as i32) >=
    (other.origin.x + other.size.w as i32) &&
    (self.origin.y + self.size.h as i32) >=
    (other.origin.y + other.size.h as i32)
  }
}

pub const POINT_ZERO: Point = Point { x: 0, y: 0 };

pub const SIZE_ZERO: Dimensions = Dimensions { w: 0, h: 0 };

pub const GEOMETRY_ZERO: Geometry = Geometry {
  origin: POINT_ZERO,
  size: SIZE_ZERO,
};
