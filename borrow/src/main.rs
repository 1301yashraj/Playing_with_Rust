fn main() {
    let mut a = String::from("hello");
    println!("Got the value {a}");
    let x =  &a;
    println!("Got the value  x as {x}");
    let y = change_value(&mut a);

    println!("Got the value {a} {y}");

}

fn change_value(val : &mut String) -> String{
   val.replace("hello","YASH RAJ")
}
