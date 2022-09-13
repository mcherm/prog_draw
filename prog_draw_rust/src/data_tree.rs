//
// Stealing the design from what tidy_tree uses, this is my own implemention of a tree
// data structure.
//


use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::cell::Cell;
use crate::svg_writer::{Renderable, TagWriter, Attributes, TagWriterError};
use crate::svg_render::SvgPositioned;
use crate::geometry::{Coord, Rect};


static LINE_CTRL_OFFSET: Coord = 10.0;


pub struct DTNode<T> {
    pub data: T,
    pub collapsed: bool,
    pub children: Vec<Box<DTNode<T>>>,
}

/// This enum type is used along with DTNode::grow_tree() to provide a way for callers
/// to easily enter tree data. It's by no means the only good way to achieve this, but
/// it is ONE possible way to put tree-creation right into rust code.
///
/// The AddData() item is used to add a data item as a child of the current node. StartChildren
/// will make the most recently added item the new "current node". EndChildren will make the
/// "current node" be the parent of the current "current node".
#[derive(Debug)]
pub enum DTNodeBuild<T> {
    AddData(T),
    StartChildren(bool), // the bool indicates whether the children are collapsed
    EndChildren,
}

#[derive(Debug)]
pub struct InvalidGrowth;
impl Error for InvalidGrowth {}
impl Display for InvalidGrowth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Growth")
    }
}


/// Trees can be laid out two ways: to the left or to the right.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TreeLayoutDirection {
    Right,
    Left
}

thread_local!{
    /// This threadlocal variable is set before rendering to say which way the tree
    /// should be laid out. If NOT set, it defaults to laying out to the right.
    pub static LAYOUT_DIRECTION: Cell<Option<TreeLayoutDirection>> = Cell::new(None);
}


impl<T: Debug> Debug for DTNode<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node{{data={:?}, children=[", self.data)?;
        let mut first = true;
        for child in self.children.iter() {
            if first {
                first = false;
            } else {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", child)?;
        }
        write!(f, "]}}")
    }
}

impl<T: Display> Display for DTNode<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node{{data={}, children=[", self.data)?;
        let mut first = true;
        for child in self.children.iter() {
            if first {
                first = false;
            } else {
                write!(f, ", ")?;
            }
            write!(f, "{}", child)?;
        }
        write!(f, "]}}")
    }
}

impl<T> DTNode<T> {

    /// Returns a new top-level node with this data.
    pub fn new(data: T) -> Self {
        let collapse = false;
        let children = vec![];
        DTNode{data, collapsed: collapse, children}
    }

    #[allow(dead_code)]
    /// Returns a count of the number of nodes including this and all its descendants.
    pub fn len(&self) -> usize {
        1 + self.children.iter().map(|x| x.len()).sum::<usize>()
    }

    /// Add a new child to this node which has the specified data.
    pub fn add_child_data(&mut self, data: T) {
        let child = DTNode::new(data);
        self.children.push(Box::new(child));
    }


    /// Initialize a tree with a syntax that allows us to pass an iterator.
    ///
    /// Example:
    /// ```
    /// let mut root = DTNode::new(MyNode::new("ROOT", &mut id_source));
    /// root.grow_tree([
    ///     AddData(MyNode::new(core_0, &mut id_source)),
    ///     StartChildren,
    ///     AddData(MyNode::new(core_0_0, &mut id_source)),
    ///     StartChildren,
    ///     AddData(MyNode::new(core_0_0_0, &mut id_source)),
    ///     AddData(MyNode::new(core_0_0_1, &mut id_source)),
    ///     AddData(MyNode::new(core_0_0_2, &mut id_source)),
    ///     EndChildren,
    ///     AddData(MyNode::new(core_0_1, &mut id_source)),
    /// ]);
    /// ```
    pub fn grow_tree(&mut self, items: impl IntoIterator<Item=DTNodeBuild<T>>) -> Result<(),InvalidGrowth> {
        let mut stack: Vec<usize> = vec![];
        let mut current = &mut *self;

        for action in items {
            match action {
                DTNodeBuild::AddData(data) => {
                    current.add_child_data(data);
                },
                DTNodeBuild::StartChildren(collapsed) => {
                    stack.push(current.children.len() - 1); // FIXME: watch for underflow
                    current = current.children.last_mut().unwrap().as_mut();
                    current.collapsed = collapsed
                }
                DTNodeBuild::EndChildren => {
                    stack.pop();
                    let mut n = &mut *self;
                    for idx in stack.iter() {
                        n = n.children[*idx].as_mut();
                    }
                    current = n;
                }
            }
        }
        Ok(())
    }


    #[allow(dead_code)] // FIXME: Remove this if it remains unused after a long time
    /// Add a node (which may have its own children) as a child of this node.
    pub fn add_child_node(&mut self, child: DTNode<T>) {
        self.children.push(Box::new(child));
    }

}

impl<T: SvgPositioned> Renderable for DTNode<T> {
    fn render(&self, tag_writer: &mut dyn TagWriter) -> Result<(), TagWriterError> {
        if !self.collapsed {
            // --- Use context to decide whether to draw to the right or the left ---
            let leftward: bool = match LAYOUT_DIRECTION.with(|it| it.get()) {
                Some(TreeLayoutDirection::Left) => true,
                _ => false,
            };

            // --- Draw lines to child nodes ---
            let parent_bbox = self.data.get_bbox();
            let parent_line_end_x = if leftward {parent_bbox.left()} else {parent_bbox.right()};
            let parent_line_end_y = parent_bbox.top() + parent_bbox.height() / 2.0;
            let parent_line_ctrl_x = parent_line_end_x + LINE_CTRL_OFFSET * if leftward {-1.0} else {1.0};
            let parent_line_ctrl_y = parent_line_end_y;
            for child in self.children.iter() {
                let child_bbox = child.data.get_bbox();
                let child_line_end_x = if leftward {child_bbox.right()} else {child_bbox.left()};
                let child_line_end_y = child_bbox.top() + child_bbox.height() / 2.0;
                let child_line_ctrl_x = child_line_end_x + LINE_CTRL_OFFSET * if leftward {1.0} else {-1.0};
                let child_line_ctrl_y = child_line_end_y;
                let path_code: String = format_args!(
                    "M {} {} C {} {}, {} {}, {} {}",
                    parent_line_end_x, parent_line_end_y,
                    parent_line_ctrl_x, parent_line_ctrl_y,
                    child_line_ctrl_x, child_line_ctrl_y,
                    child_line_end_x, child_line_end_y
                ).to_string();
                tag_writer.single_tag("path", Attributes::from([
                    ("d", &*path_code),
                    ("fill", "none"),
                    ("stroke", "black"),
                ]))?;
            }

            // --- Draw child nodes ---
            for child in self.children.iter() {
                child.render(tag_writer)?;
            }
        }

        // --- Draw this node ---
        self.data.render(tag_writer)?;
        Ok(())
    }
}

impl<T: SvgPositioned> SvgPositioned for DTNode<T> {
    // Returns the bbox that covers the root node AND all non-collapsed descendant nodes.
    fn get_bbox(&self) -> Rect {
        let root_rect: Rect = self.data.get_bbox();
        if self.collapsed {
            root_rect
        } else {
            self.children.iter()
                .map(|child| child.get_bbox())
                .fold(root_rect, |r1, r2| r1.cover(&r2))
        }
    }
}
