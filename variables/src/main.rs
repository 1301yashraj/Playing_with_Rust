fn main() {
   let x = 5;
    println!("The Variable  is {}",x);
    // x= 6;
    // x= 6;
    //   |     ^^^^ cannot assign twice to immutable variable
    // so a variable is immutable in RUST by default
    // but we can shadow it
    let x  = 7f64/3.0;
    println!("The Variable updated is {}",x);
    let heart_eyed_cat = 1_000; // this is 1000 not 1.000
    println!("The Variable updated is {}",heart_eyed_cat);
    let float = 6_9f32;
    println!("Printing a float {}",float);
}
