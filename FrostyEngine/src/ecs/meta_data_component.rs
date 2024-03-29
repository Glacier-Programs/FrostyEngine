use hashbrown::HashMap;
use std::any::TypeId;

use super::{Component, ComponentFlags, Entity};

// A component that holds the meta data required for Entity Function
// no builder since this should only be called by Entity
#[derive(core::fmt::Debug)]
pub struct MetaDataComponent{
    // maps a components UUID to its index in Entity.components
    pub(crate) component_indices: HashMap<TypeId, usize>,
    // indices of the components that are flagged as Updating
    pub(crate) updating_component_index: HashMap<TypeId, usize>,
    pub(crate) ephemeral_components: Vec<usize>,
    pub renderable_index: usize,
}

impl MetaDataComponent{
    pub fn get_renderability(&self) -> bool{
        self.renderable_index > 0usize
    }
}

impl Component for MetaDataComponent{
    fn get_flags(&self) -> Vec<ComponentFlags> { vec![ComponentFlags::Unflagged] }
    // these will be implemented as default implementation when #[derive(Component)] is implemented
    fn id() -> TypeId{ TypeId::of::<MetaDataComponent>() }
    fn get_type_id(&self) -> TypeId { TypeId::of::<MetaDataComponent>() }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_dyn_component(&self) -> &dyn Component { self }
}