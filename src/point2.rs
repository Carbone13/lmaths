use std::ops::*;
use std::fmt::Formatter;

use crate::vector2::Vector2;

#[allow(dead_code)]
#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct Point2
{
    pub x:isize,
    pub y:isize,
}

#[allow(dead_code)]
impl Point2
{
    pub const ZERO:Self = Self { x: 0, y:0 };
    pub const ONE:Self = Self { x: 1, y:1 };
    pub const MINUS_ONE:Self = Self { x: -1, y:-1 };
    pub const X_UNIT:Self = Self { x: 1, y:0 };
    pub const Y_UNIT:Self = Self { x: 0, y:1 };

    #[inline]
    pub fn new (x:isize, y:isize) -> Self
    {
        Point2 { x, y }
    }

    #[inline]
    pub fn length (self) -> f64
    {
        self.sqr_length().sqrt()
    }

    #[inline]
    pub fn sqr_length (self) -> f64
    {
        (self.x*self.x + self.y*self.y) as f64
    }

    #[inline]
    pub fn dot (a:Point2, b:Point2) -> f64
    {
        (a.x * b.x + a.y * b.y) as f64
    }

    #[inline]
    pub fn scaled (self, f:Point2) -> Self
    {
        Point2 { x: self.x * f.x, y: self.y * f.y }
    }

    #[inline]
    pub fn scale (&mut self, f:Point2) {
        self.x *= f.x;
        self.y *= f.y;
    }

    #[inline]
    pub fn distance (a:Point2, b:Point2) -> f64
    {
        (a - b).length()
    }

    #[inline]
    pub fn as_vector2(self) -> Vector2
    {
        Vector2::new(self.x as f64, self.y as f64)
    }

    #[inline]
    pub fn from_vector2 (p:Vector2) -> Self
    {
        Point2 { x: p.x as isize, y:p.y as isize }
    }

    #[inline]
    pub fn from_vec2(v:[f32; 2]) -> Self
    {
        Point2 { x: v[0] as isize, y:v[1] as isize }
    }

    #[inline]
    pub fn to_vec2(self) -> [f32; 2]
    {
        [self.x as f32, self.y as f32]
    }
}

impl std::fmt::Display for Point2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "x:{0} y:{1}", self.x, self.y)
    }
}

impl Sub for Point2 
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output 
    {
        Self 
        {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Point2
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output
    {
        Self
        {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul for Point2
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output
    {
        Self
        {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<isize> for Point2
{
    type Output = Self;

    fn mul(self, rhs:isize) -> Self::Output
    {
        Self
        {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign for Point2
{
    fn mul_assign(&mut self, rhs: Self)
    {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
    }
}

impl MulAssign<isize> for Point2
{
    fn mul_assign(&mut self, rhs:isize)
    {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

impl Neg for Point2
{
    type Output = Self;

    fn neg(self) -> Self::Output
    {
        Self
        {
            x: -self.x,
            y: -self.y,
        }
    }
}