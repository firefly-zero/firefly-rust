use crate::bindings as b;

// pub struct FileBuf {
//     raw: [u8],
// }

// impl FileBuf {
//     pub fn load(name: &str) -> Self {
//         let size = File::get_size(name);
//         let mut buf: [u8] = todo!();
//         Self { buf }
//     }
// }

pub struct RomFile<'a> {
    raw: &'a [u8],
}

impl<'a> RomFile<'a> {
    pub fn get_size(name: &str) -> usize {
        let path_ptr = name.as_ptr();
        let size = unsafe { b::get_rom_file_size(path_ptr as u32, name.len() as u32) };
        size as usize
    }

    pub fn load(name: &str, buf: &'a mut [u8]) -> Self {
        let path_ptr = name.as_ptr();
        let buf_ptr = buf.as_mut_ptr();
        unsafe {
            b::load_rom_file(
                path_ptr as u32,
                name.len() as u32,
                buf_ptr as u32,
                buf.len() as u32,
            );
        }
        Self { raw: buf }
    }
}

pub struct Font<'a> {
    pub(crate) raw: &'a [u8],
}

impl<'a> From<RomFile<'a>> for Font<'a> {
    fn from(value: RomFile<'a>) -> Self {
        Self { raw: value.raw }
    }
}

pub struct Image<'a> {
    pub(crate) raw: &'a [u8],
}

impl<'a> From<RomFile<'a>> for Image<'a> {
    fn from(value: RomFile<'a>) -> Self {
        Self { raw: value.raw }
    }
}
