extern crate sdl2;


use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;


// Notes: unwrap(), extract value from a Some or a None without errors, I think...
// Use a lot except for places where panicking is a no-go

fn main() {
    // Initialization
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Main window
    let window = video_subsystem.window("Sunbeam", 600, 600).position_centered().opengl().build().unwrap();

    // Canvas object, dont understand why mutable
    let mut canvas = window.into_canvas().build().unwrap();

    // Events...? Why pump
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    // Game condition 
    let mut quit = false;

    // Main loop 
    while !quit {

        // Iterate through polls
        for event in event_pump.poll_iter() {
            
            // Depending on the event type, perform some action
            match event {
                // Quit if window is closed I am guessing
                Event::Quit { .. } => quit = true,
                // Quit if 'esc' key is pressed
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => quit = true,
                // No others at the moment
                _ => {}
            }
        }


        // Tell canvas it will draw Red
        canvas.set_draw_color(Color::RGB(255, 0, 0)); // Red color
        
        // Clear canvas
        canvas.clear();

        // Render to screen I think
        canvas.present();

    }
}
