
extern crate term_size;
//use ansi_term::Style;
//use std::io::{stdout, Write};
//use std::time;
use std::thread;
use termion::{color, cursor, clear, style};
use std::io::prelude::*;
use std::fs::File;
//use crossterm::cursor;


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
   clear = clear::All);                                      // Clears screen
   looper();   
   

fn looper() {
 for _t in 0..10 {                                       // Loops for 10 times
     if let Some((w, h)) = term_size::dimensions() {    // Gets terminal dimensions
         
     let hight = (h / 2) + 2 ;                        // Sets hight
     let weight = (w / 2) + 2;                       // Set weight 
     //debug_size(weight, hight);                   // Prints dimensions
     //calculation_re(weight, hight);              // Sends dimensions to calculating_re
     //awaiting(weight, hight);

     let mut file = File::open("input/display_file").expect("Unable to open the file"); // Gets input from input_file and converts it to string
     let mut contents = String::new();
     file.read_to_string(&mut contents).expect("Unable to read the file");
     let mut loops: u32 = 1;     

   debug_println!("loops = {}", loops);


     if &contents.trim() == &"" {		// Checks if the file has commands in it, then start a task
       	if loops < 4 {
	  loops = loops + 1;
          stand_by(weight, hight);		// Goes to stand_by mode
          //write();
 	  debug_println!("loops = {}", loops);
	} else {
	   awaiting(weight, hight);
	   loops = 0;
	}
     } 	else {
         input(weight, hight, &contents);	// If file has inpute, sends it to be displayed
         //stand_by(weight, hight);
     }
     
    }
  }

}
// *************************************************************

fn debug_size(x: usize, y: usize) { // Gets cordinents (x, y) and sets the censor, used only for de_buging the screen resolution
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
  //std::thread::sleep_ms(2000);
  
 


// *************************************************************

fn calculation_re(x: usize, y: usize) { // Set's stand_by status
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


// *************************************************************

fn input(x: usize, y: usize, contents: &str) { // Prints the input
   let x1: u16 = x as u16;
   let y1: u16 = y as u16;
   //let input1 = "test";
    for splited_input in contents.split_whitespace() {		// Loop as many times as words exists in splited_input
       thread::sleep_ms(1250);
       println!("{clear}",
       clear = clear::All);
   
       //let actuall_lenght = contents.len();
       //let half_lenght = (actuall_lenght - 1) / 2;
       //let half_lenght_u16: u16 = half_lenght as u16;
   
       let actuall_lenght = splited_input.len();		// Gets actual word lenght
       let half_lenght = (actuall_lenght - 1) / 2;		// Calculates half word lengh
       let half_lenght_u16: u16 = half_lenght as u16;		// Converts value to u16
   

       println!("{goto_1}{underline}{input_0}{reset_style}{goto_2}{red}^{reset_color}{goto_3}",
       goto_1 = cursor::Goto(x1 - half_lenght_u16, y1),
       underline = style::Underline,
       reset_style = style::Reset,
       reset_color = color::Fg(color::Reset),
       red = color::Fg(color::Red),
       goto_2 = cursor::Goto(x1, y1 + 1),
       input_0 = splited_input/*.to_owned() + " "*/,
       goto_3 = cursor::Goto(x1, y1 * 2));
       //input_1 = actuall_lenght -1 );
       debug_println!("**debug: input fn");
       debug_println!("**debug: Actuall lenght {}",
       actuall_lenght);
    
    }
  thread::sleep_ms(1500);
  clear_file(x, y);
  //stand_by(x, y);
}


// *************************************************************

fn awaiting (x: usize, y: usize) { // Set's stand_by status
   //x as i32;
   //y as i32;
   let x1: u16 = x as u16;
   let y1: u16 = y as u16;

   println!("{goto_1}AWAITIN{underline}G INSTR{reset_style}UCTIONS{goto_2}{red}^{reset_color}",
   goto_1 = cursor::Goto(x1 - 10, y1),
   underline = style::Underline,
   reset_style = style::Reset,
   reset_color = color::Fg(color::Reset),
   red = color::Fg(color::Red),
   goto_2 = cursor::Goto(x1, y1 + 1));
   thread::sleep_ms(1500);


}


// *************************************************************

fn clear_file(x: usize, y: usize) {
   let buffer = File::create("input/display_file");
   //buffer.write_all(b"test");
   debug_println!("**debug: clear_file fn");
   stand_by(x, y);
   
}


// *************************************************************

fn stand_by(x: usize, y: usize,) {
   let x1: u16 = x as u16;
   let y1: u16 = y as u16;
   
   println!("{clear}",
   clear = clear::All);
   
   println!("{goto_1}{underline}{input_0}{reset_style}{goto_2}",
   goto_1 = cursor::Goto(x1 - 1, y1),
   underline = style::Underline,
   reset_style = style::Reset,
   goto_2 = cursor::Goto(x1, y1 * 2),
   input_0 = "   ");
   debug_println!("**debug: stand_by fn");
   
   thread::sleep_ms(750);
   println!("{clear}",
   clear = clear::All);

   println!("{goto_1}{underline}{input_0}{reset_style}{goto_2}{red}^{reset_color}{goto_3}",
   goto_1 = cursor::Goto(x1 - 1, y1),
   underline = style::Underline,
   reset_style = style::Reset,
   reset_color = color::Fg(color::Reset),
   red = color::Fg(color::Red),
   goto_2 = cursor::Goto(x1, y1 + 1),
   goto_3 = cursor::Goto(x1, y1 * 2),
   input_0 = "   ");
   debug_println!("**debug: stand_by fn");
   
   thread::sleep_ms(750);
   looper();     
}


// *************************************************************




}
