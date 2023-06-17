// #[derive(Copy, Clone)]
pub enum Expression {
    Add(Vec<Expression>),
    Multiply(Vec<Expression>),
    Number(i32)
}

pub fn evaluate_addition(expression: &Expression) -> i32 {
    if let Expression::Add(expressions) = expression {
        let iter = expressions.iter();
        iter.fold(0, |total, next| total + evaluate(next))
    } else {
        panic!("addition not provided")
    }
}

pub fn evaluate_multiply(expression: &Expression) -> i32 {
    if let Expression::Multiply(expressions) = expression {
        let iter = expressions.iter();
        iter.fold(1, |total, next| total * evaluate(next))
    } else {
        panic!("addition not provided")
    }
}

pub fn evaluate(expression: &Expression) -> i32 {
    match expression {
        Expression::Add(_) => evaluate_addition(expression),
        Expression::Multiply(_) => evaluate_multiply(expression),
        Expression::Number(val) => *val
    }
}

fn main() {
    let addition = Expression::Add(vec![Expression::Number(2), Expression::Number(2)]);
    println!("2 + 2 is {}", evaluate(&addition));
}

// Arrange
// Act
// Assert

// Given
// When
// Then

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }

    #[test]
    fn test_addition() {
        // Arrange
        let three = crate::Expression::Number(3);
        let four = crate::Expression::Number(4);
        let addition = crate::Expression::Add(vec![three, four]);
        // Act
        let sum = crate::evaluate_addition(&addition);
        // Assert
        assert_eq!(sum, 7);
    }

    #[test]
    fn test_multiply() {
        // Arrange
        let three = crate::Expression::Number(3);
        let four = crate::Expression::Number(4);
        let multiplication = crate::Expression::Multiply(vec![three, four]);
        // Act
        let product = crate::evaluate_multiply(&multiplication);
        // Assert
        assert_eq!(product, 12);
    }
}