/// An iterator that may or may not have any values inside.
///
/// I need this for some things
pub enum OptionalIterator<I> {
    None,
    Some(I),
}

impl<I> OptionalIterator<I> {
    /// Map an `Option` to an `OptionalIterator` by passing through `None` and applying the function if `Some`.
    pub fn from_mapped<F, J>(inner: Option<J>, f: F) -> Self
    where
        F: FnOnce(J) -> I,
    {
        match inner {
            Some(it) => OptionalIterator::Some(f(it)),
            None => OptionalIterator::None,
        }
    }
}

impl<I> From<Option<I>> for OptionalIterator<I> {
    fn from(it: Option<I>) -> Self {
        match it {
            Some(it) => OptionalIterator::Some(it),
            None => OptionalIterator::None,
        }
    }
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
