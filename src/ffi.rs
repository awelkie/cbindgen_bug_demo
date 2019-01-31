type SomeType = super::MyStruct<u32>;

#[repr(C)]
pub struct MyStruct;

#[no_mangle]
pub extern "C" fn use_mystruct(my_struct: *mut MyStruct) {
}
