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
        self.0 >> p.0 & 1 != 0
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

/// Get the peer cirresponding to the local device.
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

/// Internal bindings to the raw runtime functions.
mod bindings {
    #[link(wasm_import_module = "net")]
    extern {
        pub(crate) fn get_me() -> u32;
        pub(crate) fn get_peers() -> u32;
    }
}
