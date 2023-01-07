use hashbrown::HashMap;

use super::{Component, ComponentFlags, Entity};

// A component that holds the meta data required for Entity Function
#[derive(core::fmt::Debug)]
pub struct MetaDataComponent{
    // maps a components UUID to its index in Entity.components
    pub(crate) component_indices: HashMap<uuid::Uuid, usize>,
    // indices of the components that are flagged as Updating
    pub(crate) updating_component_indice: HashMap<uuid::Uuid, usize>,
    pub(crate) is_renderable: bool
}

impl MetaDataComponent{
    pub fn get_renderability(&self) -> bool{
        self.is_renderable
    }
}

impl Component for MetaDataComponent{
    fn check_required_components(&self, parent: &Entity) { /* No other components needed */ }
    fn get_flags(&self) -> Vec<ComponentFlags> { vec![ComponentFlags::Unflagged] }
    fn id() -> uuid::Uuid{todo!();}
    fn get_type_id(&self) -> uuid::Uuid {todo!();}
}