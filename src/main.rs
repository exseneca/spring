use std::io;
use term_size;
use std::{thread, time};
fn main() {
    //
    let stiffness = 0.03;
    let mut position = String::new();
    let mut velocity: f64 = 0.0;
    println!("Enter initial position: ");
    io::stdin()
        .read_line(&mut position)
        .expect("Failed to read line");
    let mut position: f64 = position.trim().parse().expect("Please provide a floating number");
    // position is the distance from the equilibrium - is closer to the left and +vs is to the right
    // get the number of chars width of the screen.
    // calculate equilibrium
    // draw the left side
    // draw the ball
    // update the postion and draw again.
    let mut width = 0;
    if let Some((w, _)) = term_size::dimensions() {
        width = w;
    } else {
        println!("Unable to get term size")
    }
    // get repeated string
    let midpoint = width/2 - 1;
    let mut string_length: i64 = (midpoint as i64) + (position as i64);
    // TODO: what if the string_length is less than zero?
    if string_length < 0 || string_length > (width as i64) {
        eprintln!("Went off screen!");
        std::process::exit(1)
    }
    let mut string_str  = std::iter::repeat("-").take(string_length as usize).collect::<String>();
    //println!("start position {position}, length: {string_length}");
    println!("{}o", string_str);
    loop {
        // calulate new position
        let force = - stiffness * position;
        velocity = velocity + force;
        position = position + velocity;
        string_length = (midpoint as i64) + (position as i64);
        string_str = std::iter::repeat("-").take(string_length as usize).collect::<String>();

        if string_length < 0 || string_length > (width as i64) {
        eprintln!("Went off screen!");
        std::process::exit(1)
        }
        println!("{}o", string_str);
        let hundred_millis = time::Duration::from_millis(100);
        thread::sleep(hundred_millis);
    }
}
