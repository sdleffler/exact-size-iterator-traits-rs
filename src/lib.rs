use std::iter::Chain;


pub trait FromExactSizeIterator<A> {
    fn from_exact_size_iter<T: IntoIterator<Item = A>>(T) -> Self
        where T::IntoIter: ExactSizeIterator;
}


pub trait CollectExactExt: ExactSizeIterator {
    fn collect_exact<B: FromExactSizeIterator<Self::Item>>(self) -> B where Self: Sized;
}


impl<I: ExactSizeIterator> CollectExactExt for I {
    fn collect_exact<B: FromExactSizeIterator<Self::Item>>(self) -> B
        where Self: Sized
    {
        B::from_exact_size_iter(self)
    }
}


pub struct ChainExact<A, B>(Chain<A, B>, usize)
    where A: Iterator,
          B: Iterator<Item = A::Item>;


pub trait ChainExactExt: ExactSizeIterator {
    fn chain_exact<U>(self, other: U) -> ChainExact<Self, U::IntoIter>
        where Self: Sized,
              U: IntoIterator<Item = Self::Item>,
              U::IntoIter: ExactSizeIterator;
}


impl<I: ExactSizeIterator> ChainExactExt for I {
    fn chain_exact<U>(self, other: U) -> ChainExact<Self, U::IntoIter>
        where U: IntoIterator<Item = Self::Item>,
              U::IntoIter: ExactSizeIterator
    {
        let other = other.into_iter();
        let len = self.len() + other.len();
        ChainExact(self.chain(other), len)
    }
}


impl<A, B> Iterator for ChainExact<A, B>
    where A: Iterator,
          B: Iterator<Item = A::Item>
{
    type Item = A::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 > 0 {
            self.1 -= 1;
            self.0.next()
        } else {
            None
        }
    }


    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.1, Some(self.1))
    }
}


impl<A, B> ExactSizeIterator for ChainExact<A, B>
    where A: Iterator,
          B: Iterator<Item = A::Item>
{
}
