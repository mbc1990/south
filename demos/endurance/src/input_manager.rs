use sdl2::EventPump;
use crate::keyboard_state::KeyboardState;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct InputManager {
    event_pump: EventPump,
    keyboard_state: KeyboardState
}

impl InputManager {
    pub fn new(event_pump: EventPump) -> InputManager {
        let keyboard_state = KeyboardState{w:false, a:false, s:false, d:false, esc:false};
        return InputManager{event_pump, keyboard_state};
    }

    pub fn get_keyboard_state(&mut self) -> KeyboardState {
        // get the inputs here
        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::W), repeat: false, .. } => {
                    // world.key_w();
                    println!("Key down W");
                    self.keyboard_state.w = true;
                },
                Event::KeyDown { keycode: Some(Keycode::A), repeat: false, .. } => {
                    // world.key_a();
                    println!("Key down A");
                    self.keyboard_state.a = true;
                },
                Event::KeyDown { keycode: Some(Keycode::S), repeat: false, .. } => {
                    // world.key_s();
                    println!("Key down S");
                    self.keyboard_state.s = true;
                },
                Event::KeyDown { keycode: Some(Keycode::D), repeat: false, .. } => {
                    // world.key_d();
                    println!("Key down D");
                    self.keyboard_state.d = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Escape), repeat: false, .. } => {
                    // world.key_d();
                    println!("Key down esc");
                    self.keyboard_state.esc = true;
                },
                Event::KeyUp { keycode: Some(Keycode::W), repeat: false, .. } => {
                    self.keyboard_state.w = false;
                    println!("Key up W");
                },
                Event::KeyUp { keycode: Some(Keycode::A), repeat: false, .. } => {
                    self.keyboard_state.a = false;
                    println!("Key up A");
                },
                Event::KeyUp { keycode: Some(Keycode::S), repeat: false, .. } => {
                    self.keyboard_state.s = false;
                    println!("Key up S");
                },
                Event::KeyUp { keycode: Some(Keycode::D), repeat: false, .. } => {
                    self.keyboard_state.d = false;
                    println!("Key up D");
                },
                Event::KeyUp{ keycode: Some(Keycode::Escape), repeat: false, .. } => {
                    // world.key_d();
                    println!("Key down esc");
                    self.keyboard_state.esc = false;
                },
                /*
                Event::MouseButtonDown { x, y, mouse_btn: MouseButton::Left, .. } => {
                },
                */
                _ => {}
            }
        }
        return self.keyboard_state;
    }
}