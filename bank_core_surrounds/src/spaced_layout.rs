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
fn find_pair_to_merge<T:Spaceable>(spans: &Vec<CompactSpan<T>>) -> Option<(usize, usize)> {
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
    item_vec: &'a Vec<&'a mut T>,
    item_idxs: Vec<usize>,
    desired_min: Coord,
    desired_max: Coord,
    min_extent: Coord,
}


impl<'a,T: Spaceable> CompactSpan<'a,T> {
    /// Create a new span. Must be from a single item that has a desired position
    fn new(item_vec: &'a Vec<&'a mut T>, idx: usize) -> Self {
        let item_idxs = vec![idx];
        let item = item_vec.get(idx).unwrap();
        let desired_min = item.get_desired_loc();
        let desired_max = item.get_desired_loc();
        let min_extent = item_idxs.iter()
            .map(|x| item_vec.get(*x).unwrap().get_extent())
            .sum::<Coord>();
        CompactSpan{item_vec, item_idxs, desired_min, desired_max, min_extent}
    }

    /// gets the item for a given index
    fn get(&self, idx: &usize) -> &T {
        self.item_vec.get(*idx).unwrap()
    }

    fn desired_center(&self) -> Coord {
        (self.desired_min + self.desired_max) / 2.0
    }

    fn required_min(&self) -> Coord {
        self.desired_center() - self.min_extent / 2.0
    }

    fn required_max(&self) -> Coord {
        self.desired_center() + self.min_extent / 2.0
    }

    fn overlaps(&self, other: &CompactSpan<'a,T>) -> bool {
        let answer = !(
            self.required_min() > other.required_max() ||
            self.required_max() < other.required_min()
        );
        answer
    }

    /// Modifies self to be the merge of other and self
    fn merge(&mut self, other: CompactSpan<'a,T>) {
        self.item_idxs.extend(other.item_idxs);
        self.desired_min = self.desired_min.min(other.desired_min);
        self.desired_max = self.desired_max.max(other.desired_max);
        self.min_extent = self.min_extent + other.min_extent;
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



#[cfg(test)]
mod tests {
    use super::*;


    struct Dummy {
        desired: Coord,
        actual: Coord,
    }

    impl Dummy {
        fn new(desired: &Coord) -> Self { Dummy{desired: *desired, actual: Coord::NAN} }
    }

    impl Spaceable for Dummy {
        fn get_desired_loc(&self) -> Coord { self.desired }
        fn get_extent(&self) -> Coord { 10.0 }
        fn set_position(&mut self, pos: Coord) { self.actual = pos }
    }

    fn run_test(input: Vec<Coord>, expect: Vec<Coord>) {
        let mut orig_items: Vec<Dummy> = input.iter().map(|x| Dummy::new(x)).collect();
        let mut items: Vec<&mut Dummy> = orig_items.iter_mut().map(|x| x).collect();
        layout(&mut items);
        let output = orig_items.iter().map(|x| x.actual).collect_vec();
        let expected = expect.iter().map(|x| *x).collect_vec();
        assert_eq!(output, expected);
    }


    #[test]
    fn test_two_overlapping() {
        run_test(
            vec![4.0, -4.0],
            vec![5.0, -5.0]
        );
    }
}
