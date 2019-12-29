extern crate term_size;
//use ansi_term::Style;
use std::io::{stdout, Write};
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
for x in 0..10 {
    if let Some((w, h)) = term_size::dimensions() {
        
        if w % 2 == 0 {
            debug_println!("even");
        } else {
            debug_println!("odd");
        }
        if h % 2 == 0 {
            debug_println!("even");
            let hight = (h-4) / 2;
            debug_println!("{} {}", h-4, hight);
        } else {
            debug_println!("odd");
            let hight = (h-4) / 2;
            debug_println!("{} {}", h-4, hight);
        }
        feedback();
    } else {
        println!("Unable to get term size :(")
    }


fn feedback() {
   println!("{goto}{underline} TEST ",
   goto = cursor::Goto(5, 15),
   underline = style::Underline);


}
   
  thread::sleep_ms(2000);
  println!("{clear}",
  clear = clear::All);
  
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

