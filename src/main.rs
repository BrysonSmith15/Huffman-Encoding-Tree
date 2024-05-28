use core::mem::size_of_val;
use math::round::ceil;
use std::{collections::HashMap, iter::zip};

use crate::binary_tree::Node;

mod binary_tree;
mod huffman_tree;

fn tree_test() {
    /*
        /*
         *       5
         *     /  \
         *    /    \
         *   3      7
         *  / \    / \
         * 2   4  6   8
         * */
        let mut root = binary_tree::Node::new(Some(5));
        /*
         *   3
         *  / \
         * 2   4
         */
        root.set_left(Some(3));
        root.lef.as_mut().unwrap().set_left(Some(2));
        root.lef.as_mut().unwrap().set_right(Some(4));

        /*
         *   7
         *  / \
         * 6   8
         */
        root.set_right(Some(7));
        root.rig.as_mut().unwrap().set_left(Some(6));
        root.rig.as_mut().unwrap().set_right(Some(8));

        root.print(); //                       3, 5, 7
        root.lef.as_ref().unwrap().print(); // 2, 3, 4
        root.rig.as_ref().unwrap().print(); // 6, 7, 8

        // prints the tree on its side
        root.tree_print(None);
        root.set_by_id(&[1, 1, 1, 0], Some(9)).expect("Failed");
        println!("----------------------------------------------------");
        root.tree_print(None);
        root.set_by_id(&[1, 1, 1, 1], Some(10)).expect("Failed");
        println!("----------------------------------------------------");
        root.tree_print(None);
        println!("----------------------------------------------------");
        println!("{}", root.get_by_id(&[1, 1, 1, 1]).unwrap().unwrap());
    */
}

/*
fn manual_huffman() -> Node<(u32, Option<char>)> {
        let text = "BCAADDDCCACACAC";
        let mut freqs: HashMap<char, u32> = HashMap::new();
        for chr in text.chars() {
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
        let tree = node_queue.swap_remove(0);
        return tree;
}
    */

fn main() {
    let my_huffman = huffman_tree::HuffmanTree::new("
Lorem ipsum dolor sit amet consectetur adipiscing elit feugiat neque integer tortor sociis laoreet netus, dictumst nullam tincidunt potenti tristique orci duis mollis ridiculus odio egestas primis curae. Mauris sociis orci cras malesuada dictum ridiculus augue quis sodales, nec faucibus netus enim aliquet taciti dictumst congue eget ad, iaculis dui dis conubia class fermentum suspendisse placerat. Curae torquent sollicitudin lacinia urna dis penatibus iaculis ullamcorper arcu non maecenas quam congue, habitasse feugiat libero laoreet sagittis pellentesque nulla fringilla velit odio malesuada ridiculus.Vel iaculis viverra est senectus sollicitudin nisl tincidunt cubilia mus nascetur, rutrum ad convallis montes posuere dignissim malesuada penatibus vitae, fermentum inceptos fames erat elementum turpis nullam feugiat varius. Imperdiet eros senectus vestibulum urna tortor purus feugiat, blandit fames habitant vivamus mi arcu magna est, convallis sociis id morbi mus malesuada. Felis posuere tellus dictum non sagittis semper nascetur, arcu per tristique mauris dictumst mus feugiat neque, penatibus sem tincidunt velit facilisis pellentesque.");
    my_huffman.tree.tree_print(None);
    println!("---------------");
    // test that the tree is generated correctly
    //assert!(my_huffman.tree == manual_huffman());

    // test the map
    let m = my_huffman.get_map();
    for (key, val) in &m {
        println!("{:?}={:?}", key, val);
    }
    for key in m.keys() {
        println!("{}", key);
        assert_eq!(
            my_huffman
                .tree
                .get_by_id(m.get(key).unwrap().to_vec())
                .unwrap()
                .unwrap()
                .1
                .unwrap(),
            *key
        );
    }

    // test encoder
    let input = "lorem";

    //assert!(
    //    out.unwrap()
    //        == vec![
    //            1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0
    //        ]
    //);
    // test that it returns an err if stuff is bad

    let mut long_input = String::from("");
    for _ in 0..100000 {
        long_input.push_str(&input);
    }
    println!("{}", input);

    let out = &mut my_huffman.encode(&input).unwrap();
    let bin_out = my_huffman.vu8_to_bitvec(out.to_vec()).unwrap();
    //let out_decode = my_huffman.decode(out);
    //assert_eq!(input, out_decode);
    //println!("{}\n{}", input, out_decode);
    //println!("{:?}", out);
    println!(
        "All in Bytes\nOriginal Len:\t{}\nVec Len:\t{}\nBinaryLen:\t{}",
        input.len(),
        out.len(), // pretty close
        bin_out.len() / 8
    );
    println!("{:#?}", bin_out);
}
