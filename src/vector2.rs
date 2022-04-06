use std::ops::*;
use std::fmt::Formatter;

use crate::point2::Point2;

#[allow(dead_code)]
#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct Vector2
{
    pub x:f64,
    pub y:f64
}

#[allow(dead_code)]
impl Vector2
{
    pub const ZERO:Self = Self { x: 0.0, y:0.0 };
    pub const ONE:Self = Self { x: 1.0, y:1.0 };
    pub const MINUS_ONE:Self = Self { x: -1.0, y:-1.0 };
    pub const X_UNIT:Self = Self { x: 1.0, y:0.0 };
    pub const Y_UNIT:Self = Self { x: 0.0, y:1.0 };

    #[inline]
    pub fn new (x:f64, y:f64) -> Self 
    {
        Vector2 { x, y }
    }

    #[inline]
    pub fn length (self) -> f64
    {
        self.sqr_length().sqrt()
    }

    #[inline]
    pub fn sqr_length (self) -> f64
    {
        self.x*self.x + self.y*self.y
    }

    #[inline]
    pub fn normalize (mut self) {
        let l:f64 = self.length();
        self.x /= l;
        self.y /= l;
    }

    #[inline]
    pub fn normalized (self) -> Self {
        let l:f64 = self.length();
        
        if l == 0.0
        {
            return Vector2::new(1.0, 0.0);
        }
        
        Vector2
        {
            x: self.x / l,
            y: self.y / l,
        }
    }

    #[inline]
    pub fn dot (self, b:Vector2) -> f64
    {
        self.x * b.x + self.y * b.y
    }

    #[inline]
    pub fn distance (a:Vector2, b:Vector2) -> f64 
    {
        (a - b).length()
    }

    #[inline]
    pub fn lerp(a:Vector2, b:Vector2, t:f64) -> Self
    {
        a + (b - a) * t
    }

    #[inline]
    pub fn as_point2(self) -> Point2 
    {
        Point2::new(self.x as isize, self.y as isize)
    }

    #[inline]
    pub fn from_point2 (p:Point2) -> Self
    {
        Vector2 { x: p.x as f64, y:p.y as f64 }
    }

    #[inline]
    pub fn from_vec2(v:[f32; 2]) -> Self 
    {
        Vector2 { x: v[0] as f64, y:v[1] as f64 }
    }

    #[inline]
    pub fn to_vec2(self) -> [f32; 2] 
    {
        [self.x as f32, self.y as f32]
    }
}

#[inline]
pub fn dot (a:Vector2, b:Vector2) -> f64
{
    a.x * b.x + a.y * b.y
}

#[inline]
pub fn triple_product(a:Vector2, b:Vector2, c:Vector2) -> Vector2 {
    let ac = a.x * c.x + a.y * c.y;
    let bc = b.x * c.x + b.y * c.y;
    Vector2::new(b.x * ac - a.x * bc, b.y * ac - a.y * bc)
}

impl std::fmt::Display for Vector2
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "x:{0} y:{1}", self.x, self.y)
    }
}

impl Sub for Vector2
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

impl Add for Vector2
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

impl SubAssign for Vector2
{
    fn sub_assign(&mut self, other: Self)
    {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
    }
}

impl AddAssign for Vector2
{
    fn add_assign(&mut self, other: Self)
    {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
}

impl Mul for Vector2
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

impl Mul<f64> for Vector2
{
    type Output = Self;

    fn mul(self, rhs:f64) -> Self::Output
    {
        Self
        {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<isize> for Vector2
{
    type Output = Self;

    fn mul(self, rhs:isize) -> Self::Output
    {
        Self
        {
            x: self.x * rhs as f64,
            y: self.y * rhs as f64,
        }
    }
}

impl MulAssign for Vector2
{
    fn mul_assign(&mut self, rhs: Self)
    {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
    }
}

impl MulAssign<f64> for Vector2
{
    fn mul_assign(&mut self, rhs:f64)
    {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

impl MulAssign<isize> for Vector2
{
    fn mul_assign(&mut self, rhs:isize)
    {
        self.x = self.x * rhs as f64;
        self.y = self.y * rhs as f64;
    }
}

impl Div for Vector2
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output
    {
        Self
        {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Div<f64> for Vector2
{
    type Output = Self;

    fn div(self, rhs:f64) -> Self::Output
    {
        Self
        {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Div<isize> for Vector2
{
    type Output = Self;

    fn div(self, rhs:isize) -> Self::Output
    {
        Self
        {
            x: self.x / rhs as f64,
            y: self.y / rhs as f64,
        }
    }
}

impl DivAssign for Vector2
{
    fn div_assign(&mut self, rhs: Self)
    {
        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
    }
}

impl DivAssign<f64> for Vector2
{
    fn div_assign(&mut self, rhs:f64)
    {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
    }
}

impl DivAssign<isize> for Vector2
{
    fn div_assign(&mut self, rhs:isize)
    {
        self.x = self.x / rhs as f64;
        self.y = self.y / rhs as f64;
    }
}

impl Neg for Vector2
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