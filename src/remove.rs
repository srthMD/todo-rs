// https://stackoverflow.com/questions/57947441/remove-a-sequence-of-values-from-a-vec-in-rust
// i was too tired to figure this out by myself

pub trait RemoveMultiple<T> {
    /// Remove multiple indices
    fn remove_multiple(&mut self, to_remove: Vec<usize>);
}

impl<T> RemoveMultiple<T> for Vec<T> {
    fn remove_multiple(&mut self, mut to_remove: Vec<usize>) {
        to_remove.sort();
        to_remove.reverse();
        for r in to_remove {
            self.remove(r);
        }
    }
}