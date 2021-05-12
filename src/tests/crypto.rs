use crate::crypto::{self, hexdigest};

#[test]
fn test_hexdigest() {
    let verified = [
        ("Notch", "4ed1f46bbe04bc756bcb17c0c7ce3e4632f06a48"),
        ("jeb_", "-7c9d5b0044c130109a5d7b5fb5c317c02b4e28c1"),
        ("simon", "88e16a1019277b15d58faf0541e11910eb756f6"),
    ];

    for (n, h) in verified.iter() {
        let mut hasher = crypto::Sha1::new();
        hasher.update(n.as_bytes());
        assert_eq!(hexdigest(hasher), h.to_owned())
    }
}
