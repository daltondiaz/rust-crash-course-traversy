// Vectors - Resizable arrays

use std::mem;

pub fn run(){
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    numbers.push(6);
    numbers.push(7);

    println!("===== VECTOR =======");
    println!("Vector: {:?}", numbers);

    numbers.pop();
    println!("Vector after pop: {:?}", numbers);

    println!("Vector - single value: {}", numbers[3]);


    println!("Vector - occupiues {} bytes ", mem::size_of_val(&numbers));
    
    let slice: &[i32] = &numbers[0..3];

    println!("Slice Vector: {:?}", slice);


    // Loop through vector values

    for x in numbers.iter(){
        println!("Number: {}", x);
    }
    
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec {:?}", numbers);
}