fn main() {
  let a: &i32;
// ffsdfgfcfcgcvvcffghgccfccddfvdd
  // a and b have same lifetime
  let b = 3;  
  a = &b;
  println!("{}",a);
  println!("{}",b);
}
