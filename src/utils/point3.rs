use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}
impl<T> Point3<T>
where
    T: Copy + Sub<Output = T> + Mul<Output = T> + Add<Output = T>,
{
    pub fn dist2(&self, p: &Self) -> T {
        let x = p.x - self.x;
        let y = p.y - self.y;
        let z = p.z - self.z;
        (x * x) + (y * y) + (z * z)
    }
}
impl<T: Display> Display for Point3<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}
