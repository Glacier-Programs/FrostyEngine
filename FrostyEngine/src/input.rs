use winit::event::{VirtualKeyCode, WindowEvent, KeyboardInput, ElementState };
use hashbrown::HashMap;

use crate::ecs::Component;

fn create_keyboard_hash_map() -> HashMap<VirtualKeyCode, bool>{
    // constructs a hashmap with each key press as an index
    let keys: HashMap<VirtualKeyCode, bool> = HashMap::new();
    todo!();
}

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
    //key_binds: HashMap<String, KeyBinding>,
    actions_to_keys: HashMap<String, VirtualKeyCode>,
    key_states: HashMap<VirtualKeyCode, bool>
}

impl InputHandler{
    pub fn new_default() -> Self{
        let actions_to_keys: HashMap<String, VirtualKeyCode> = HashMap::new();
        Self{ 
            actions_to_keys: HashMap::new(),
            key_states: create_keyboard_hash_map()
         }
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
        for (name, key)  in &mut self.actions_to_keys.iter(){
            return_vec.push(name.to_string()); // use .to_string to drop &
        }
        return_vec
    }

    // check the state of a key action
    // returns true if pressed,
    // false if not,
    // None if action is unrecognized
    pub fn get_key_action(&mut self, action: String) -> Option<bool>{
        let action_check = self.actions_to_keys.get(&action);
        let result = match action_check{
            // HashMap.get() returns Option<&T>, so have to drop internal borrow 
            // since key_states has each key, unwrap() will never panic
            // then has to be put back in an Option<> with Some()
            Some(key_code) => { Some(*self.key_states.get(key_code).unwrap()) },
            None => {None},
        };
        result
    }

    // same logic as get_key_action, but takes multiple actions
    // true = key pressed
    // false = key not pressed
    // None = unrecognized command
    pub fn get_key_actions<I: Iterator<Item=String>>(&mut self, actions: I) -> Vec<Option<bool>>{
        let mut return_vec: Vec<Option<bool>> = Vec::new();
        for action in actions{
            match self.actions_to_keys.get(&action){
                // add to vec instead of returning each action
                Some(key_code) =>  return_vec.push(Some( *self.key_states.get(key_code).unwrap() )),
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