//
// This is for laying out the surrounds. It could probably be generalized later, and if
// so it could be moved to the library section.
//
// Overall: I haven't attempted to be efficient in this. Maybe should consider doing so?
//

use itertools::Itertools;
use prog_draw::geometry::Coord;


/// A trait for items that can be laid out by this algorithm
pub trait Spaceable {
    /// Returns the position the item would like to be in
    fn get_desired_loc(&self) -> Coord;

    /// Returns the extent of the item
    fn get_extent(&self) -> Coord;

    /// Called at the end to set the position
    fn set_position(&mut self, pos: Coord);
}


/// Call this to lay out the items. It will call set_position() once on each item.
pub fn layout<'a,T: Spaceable>(items: &'a mut Vec<&'a mut T>) {
    // --- make a list of spans ---
    let mut spans = (0..items.len()).map(|x| CompactSpan::new(items, x)).collect();

    // --- merge spans that will overlap until none of the spans overlap ---
    loop {
        // in each pass through the loop we merge 2 spans
        match find_pair_to_merge(&spans) {
            None => break, // exit the loop
            Some((i,j)) => {
                let span_2 = spans.remove(j);
                spans[i].merge(span_2)
            }
        }
    }

    // --- allow each span to re-position the items in it ---
    let mut position_instructions: Vec<(usize,Coord)> = Vec::new();
    for span in spans.iter_mut() {
        position_instructions.extend( span.get_item_positions() );
    }

    // --- Now we can drop all the CompactSpans and update the positions ---
    for (idx, pos) in position_instructions.iter() {
        items.get_mut(*idx).unwrap().set_position(*pos);
    }
}


/// A helper inside layout().
fn find_pair_to_merge<'a,T:Spaceable>(spans: &Vec<CompactSpan<'a,T>>) -> Option<(usize, usize)> {
    for i in 0..spans.len() {
        let span_1 = &spans[i];
        for j in (i+1)..spans.len() {
            let span_2 = &spans[j];
            if span_1.overlaps(span_2) {
                return Some((i,j));
            }
        }
    }
    return None
}



/// Represents a group of Spaceable items that might overlap. Always has at least one
/// item in item_ids. Uses a vector of indexes (and a link to the list of mutable
/// vectors) because when it kept the mutable vectors itself I couldn't figure out
/// how to pass them over when two were merged.
struct CompactSpan<'a,T:Spaceable> {
    // FIXME: Remove next
    //items: Vec<&'a mut T>, // min length 1; all have desired_y; kept in order by position

    item_vec: &'a Vec<&'a mut T>,
    item_idxs: Vec<usize>,
}


impl<'a,T: Spaceable> CompactSpan<'a,T> {
    /// Create a new span. Must be from a single item that has a desired position
    // FIXME: Old version; remove
    // fn new(item: &'a mut T) -> Self {
    //     CompactSpan{items: vec![item]}
    // }
    fn new(item_vec: &'a Vec<&'a mut T>, idx: usize) -> Self {
        let item_idxs = vec![idx];
        CompactSpan{item_vec, item_idxs}
    }

    /// gets the item for a given index
    fn get(&self, idx: &usize) -> &T {
        self.item_vec.get(*idx).unwrap()
    }

    fn desired_min(&self) -> Coord {
        self.item_idxs.iter()
            .map(|x| self.get(x).get_desired_loc() + self.get(x).get_extent() / 2.0)
            .min_by(|x, y| Coord::total_cmp(x, y)).unwrap()
    }

    fn desired_max(&self) -> Coord {
        self.item_idxs.iter()
            .map(|x| self.get(x).get_desired_loc() + self.get(x).get_extent() / 2.0)
            .max_by(|x, y| Coord::total_cmp(x, y)).unwrap()
    }

    fn desired_center(&self) -> Coord {
        (self.desired_min() + self.desired_max()) / 2.0
    }

    fn min_extent(&self) -> Coord {
        self.item_idxs.iter()
            .map(|x| self.get(x).get_extent())
            .sum::<Coord>()
    }

    fn required_min(&self) -> Coord {
        self.desired_center() - self.min_extent() / 2.0
    }

    fn required_max(&self) -> Coord {
        self.desired_center() + self.min_extent() / 2.0
    }

    fn overlaps(&self, other: &CompactSpan<'a,T>) -> bool {
        self.required_min() > other.required_max() ||
            self.required_max() > other.required_min()
    }

    /// Modifies self to be the merge of other and self
    fn merge(&mut self, other: CompactSpan<'a,T>) {
        self.item_idxs.extend(other.item_idxs);
    }

    /// Returns a vector telling where to position the items. We can't actually move
    /// them until after all the CompactSpans have been dropped because the spans have
    /// (shared) references to the array. Returns pairs (idx, correct_location).
    fn get_item_positions(&mut self) -> Vec<(usize,Coord)> {
        // --- sort the items ---
        // make a vec of (desired_loc, idx) so we can sort them
        let mut sortable_idxs = self.item_idxs.iter()
            .map(|x| (self.get(x).get_desired_loc(), *x))
            .collect_vec();
        sortable_idxs.sort_by(|x,y| x.0.total_cmp(&y.0));

        // --- create the output vec ---
        let mut answer: Vec<(usize,Coord)> = Vec::new();

        // --- position them ---
        let mut pos = self.required_min();
        for (_, idx) in sortable_idxs.iter() {
            let item = self.get(idx);
            let item_extent = item.get_extent();
            let item_pos = pos + item_extent / 2.0;
            answer.push((*idx, item_pos));
            pos += item_extent;
        }

        // --- return ---
        answer
    }
}
