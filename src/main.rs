pub struct Solution {}
mod list_node {
    mod l19;
}
mod interview {
    mod a238;
    mod a274;
    mod a380;
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

mod sliding_window {
    mod s1052;
    mod s1343;
    mod s1456;
    mod s2090;
    mod s2379;
    mod s643;
}

fn main() {
    let result = Solution::max_satisfied(
        vec![1, 0, 1, 2, 1, 1, 7, 5],
        vec![0, 1, 0, 1, 0, 1, 0, 1],
        3,
    );
    println!("{:?}", result);
}
