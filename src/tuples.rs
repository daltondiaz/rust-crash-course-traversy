// Tuples group together values of diffrent types
// Max 12 elements

pub fn run(){
    let person: (&str, &str, i8) = ("Dalton", "SP", 29);

    println!("Tuples: {} is from {} and is {}", person.0, person.1, person.2);
}