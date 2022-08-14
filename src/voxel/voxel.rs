use std::{
    cmp::PartialEq,
    default::Default,
    fmt::Debug,
};

use crate::voxel::octree::Octree;

#[derive(Default, PartialEq, Debug)]
pub struct VoxelData {
    material: u8,
}

pub type VoxelOctree = Octree<VoxelData>;
