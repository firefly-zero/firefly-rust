/// Add a custom item on the app menu.
///
/// The `i` index is the value passed into the `handle_menu` callback
/// when the menu item is selected by the user.
/// Its value doesn't have to be unique or continious.
pub fn add_menu_item(i: u8, t: &str) {
    let ptr = t.as_ptr() as u32;
    let len = t.len() as u32;
    unsafe {
        bindings::add_menu_item(u32::from(i), ptr, len);
    }
}

/// Remove a custom menu item with the given index.
pub fn remove_menu_item(i: u8) {
    unsafe {
        bindings::remove_menu_item(u32::from(i));
    }
}

/// Open the app menu.
///
/// It will be opened before the next update.
/// The current update and then render will proceed as planned.
pub fn open_menu() {
    unsafe {
        bindings::open_menu();
    }
}

mod bindings {
    #[link(wasm_import_module = "menu")]
    unsafe extern "C" {
        pub(crate) fn add_menu_item(index: u32, text_ptr: u32, text_len: u32);
        pub(crate) fn remove_menu_item(index: u32);
        pub(crate) fn open_menu();
    }
}
