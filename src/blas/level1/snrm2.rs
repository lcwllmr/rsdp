pub fn snrm2(x: &[f32]) -> f32 {
    let mut sum = 0.0f32;
    for i in 0..x.len() {
        sum += x[i] * x[i];
    }
    f32::sqrt(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snrm2() {
        let v = [3.0f32, 4.0];
        let result = snrm2(&v);
        assert_eq!(result, 5.0);
    }
}
