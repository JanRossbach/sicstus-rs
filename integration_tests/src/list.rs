use sicstus_rs::TermRef;

pub fn test_list() {
    test_vec_roundtrip();
}

pub fn test_vec_roundtrip() {
    let v: Vec<i32> = vec![1,2,3,4,5];
    let t: TermRef = v.iter().map(|x| TermRef::new_integer(*x as i64)).collect();
    let v2: Vec<i32> = t.into_iter().map(|x| {
        x.get_integer().unwrap() as i32
    } ).collect();
    let v: Vec<i32> = v.into_iter().rev().collect();
    assert_eq!(v, v2);
    println!("test_vec_roundtrip, Ok");
}
