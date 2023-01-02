use winit::event::{VirtualKeyCode, WindowEvent, KeyboardInput, ElementState, MouseButton };
use hashbrown::HashMap;

use crate::ecs::Component;
use crate::util::VIRTUALKEYS;

fn create_keyboard_hash_map() -> HashMap<VirtualKeyCode, bool>{
    // constructs a hashmap with each key press as an index
    let mut keys: HashMap<VirtualKeyCode, bool> = HashMap::new();
    for k in &VIRTUALKEYS{
        keys.insert(*k, false);
    }
    keys
}

// an object that keeps track of user input and resets at each frame end
// follows mouse, key presses, and allows for specific key bindings
// such as an "up" button
pub struct InputHandler{
    // basic key bindings
    actions_to_keys: HashMap<String, VirtualKeyCode>,
    key_states: HashMap<VirtualKeyCode, bool>,
    mouse_states: HashMap<MouseButton, bool>
}

impl InputHandler{
    pub fn new_default() -> Self{
        let mut actions_to_keys: HashMap<String, VirtualKeyCode> = HashMap::new();
        actions_to_keys.insert("up".into(), VirtualKeyCode::W);
        actions_to_keys.insert("down".into(), VirtualKeyCode::S);
        actions_to_keys.insert("left".into(), VirtualKeyCode::A);
        actions_to_keys.insert("right".into(), VirtualKeyCode::D);
        actions_to_keys.insert("jump".into(), VirtualKeyCode::Space);
        actions_to_keys.insert("crouch".into(), VirtualKeyCode::LShift);
        actions_to_keys.insert("action1".into(), VirtualKeyCode::E);
        actions_to_keys.insert("action2".into(), VirtualKeyCode::R);
        actions_to_keys.insert("action3".into(), VirtualKeyCode::T);
        actions_to_keys.insert("action4".into(), VirtualKeyCode::C);
        actions_to_keys.insert("action5".into(), VirtualKeyCode::V);
        
        let mut mouse_states: HashMap<MouseButton, bool> = HashMap::new();
        mouse_states.insert(MouseButton::Left, false);
        mouse_states.insert(MouseButton::Right, false);
        mouse_states.insert(MouseButton::Middle, false);

        Self{ 
            actions_to_keys: actions_to_keys,
            key_states: create_keyboard_hash_map(),
            mouse_states
         }
    }

    // handle key downs and mouse move events
    // only accpets winit window events
    // returns a bool as to whether the input was handled or not
    pub fn recieve_window_input(&mut self, event: &WindowEvent) -> bool{
        match event{ // return from match is return from method
            // if this iterates over each key binding then keyboard input will
            // have O(cn) where c is the number of bindings and n is the number of inputs
            WindowEvent::KeyboardInput{ input: KeyboardInput{ virtual_keycode, state, .. }, .. } => {
                // A key being true means that it is being pressed
                self.key_states.insert(virtual_keycode.unwrap(), *state==ElementState::Pressed);
                true
            },
            WindowEvent::MouseInput { state, button, .. } => {
                // A button is true if it is being pressed
                self.mouse_states.insert(*button, *state == ElementState::Pressed);
                true
            }
            _ => { false } // nothing norworthy happened
        }
    }

    // handle mouse motion
    // handles winit::Event events
    pub fn recieve_general_events(&mut self){

    }

    // get a list of all registered key actions
    pub fn get_key_action_names(&mut self) -> Vec<String>{
        let mut return_vec: Vec<String> = Vec::new();
        for (name, _)  in &mut self.actions_to_keys.iter(){
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
            None => { None },
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

    // return the state of a specfic mouse button
    // true -> it is pressed
    // false -> it isn't
    pub fn get_mouse_action(&mut self, mouse_button: MouseButton) -> bool{
        match mouse_button{
            // engine currently doesn't support MouseButton::Other inputs
            MouseButton::Other(_) => { return false },
            _ => { return *self.mouse_states.get(&mouse_button).unwrap() }
        }
    }

    // same as get_mouse_action, but accepts multiple arguments at once
    pub fn get_mouse_actions<I: Iterator<Item=MouseButton>>(&mut self, mouse_buttons: I) -> Vec<bool>{
        let mut return_vec: Vec<bool> = Vec::new();
        for btn in mouse_buttons{
            match btn {
                // MouseButton::Other is currently unsupported
                MouseButton::Other(_) => { return_vec.push(false) },
                _ => { return_vec.push(*self.mouse_states.get(&btn).unwrap()) }
            }
        }
        return_vec
    }
}

// A component that has capabilities based on input commands
pub trait InputComponent: Component{
    fn handle_input(&self, inputs: &InputHandler);
}