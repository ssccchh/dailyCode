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
    mod array9;
    mod db70;
    mod i13;
    mod i134;
    mod i135;
    mod i14;
    mod i151;
    mod i289;
    mod i30;
    mod i36;
    mod i42;
    mod i48;
    mod i54;
    mod i58;
    mod i73;
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
    let result = Solution::find_substring(
        String::from("wordgoodgoodgoodbestword"),
        vec![
            String::from("word"),
            String::from("good"),
            String::from("best"),
            String::from("good"),
        ],
    );
    println!("{:?}", result);
}
