// The peer ID.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Peer(pub(crate) u8);

impl Peer {
    pub const COMBINED: Self = Peer(0xFF);
}

/// The list of peers online.
///
/// Includes all connected peers as well as the local device.
///
/// The order is deterministic between calls and between runs.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Peers(pub(crate) u32);

impl Peers {
    /// Iterate over peers.
    #[must_use]
    pub fn iter(&self) -> PeersIter {
        PeersIter {
            peer: 0,
            peers: self.0,
        }
    }

    /// Check if the given peer is online.
    #[must_use]
    pub fn contains(&self, p: &Peer) -> bool {
        (self.0 >> p.0) & 1 != 0
    }

    /// Get the number of peers online.
    ///
    /// Never zero. 1 for local single-player game. 2 or more for multiplayer.
    #[must_use]
    #[expect(clippy::len_without_is_empty)] // always non-empty
    pub fn len(&self) -> usize {
        self.0.count_ones() as usize
    }

    /// Convert the list of peers into a vector.
    #[cfg(feature = "alloc")]
    #[must_use]
    pub fn as_vec(&self) -> alloc::vec::Vec<Peer> {
        self.iter().collect()
    }
}

impl IntoIterator for Peers {
    type Item = Peer;
    type IntoIter = PeersIter;

    fn into_iter(self) -> Self::IntoIter {
        PeersIter {
            peer: 0,
            peers: self.0,
        }
    }
}

pub struct PeersIter {
    peer: u8,
    peers: u32,
}

impl Iterator for PeersIter {
    type Item = Peer;

    fn next(&mut self) -> Option<Self::Item> {
        while self.peers != 0 {
            let peer = self.peer;
            let peers = self.peers;
            self.peer += 1;
            self.peers >>= 1;
            if peers & 1 != 0 {
                return Some(Peer(peer));
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.peers.count_ones() as usize;
        (size, Some(size))
    }

    fn count(self) -> usize {
        self.peers.count_ones() as usize
    }
}

/// Stash is a serialized binary state of the app that you want to persist
/// between app runs and to be available in multiplayer.
///
/// For single-player purposes, you can save data in a regular file
/// using [`dump_file`](crate::dump_file). File saved that way can be bigger
/// (and you can create lots of them) but it cannot be accessed in multiplayer.
///
/// It's your job to serialize data into a binary stash and later parse it.
/// Stash can be saved using [`save_stash`] and later read using [`load_stash`].
type Stash = [u8];

/// Get the peer corresponding to the local device.
#[must_use]
pub fn get_me() -> Peer {
    let me = unsafe { bindings::get_me() };
    Peer(me as u8)
}

/// Get the list of peers online.
#[must_use]
pub fn get_peers() -> Peers {
    let peers = unsafe { bindings::get_peers() };
    Peers(peers)
}

/// Save the given [`Stash`].
///
/// When called, the stash for the given peer will be stored in RAM.
/// Calling [`load_stash`] for the same peer will return that stash.
/// On exit, the runtime will persist the stash in FS.
/// Next time the app starts, calling [`load_stash`] will restore the stash
/// saved earlier.
pub fn save_stash(peer: Peer, stash: &Stash) {
    let ptr = stash.as_ptr();
    let peer = u32::from(peer.0);
    unsafe {
        bindings::save_stash(peer, ptr as u32, stash.len() as u32);
    }
}

/// Read the stash of the given peer using the passed buffer.
///
/// It's the app's responsibility to ensure that the passed buffer is big enough
/// to fit the stash. If it doesn't fit, runtime will fill the buffer
/// and discard the rest.
///
/// The returned stash is a slice of the passed in buffer of the actual content size
/// (up to the buffer size).
///
/// If there is no stash (which is always true before [`save_stash`]
/// is called for the first time ever), the result is `None`.
#[must_use]
pub fn load_stash(peer: Peer, stash: &mut Stash) -> Option<&mut Stash> {
    let ptr = stash.as_ptr();
    let peer = u32::from(peer.0);
    let size = unsafe { bindings::load_stash(peer, ptr as u32, stash.len() as u32) };
    if size == 0 {
        return None;
    }
    let size = size as usize;
    Some(&mut stash[..size])
}

/// Similar to [`load_stash`] but statically allocates the stash of the right size.
///
/// Unlike [`load_stash`], the returned stash size is not adjusted
/// for the actual content size.
///
/// Unlike other `_buf` functions, like [`load_file_buf`][crate::load_file_buf],
/// this one allocates the buffer statically, not dynamically.
/// The app must statically know the max stash size.
#[must_use]
pub fn load_stash_buf<const N: usize>(peer: Peer) -> Option<[u8; N]> {
    let stash = [0u8; N];
    let ptr = stash.as_ptr();
    let peer = u32::from(peer.0);
    let size = unsafe { bindings::load_stash(peer, ptr as u32, stash.len() as u32) };
    if size == 0 {
        return None;
    }
    Some(stash)
}

/// Internal bindings to the raw runtime functions.
mod bindings {
    #[link(wasm_import_module = "net")]
    extern {
        pub(crate) fn get_me() -> u32;
        pub(crate) fn get_peers() -> u32;
        pub(crate) fn save_stash(peer: u32, ptr: u32, len: u32);
        pub(crate) fn load_stash(peer: u32, ptr: u32, len: u32) -> u32;
    }
}
