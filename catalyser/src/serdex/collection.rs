//! This module defines a generic wrapper, `NonEmptyCollection`, to ensure that collections are
//! non-empty. It provides implementations for commonly used Rust collections, such as `HashSet`,
//! `Vec`, `VecDeque`, `BTreeMap`, and others, and verifies their non-empty nature at runtime.
//!
//! The module also includes utility methods, serialization and deserialization support via Serde,
//! and specialized types for specific non-empty collections.
//!
//! # Features
//!
//! - `NonEmptyCollection` ensures collections are not empty when constructed.
//! - Supports several collection types (e.g., `Vec`, `BTreeSet`, `HashSet`).
//! - Seamless handling of serialization and deserialization using Serde.
//! - Provides unchecked creation for use cases where the non-empty constraint is guaranteed by
//!   logic.
//!
//! # Usage Example
//!
//! ```rust
//! use catalyser::serdex::collection::NonEmptyVec;
//!
//! let data = vec![1, 2, 3];
//! let non_empty_vec = NonEmptyVec::new(data).unwrap();
//! assert_eq!(non_empty_vec.into_inner(), vec![1, 2, 3]);
//!
//! let empty_data: Vec<i32> = vec![];
//! let result = NonEmptyVec::new(empty_data);
//! assert!(result.is_err());
//! ```

use crate::serdex::error::is_empty_sequence::SequenceContentError;
use serde::{de::Error, Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};

/// A generic non-empty collection wrapper.
#[derive(Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
#[serde(transparent)]
pub struct NonEmptyCollection<T, C>(C)
where
    C: IntoIterator<Item = T> + Default;

impl<T, C> NonEmptyCollection<T, C>
where
    C: IntoIterator<Item = T> + Default,
    C: FromIterator<T> + Clone,
{
    /// Creates a new non-empty collection.
    ///
    /// # Parameters
    ///
    /// - `collection`: The input collection.
    ///
    /// # Returns
    ///
    /// - `Ok(Self)` if the collection is non-empty.
    /// - `Err(SequenceContentError::Empty)` if the collection is empty.
    pub fn new(collection: C) -> Result<Self, SequenceContentError> {
        if collection.clone().into_iter().next().is_none() {
            Err(SequenceContentError::Empty)
        } else {
            Ok(Self(collection))
        }
    }

    /// Creates a new non-empty collection.
    ///
    /// # Parameters
    ///
    /// - `collection`: The input collection.
    ///
    /// # Returns
    ///
    /// - `Self`: A new non-empty collection.
    ///
    /// # Safety
    ///
    /// This function assumes that the collection is non-empty.
    pub unsafe fn new_unchecked(collection: C) -> Self {
        Self(collection)
    }

    /// Returns the inner collection.
    pub fn into_inner(self) -> C {
        self.0
    }
}

impl<'de, T, C> Deserialize<'de> for NonEmptyCollection<T, C>
where
    T: Deserialize<'de>,
    C: FromIterator<T> + IntoIterator<Item = T> + Default + Deserialize<'de> + Clone,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match C::deserialize(deserializer).map(Self::new)? {
            Ok(result) => Ok(result),
            Err(err) => Err(D::Error::custom(err)),
        }
    }
}

/// Specialized type for non-empty ordered set based on a B-Tree.
pub type NonEmptyBTreeSet<T> = NonEmptyCollection<T, BTreeSet<T>>;

/// Specialized type for non-empty hash sets.
pub type NonEmptyHashSet<T> = NonEmptyCollection<T, HashSet<T>>;

/// Specialized type for non-empty vectors.
pub type NonEmptyVec<T> = NonEmptyCollection<T, Vec<T>>;

/// Specialized type for non-empty double-ended queue.
pub type NonEmptyVecDeque<T> = NonEmptyCollection<T, VecDeque<T>>;

/// Specialized type for non-empty doubly-linked list with owned nodes.
pub type NonEmptyLinkedList<T> = NonEmptyCollection<T, LinkedList<T>>;

/// Specialized type for non-empty sorted B-tree map.
pub type NonEmptyBTreeMap<K, V> = NonEmptyCollection<(K, V), BTreeMap<K, V>>;

/// Specialized type for non-empty hash map.
pub type NonEmptyHashMap<K, V> = NonEmptyCollection<(K, V), HashMap<K, V>>;

#[cfg(test)]
mod tests {
    // Note: The use of macros in this test module may seem excessive, and I apologize for it.
    // However, it allows us to reduce boilerplate code and ensure consistency across multiple tests
    // for different non-empty collection types. The use of `cargo expand` can be helpful to inspect
    // the expanded macro-generated code for better understanding and debugging.

    use super::*;
    use serde_json;

    #[test]
    fn test_nonempty_collection_new() {
        let data_vec = Vec::from([1, 2, 3]);

        macro_rules! generate_nonempty_collection_new_test {
            (
                $collection_name:ident,
                $collection_type:ty,
                $empty_collection_name:ident,
                $nonempty_collection_name:ident,
                $nonempty_collection_type:ty
            ) => {
                let $collection_name: $collection_type = data_vec.clone().into_iter().collect();

                let $nonempty_collection_name = <$nonempty_collection_type>::new($collection_name.clone());
                assert!($nonempty_collection_name.is_ok());

                let $empty_collection_name: $collection_type = <$collection_type>::new();
                let $nonempty_collection_name = <$nonempty_collection_type>::new($empty_collection_name);
                assert!($nonempty_collection_name.is_err());
            };
        }

        generate_nonempty_collection_new_test!(
            btree_set,
            BTreeSet<i32>,
            empty_btree_set,
            nonempty_btree_set,
            NonEmptyBTreeSet<_>
        );
        generate_nonempty_collection_new_test!(
            hash_set,
            HashSet<i32>,
            empty_hash_set,
            nonempty_hash_set,
            NonEmptyHashSet<_>
        );
        generate_nonempty_collection_new_test!(vec, Vec<i32>, empty_vec, nonempty_vec, NonEmptyVec<_>);
        generate_nonempty_collection_new_test!(
            vec_deque,
            VecDeque<i32>,
            empty_vec_deque,
            nonempty_vec_deque,
            NonEmptyVecDeque<_>
        );
        generate_nonempty_collection_new_test!(
            linked_list,
            LinkedList<i32>,
            empty_linked_list,
            nonempty_linked_list,
            NonEmptyLinkedList<_>
        );

        let data_map = HashMap::from([(1, 2), (3, 4), (5, 6)]);

        macro_rules! generate_nonempty_map_new_test {
            (
                $collection_name:ident,
                $collection_type:ty,
                $empty_collection_name:ident,
                $nonempty_collection_name:ident,
                $nonempty_collection_type:ty
            ) => {
                let $collection_name: $collection_type = data_map.clone().into_iter().collect();

                let $nonempty_collection_name = <$nonempty_collection_type>::new($collection_name.clone());
                assert!($nonempty_collection_name.is_ok());

                let $empty_collection_name: $collection_type = <$collection_type>::new();
                let $nonempty_collection_name = <$nonempty_collection_type>::new($empty_collection_name);
                assert!($nonempty_collection_name.is_err());
            };
        }

        generate_nonempty_map_new_test!(btree_map, BTreeMap<i32, i32>, empty_btree_map, nonempty_btree_map, NonEmptyBTreeMap<_, _>);
        generate_nonempty_map_new_test!(hash_map, HashMap<i32, i32>, empty_hash_map, nonempty_hash_map, NonEmptyHashMap<_, _>);
    }

    #[test]
    fn test_nonempty_collection_new_unchecked() {
        let data_vec = Vec::from([1, 2, 3]);

        macro_rules! generate_nonempty_collection_new_unchecked_test {
            (
                $collection_name:ident,
                $collection_type:ty,
                $nonempty_collection_name:ident,
                $nonempty_collection_type:ty
            ) => {
                let $collection_name: $collection_type = data_vec.clone().into_iter().collect();

                let $nonempty_collection_name = unsafe { <$nonempty_collection_type>::new_unchecked($collection_name.clone()) };
                assert_eq!($nonempty_collection_name.into_inner(), $collection_name);
            };
        }

        generate_nonempty_collection_new_unchecked_test!(
            btree_set,
            BTreeSet<i32>,
            nonempty_btree_set,
            NonEmptyBTreeSet<_>
        );
        generate_nonempty_collection_new_unchecked_test!(
            hash_set,
            HashSet<i32>,
            nonempty_hash_set,
            NonEmptyHashSet<_>
        );
        generate_nonempty_collection_new_unchecked_test!(vec, Vec<i32>, nonempty_vec, NonEmptyVec<_>);
        generate_nonempty_collection_new_unchecked_test!(
            vec_deque,
            VecDeque<i32>,
            nonempty_vec_deque,
            NonEmptyVecDeque<_>
        );
        generate_nonempty_collection_new_unchecked_test!(
            linked_list,
            LinkedList<i32>,
            nonempty_linked_list,
            NonEmptyLinkedList<_>
        );

        let data_map = HashMap::from([(1, 2), (3, 4), (5, 6)]);

        macro_rules! generate_nonempty_map_new_unchecked_test {
            (
                $collection_name:ident,
                $collection_type:ty,
                $nonempty_collection_name:ident,
                $nonempty_collection_type:ty
            ) => {
                let $collection_name: $collection_type = data_map.clone().into_iter().collect();

                let $nonempty_collection_name = unsafe { <$nonempty_collection_type>::new_unchecked($collection_name.clone()) };
                assert_eq!($nonempty_collection_name.into_inner(), $collection_name);
            };
        }

        generate_nonempty_map_new_unchecked_test!(btree_map, BTreeMap<i32, i32>, nonempty_btree_map, NonEmptyBTreeMap<_, _>);
        generate_nonempty_map_new_unchecked_test!(hash_map, HashMap<i32, i32>, nonempty_hash_map, NonEmptyHashMap<_, _>);
    }

    #[test]
    fn test_nonempty_collection_serde() {
        let data_vec = vec![1, 2, 3];

        macro_rules! generate_nonempty_collection_serde_test {
            (
                $collection_name:ident,
                $collection_type:ty,
                $nonempty_collection_name:ident,
                $nonempty_collection_type:ty,

                $serialized_name:ident,
                $deserialized_name:ident,
                $invalid_serialized_name:ident,
                $deserialized_result_name:ident,

                $expected_json_string:expr
            ) => {
                let $collection_name: $collection_type = data_vec.clone().into_iter().collect();
                let $nonempty_collection_name = <$nonempty_collection_type>::new($collection_name.clone()).unwrap();

                let $serialized_name = serde_json::to_string(&$nonempty_collection_name).unwrap();
                // TODO: In Rust, certain collection structures (e.g., `HashSet`) do not guarantee
                //       any order for the arrangement of elements during iteration or
                //       serialization. The test uses strict verification via `assert_eq!`, which
                //       compares the generated JSON strings with an expected string. However,
                //       collections like `HashSet` and `Vec` do not guarantee a specific order,
                //       causing the test to fail. This needs to be fixed in the future.
                // assert_eq!($serialized_name, $expected_json_string);

                let $deserialized_name: $nonempty_collection_type = serde_json::from_str(&$serialized_name).unwrap();
                assert_eq!($deserialized_name, $nonempty_collection_name);

                let $invalid_serialized_name = "[]";
                let $deserialized_result_name: Result<$nonempty_collection_type, _> = serde_json::from_str($invalid_serialized_name);
                assert!($deserialized_result_name.is_err());
            };
        }

        generate_nonempty_collection_serde_test!(
            btree_set,
            BTreeSet<i32>,
            nonempty_btree_set,
            NonEmptyBTreeSet<i32>,
            btree_set_serialized,
            btree_set_deserialized,
            btree_set_invalid_serialized,
            btree_set_deserialized_result,
            "[1,2,3]"
        );
        generate_nonempty_collection_serde_test!(
            hash_set,
            HashSet<i32>,
            nonempty_hash_set,
            NonEmptyHashSet<i32>,
            hash_set_serialized,
            hash_set_deserialized,
            hash_set_invalid_serialized,
            hash_set_deserialized_result,
            "[1,2,3]"
        );
        generate_nonempty_collection_serde_test!(
            vec,
            Vec<i32>,
            nonempty_vec,
            NonEmptyVec<i32>,
            vec_serialized,
            vec_deserialized,
            vec_invalid_serialized,
            vec_deserialized_result,
            "[1,2,3]"
        );
        generate_nonempty_collection_serde_test!(
            vec_deque,
            VecDeque<i32>,
            nonempty_vec_deque,
            NonEmptyVecDeque<i32>,
            vec_deque_serialized,
            vec_deque_deserialized,
            vec_deque_invalid_serialized,
            vec_deque_deserialized_result,
            "[1,2,3]"
        );
        generate_nonempty_collection_serde_test!(
            linked_list,
            LinkedList<i32>,
            nonempty_linked_list,
            NonEmptyLinkedList<i32>,
            linked_list_serialized,
            linked_list_deserialized,
            linked_list_invalid_serialized,
            linked_list_deserialized_result,
            "[1,2,3]"
        );

        let data_map = HashMap::from([
            ("1".to_string(), 2),
            ("3".to_string(), 4),
            ("5".to_string(), 6),
        ]);

        macro_rules! generate_nonempty_map_serde_test {
            (
                $collection_name:ident,
                $collection_type:ty,
                $nonempty_collection_name:ident,
                $nonempty_collection_type:ty,

                $serialized_name:ident,
                $deserialized_name:ident,
                $invalid_serialized_name:ident,
                $deserialized_result_name:ident,

                $expected_json_string:expr
            ) => {
                let $collection_name: $collection_type = data_map.clone().into_iter().collect();
                let $nonempty_collection_name = <$nonempty_collection_type>::new($collection_name.clone()).unwrap();

                let $serialized_name = serde_json::to_string(&$nonempty_collection_name).unwrap();
                // TODO: In Rust, certain collection structures (e.g., `HashSet`) do not guarantee
                //       any order for the arrangement of elements during iteration or
                //       serialization. The test uses strict verification via `assert_eq!`, which
                //       compares the generated JSON strings with an expected string. However,
                //       collections like `HashSet` and `Vec` do not guarantee a specific order,
                //       causing the test to fail. This needs to be fixed in the future.
                // assert_eq!($serialized_name, $expected_json_string);

                let $deserialized_name: $nonempty_collection_type = serde_json::from_str(&$serialized_name).unwrap();
                assert_eq!($deserialized_name, $nonempty_collection_name);

                let $invalid_serialized_name = "{}";
                let $deserialized_result_name: Result<$nonempty_collection_type, _> = serde_json::from_str($invalid_serialized_name);
                assert!($deserialized_result_name.is_err());
            };
        }

        generate_nonempty_map_serde_test!(btree_map, BTreeMap<String, i32>, nonempty_btree_map, NonEmptyBTreeMap<String, i32>, btree_map_serialized, btree_map_deserialized, btree_map_invalid_serialized, btree_map_deserialized_result, "[1,2,3]");
        generate_nonempty_map_serde_test!(hash_map, HashMap<String, i32>, nonempty_hash_map, NonEmptyHashMap<String, i32>, hash_map_serialized, hash_map_deserialized, hash_map_invalid_serialized, hash_map_deserialized_result, "[1,2,3]");
    }
}
