
use crate::svg_writer::{Renderable, TagWriter, TagWriterError, Attributes, Context};
use std::fs::File;
use geometry::{Coord, Rect};

// FIXME: Maybe this belongs somewhere else
pub mod geometry {
    // FIXME: Should I define my own? Pull from somewhere else? Should it be usize instead?
    pub type Coord = f64;

    pub type Point = (Coord, Coord);

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
    }

}


/// A trait for anything whose SVG dimensions can be measured and used to lay it
/// out.
pub trait SvgPositioned: Renderable {
    /// Returns a bounding box for this item. The bounding box is relative to the local
    /// coordinate system.
    fn get_bbox(&self, context: &mut Context) -> Rect;
}



pub struct BasicBox {
    x: Coord,
    y: Coord,
    height: Coord,
    width: Coord,
}

impl Renderable for BasicBox {
    fn render(&self, tag_writer: &mut TagWriter, _context: &mut Context) -> Result<(), TagWriterError> {
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
    fn get_bbox(&self, _context: &mut Context) -> Rect {
        Rect::new_ltwh(self.x, self.y, self.width, self.height)
    }
}



pub struct Group {
    pub items: Vec<Box<dyn SvgPositioned>>,
    transform: Option<String>,
}

impl Group {
    pub fn new() -> Self {
        Group{items: Vec::new(), transform: None}
    }

    pub fn item_transformed(item: Box<dyn SvgPositioned>, transform: &str) -> Self {
        Group{items: vec![item], transform: Some(transform.to_string())}
    }

    pub fn add(&mut self, item: Box<dyn SvgPositioned>) {
        self.items.push(item);
    }

    #[allow(dead_code)] // FIXME: It isn't used yet.
    pub fn set_transform(&mut self, transform: Option<String>) {
        self.transform = transform;
    }
}

impl Renderable for Group {
    fn render(&self, tag_writer: &mut TagWriter, context: &mut Context) -> Result<(), TagWriterError> {
        let attributes = match &self.transform {
            None => Attributes::new(),
            Some(transform) => Attributes::from([("transform", transform)]),
        };
        tag_writer.begin_tag("g", attributes)?;
        for item in self.items.iter() {
            item.render(tag_writer, context)?;
        }
        tag_writer.end_tag("g")?;
        Ok(())
    }
}


impl SvgPositioned for Group {
    fn get_bbox(&self, context: &mut Context) -> Rect {
        self.items.iter()
            .map(|item| item.get_bbox(context))
            .reduce(|accum, rect| accum.cover(&rect))
            .unwrap_or(Rect::new_cwh((0.0, 0.0), 0.0, 0.0))
    }
}


impl<const N: usize> From<[Box<dyn SvgPositioned>; N]> for Group {
    fn from(arr: [Box<dyn SvgPositioned>; N]) -> Self {
        let mut items: Vec<Box<dyn SvgPositioned>> = Vec::with_capacity(N);
        for item in arr {
            items.push(item);
        }
        Group{items, transform: None}
    }
}



pub struct Svg<T: Renderable> {
    content: T,
}

impl<T: Renderable> Svg<T> {
    pub fn new(content: T) -> Self {
        Svg{content}
    }
}

impl<T: SvgPositioned> Svg<T> {
    pub fn render(&self, tag_writer: &mut TagWriter, context: &mut Context) -> Result<(), TagWriterError> {
        let bbox = self.content.get_bbox(context);
        let viewbox: String = format_args!("{} {} {} {}", bbox.left(), bbox.top(), bbox.width(), bbox.height()).to_string();
        tag_writer.begin_tag("svg", Attributes::from([
            ("viewBox", &*viewbox),
            ("xmlns", "http://www.w3.org/2000/svg"),
        ]))?;
        self.content.render(tag_writer, context)?;
        tag_writer.end_tag("svg")?;
        Ok(())
    }
}




// Demonstrate how to build a diagram.
fn run() -> Result<(),TagWriterError> {

    let box_1 = BasicBox{x:  5.0, y: 3.0, height: 40.0, width: 50.0};
    let box_2 = BasicBox{x: 12.0, y: 6.0, height: 40.0, width: 50.0};
    let mut group = Group::new();
    group.add(Box::new(box_1));
    group.add(Box::new(box_2));
    let svg = Svg{content: group};

    let output2: File = File::create("output/test.svg")?;
    let mut tag_writer = TagWriter::new(output2);
    svg.render(&mut tag_writer, &mut Context::default())?;
    tag_writer.close()?;

    Ok(())
}



#[allow(dead_code)]
pub fn main() {
    match run() {
        Ok(()) => {
            println!("Begin!");
            match run() {
                Ok(()) => {
                    println!("Done!");
                }
                Err(err) => {
                    println!("Error: {}", err);
                }
            }
        },
        Err(err) => {
            println!("Error: {}", err);
        }
    }

}
