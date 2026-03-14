enum Shape {
    Circle,
    Square,
    Rectangle,
    Triangle,
    Pentagon,
}

impl Shape {
    
    pub fn n_sides(&self) -> u8 {
        match self {
            Self::Circle => 0,
            Self::Triangle => 3,
            Self::Rectangle => 4,
            Self::Square => 4,
            Self::Pentagon => 5,
            _ => 0xff // catch all
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        assert_eq!(Shape::Circle.n_sides(), 0);
    }

    #[test]
    fn test_square() {
        assert_eq!(Shape::Square.n_sides(), 4);
    }

    #[test]
    fn test_rectangle() {
        assert_eq!(Shape::Rectangle.n_sides(), 4);
    }

    #[test]
    fn test_triangle() {
        assert_eq!(Shape::Triangle.n_sides(), 3);
    }

    #[test]
    fn test_pentagon() {
        assert_eq!(Shape::Pentagon.n_sides(), 5);
    }
}
