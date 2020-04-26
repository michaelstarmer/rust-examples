/*
 * Tutorial 01
 */
fn main()
{
  let mut x = 10;

  {
      let xr = &mut x;
      *xr += 1;
      println!("xr: {}", xr);
  }

  println!("x: {}", x);
  
}