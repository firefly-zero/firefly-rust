use crate::Peer;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Badge(pub u8);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Board(pub u8);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Progress {
    /// How many points the player already has.
    pub done: u16,
    /// How many points the player needs to earn the badge.
    pub goal: u16,
}

impl Progress {
    #[must_use]
    pub fn earned(&self) -> bool {
        self.done >= self.goal
    }
}

/// Get the progress of earning the badge.
#[must_use]
pub fn get_progress(p: Peer, b: Badge) -> Progress {
    add_progress(p, b, 0)
}

/// Add the given value to the progress for the badge.
///
/// May be negative if you want to decrease the progress.
/// If zero, does not change the progress.
///
/// If the Peer is [`Peer::COMBINED`], the progress is added to every peer
/// and the returned value is the lowest progress.
#[expect(clippy::must_use_candidate)]
pub fn add_progress(p: Peer, b: Badge, v: i16) -> Progress {
    let r = unsafe { bindings::add_progress(u32::from(p.0), u32::from(b.0), i32::from(v)) };
    Progress {
        done: (r >> 16) as u16,
        goal: (r) as u16,
    }
}

/// Get the personal best of the player.
#[must_use]
pub fn get_score(p: Peer, b: Board) -> i16 {
    add_score(p, b, 0)
}

/// Add the given score to the board.
///
/// May be negative if you want the lower scores
/// to rank higher. Zero value is not added to the board.
///
/// If the Peer is [`Peer::COMBINED`], the score is added for every peer
/// and the returned value is the lowest of their best scores.
#[expect(clippy::must_use_candidate)]
pub fn add_score(p: Peer, b: Board, v: i16) -> i16 {
    let r = unsafe { bindings::add_score(u32::from(p.0), u32::from(b.0), i32::from(v)) };
    r as i16
}

mod bindings {
    #[link(wasm_import_module = "stats")]
    unsafe extern "C" {
        pub(crate) fn add_progress(peer_id: u32, badge_id: u32, val: i32) -> u32;
        pub(crate) fn add_score(peer_id: u32, board_id: u32, new_score: i32) -> i32;
    }
}
