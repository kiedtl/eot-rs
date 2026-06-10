extern "C" {
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn MTX_mem_malloc(
        t: *mut MTX_MemHandler,
        size: ::core::ffi::c_ulong,
    ) -> *mut ::core::ffi::c_void;
    fn MTX_mem_free(t: *mut MTX_MemHandler, deadObject: *mut ::core::ffi::c_void);
    fn MTX_BITIO_input_bit(t: *mut BITIO) -> ::core::ffi::c_short;
    fn MTX_BITIO_output_bit(t: *mut BITIO, bit: ::core::ffi::c_ulong);
}
pub type size_t = usize;
pub type __jmp_buf = [::core::ffi::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: ::core::ffi::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mem_struct {
    pub pointermem: *mut ::core::ffi::c_void,
    pub pointersize: ::core::ffi::c_long,
}
pub type MTX_MALLOCPTR = Option<
    unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void,
>;
pub type MTX_REALLOCPTR = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void,
>;
pub type MTX_FREEPTR = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MTX_MemHandler {
    pub mem_pointers: *mut mem_struct,
    pub mem_maxPointers: ::core::ffi::c_long,
    pub mem_numPointers: ::core::ffi::c_long,
    pub mem_numNewCalls: ::core::ffi::c_long,
    pub malloc: MTX_MALLOCPTR,
    pub realloc: MTX_REALLOCPTR,
    pub free: MTX_FREEPTR,
    pub env: jmp_buf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BITIO {
    pub mem_bytes: *mut ::core::ffi::c_uchar,
    pub mem_index: ::core::ffi::c_long,
    pub mem_size: ::core::ffi::c_long,
    pub input_bit_count: ::core::ffi::c_ushort,
    pub input_bit_buffer: ::core::ffi::c_ushort,
    pub bytes_in: ::core::ffi::c_long,
    pub output_bit_count: ::core::ffi::c_ushort,
    pub output_bit_buffer: ::core::ffi::c_ushort,
    pub bytes_out: ::core::ffi::c_long,
    pub ReadOrWrite: ::core::ffi::c_char,
    pub mem: *mut MTX_MemHandler,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nodeType {
    pub up: ::core::ffi::c_short,
    pub left: ::core::ffi::c_short,
    pub right: ::core::ffi::c_short,
    pub code: ::core::ffi::c_short,
    pub weight: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AHUFF {
    pub tree: *mut nodeType,
    pub symbolIndex: *mut ::core::ffi::c_short,
    pub bitCount: ::core::ffi::c_long,
    pub bitCount2: ::core::ffi::c_long,
    pub range: ::core::ffi::c_long,
    pub bio: *mut BITIO,
    pub mem: *mut MTX_MemHandler,
    pub maxSymbol: ::core::ffi::c_int,
    pub countA: ::core::ffi::c_long,
    pub countB: ::core::ffi::c_long,
    pub sym_count: ::core::ffi::c_long,
}
#[no_mangle]
pub unsafe extern "C" fn MTX_AHUFF_BitsUsed(
    mut x: ::core::ffi::c_long,
) -> ::core::ffi::c_long {
    let mut n: ::core::ffi::c_long = 0;
    if x >= 0 as ::core::ffi::c_long {} else {
        __assert_fail(
            b"x >= 0\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/lzcomp/ahuff.c\0" as *const u8 as *const ::core::ffi::c_char,
            22 as ::core::ffi::c_uint,
            b"long MTX_AHUFF_BitsUsed(long)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    if x & 0xffff0000 as ::core::ffi::c_uint as ::core::ffi::c_long != 0 {
        if x & 0xff000000 as ::core::ffi::c_uint as ::core::ffi::c_long != 0 {
            if x & 0xf0000000 as ::core::ffi::c_uint as ::core::ffi::c_long != 0 {
                if x & 0xc0000000 as ::core::ffi::c_uint as ::core::ffi::c_long != 0 {
                    n = (if x & 0x80000000 as ::core::ffi::c_uint as ::core::ffi::c_long
                        != 0
                    {
                        32 as ::core::ffi::c_int
                    } else {
                        31 as ::core::ffi::c_int
                    }) as ::core::ffi::c_long;
                } else {
                    n = (if x & 0x20000000 as ::core::ffi::c_long != 0 {
                        30 as ::core::ffi::c_int
                    } else {
                        29 as ::core::ffi::c_int
                    }) as ::core::ffi::c_long;
                }
            } else if x & 0xc000000 as ::core::ffi::c_long != 0 {
                n = (if x & 0x8000000 as ::core::ffi::c_long != 0 {
                    28 as ::core::ffi::c_int
                } else {
                    27 as ::core::ffi::c_int
                }) as ::core::ffi::c_long;
            } else {
                n = (if x & 0x2000000 as ::core::ffi::c_long != 0 {
                    26 as ::core::ffi::c_int
                } else {
                    25 as ::core::ffi::c_int
                }) as ::core::ffi::c_long;
            }
        } else if x & 0xf00000 as ::core::ffi::c_long != 0 {
            if x & 0xc00000 as ::core::ffi::c_long != 0 {
                n = (if x & 0x800000 as ::core::ffi::c_long != 0 {
                    24 as ::core::ffi::c_int
                } else {
                    23 as ::core::ffi::c_int
                }) as ::core::ffi::c_long;
            } else {
                n = (if x & 0x200000 as ::core::ffi::c_long != 0 {
                    22 as ::core::ffi::c_int
                } else {
                    21 as ::core::ffi::c_int
                }) as ::core::ffi::c_long;
            }
        } else if x & 0xc0000 as ::core::ffi::c_long != 0 {
            n = (if x & 0x80000 as ::core::ffi::c_long != 0 {
                20 as ::core::ffi::c_int
            } else {
                19 as ::core::ffi::c_int
            }) as ::core::ffi::c_long;
        } else {
            n = (if x & 0x20000 as ::core::ffi::c_long != 0 {
                18 as ::core::ffi::c_int
            } else {
                17 as ::core::ffi::c_int
            }) as ::core::ffi::c_long;
        }
    } else if x & 0xff00 as ::core::ffi::c_long != 0 {
        if x & 0xf000 as ::core::ffi::c_long != 0 {
            if x & 0xc000 as ::core::ffi::c_long != 0 {
                n = (if x & 0x8000 as ::core::ffi::c_long != 0 {
                    16 as ::core::ffi::c_int
                } else {
                    15 as ::core::ffi::c_int
                }) as ::core::ffi::c_long;
            } else {
                n = (if x & 0x2000 as ::core::ffi::c_long != 0 {
                    14 as ::core::ffi::c_int
                } else {
                    13 as ::core::ffi::c_int
                }) as ::core::ffi::c_long;
            }
        } else if x & 0xc00 as ::core::ffi::c_long != 0 {
            n = (if x & 0x800 as ::core::ffi::c_long != 0 {
                12 as ::core::ffi::c_int
            } else {
                11 as ::core::ffi::c_int
            }) as ::core::ffi::c_long;
        } else {
            n = (if x & 0x200 as ::core::ffi::c_long != 0 {
                10 as ::core::ffi::c_int
            } else {
                9 as ::core::ffi::c_int
            }) as ::core::ffi::c_long;
        }
    } else if x & 0xf0 as ::core::ffi::c_long != 0 {
        if x & 0xc0 as ::core::ffi::c_long != 0 {
            n = (if x & 0x80 as ::core::ffi::c_long != 0 {
                8 as ::core::ffi::c_int
            } else {
                7 as ::core::ffi::c_int
            }) as ::core::ffi::c_long;
        } else {
            n = (if x & 0x20 as ::core::ffi::c_long != 0 {
                6 as ::core::ffi::c_int
            } else {
                5 as ::core::ffi::c_int
            }) as ::core::ffi::c_long;
        }
    } else if x & 0xc as ::core::ffi::c_long != 0 {
        n = (if x & 0x8 as ::core::ffi::c_long != 0 {
            4 as ::core::ffi::c_int
        } else {
            3 as ::core::ffi::c_int
        }) as ::core::ffi::c_long;
    } else {
        n = (if x & 0x2 as ::core::ffi::c_long != 0 {
            2 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) as ::core::ffi::c_long;
    }
    return n;
}
unsafe extern "C" fn SwapNodes(
    mut t: *mut AHUFF,
    mut a: ::core::ffi::c_short,
    mut b: ::core::ffi::c_short,
) {
    
    
    
    
    let mut tree: *mut nodeType = (*t).tree;
    let ROOT: ::core::ffi::c_short = 1 as ::core::ffi::c_short;
    if a as ::core::ffi::c_int != b as ::core::ffi::c_int {} else {
        __assert_fail(
            b"a != b\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/lzcomp/ahuff.c\0" as *const u8 as *const ::core::ffi::c_char,
            197 as ::core::ffi::c_uint,
            b"void SwapNodes(AHUFF *, short, short)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    if a as ::core::ffi::c_int > ROOT as ::core::ffi::c_int {} else {
        __assert_fail(
            b"a > ROOT\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/lzcomp/ahuff.c\0" as *const u8 as *const ::core::ffi::c_char,
            198 as ::core::ffi::c_uint,
            b"void SwapNodes(AHUFF *, short, short)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    if b as ::core::ffi::c_int > ROOT as ::core::ffi::c_int {} else {
        __assert_fail(
            b"b > ROOT\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/lzcomp/ahuff.c\0" as *const u8 as *const ::core::ffi::c_char,
            199 as ::core::ffi::c_uint,
            b"void SwapNodes(AHUFF *, short, short)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    if (a as ::core::ffi::c_long) < 2 as ::core::ffi::c_long * (*t).range {} else {
        __assert_fail(
            b"a < 2 * t->range\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/lzcomp/ahuff.c\0" as *const u8 as *const ::core::ffi::c_char,
            200 as ::core::ffi::c_uint,
            b"void SwapNodes(AHUFF *, short, short)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    if (b as ::core::ffi::c_long) < 2 as ::core::ffi::c_long * (*t).range {} else {
        __assert_fail(
            b"b < 2 * t->range\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/lzcomp/ahuff.c\0" as *const u8 as *const ::core::ffi::c_char,
            201 as ::core::ffi::c_uint,
            b"void SwapNodes(AHUFF *, short, short)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    if ((*tree.offset(a as isize)).code as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        || *(*t).symbolIndex.offset((*tree.offset(a as isize)).code as isize)
            as ::core::ffi::c_int == a as ::core::ffi::c_int
    {} else {
        __assert_fail(
            b"tree[a].code < 0 || t->symbolIndex[tree[a].code] == a\0" as *const u8
                as *const ::core::ffi::c_char,
            b"src/lzcomp/ahuff.c\0" as *const u8 as *const ::core::ffi::c_char,
            202 as ::core::ffi::c_uint,
            b"void SwapNodes(AHUFF *, short, short)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    if ((*tree.offset(b as isize)).code as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        || *(*t).symbolIndex.offset((*tree.offset(b as isize)).code as isize)
            as ::core::ffi::c_int == b as ::core::ffi::c_int
    {} else {
        __assert_fail(
            b"tree[b].code < 0 || t->symbolIndex[tree[b].code] == b\0" as *const u8
                as *const ::core::ffi::c_char,
            b"src/lzcomp/ahuff.c\0" as *const u8 as *const ::core::ffi::c_char,
            203 as ::core::ffi::c_uint,
            b"void SwapNodes(AHUFF *, short, short)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    
    let mut upa:  ::core::ffi::c_short =  (*tree.offset(a as isize)).up;let mut upb:  ::core::ffi::c_short =  (*tree.offset(b as isize)).up;
    if ((*tree.offset(upa as isize)).code as ::core::ffi::c_int)
        < 0 as ::core::ffi::c_int
    {} else {
        __assert_fail(
            b"tree[upa].code < 0\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/lzcomp/ahuff.c\0" as *const u8 as *const ::core::ffi::c_char,
            207 as ::core::ffi::c_uint,
            b"void SwapNodes(AHUFF *, short, short)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    if ((*tree.offset(upb as isize)).code as ::core::ffi::c_int)
        < 0 as ::core::ffi::c_int
    {} else {
        __assert_fail(
            b"tree[upb].code < 0\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/lzcomp/ahuff.c\0" as *const u8 as *const ::core::ffi::c_char,
            208 as ::core::ffi::c_uint,
            b"void SwapNodes(AHUFF *, short, short)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    if (*tree.offset(upa as isize)).left as ::core::ffi::c_int == a as ::core::ffi::c_int
        || (*tree.offset(upa as isize)).right as ::core::ffi::c_int
            == a as ::core::ffi::c_int
    {} else {
        __assert_fail(
            b"tree[upa].left == a || tree[upa].right == a\0" as *const u8
                as *const ::core::ffi::c_char,
            b"src/lzcomp/ahuff.c\0" as *const u8 as *const ::core::ffi::c_char,
            210 as ::core::ffi::c_uint,
            b"void SwapNodes(AHUFF *, short, short)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    if (*tree.offset(upb as isize)).left as ::core::ffi::c_int == b as ::core::ffi::c_int
        || (*tree.offset(upb as isize)).right as ::core::ffi::c_int
            == b as ::core::ffi::c_int
    {} else {
        __assert_fail(
            b"tree[upb].left == b || tree[upb].right == b\0" as *const u8
                as *const ::core::ffi::c_char,
            b"src/lzcomp/ahuff.c\0" as *const u8 as *const ::core::ffi::c_char,
            211 as ::core::ffi::c_uint,
            b"void SwapNodes(AHUFF *, short, short)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    if (*tree.offset(a as isize)).weight == (*tree.offset(b as isize)).weight {} else {
        __assert_fail(
            b"tree[a].weight == tree[b].weight\0" as *const u8
                as *const ::core::ffi::c_char,
            b"src/lzcomp/ahuff.c\0" as *const u8 as *const ::core::ffi::c_char,
            213 as ::core::ffi::c_uint,
            b"void SwapNodes(AHUFF *, short, short)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    let mut tNode:  nodeType =  *tree.offset(a as isize);
    *tree.offset(a as isize) = *tree.offset(b as isize);
    *tree.offset(b as isize) = tNode;
    (*tree.offset(a as isize)).up = upa;
    (*tree.offset(b as isize)).up = upb;
    let mut code:  ::core::ffi::c_short =  (*tree.offset(a as isize)).code;
    if (code as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        (*tree.offset((*tree.offset(a as isize)).left as isize)).up = a;
        (*tree.offset((*tree.offset(a as isize)).right as isize)).up = a;
    } else {
        if (code as ::core::ffi::c_long) < (*t).range {} else {
            __assert_fail(
                b"code < t->range\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/lzcomp/ahuff.c\0" as *const u8 as *const ::core::ffi::c_char,
                234 as ::core::ffi::c_uint,
                b"void SwapNodes(AHUFF *, short, short)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        };
        *(*t).symbolIndex.offset(code as isize) = a;
    }
    code = (*tree.offset(b as isize)).code;
    if (code as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        (*tree.offset((*tree.offset(b as isize)).left as isize)).up = b;
        (*tree.offset((*tree.offset(b as isize)).right as isize)).up = b;
    } else {
        if (code as ::core::ffi::c_long) < (*t).range {} else {
            __assert_fail(
                b"code < t->range\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/lzcomp/ahuff.c\0" as *const u8 as *const ::core::ffi::c_char,
                244 as ::core::ffi::c_uint,
                b"void SwapNodes(AHUFF *, short, short)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        };
        *(*t).symbolIndex.offset(code as isize) = b;
    }
    if (*tree.offset(upa as isize)).left as ::core::ffi::c_int == a as ::core::ffi::c_int
        || (*tree.offset(upa as isize)).right as ::core::ffi::c_int
            == a as ::core::ffi::c_int
    {} else {
        __assert_fail(
            b"tree[upa].left == a || tree[upa].right == a\0" as *const u8
                as *const ::core::ffi::c_char,
            b"src/lzcomp/ahuff.c\0" as *const u8 as *const ::core::ffi::c_char,
            247 as ::core::ffi::c_uint,
            b"void SwapNodes(AHUFF *, short, short)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    if (*tree.offset(upb as isize)).left as ::core::ffi::c_int == b as ::core::ffi::c_int
        || (*tree.offset(upb as isize)).right as ::core::ffi::c_int
            == b as ::core::ffi::c_int
    {} else {
        __assert_fail(
            b"tree[upb].left == b || tree[upb].right == b\0" as *const u8
                as *const ::core::ffi::c_char,
            b"src/lzcomp/ahuff.c\0" as *const u8 as *const ::core::ffi::c_char,
            248 as ::core::ffi::c_uint,
            b"void SwapNodes(AHUFF *, short, short)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
}
unsafe extern "C" fn UpdateWeight(mut t: *mut AHUFF, mut a: ::core::ffi::c_short) {
    let mut tree: *mut nodeType = (*t).tree;
    let ROOT: ::core::ffi::c_short = 1 as ::core::ffi::c_short;
    while a as ::core::ffi::c_int != ROOT as ::core::ffi::c_int {
        let mut weightA: ::core::ffi::c_long = (*tree.offset(a as isize)).weight;
        let mut b: ::core::ffi::c_short = (a as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as ::core::ffi::c_short;
        if (*tree.offset(b as isize)).weight >= weightA {} else {
            __assert_fail(
                b"tree[b].weight >= weightA\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"src/lzcomp/ahuff.c\0" as *const u8 as *const ::core::ffi::c_char,
                261 as ::core::ffi::c_uint,
                b"void UpdateWeight(AHUFF *, short)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        };
        if (*tree.offset(b as isize)).weight == weightA {
            loop {
                b -= 1;
                if !((*tree.offset(b as isize)).weight == weightA) {
                    break;
                }
            }
            b += 1;
            if b as ::core::ffi::c_int >= ROOT as ::core::ffi::c_int {} else {
                __assert_fail(
                    b"b >= ROOT\0" as *const u8 as *const ::core::ffi::c_char,
                    b"src/lzcomp/ahuff.c\0" as *const u8 as *const ::core::ffi::c_char,
                    267 as ::core::ffi::c_uint,
                    b"void UpdateWeight(AHUFF *, short)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            };
            if b as ::core::ffi::c_int > ROOT as ::core::ffi::c_int {
                SwapNodes(t, a, b);
                a = b;
            }
        }
        weightA += 1;
        (*tree.offset(a as isize)).weight = weightA;
        a = (*tree.offset(a as isize)).up;
    }
    if a as ::core::ffi::c_int == ROOT as ::core::ffi::c_int {} else {
        __assert_fail(
            b"a == ROOT\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/lzcomp/ahuff.c\0" as *const u8 as *const ::core::ffi::c_char,
            281 as ::core::ffi::c_uint,
            b"void UpdateWeight(AHUFF *, short)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    let ref mut fresh0 = (*tree.offset(a as isize)).weight;
    *fresh0 += 1;
    if (*tree.offset(a as isize)).weight
        == (*tree.offset((*tree.offset(a as isize)).left as isize)).weight
            + (*tree.offset((*tree.offset(a as isize)).right as isize)).weight
    {} else {
        __assert_fail(
            b"tree[a].weight == tree[tree[a].left].weight + tree[tree[a].right].weight\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"src/lzcomp/ahuff.c\0" as *const u8 as *const ::core::ffi::c_char,
            284 as ::core::ffi::c_uint,
            b"void UpdateWeight(AHUFF *, short)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
}
unsafe extern "C" fn init_weight(
    mut t: *mut AHUFF,
    mut a: ::core::ffi::c_int,
) -> ::core::ffi::c_long {
    let mut tree: *mut nodeType = (*t).tree;
    if ((*tree.offset(a as isize)).code as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
    {
        (*tree.offset(a as isize)).weight = init_weight(
            t,
            (*tree.offset(a as isize)).left as ::core::ffi::c_int,
        ) + init_weight(t, (*tree.offset(a as isize)).right as ::core::ffi::c_int);
    }
    return (*tree.offset(a as isize)).weight;
}
#[no_mangle]
pub unsafe extern "C" fn MTX_AHUFF_Create(
    mut mem: *mut MTX_MemHandler,
    mut bio: *mut BITIO,
    mut rangeIn: ::core::ffi::c_short,
) -> *mut AHUFF {
    
    
    
    let mut j: ::core::ffi::c_long = 0;
    let ROOT: ::core::ffi::c_short = 1 as ::core::ffi::c_short;
    let mut t: *mut AHUFF = MTX_mem_malloc(
        mem,
        ::core::mem::size_of::<AHUFF>() as ::core::ffi::c_ulong,
    ) as *mut AHUFF;
    (*t).mem = mem;
    (*t).bio = bio;
    let mut range:  ::core::ffi::c_short =  rangeIn;
    (*t).range = rangeIn as ::core::ffi::c_long;
    (*t).bitCount = MTX_AHUFF_BitsUsed(
        (rangeIn as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as ::core::ffi::c_long,
    );
    (*t).bitCount2 = 0 as ::core::ffi::c_long;
    if rangeIn as ::core::ffi::c_int > 256 as ::core::ffi::c_int
        && (rangeIn as ::core::ffi::c_int) < 512 as ::core::ffi::c_int
    {
        rangeIn = (rangeIn as ::core::ffi::c_int - 256 as ::core::ffi::c_int)
            as ::core::ffi::c_short;
        (*t).bitCount2 = MTX_AHUFF_BitsUsed(
            (rangeIn as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_long,
        );
        (*t).bitCount2 += 1;
    }
    (*t).maxSymbol = range as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    (*t).sym_count = 0 as ::core::ffi::c_long;
    (*t).countB = 100 as ::core::ffi::c_long;
    (*t).countA = (*t).countB;
    (*t).symbolIndex = MTX_mem_malloc(
        mem,
        (::core::mem::size_of::<::core::ffi::c_short>() as ::core::ffi::c_ulong)
            .wrapping_mul(range as ::core::ffi::c_ulong),
    ) as *mut ::core::ffi::c_short;
    (*t).tree = MTX_mem_malloc(
        mem,
        (::core::mem::size_of::<nodeType>() as ::core::ffi::c_ulong)
            .wrapping_mul(2 as ::core::ffi::c_ulong)
            .wrapping_mul(range as ::core::ffi::c_ulong),
    ) as *mut nodeType;
    
    let mut limit:  ::core::ffi::c_short =
     (2 as ::core::ffi::c_int as ::core::ffi::c_short as ::core::ffi::c_int
        * range as ::core::ffi::c_int) as ::core::ffi::c_short;let mut i:  ::core::ffi::c_short =  2 as ::core::ffi::c_short;
    while (i as ::core::ffi::c_int) < limit as ::core::ffi::c_int {
        (*(*t).tree.offset(i as isize)).up = (i as ::core::ffi::c_int
            / 2 as ::core::ffi::c_int as ::core::ffi::c_short as ::core::ffi::c_int)
            as ::core::ffi::c_short;
        (*(*t).tree.offset(i as isize)).weight = 1 as ::core::ffi::c_int
            as ::core::ffi::c_short as ::core::ffi::c_long;
        i += 1;
    }
    i = 1 as ::core::ffi::c_short;
    while (i as ::core::ffi::c_int) < range as ::core::ffi::c_int {
        (*(*t).tree.offset(i as isize)).left = (2 as ::core::ffi::c_int
            * i as ::core::ffi::c_int) as ::core::ffi::c_short;
        (*(*t).tree.offset(i as isize)).right = (2 as ::core::ffi::c_int
            * i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as ::core::ffi::c_short;
        i += 1;
    }
    i = 0 as ::core::ffi::c_short;
    while (i as ::core::ffi::c_int) < range as ::core::ffi::c_int {
        (*(*t).tree.offset(i as isize)).code = -(1 as ::core::ffi::c_int)
            as ::core::ffi::c_short;
        (*(*t)
            .tree
            .offset((range as ::core::ffi::c_int + i as ::core::ffi::c_int) as isize))
            .code = i;
        (*(*t)
            .tree
            .offset((range as ::core::ffi::c_int + i as ::core::ffi::c_int) as isize))
            .left = -(1 as ::core::ffi::c_int) as ::core::ffi::c_short;
        (*(*t)
            .tree
            .offset((range as ::core::ffi::c_int + i as ::core::ffi::c_int) as isize))
            .right = -(1 as ::core::ffi::c_int) as ::core::ffi::c_short;
        *(*t).symbolIndex.offset(i as isize) = (range as ::core::ffi::c_int
            + i as ::core::ffi::c_int) as ::core::ffi::c_short;
        i += 1;
    }
    init_weight(t, ROOT as ::core::ffi::c_int);
    if (*t).bitCount2 != 0 as ::core::ffi::c_long {
        UpdateWeight(t, *(*t).symbolIndex.offset(256 as ::core::ffi::c_int as isize));
        UpdateWeight(t, *(*t).symbolIndex.offset(257 as ::core::ffi::c_int as isize));
        if (258 as ::core::ffi::c_int) < range as ::core::ffi::c_int {} else {
            __assert_fail(
                b"258 < range\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/lzcomp/ahuff.c\0" as *const u8 as *const ::core::ffi::c_char,
                376 as ::core::ffi::c_uint,
                b"AHUFF *MTX_AHUFF_Create(MTX_MemHandler *, BITIO *, short)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        };
        i = 0 as ::core::ffi::c_short;
        while (i as ::core::ffi::c_int) < 12 as ::core::ffi::c_int {
            UpdateWeight(
                t,
                *(*t)
                    .symbolIndex
                    .offset(
                        (range as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as isize,
                    ),
            );
            i += 1;
        }
        i = 0 as ::core::ffi::c_short;
        while (i as ::core::ffi::c_int) < 6 as ::core::ffi::c_int {
            UpdateWeight(
                t,
                *(*t)
                    .symbolIndex
                    .offset(
                        (range as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize,
                    ),
            );
            i += 1;
        }
    } else {
        j = 0 as ::core::ffi::c_long;
        while j < 2 as ::core::ffi::c_long {
            i = 0 as ::core::ffi::c_short;
            while (i as ::core::ffi::c_int) < range as ::core::ffi::c_int {
                UpdateWeight(t, *(*t).symbolIndex.offset(i as isize));
                i += 1;
            }
            j += 1;
        }
    }
    (*t).countB = 0 as ::core::ffi::c_long;
    (*t).countA = (*t).countB;
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn MTX_AHUFF_Destroy(mut t: *mut AHUFF) {
    MTX_mem_free((*t).mem, (*t).symbolIndex as *mut ::core::ffi::c_void);
    MTX_mem_free((*t).mem, (*t).tree as *mut ::core::ffi::c_void);
    MTX_mem_free((*t).mem, t as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn MTX_AHUFF_WriteSymbolCost(
    mut t: *mut AHUFF,
    mut symbol: ::core::ffi::c_short,
) -> ::core::ffi::c_long {
    let mut tree: *mut nodeType = (*t).tree;
    
    let mut sp: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let ROOT: ::core::ffi::c_short = 1 as ::core::ffi::c_short;
    let mut a:  ::core::ffi::c_short =  *(*t).symbolIndex.offset(symbol as isize);
    if (*(*t).tree.offset(a as isize)).code as ::core::ffi::c_int
        == symbol as ::core::ffi::c_int
    {} else {
        __assert_fail(
            b"t->tree[a].code == symbol\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/lzcomp/ahuff.c\0" as *const u8 as *const ::core::ffi::c_char,
            419 as ::core::ffi::c_uint,
            b"long MTX_AHUFF_WriteSymbolCost(AHUFF *, short)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    loop {
        sp += 1;
        a = (*tree.offset(a as isize)).up;
        if !(a as ::core::ffi::c_int != ROOT as ::core::ffi::c_int) {
            break;
        }
    }
    return (sp as ::core::ffi::c_long) << 16 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn MTX_AHUFF_WriteSymbol(
    mut t: *mut AHUFF,
    mut symbol: ::core::ffi::c_short,
) {
    let mut tree: *mut nodeType = (*t).tree;
    
    
    let mut sp: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut stackArr: [::core::ffi::c_char; 50] = [0; 50];
    let mut stack: *mut ::core::ffi::c_char = &raw mut stackArr
        as *mut ::core::ffi::c_char;
    let mut bio: *mut BITIO = (*t).bio;
    let mut up: ::core::ffi::c_short = 0;
    let ROOT: ::core::ffi::c_short = 1 as ::core::ffi::c_short;
    let mut a:  ::core::ffi::c_short =  *(*t).symbolIndex.offset(symbol as isize);
    if (*(*t).tree.offset(a as isize)).code as ::core::ffi::c_int
        == symbol as ::core::ffi::c_int
    {} else {
        __assert_fail(
            b"t->tree[a].code == symbol\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/lzcomp/ahuff.c\0" as *const u8 as *const ::core::ffi::c_char,
            442 as ::core::ffi::c_uint,
            b"void MTX_AHUFF_WriteSymbol(AHUFF *, short)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    let mut aa:  ::core::ffi::c_short =  a;
    loop {
        up = (*tree.offset(a as isize)).up;
        let fresh1 = sp;
        sp = sp + 1;
        *stack.offset(fresh1 as isize) = ((*tree.offset(up as isize)).right
            as ::core::ffi::c_int == a as ::core::ffi::c_int) as ::core::ffi::c_int
            as ::core::ffi::c_char;
        a = up;
        if !(a as ::core::ffi::c_int != ROOT as ::core::ffi::c_int) {
            break;
        }
    }
    if sp < 50 as ::core::ffi::c_int {} else {
        __assert_fail(
            b"sp < 50\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/lzcomp/ahuff.c\0" as *const u8 as *const ::core::ffi::c_char,
            450 as ::core::ffi::c_uint,
            b"void MTX_AHUFF_WriteSymbol(AHUFF *, short)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    loop {
        sp -= 1;
        MTX_BITIO_output_bit(bio, *stack.offset(sp as isize) as ::core::ffi::c_ulong);
        if !(sp != 0) {
            break;
        }
    }
    UpdateWeight(t, aa);
}
#[no_mangle]
pub unsafe extern "C" fn MTX_AHUFF_ReadSymbol(
    mut t: *mut AHUFF,
) -> ::core::ffi::c_short {
    let ROOT: ::core::ffi::c_short = 1 as ::core::ffi::c_short;
    let mut tree: *mut nodeType = (*t).tree;
    let mut a: ::core::ffi::c_short = ROOT;
    let mut symbol: ::core::ffi::c_short = 0;
    let mut bio: *mut BITIO = (*t).bio;
    loop {
        a = (if MTX_BITIO_input_bit(bio) as ::core::ffi::c_int != 0 {
            (*tree.offset(a as isize)).right as ::core::ffi::c_int
        } else {
            (*tree.offset(a as isize)).left as ::core::ffi::c_int
        }) as ::core::ffi::c_short;
        symbol = (*tree.offset(a as isize)).code;
        if !((symbol as ::core::ffi::c_int) < 0 as ::core::ffi::c_int) {
            break;
        }
    }
    UpdateWeight(t, a);
    return symbol;
}
