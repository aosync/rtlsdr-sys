#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub const rtlsdr_tuner_RTLSDR_TUNER_UNKNOWN: rtlsdr_tuner = 0;
pub const rtlsdr_tuner_RTLSDR_TUNER_E4000: rtlsdr_tuner = 1;
pub const rtlsdr_tuner_RTLSDR_TUNER_FC0012: rtlsdr_tuner = 2;
pub const rtlsdr_tuner_RTLSDR_TUNER_FC0013: rtlsdr_tuner = 3;
pub const rtlsdr_tuner_RTLSDR_TUNER_FC2580: rtlsdr_tuner = 4;
pub const rtlsdr_tuner_RTLSDR_TUNER_R820T: rtlsdr_tuner = 5;
pub const rtlsdr_tuner_RTLSDR_TUNER_R828D: rtlsdr_tuner = 6;
pub type rtlsdr_tuner = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rtlsdr_dev {
    _unused: [u8; 0],
}

pub type rtlsdr_dev_t = rtlsdr_dev;

pub type rtlsdr_read_async_cb_t = ::std::option::Option<
    unsafe extern "C" fn(
        buf: *mut ::std::os::raw::c_uchar,
        len: u32,
        ctx: *mut ::std::os::raw::c_void,
    ),
>;

extern "C" {
    pub fn rtlsdr_get_device_count() -> u32;

    pub fn rtlsdr_get_device_name(index: u32) -> *const ::std::os::raw::c_char;

    pub fn rtlsdr_get_device_usb_strings(
        index: u32,
        manufact: *mut ::std::os::raw::c_char,
        product: *mut ::std::os::raw::c_char,
        serial: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn rtlsdr_get_index_by_serial(
        serial: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn rtlsdr_open(dev: *mut *mut rtlsdr_dev_t, index: u32) -> ::std::os::raw::c_int;

    pub fn rtlsdr_close(dev: *mut rtlsdr_dev_t) -> ::std::os::raw::c_int;

    pub fn rtlsdr_set_xtal_freq(
        dev: *mut rtlsdr_dev_t,
        rtl_freq: u32,
        tuner_freq: u32,
    ) -> ::std::os::raw::c_int;

    pub fn rtlsdr_get_xtal_freq(
        dev: *mut rtlsdr_dev_t,
        rtl_freq: *mut u32,
        tuner_freq: *mut u32,
    ) -> ::std::os::raw::c_int;

    pub fn rtlsdr_get_usb_strings(
        dev: *mut rtlsdr_dev_t,
        manufact: *mut ::std::os::raw::c_char,
        product: *mut ::std::os::raw::c_char,
        serial: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn rtlsdr_write_eeprom(
        dev: *mut rtlsdr_dev_t,
        data: *mut u8,
        offset: u8,
        len: u16,
    ) -> ::std::os::raw::c_int;

    pub fn rtlsdr_read_eeprom(
        dev: *mut rtlsdr_dev_t,
        data: *mut u8,
        offset: u8,
        len: u16,
    ) -> ::std::os::raw::c_int;

    pub fn rtlsdr_set_center_freq(dev: *mut rtlsdr_dev_t, freq: u32) -> ::std::os::raw::c_int;

    pub fn rtlsdr_get_center_freq(dev: *mut rtlsdr_dev_t) -> u32;

    pub fn rtlsdr_set_freq_correction(
        dev: *mut rtlsdr_dev_t,
        ppm: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn rtlsdr_get_freq_correction(dev: *mut rtlsdr_dev_t) -> ::std::os::raw::c_int;

    pub fn rtlsdr_get_tuner_type(dev: *mut rtlsdr_dev_t) -> rtlsdr_tuner;

    pub fn rtlsdr_get_tuner_gains(
        dev: *mut rtlsdr_dev_t,
        gains: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn rtlsdr_set_tuner_gain(
        dev: *mut rtlsdr_dev_t,
        gain: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn rtlsdr_set_tuner_bandwidth(dev: *mut rtlsdr_dev_t, bw: u32) -> ::std::os::raw::c_int;

    pub fn rtlsdr_get_tuner_gain(dev: *mut rtlsdr_dev_t) -> ::std::os::raw::c_int;

    pub fn rtlsdr_set_tuner_if_gain(
        dev: *mut rtlsdr_dev_t,
        stage: ::std::os::raw::c_int,
        gain: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn rtlsdr_set_tuner_gain_mode(
        dev: *mut rtlsdr_dev_t,
        manual: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn rtlsdr_set_sample_rate(dev: *mut rtlsdr_dev_t, rate: u32) -> ::std::os::raw::c_int;

    pub fn rtlsdr_get_sample_rate(dev: *mut rtlsdr_dev_t) -> u32;

    pub fn rtlsdr_set_testmode(
        dev: *mut rtlsdr_dev_t,
        on: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn rtlsdr_set_agc_mode(
        dev: *mut rtlsdr_dev_t,
        on: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn rtlsdr_set_direct_sampling(
        dev: *mut rtlsdr_dev_t,
        on: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn rtlsdr_get_direct_sampling(dev: *mut rtlsdr_dev_t) -> ::std::os::raw::c_int;

    pub fn rtlsdr_set_offset_tuning(
        dev: *mut rtlsdr_dev_t,
        on: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn rtlsdr_get_offset_tuning(dev: *mut rtlsdr_dev_t) -> ::std::os::raw::c_int;

    pub fn rtlsdr_reset_buffer(dev: *mut rtlsdr_dev_t) -> ::std::os::raw::c_int;

    pub fn rtlsdr_read_sync(
        dev: *mut rtlsdr_dev_t,
        buf: *mut ::std::os::raw::c_void,
        len: ::std::os::raw::c_int,
        n_read: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn rtlsdr_wait_async(
        dev: *mut rtlsdr_dev_t,
        cb: rtlsdr_read_async_cb_t,
        ctx: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn rtlsdr_read_async(
        dev: *mut rtlsdr_dev_t,
        cb: rtlsdr_read_async_cb_t,
        ctx: *mut ::std::os::raw::c_void,
        buf_num: u32,
        buf_len: u32,
    ) -> ::std::os::raw::c_int;

    pub fn rtlsdr_cancel_async(dev: *mut rtlsdr_dev_t) -> ::std::os::raw::c_int;

    pub fn rtlsdr_set_bias_tee(
        dev: *mut rtlsdr_dev_t,
        on: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn rtlsdr_set_bias_tee_gpio(
        dev: *mut rtlsdr_dev_t,
        gpio: ::std::os::raw::c_int,
        on: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let devcount = unsafe { rtlsdr_get_device_count() };
        
        println!("{} devices detected!", devcount);
    }
}
