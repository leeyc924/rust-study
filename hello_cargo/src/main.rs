fn main() {

    let guess: u32 = "42".parse().expect("Not a number!");
    let x: i32 = 32;
    let y: char = 'A';
    let c    = "Asss";
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); // This will print `hello, world!`
    


}
