#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate libc;
extern "C" {
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn ttstub_input_open(
        path: *const libc::c_char,
        format: tt_input_format_type,
        is_gz: libc::c_int,
    ) -> rust_input_handle_t;
    #[no_mangle]
    fn ttstub_input_close(handle: rust_input_handle_t) -> libc::c_int;
    #[no_mangle]
    static mut CSI_IDENTITY: CIDSysInfo;
    #[no_mangle]
    fn dpx_warning(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn dpx_message(fmt: *const libc::c_char, _: ...);
    /* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

        Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
        the dvipdfmx project team.

        Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

        This program is free software; you can redistribute it and/or modify
        it under the terms of the GNU General Public License as published by
        the Free Software Foundation; either version 2 of the License, or
        (at your option) any later version.

        This program is distributed in the hope that it will be useful,
        but WITHOUT ANY WARRANTY; without even the implied warranty of
        MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
        GNU General Public License for more details.

        You should have received a copy of the GNU General Public License
        along with this program; if not, write to the Free Software
        Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
    */
    #[no_mangle]
    fn new(size: uint32_t) -> *mut libc::c_void;
    #[no_mangle]
    fn renew(p: *mut libc::c_void, size: uint32_t) -> *mut libc::c_void;
    /* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

        Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
        the dvipdfmx project team.

        Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

        This program is free software; you can redistribute it and/or modify
        it under the terms of the GNU General Public License as published by
        the Free Software Foundation; either version 2 of the License, or
        (at your option) any later version.

        This program is distributed in the hope that it will be useful,
        but WITHOUT ANY WARRANTY; without even the implied warranty of
        MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
        GNU General Public License for more details.

        You should have received a copy of the GNU General Public License
        along with this program; if not, write to the Free Software
        Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
    */
    #[no_mangle]
    fn CMap_parse_check_sig(handle: rust_input_handle_t) -> libc::c_int;
    #[no_mangle]
    fn CMap_parse(cmap: *mut CMap, handle: rust_input_handle_t) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
/* The weird enum values are historical and could be rationalized. But it is
 * good to write them explicitly since they must be kept in sync with
 * `src/engines/mod.rs`.
 */
pub type tt_input_format_type = libc::c_uint;
pub const TTIF_TECTONIC_PRIMARY: tt_input_format_type = 59;
pub const TTIF_OPENTYPE: tt_input_format_type = 47;
pub const TTIF_SFD: tt_input_format_type = 46;
pub const TTIF_CMAP: tt_input_format_type = 45;
pub const TTIF_ENC: tt_input_format_type = 44;
pub const TTIF_MISCFONTS: tt_input_format_type = 41;
pub const TTIF_BINARY: tt_input_format_type = 40;
pub const TTIF_TRUETYPE: tt_input_format_type = 36;
pub const TTIF_VF: tt_input_format_type = 33;
pub const TTIF_TYPE1: tt_input_format_type = 32;
pub const TTIF_TEX_PS_HEADER: tt_input_format_type = 30;
pub const TTIF_TEX: tt_input_format_type = 26;
pub const TTIF_PICT: tt_input_format_type = 25;
pub const TTIF_OVF: tt_input_format_type = 23;
pub const TTIF_OFM: tt_input_format_type = 20;
pub const TTIF_FONTMAP: tt_input_format_type = 11;
pub const TTIF_FORMAT: tt_input_format_type = 10;
pub const TTIF_CNF: tt_input_format_type = 8;
pub const TTIF_BST: tt_input_format_type = 7;
pub const TTIF_BIB: tt_input_format_type = 6;
pub const TTIF_AFM: tt_input_format_type = 4;
pub const TTIF_TFM: tt_input_format_type = 3;
pub type rust_input_handle_t = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CIDSysInfo {
    pub registry: *mut libc::c_char,
    pub ordering: *mut libc::c_char,
    pub supplement: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rangeDef {
    pub dim: size_t,
    pub codeLo: *mut libc::c_uchar,
    pub codeHi: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapDef {
    pub flag: libc::c_int,
    pub len: size_t,
    pub code: *mut libc::c_uchar,
    pub next: *mut mapDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapData {
    pub data: *mut libc::c_uchar,
    pub prev: *mut mapData,
    pub pos: libc::c_int,
}
/* quasi-hack to get the primary input */
/* CID, Code... MEM_ALLOC_SIZE bytes  */
/* Previous mapData data segment      */
/* Position of next free data segment */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CMap {
    pub name: *mut libc::c_char,
    pub type_0: libc::c_int,
    pub wmode: libc::c_int,
    pub CSI: *mut CIDSysInfo,
    pub useCMap: *mut CMap,
    pub codespace: C2RustUnnamed_0,
    pub mapTbl: *mut mapDef,
    pub mapData: *mut mapData,
    pub flags: libc::c_int,
    pub profile: C2RustUnnamed,
    pub reverseMap: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub minBytesIn: size_t,
    pub maxBytesIn: size_t,
    pub minBytesOut: size_t,
    pub maxBytesOut: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub num: libc::c_uint,
    pub max: libc::c_uint,
    pub ranges: *mut rangeDef,
}
pub type CID = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CMap_cache {
    pub num: libc::c_int,
    pub max: libc::c_int,
    pub cmaps: *mut *mut CMap,
}
#[inline]
unsafe extern "C" fn mfree(mut ptr: *mut libc::c_void) -> *mut libc::c_void {
    free(ptr);
    return 0 as *mut libc::c_void;
}
/* tectonic/core-strutils.h: miscellaneous C string utilities
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
/* Note that we explicitly do *not* change this on Windows. For maximum
 * portability, we should probably accept *either* forward or backward slashes
 * as directory separators. */
#[inline]
unsafe extern "C" fn streq_ptr(mut s1: *const libc::c_char, mut s2: *const libc::c_char) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return strcmp(s1, s2) == 0i32;
    }
    return 0i32 != 0;
}
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
   the dvipdfmx project team.

   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 2 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program; if not, write to the Free Software
   Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
*/
/*
 * References:
 *
 *  PostScript Language Reference Manual, 3rd. ed. (Adobe Systems Inc.)
 *    5.11.4 CMap Dictionaries
 *    5.11.5 FMapType 9 Composite Fonts
 *  Building CMap Files for CID-Keyed Fonts, Adobe Technical Note #5099
 *  CID-Keyed Font Technology Overview, Adobe Technical Note #5092
 *  Adobe CMap and CIDFont Files Specification, Adobe Technical Specification #5014
 *
 *  Undefined Character Handling:
 *    PLRM 3rd. ed., sec. 5.11.5., "Handling Undefined Characters"
 *
 * TODO:
 *   Only cid(range|char) allowed for CODE_TO_CID and bf(range|char) for CID_TO_CODE ?
 */
static mut __verbose: libc::c_int = 0i32;
static mut __silent: libc::c_int = 0i32;
#[no_mangle]
pub unsafe extern "C" fn CMap_set_verbose(mut level: libc::c_int) {
    __verbose = level;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_set_silent(mut value: libc::c_int) {
    __silent = if value != 0 { 1i32 } else { 0i32 };
}
#[no_mangle]
pub unsafe extern "C" fn CMap_new() -> *mut CMap {
    let mut cmap: *mut CMap = 0 as *mut CMap;
    cmap = new((1i32 as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<CMap>() as libc::c_ulong) as uint32_t)
        as *mut CMap;
    (*cmap).name = 0 as *mut libc::c_char;
    (*cmap).type_0 = 1i32;
    (*cmap).wmode = 0i32;
    (*cmap).useCMap = 0 as *mut CMap;
    (*cmap).CSI = 0 as *mut CIDSysInfo;
    (*cmap).profile.minBytesIn = 2i32 as size_t;
    (*cmap).profile.maxBytesIn = 2i32 as size_t;
    (*cmap).profile.minBytesOut = 2i32 as size_t;
    (*cmap).profile.maxBytesOut = 2i32 as size_t;
    (*cmap).flags = 0i32;
    (*cmap).codespace.num = 0i32 as libc::c_uint;
    (*cmap).codespace.max = 10i32 as libc::c_uint;
    (*cmap).codespace.ranges = new((10i32 as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<rangeDef>() as libc::c_ulong)
        as uint32_t) as *mut rangeDef;
    (*cmap).mapTbl = 0 as *mut mapDef;
    (*cmap).mapData = new((1i32 as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<mapData>() as libc::c_ulong)
        as uint32_t) as *mut mapData;
    (*(*cmap).mapData).prev = 0 as *mut mapData;
    (*(*cmap).mapData).pos = 0i32;
    (*(*cmap).mapData).data = new((4096i32 as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
        as uint32_t) as *mut libc::c_uchar;
    (*cmap).reverseMap = new((65536i32 as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as uint32_t) as *mut libc::c_int;
    memset(
        (*cmap).reverseMap as *mut libc::c_void,
        0i32,
        (65536i32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    return cmap;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_release(mut cmap: *mut CMap) {
    if cmap.is_null() {
        return;
    }
    free((*cmap).name as *mut libc::c_void);
    if !(*cmap).CSI.is_null() {
        free((*(*cmap).CSI).registry as *mut libc::c_void);
        free((*(*cmap).CSI).ordering as *mut libc::c_void);
        free((*cmap).CSI as *mut libc::c_void);
    }
    free((*cmap).codespace.ranges as *mut libc::c_void);
    if !(*cmap).mapTbl.is_null() {
        mapDef_release((*cmap).mapTbl);
    }
    let mut map: *mut mapData = (*cmap).mapData;
    while !map.is_null() {
        let mut prev: *mut mapData = (*map).prev;
        free((*map).data as *mut libc::c_void);
        free(map as *mut libc::c_void);
        map = prev
    }
    free((*cmap).reverseMap as *mut libc::c_void);
    free(cmap as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn CMap_is_Identity(mut cmap: *mut CMap) -> bool {
    if !cmap.is_null() {
    } else {
        __assert_fail(
            b"cmap\x00" as *const u8 as *const libc::c_char,
            b"dpx-cmap.c\x00" as *const u8 as *const libc::c_char,
            149i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                b"_Bool CMap_is_Identity(CMap *)\x00",
            ))
            .as_ptr(),
        );
    }
    return streq_ptr(
        (*cmap).name,
        b"Identity-H\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int
        != 0
        || streq_ptr(
            (*cmap).name,
            b"Identity-V\x00" as *const u8 as *const libc::c_char,
        ) as libc::c_int
            != 0;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_is_valid(mut cmap: *mut CMap) -> bool {
    /* Quick check */
    if cmap.is_null()
        || (*cmap).name.is_null()
        || (*cmap).type_0 < 0i32
        || (*cmap).type_0 > 3i32
        || (*cmap).codespace.num < 1i32 as libc::c_uint
        || (*cmap).type_0 != 0i32 && (*cmap).mapTbl.is_null()
    {
        return 0i32 != 0;
    }
    if !(*cmap).useCMap.is_null() {
        let mut csi1: *mut CIDSysInfo = 0 as *mut CIDSysInfo;
        let mut csi2: *mut CIDSysInfo = 0 as *mut CIDSysInfo;
        csi1 = CMap_get_CIDSysInfo(cmap);
        csi2 = CMap_get_CIDSysInfo((*cmap).useCMap);
        if strcmp((*csi1).registry, (*csi2).registry) != 0
            || strcmp((*csi1).ordering, (*csi2).ordering) != 0
        {
            dpx_warning(
                b"CIDSystemInfo mismatched %s <--> %s\x00" as *const u8 as *const libc::c_char,
                CMap_get_name(cmap),
                CMap_get_name((*cmap).useCMap),
            );
            return 0i32 != 0;
        }
    }
    return 1i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_get_profile(
    mut cmap: *mut CMap,
    mut type_0: libc::c_int,
) -> libc::c_int {
    let mut value: libc::c_int = 0i32;
    if !cmap.is_null() {
    } else {
        __assert_fail(
            b"cmap\x00" as *const u8 as *const libc::c_char,
            b"dpx-cmap.c\x00" as *const u8 as *const libc::c_char,
            184i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                b"int CMap_get_profile(CMap *, int)\x00",
            ))
            .as_ptr(),
        );
    }
    match type_0 {
        0 => value = (*cmap).profile.minBytesIn as libc::c_int,
        1 => value = (*cmap).profile.maxBytesIn as libc::c_int,
        2 => value = (*cmap).profile.maxBytesOut as libc::c_int,
        3 => value = (*cmap).profile.maxBytesOut as libc::c_int,
        _ => {
            _tt_abort(
                b"%s: Unrecognized profile type %d.\x00" as *const u8 as *const libc::c_char,
                b"CMap\x00" as *const u8 as *const libc::c_char,
                type_0,
            );
        }
    }
    return value;
}
/*
 * Put notdef chars for codes not declared in notdef(range|char)
 */
unsafe extern "C" fn handle_undefined(
    mut cmap: *mut CMap,
    mut inbuf: *mut *const libc::c_uchar,
    mut inbytesleft: *mut size_t,
    mut outbuf: *mut *mut libc::c_uchar,
    mut outbytesleft: *mut size_t,
) {
    let mut len: size_t = 0i32 as size_t;
    if *outbytesleft < 2i32 as libc::c_ulong {
        _tt_abort(
            b"%s: Buffer overflow.\x00" as *const u8 as *const libc::c_char,
            b"CMap\x00" as *const u8 as *const libc::c_char,
        );
    }
    match (*cmap).type_0 {
        1 => {
            memcpy(
                *outbuf as *mut libc::c_void,
                b"\x00\x00\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                2i32 as libc::c_ulong,
            );
        }
        2 => {
            memcpy(
                *outbuf as *mut libc::c_void,
                b"\xff\xfd\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                2i32 as libc::c_ulong,
            );
        }
        _ => {
            dpx_warning(
                b"Cannot handle undefined mapping for this type of CMap mapping: %d\x00"
                    as *const u8 as *const libc::c_char,
                (*cmap).type_0,
            );
            dpx_warning(
                b"<0000> is used for .notdef char.\x00" as *const u8 as *const libc::c_char,
            );
            memset(*outbuf as *mut libc::c_void, 0i32, 2i32 as libc::c_ulong);
        }
    }
    *outbuf = (*outbuf).offset(2);
    *outbytesleft =
        (*outbytesleft as libc::c_ulong).wrapping_sub(2i32 as libc::c_ulong) as size_t as size_t;
    len = bytes_consumed(cmap, *inbuf, *inbytesleft);
    *inbuf = (*inbuf).offset(len as isize);
    *inbytesleft = (*inbytesleft as libc::c_ulong).wrapping_sub(len) as size_t as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_decode_char(
    mut cmap: *mut CMap,
    mut inbuf: *mut *const libc::c_uchar,
    mut inbytesleft: *mut size_t,
    mut outbuf: *mut *mut libc::c_uchar,
    mut outbytesleft: *mut size_t,
) {
    let mut t: *mut mapDef = 0 as *mut mapDef;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut save: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut c: libc::c_uchar = 0i32 as libc::c_uchar;
    let mut count: size_t = 0i32 as size_t;
    save = *inbuf;
    p = save;
    /*
     * First handle some special cases:
     */
    if (*cmap).type_0 == 0i32 {
        if (*inbytesleft).wrapping_rem(2i32 as libc::c_ulong) != 0 {
            _tt_abort(
                b"%s: Invalid/truncated input string.\x00" as *const u8 as *const libc::c_char,
                b"CMap\x00" as *const u8 as *const libc::c_char,
            );
        }
        if *outbytesleft < 2i32 as libc::c_ulong {
            _tt_abort(
                b"%s: Buffer overflow.\x00" as *const u8 as *const libc::c_char,
                b"CMap\x00" as *const u8 as *const libc::c_char,
            );
        }
        memcpy(
            *outbuf as *mut libc::c_void,
            *inbuf as *const libc::c_void,
            2i32 as libc::c_ulong,
        );
        *inbuf = (*inbuf).offset(2);
        *outbuf = (*outbuf).offset(2);
        *outbytesleft = (*outbytesleft as libc::c_ulong).wrapping_sub(2i32 as libc::c_ulong)
            as size_t as size_t;
        *inbytesleft =
            (*inbytesleft as libc::c_ulong).wrapping_sub(2i32 as libc::c_ulong) as size_t as size_t;
        return;
    } else {
        if (*cmap).mapTbl.is_null() {
            if !(*cmap).useCMap.is_null() {
                CMap_decode_char((*cmap).useCMap, inbuf, inbytesleft, outbuf, outbytesleft);
                return;
            } else {
                /* no mapping available in this CMap */
                dpx_warning(
                    b"No mapping available for this character.\x00" as *const u8
                        as *const libc::c_char,
                );
                handle_undefined(cmap, inbuf, inbytesleft, outbuf, outbytesleft);
                return;
            }
        }
    }
    if !(*cmap).mapTbl.is_null() {
    } else {
        __assert_fail(b"cmap->mapTbl\x00" as *const u8 as *const libc::c_char,
                      b"dpx-cmap.c\x00" as *const u8 as *const libc::c_char,
                      276i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 92],
                                                &[libc::c_char; 92]>(b"void CMap_decode_char(CMap *, const unsigned char **, size_t *, unsigned char **, size_t *)\x00")).as_ptr());
    }
    t = (*cmap).mapTbl;
    while count < *inbytesleft {
        let fresh0 = p;
        p = p.offset(1);
        c = *fresh0;
        count = count.wrapping_add(1);
        if (*t.offset(c as isize)).flag & 1i32 << 4i32 == 0 {
            break;
        }
        t = (*t.offset(c as isize)).next
    }
    if (*t.offset(c as isize)).flag & 1i32 << 4i32 != 0 {
        /* need more bytes */
        _tt_abort(
            b"%s: Premature end of input string.\x00" as *const u8 as *const libc::c_char,
            b"CMap\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        if if (*t.offset(c as isize)).flag & 0xfi32 != 0i32 {
            1i32
        } else {
            0i32
        } == 0
        {
            if !(*cmap).useCMap.is_null() {
                CMap_decode_char((*cmap).useCMap, inbuf, inbytesleft, outbuf, outbytesleft);
                return;
            } else {
                /* no mapping available in this CMap */
                dpx_warning(
                    b"No character mapping available.\x00" as *const u8 as *const libc::c_char,
                );
                dpx_message(
                    b" CMap name: %s\n\x00" as *const u8 as *const libc::c_char,
                    CMap_get_name(cmap),
                );
                dpx_message(b" input str: \x00" as *const u8 as *const libc::c_char);
                dpx_message(b"<\x00" as *const u8 as *const libc::c_char);
                while save < p {
                    dpx_message(
                        b"%02x\x00" as *const u8 as *const libc::c_char,
                        *save as libc::c_int,
                    );
                    save = save.offset(1)
                }
                dpx_message(b">\n\x00" as *const u8 as *const libc::c_char);
                /*
                 * We know partial match found up to `count' bytes,
                 * but we will not use this information for the sake of simplicity.
                 */
                handle_undefined(cmap, inbuf, inbytesleft, outbuf, outbytesleft);
                return;
            }
        } else {
            match (*t.offset(c as isize)).flag & 0xfi32 {
                8 => {
                    dpx_warning(
                        b"Character mapped to .notdef found.\x00" as *const u8
                            as *const libc::c_char,
                    );
                }
                1 | 4 => {}
                2 => {
                    _tt_abort(
                        b"%s: CharName mapping not supported.\x00" as *const u8
                            as *const libc::c_char,
                        b"CMap\x00" as *const u8 as *const libc::c_char,
                    );
                }
                _ => {
                    _tt_abort(
                        b"%s: Unknown mapping type.\x00" as *const u8 as *const libc::c_char,
                        b"CMap\x00" as *const u8 as *const libc::c_char,
                    );
                }
            }
            /* continue */
            if *outbytesleft >= (*t.offset(c as isize)).len {
                memcpy(
                    *outbuf as *mut libc::c_void,
                    (*t.offset(c as isize)).code as *const libc::c_void,
                    (*t.offset(c as isize)).len,
                );
            } else {
                _tt_abort(
                    b"%s: Buffer overflow.\x00" as *const u8 as *const libc::c_char,
                    b"CMap\x00" as *const u8 as *const libc::c_char,
                );
            }
            *outbuf = (*outbuf).offset((*t.offset(c as isize)).len as isize);
            *outbytesleft = (*outbytesleft as libc::c_ulong)
                .wrapping_sub((*t.offset(c as isize)).len) as size_t
                as size_t;
            if !inbytesleft.is_null() {
                *inbytesleft =
                    (*inbytesleft as libc::c_ulong).wrapping_sub(count) as size_t as size_t
            }
            *inbuf = p
        }
    };
}
/*
 * For convenience, it does not do decoding to CIDs.
 */
#[no_mangle]
pub unsafe extern "C" fn CMap_decode(
    mut cmap: *mut CMap,
    mut inbuf: *mut *const libc::c_uchar,
    mut inbytesleft: *mut size_t,
    mut outbuf: *mut *mut libc::c_uchar,
    mut outbytesleft: *mut size_t,
) -> size_t {
    let mut count: size_t = 0;
    if !cmap.is_null() && !inbuf.is_null() && !outbuf.is_null() {
    } else {
        __assert_fail(b"cmap && inbuf && outbuf\x00" as *const u8 as
                          *const libc::c_char,
                      b"dpx-cmap.c\x00" as *const u8 as *const libc::c_char,
                      344i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 89],
                                                &[libc::c_char; 89]>(b"size_t CMap_decode(CMap *, const unsigned char **, size_t *, unsigned char **, size_t *)\x00")).as_ptr());
    }
    if !inbytesleft.is_null() && !outbytesleft.is_null() {
    } else {
        __assert_fail(b"inbytesleft && outbytesleft\x00" as *const u8 as
                          *const libc::c_char,
                      b"dpx-cmap.c\x00" as *const u8 as *const libc::c_char,
                      345i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 89],
                                                &[libc::c_char; 89]>(b"size_t CMap_decode(CMap *, const unsigned char **, size_t *, unsigned char **, size_t *)\x00")).as_ptr());
    }
    count = 0i32 as size_t;
    while *inbytesleft > 0i32 as libc::c_ulong && *outbytesleft > 0i32 as libc::c_ulong {
        CMap_decode_char(cmap, inbuf, inbytesleft, outbuf, outbytesleft);
        count = count.wrapping_add(1)
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_reverse_decode(mut cmap: *mut CMap, mut cid: CID) -> libc::c_int {
    let mut ch: libc::c_int = if !(*cmap).reverseMap.is_null() {
        *(*cmap).reverseMap.offset(cid as isize)
    } else {
        -1i32
    };
    if ch == 0i32 && !(*cmap).useCMap.is_null() {
        return CMap_reverse_decode((*cmap).useCMap, cid);
    }
    return ch;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_get_name(mut cmap: *mut CMap) -> *mut libc::c_char {
    if !cmap.is_null() {
    } else {
        __assert_fail(
            b"cmap\x00" as *const u8 as *const libc::c_char,
            b"dpx-cmap.c\x00" as *const u8 as *const libc::c_char,
            363i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                b"char *CMap_get_name(CMap *)\x00",
            ))
            .as_ptr(),
        );
    }
    return (*cmap).name;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_get_type(mut cmap: *mut CMap) -> libc::c_int {
    if !cmap.is_null() {
    } else {
        __assert_fail(
            b"cmap\x00" as *const u8 as *const libc::c_char,
            b"dpx-cmap.c\x00" as *const u8 as *const libc::c_char,
            370i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"int CMap_get_type(CMap *)\x00",
            ))
            .as_ptr(),
        );
    }
    return (*cmap).type_0;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_get_wmode(mut cmap: *mut CMap) -> libc::c_int {
    if !cmap.is_null() {
    } else {
        __assert_fail(
            b"cmap\x00" as *const u8 as *const libc::c_char,
            b"dpx-cmap.c\x00" as *const u8 as *const libc::c_char,
            377i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                b"int CMap_get_wmode(CMap *)\x00",
            ))
            .as_ptr(),
        );
    }
    return (*cmap).wmode;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_get_CIDSysInfo(mut cmap: *mut CMap) -> *mut CIDSysInfo {
    if !cmap.is_null() {
    } else {
        __assert_fail(
            b"cmap\x00" as *const u8 as *const libc::c_char,
            b"dpx-cmap.c\x00" as *const u8 as *const libc::c_char,
            384i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                b"CIDSysInfo *CMap_get_CIDSysInfo(CMap *)\x00",
            ))
            .as_ptr(),
        );
    }
    return (*cmap).CSI;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_set_name(mut cmap: *mut CMap, mut name: *const libc::c_char) {
    if !cmap.is_null() {
    } else {
        __assert_fail(
            b"cmap\x00" as *const u8 as *const libc::c_char,
            b"dpx-cmap.c\x00" as *const u8 as *const libc::c_char,
            391i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                b"void CMap_set_name(CMap *, const char *)\x00",
            ))
            .as_ptr(),
        );
    }
    free((*cmap).name as *mut libc::c_void);
    (*cmap).name = new(
        (strlen(name).wrapping_add(1i32 as libc::c_ulong) as uint32_t as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            as uint32_t,
    ) as *mut libc::c_char;
    strcpy((*cmap).name, name);
}
#[no_mangle]
pub unsafe extern "C" fn CMap_set_type(mut cmap: *mut CMap, mut type_0: libc::c_int) {
    if !cmap.is_null() {
    } else {
        __assert_fail(
            b"cmap\x00" as *const u8 as *const libc::c_char,
            b"dpx-cmap.c\x00" as *const u8 as *const libc::c_char,
            400i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                b"void CMap_set_type(CMap *, int)\x00",
            ))
            .as_ptr(),
        );
    }
    (*cmap).type_0 = type_0;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_set_wmode(mut cmap: *mut CMap, mut wmode: libc::c_int) {
    if !cmap.is_null() {
    } else {
        __assert_fail(
            b"cmap\x00" as *const u8 as *const libc::c_char,
            b"dpx-cmap.c\x00" as *const u8 as *const libc::c_char,
            407i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                b"void CMap_set_wmode(CMap *, int)\x00",
            ))
            .as_ptr(),
        );
    }
    (*cmap).wmode = wmode;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_set_CIDSysInfo(mut cmap: *mut CMap, mut csi: *const CIDSysInfo) {
    if !cmap.is_null() {
    } else {
        __assert_fail(
            b"cmap\x00" as *const u8 as *const libc::c_char,
            b"dpx-cmap.c\x00" as *const u8 as *const libc::c_char,
            414i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 53], &[libc::c_char; 53]>(
                b"void CMap_set_CIDSysInfo(CMap *, const CIDSysInfo *)\x00",
            ))
            .as_ptr(),
        );
    }
    if !(*cmap).CSI.is_null() {
        free((*(*cmap).CSI).registry as *mut libc::c_void);
        free((*(*cmap).CSI).ordering as *mut libc::c_void);
        free((*cmap).CSI as *mut libc::c_void);
    }
    if !csi.is_null() && !(*csi).registry.is_null() && !(*csi).ordering.is_null() {
        (*cmap).CSI = new((1i32 as uint32_t as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<CIDSysInfo>() as libc::c_ulong)
            as uint32_t) as *mut CIDSysInfo;
        (*(*cmap).CSI).registry = new((strlen((*csi).registry).wrapping_add(1i32 as libc::c_ulong)
            as uint32_t as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            as uint32_t) as *mut libc::c_char;
        strcpy((*(*cmap).CSI).registry, (*csi).registry);
        (*(*cmap).CSI).ordering = new((strlen((*csi).ordering).wrapping_add(1i32 as libc::c_ulong)
            as uint32_t as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            as uint32_t) as *mut libc::c_char;
        strcpy((*(*cmap).CSI).ordering, (*csi).ordering);
        (*(*cmap).CSI).supplement = (*csi).supplement
    } else {
        dpx_warning(b"Invalid CIDSystemInfo.\x00" as *const u8 as *const libc::c_char);
        (*cmap).CSI = 0 as *mut CIDSysInfo
    };
}
/*
 * Can have muliple entry ?
 */
#[no_mangle]
pub unsafe extern "C" fn CMap_set_usecmap(mut cmap: *mut CMap, mut ucmap: *mut CMap) {
    let mut i: libc::c_uint = 0; /* Maybe if (!ucmap) _tt_abort() is better for this. */
    if !cmap.is_null() {
    } else {
        __assert_fail(
            b"cmap\x00" as *const u8 as *const libc::c_char,
            b"dpx-cmap.c\x00" as *const u8 as *const libc::c_char,
            443i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                b"void CMap_set_usecmap(CMap *, CMap *)\x00",
            ))
            .as_ptr(),
        );
    }
    if !ucmap.is_null() {
    } else {
        __assert_fail(
            b"ucmap\x00" as *const u8 as *const libc::c_char,
            b"dpx-cmap.c\x00" as *const u8 as *const libc::c_char,
            444i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                b"void CMap_set_usecmap(CMap *, CMap *)\x00",
            ))
            .as_ptr(),
        );
    }
    if cmap == ucmap {
        _tt_abort(
            b"%s: Identical CMap object cannot be used for usecmap CMap: 0x%p=0x%p\x00" as *const u8
                as *const libc::c_char,
            b"CMap\x00" as *const u8 as *const libc::c_char,
            cmap,
            ucmap,
        );
    }
    /* Check if ucmap have neccesary information. */
    if !CMap_is_valid(ucmap) {
        _tt_abort(
            b"%s: Invalid CMap.\x00" as *const u8 as *const libc::c_char,
            b"CMap\x00" as *const u8 as *const libc::c_char,
        );
    }
    /*
     *  CMapName of cmap can be undefined when usecmap is executed in CMap parsing.
     *  And it is also possible CSI is not defined at that time.
     */
    if streq_ptr((*cmap).name, (*ucmap).name) {
        _tt_abort(
            b"%s: CMap refering itself not allowed: CMap %s --> %s\x00" as *const u8
                as *const libc::c_char,
            b"CMap\x00" as *const u8 as *const libc::c_char,
            (*cmap).name,
            (*ucmap).name,
        );
    }
    if !(*cmap).CSI.is_null()
        && !(*(*cmap).CSI).registry.is_null()
        && !(*(*cmap).CSI).ordering.is_null()
    {
        if strcmp((*(*cmap).CSI).registry, (*(*ucmap).CSI).registry) != 0
            || strcmp((*(*cmap).CSI).ordering, (*(*ucmap).CSI).ordering) != 0
        {
            _tt_abort(
                b"%s: CMap %s required by %s have different CSI.\x00" as *const u8
                    as *const libc::c_char,
                b"CMap\x00" as *const u8 as *const libc::c_char,
                CMap_get_name(cmap),
                CMap_get_name(ucmap),
            );
        }
    }
    /* We must copy codespaceranges. */
    i = 0i32 as libc::c_uint;
    while i < (*ucmap).codespace.num {
        let mut csr: *mut rangeDef = (*ucmap).codespace.ranges.offset(i as isize);
        CMap_add_codespacerange(cmap, (*csr).codeLo, (*csr).codeHi, (*csr).dim);
        i = i.wrapping_add(1)
    }
    (*cmap).useCMap = ucmap;
}
/* Test the validity of character c. */
unsafe extern "C" fn CMap_match_codespace(
    mut cmap: *mut CMap,
    mut c: *const libc::c_uchar,
    mut dim: size_t,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut pos: libc::c_uint = 0;
    if !cmap.is_null() {
    } else {
        __assert_fail(
            b"cmap\x00" as *const u8 as *const libc::c_char,
            b"dpx-cmap.c\x00" as *const u8 as *const libc::c_char,
            484i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 64], &[libc::c_char; 64]>(
                b"int CMap_match_codespace(CMap *, const unsigned char *, size_t)\x00",
            ))
            .as_ptr(),
        );
    }
    i = 0i32 as libc::c_uint;
    while i < (*cmap).codespace.num {
        let mut csr: *mut rangeDef = (*cmap).codespace.ranges.offset(i as isize);
        if !((*csr).dim != dim) {
            pos = 0i32 as libc::c_uint;
            while (pos as libc::c_ulong) < dim {
                if *c.offset(pos as isize) as libc::c_int
                    > *(*csr).codeHi.offset(pos as isize) as libc::c_int
                    || (*c.offset(pos as isize) as libc::c_int)
                        < *(*csr).codeLo.offset(pos as isize) as libc::c_int
                {
                    break;
                }
                pos = pos.wrapping_add(1)
            }
            if pos as libc::c_ulong == dim {
                return 0i32;
            }
        }
        i = i.wrapping_add(1)
        /* Valid */
    }
    return -1i32;
    /* Invalid */
}
/*
 * No overlapping codespace ranges are allowed, otherwise mapping is ambiguous.
 */
#[no_mangle]
pub unsafe extern "C" fn CMap_add_codespacerange(
    mut cmap: *mut CMap,
    mut codelo: *const libc::c_uchar,
    mut codehi: *const libc::c_uchar,
    mut dim: size_t,
) -> libc::c_int {
    let mut csr: *mut rangeDef = 0 as *mut rangeDef;
    let mut i: libc::c_uint = 0;
    if !cmap.is_null() && dim > 0i32 as libc::c_ulong {
    } else {
        __assert_fail(b"cmap && dim > 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"dpx-cmap.c\x00" as *const u8 as *const libc::c_char,
                      510i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 90],
                                                &[libc::c_char; 90]>(b"int CMap_add_codespacerange(CMap *, const unsigned char *, const unsigned char *, size_t)\x00")).as_ptr());
    }
    i = 0i32 as libc::c_uint;
    while i < (*cmap).codespace.num {
        let mut j: size_t = 0;
        let mut overlap: bool = 1i32 != 0;
        csr = (*cmap).codespace.ranges.offset(i as isize);
        j = 0i32 as size_t;
        while j < (if (*csr).dim < dim { (*csr).dim } else { dim }) && overlap as libc::c_int != 0 {
            if *codelo.offset(j as isize) as libc::c_int
                >= *(*csr).codeLo.offset(j as isize) as libc::c_int
                && *codelo.offset(j as isize) as libc::c_int
                    <= *(*csr).codeHi.offset(j as isize) as libc::c_int
                || *codehi.offset(j as isize) as libc::c_int
                    >= *(*csr).codeLo.offset(j as isize) as libc::c_int
                    && *codehi.offset(j as isize) as libc::c_int
                        <= *(*csr).codeHi.offset(j as isize) as libc::c_int
            {
                overlap = 1i32 != 0
            } else {
                overlap = 0i32 != 0
            }
            j = j.wrapping_add(1)
        }
        if overlap {
            dpx_warning(
                b"Overlapping codespace found. (ingored)\x00" as *const u8 as *const libc::c_char,
            );
            return -1i32;
        }
        i = i.wrapping_add(1)
    }
    if dim < (*cmap).profile.minBytesIn {
        (*cmap).profile.minBytesIn = dim
    }
    if dim > (*cmap).profile.maxBytesIn {
        (*cmap).profile.maxBytesIn = dim
    }
    if (*cmap).codespace.num.wrapping_add(1i32 as libc::c_uint) > (*cmap).codespace.max {
        (*cmap).codespace.max = (*cmap).codespace.max.wrapping_add(10i32 as libc::c_uint);
        (*cmap).codespace.ranges = renew(
            (*cmap).codespace.ranges as *mut libc::c_void,
            ((*cmap).codespace.max as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<rangeDef>() as libc::c_ulong)
                as uint32_t,
        ) as *mut rangeDef
    }
    csr = (*cmap)
        .codespace
        .ranges
        .offset((*cmap).codespace.num as isize);
    (*csr).dim = dim;
    (*csr).codeHi = get_mem(cmap, dim as libc::c_int);
    (*csr).codeLo = get_mem(cmap, dim as libc::c_int);
    memcpy(
        (*csr).codeHi as *mut libc::c_void,
        codehi as *const libc::c_void,
        dim,
    );
    memcpy(
        (*csr).codeLo as *mut libc::c_void,
        codelo as *const libc::c_void,
        dim,
    );
    (*cmap).codespace.num = (*cmap).codespace.num.wrapping_add(1);
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_add_notdefchar(
    mut cmap: *mut CMap,
    mut src: *const libc::c_uchar,
    mut srcdim: size_t,
    mut dst: CID,
) -> libc::c_int {
    return CMap_add_notdefrange(cmap, src, src, srcdim, dst);
}
#[no_mangle]
pub unsafe extern "C" fn CMap_add_notdefrange(
    mut cmap: *mut CMap,
    mut srclo: *const libc::c_uchar,
    mut srchi: *const libc::c_uchar,
    mut srcdim: size_t,
    mut dst: CID,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut cur: *mut mapDef = 0 as *mut mapDef;
    if !cmap.is_null() {
    } else {
        __assert_fail(b"cmap\x00" as *const u8 as *const libc::c_char,
                      b"dpx-cmap.c\x00" as *const u8 as *const libc::c_char,
                      564i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 92],
                                                &[libc::c_char; 92]>(b"int CMap_add_notdefrange(CMap *, const unsigned char *, const unsigned char *, size_t, CID)\x00")).as_ptr());
    }
    /* dst not used here */
    /* FIXME */
    if check_range(
        cmap,
        srclo,
        srchi,
        srcdim,
        &mut dst as *mut CID as *const libc::c_uchar,
        2i32 as size_t,
    ) < 0i32
    {
        return -1i32;
    }
    if (*cmap).mapTbl.is_null() {
        (*cmap).mapTbl = mapDef_new()
    }
    cur = (*cmap).mapTbl;
    if locate_tbl(&mut cur, srclo, srcdim as libc::c_int) < 0i32 {
        return -1i32;
    }
    c = *srclo.offset(srcdim.wrapping_sub(1i32 as libc::c_ulong) as isize) as libc::c_int;
    while c <= *srchi.offset(srcdim.wrapping_sub(1i32 as libc::c_ulong) as isize) as libc::c_int {
        if if (*cur.offset(c as isize)).flag & 0xfi32 != 0i32 {
            1i32
        } else {
            0i32
        } != 0
        {
            if __silent == 0 {
                dpx_warning(
                    b"Trying to redefine already defined code mapping. (ignored)\x00" as *const u8
                        as *const libc::c_char,
                );
            }
        } else {
            (*cur.offset(c as isize)).flag = 0i32 | 1i32 << 3i32;
            let ref mut fresh1 = (*cur.offset(c as isize)).code;
            *fresh1 = get_mem(cmap, 2i32);
            (*cur.offset(c as isize)).len = 2i32 as size_t;
            *(*cur.offset(c as isize)).code.offset(0) =
                (dst as libc::c_int >> 8i32) as libc::c_uchar;
            *(*cur.offset(c as isize)).code.offset(1) =
                (dst as libc::c_int & 0xffi32) as libc::c_uchar
        }
        c += 1
        /* Do not do dst++ for notdefrange  */
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_add_bfchar(
    mut cmap: *mut CMap,
    mut src: *const libc::c_uchar,
    mut srcdim: size_t,
    mut dst: *const libc::c_uchar,
    mut dstdim: size_t,
) -> libc::c_int {
    return CMap_add_bfrange(cmap, src, src, srcdim, dst, dstdim);
}
#[no_mangle]
pub unsafe extern "C" fn CMap_add_bfrange(
    mut cmap: *mut CMap,
    mut srclo: *const libc::c_uchar,
    mut srchi: *const libc::c_uchar,
    mut srcdim: size_t,
    mut base: *const libc::c_uchar,
    mut dstdim: size_t,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut last_byte: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut cur: *mut mapDef = 0 as *mut mapDef;
    if !cmap.is_null() {
    } else {
        __assert_fail(b"cmap\x00" as *const u8 as *const libc::c_char,
                      b"dpx-cmap.c\x00" as *const u8 as *const libc::c_char,
                      610i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 114],
                                                &[libc::c_char; 114]>(b"int CMap_add_bfrange(CMap *, const unsigned char *, const unsigned char *, size_t, const unsigned char *, size_t)\x00")).as_ptr());
    }
    if check_range(cmap, srclo, srchi, srcdim, base, dstdim) < 0i32 {
        return -1i32;
    }
    if (*cmap).mapTbl.is_null() {
        (*cmap).mapTbl = mapDef_new()
    }
    cur = (*cmap).mapTbl;
    if locate_tbl(&mut cur, srclo, srcdim as libc::c_int) < 0i32 {
        return -1i32;
    }
    c = *srclo.offset(srcdim.wrapping_sub(1i32 as libc::c_ulong) as isize) as libc::c_int;
    while c <= *srchi.offset(srcdim.wrapping_sub(1i32 as libc::c_ulong) as isize) as libc::c_int {
        /* According to 5014.CIDFont_Spec.pdf (p.52),
         * Code mappings (unlike codespace ranges) may overlap,
         * but succeeding maps superceded preceding maps.
         * (reported and patched by Luo Jie on 2007/12/2)
         */
        if (if (*cur.offset(c as isize)).flag & 0xfi32 != 0i32 {
            1i32
        } else {
            0i32
        }) == 0
            || (*cur.offset(c as isize)).len < dstdim
        {
            (*cur.offset(c as isize)).flag = 0i32 | 1i32 << 2i32;
            let ref mut fresh2 = (*cur.offset(c as isize)).code;
            *fresh2 = get_mem(cmap, dstdim as libc::c_int)
        }
        /*
         * We assume restriction to code ranges also applied here.
         * Addition <00FF> + 1 is undefined.
         *
         * Changed on 2004-03-20:
         *
         *  Should be treated as <0100> in Acrobat's "ToUnicode" CMap.
         */
        (*cur.offset(c as isize)).len = dstdim;
        memcpy(
            (*cur.offset(c as isize)).code as *mut libc::c_void,
            base as *const libc::c_void,
            dstdim,
        );
        last_byte = c - *srclo.offset(srcdim.wrapping_sub(1i32 as libc::c_ulong) as isize)
            as libc::c_int
            + *base.offset(dstdim.wrapping_sub(1i32 as libc::c_ulong) as isize) as libc::c_int;
        *(*cur.offset(c as isize))
            .code
            .offset(dstdim.wrapping_sub(1i32 as libc::c_ulong) as isize) =
            (last_byte & 0xffi32) as libc::c_uchar;
        i = dstdim.wrapping_sub(2i32 as libc::c_ulong) as libc::c_int;
        while i >= 0i32 && last_byte > 255i32 {
            last_byte = *(*cur.offset(c as isize)).code.offset(i as isize) as libc::c_int + 1i32;
            *(*cur.offset(c as isize)).code.offset(i as isize) =
                (last_byte & 0xffi32) as libc::c_uchar;
            i -= 1
        }
        c += 1
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_add_cidchar(
    mut cmap: *mut CMap,
    mut src: *const libc::c_uchar,
    mut srcdim: size_t,
    mut dst: CID,
) -> libc::c_int {
    return CMap_add_cidrange(cmap, src, src, srcdim, dst);
}
#[no_mangle]
pub unsafe extern "C" fn CMap_add_cidrange(
    mut cmap: *mut CMap,
    mut srclo: *const libc::c_uchar,
    mut srchi: *const libc::c_uchar,
    mut srcdim: size_t,
    mut base: CID,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut c: size_t = 0;
    let mut v: size_t = 0;
    let mut cur: *mut mapDef = 0 as *mut mapDef;
    if !cmap.is_null() {
    } else {
        __assert_fail(b"cmap\x00" as *const u8 as *const libc::c_char,
                      b"dpx-cmap.c\x00" as *const u8 as *const libc::c_char,
                      666i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 89],
                                                &[libc::c_char; 89]>(b"int CMap_add_cidrange(CMap *, const unsigned char *, const unsigned char *, size_t, CID)\x00")).as_ptr());
    }
    /* base not used here */
    if check_range(
        cmap,
        srclo,
        srchi,
        srcdim,
        &mut base as *mut CID as *const libc::c_uchar,
        2i32 as size_t,
    ) < 0i32
    {
        /* FIXME */
        return -1i32;
    }
    if (*cmap).mapTbl.is_null() {
        (*cmap).mapTbl = mapDef_new()
    }
    cur = (*cmap).mapTbl;
    if locate_tbl(&mut cur, srclo, srcdim as libc::c_int) < 0i32 {
        return -1i32;
    }
    v = 0i32 as size_t;
    i = 0i32 as size_t;
    while i < srcdim.wrapping_sub(1i32 as libc::c_ulong) {
        v = (v << 8i32).wrapping_add(*srclo.offset(i as isize) as libc::c_ulong);
        i = i.wrapping_add(1)
    }
    *(*cmap).reverseMap.offset(base as isize) = v as libc::c_int;
    c = *srclo.offset(srcdim.wrapping_sub(1i32 as libc::c_ulong) as isize) as size_t;
    while c <= *srchi.offset(srcdim.wrapping_sub(1i32 as libc::c_ulong) as isize) as libc::c_ulong {
        if (*cur.offset(c as isize)).flag != 0i32 {
            if __silent == 0 {
                dpx_warning(
                    b"Trying to redefine already defined CID mapping. (ignored)\x00" as *const u8
                        as *const libc::c_char,
                );
            }
        } else {
            (*cur.offset(c as isize)).flag = 0i32 | 1i32 << 0i32;
            (*cur.offset(c as isize)).len = 2i32 as size_t;
            let ref mut fresh3 = (*cur.offset(c as isize)).code;
            *fresh3 = get_mem(cmap, 2i32);
            *(*cur.offset(c as isize)).code.offset(0) =
                (base as libc::c_int >> 8i32) as libc::c_uchar;
            *(*cur.offset(c as isize)).code.offset(1) =
                (base as libc::c_int & 0xffi32) as libc::c_uchar;
            *(*cmap).reverseMap.offset(base as isize) = (v << 8i32).wrapping_add(c) as libc::c_int
        }
        if base as libc::c_int >= 65535i32 {
            dpx_warning(b"CID number too large.\x00" as *const u8 as *const libc::c_char);
        }
        base = base.wrapping_add(1);
        c = c.wrapping_add(1)
    }
    return 0i32;
}
unsafe extern "C" fn mapDef_release(mut t: *mut mapDef) {
    let mut c: libc::c_int = 0;
    if !t.is_null() {
    } else {
        __assert_fail(
            b"t\x00" as *const u8 as *const libc::c_char,
            b"dpx-cmap.c\x00" as *const u8 as *const libc::c_char,
            709i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 30], &[libc::c_char; 30]>(
                b"void mapDef_release(mapDef *)\x00",
            ))
            .as_ptr(),
        );
    }
    c = 0i32;
    while c < 256i32 {
        if (*t.offset(c as isize)).flag & 1i32 << 4i32 != 0 {
            mapDef_release((*t.offset(c as isize)).next);
        }
        c += 1
    }
    free(t as *mut libc::c_void);
}
unsafe extern "C" fn mapDef_new() -> *mut mapDef {
    let mut t: *mut mapDef = 0 as *mut mapDef;
    let mut c: libc::c_int = 0;
    t = new((256i32 as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<mapDef>() as libc::c_ulong) as uint32_t)
        as *mut mapDef;
    c = 0i32;
    while c < 256i32 {
        (*t.offset(c as isize)).flag = 0i32 | 0i32;
        let ref mut fresh4 = (*t.offset(c as isize)).code;
        *fresh4 = 0 as *mut libc::c_uchar;
        let ref mut fresh5 = (*t.offset(c as isize)).next;
        *fresh5 = 0 as *mut mapDef;
        c += 1
    }
    return t;
}
unsafe extern "C" fn get_mem(mut cmap: *mut CMap, mut size: libc::c_int) -> *mut libc::c_uchar {
    let mut map: *mut mapData = 0 as *mut mapData;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if !cmap.is_null() && !(*cmap).mapData.is_null() && size >= 0i32 {
    } else {
        __assert_fail(
            b"cmap && cmap->mapData && size >= 0\x00" as *const u8 as *const libc::c_char,
            b"dpx-cmap.c\x00" as *const u8 as *const libc::c_char,
            739i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                b"unsigned char *get_mem(CMap *, int)\x00",
            ))
            .as_ptr(),
        );
    }
    map = (*cmap).mapData;
    if (*map).pos + size >= 4096i32 {
        let mut prev: *mut mapData = map;
        map = new((1i32 as uint32_t as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mapData>() as libc::c_ulong)
            as uint32_t) as *mut mapData;
        (*map).data = new((4096i32 as uint32_t as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            as uint32_t) as *mut libc::c_uchar;
        (*map).prev = prev;
        (*map).pos = 0i32;
        (*cmap).mapData = map
    }
    p = (*map).data.offset((*map).pos as isize);
    (*map).pos += size;
    return p;
}
unsafe extern "C" fn locate_tbl(
    mut cur: *mut *mut mapDef,
    mut code: *const libc::c_uchar,
    mut dim: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    if !cur.is_null() && !(*cur).is_null() {
    } else {
        __assert_fail(
            b"cur && *cur\x00" as *const u8 as *const libc::c_char,
            b"dpx-cmap.c\x00" as *const u8 as *const libc::c_char,
            760i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 54], &[libc::c_char; 54]>(
                b"int locate_tbl(mapDef **, const unsigned char *, int)\x00",
            ))
            .as_ptr(),
        );
    }
    i = 0i32;
    while i < dim - 1i32 {
        c = *code.offset(i as isize) as libc::c_int;
        if if (*(*cur).offset(c as isize)).flag & 0xfi32 != 0i32 {
            1i32
        } else {
            0i32
        } != 0
        {
            dpx_warning(b"Ambiguous CMap entry.\x00" as *const u8 as *const libc::c_char);
            return -1i32;
        }
        if (*(*cur).offset(c as isize)).next.is_null() {
            /* create new node */
            let ref mut fresh6 = (*(*cur).offset(c as isize)).next;
            *fresh6 = mapDef_new()
        }
        (*(*cur).offset(c as isize)).flag |= 1i32 << 4i32;
        *cur = (*(*cur).offset(c as isize)).next;
        i += 1
    }
    return 0i32;
}
/* Private funcs. */
/*
 * Guess how many bytes consumed as a `single' character:
 * Substring of length bytesconsumed bytes of input string is interpreted as
 * a `single' character by CMap_decode().
 */
unsafe extern "C" fn bytes_consumed(
    mut cmap: *mut CMap,
    mut instr: *const libc::c_uchar,
    mut inbytes: size_t,
) -> size_t {
    let mut i: size_t = 0;
    let mut pos: size_t = 0;
    let mut longest: size_t = 0i32 as size_t;
    let mut bytesconsumed: size_t = 0;
    if !cmap.is_null() {
    } else {
        __assert_fail(
            b"cmap\x00" as *const u8 as *const libc::c_char,
            b"dpx-cmap.c\x00" as *const u8 as *const libc::c_char,
            786i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 61], &[libc::c_char; 61]>(
                b"size_t bytes_consumed(CMap *, const unsigned char *, size_t)\x00",
            ))
            .as_ptr(),
        );
    }
    i = 0i32 as size_t;
    while i < (*cmap).codespace.num as libc::c_ulong {
        let mut csr: *mut rangeDef = (*cmap).codespace.ranges.offset(i as isize);
        pos = 0i32 as size_t;
        while pos
            < (if (*csr).dim < inbytes {
                (*csr).dim
            } else {
                inbytes
            })
        {
            if *instr.offset(pos as isize) as libc::c_int
                > *(*csr).codeHi.offset(pos as isize) as libc::c_int
                || (*instr.offset(pos as isize) as libc::c_int)
                    < *(*csr).codeLo.offset(pos as isize) as libc::c_int
            {
                break;
            }
            pos = pos.wrapping_add(1)
        }
        if pos == (*csr).dim {
            /* part of instr is totally valid in this codespace. */
            return (*csr).dim;
        }
        if pos > longest {
            longest = pos
        }
        i = i.wrapping_add(1)
    }
    if i == (*cmap).codespace.num as libc::c_ulong {
        /* No matching at all */
        bytesconsumed = (*cmap).profile.minBytesIn
    } else {
        bytesconsumed = (*cmap).profile.maxBytesIn;
        i = 0i32 as size_t;
        while i < (*cmap).codespace.num as libc::c_ulong {
            let mut csr_0: *mut rangeDef = (*cmap).codespace.ranges.offset(i as isize);
            if (*csr_0).dim > longest && (*csr_0).dim < bytesconsumed {
                bytesconsumed = (*csr_0).dim
            }
            i = i.wrapping_add(1)
        }
    }
    return bytesconsumed;
}
unsafe extern "C" fn check_range(
    mut cmap: *mut CMap,
    mut srclo: *const libc::c_uchar,
    mut srchi: *const libc::c_uchar,
    mut srcdim: size_t,
    mut dst: *const libc::c_uchar,
    mut dstdim: size_t,
) -> libc::c_int {
    if srcdim < 1i32 as libc::c_ulong
        || dstdim < 1i32 as libc::c_ulong
        || (srclo.is_null() || srchi.is_null() || dst.is_null())
        || memcmp(
            srclo as *const libc::c_void,
            srchi as *const libc::c_void,
            srcdim.wrapping_sub(1i32 as libc::c_ulong),
        ) != 0
        || *srclo.offset(srcdim.wrapping_sub(1i32 as libc::c_ulong) as isize) as libc::c_int
            > *srchi.offset(srcdim.wrapping_sub(1i32 as libc::c_ulong) as isize) as libc::c_int
    {
        dpx_warning(
            b"Invalid CMap mapping entry. (ignored)\x00" as *const u8 as *const libc::c_char,
        );
        return -1i32;
    }
    if CMap_match_codespace(cmap, srclo, srcdim) < 0i32
        || CMap_match_codespace(cmap, srchi, srcdim) < 0i32
    {
        dpx_warning(
            b"Invalid CMap mapping entry. (ignored)\x00" as *const u8 as *const libc::c_char,
        );
        return -1i32;
    }
    if srcdim < (*cmap).profile.minBytesIn {
        (*cmap).profile.minBytesIn = srcdim
    }
    if srcdim > (*cmap).profile.maxBytesIn {
        (*cmap).profile.maxBytesIn = srcdim
    }
    if dstdim < (*cmap).profile.minBytesOut {
        (*cmap).profile.minBytesOut = dstdim
    }
    if dstdim > (*cmap).profile.maxBytesOut {
        (*cmap).profile.maxBytesOut = dstdim
    }
    return 0i32;
}
static mut __cache: *mut CMap_cache = 0 as *const CMap_cache as *mut CMap_cache;
#[no_mangle]
pub unsafe extern "C" fn CMap_cache_init() {
    static mut range_min: [libc::c_uchar; 2] = [0i32 as libc::c_uchar, 0i32 as libc::c_uchar];
    static mut range_max: [libc::c_uchar; 2] = [0xffi32 as libc::c_uchar, 0xffi32 as libc::c_uchar];
    if !__cache.is_null() {
        _tt_abort(
            b"%s: Already initialized.\x00" as *const u8 as *const libc::c_char,
            b"CMap\x00" as *const u8 as *const libc::c_char,
        );
    }
    __cache = new((1i32 as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<CMap_cache>() as libc::c_ulong)
        as uint32_t) as *mut CMap_cache;
    (*__cache).max = 16u32 as libc::c_int;
    (*__cache).cmaps = new(((*__cache).max as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<*mut CMap>() as libc::c_ulong)
        as uint32_t) as *mut *mut CMap;
    (*__cache).num = 0i32;
    /* Create Identity mapping */
    let ref mut fresh7 = *(*__cache).cmaps.offset(0);
    *fresh7 = CMap_new();
    CMap_set_name(
        *(*__cache).cmaps.offset(0),
        b"Identity-H\x00" as *const u8 as *const libc::c_char,
    );
    CMap_set_type(*(*__cache).cmaps.offset(0), 0i32);
    CMap_set_wmode(*(*__cache).cmaps.offset(0), 0i32);
    CMap_set_CIDSysInfo(*(*__cache).cmaps.offset(0), &mut CSI_IDENTITY);
    CMap_add_codespacerange(
        *(*__cache).cmaps.offset(0),
        range_min.as_mut_ptr(),
        range_max.as_mut_ptr(),
        2i32 as size_t,
    );
    let ref mut fresh8 = *(*__cache).cmaps.offset(1);
    *fresh8 = CMap_new();
    CMap_set_name(
        *(*__cache).cmaps.offset(1),
        b"Identity-V\x00" as *const u8 as *const libc::c_char,
    );
    CMap_set_type(*(*__cache).cmaps.offset(1), 0i32);
    CMap_set_wmode(*(*__cache).cmaps.offset(1), 1i32);
    CMap_set_CIDSysInfo(*(*__cache).cmaps.offset(1), &mut CSI_IDENTITY);
    CMap_add_codespacerange(
        *(*__cache).cmaps.offset(1),
        range_min.as_mut_ptr(),
        range_max.as_mut_ptr(),
        2i32 as size_t,
    );
    (*__cache).num += 2i32;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_cache_get(mut id: libc::c_int) -> *mut CMap {
    if __cache.is_null() {
        _tt_abort(
            b"%s: CMap cache not initialized.\x00" as *const u8 as *const libc::c_char,
            b"CMap\x00" as *const u8 as *const libc::c_char,
        );
    }
    if id < 0i32 || id >= (*__cache).num {
        _tt_abort(
            b"Invalid CMap ID %d\x00" as *const u8 as *const libc::c_char,
            id,
        );
    }
    return *(*__cache).cmaps.offset(id as isize);
}
#[no_mangle]
pub unsafe extern "C" fn CMap_cache_find(mut cmap_name: *const libc::c_char) -> libc::c_int {
    let mut id: libc::c_int = 0i32;
    let mut handle: rust_input_handle_t = 0 as *mut libc::c_void;
    if __cache.is_null() {
        CMap_cache_init();
    }
    if !__cache.is_null() {
    } else {
        __assert_fail(
            b"__cache\x00" as *const u8 as *const libc::c_char,
            b"dpx-cmap.c\x00" as *const u8 as *const libc::c_char,
            914i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                b"int CMap_cache_find(const char *)\x00",
            ))
            .as_ptr(),
        );
    }
    id = 0i32;
    while id < (*__cache).num {
        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
        /* CMapName may be undefined when processing usecmap. */
        name = CMap_get_name(*(*__cache).cmaps.offset(id as isize));
        if !name.is_null() && streq_ptr(cmap_name, name) as libc::c_int != 0 {
            return id;
        }
        id += 1
    }
    handle = ttstub_input_open(cmap_name, TTIF_CMAP, 0i32);
    if handle.is_null() {
        return -1i32;
    }
    if CMap_parse_check_sig(handle) < 0i32 {
        ttstub_input_close(handle);
        return -1i32;
    }
    if __verbose != 0 {
        dpx_message(
            b"(CMap:%s\x00" as *const u8 as *const libc::c_char,
            cmap_name,
        );
    }
    if (*__cache).num >= (*__cache).max {
        (*__cache).max =
            ((*__cache).max as libc::c_uint).wrapping_add(16u32) as libc::c_int as libc::c_int;
        (*__cache).cmaps = renew(
            (*__cache).cmaps as *mut libc::c_void,
            ((*__cache).max as uint32_t as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut CMap>() as libc::c_ulong)
                as uint32_t,
        ) as *mut *mut CMap
    }
    id = (*__cache).num;
    (*__cache).num += 1;
    let ref mut fresh9 = *(*__cache).cmaps.offset(id as isize);
    *fresh9 = CMap_new();
    if CMap_parse(*(*__cache).cmaps.offset(id as isize), handle) < 0i32 {
        _tt_abort(
            b"%s: Parsing CMap file failed.\x00" as *const u8 as *const libc::c_char,
            b"CMap\x00" as *const u8 as *const libc::c_char,
        );
    }
    ttstub_input_close(handle);
    if __verbose != 0 {
        dpx_message(b")\x00" as *const u8 as *const libc::c_char);
    }
    return id;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_cache_add(mut cmap: *mut CMap) -> libc::c_int {
    let mut id: libc::c_int = 0;
    let mut cmap_name0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmap_name1: *mut libc::c_char = 0 as *mut libc::c_char;
    if !CMap_is_valid(cmap) {
        _tt_abort(
            b"%s: Invalid CMap.\x00" as *const u8 as *const libc::c_char,
            b"CMap\x00" as *const u8 as *const libc::c_char,
        );
    }
    id = 0i32;
    while id < (*__cache).num {
        cmap_name0 = CMap_get_name(cmap);
        cmap_name1 = CMap_get_name(*(*__cache).cmaps.offset(id as isize));
        if streq_ptr(cmap_name0, cmap_name1) {
            _tt_abort(
                b"%s: CMap \"%s\" already defined.\x00" as *const u8 as *const libc::c_char,
                b"CMap\x00" as *const u8 as *const libc::c_char,
                cmap_name0,
            );
        }
        id += 1
    }
    if (*__cache).num >= (*__cache).max {
        (*__cache).max =
            ((*__cache).max as libc::c_uint).wrapping_add(16u32) as libc::c_int as libc::c_int;
        (*__cache).cmaps = renew(
            (*__cache).cmaps as *mut libc::c_void,
            ((*__cache).max as uint32_t as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut CMap>() as libc::c_ulong)
                as uint32_t,
        ) as *mut *mut CMap
    }
    id = (*__cache).num;
    (*__cache).num += 1;
    let ref mut fresh10 = *(*__cache).cmaps.offset(id as isize);
    *fresh10 = cmap;
    return id;
}
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software
    Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
*/
/* Limits */
/*
 * TYPE_IDENTITY and TYPE_CID_TO_CODE is not defined in the CMap spec.
 */
/* ************************* CMAP_MAIN **************************/
/* charName not supported */
#[no_mangle]
pub unsafe extern "C" fn CMap_cache_close() {
    if !__cache.is_null() {
        let mut id: libc::c_int = 0;
        id = 0i32;
        while id < (*__cache).num {
            CMap_release(*(*__cache).cmaps.offset(id as isize));
            id += 1
        }
        free((*__cache).cmaps as *mut libc::c_void);
        __cache = mfree(__cache as *mut libc::c_void) as *mut CMap_cache
    };
}