use std::io;
use term_size;
use std::{thread, time};

fn get_position(a_const: f64, beta: f64, omega_1: f64, delta: f64, time: f64) -> f64 {
    return  a_const*(- beta*time).exp()*(omega_1 * time - delta).cos();
}
fn main() {
    
    const STIFFNESS: f64 = 0.03;
    const MASS: f64 = 1.0;
    const DELTA_TIME: f64 = 1.0;
    let hundred_millis = time::Duration::from_millis(100);
    let mut position = String::new();
    println!("Enter initial position: ");
    io::stdin()
        .read_line(&mut position)
        .expect("Failed to read line");
    let mut position: f64 = position.trim().parse().expect("Please provide a floating number");
    
    let mut damping = String::new();
     println!("Enter damping: ");
    io::stdin()
        .read_line(&mut damping)
        .expect("Failed to read line");
    let damping: f64 = damping.trim().parse().expect("Please provide a floating number");
    let mut width = 0;
    if let Some((w, _)) = term_size::dimensions() {
        width = w;
    } else {
        println!("Unable to get term size")
    }
    // get repeated string
    let midpoint = width/2 - 1;
    let mut string_length: i64 = (midpoint as i64) + (position as i64);
    if string_length < 0 || string_length > (width as i64) {
        eprintln!("Went off screen!");
        std::process::exit(1)
    }
    let mut string_str  = std::iter::repeat("-").take(string_length as usize).collect::<String>();
    println!("{}o", string_str);

    let omega_0_squared = STIFFNESS/MASS;
    let beta = damping/(2.0*MASS);
    let beta_squared = beta*beta;
    if beta_squared > omega_0_squared {
        eprintln!("beta (damping/2mass) must be less than omega0 squared (stiffness/mass)");
        std::process::exit(1);
    }
    let omega_1 = (omega_0_squared - beta_squared).sqrt();
    let delta = (beta/omega_1).atan();
    let a_const = position / delta.cos();

    //let D_1 = -beta + (beta_squared - omega_0_squared).sqrt();
    //let D_2 = -beta - (beta_squared - omega_0_squared).sqrt();
    //let C_2 = position*D_1/(D_1 - D_2);
    //let C_1 = position - C_2;
    let mut time: f64 = 0.0;
    loop {

        position = get_position(a_const, beta, omega_1, delta, time);

        string_length  = (midpoint as i64) + (position as i64);
        if string_length < 0 || string_length > (width as i64) {
            eprintln!("Went off screen!");
            std::process::exit(1)
        }
        string_str  = std::iter::repeat("-").take(string_length as usize).collect::<String>();
        println!("{}o", string_str);
        time += DELTA_TIME;
        thread::sleep(hundred_millis);
    }
}
