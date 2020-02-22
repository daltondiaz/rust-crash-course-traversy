// Arrays - Fixed list where elements are the same data types

use std::mem;

pub fn run(){
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("===== ARRAY =======");
    println!("Arrays: {:?}", numbers);

    println!("Arrays - single value: {}", numbers[3]);

    let mut mut_numbers: [i32; 2] = [1, 2];

    println!("Arrays - mutable : {:?}", mut_numbers);
    
    mut_numbers[1] = 100;
    
    println!("Arrays - mutable changed : {:?}", mut_numbers);
    
    println!("Arrays - occupiues {} bytes ", mem::size_of_val(&numbers));
    println!("Arrays - mutable occupiues {} bytes ", mem::size_of_val(&mut_numbers));

    let slice: &[i32] = &numbers[0..2];

    println!("Slice: {:?}", slice);
    
}