extern crate term_size;
//extern crate num;
//use num::Integer;

fn main() {
    if let Some((w, h)) = term_size::dimensions() {
        if w % 2 == 0 {
            println!("even")
            
        } else {
            println!("odd")
        }
        if h % 2 == 0 {
            println!("even");
            let hight = (h-4) / 2;
            println!("{} {}", h-4, hight);
        } else {
            println!("odd")
        }
    } else {
        println!("Unable to get term size :(")
    }
}

/*
fn even_or_odd() {
  let number = w;
  if number % 2 == 0 {
      println!("{} its even.", w);
  } else {
      println!("{} its odd.", w);
  }

}

fn fool(var:bool) {
  if var {
     println!("1");
  } else if !var {
     println! ("0");
  }
}
*/
