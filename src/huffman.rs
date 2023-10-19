pub mod huffman {
    use rust_ds::heap::heap::BinaryHeap;
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::fmt;
    use std::fs;
    use std::path::Path;

    #[derive(Clone)]
    pub struct HuffmanNode {
        ch: Option<char>,
        f: u32,
        l: Option<Box<HuffmanNode>>,
        r: Option<Box<HuffmanNode>>,
    }

    impl HuffmanNode {
        pub fn single(ch: Option<char>, f: u32) -> Self {
            HuffmanNode {
                f,
                ch,
                l: None,
                r: None,
            }
        }
    }

    impl PartialOrd for HuffmanNode {
        fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
            Some(self.cmp(o))
        }
    }

    impl Ord for HuffmanNode {
        fn cmp(&self, o: &Self) -> Ordering {
            self.f.cmp(&o.f)
        }
    }

    impl PartialEq for HuffmanNode {
        fn eq(&self, o: &Self) -> bool {
            self.f == o.f && self.ch == o.ch && self.l == o.l && self.r == o.r
        }
    }

    impl Eq for HuffmanNode {}

    fn build_huffman_tree(content: &String) -> BinaryHeap<HuffmanNode> {
        // Logic:
        // (i) Count char frequencies and place these in a priority queue (BinaryHeap).
        // (ii) Construct a Huffman tree:
        // Properties:
        // Minimal external path weight - meaning smallest sum of paths for leaves.
        // This can be constructed by adding greedily the smallest node to a tree so that the leaf distance is minimized.

        let mut pq = content
            .chars()
            .fold(HashMap::new(), |mut acc, c| {
                let counter = acc.entry(c).or_insert(0 as u16);
                *counter += 1;
                acc
            })
            .iter()
            .fold(BinaryHeap::default(), |mut pq, (c, f)| {
                pq.insert(HuffmanNode::single(Some(*c), *f as u32));
                pq
            });

        // while left and right still in queue,
        while pq.size() > 1 {
            let l = pq.pop().unwrap();
            let r = pq.pop().unwrap();

            let p = HuffmanNode {
                f: l.f + r.f,
                ch: None, // dummy since won't be used
                l: Some(Box::new(l)),
                r: Some(Box::new(r)),
            };

            pq.insert(p);
        }

        pq
    }

    pub fn build_huffman_encoding(
        content: &String,
    ) -> Option<(HashMap<char, String>, BinaryHeap<HuffmanNode>)> {
        // Idea:
        // Recursively walk down the Huffman tree, add to encoding if leaf node.
        let mut pq = build_huffman_tree(content);

        if pq.size() == 0 {
            return None;
        }

        let pq_clone = pq.clone();

        let root = Box::new(pq.pop().unwrap());

        fn recursive_encoding(n: &Box<HuffmanNode>, e: &mut HashMap<char, String>, s: String) {
            if let Some(ch) = n.ch {
                e.insert(ch, s);
            } else {
                if let Some(ref l) = n.l {
                    recursive_encoding(l, e, s.clone() + "0");
                }
                if let Some(ref r) = n.r {
                    recursive_encoding(r, e, s.clone() + "1");
                }
            }
        }

        let mut encodings: HashMap<char, String> = HashMap::new();

        recursive_encoding(&root, &mut encodings, "".to_string());

        Some((encodings, pq_clone))
    }

    fn encode_string(input_content: &String) {
        let mut bits: usize;

        bits = input_content.len() * 8;

        println!("Uncompressed data size: {bits} bits.");

        println!("Building Huffman encoding..");
        let (encoding_map, huffman_tree) = build_huffman_encoding(&input_content).unwrap();

        let compressed_content = input_content.chars().fold(String::new(), |mut acc, c| {
            acc.push_str(encoding_map.get(&c).unwrap());
            acc
        });

        bits = compressed_content.len();
        println!("Compressed data size: {bits} bits.");
        println!("Compressed data:\n{compressed_content}");
    }

    pub fn encode(input_path: &String, output_path: &String) {
        let input_content = read_file(input_path);

        encode_string(&input_content);
    }

    pub fn decode(input_path: &String, output_path: &String) {
        todo!();
    }
    fn read_file(file_path: &String) -> String {
        fs::read_to_string(file_path).expect("Should have been able to read the file")
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        #[test]
        fn test_encode_decode() {}
    }
}
