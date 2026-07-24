/*
[PL]
2. Napisz funkcję o nagłówku

   ```
   fn eight_b_bin_into_decimal(z: &str) -> Option<u8>
   ```

   obliczającą wartość całkowitą bez znaku zapisaną w systemie dwójkowym — pod warunkiem, że mieści się na ośmiu bitach. Jeśli nie (lub w zapisie występuje znak inny niż cyfra dwójkowa lub parametr jest pusty), to wynikiem jest `None`.


[EN]
2. Write a function about the header

   ```
   fn 2_sys_value(z: &str) -> Option<u8>
   ```

   that computes an unsigned integer value written in binary - provided it fits in eight bits. If not (or the notation contains a non-binary character or the parameter is empty), the result is `None`.
 */

fn main() {
    let example = "02";
    println!(
        "Example {example}, res: {:?}",
        z2_bin_into_oct::eight_b_bin_into_decimal(example)
    );
}
