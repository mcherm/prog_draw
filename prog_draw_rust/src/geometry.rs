
// FIXME: Should I define my own? Pull from somewhere else? Should it be usize instead?
pub type Coord = f64;

pub type Point = (Coord, Coord);

#[derive(Debug, Copy, Clone)]
pub struct Rect {
    left: Coord,
    top: Coord,
    width: Coord,
    height: Coord,
}

impl Rect {
    #![allow(dead_code)]

    pub fn new_ltwh(left: Coord, top: Coord, width: Coord, height: Coord) -> Self {
        Rect{left, top, width, height}
    }

    pub fn new_cwh(center: Point, width: Coord, height: Coord) -> Self {
        Rect{
            left: center.0 - width / 2.0,
            top: center.1 - height / 2.0,
            width,
            height
        }
    }

    pub fn new_ltrb(left: Coord, top: Coord, right: Coord, bottom: Coord) -> Self {
        Rect{left, top, width: right - left, height: bottom - top}
    }

    pub fn top(&self) -> Coord {self.top}
    pub fn left(&self) -> Coord {self.left}
    pub fn right(&self) -> Coord {self.left + self.width}
    pub fn bottom(&self) -> Coord {self.top + self.height}
    pub fn width(&self) -> Coord {self.width}
    pub fn height(&self) -> Coord {self.height}
    pub fn center(&self) -> Point {(self.left + self.width / 2.0, self.top + self.height / 2.0)}

    /// Returns the smallest Rect that covers both self and other.
    pub fn cover(&self, other: &Rect) -> Rect {
        Rect::new_ltrb(
            f64::min(self.left(), other.left()),
            f64::min(self.top(), other.top()),
            f64::max(self.right(), other.right()),
            f64::max(self.bottom(), other.bottom()),
        )
    }

    /// Translates this Rect by (dx,dy).
    pub fn translate(&mut self, dx: Coord, dy: Coord) {
        self.left += dx;
        self.top += dy;
    }

    /// Returns a new Rect that is the old one but translated by (dx,dy).
    pub fn translated(&self, dx: Coord, dy: Coord) -> Self {
        Rect{left: self.left + dx, top: self.top + dy, width: self.width, height: self.height}
    }

    /// Scales this Rect by s, keeping the center fixed.
    pub fn scale_about_center(&mut self, s: Coord) {
        let center_adj = (1.0 - s) / 2.0;
        self.left += self.width * center_adj;
        self.top += self.height * center_adj;
        self.width *= s;
        self.height *= s;
    }

    /// Scales this Rect by s, keeping the center fixed.
    pub fn scaled_about_center(&mut self, s: Coord) -> Self {
        Rect::new_cwh(self.center(), s * self.width, s * self.height)
    }

}
