/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct C<T> {
    pub foo: *const T,
    pub bar: *mut T,
    _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl <T> Default for C<T> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}