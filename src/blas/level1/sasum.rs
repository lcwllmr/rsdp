pub fn sasum(x: &[f32]) -> f32 {
    let mut sum = 0.0f32;
    for i in 0..x.len() {
        sum += x[i].abs();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sasum() {
        let v = [1.0f32, -2.0, 3.0];
        let result = sasum(&v);
        assert_eq!(result, 6.0);
    }
}
