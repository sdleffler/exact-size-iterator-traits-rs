pub trait FromExactSizeIterator<A> {
    fn from_exact_size_iter<T: IntoIterator<Item = A>>(T) -> Self
        where <T as IntoIterator>::IntoIter: ExactSizeIterator;
}


pub trait IntoExactSizeIterator: IntoIterator
    where Self::IntoIter: ExactSizeIterator
{
}


impl<T: IntoIterator> IntoExactSizeIterator for T where T::IntoIter: ExactSizeIterator {}


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