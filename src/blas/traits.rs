pub trait Abs {
    type Output;
    fn abs(&self) -> Self::Output;
}

impl Abs for f32 {
    type Output = f32;
    fn abs(&self) -> f32 {
        f32::abs(*self)
    }
}

impl Abs for f64 {
    type Output = f64;
    fn abs(&self) -> f64 {
        f64::abs(*self)
    }
}

pub trait Sqrt {
    fn sqrt(self) -> Self;
}

impl Sqrt for f32 {
    fn sqrt(self) -> Self {
        f32::sqrt(self)
    }
}

impl Sqrt for f64 {
    fn sqrt(self) -> Self {
        f64::sqrt(self)
    }
}
