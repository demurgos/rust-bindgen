/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct mach_msg_type_descriptor_t {
    pub _bitfield_1: u32,
    pub __bindgen_align: [u32; 0usize],
}
#[test]
fn bindgen_test_layout_mach_msg_type_descriptor_t() {
    assert_eq!(::std::mem::size_of::<mach_msg_type_descriptor_t>() , 4usize ,
               concat ! (
               "Size of: " , stringify ! ( mach_msg_type_descriptor_t ) ));
    assert_eq! (::std::mem::align_of::<mach_msg_type_descriptor_t>() , 4usize
                , concat ! (
                "Alignment of " , stringify ! ( mach_msg_type_descriptor_t )
                ));
}
impl Clone for mach_msg_type_descriptor_t {
    fn clone(&self) -> Self { *self }
}
impl mach_msg_type_descriptor_t {
    #[inline]
    pub fn pad3(&self) -> ::std::os::raw::c_uint {
        let mask = 16777215usize as u32;
        let unit_field_val: u32 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 0usize;
        unsafe { ::std::mem::transmute(val as u32) }
    }
    #[inline]
    pub fn set_pad3(&mut self, val: ::std::os::raw::c_uint) {
        let mask = 16777215usize as u32;
        let val = val as u32 as u32;
        let mut unit_field_val: u32 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 0usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn type_(&self) -> ::std::os::raw::c_uint {
        let mask = 4278190080usize as u32;
        let unit_field_val: u32 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 24usize;
        unsafe { ::std::mem::transmute(val as u32) }
    }
    #[inline]
    pub fn set_type(&mut self, val: ::std::os::raw::c_uint) {
        let mask = 4278190080usize as u32;
        let val = val as u32 as u32;
        let mut unit_field_val: u32 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 24usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn new_bitfield_1(pad3: ::std::os::raw::c_uint,
                          type_: ::std::os::raw::c_uint) -> u32 {
        ({
             ({ 0 } |
                  ((pad3 as u32 as u32) << 0usize) & (16777215usize as u32))
         } | ((type_ as u32 as u32) << 24usize) & (4278190080usize as u32))
    }
}
