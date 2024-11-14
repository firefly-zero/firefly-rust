use crate::Peer;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Badge(pub u8);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Board(pub u8);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Progress {
    pub done: u16,
    pub goal: u16,
}

impl Progress {
    #[must_use]
    pub fn earned(&self) -> bool {
        self.done >= self.goal
    }
}

#[must_use]
pub fn get_progress(p: Peer, b: Badge) -> Progress {
    add_progress(p, b, 0)
}

#[expect(clippy::must_use_candidate)]
pub fn add_progress(p: Peer, b: Badge, v: i16) -> Progress {
    let r = unsafe { bindings::add_progress(u32::from(p.0), u32::from(b.0), i32::from(v)) };
    Progress {
        done: (r >> 16) as u16,
        goal: (r) as u16,
    }
}

#[must_use]
pub fn get_score(p: Peer, b: Badge) -> i16 {
    add_score(p, b, 0)
}

#[expect(clippy::must_use_candidate)]
pub fn add_score(p: Peer, b: Badge, v: i16) -> i16 {
    let r = unsafe { bindings::add_score(u32::from(p.0), u32::from(b.0), i32::from(v)) };
    r as i16
}

mod bindings {
    #[link(wasm_import_module = "stats")]
    extern {
        pub(crate) fn add_progress(peer_id: u32, badge_id: u32, val: i32) -> u32;
        pub(crate) fn add_score(peer_id: u32, board_id: u32, new_score: i32) -> i32;
    }
}
