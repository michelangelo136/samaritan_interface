
extern crate term_size;
//use ansi_term::Style;
//use std::io::{stdout, Write};
//use std::time;
use std::thread;
use termion::{color, cursor, clear, style};
use std::io::prelude::*;
use std::fs::File;
use std::mem;



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
   clear = clear::All);                                    // Clears screen

 for _t in 0..10 {                                       // Loops for 10 times
     if let Some((w, h)) = term_size::dimensions() {    // Gets terminal dimensions
         
     let hight = h / 2;                               // Sets hight and sends it to debug_size
     let weight = w / 2;                             // set weght and sends it to debug_size
     //debug_size(weight, hight);                   // Prints dimensions
     //stand_by(weight, hight);                    // Sends parameters to stand_by
     
     let mut file = File::open("input/input_file").expect("Unable to open th file"); // Gets input from input_file and converts it to string
     let mut contents = String::new();
     file.read_to_string(&mut contents).expect("Unable to read the file");
     
     if (contents == " ") {
     } else {
       input(weight, hight, &contents);
     }
     
    
    }


fn debug_size(x: usize, y: usize) { // Gets cordinents (x, y) and sets the censor
   x as i32;
   y as i32;
   let  x1: u16 = x as u16;
   let  y1: u16 = y as u16;

   println!("{goto}{underline} {input_x} {input_y} {reset_style}",
   goto = cursor::Goto(x1, y1),
   underline = style::Underline,
   reset_style = style::Reset,
   input_x = x,
   input_y = y);
   println!("{goto}{red}^{reset_color}",
   red = color::Fg(color::Red),
   goto = cursor::Goto(x1, y1 + 1),
   reset_color = color::Fg(color::Reset));
   debug_println!("{} {}", x, y);

}
  /* println!("{goto}{red}^",
  red = color::Fg(color::Red),
  goto = cursor::Goto(x1, y1 - 1)); */
  thread::sleep_ms(2000);
  println!("{clear}",
  clear = clear::All);
  
 }


fn stand_by(x: usize, y: usize) { // Set's stand_by ststus
   //x as i32;
   //y as i32;
   let x1: u16 = x as u16;
   let y1: u16 = y as u16;
   
   println!("{goto_1}CALCUL{underline}ATING RES{reset_style}PONSE{goto_2}{red}^{reset_color}",
   goto_1 = cursor::Goto(x1 - 10, y1),
   underline = style::Underline,
   reset_style = style::Reset,
   reset_color = color::Fg(color::Reset),
   red = color::Fg(color::Red),
   goto_2 = cursor::Goto(x1, y1 + 1));
    

}

fn input(x: usize, y: usize, contents: &str) { // Prints the input
   let x1: u16 = x as u16;
   let y1: u16 = y as u16;
   //let input1 = "test";
   let actuall_lenght = contents.len();
   let half_lenght = (actuall_lenght - 1) / 2;
   let half_lenght_u16: u16 = half_lenght as u16;  

   println!("{goto_1}{underline} {input_0}{reset_style}{input_1}{goto_2}{red}^{reset_color}",
   goto_1 = cursor::Goto(x1 - half_lenght_u16, y1),
   underline = style::Underline,
   reset_style = style::Reset,
   reset_color = color::Fg(color::Reset),
   red = color::Fg(color::Red),
   goto_2 = cursor::Goto(x1, y1 + 1),
   input_0 = contents,
   input_1 = actuall_lenght -1 );



}



}
