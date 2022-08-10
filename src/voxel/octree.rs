use std::borrow::{Borrow, BorrowMut};
use std::cmp::PartialEq;
use std::default::Default;
use std::ops::Index;
use Vec;

pub enum AddChildError {
    AlreadyAdded,
    OutOfBounds,
}

const EMPTY_NODE: u32 = u32::MAX;

#[derive(Default)]
pub struct Octree<T> where T: Default + PartialEq {
    nodes: Vec<Node>,
    data: Vec<T>,
    root: u32,
}

#[derive(Default)]
pub struct Node {
    index: u32,
    children: [u32; 8],
    data: u32,
}


impl<T> Octree<T> where T: Default + PartialEq {
    pub fn new() -> Self { Self::default() }

    pub fn get_node(&mut self, i: usize) -> &Node {
        &self.nodes[i]
    }

    pub fn get_node_mut(&mut self, i: usize) -> &mut Node {
        &mut self.nodes[i]
    }

    pub fn get_root(&mut self) -> &Node {
        &self.nodes[self.root as usize]
    }

    pub fn get_root_mut(&mut self) -> &mut Node {
        &mut self.nodes[self.root as usize]
    }

    pub fn add_node(&mut self, node: &mut Node, i: usize) -> Result<&mut Self, AddChildError> {
        if (node.children[i] != EMPTY_NODE) { Err(AddChildError::AlreadyAdded) } else if (i >= 8) { Err(AddChildError::OutOfBounds) } else {
            let mut newNode = Node::new();
            let index = self.nodes.len() as u32;
            newNode.index = index;
            node.children[i] = index;
            self.nodes.push(newNode);
            Ok(self)
        }
    }

    pub fn add_data(&mut self, node: &mut Node, data: T) {
        match self.data.iter().position(|x| x == &data) {
            Some(index) => { node.data = index as u32; }
            None => {
                self.data.push(data);
                node.index = (self.data.len() - 1) as u32;
            }
        }
    }
}

impl Node {
    pub fn new() -> Self { Self::default() }
}

/*
impl<T> Node<T> where T: Default {
    pub fn new() -> Self { Self::default() }

    pub fn add_node(&mut self, i: usize) -> Result<&mut Self, AddChildError> {
        if i >= 8 { Err(AddChildError::OutOfBounds) } else if self.children[i] != EMPTY_NODE { Err(AddChildError::AlreadyAdded) } else {
            self.children[i] =
                Ok(self)
        }
    }

    pub fn set_data(&mut self, value: T) {
        self.data = Some(value);
    }
    pub fn remove_node(&mut self, i: usize) -> Option<Self> {
        if self.children.get(i).is_none() { None } else { self.children[i].take().map(|c| *c) }
    }
    pub fn get_node(&mut self, i: usize) -> Option<&Self> {
        self.children[i].as_ref().map(AsRef::as_ref)
    }

    pub fn get_node_mut(&mut self, i: usize) -> Option<&mut Self> {
        self.children[i].as_mut().map(AsMut::as_mut)
    }

    pub fn get_data(&mut self) -> Option<&T> {
        self.data.borrow().as_ref()
    }

    pub fn get_data_mut(&mut self) -> Option<&mut T> {
        self.data.borrow_mut().as_mut()
    }
}

impl<T> From<Octree<T>> for Option<Box<Octree<T>>> where T: Default {
    fn from(tree: Octree<T>) -> Self {
        Some(Box::new(tree))
    }
}
 */
