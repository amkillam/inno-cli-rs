//Defines TPSExec pascalscript type
//TPSExec in pascal is a class, so the best we can do is define it as opaque,
//and pass it to pascal exported functions for any usage otherwise

#[repr(C)]
pub struct PascalTPSExec {
    _private: [u8; 0],
    _marker: core::marker::PhantomData<*mut u8>,
}
