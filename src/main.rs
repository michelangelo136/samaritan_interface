
extern crate term_size;
//use ansi_term::Style;
//use std::io::{stdout, Write};
//use std::time;
use std::thread;
use termion::{color, cursor, clear, style};





// The debug version
#[cfg(feature = "debug")]
macro_rules! debug_println {
    ($( $args:expr ),*) => { println!( $( $args ),* ); }
}

// Non-debug version
#[cfg(not(feature = "debug"))]
macro_rules! debug_println {
    ($( $args:expr ),*) => {}
}


fn main() {
   println!("{}",
   clear = clear::All); // Clears screen

for _t in 0..10 { // Loops for 10 times
    if let Some((w, h)) = term_size::dimensions() { // Gets terminal dimensions
        
    let hight = h / 2; // Sets hight/weight and sends it to feedback
    let weight = w / 2;
    feedback(weight, hight);
    }


fn feedback(x: usize, y: usize) { // Gets cordinents (x, y) and sets the censor
   x as i32;
   y as i32;
   let  x1: u16 = x as u16;
   let  y1: u16 = y as u16;

   println!("{goto}{underline} {input_x} {input_y} ",
   goto = cursor::Goto(x1, y1),
   underline = style::Underline,
   input_x = x,
   input_y = y);
   debug_println!("{} {}", x, y);

}
   
  thread::sleep_ms(2000);
  println!("{clear}",
  clear = clear::All);
  
 }
}
