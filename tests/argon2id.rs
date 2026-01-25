use cryptal::derivation::{Argon2Params, argon2id};

#[test]
fn argon2id_is_deterministic() {
    let params = Argon2Params {
        mem_kib: 32,
        lanes: 4,
        time: 3,
        tag_len: 32,
    };
    let a = argon2id(b"password", b"saltsalt", &params).unwrap();
    let b = argon2id(b"password", b"saltsalt", &params).unwrap();
    assert_eq!(a, b);
}

#[test]
fn argon2id_changes_with_salt() {
    let params = Argon2Params {
        mem_kib: 32,
        lanes: 4,
        time: 3,
        tag_len: 32,
    };
    let a = argon2id(b"password", b"saltAAAA", &params).unwrap();
    let b = argon2id(b"password", b"saltBBBB", &params).unwrap();
    assert_ne!(a, b);
}

#[test]
fn argon2id_respects_output_length() {
    let params = Argon2Params {
        mem_kib: 32,
        lanes: 4,
        time: 1,
        tag_len: 64,
    };
    let out = argon2id(b"password", b"saltsalt", &params).unwrap();
    assert_eq!(out.len(), 64);
}

#[test]
fn argon2id_simple_vectors() {
    let params1 = Argon2Params {
        mem_kib: 32,
        lanes: 1,
        time: 1,
        tag_len: 32,
    };
    let result1 = argon2id(b"password", b"saltsalt", &params1).unwrap();
    assert_eq!(result1.len(), 32);

    let params2 = Argon2Params {
        mem_kib: 64,
        lanes: 2,
        time: 2,
        tag_len: 32,
    };
    let result2 = argon2id(b"password", b"saltsalt", &params2).unwrap();
    assert_ne!(result1, result2,);

    let result3 = argon2id(b"different", b"saltsalt", &params1).unwrap();
    assert_ne!(result1, result3,);
}

// #[test]
// fn argon2id_recommended_params() {
//     let params = Argon2Params {
//         mem_kib: 65536,
//         lanes: 4,
//         time: 3,
//         tag_len: 32,
//     };

//     let result = argon2id(b"my_secure_password", b"random_salt_16_b", &params).unwrap();
//     assert_eq!(result.len(), 32);
// }
