#![feature(extern_types)]
#![feature(ptr_wrapping_offset_from)]
#![feature(c_variadic)]

extern crate tectonic_bridge as bridge;
use bridge::*;

#[macro_export]
macro_rules! info(
    ($($arg:tt)*) => {
        if !(crate::dpx_error::_dpx_quietness > 0) {
            print!($($arg)*);
            crate::dpx_error::_last_message_type = crate::dpx_error::DPX_MESG_INFO;
        }
    };
);

#[macro_export]
macro_rules! warn(
    ($($arg:tt)*) => {
        if !(crate::dpx_error::_dpx_quietness > 1) {
            if crate::dpx_error::_last_message_type as u32 == crate::dpx_error::DPX_MESG_INFO as u32 {
                println!("");
            }
            print!("warning: ");
            println!($($arg)*);
            crate::dpx_error::_last_message_type = crate::dpx_error::DPX_MESG_WARN;
        }
    };
);

pub(crate) fn isblank(c: libc::c_int) -> libc::c_int {
    (c == ' ' as _ || c == '\t' as _) as _
}

#[inline]
pub(crate) unsafe fn strstartswith(s: *const i8, prefix: *const i8) -> *const i8 {
    let length = libc::strlen(prefix);
    if libc::strncmp(s, prefix, length) == 0i32 {
        return s.offset(length as isize);
    }
    0 as *const i8
}

#[inline]
pub(crate) unsafe fn streq_ptr(s1: *const i8, s2: *const i8) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return libc::strcmp(s1, s2) == 0i32;
    }
    false
}

#[inline]
pub(crate) unsafe fn mfree(ptr: *mut libc::c_void) -> *mut libc::c_void {
    libc::free(ptr);
    std::ptr::null_mut()
}

pub mod dpx_agl;
pub mod dpx_bmpimage;
pub mod dpx_cff;
pub mod dpx_cff_dict;
pub mod dpx_cid;
pub mod dpx_cidtype0;
pub mod dpx_cidtype2;
pub mod dpx_cmap;
pub mod dpx_cmap_read;
pub mod dpx_cmap_write;
pub mod dpx_cs_type2;
pub mod dpx_dpxconf;
pub mod dpx_dpxcrypt;
pub mod dpx_dpxfile;
pub mod dpx_dpxutil;
pub mod dpx_dvi;
pub mod dpx_dvipdfmx;
pub mod dpx_epdf;
pub mod dpx_error;
pub mod dpx_fontmap;
pub mod dpx_jp2image;
pub mod dpx_jpegimage;
pub mod dpx_mem;
pub mod dpx_mfileio;
pub mod dpx_mpost;
pub mod dpx_numbers;
pub mod dpx_otl_conf;
pub mod dpx_otl_opt;
pub mod dpx_pdfcolor;
pub mod dpx_pdfdev;
pub mod dpx_pdfdoc;
pub mod dpx_pdfdraw;
pub mod dpx_pdfencoding;
pub mod dpx_pdfencrypt;
pub mod dpx_pdffont;
pub mod dpx_pdfnames;
pub mod dpx_pdfobj;
pub mod dpx_pdfparse;
pub mod dpx_pdfresource;
pub mod dpx_pdfximage;
pub mod dpx_pkfont;
pub mod dpx_pngimage;
pub mod dpx_pst;
pub mod dpx_pst_obj;
pub mod dpx_sfnt;
pub mod dpx_spc_color;
pub mod dpx_spc_dvipdfmx;
pub mod dpx_spc_dvips;
pub mod dpx_spc_html;
pub mod dpx_spc_misc;
pub mod dpx_spc_pdfm;
pub mod dpx_spc_tpic;
pub mod dpx_spc_util;
pub mod dpx_spc_xtx;
pub mod dpx_specials;
pub mod dpx_subfont;
pub mod dpx_t1_char;
pub mod dpx_t1_load;
pub mod dpx_tfm;
pub mod dpx_truetype;
pub mod dpx_tt_aux;
pub mod dpx_tt_cmap;
pub mod dpx_tt_glyf;
pub mod dpx_tt_gsub;
pub mod dpx_tt_post;
pub mod dpx_tt_table;
pub mod dpx_type0;
pub mod dpx_type1;
pub mod dpx_type1c;
pub mod dpx_unicode;
pub mod dpx_vf;

pub use crate::dpx_dvipdfmx::dvipdfmx_main;