use std::ops::{Add, AddAssign, Sub, SubAssign, Neg};

pub trait Angle: Into<Degrees> + From<Degrees> + From<Radians> + Into<Radians> {}

#[derive(Default, Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Degrees(pub f64);

#[derive(Default, Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Radians(pub f64);

impl Angle for Degrees {}
impl Angle for Radians {}

///////////////////////////////////////////////////////////////////////////////////////////////////
/// Conversion ////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////

impl From<Radians> for Degrees {
    fn from(rad: Radians) -> Self {
        Self(rad.0.to_degrees())
    }
}

impl From<Degrees> for Radians {
    fn from(deg: Degrees) -> Self {
        Self(deg.0.to_radians())
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
/// Operators /////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////

impl<T> Add<T> for Degrees
where
    T: Angle,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let rhs: Self = rhs.into();
        Self(self.0 + rhs.0)
    }
}
impl<T> Add<T> for Radians
where
    T: Angle,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let rhs: Self = rhs.into();
        Self(self.0 + rhs.0)
    }
}

impl<T> AddAssign<T> for Degrees
where
    T: Angle,
{
    fn add_assign(&mut self, rhs: T) {
        let rhs: Self = rhs.into();
        self.0 += rhs.0;
    }
}
impl<T> AddAssign<T> for Radians
where
    T: Angle,
{
    fn add_assign(&mut self, rhs: T) {
        let rhs: Self = rhs.into();
        self.0 += rhs.0;
    }
}

impl<T> Sub<T> for Degrees
where
    T: Angle,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs: Self = rhs.into();
        Self(self.0 - rhs.0)
    }
}
impl<T> Sub<T> for Radians
where
    T: Angle,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs: Self = rhs.into();
        Self(self.0 - rhs.0)
    }
}

impl<T> SubAssign<T> for Degrees
where
    T: Angle,
{
    fn sub_assign(&mut self, rhs: T) {
        let rhs: Self = rhs.into();
        self.0 -= rhs.0;
    }
}
impl<T> SubAssign<T> for Radians
where
    T: Angle,
{
    fn sub_assign(&mut self, rhs: T) {
        let rhs: Self = rhs.into();
        self.0 -= rhs.0;
    }
}

impl Neg for Degrees {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}
impl Neg for Radians {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}