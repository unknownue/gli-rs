
#[cfg(test)]
mod format {

    extern crate gli_rs as gli;

    #[test]
    fn is_compressed() {

        let test_format = gli::Format::RGB_ETC_UNORM_BLOCK8;

        assert!(test_format.is_compressed());
    }
}
