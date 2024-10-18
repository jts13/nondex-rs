#![cfg(test)]

use core::mem;
use std::{collections::BinaryHeap, env, fs, io};

// TODO(toms): investigate all sources of 'ID' in Rust standard library
//   * [x] `std::env`
//       * The first element is traditionally the path of the executable, but it can be set to arbitrary text, and might
//         not even exist. This means this property should not be relied upon for security purposes.
//       * Is the `Args` iterator order defined?
//   * [x] `BinaryHeap::iter`
//   * [ ] `std::fs::read_dir`
//       * The order in which this iterator returns entries is platform and filesystem dependent.
//   * [x] Changing how things implement `Debug`? (folks could make assumptions on format)
//       * https://doc.rust-lang.org/std/fmt/trait.Debug.html#stability
//   * [x] Vec: https://doc.rust-lang.org/std/vec/struct.Vec.html
//       * 'Note: the ABI is not stable and Vec makes no guarantees about its memory layout (including the order of fields).'
//   * Time
//       * [ ] `Instant`
//           * `monotonically nondecreasing clock`
//           * OS-specific behaviors
//       * [ ] `SystemTime`
//           * time going backwards?
//   * Other ideas?
//       * OS-specific, platform-specific?
//       * comparing floating-point numbers directly?
//       * Endianness assumptions? (e.g. `to_bits`)
//       * 'undefined behavior'?
//           * Verify that functions like `Option::unwrap_unchecked` are always called 'correctly'
//               * Does 'miri' check this?
//           * Verify that `unreachable_unchecked` is never called

/// # HashMap and HashSet
///
/// From the docs (abbreviated):
///
/// > By default, `HashMap` and `HashSet` use a hashing algorithm selected to provide
/// > resistance against HashDoS attacks: currently, SipHash 1-3. The algorithm is
/// > randomly seeded from a high quality, secure source of randomness provided by the host.
///
/// Due to the default use of a randomized hash, finding many flaky tests in the wild is not likely.
mod hash {
    use std::collections::{HashMap, HashSet};

    #[test]
    #[ignore]
    fn test_hashmap_iter() {
        let items: HashMap<_, _> = ["foo", "bar", "baz"]
            .iter()
            .cloned()
            .enumerate()
            .map(|(k, v)| (v, k))
            .collect();

        let keys: Box<[_]> = items.keys().cloned().collect();

        assert_eq!(keys, ["foo", "bar", "baz"].into());
    }

    #[test]
    #[ignore]
    fn test_hashset_iter() {
        let items: HashSet<_> = ["foo", "bar", "baz"].iter().cloned().collect();

        let keys: Box<[_]> = items.iter().cloned().collect();

        assert_eq!(keys, ["foo", "bar", "baz"].into());
    }
}

mod ub {
    #[test]
    #[ignore]
    fn test_unwrap_unchecked() {
        let value: Option<()> = None;
        let value = unsafe { value.unwrap_unchecked() };
        assert_eq!(value, ());
    }
}

#[test]
fn test_binary_heap_iter() {
    let items: BinaryHeap<_> = ["foo", "bar", "baz"].iter().cloned().collect();

    let mut items1: Box<[_]> = items.iter().cloned().collect();
    let mut items2: Box<[_]> = items.iter().cloned().collect();

    assert_eq!(items1.len(), items2.len());

    assert_ne!(items1, items2); // != due to chaos

    items1.sort();
    items2.sort();

    assert_eq!(items1, items2);
}

#[test]
fn test_env_vars_order() {
    let mut vars1: Box<[_]> = env::vars().collect();
    let mut vars2: Box<[_]> = env::vars().collect();

    assert_eq!(vars1.len(), vars2.len());

    assert_ne!(vars1, vars2); // != due to chaos

    vars1.sort();
    vars2.sort();

    assert_eq!(vars1, vars2);
}

#[test]
fn test_fs_read_dir_order() {
    let read = || -> io::Result<Box<[_]>> {
        fs::read_dir("..")?
            .into_iter()
            .map(|e| e.map(|e| e.path()))
            .collect()
    };

    let mut e1 = read().unwrap();
    let mut e2 = read().unwrap();

    assert_eq!(e1.len(), e2.len());

    // TODO(toms): assert_ne!(e1, e2); // != due to chaos

    e1.sort();
    e2.sort();

    assert_eq!(e1, e2);
}

#[test]
fn test_vec_layout() {
    // The layout of this matches the
    #[repr(C)]
    struct RawVec<T> {
        ptr: core::ptr::NonNull<u8>,
        _mk: core::marker::PhantomData<T>,
        cap: usize,
        len: usize,
    }

    assert_eq!(mem::size_of::<Vec<()>>(), mem::size_of::<RawVec<()>>());

    let v: Vec<i32> = vec![1, 2, 3];
    let v: RawVec<i32> = unsafe { mem::transmute(v) };

    // TODO(toms): this could be a bit flaky as memory addresses and padding could coincidentally match

    assert_ne!(v.len, 3); // != due to chaos
}

#[test]
fn test_debug() {
    // assert_eq!(format!("{:?}", vec![1, 2, 3]), "[1, 2, 3]");
    assert_eq!(format!("{:?}", vec![1, 2, 3]), ""); // != due to chaos
}
