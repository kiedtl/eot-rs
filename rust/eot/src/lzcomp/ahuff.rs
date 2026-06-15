use crate::core::Error;
use crate::lzcomp::bitio::*;

#[derive(Copy, Clone)]
struct Node {
    up: usize,
    left: usize,
    right: usize,
    code: i16,
    weight: i64,
}

impl Node {
    const ZEROED: Node = Node { up: 0, left: 0, right: 0, code: 0, weight: 0 };
}

const ROOT: usize = 1;

#[derive(Clone)]
pub struct AHUFF {
    tree: Vec<Node>,
    symbolIndex: Vec<usize>,
    bitCount: i64,
    bitCount2: i64,
    range: i64,
    maxSymbol: i32,
    countA: i64,
    countB: i64,
    sym_count: i64,
}

impl AHUFF {
    pub const PLACEHOLDER: AHUFF = AHUFF {
        tree: Vec::new(),
        symbolIndex: Vec::new(),
        bitCount: 0,
        bitCount2: 0,
        range: 0,
        maxSymbol: 0,
        countA: 0,
        countB: 0,
        sym_count: 0,
    };

    pub fn new(mut rangeIn: i16) -> AHUFF {
        let mut t = AHUFF::PLACEHOLDER;

        let mut range =  rangeIn;
        t.range = rangeIn as i64;
        t.bitCount = bits_used((rangeIn - 1 ) as _);
        t.bitCount2 = 0 as i64;
        if rangeIn > 256 && rangeIn < 512 {
            rangeIn -= 256;
            t.bitCount2 = bits_used((rangeIn - 1) as _) + 1;
        }
        t.maxSymbol = (range - 1) as _;
        t.sym_count = 0;
        t.countB = 100;
        t.countA = t.countB;
        t.symbolIndex.resize(range as _, 0);
        t.tree.resize(2 * range as usize, Node::ZEROED);

        /* Initialize the Huffman tree */
        let mut limit = 2 * range as usize;
        for i in 2..limit {
            t.tree[i].up = i / 2;
            t.tree[i].weight = 1;
        }
        for i in 1..(range as usize) {
            t.tree[i].left = 2 * i ;
            t.tree[i].right = 2 * i + 1;
        }
        for i in 0..(range as usize) {
            t.tree[i].code = -1;
            t.tree[(range as usize) + i].code = i as i16;
            t.tree[(range as usize) + i].left = usize::MAX;
            t.tree[(range as usize) + i].right = usize::MAX;
            t.symbolIndex[i] = range as usize + i;
        }

        t.init_weight(ROOT as _);

        if t.bitCount2 != 0 {
            let wa = t.symbolIndex[256];
            t.update_weight(wa);
            let wb = t.symbolIndex[257];
            t.update_weight(wb);
            assert!(258 < range);
            for _ in 0..12 {
                let wx = t.symbolIndex[(range - 3) as usize];
                t.update_weight(wx);
            }
            for _ in 0..6 {
                let wx = t.symbolIndex[(range - 2) as usize];
                t.update_weight(wx);
            }
        } else {
            for _ in 0..2 {
                for i in 0..(range as usize) {
                    let wx = t.symbolIndex[i];
                    t.update_weight(wx);
                }
            }
        }

        t.countB = 0;
        t.countA = t.countB;
        return t;
    }

    pub fn read_symbol(&mut self, bio: &mut BITIO) -> Result<i16, Error> {
        let mut a = ROOT;
        let mut symbol = 0i16;
        loop {
            let n = &self.tree[a];
            let bit = unsafe { bio.input_bit()? };
            a = if bit != 0 { n.right } else { n.left };
            symbol = self.tree[a].code;
            if symbol >= 0 {
                break;
            }
        }
        self.update_weight(a);
        Ok(symbol)
    }

    /* Swaps the nodes a and b */
    fn swap_nodes(&mut self, a: usize, b: usize) {
        assert!(a != b);
        assert!(a > ROOT);
        assert!(b > ROOT);
        assert!((a as i64) < 2 * self.range);
        assert!((b as i64) < 2 * self.range);
        // assert!(tree[a].code < 0 || self.symbolIndex[tree[a].code] == a);
        // assert!(tree[b].code < 0 || self.symbolIndex[tree[b].code] == b);

        let upa = self.tree[a].up;
        let upb = self.tree[b].up;

        assert!(self.tree[upa].code < 0);
        assert!(self.tree[upb].code < 0);
        assert!(self.tree[upa].left == a || self.tree[upa].right == a);
        assert!(self.tree[upb].left == b || self.tree[upb].right == b);
        assert!(self.tree[a].weight == self.tree[b].weight);

        let tNode = self.tree[a];
        self.tree[a] = self.tree[b];
        self.tree[b] = tNode;
        self.tree[a].up = upa;
        self.tree[b].up = upb;

        let code = self.tree[a].code;
        if code < 0 {
            /* Internal nodes have children */
            let left = self.tree[a].left as usize;
            let right = self.tree[a].right as usize;
            self.tree[left].up = a;
            self.tree[right].up = a;
        } else {
            assert!((code as i64) < self.range);
            self.symbolIndex[code as usize] = a;
        }

        let code = self.tree[b].code;
        if code < 0 {
            /* Internal nodes have children */
            let left = self.tree[b].left as usize;
            let right = self.tree[b].right as usize;
            self.tree[left].up = b;
            self.tree[right].up = b;
        } else {
            //assert(code < self->range);
            self.symbolIndex[code as usize] = b;
        }

        assert!(self.tree[upa].left == a || self.tree[upa].right == a);
        assert!(self.tree[upb].left == b || self.tree[upb].right == b);
    }

    fn update_weight(&mut self, mut a: usize) {
        while a != ROOT {
            let mut weightA = self.tree[a as usize].weight;
            let mut b = a - 1;
            /* This if statement prevents sibling rule violations */
            assert!(self.tree[b].weight >= weightA);
            if self.tree[b as usize].weight == weightA {
                loop {
                    b -= 1;
                    if self.tree[b as usize].weight != weightA {
                        break;
                    }
                }
                b += 1;
                //assert(b >= ROOT);
                if b > ROOT {
                    self.swap_nodes(a, b);
                    a = b;
                }
            }
            weightA += 1;
            self.tree[a as usize].weight = weightA;
            a = self.tree[a as usize].up;
        }
        assert_eq!(a, ROOT);
        self.tree[a as usize].weight += 1;
        assert_eq!(
            self.tree[a].weight,
            self.tree[self.tree[a].left].weight + self.tree[self.tree[a].right].weight
        );
        /*check_tree(); slooow */
    }

    /* Recursively sets the parent weight equal to the sum of the two chilren's
     * weights. */
    fn init_weight(&mut self, a: usize) -> i64 {
        if self.tree[a as usize].code < 0 {
            /* Internal node */
            self.tree[a as usize].weight =
                self.init_weight(self.tree[a as usize].left as _) +
                self.init_weight(self.tree[a as usize].right as _);
        }
        return self.tree[a as usize].weight;
    }
}

// Returns number of bits used in the positive number x
fn bits_used(x: i64) -> i64 {
    assert!(x >= 0);
    if x == 0 { // Deliberate
        return 1;
    }
    (32 - (x as u32).leading_zeros()) as _
}
