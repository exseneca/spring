# Spring 
A simple command line spring simulator. 

# Description
Ever wanted to simulate a vertical spring horizontally on the command line?
Now you can.

# Usage
Run with `cargo run` and provide a floating number start position and a damping amount.
Beware it will crash if it goes off the screen.

It's not yet calibrated to a realistic time. 

# Demo
## Undamped
![Screen recording of undamped motion](undamped.gif)
## Damped
![Screen recording of damped motion](damped.gif)

# TODO
- Deal with screen resizing.
- Callibrate to realistic units.
