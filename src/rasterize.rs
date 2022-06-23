use bytemuck::{cast_slice, Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Pod, Zeroable)]
pub struct Color(pub u8, pub u8, pub u8);

fn foo() {
    let fuck = 123;
}
