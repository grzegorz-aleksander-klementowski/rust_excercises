// 1. Napisz funkcję trójargumentową, która poprzestawia wartości swoich argumentów (dla ustalenia uwagi: typu i32) tak, by były uporządkowane niemalejąco.
// 1. Write a three-argument function that rearranges the values of its arguments (for reference: type i32) so that they are ordered in non-descending order.,

fn sort3(a: i32, b: i32, c: i32) -> (i32, i32, i32) {
    let mut result = [a, b, c];
    result.sort();

    (result[0], result[1], result[2])
}

fn main() {
    println!("{:?}", sort3(2, 1, 3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn already_sorted() {
        assert_eq!(sort3(1, 2, 3), (1, 2, 3));
    }

    #[test]
    fn reverse_order() {
        assert_eq!(sort3(3, 2, 1), (1, 2, 3));
    }

    #[test]
    fn random_order() {
        assert_eq!(sort3(2, 3, 1), (1, 2, 3));
    }

    #[test]
    fn duplicates() {
        assert_eq!(sort3(4, 1, 4), (1, 4, 4));
    }

    #[test]
    fn all_equal() {
        assert_eq!(sort3(5, 5, 5), (5, 5, 5));
    }

    #[test]
    fn negative_numbers() {
        assert_eq!(sort3(-3, 10, -1), (-3, -1, 10));
    }

    #[test]
    fn includes_zero() {
        assert_eq!(sort3(0, -5, 2), (-5, 0, 2));
    }

    #[test]
    fn i32_boundaries() {
        assert_eq!(sort3(i32::MAX, 0, i32::MIN), (i32::MIN, 0, i32::MAX));
    }
}
