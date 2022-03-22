use std::env;//import module from standard library
fn main {
let args: Vec<String> = env::args().collect();//args is a method of this module which can be collected in vector string
if args.len() != 3 {
println!("Usage:");
println!("sha1_cracker: <wordlist.txt> <sha1_hash>");//Here we can see println!, This is not a funtion, rust doesn't support println funtion and it's called as macro.
return;
}
}
//This is a error code 
