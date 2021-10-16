pub trait Trees {
    type Item;
    fn insert(&mut self, item: Self::Item) -> Result<(), Self::Item>;
    fn len(&self) -> usize;

    fn find_by_value(&self, value: Self::Item) -> Option<&Self::Item>;

    fn find_by_predicate<P>(&self, predicate: P) -> Option<&Self::Item>
    where
        P: FnMut(&Self::Item) -> bool;

    fn delete_by_value(&mut self, value: Self::Item) -> Option<Self::Item>;
    fn delete_by_predicate<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool;
}
