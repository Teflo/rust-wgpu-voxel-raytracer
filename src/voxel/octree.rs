use std::{
    cmp::PartialEq,
    default::Default,
    fmt::Debug,
};
use Vec;

#[derive(Debug)]
pub enum AddChildError {
    AlreadyAdded,
    OutOfBounds,
    NodeNotExisting,
}

#[derive(Default)]
pub struct Octree<T> where T: Default + PartialEq {
    nodes: Vec<Node>,
    data: Vec<T>,
    pub root: u32,
}

#[derive(Default)]
pub struct Node {
    children: [Option<u32>; 8],
    data: Option<u32>,
}


impl<T> Octree<T> where T: Default + PartialEq {
    pub fn new() -> Self {
        let mut new = Self::default();
        new.add_root();
        new
    }

    fn add_root(&mut self) {
        self.nodes.push(Node::new());
        self.root = self.nodes.len() as u32 - 1;
    }

    pub fn get_node(&mut self, i: usize) -> &Node {
        &self.nodes[i]
    }

    pub fn get_node_mut(&mut self, i: usize) -> &mut Node {
        &mut self.nodes[i]
    }

    pub fn get_child(&mut self, node_index: u32, i: usize) -> &Node { self.get_node(self.nodes[node_index as usize].children[i].unwrap() as usize) }

    pub fn get_root(&mut self) -> &Node {
        &self.nodes[self.root as usize]
    }

    pub fn get_root_mut(&mut self) -> &mut Node {
        &mut self.nodes[self.root as usize]
    }

    pub fn add_node(&mut self, node_index: u32, i: usize) -> Result<u32, AddChildError> {
        if i >= 8 { return Err(AddChildError::OutOfBounds); }
        let node = &mut self.nodes[node_index as usize];
        let mut child = node.children[i];
        match child {
            Some(_) => Err(AddChildError::AlreadyAdded),
            None => {
                let mut new_node = Node::new();
                let index = self.nodes.len() as u32;
                node.children[i].map(|_| index);
                self.nodes.push(new_node);
                Ok(index)
            }
        }
    }

    pub fn add_data(&mut self, node: &mut Node, data: T) {
        match self.data.iter().position(|x| x == &data) {
            Some(index) => { node.data = Some(index as u32); }
            None => {
                self.data.push(data);
                node.data = Some((self.data.len() - 1) as u32);
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
