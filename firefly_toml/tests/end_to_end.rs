mod badges {
use firefly_rust::{Badge, Peer, add_progress, Progress, Board};
/// caused an unsoundness without unsafe code
pub fn segfault(peer: Peer, ) -> Progress { add_progress(peer, Badge(1), 1) }
/// found a secret
pub fn secret(peer: Peer, ) -> Progress { add_progress(peer, Badge(2), 1) }
}enum Cheats {
goto_level = 1,
}impl Cheats { fn from_id(id: i32) -> Self { match id {
1 => Self::goto_level,
_ => unreachable!(),
} } }
mod boards {
use firefly_rust::{Peer, add_score, Progress, Board};
pub fn speedrun(peer: Peer, score: i16) -> i16 { add_score(peer, Board(3), score) }
pub fn level_1(peer: Peer, score: i16) -> i16 { add_score(peer, Board(1), score) }
pub fn level_2(peer: Peer, score: i16) -> i16 { add_score(peer, Board(2), score) }
}