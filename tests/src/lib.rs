#![cfg(test)]

use std::{collections::BinaryHeap, env};

// TODO(toms): investigate all sources of 'ID' in Rust standard library
//   * OS-specific, platform-specific?
//   * `std::env`
//       * The first element is traditionally the path of the executable, but it can be set to arbitrary text, and might
//         not even exist. This means this property should not be relied upon for security purposes.
//       * Is the `Args` iterator order defined?
//   * `std::fs::read_dir`
//       * The order in which this iterator returns entries is platform and filesystem dependent.
//   * `Instant`
//       * `monotonically nondecreasing clock`
//       * OS-specific behaviors
//   * `SystemTime`
//       * time going backwards?
//   * comparing floating-point numbers directly?
//   * Changing how things implement `Debug`? (folks could make assumptions on format)
//       * https://doc.rust-lang.org/std/fmt/trait.Debug.html#stability
//   * Vec: https://doc.rust-lang.org/std/vec/struct.Vec.html
//       * 'Note: the ABI is not stable and Vec makes no guarantees about its memory layout (including the order of
//         fields).'
//   * Endianness assumptions? (e.g. `to_bits`)
//   * 'undefined behavior'?
//       * Verify that functions like `Option::unwrap_unchecked` are always called 'correctly'
//           * Does 'miri' check this?
//       * Verify that `unreachable_unchecked` is never called

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

    let keys: Box<[_]> = items.iter().cloned().collect();

    assert_eq!(keys, ["foo", "bar", "baz"].into());
}

#[test]
fn test_env_vars_order() {
    let vars1 = env::vars().collect::<Vec<_>>();
    let vars2 = env::vars().collect::<Vec<_>>();

    assert_eq!(vars1.len(), vars2.len());
    assert_eq!(vars1, vars2);
}
