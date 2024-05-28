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

        // keep smallest elements at the end so removing them is less bad for performance when
        // popping
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

    pub fn get_map(&self) -> HashMap<char, Vec<u8>> {
        let mut map = HashMap::<char, Vec<u8>>::new();
        let mut stack: Vec<(&Node<(u32, Option<char>)>, Vec<u8>)> = vec![(&self.tree, vec![])];
        while let Some((next_node, p)) = stack.pop() {
            match &next_node.get_val() {
                Some((_, Some(c))) => {
                    map.insert(*c, p.clone());
                    ()
                }
                None => (),
                _ => (),
            };

            match &next_node.lef {
                Some(l) => stack.push((&**l, {
                    let mut pl = p.clone();
                    pl.extend(vec![0]);
                    pl
                })),
                None => (),
            };

            match &next_node.rig {
                Some(r) => stack.push((&**r, {
                    let mut pr = p.clone();
                    pr.extend(vec![1]);
                    pr
                })),
                None => (),
            }
        }

        map
    }

    pub fn encode(&self, text: &str) -> Result<Vec<u8>, String> {
        let m = self.get_map();
        let mut out: Vec<u8> = vec![];
        for chr in text.chars() {
            match m.get(&chr) {
                Some(c) => out.extend(c),
                None => return Err(String::from("A value in the text is not in the tree")),
            }
        }
        Ok(out)
    }

    pub fn decode(&self, mut encoded: Vec<u8>) -> String {
        let mut out: String = String::from("");
        let mut curr: Vec<u8> = vec![];
        let mut inverse_map = HashMap::new();
        for (key, val) in self.get_map().into_iter() {
            inverse_map.insert(val, key);
        }

        while let Some((_bit, tail)) = encoded.split_first() {
            match inverse_map.get(&curr) {
                Some(c) => {
                    out.push(*c);
                    curr = vec![];
                }
                None => (),
            };
            let a = &mut tail.to_vec();
            encoded = a.to_vec();
        }
        out
    }

    pub fn encode_to_u128(&self, text: &str) -> Result<u128, String> {
        let v = self.encode(text);
        let mut out = 0u128;
        if v.is_err() {
            return Err(v.err().unwrap());
        }
        for bit in v?.iter() {
            out <<= 1;
            match bit {
                1 => {
                    out += 1;
                }
                0 => (),
                _ => unreachable!("A binary id only has 1 or 0"),
            };
        }
        Ok(out)
    }
}
