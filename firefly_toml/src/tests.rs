use crate::{Config, write_badges, write_boards, write_cheats};

#[test]
fn end_to_end() {
    run("end_to_end");
}

#[test]
fn empty() {
    run("empty")
}

fn run(name: &str) {
    let input = std::fs::read_to_string(format!("tests/{name}.toml")).unwrap();
    let Config {
        cheats,
        badges,
        boards,
    } = toml::from_str(&input).unwrap();
    let mut s = String::new();
    write_badges(badges.unwrap_or_default(), &mut s);
    write_cheats(cheats.unwrap_or_default(), &mut s);
    write_boards(boards.unwrap_or_default(), &mut s);
    snapshot_testing::assert_eq_or_update(s, format!("tests/{name}.rs"));
}
