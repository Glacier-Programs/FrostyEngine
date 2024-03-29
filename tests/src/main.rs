extern crate pollster;

use std::{
    rc::Rc, 
    cell::RefCell,
    any::TypeId
};
use FrostyEngine as fe;
use fe::ecs::Component;

mod moving_box;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct CompTest{
    value: i32,
    other_val: i32
}

impl Component for CompTest{
    fn get_flags(&self) -> Vec<fe::ecs::ComponentFlags> { vec![fe::ecs::ComponentFlags::Unflagged] }
    fn get_type_id(&self) -> TypeId { todo!(); }
    fn id() -> TypeId where Self: Sized { todo!(); }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_dyn_component(&self) -> &dyn Component { self }
}

fn main(){
    // testing component downcasting
    // need to make two of the same comp
    let component = CompTest{ value: i32::MAX, other_val: 9999999 };
    let vtable_component= component.as_dyn_component();
    let detabled_component = unsafe{ fe::ecs::component::downcast_component::<CompTest>(vtable_component).unwrap() };
    assert_eq!(component, *detabled_component);


    pollster::block_on( moving_box::moving_box_example() );
}
