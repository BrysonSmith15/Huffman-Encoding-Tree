use core::iter::zip;
use std::collections::HashMap;

use crate::binary_tree;
use crate::binary_tree::Node;

pub struct HuffmanTree {
    pub tree: binary_tree::Node<(u32, Option<char>)>,
}

impl HuffmanTree {
    pub fn new(to_encode: &str) -> Self {
        let mut freqs: HashMap<char, u32> = HashMap::new();
        for chr in to_encode.chars() {
            let curr_freq = freqs.get(&chr).unwrap_or(&0);
            freqs.insert(chr, curr_freq + 1);
        }
        let mut queue: Vec<&char> = freqs.keys().collect();

        // keep smallest elements at the end so removing them is less bad for perf
        queue.sort_by(
            |c1, c2| match freqs.get(c1).unwrap() < freqs.get(c2).unwrap() {
                true => std::cmp::Ordering::Greater,
                false => std::cmp::Ordering::Less,
            },
        );

        let sorted_freqs = queue
            .iter()
            .map(|x| freqs.get(x).unwrap())
            .collect::<Vec<&u32>>();

        let mut node_queue: Vec<Node<(u32, Option<char>)>> = vec![];
        for (fq, chr) in zip(sorted_freqs, queue.clone()) {
            node_queue.push(Node::<(u32, Option<char>)>::new(Some((*fq, Some(*chr)))));
        }

        while let Some(l) = node_queue.pop() {
            let mut new_node: Node<(u32, Option<char>)> = Node::new(None);
            new_node.lef = Some(Box::new(l));
            let r = node_queue.pop();
            new_node.rig = Some(Box::new(r.unwrap()));

            new_node.set_val(Some((
                new_node.get_left_val().unwrap().0 + new_node.get_right_val().unwrap().0,
                None,
            )));
            node_queue.push(new_node);
            node_queue.sort_by(
                |n1, n2| match n1.get_val().unwrap() < n2.get_val().unwrap() {
                    true => std::cmp::Ordering::Greater,
                    false => std::cmp::Ordering::Less,
                },
            );
            if node_queue.len() <= 1 {
                break;
            }
        }
        HuffmanTree {
            tree: node_queue.swap_remove(0),
        }
    }
}