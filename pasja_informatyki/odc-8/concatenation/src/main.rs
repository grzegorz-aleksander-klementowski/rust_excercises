fn main() {
    let s1 = "Ala ma ".to_string();
    let s2 = "kota";

    let con = s1 + s2;
    // example to to_uppercase
    let con = con.to_uppercase();
    println!("{con}");

    // alternative
    //let con2 = format!("{s1}{s2}");
}
