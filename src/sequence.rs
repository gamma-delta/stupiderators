/// A counter between different values that stays on each for a given amount of time.
pub struct SequencedCounter<T> {
    remainder: Vec<(T, usize)>,
    count_left: usize,
}

impl<T> SequencedCounter<T> {
    /// The iterator will count over each element and stay there the given number of times.
    pub fn new(counts: Vec<(T, usize)>) -> Self {
        let mut out = Vec::with_capacity(counts.len());
        let mut count_left = 0;
        for (it, count) in counts {
            if count != 0 {
                count_left += count;
                out.push((it, count));
            }
        }
        out.reverse();
        Self {
            remainder: out,
            count_left,
        }
    }
}

impl<T> Iterator for SequencedCounter<T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count_left == 0 {
            return None;
        }

        let (last, count) = self.remainder.last_mut()?;
        let out = last.clone();
        *count -= 1;

        if *count == 0 {
            self.remainder.pop();
        }

        self.count_left -= 1;
        Some(out)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count_left, Some(self.count_left))
    }
}

impl<T> ExactSizeIterator for SequencedCounter<T> where T: Clone {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke() {
        let sq: String =
            SequencedCounter::new(vec![('a', 7), ('c', 6), ('a', 0), ('b', 4)]).collect();
        assert_eq!(&sq, "aaaaaaaccccccbbbb");
    }

    #[test]
    fn exact_size() {
        let mut sq =
            SequencedCounter::new(vec![('a', 150), ('b', 23), ('c', 69), ('d', 0), ('e', 12)]);
        assert_eq!(sq.len(), 150 + 23 + 69 + 0 + 12);
        for _ in 0..12 {
            sq.next().unwrap();
        }
        assert_eq!(sq.len(), 150 + 23 + 69 + 0 + 12 - 12);
        for _ in 0..98 {
            sq.next().unwrap();
        }
        assert_eq!(sq.len(), 150 + 23 + 69 + 0 + 12 - 12 - 98);
        let collected: String = sq.collect();
        assert_eq!(collected.len(), 150 + 23 + 69 + 0 + 12 - 12 - 98);
    }
}
