pub struct Solution {}
mod list_node {
    mod l19;
}
mod interview {
    mod a209;
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
    mod s1208;
    mod s1343;
    mod s1358;
    mod s1456;
    mod s1461;
    mod s1493;
    mod s2090;
    mod s2379;
    mod s2730;
    mod s2962;
    mod s3;
    mod s3090;
    mod s643;
    mod s713;
    mod s76;
    mod s904;
}

fn main() {
    let result = Solution::number_of_substrings(String::from("abcabc"));
    println!("{:?}", result);
}
