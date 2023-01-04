use super::{change_lifetime, Iterator};

/// See [`IntoLending::into_lending`]
pub struct FromCore<I>(pub(crate) I);

impl<I> Iterator for FromCore<I>
where
    I: core::iter::Iterator,
{
    type Item<'a> = I::Item
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.0.next()
    }
}

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

/// See [`Iterator::filter`]
pub struct Filter<I, F> {
    iter: I,
    func: F,
}

impl<I, F> Filter<I, F> {
    pub(crate) fn new(iter: I, func: F) -> Filter<I, F> {
        Filter { iter, func }
    }
}

impl<I, F> Iterator for Filter<I, F>
where
    I: Iterator,
    F: FnMut(&I::Item<'_>) -> bool,
{
    type Item<'a> = I::Item<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        while let Some(val) = self.iter.next() {
            if (self.func)(&val) {
                // SAFETY: This is the polonius case
                return Some(unsafe { change_lifetime::<Self>(val) });
            }
        }
        None
    }
}

/// See [`Iterator::step_by`]
pub struct StepBy<I> {
    iter: I,
    step: usize,
    first: bool,
}

impl<I> StepBy<I> {
    pub(crate) fn new(iter: I, step: usize) -> StepBy<I> {
        StepBy {
            iter,
            step,
            first: true,
        }
    }
}

impl<I> Iterator for StepBy<I>
where
    I: Iterator,
{
    type Item<'a> = I::Item<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        if self.first {
            self.first = false;
            self.iter.next()
        } else {
            self.nth(self.step - 1)
        }
    }
}

/// See [`Iterator::chain`]
pub struct Chain<I1, I2> {
    first: Option<I1>,
    second: Option<I2>,
}

impl<I1, I2> Chain<I1, I2> {
    pub(crate) fn new(first: I1, second: I2) -> Chain<I1, I2> {
        Chain {
            first: Some(first),
            second: Some(second),
        }
    }
}

impl<'b, I1, I2> Iterator for Chain<I1, I2>
where
    I1: Iterator + 'b,
    I2: Iterator<Item<'b> = I1::Item<'b>> + 'b,
{
    type Item<'a> = I1::Item<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        if let Some(iter) = &mut self.first {
            if let Some(val) = iter.next() {
                // SAFETY: This is the polonius case
                return Some(unsafe { change_lifetime::<Self>(val) });
            }
            self.first = None;
        }

        if let Some(iter) = &mut self.second {
            // SAFETY: Due to our lie in the where bounds, we need to convince Rust
            //         that we actually borrow iter for a different length of time than it
            //         thinks
            let iter = unsafe { core::mem::transmute::<&mut I2, &mut I2>(iter) };
            if let Some(val) = iter.next() {
                // SAFETY: This is the polonius case
                return Some(unsafe { change_lifetime::<Self>(val) });
            }
            self.second = None;
        }
        None
    }
}

/// See [`Iterator::zip`]
pub struct Zip<I1, I2> {
    left: I1,
    right: I2,
}

impl<I1, I2> Zip<I1, I2> {
    pub(crate) fn new(left: I1, right: I2) -> Zip<I1, I2> {
        Zip { left, right }
    }
}

impl<I1, I2> Iterator for Zip<I1, I2>
where
    I1: Iterator,
    I2: Iterator,
{
    type Item<'a> = (I1::Item<'a>, I2::Item<'a>)
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        let left = self.left.next()?;
        let right = self.right.next()?;
        Some((left, right))
    }
}

/// See [`Iterator::enumerate`]
pub struct Enumerate<I> {
    iter: I,
    pos: usize,
}

impl<I> Enumerate<I> {
    pub(crate) fn new(iter: I) -> Enumerate<I> {
        Enumerate { iter, pos: 0 }
    }
}

impl<I> Iterator for Enumerate<I>
where
    I: Iterator,
{
    type Item<'a> = (usize, I::Item<'a>)
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        let out = (self.pos, self.iter.next()?);
        self.pos += 1;
        Some(out)
    }
}

/// See [`Iterator::skip_while`]
pub struct SkipWhile<I, F> {
    iter: I,
    func: Option<F>,
}

impl<I, F> SkipWhile<I, F> {
    pub(crate) fn new(iter: I, func: F) -> SkipWhile<I, F> {
        SkipWhile {
            iter,
            func: Some(func),
        }
    }
}

impl<I, F> Iterator for SkipWhile<I, F>
where
    I: Iterator,
    F: FnMut(&I::Item<'_>) -> bool,
{
    type Item<'a> = I::Item<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        match self.func.take() {
            Some(mut f) => {
                while let Some(val) = self.iter.next() {
                    if !f(&val) {
                        // SAFETY: This is the polonius case
                        return Some(unsafe { change_lifetime::<Self>(val) });
                    }
                }
                None
            }
            None => self.iter.next(),
        }
    }
}

/// See [`Iterator::take_while`]
pub struct TakeWhile<I, F> {
    iter: I,
    func: Option<F>,
}

impl<I, F> TakeWhile<I, F> {
    pub(crate) fn new(iter: I, func: F) -> TakeWhile<I, F> {
        TakeWhile {
            iter,
            func: Some(func),
        }
    }
}

impl<I, F> Iterator for TakeWhile<I, F>
where
    I: Iterator,
    F: FnMut(&I::Item<'_>) -> bool,
{
    type Item<'a> = I::Item<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        match &mut self.func {
            Some(f) => {
                let next = self.iter.next()?;
                if !f(&next) {
                    self.func = None;
                    None
                } else {
                    Some(next)
                }
            }
            None => None,
        }
    }
}

/// See [`Iterator::skip`]
pub struct Skip<I> {
    iter: I,
    skip: usize,
}

impl<I> Skip<I> {
    pub(crate) fn new(iter: I, skip: usize) -> Skip<I> {
        Skip { iter, skip }
    }
}

impl<I> Iterator for Skip<I>
where
    I: Iterator,
{
    type Item<'a> = I::Item<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        while self.skip > 0 {
            self.skip -= 1;
            match self.iter.next() {
                Some(_) => self.skip -= 1,
                None => {
                    self.skip = 0;
                    return None;
                }
            }
        }
        self.iter.next()
    }
}

/// See [`Iterator::take`]
pub struct Take<I> {
    iter: I,
    take: usize,
}

impl<I> Take<I> {
    pub(crate) fn new(iter: I, take: usize) -> Take<I> {
        Take { iter, take }
    }
}

impl<I> Iterator for Take<I>
where
    I: Iterator,
{
    type Item<'a> = I::Item<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        if self.take > 0 {
            self.take -= 1;
            self.iter.next()
        } else {
            None
        }
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
