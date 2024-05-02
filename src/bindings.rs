#[link(wasm_import_module = "fs")]
extern {
    pub(crate) fn get_rom_file_size(path_ptr: u32, path_len: u32) -> u32;
    pub(crate) fn get_file_size(path_ptr: u32, path_len: u32) -> u32;
    pub(crate) fn load_rom_file(path_ptr: u32, path_len: u32, buf_ptr: u32, buf_len: u32) -> u32;
    pub(crate) fn load_file(path_ptr: u32, path_len: u32, buf_ptr: u32, buf_len: u32) -> u32;
    pub(crate) fn dump_file(path_ptr: u32, path_len: u32, buf_ptr: u32, buf_len: u32) -> u32;
    pub(crate) fn remove_file(path_ptr: u32, path_len: u32);
}

#[link(wasm_import_module = "input")]
extern {
    pub(crate) fn read_pad(player: u32) -> u32;
    pub(crate) fn read_buttons(player: u32) -> u32;
}

#[link(wasm_import_module = "misc")]
extern {
    pub(crate) fn log_debug(ptr: u32, len: u32);
    pub(crate) fn log_error(ptr: u32, len: u32);
    pub(crate) fn set_seed(seed: u32);
    pub(crate) fn get_random() -> u32;
}
