/* [PL]
3. Napisz funkcję

   ```
   fn indeksy(tablica: ..., element: &str) -> ...
   ```

   która zwróci wektor indeksów (licząc od zera), na których znajduje się w zadanej tablicy podany element.

[EN]
3. Write:

   ```rust
   fn indeksy(tablica: ..., element: &str) -> ...
   ```

   It returns a vector of zero-based indices at which the specified element occurs in the given array. */

fn indeksy(tablica: &[&str], element: &str) -> Vec<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_single_occurrence() {
        let array = ["apple", "banana", "cherry"];

        assert_eq!(indeksy(&array, "banana"), vec![1]);
    }

    #[test]
    fn finds_multiple_occurrences() {
        let array = ["apple", "banana", "apple", "cherry", "apple"];

        assert_eq!(indeksy(&array, "apple"), vec![0, 2, 4]);
    }

    #[test]
    fn returns_empty_when_element_is_absent() {
        let array = ["apple", "banana", "cherry"];

        assert_eq!(indeksy(&array, "orange"), Vec::<usize>::new());
    }

    #[test]
    fn handles_empty_array() {
        let array: [&str; 0] = [];

        assert_eq!(indeksy(&array, "apple"), Vec::<usize>::new());
    }

    #[test]
    fn finds_all_elements_when_identical() {
        let array = ["rust", "rust", "rust", "rust"];

        assert_eq!(indeksy(&array, "rust"), vec![0, 1, 2, 3]);
    }

    #[test]
    fn distinguishes_exact_matches_from_substrings() {
        let array = ["rust", "rustacean", "trust", "rust"];

        assert_eq!(indeksy(&array, "rust"), vec![0, 3]);
    }

    #[test]
    fn handles_unicode_and_case_sensitivity() {
        let array = ["żółw", "Żółw", "kot", "żółw", "zolw"];

        assert_eq!(indeksy(&array, "żółw"), vec![0, 3]);
    }
}
