pub mod point2;
pub mod vector2;

#[cfg(test)]
mod tests 
{
    use crate::point2::Point2;
    use crate::vector2::Vector2;

    #[test]
    fn it_works() 
    {
        let vec_a = Vector2::new(1.0, 1.0);
        let vec_b = Vector2::new(0.0, 2.0);
        
        assert_eq!(Vector2::dot(vec_a, vec_b), 2.0);

        assert_eq!(vec_a.length(), (2.0_f64).sqrt());
    }
    
    #[test]
    fn vector_operators ()
    {
        
    }

    #[test]
    fn point_operators ()
    {

    }
}