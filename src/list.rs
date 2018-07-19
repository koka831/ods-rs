

trait List<T> {
    // return size of list
    fn size(&self) -> usize;

    // return list[i]
    fn get(&mut self, i: usize) -> T;

    // overwrite list[i] with x
    fn set(&mut self, i: usize, x: T);

    // push x in list[i]
    // == add_first(0, x)
    fn add(&mut self, i: usize, x: T);

    // remove x_i, x_(i+1) -> x_i, x_(i+2) -> x_(i+1)
    fn remove(&mut self, i: usize);
}

// unordered set
trait USet<T> {
    fn size(&self) -> usize;

    // append x if x is not in list
    // return:
    //  true: when x appended
    //  false: otherwise
    fn add(&mut self, x: T) -> bool;

    fn remove(&mut self, x: T) -> Option<T>;

    fn find(&self, x: T) -> Option<T>;
}

// Sorted Set
trait SSet<T: Eq + PartialEq + USet<T>> {
    fn compare(&self, other: T) -> ::std::cmp::Ordering;
}
