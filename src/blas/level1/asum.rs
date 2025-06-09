use crate::blas::traits::Abs;

pub fn asum<T: Abs>(x: &[T]) -> T::Output
where
    T::Output: std::ops::Add<Output = T::Output> + Default + Copy,
{
    let mut sum = T::Output::default();
    for xi in x {
        sum = sum + xi.abs();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asum_f32() {
        let v = [1.0f32, -2.0, 3.0];
        let result = asum(&v);
        assert_eq!(result, 6.0);
    }
}
