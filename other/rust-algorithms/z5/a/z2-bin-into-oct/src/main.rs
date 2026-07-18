/*
[PL]
2. Napisz funkcję o nagłówku

   ```
   fn wartosc_syst2(z: &str) -> Option<u8>
   ```

   obliczającą wartość całkowitą bez znaku zapisaną w systemie dwójkowym — pod warunkiem, że mieści się na ośmiu bitach. Jeśli nie (lub w zapisie występuje znak inny niż cyfra dwójkowa lub parametr jest pusty), to wynikiem jest `None`.


[EN]
2. Write a function about the header

   ```
   fn 2_sys_value(z: &str) -> Option<u8>
   ```

   that computes an unsigned integer value written in binary - provided it fits in eight bits. If not (or the notation contains a non-binary character or the parameter is empty), the result is `None`.
 */

// Change the name for better understanding from `2_sys_value`
fn bin_sys_value(z: &str) -> Option<u8> {
    // Validate the string
    if z.is_empty() {
        return None;
    }
    for c in z.chars() {
        if !(c != '0' || c != '1') {
            return None;
        }
    }
    todo!()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_validation_input_str() {
        assert_eq!(bin_sys_value(""), None);
        assert_eq!(bin_sys_value("abc"), None);
        assert_eq!(bin_sys_value("10c"), None);
        assert_eq!(bin_sys_value("02"), None);
        assert_eq!(bin_sys_value("102"), None);
    }
}
