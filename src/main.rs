use std::{cell::RefCell, rc::Rc};

pub struct Solution {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

mod list_node {
    mod l19;
}
mod interview {
    mod i1;
    mod i100;
    mod i101;
    mod i102;
    mod i104;
    mod i105;
    mod i106;
    mod i108;
    mod i11;
    mod i121;
    mod i125;
    mod i128;
    mod i13;
    mod i134;
    mod i135;
    mod i14;
    mod i15;
    mod i151;
    mod i155;
    mod i167;
    mod i169;
    mod i17;
    mod i189;
    mod i199;
    mod i2;
    mod i20;
    mod i200;
    mod i202;
    mod i205;
    mod i208;
    mod i209;
    mod i21;
    mod i226;
    mod i228;
    mod i230;
    mod i238;
    mod i242;
    mod i26;
    mod i27;
    mod i274;
    mod i283;
    mod i289;
    mod i290;
    mod i30;
    mod i35;
    mod i36;
    mod i380;
    mod i383;
    mod i39;
    mod i392;
    mod i42;
    mod i45;
    mod i452;
    mod i46;
    mod i48;
    mod i49;
    mod i530;
    mod i54;
    mod i55;
    mod i56;
    mod i57;
    mod i58;
    mod i637;
    mod i70;
    mod i71;
    mod i73;
    mod i77;
    mod i80;
    mod i88;
    mod i92;
}

mod search {
    mod search1;
    mod search2;
}

mod sliding_window {
    mod s1052;
    mod s1208;
    mod s1343;
    mod s1358;
    mod s1456;
    mod s1461;
    mod s1493;
    mod s2090;
    mod s2302;
    mod s2379;
    mod s2730;
    mod s2962;
    mod s3;
    mod s3090;
    mod s344;
    mod s643;
    mod s713;
    mod s76;
    mod s904;
}

mod data {
    mod d1;
    mod d121;
    mod d1512;
    mod d219;
    mod d2909;
    mod d303;
    mod d454;
    mod d53;
    mod d560;
    mod d6;
}

fn main() {
    let result = Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]);
    println!("{:?}", result);
}
