use std::fs;

mod binary_tree;
mod huffman_tree;

fn main() {
    let contents = fs::read_to_string("src/gpt_input.txt");
    let my_huffman = huffman_tree::HuffmanTree::new(&contents.unwrap());
    my_huffman.tree.tree_print(None);
    println!("---------------");
    // test that the tree is generated correctly
    //assert!(my_huffman.tree == manual_huffman());

    // test the map
    let m = my_huffman.get_map();
    for key in m.keys() {
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

    let test_strs = vec![
        "fox",
        "Lorem",
        "turpis",
        "Nullam",
        "efficitur",
        "convallis",
        "Vestibulum",
        "suscipit",
        "aliquam",
        "mauris",
        "ultrices",
        "ligula",
        "fringilla",
        "dapibus",
        "volutpat",
        "i",
    ];

    for test in test_strs {
        let mut long_input = String::from("");
        for _ in 0..100000 {
            long_input.push_str(test);
        }
        println!("{}", test);

        let bin_out = my_huffman.encode_to_bitvec(test).unwrap();
        println!(
            "All in Bytes\nOriginal Len:\t{}\nBinaryLen:\t{}",
            test.len(),
            bin_out.len() / 8
        );
        println!(
            "Big Stuff\nlong input len\t{}\nshortened len\t{}",
            long_input.len(),
            my_huffman.encode_to_bitvec(&long_input).unwrap().len() / 8
        );
        //println!("{:#?}", bin_out);
        println!("------------------------------------");
    }
}
