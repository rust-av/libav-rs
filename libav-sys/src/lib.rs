// TODO do w/out the unions?
#![feature(untagged_unions)]

pub mod avcodec;
pub mod avformat;

#[cfg(test)]
mod tests {
    use super::avcodec::avcodec_configuration;
    use super::avformat::avformat_configuration;
    use std::ffi::CStr;
    #[test]
    fn version() {
        println!("{}{}", unsafe {
            CStr::from_ptr(avcodec_configuration()).to_string_lossy() },
        unsafe {
            CStr::from_ptr(avformat_configuration()).to_string_lossy()
        });
    }
}
