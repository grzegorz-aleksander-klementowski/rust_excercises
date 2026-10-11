/*
[PL]
2. Napisz zestaw funkcji, które dla danego wektora napisów zwrócą:

   * wektor zawierający napisy krótsze niż 4 znaki;
   * wektor napisów nie zawierających liter 'a' ani 'A';
   * wektor napisów zawierających cyfry;
   * wektor zawierający te same napisy ale odwrócone;
   * wektor napisów zawierających podwojoną literę (np.: inny, pizza, brutto, lekki, dzienny, itp).

[EN]
2. Write functions which, for a given vector of strings, return:

   * strings shorter than four characters;
   * strings containing neither `a` nor `A`;
   * strings containing digits;
   * the same strings reversed;
   * strings containing a doubled letter, for example `inny`, `pizza`, `brutto`, `lekki`, or `dzienny`.
 */

// === SUBEXERCISE 1: strings shorter than four characters === \\
#[allow(dead_code)]
fn shorter_than_four(strings: &[String]) -> Vec<String> {
    strings
        .iter()
        .filter(|str| str.len() < 4)
        .cloned()
        .collect()
}

// === SUBEXERCISE 2: strings shorter than four characters === \\
#[allow(dead_code)]
fn without_a(strings: &[String]) -> Vec<String> {
    strings
        .iter()
        .filter(|str| str.chars().all(|char| char != 'a' && char != 'A'))
        .cloned()
        .collect()
}

// === SUBEXERCISE 3: strings containing digits === \\
#[allow(dead_code)]
fn containing_digits(strings: &[String]) -> Vec<String> {
    strings
        .iter()
        .filter(|str| str.chars().any(|char| char.is_ascii_digit()))
        .cloned()
        .collect()
}

// === SUBEXERCISE 4: strings shorter than four characters === \\
#[allow(dead_code)]
fn reversed(strings: &[String]) -> Vec<String> {
    todo!()
}

// === SUBEXERCISE 6: strings shorter than four characters === \\
#[allow(dead_code)]
fn containing_doubled_letter(strings: &[String]) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_strings() -> Vec<String> {
        vec![
            "kot".into(),    // shorter than four characters
            "dom".into(),    // shorter than four characters
            "Ala".into(),    // contains A
            "pies".into(),   // exactly four characters
            "123abc".into(), // contains digits and 'a'
            "brutto".into(), // doubled 't'
            "pizza".into(),  // doubled 'z'
            "lekki".into(),  // doubled 'k'
            "rower".into(),  // normal word
            "Inny".into(),   // doubled 'n'
        ]
    }

    // === SUBEXERCISE 1: strings shorter than four characters === \\
    #[test]
    fn returns_strings_shorter_than_four_characters() {
        let input = test_strings();

        let result = shorter_than_four(&input);

        assert_eq!(
            result,
            vec!["kot".to_string(), "dom".to_string(), "Ala".to_string()]
        );
    }

    // === SUBEXERCISE 2: strings containing neither 'a' nor 'A' === \\
    #[test]
    fn returns_strings_without_a_or_a() {
        let input = test_strings();

        let result = without_a(&input);

        assert_eq!(
            result,
            vec![
                "kot".to_string(),
                "dom".to_string(),
                "pies".to_string(),
                "brutto".to_string(),
                "lekki".to_string(),
                "rower".to_string(),
                "Inny".to_string(),
            ]
        );
    }

    // === SUBEXERCISE 3: strings containing digits === \\
    #[test]
    fn returns_strings_containing_digits() {
        let input = test_strings();

        let result = containing_digits(&input);

        assert_eq!(result, vec!["123abc".to_string(),]);
    }

    // === SUBEXERCISE 4: strings reversed === \\
    #[test]
    fn returns_reversed_strings() {
        let input = vec!["rust".to_string(), "abc".to_string()];

        let result = reversed(&input);

        assert_eq!(result, vec!["tsur".to_string(), "cba".to_string(),]);
    }

    // === SUBEXERCISE 5: strings containing doubled letters === \\
    #[test]
    fn returns_strings_with_doubled_letters() {
        let input = test_strings();

        let result = containing_doubled_letter(&input);

        assert_eq!(
            result,
            vec![
                "brutto".to_string(),
                "pizza".to_string(),
                "lekki".to_string(),
                "Inny".to_string(),
            ]
        );
    }

    // === EDGE CASE: empty vector === \\
    #[test]
    fn handles_empty_vector() {
        let input: Vec<String> = vec![];

        assert!(shorter_than_four(&input).is_empty());
        assert!(without_a(&input).is_empty());
        assert!(containing_digits(&input).is_empty());
        assert!(reversed(&input).is_empty());
        assert!(containing_doubled_letter(&input).is_empty());
    }

    // === EDGE CASE: single-character strings === \\
    #[test]
    fn handles_single_character_strings() {
        let input = vec!["a".to_string(), "x".to_string()];

        assert_eq!(
            shorter_than_four(&input),
            vec!["a".to_string(), "x".to_string(),]
        );
    }
}
