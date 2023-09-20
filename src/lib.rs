use sicstus_sys::SP_integer;

#[no_mangle]
pub extern "C" fn c1(a: SP_integer) -> SP_integer {
    a + 1
}
