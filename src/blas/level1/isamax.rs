pub fn isamax(x: &[f32]) -> usize {
    let mut max_idx = 0;
    let mut max_val = f32::abs(x[0]);
    for i in 1..x.len() {
        let abs = f32::abs(x[i]);
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
    fn test_isamax() {
        let v = [1.0f32, -5.0, 3.0, 5.0];
        let idx = isamax(&v);
        assert_eq!(idx, 1);
    }
}
