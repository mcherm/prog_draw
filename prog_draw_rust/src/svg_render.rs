
use crate::svg_writer::{Renderable, TagWriter, TagWriterError, Attributes};
use geometry::{Coord, Rect};

// FIXME: Maybe this belongs somewhere else
pub mod geometry {
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

}


/// A trait for anything whose SVG dimensions can be measured and used to lay it
/// out.
pub trait SvgPositioned: Renderable {
    /// Returns a bounding box for this item. The bounding box is relative to the local
    /// coordinate system.
    fn get_bbox(&self) -> Rect;
}



pub struct BasicBox {
    x: Coord,
    y: Coord,
    height: Coord,
    width: Coord,
}

impl Renderable for BasicBox {
    fn render(&self, tag_writer: &mut dyn TagWriter) -> Result<(), TagWriterError> {
        tag_writer.single_tag("rect", Attributes::from([
            ("x", self.x.to_string()),
            ("y", self.y.to_string()),
            ("height", self.height.to_string()),
            ("width", self.width.to_string()),
            ("fill", "none".to_string()),
            ("stroke", "black".to_string()),
            ("stroke-width", 3.to_string()),
        ]))
    }
}

impl SvgPositioned for BasicBox {
    fn get_bbox(&self) -> Rect {
        Rect::new_ltwh(self.x, self.y, self.width, self.height)
    }
}



pub struct Group<'a> {
    pub items: Vec<&'a dyn SvgPositioned>,
    translate: Option<(Coord, Coord)>,
    scale: Option<Coord>,
}

impl<'a> Group<'a> {
    #![allow(dead_code)]


    pub fn new() -> Self {
        Group{items: Vec::new(), translate: None, scale: None}
    }

    pub fn item_transformed(item: &'a dyn SvgPositioned, translate: Option<(Coord,Coord)>, scale: Option<Coord>) -> Self {
        Group{items: vec![item], translate, scale}
    }

    pub fn add(&mut self, item: &'a dyn SvgPositioned) {
        self.items.push(item);
    }

    /// Call this to set the translate for the group.
    pub fn set_translate(&mut self, translate: Option<(Coord, Coord)>) {
        self.translate = translate;
    }

    /// Call this to set the scale factor for the group.
    pub fn set_scale(&mut self, scale: Option<Coord>) {
        self.scale = scale;
    }

    /// Returns the translate string.
    fn get_transform(&self) -> Option<String> {
        match (self.translate, self.scale) {
            (None, None) => None,
            (Some((x,y)), None) => Some(format!("translate({}, {})", x, y)),
            (None, Some(s)) => Some(format!("scale({})", s)),
            (Some((x,y)), Some(s)) => Some(format!("translate({}, {}) scale({})", x, y, s)),
        }
    }
}

impl<'a> Renderable for Group<'a> {
    fn render(&self, tag_writer: &mut dyn TagWriter) -> Result<(), TagWriterError> {
        let attributes = match &self.get_transform() {
            None => Attributes::new(),
            Some(transform) => Attributes::from([("transform", transform)]),
        };
        tag_writer.begin_tag("g", attributes)?;
        for item in self.items.iter() {
            item.render(tag_writer)?;
        }
        tag_writer.end_tag("g")?;
        Ok(())
    }
}


impl<'a> SvgPositioned for Group<'a> {
    fn get_bbox(&self) -> Rect {
        let mut r: Rect = self.items.iter()
            .map(|item| item.get_bbox())
            .reduce(|accum, rect| accum.cover(&rect))
            .unwrap_or(Rect::new_cwh((0.0, 0.0), 0.0, 0.0));
        if let Some(s) = self.scale {
            r.scale_about_center(s);
        }
        if let Some((dx, dy)) = self.translate {
            r.translate(dx, dy);
        }
        r
    }
}


impl<'a, const N: usize> From<[&'a dyn SvgPositioned; N]> for Group<'a> {
    fn from(arr: [&'a dyn SvgPositioned; N]) -> Self {
        let mut items: Vec<&'a dyn SvgPositioned> = Vec::with_capacity(N);
        for item in arr {
            items.push(item);
        }
        Group{items, translate: None, scale: None}
    }
}



pub struct Svg<'a> {
    content: Group<'a>,
    margin: Coord,
}

impl<'a> Svg<'a> {
    pub fn new(content: Group<'a>, margin: Coord) -> Self {
        Svg{content, margin}
    }
}

impl<'a> Renderable for Svg<'a> {
    fn render(&self, tag_writer: &mut dyn TagWriter) -> Result<(), TagWriterError> {
        let bbox = self.content.get_bbox();
        let viewbox: String = format_args!(
            "{} {} {} {}",
            bbox.left() - self.margin,
            bbox.top() - self.margin,
            bbox.width() + 2.0 * self.margin,
            bbox.height() + 2.0 * self.margin
        ).to_string();
        tag_writer.begin_tag("svg", Attributes::from([
            ("viewBox", &*viewbox),
            ("xmlns", "http://www.w3.org/2000/svg"),
        ]))?;
        self.content.render(tag_writer)?;
        tag_writer.end_tag("svg")?;
        Ok(())
    }
}

