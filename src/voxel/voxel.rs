use std::cmp::PartialEq;
use std::default::Default;

use crate::voxel::octree::Octree;

#[derive(Default, PartialEq)]
pub struct VoxelData {
    material: u8,
}

pub type VoxelOctree = Octree<VoxelData>;
