use crate::blas::traits::Abs;

pub fn i_amax<T: Abs>(x: &[T]) -> usize
where
    T::Output: PartialOrd + Copy,
{
    let mut max_idx = 0;
    let mut max_val = x[0].abs();
    for (i, xi) in x.iter().enumerate().skip(1) {
        let abs = xi.abs();
        if abs > max_val {
            max_val = abs;
            max_idx = i;
        }
    }
    max_idx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i_amax_f32() {
        let v = [1.0f32, -5.0, 3.0, 5.0];
        let idx = i_amax(&v);
        assert_eq!(idx, 1);
    }
}
