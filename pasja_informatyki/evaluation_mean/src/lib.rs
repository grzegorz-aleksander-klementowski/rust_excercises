pub enum Messeges {
    Welcome,
    PrintEvaluations,
}

impl Messeges {
    pub fn print_messge (&self) {
        mach self {
            Messeges::Welcome => println!("Please enter value:"),
            PrintEvaluations    => println!("…"),
        }
    }
}


