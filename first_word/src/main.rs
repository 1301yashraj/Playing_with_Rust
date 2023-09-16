fn main() {
    println!("Return the first word that comes before space");
    let str = String::from("Hello World");
    let (pointer_to_end_of_first_word,sec_word) = first_word(&str);
    println!("First WOrd :: {}and Second WOrd ::{}",pointer_to_end_of_first_word,sec_word);

}

fn first_word(s: &str)-> (&str,&str) {
   let bytes  = s.as_bytes();
   println!("What the F is this {:?}",bytes);
   for (i,&item) in bytes.iter().enumerate(){
      if item == b' ' {return (&s[..i+1],&s[i+1..])}
   }
   (&s[..],&s[..])
}