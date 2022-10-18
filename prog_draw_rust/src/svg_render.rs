use crate::svg_writer::{Renderable, TagWriter, TagWriterError, Attributes};
use crate::geometry::{Coord, Point, Rect};
use crate::text_size::get_system_text_sizer;


/// A trait for anything whose SVG dimensions can be measured and used to lay it
/// out.
pub trait SvgPositioned: Renderable {
    /// Returns a bounding box for this item. The bounding box is relative to the local
    /// coordinate system.
    /// FIXME: I keep encountering things that sometimes take up NO space. Maybe
    ///   I need to make this return Option<Rect> to handle that case. If so, then
    ///   I also need to make reduce() with Rect::cover() continue to work somehow.
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


pub struct Text {
    text: String,
    position: Point, // The center of the text
    text_size_cached: Point,
    font_family: Option<String>,
    font_size: Option<String>,
}

const DEFAULT_FONT: &str = "Arial";
const DEFAULT_SIZE: f32 = 12.0;
const DEFAULT_COLOR: &str = "#000000";

impl Text {
    /// Construct a new Text, providing the text and the position.
    pub fn new(text: &str, position: Point) -> Self {
        let mut answer = Text{text: text.to_string(), position, text_size_cached: (0.0, 0.0), font_family: None, font_size: None};
        answer.cache_text_size();
        answer
    }

    /// Construct a new Text, providing the text, position, and styling. font_family and font_size are
    /// css strings for their corresponding CSS fields.
    pub fn new_styled(text: &str, position: Point, font_family: Option<String>, font_size: Option<String>) -> Self {
        let mut answer = Text{text: text.to_string(), position, text_size_cached: (0.0, 0.0), font_family, font_size};
        answer.cache_text_size();
        answer
    }

    /// Internal function to find the value we will store in text_size_cached.
    fn cache_text_size(&mut self) {
        let font = match &self.font_family {
            None => DEFAULT_FONT,
            Some(s) => s.as_str(),
        };
        let size: f32 = match &self.font_size {
            None => DEFAULT_SIZE,
            Some(s) if s.ends_with("px") => {
                let num_part = &s[..s.len() - 2];
                match num_part.parse::<f32>() {
                    Err(_) => DEFAULT_SIZE,
                    Ok(size) => size,
                }
            },
            Some(_) => DEFAULT_SIZE,
        };
        self.text_size_cached = match get_system_text_sizer().text_size(&self.text, font, size) {
            Err(_) => panic!("Sizing isn't working."),
            Ok((width,height)) => (width as Coord, height as Coord)
        };
    }
}


impl SvgPositioned for Text {
    fn get_bbox(&self) -> Rect {
        Rect::new_cwh(self.position, self.text_size_cached.0, self.text_size_cached.1)
    }
}

impl Renderable for Text {
    fn render(&self, tag_writer: &mut dyn TagWriter) -> Result<(), TagWriterError> {
        let mut attributes = Attributes::from([
            ("x", self.position.0.to_string().as_str()),
            ("y", self.position.1.to_string().as_str()),
            ("fill", DEFAULT_COLOR),
            ("text-anchor", "middle"),
            ("dominant-baseline", "central"),
            // FIXME: Now that I've figured out how to align text properly, maybe I should use that in OTHER places.
        ]);
        if self.font_family.is_some() {
            attributes = attributes.with_field("font-family", self.font_family.as_ref().unwrap());
        }
        if self.font_size.is_some() {
            attributes = attributes.with_field("style", format!("font-size: {}", self.font_size.as_ref().unwrap()));
        }
        tag_writer.tag_with_text("text", attributes, &self.text)
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

