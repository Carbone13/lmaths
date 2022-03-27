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
    
    pub fn new (x:f64, y:f64) -> Self 
    {
        Vector2 { x, y }
    }
    
    pub fn length (self) -> f64
    {
        self.sqr_length().sqrt()
    }
    
    pub fn sqr_length (self) -> f64
    {
        self.x*self.x + self.y*self.y
    }
    
    pub fn normalize (mut self)
    {
        let l:f64 = self.length();
        self.x /= l;
        self.y /= l;
    }
    
    pub fn normalized (self) -> Self
    {
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
    
    pub fn dot (a:Vector2, b:Vector2) -> f64
    {
        a.x * b.x + a.y * b.y
    }
        
    pub fn scaled (self, f:Vector2) -> Self
    {
        Vector2 { x: self.x * f.x, y: self.y * f.y }
    }

    pub fn scale (&mut self, f:Vector2)
    {
        self.x *= f.x;
        self.y *= f.y;
    }
    
    pub fn distance (a:Vector2, b:Vector2) -> f64 
    {
        (a - b).length()
    }

    pub fn lerp(a:Vector2, b:Vector2, t:f64) -> Self
    {
        a + (b - a) * t
    }

    pub fn as_point2(self) -> Point2 
    {
        Point2::new(self.x as isize, self.y as isize)
    }
    
    pub fn from_point2 (p:Point2) -> Self
    {
        Vector2 { x: p.x as f64, y:p.y as f64 }
    }
    
    pub fn from_vec2(v:[f32; 2]) -> Self 
    {
        Vector2 { x: v[0] as f64, y:v[1] as f64 }
    }

    pub fn to_vec2(self) -> [f32; 2] 
    {
        [self.x as f32, self.y as f32]
    }
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