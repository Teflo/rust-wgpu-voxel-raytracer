use std::{
    cmp::PartialEq,
    default::Default,
    fmt::{Debug, Display, Error, Formatter},
};
use std::option::IntoIter;
use Vec;

#[derive(Debug)]
pub enum AddChildError {
    AlreadyAdded,
    OutOfBounds,
    NodeNotExisting,
}

#[derive(Default, Debug)]
pub struct Octree<T> where T: Default + PartialEq {
    nodes: Vec<Node>,
    data: Vec<T>,
    pub root: u32,
}

#[derive(Default, Debug)]
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

        let child = self.nodes[node_index as usize].children[i];
        match child {
            Some(_) => Err(AddChildError::AlreadyAdded),
            None => {
                let new_node = Node::new();
                let index = self.nodes.len() as u32;
                self.nodes.push(new_node);
                self.nodes[node_index as usize].children[i] = Some(index);
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

    pub fn traverse(&self) -> Vec<u32> {
        let mut result = Vec::new();
        let mut stack = Vec::new();
        stack.push(self.root);
        while !stack.is_empty() {
            let index = stack.pop();
            if index.is_none() { continue; }
            result.push(index.unwrap());
            let node = &self.nodes[index.unwrap() as usize];
            for i in (0..8).rev() {
                match node.children[i] {
                    None => {}
                    Some(child) => {
                        stack.push(child);
                    }
                }
            }
        }
        result
    }
}
/*
impl Display for Octree<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut display = String::new();
        let mut node_index = self.root;
        let mut node = self.nodes[node_index];
        while true {
            display.push_str("[");

            display.push_str("]");
        }
    }
}

impl<'a, T> IntoIterator for Octree<T> {
    type Item = &'a Node;
    type IntoIter = std::vec::IntoIter<&'a Node>;

    fn into_iter(self) -> Self::IntoIter {
        fn append<'a, T>(node : Node, v : &mut Vec<&'a Node>) {
            for i in 0..node.children.len()-1 {
                match node.children[i] {
                    Some(node_index) => {
                        let child = &'a self.nodes[node_index];
                        v.push(child);
                        v.extend(append())
                    },
                    None => {

                    }
                }
            }
        }

        let mut result = Vec::new();
        append(self, &mut result);
        result.into_iter()
    }
}
*/
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
