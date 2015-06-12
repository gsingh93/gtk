// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

// TODO: feature = ?

use ffi::{self, GtkCssProvider};
use glib::translate::ToGlibPtr;

#[repr(C)]
pub struct CssProvider {
    pub pointer: *mut GtkCssProvider
}

impl CssProvider {
    pub fn new() -> Self {
        unsafe { CssProvider { pointer: ffi::gtk_css_provider_new() } }
    }
    // pub fn get_default() -> GtkCssProvider;
    // pub fn get_named() -> GtkCssProvider;
    // pub fn load_from_data() -> gboolean;
    // pub fn load_from_file(file: File) -> GtkCssProvider {
    //     gtk_css_provider_load_from_file()
    // }
    pub fn load_from_path(path: &str) -> CssProvider {
        unsafe {
            let pointer = ffi::gtk_css_provider_new();
            ffi::gtk_css_provider_load_from_path(pointer, path.to_glib_none().0, 0);
            CssProvider { pointer: pointer }
        }
    }
    // pub fn load_from_resource(resource_path: &str);
}

// impl Display for CssProvider {
//     pub fn to_string() -> String;
// }

impl_GObjectFunctions!(CssProvider, GtkCssProvider);
impl_TraitObject!(CssProvider, GtkCssProvider);
