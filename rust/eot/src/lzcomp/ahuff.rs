use crate::{core::Error, lzcomp::bitio::*};

#[derive(Copy, Clone)]
struct Node {
    up: usize,
    left: usize,
    right: usize,
    code: i16,
    weight: i64,
}

impl Node {
    const ZEROED: Node = Node {
        up: 0,
        left: 0,
        right: 0,
        code: 0,
        weight: 0,
    };
}

const ROOT: usize = 1;

#[derive(Clone)]
pub struct Ahuff {
    tree: Vec<Node>,
    symbol_index: Vec<usize>,
    bit_count: i64,
    bit_count2: i64,
    range: i64,
    max_symbol: i32,
    count_a: i64,
    count_b: i64,
    sym_count: i64,
}

impl Ahuff {
    pub const PLACEHOLDER: Ahuff = Ahuff {
        tree: Vec::new(),
        symbol_index: Vec::new(),
        bit_count: 0,
        bit_count2: 0,
        range: 0,
        max_symbol: 0,
        count_a: 0,
        count_b: 0,
        sym_count: 0,
    };

    pub fn new(mut range_in: i16) -> Ahuff {
        let mut t = Ahuff::PLACEHOLDER;

        let range = range_in;
        t.range = range_in as i64;
        t.bit_count = bits_used((range_in - 1) as _);
        t.bit_count2 = 0_i64;
        if range_in > 256 && range_in < 512 {
            range_in -= 256;
            t.bit_count2 = bits_used((range_in - 1) as _) + 1;
        }
        t.max_symbol = (range - 1) as _;
        t.sym_count = 0;
        t.count_b = 100;
        t.count_a = t.count_b;
        t.symbol_index.resize(range as _, 0);
        t.tree.resize(2 * range as usize, Node::ZEROED);

        // Initialize the Huffman tree
        let limit = 2 * range as usize;
        for i in 2..limit {
            t.tree[i].up = i / 2;
            t.tree[i].weight = 1;
        }
        for i in 1..(range as usize) {
            t.tree[i].left = 2 * i;
            t.tree[i].right = 2 * i + 1;
        }
        for i in 0..(range as usize) {
            t.tree[i].code = -1;
            t.tree[(range as usize) + i].code = i as i16;
            t.tree[(range as usize) + i].left = usize::MAX;
            t.tree[(range as usize) + i].right = usize::MAX;
            t.symbol_index[i] = range as usize + i;
        }

        t.init_weight(ROOT as _);

        if t.bit_count2 != 0 {
            let wa = t.symbol_index[256];
            t.update_weight(wa);
            let wb = t.symbol_index[257];
            t.update_weight(wb);
            assert!(258 < range);
            for _ in 0..12 {
                let wx = t.symbol_index[(range - 3) as usize];
                t.update_weight(wx);
            }
            for _ in 0..6 {
                let wx = t.symbol_index[(range - 2) as usize];
                t.update_weight(wx);
            }
        } else {
            for _ in 0..2 {
                for i in 0..(range as usize) {
                    let wx = t.symbol_index[i];
                    t.update_weight(wx);
                }
            }
        }

        t.count_b = 0;
        t.count_a = t.count_b;
        t
    }

    pub fn read_symbol(&mut self, bio: &mut BITIO) -> Result<i16, Error> {
        let mut a = ROOT;
        let mut symbol;
        loop {
            let n = &self.tree[a];
            let bit = bio.input_bit()?;
            a = if bit != 0 { n.right } else { n.left };
            symbol = self.tree[a].code;
            if symbol >= 0 {
                break;
            }
        }
        self.update_weight(a);
        Ok(symbol)
    }

    // Swaps the nodes a and b
    fn swap_nodes(&mut self, a: usize, b: usize) {
        assert!(a != b);
        assert!(a > ROOT);
        assert!(b > ROOT);
        assert!((a as i64) < 2 * self.range);
        assert!((b as i64) < 2 * self.range);
        // assert!(tree[a].code < 0 || self.symbol_index[tree[a].code] == a);
        // assert!(tree[b].code < 0 || self.symbol_index[tree[b].code] == b);

        let upa = self.tree[a].up;
        let upb = self.tree[b].up;

        assert!(self.tree[upa].code < 0);
        assert!(self.tree[upb].code < 0);
        assert!(self.tree[upa].left == a || self.tree[upa].right == a);
        assert!(self.tree[upb].left == b || self.tree[upb].right == b);
        assert!(self.tree[a].weight == self.tree[b].weight);

        self.tree.swap(a, b);
        self.tree[a].up = upa;
        self.tree[b].up = upb;

        let code = self.tree[a].code;
        if code < 0 {
            // Internal nodes have children
            let left = self.tree[a].left;
            let right = self.tree[a].right;
            self.tree[left].up = a;
            self.tree[right].up = a;
        } else {
            assert!((code as i64) < self.range);
            self.symbol_index[code as usize] = a;
        }

        let code = self.tree[b].code;
        if code < 0 {
            // Internal nodes have children
            let left = self.tree[b].left;
            let right = self.tree[b].right;
            self.tree[left].up = b;
            self.tree[right].up = b;
        } else {
            // assert(code < self->range);
            self.symbol_index[code as usize] = b;
        }

        assert!(self.tree[upa].left == a || self.tree[upa].right == a);
        assert!(self.tree[upb].left == b || self.tree[upb].right == b);
    }

    fn update_weight(&mut self, mut a: usize) {
        while a != ROOT {
            let mut weight_a = self.tree[a].weight;
            let mut b = a - 1;
            // This if statement prevents sibling rule violations
            assert!(self.tree[b].weight >= weight_a);
            if self.tree[b].weight == weight_a {
                loop {
                    b -= 1;
                    if self.tree[b].weight != weight_a {
                        break;
                    }
                }
                b += 1;
                // assert(b >= ROOT);
                if b > ROOT {
                    self.swap_nodes(a, b);
                    a = b;
                }
            }
            weight_a += 1;
            self.tree[a].weight = weight_a;
            a = self.tree[a].up;
        }
        assert_eq!(a, ROOT);
        self.tree[a].weight += 1;
        assert_eq!(
            self.tree[a].weight,
            self.tree[self.tree[a].left].weight + self.tree[self.tree[a].right].weight
        );
        // check_tree(); slooow
    }

    // Recursively sets the parent weight equal to the sum of the two chilren's
    // weights.
    fn init_weight(&mut self, a: usize) -> i64 {
        if self.tree[a].code < 0 {
            // Internal node
            self.tree[a].weight =
                self.init_weight(self.tree[a].left as _) + self.init_weight(self.tree[a].right as _);
        }
        self.tree[a].weight
    }
}

// Returns number of bits used in the positive number x
fn bits_used(x: i64) -> i64 {
    assert!(x >= 0);
    if x == 0 {
        // Deliberate
        return 1;
    }
    (32 - (x as u32).leading_zeros()) as _
}
