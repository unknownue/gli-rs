
extern crate gli_rs as gli;

#[cfg(test)]
mod test_format {

    #[test]
    fn is_compressed() {

        let test_format = gli::ffi::root::gli::format_FORMAT_RGB_ETC_UNORM_BLOCK8;

        unsafe {
            assert!(gli::ffi::root::gli::is_compressed(test_format));
        }
    }
}
