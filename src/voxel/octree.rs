use std::{
    cmp::PartialEq,
    default::Default,
    fmt::{Debug, Display, Error, Formatter},
};
use std::option::IntoIter;
use std::ptr::write;
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
    pub children: [Option<u32>; 8],
    pub data: Option<u32>,
    pub index: u32,
    pub flat_index: i32,
}

impl Node {
    pub fn new() -> Self {
        Self {
            flat_index: -1,
            ..Self::default()
        }
    }

    pub fn to_byte(&self) -> u8 {
        let mut byte: u8 = 0;
        for i in 0..8 {
            match self.children[i] {
                None => {}
                Some(_) => byte |= 1 << i
            }
        }
        byte
    }
}


impl<T> Octree<T> where T: Default + PartialEq {
    pub fn new() -> Self {
        let mut new = Self::default();
        new.add_root();
        new
    }

    fn add_root(&mut self) {
        let mut new_node = Node::new();
        self.root = self.nodes.len() as u32;
        new_node.index = self.root;
        self.nodes.push(new_node);
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
                let mut new_node = Node::new();
                let index = self.nodes.len() as u32;
                new_node.index = index;
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

    pub fn iter(&self) -> OctreeIter<'_, T> {
        OctreeIter {
            tree: &self,
            stack: vec![self.root],
            depth_stack: vec![],
            depth_counter: 0,
        }
    }

    pub fn serialize(&mut self) -> Vec<u8> {
        let mut voxels = vec![];
        let mut unreferenced = vec![];
        for i in 0..self.nodes.len() {
            let node = &mut self.nodes[i];
            voxels.push(node.to_byte());
            if node.flat_index >= 0 {
                voxels[node.flat_index as usize] = (voxels.len() - 1) as u8;
            }
            for child in node.children {
                match child {
                    Some(c) => {
                        voxels.push(0);
                        let n = &mut self.nodes[c as usize];
                        n.flat_index = (voxels.len() - 1) as i32;
                        unreferenced.push(n);
                    }
                    _ => {}
                }
            }
        }

        return voxels;
    }
}

impl<T> Display for Octree<T> where T: Default + PartialEq {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.iter();
        let mut i = 0;
        while let Some(node) = iter.next() {
            let ws = "-".repeat(iter.depth_stack.len());
            //println!("{}", iter.depthStack.len());
            write!(f, "{:0}Node {:1}: {:0>8b}\n", ws, node.index, node.to_byte())?;
            i += 1;
        }
        Ok(())
    }
}

pub struct OctreeIter<'a, T> where T: Default + PartialEq {
    tree: &'a Octree<T>,
    stack: Vec<u32>,
    depth_stack: Vec<i32>,
    depth_counter: i32,
}


impl<'a, T> Iterator for OctreeIter<'a, T> where T: Default + PartialEq {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        fn red_and_pop(stack: &mut Vec<i32>) {
            let len = stack.len();
            if len > 0 {
                stack[len - 1] -= 1;
                if stack[len - 1] < 0 { stack.pop(); }
            }
        }

        if self.stack.is_empty() { return None; }

        //println!("pre - {:?}, {:?}", self.depth_stack, self.stack);
        if self.depth_counter > 0 {
            self.depth_stack.push(self.depth_counter + 1);
        } else {
            red_and_pop(&mut self.depth_stack);
        }

        red_and_pop(&mut self.depth_stack);


        match self.stack.pop() {
            None => {
                return None;
            }
            Some(index) => {
                let node = &self.tree.nodes[index as usize];
                self.depth_counter = 0;
                for i in (0..8).rev() {
                    match node.children[i] {
                        None => {}
                        Some(child) => {
                            self.stack.push(child as u32);
                            self.depth_counter += 1;
                        }
                    }
                }
                //println!("post - {:?}, {:?}", self.depth_stack, self.stack);
                Some(&node)
            }
        }
    }
}
