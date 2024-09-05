fn main() {
  let a: &i32;
// vgfgfffgfdfgxxdfff
  // a and b have same lifetime
  let b = 3;  
  a = &b;
  println!("{}",a);
  println!("{}",b);
}
