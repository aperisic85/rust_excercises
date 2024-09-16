enum Shape {
    Circle,
    Square,
    Rectangle,
    Triangle,
    Pentagon,
}

impl Shape {
    // TODO: Implement the `n_sides` method using a `match`.
    pub fn n_sides(&self) -> u8 {
        match self {
            Self::Circle => 0,
            Self::Rectangle => 4,
            Self::Pentagon => 5,
            Self::Triangle => 3,
            Self::Square => 4,
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
