use sicstus_sys::SP_integer;

pub extern "C" fn c1(a: SP_integer, b: SP_integer) -> SP_integer {
    a + b
}
