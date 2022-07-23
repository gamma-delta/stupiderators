/// An iterator that may or may not have any values inside.
///
/// I need this for some things
pub enum OptionalIterator<I> {
    None,
    Some(I),
}

impl<I> Iterator for OptionalIterator<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            OptionalIterator::None => None,
            OptionalIterator::Some(iter) => iter.next(),
        }
    }
}

impl<I> ExactSizeIterator for OptionalIterator<I>
where
    I: ExactSizeIterator,
{
    fn len(&self) -> usize {
        match self {
            OptionalIterator::None => 0,
            OptionalIterator::Some(iter) => iter.len(),
        }
    }
}
