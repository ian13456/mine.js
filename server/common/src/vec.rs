use num::{cast, Float, Num};

use std::ops::{Index, IndexMut};

#[derive(Debug, Eq, PartialEq, Clone, Default, Hash)]
pub struct Vec2<T>(pub T, pub T);

impl<T: Copy + 'static> Vec2<T> {
    pub fn from<U: cast::AsPrimitive<T>>(other: &Vec2<U>) -> Vec2<T> {
        Vec2(other.0.as_(), other.1.as_())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default, Hash)]
pub struct Vec3<T>(pub T, pub T, pub T);

impl<T: Copy + 'static> Vec3<T> {
    pub fn from<U: cast::AsPrimitive<T>>(other: &Vec3<U>) -> Vec3<T> {
        Vec3(other.0.as_(), other.1.as_(), other.2.as_())
    }
}

impl<T> Vec3<T>
where
    T: Num + Copy,
{
    pub fn add(&self, other: &Self) -> Self {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }

    pub fn sub(&self, other: &Self) -> Self {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }

    pub fn copy(&mut self, other: &Self) -> &Self {
        self.0 = other.0;
        self.1 = other.1;
        self.2 = other.2;
        self
    }

    pub fn set(&mut self, x: T, y: T, z: T) -> &Self {
        self.0 = x;
        self.1 = y;
        self.2 = z;
        self
    }

    pub fn scale(&self, scale: T) -> Self {
        Vec3(self.0 * scale, self.1 * scale, self.2 * scale)
    }

    pub fn scale_and_add(&self, other: &Self, scale: T) -> Self {
        Vec3(
            self.0 + other.0 * scale,
            self.1 + other.1 * scale,
            self.2 + other.2 * scale,
        )
    }
}

impl<T> Vec3<T>
where
    T: Float,
{
    pub fn len(&self) -> T {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    pub fn max(&self, other: &Self) -> Self {
        Vec3(
            Float::max(self.0, other.0),
            Float::max(self.1, other.1),
            Float::max(self.2, other.2),
        )
    }

    pub fn min(&self, other: &Self) -> Self {
        Vec3(
            Float::min(self.0, other.0),
            Float::min(self.1, other.1),
            Float::min(self.2, other.2),
        )
    }
}

impl<T: Num + Clone> Index<usize> for Vec3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index == 0 {
            &self.0
        } else if index == 1 {
            &self.1
        } else if index == 2 {
            &self.2
        } else {
            panic!("Index out of bounds for accessing Vec3.");
        }
    }
}

impl<T: Num + Clone> IndexMut<usize> for Vec3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index == 0 {
            &mut self.0
        } else if index == 1 {
            &mut self.1
        } else if index == 2 {
            &mut self.2
        } else {
            panic!("Index out of bounds for accessing Vec3.");
        }
    }
}
