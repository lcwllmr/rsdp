use crate::blas::traits::{Abs, Sqrt};

pub fn nrm2<T: Abs>(x: &[T]) -> T::Output
where
    T::Output: std::ops::Add<Output = T::Output>
        + std::ops::Mul<Output = T::Output>
        + Default
        + Copy
        + Sqrt,
{
    let mut sum = T::Output::default();
    for xi in x {
        let abs = xi.abs();
        sum = sum + abs * abs;
    }
    sum.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nrm2_f32() {
        let v = [3.0f32, 4.0];
        let result = nrm2(&v);
        assert_eq!(result, 5.0);
    }
}
