extern crate pollster;
use uuid;
use std::{rc::Rc, cell::RefCell};
use FrostyEngine as fe;
use fe::ecs::Component;

mod moving_box;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct CompTest{
    value: i32,
    other_val: i32
}

impl Component for CompTest{
    fn check_required_components(&self, parent: &fe::ecs::Entity) { }
    fn get_flags(&self) -> Vec<fe::ecs::ComponentFlags> { vec![fe::ecs::ComponentFlags::Unflagged] }
    fn get_type_id(&self) -> uuid::Uuid { todo!(); }
    fn id() -> uuid::Uuid where Self: Sized { todo!(); }
}

//impl std::raw::TraitObject for CompTest{}

fn main(){
    // testing component downcasting
    // need to make two of the same comp
    let component = CompTest{ value: i32::MAX, other_val: 9999999 };
    let vtable_component: Rc<RefCell<dyn Component>> = Rc::new( RefCell::new( component ) );
    let detabled_component = unsafe{ fe::ecs::component::downcast_component::<CompTest>(&vtable_component) };
    assert_eq!(component, *detabled_component);

    pollster::block_on( moving_box::moving_box_example() );
}
