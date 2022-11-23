use winit::event::{VirtualKeyCode, WindowEvent, KeyboardInput, ElementState };
use hashbrown::HashMap;

use crate::ecs::Component;

// a struct connecting a key event to a game event
pub struct KeyBinding{
    key: VirtualKeyCode,
    pressed: bool
}

impl KeyBinding{
    pub fn new(key: VirtualKeyCode) -> Self{
        Self{
            key: key,
            pressed: false
        }
    }
}

// an object that keeps track of user input and resets at each frame end
// follows mouse, key presses, and allows for specific key bindings
// such as an "up" button
pub struct InputHandler{
    // basic key bindings
    key_binds: HashMap<String, KeyBinding>,
    /*E
    actions_to_keys: HashMap<String, VirtualKeyCode>,
    key_states: HashMap<VirtualKeyCode, bool>
    */
}

impl InputHandler{
    pub fn new_default() -> Self{
        let mut key_binds: HashMap<String, KeyBinding> = HashMap::new();
        key_binds.insert("up".into(), KeyBinding::new(VirtualKeyCode::W));
        key_binds.insert("down".into(), KeyBinding::new(VirtualKeyCode::S));
        key_binds.insert("left".into(), KeyBinding::new(VirtualKeyCode::A));
        key_binds.insert("right".into(), KeyBinding::new(VirtualKeyCode::D));
        key_binds.insert("jump".into(), KeyBinding::new(VirtualKeyCode::Space));
        key_binds.insert("crouch".into(), KeyBinding::new(VirtualKeyCode::LShift));
        key_binds.insert("action1".into(), KeyBinding::new(VirtualKeyCode::E));
        key_binds.insert("action2".into(), KeyBinding::new(VirtualKeyCode::R));
        key_binds.insert("action3".into(), KeyBinding::new(VirtualKeyCode::T));
        key_binds.insert("action4".into(), KeyBinding::new(VirtualKeyCode::C));
        key_binds.insert("action5".into(), KeyBinding::new(VirtualKeyCode::V));
        Self{ key_binds }
    }

    // handle key downs and mouse move events
    // returns a bool as to whether the input was handled or not
    pub fn recieve_input(&self, event: &WindowEvent) -> bool{
        match event{ // return from match is return from method
            // if this iterates over each key binding then keyboard input will
            // have O(cn) where c is the number of bindings and n is the number of inputs
            WindowEvent::KeyboardInput{ input: KeyboardInput{ virtual_keycode, state, .. }, .. } => {
                // create a macro to run this command for each key binding {action}:
                true
            },
            _ => { false } // nothing norworthy happened
        }
    }

    // get a list of all registered key actions
    pub fn get_key_action_names(&mut self) -> Vec<String>{
        let mut return_vec: Vec<String> = Vec::new();
        for (name, key)  in &mut self.key_binds.iter(){
            return_vec.push(name.to_string()); // use .to_string to drop &
        }
        return_vec
    }

    // check the state of a key action
    // returns true if pressed,
    // false if not,
    // None if action is unrecognized
    pub fn get_key_action(&mut self, action: String) -> Option<bool>{
        let result = self.key_binds.get(&action);
        match result{
            Some(key_bind) => return Some(key_bind.pressed),
            None => return None,
        }
    }

    // same logic as get_key_action, but takes multiple actions
    // true = key pressed
    // false = key not pressed
    // None = unrecognized command
    pub fn get_key_actions<I: Iterator<Item=String>>(&mut self, actions: I) -> Vec<Option<bool>>{
        let mut return_vec: Vec<Option<bool>> = Vec::new();
        for action in actions{
            match self.key_binds.get(&action){
                // add to vec instead of returning each action
                Some(key_bind) =>  return_vec.push(Some(key_bind.pressed)),
                None => return_vec.push(None)
            }
        }
        return_vec
    }
}

// A component that has capabilities based on input commands
pub trait InputComponent: Component{
    fn handle_input(&self, inputs: &InputHandler);
}