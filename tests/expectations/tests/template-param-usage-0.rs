/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UsesTemplateParameter<T> {
    pub t: T,
    _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl <T> Default for UsesTemplateParameter<T> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}