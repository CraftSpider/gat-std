use super::Iterator;

/// See [`Iterator::map`]
pub struct Map<I, F> {
    iter: I,
    func: F,
}

impl<I, F> Map<I, F> {
    pub(crate) fn new(iter: I, func: F) -> Map<I, F> {
        Map { iter, func }
    }
}

impl<I, F, O> core::iter::Iterator for Map<I, F>
where
    I: Iterator,
    F: FnMut(I::Item<'_>) -> O,
{
    type Item = O;

    fn next(&mut self) -> Option<Self::Item> {
        Some((self.func)(self.iter.next()?))
    }
}

/// See [`Iterator::touch`]
pub struct Touch<I, F> {
    iter: I,
    func: F,
}

impl<I, F> Touch<I, F> {
    pub(crate) fn new(iter: I, func: F) -> Touch<I, F> {
        Touch { iter, func }
    }
}

impl<I, F> Iterator for Touch<I, F>
where
    I: Iterator,
    F: FnMut(&mut I::Item<'_>),
{
    type Item<'a> = I::Item<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        let mut out = self.iter.next()?;
        (self.func)(&mut out);
        Some(out)
    }
}

/// See [`Iterator::scan`]
pub struct Scan<I, T, F> {
    iter: I,
    state: T,
    func: F,
}

impl<I, T, F> Scan<I, T, F> {
    pub(crate) fn new(iter: I, state: T, func: F) -> Scan<I, T, F> {
        Scan { iter, state, func }
    }
}

impl<I, T, F, O> core::iter::Iterator for Scan<I, T, F>
where
    I: Iterator,
    F: FnMut(&mut T, I::Item<'_>) -> Option<O>,
{
    type Item = O;

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.iter.next()?;
        (self.func)(&mut self.state, a)
    }
}
