use std::{
    cmp::PartialEq,
    default::Default,
    fmt::Debug,
};

use cgmath;
use cgmath::Vector4;

#[derive(PartialEq, Debug)]
pub struct VoxelData {
    color: Vector4<u8>,
    material: Material,
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum Material {
    #[default]
    Nothing = 0,
    Solid = 1,
}