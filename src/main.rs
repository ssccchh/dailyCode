pub struct Solution {}
mod list_node {
    mod l19;
}
mod interview {
    mod a45;
    mod a55;
    mod array1;
    mod array2;
    mod array3;
    mod array4;
    mod array5;
    mod array6;
    mod array7;
    mod array8;
    mod array9;
    mod db70;
    mod is_palindrome_125;
    mod l11;
    mod l167;
    mod l283;
    mod l392;
    mod p15;
}

mod search {
    mod search1;
    mod search2;
}

fn main() {
    let result = Solution::jump(vec![7, 0, 9, 6, 9, 6, 1, 7, 9, 0, 1, 2, 9, 0, 3]);

    println!("{}", result)
}
