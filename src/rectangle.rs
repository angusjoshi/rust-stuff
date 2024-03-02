struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.height * self.width
    }
}

#[cfg(test)]
mod tests {
    use crate::rectangle::Rectangle;

    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle {
            width: 20,
            height: 10
        };

        let expected_result = 200;
        let result = rect.area();

        assert_eq!(expected_result, result)
    }
}