extern crate pollster;
use std::borrow::Cow;
use FrostyEngine::{
    render::window,
};

mod moving_box;

fn main(){
    pollster::block_on( moving_box::moving_box_example() );
}
