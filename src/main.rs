pub struct Solution {}
mod list_node {
    mod l19;
}
mod interview {
    mod a274;
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
    let result = Solution::h_index(vec![1, 3, 1]);

    println!("{}", result)
}
