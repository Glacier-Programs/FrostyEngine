pub mod util;
pub(crate) mod resource_manager;
#[macro_use]
pub mod render;
pub mod scene;
pub mod ecs;
pub mod rect;
pub mod sprite;
pub mod app;
pub mod input;
pub mod draw;

/* 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/
