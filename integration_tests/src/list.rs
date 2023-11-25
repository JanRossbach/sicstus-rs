use sicstus_rs::TermRef;

pub fn test_list() {
    test_vec_roundtrip();
}

pub fn test_vec_roundtrip() {
    let v: Vec<i32> = vec![1,2,3,4,5];
    let t = TermRef::from(v.clone());
    let v2: Vec<i64> = t.into();
    let v: Vec<i64> = v.into_iter().map(|x| x as i64).collect();
    assert_eq!(v, v2);
}
