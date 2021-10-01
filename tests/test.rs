use git_version::{git_hash, git_hash_short, run_command};

#[test]
fn git_hash() {
    assert!(git_hash!().len() == 40);
}

#[test]
fn git_hash_short_default() {
    assert!(git_hash_short!().len() == 7);
}

#[test]
fn git_hash_short_custom() {
    assert!(git_hash_short!(0).len() == 4); // minimum output length is 4
    assert!(git_hash_short!(1).len() == 4);
    assert!(git_hash_short!(2).len() == 4);
    assert!(git_hash_short!(3).len() == 4);
    assert!(git_hash_short!(4).len() == 4);
    assert!(git_hash_short!(5).len() == 5);
    assert!(git_hash_short!(6).len() == 6);
    assert!(git_hash_short!(7).len() == 7);
    assert!(git_hash_short!(8).len() == 8);
    assert!(git_hash_short!(9).len() == 9);
    assert!(git_hash_short!(10).len() == 10);
    assert!(git_hash_short!(11).len() == 11);
    assert!(git_hash_short!(12).len() == 12);
    assert!(git_hash_short!(13).len() == 13);
    assert!(git_hash_short!(14).len() == 14);
    assert!(git_hash_short!(15).len() == 15);
}

#[test]
fn run_command() {
    assert!(run_command!("git", "rev-parse" "--abbrev-ref" "HEAD") == "master")
}
