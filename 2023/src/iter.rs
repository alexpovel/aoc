use std::iter::Peekable;

pub struct CenteredWindow<I>
where
    I: Iterator,
{
    previous: Option<I::Item>,
    current: Option<I::Item>,
    underlying: Peekable<I>,
    next: Option<I::Item>,
}

impl<I> Iterator for CenteredWindow<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = (Option<I::Item>, I::Item, Option<I::Item>);

    fn next(&mut self) -> Option<Self::Item> {
        match self.underlying.next() {
            Some(item) => {
                self.previous = self.current.clone();
                self.current = Some(item.clone());
                self.next = self.underlying.peek().cloned();
                Some((self.previous.clone(), item, self.next.clone()))
            }
            None => None,
        }
    }
}

pub trait CenteredWindowExt: Iterator {
    fn centered_window(self) -> CenteredWindow<Self>
    where
        Self: Sized,
    {
        CenteredWindow {
            previous: None,
            current: None,
            underlying: self.peekable(),
            next: None,
        }
    }
}

impl<I: Iterator> CenteredWindowExt for I {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_centered_window() {
        let mut iter = (0..5).centered_window();
        assert_eq!(iter.next(), Some((None, 0, Some(1))));
        assert_eq!(iter.next(), Some((Some(0), 1, Some(2))));
        assert_eq!(iter.next(), Some((Some(1), 2, Some(3))));
        assert_eq!(iter.next(), Some((Some(2), 3, Some(4))));
        assert_eq!(iter.next(), Some((Some(3), 4, None)));
        assert_eq!(iter.next(), None);
    }
}
