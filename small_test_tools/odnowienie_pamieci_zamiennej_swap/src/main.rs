use odnowienie_pamieci_zamiennej_swap::Swap;

fn main() {
    match Swap::Off.execute() {
        Ok(status) => println!("Pomyślnie wyłączono pamięc wymiany - {}", status),
        Err(e) => eprintln!("Nie powiodło się wyłączenie pamięci wymiany - {}", e),
    }
    println!("Zakończono! ");

    match Swap::On.execute() {
        Ok(status) => println!("Pomyślnie włączono pamięc wymiany - {}", status),
        Err(e) => eprintln!("Nie powiodło się włączenie pamięci wymiany - {}", e),
    }
    println!("Odnowienie zakończono. ");
}
