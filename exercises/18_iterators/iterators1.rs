// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterators() {
        let my_fav_fruits = ["banana", "custard apple", "avocado", "peach", "raspberry"];

        // TODO: Create an iterator over the array.
        let mut my_iterable_fav_fruits = my_fav_fruits.iter();   // TODO: Step 1

        assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
        assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));     // TODO: Step 2
        assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
        assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));     // TODO: Step 3
        assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
        assert_eq!(my_iterable_fav_fruits.next(), None);     // TODO: Step 4
    }
}
