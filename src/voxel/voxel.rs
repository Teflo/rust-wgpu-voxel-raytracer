use std::{
    cmp::PartialEq,
    default::Default,
    fmt::Debug,
};

use cgmath;
use cgmath::Vector3;

use packed_struct::prelude::*;

#[derive(PackedStruct, Default, PartialEq, Debug)]
#[packed_struct(bit_numbering = "msb0")]
pub struct VoxelData {
    #[packed_field(bits = "0..24")]
    color: Vector3<f32>,
    #[packed_field(bits = "24..32", ty = "enum")]
    material: Material,
}

#[derive(PrimitiveEnum_u8, Default, Clone, Copy, Debug, PartialEq)]
pub enum Material {
    Nothing = 0,
    Solid = 1,
}