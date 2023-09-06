//use crate::restaurant::models::Items;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
    /*
    #[test]
    fn test_get_all_items() {
        expected = static value add here
        let (value, err) = Items::handle_get_all_items("");
        assert_eq!(value, expected);
    }
    */
}
