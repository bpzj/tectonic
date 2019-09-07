#![feature(extern_types)]
#![feature(ptr_wrapping_offset_from)]
#![feature(c_variadic)]

mod bibtex;
mod core_bridge;
mod core_memory;
mod dpx_agl;
mod dpx_bmpimage;
mod dpx_cff;
mod dpx_cff_dict;
mod dpx_cid;
mod dpx_cidtype0;
mod dpx_cidtype2;
mod dpx_cmap;
mod dpx_cmap_read;
mod dpx_cmap_write;
mod dpx_cs_type2;
mod dpx_dpxconf;
//mod dpx_dpxcrypt;
mod dpx_dpxfile;
mod dpx_dpxutil;
mod dpx_dvi;
mod dpx_dvipdfmx;
mod dpx_epdf;
mod dpx_error;
mod dpx_fontmap;
mod dpx_jp2image;
mod dpx_jpegimage;
mod dpx_mem;
mod dpx_mfileio;
mod dpx_mpost;
mod dpx_numbers;
mod dpx_otl_conf;
mod dpx_otl_opt;
mod dpx_pdfcolor;
mod dpx_pdfdev;
mod dpx_pdfdoc;
mod dpx_pdfdraw;
mod dpx_pdfencoding;
mod dpx_pdfencrypt;
mod dpx_pdffont;
mod dpx_pdfnames;
mod dpx_pdfobj;
mod dpx_pdfparse;
mod dpx_pdfresource;
mod dpx_pdfximage;
mod dpx_pkfont;
mod dpx_pngimage;
mod dpx_pst;
mod dpx_pst_obj;
mod dpx_sfnt;
mod dpx_spc_color;
mod dpx_spc_dvipdfmx;
mod dpx_spc_dvips;
mod dpx_spc_html;
mod dpx_spc_misc;
mod dpx_spc_pdfm;
mod dpx_spc_tpic;
mod dpx_spc_util;
mod dpx_spc_xtx;
mod dpx_specials;
mod dpx_subfont;
mod dpx_t1_char;
mod dpx_t1_load;
mod dpx_tfm;
mod dpx_truetype;
mod dpx_tt_aux;
mod dpx_tt_cmap;
mod dpx_tt_glyf;
mod dpx_tt_gsub;
mod dpx_tt_post;
mod dpx_tt_table;
mod dpx_type0;
mod dpx_type1;
mod dpx_type1c;
mod dpx_unicode;
mod dpx_vf;
mod xetex_engine_interface;
mod xetex_errors;
mod xetex_ext;
mod xetex_ini;
mod xetex_io;
mod xetex_linebreak;
mod xetex_math;
mod xetex_output;
mod xetex_pagebuilder;
mod xetex_pic;
mod xetex_scaledmath;
mod xetex_shipout;
mod xetex_stringpool;
mod xetex_synctex;
mod xetex_texmfmp;
mod xetex_xetex0;