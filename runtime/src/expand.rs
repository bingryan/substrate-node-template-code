#![feature(prelude_import)]
#![recursion_limit = "256"]

#[prelude_import]
use std::prelude::v1::*;

#[macro_use]
extern crate std;

pub const WASM_BINARY: Option<&[u8]> = Some(b"\x00asm\x01\x00\x00\x00\x01\xe3\x01#");

use sp_std::prelude::*;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
    ApplyExtrinsicResult, generic, create_runtime_str, impl_opaque_keys, MultiSignature,
    transaction_validity::{TransactionValidity, TransactionSource},
};
use sp_runtime::traits::{
    BlakeTwo256, Block as BlockT, IdentityLookup, Verify, IdentifyAccount, NumberFor, Saturating,
};
use sp_api::impl_runtime_apis;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use pallet_grandpa::{AuthorityId as GrandpaId, AuthorityList as GrandpaAuthorityList};
use pallet_grandpa::fg_primitives;
use sp_version::RuntimeVersion;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use frame_system::EnsureRoot;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use pallet_timestamp::Call as TimestampCall;
pub use pallet_balances::Call as BalancesCall;
pub use sp_runtime::{Permill, Perbill};
pub use frame_support::{
    construct_runtime, parameter_types, StorageValue,
    traits::{KeyOwnerProofSystem, Randomness},
    weights::{
        Weight, IdentityFee,
        constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
    },
};
/// Import the template pallet.
pub use template;

/// An index to a block.
pub type BlockNumber = u32;
/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;
/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
/// The type for looking up accounts. We don't expect more than 4 billion of them, but you
/// never know...
pub type AccountIndex = u32;
/// Balance of an account.
pub type Balance = u128;
/// Index of a transaction in the chain.
pub type Index = u32;
/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;
/// Digest item type.
pub type DigestItem = generic::DigestItem<Hash>;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
    use super::*;
    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

    /// Opaque block header type.
    pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// Opaque block type.
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    /// Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;

    pub struct SessionKeys {
        pub aura: <Aura as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
        pub grandpa: <Grandpa as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
    }

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for SessionKeys {
        #[inline]
        fn default() -> SessionKeys {
            SessionKeys {
                aura: ::core::default::Default::default(),
                grandpa: ::core::default::Default::default(),
            }
        }
    }

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for SessionKeys {
        #[inline]
        fn clone(&self) -> SessionKeys {
            match *self {
                SessionKeys {
                    aura: ref __self_0_0,
                    grandpa: ref __self_0_1,
                } => SessionKeys {
                    aura: ::core::clone::Clone::clone(&(*__self_0_0)),
                    grandpa: ::core::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }

    impl ::core::marker::StructuralPartialEq for SessionKeys {}

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for SessionKeys {
        #[inline]
        fn eq(&self, other: &SessionKeys) -> bool {
            match *other {
                SessionKeys {
                    aura: ref __self_1_0,
                    grandpa: ref __self_1_1,
                } => match *self {
                    SessionKeys {
                        aura: ref __self_0_0,
                        grandpa: ref __self_0_1,
                    } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &SessionKeys) -> bool {
            match *other {
                SessionKeys {
                    aura: ref __self_1_0,
                    grandpa: ref __self_1_1,
                } => match *self {
                    SessionKeys {
                        aura: ref __self_0_0,
                        grandpa: ref __self_0_1,
                    } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }

    impl ::core::marker::StructuralEq for SessionKeys {}

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for SessionKeys {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<
                    <Aura as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
                >;
                let _: ::core::cmp::AssertParamIsEq<
                    <Grandpa as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
                >;
            }
        }
    }

    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl _parity_scale_codec::Encode for SessionKeys {
            fn encode_to<EncOut: _parity_scale_codec::Output>(&self, dest: &mut EncOut) {
                dest.push(&self.aura);
                dest.push(&self.grandpa);
            }
        }
        impl _parity_scale_codec::EncodeLike for SessionKeys {}
    };
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl _parity_scale_codec::Decode for SessionKeys {
            fn decode<DecIn: _parity_scale_codec::Input>(
                input: &mut DecIn,
            ) -> core::result::Result<Self, _parity_scale_codec::Error> {
                Ok(SessionKeys {
                    aura: {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => return Err("Error decoding field SessionKeys.aura".into()),
                            Ok(a) => a,
                        }
                    },
                    grandpa: {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => return Err("Error decoding field SessionKeys.grandpa".into()),
                            Ok(a) => a,
                        }
                    },
                })
            }
        }
    };

    impl core::fmt::Debug for SessionKeys {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            fmt.debug_struct("SessionKeys")
                .field("aura", &self.aura)
                .field("grandpa", &self.grandpa)
                .finish()
        }
    }

    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for SessionKeys {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "SessionKeys",
                    false as usize + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "aura",
                    &self.aura,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "grandpa",
                    &self.grandpa,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for SessionKeys {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 2",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                    {
                        match __value {
                            "aura" => _serde::export::Ok(__Field::__field0),
                            "grandpa" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                    {
                        match __value {
                            b"aura" => _serde::export::Ok(__Field::__field0),
                            b"grandpa" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<SessionKeys>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = SessionKeys;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct SessionKeys")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            <Aura as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct SessionKeys with 2 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            <Grandpa as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct SessionKeys with 2 elements",
                                ));
                            }
                        };
                        _serde::export::Ok(SessionKeys {
                            aura: __field0,
                            grandpa: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<
                            <Aura as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
                        > = _serde::export::None;
                        let mut __field1: _serde::export::Option<
                            <Grandpa as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
                        > = _serde::export::None;
                        while let _serde::export::Some(__key) =
                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "aura",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            <Aura as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
                                        >(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "grandpa",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(match _serde::de::MapAccess::next_value::<<Grandpa as ::sp_runtime::BoundToRuntimeAppPublic>::Public>(&mut __map) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => { return _serde::export::Err(__err); }
                                    });
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("aura") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("grandpa") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(SessionKeys {
                            aura: __field0,
                            grandpa: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["aura", "grandpa"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "SessionKeys",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<SessionKeys>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };

    impl SessionKeys {
        /// Generate a set of keys with optionally using the given seed.
        ///
        /// The generated key pairs are stored in the keystore.
        ///
        /// Returns the concatenated SCALE encoded public keys.
        pub fn generate(
            seed: Option<::sp_runtime::sp_std::vec::Vec<u8>>,
        ) -> ::sp_runtime::sp_std::vec::Vec<u8> {
            let keys = Self { aura: <<Aura as ::sp_runtime::BoundToRuntimeAppPublic>::Public as ::sp_runtime::RuntimeAppPublic>::generate_pair(seed.clone()), grandpa: <<Grandpa as ::sp_runtime::BoundToRuntimeAppPublic>::Public as ::sp_runtime::RuntimeAppPublic>::generate_pair(seed.clone()) };
            ::sp_runtime::codec::Encode::encode(&keys)
        }
        /// Converts `Self` into a `Vec` of `(raw public key, KeyTypeId)`.
        pub fn into_raw_public_keys(
            self,
        ) -> ::sp_runtime::sp_std::vec::Vec<(
            ::sp_runtime::sp_std::vec::Vec<u8>,
            ::sp_runtime::KeyTypeId,
        )> {
            let mut keys = Vec::new();
            keys.push((::sp_runtime::RuntimeAppPublic::to_raw_vec(&self.aura), <<Aura as ::sp_runtime::BoundToRuntimeAppPublic>::Public as ::sp_runtime::RuntimeAppPublic>::ID));
            keys.push((::sp_runtime::RuntimeAppPublic::to_raw_vec(&self.grandpa), <<Grandpa as ::sp_runtime::BoundToRuntimeAppPublic>::Public as ::sp_runtime::RuntimeAppPublic>::ID));
            keys
        }
        /// Decode `Self` from the given `encoded` slice and convert `Self` into the raw public
        /// keys (see [`Self::into_raw_public_keys`]).
        ///
        /// Returns `None` when the decoding failed, otherwise `Some(_)`.
        pub fn decode_into_raw_public_keys(
            encoded: &[u8],
        ) -> Option<
            ::sp_runtime::sp_std::vec::Vec<(
                ::sp_runtime::sp_std::vec::Vec<u8>,
                ::sp_runtime::KeyTypeId,
            )>,
        > {
            <Self as ::sp_runtime::codec::Decode>::decode(&mut &encoded[..])
                .ok()
                .map(|s| s.into_raw_public_keys())
        }
    }

    impl ::sp_runtime::traits::OpaqueKeys for SessionKeys {
        type KeyTypeIdProviders = (Aura, Grandpa);
        fn key_ids() -> &'static [::sp_runtime::KeyTypeId] {
            &[<<Aura as ::sp_runtime::BoundToRuntimeAppPublic>::Public as ::sp_runtime::RuntimeAppPublic>::ID, <<Grandpa as ::sp_runtime::BoundToRuntimeAppPublic>::Public as ::sp_runtime::RuntimeAppPublic>::ID]
        }
        fn get_raw(&self, i: ::sp_runtime::KeyTypeId) -> &[u8] {
            match i {
                i if i == <<Aura as ::sp_runtime::BoundToRuntimeAppPublic>::Public as ::sp_runtime::RuntimeAppPublic>::ID => self.aura.as_ref(),
                i if i == <<Grandpa as ::sp_runtime::BoundToRuntimeAppPublic>::Public as ::sp_runtime::RuntimeAppPublic>::ID => self.grandpa.as_ref(),
                _ => &[],
            }
        }
    }
}

pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: { ::sp_runtime::RuntimeString::Borrowed("node-template") },
    impl_name: { ::sp_runtime::RuntimeString::Borrowed("node-template") },
    authoring_version: 1,
    spec_version: 1,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
};
pub const MILLISECS_PER_BLOCK: u64 = 6000;
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

pub struct BlockHashCount;

impl BlockHashCount {
    /// Returns the value of this parameter type.
    pub const fn get() -> BlockNumber {
        2400
    }
}

impl<I: From<BlockNumber>> ::frame_support::traits::Get<I> for BlockHashCount {
    fn get() -> I {
        I::from(2400)
    }
}

/// We allow for 2 seconds of compute with a 6 second average block time.
pub struct MaximumBlockWeight;

impl MaximumBlockWeight {
    /// Returns the value of this parameter type.
    pub const fn get() -> Weight {
        2 * WEIGHT_PER_SECOND
    }
}

impl<I: From<Weight>> ::frame_support::traits::Get<I> for MaximumBlockWeight {
    fn get() -> I {
        I::from(2 * WEIGHT_PER_SECOND)
    }
}

pub struct AvailableBlockRatio;

impl AvailableBlockRatio {
    /// Returns the value of this parameter type.
    pub const fn get() -> Perbill {
        Perbill::from_percent(75)
    }
}

impl<I: From<Perbill>> ::frame_support::traits::Get<I> for AvailableBlockRatio {
    fn get() -> I {
        I::from(Perbill::from_percent(75))
    }
}

/// Assume 10% of weight for average on_initialize calls.
pub struct MaximumExtrinsicWeight;

impl MaximumExtrinsicWeight {
    /// Returns the value of this parameter type.
    pub fn get() -> Weight {
        AvailableBlockRatio::get().saturating_sub(Perbill::from_percent(10))
            * MaximumBlockWeight::get()
    }
}

impl<I: From<Weight>> ::frame_support::traits::Get<I> for MaximumExtrinsicWeight {
    fn get() -> I {
        I::from(
            AvailableBlockRatio::get().saturating_sub(Perbill::from_percent(10))
                * MaximumBlockWeight::get(),
        )
    }
}

pub struct MaximumBlockLength;

impl MaximumBlockLength {
    /// Returns the value of this parameter type.
    pub const fn get() -> u32 {
        5 * 1024 * 1024
    }
}

impl<I: From<u32>> ::frame_support::traits::Get<I> for MaximumBlockLength {
    fn get() -> I {
        I::from(5 * 1024 * 1024)
    }
}

pub struct Version;

impl Version {
    /// Returns the value of this parameter type.
    pub const fn get() -> RuntimeVersion {
        VERSION
    }
}

impl<I: From<RuntimeVersion>> ::frame_support::traits::Get<I> for Version {
    fn get() -> I {
        I::from(VERSION)
    }
}

impl frame_system::Trait for Runtime {
    /// The basic call filter to use in dispatchable.
    type BaseCallFilter = ();
    /// The identifier used to distinguish between accounts.
    type AccountId = AccountId;
    /// The aggregated dispatch type that is available for extrinsics.
    type Call = Call;
    /// The lookup mechanism to get account ID from whatever is passed in dispatchers.
    type Lookup = IdentityLookup<AccountId>;
    /// The index type for storing how many extrinsics an account has signed.
    type Index = Index;
    /// The index type for blocks.
    type BlockNumber = BlockNumber;
    /// The type for hashing blocks and tries.
    type Hash = Hash;
    /// The hashing algorithm used.
    type Hashing = BlakeTwo256;
    /// The header type.
    type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// The ubiquitous event type.
    type Event = Event;
    /// The ubiquitous origin type.
    type Origin = Origin;
    /// Maximum number of block number to block hash mappings to keep (oldest pruned first).
    type BlockHashCount = BlockHashCount;
    /// Maximum weight of each block.
    type MaximumBlockWeight = MaximumBlockWeight;
    /// The weight of database operations that the runtime can invoke.
    type DbWeight = RocksDbWeight;
    /// The weight of the overhead invoked on the block import process, independent of the
    /// extrinsics included in that block.
    type BlockExecutionWeight = BlockExecutionWeight;
    /// The base weight of any extrinsic processed by the runtime, independent of the
    /// logic of that extrinsic. (Signature verification, nonce increment, fee, etc...)
    type ExtrinsicBaseWeight = ExtrinsicBaseWeight;
    /// The maximum weight that a single extrinsic of `Normal` dispatch class can have,
    /// idependent of the logic of that extrinsics. (Roughly max block weight - average on
    /// initialize cost).
    type MaximumExtrinsicWeight = MaximumExtrinsicWeight;
    /// Maximum size of all encoded transactions (in bytes) that are allowed in one block.
    type MaximumBlockLength = MaximumBlockLength;
    /// Portion of the block weight that is available to all normal transactions.
    type AvailableBlockRatio = AvailableBlockRatio;
    /// Version of the runtime.
    type Version = Version;
    /// Converts a module to the index of the module in `construct_runtime!`.
    ///
    /// This type is being generated by `construct_runtime!`.
    type ModuleToIndex = ModuleToIndex;
    /// What to do if a new account is created.
    type OnNewAccount = ();
    /// What to do if an account is fully reaped from the system.
    type OnKilledAccount = ();
    /// The data to be stored in an account.
    type AccountData = pallet_balances::AccountData<Balance>;
    /// Weight information for the extrinsics of this pallet.
    type SystemWeightInfo = ();
}

impl pallet_aura::Trait for Runtime {
    type AuthorityId = AuraId;
}

impl pallet_grandpa::Trait for Runtime {
    type Event = Event;
    type Call = Call;
    type KeyOwnerProofSystem = ();
    type KeyOwnerProof =
    <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;
    type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
        KeyTypeId,
        GrandpaId,
    )>>::IdentificationTuple;
    type HandleEquivocation = ();
}

pub struct MinimumPeriod;

impl MinimumPeriod {
    /// Returns the value of this parameter type.
    pub const fn get() -> u64 {
        SLOT_DURATION / 2
    }
}

impl<I: From<u64>> ::frame_support::traits::Get<I> for MinimumPeriod {
    fn get() -> I {
        I::from(SLOT_DURATION / 2)
    }
}

impl pallet_timestamp::Trait for Runtime {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = Aura;
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

pub struct ExistentialDeposit;

impl ExistentialDeposit {
    /// Returns the value of this parameter type.
    pub const fn get() -> u128 {
        500
    }
}

impl<I: From<u128>> ::frame_support::traits::Get<I> for ExistentialDeposit {
    fn get() -> I {
        I::from(500)
    }
}

impl pallet_balances::Trait for Runtime {
    type Balance = Balance;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type Event = Event;
}

pub struct TransactionByteFee;

impl TransactionByteFee {
    /// Returns the value of this parameter type.
    pub const fn get() -> Balance {
        1
    }
}

impl<I: From<Balance>> ::frame_support::traits::Get<I> for TransactionByteFee {
    fn get() -> I {
        I::from(1)
    }
}

impl pallet_transaction_payment::Trait for Runtime {
    type Currency = Balances;
    type OnTransactionPayment = ();
    type TransactionByteFee = TransactionByteFee;
    type WeightToFee = IdentityFee<Balance>;
    type FeeMultiplierUpdate = ();
}

impl pallet_sudo::Trait for Runtime {
    type Event = Event;
    type Call = Call;
}

pub struct NickReservationFee;

impl NickReservationFee {
    /// Returns the value of this parameter type.
    pub const fn get() -> u128 {
        100
    }
}

impl<I: From<u128>> ::frame_support::traits::Get<I> for NickReservationFee {
    fn get() -> I {
        I::from(100)
    }
}

pub struct MinNickLength;

impl MinNickLength {
    /// Returns the value of this parameter type.
    pub const fn get() -> usize {
        8
    }
}

impl<I: From<usize>> ::frame_support::traits::Get<I> for MinNickLength {
    fn get() -> I {
        I::from(8)
    }
}

pub struct MaxNickLength;

impl MaxNickLength {
    /// Returns the value of this parameter type.
    pub const fn get() -> usize {
        32
    }
}

impl<I: From<usize>> ::frame_support::traits::Get<I> for MaxNickLength {
    fn get() -> I {
        I::from(32)
    }
}

/// Configure the pallet template in pallets/template.
impl template::Trait for Runtime {
    type Currency = pallet_balances::Module<Runtime>;
    type ReservationFee = NickReservationFee;
    type Slashed = ();
    type ForceOrigin = EnsureRoot<AccountId>;
    type MinLength = MinNickLength;
    type MaxLength = MaxNickLength;
    type Event = Event;
}

#[doc(hidden)]
mod sp_api_hidden_includes_construct_runtime {
    pub extern crate frame_support as hidden_include;
}

pub struct Runtime;

#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Runtime {
    #[inline]
    fn clone(&self) -> Runtime {
        {
            *self
        }
    }
}

#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for Runtime {}

impl ::core::marker::StructuralPartialEq for Runtime {}

#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for Runtime {
    #[inline]
    fn eq(&self, other: &Runtime) -> bool {
        match *other {
            Runtime => match *self {
                Runtime => true,
            },
        }
    }
}

impl ::core::marker::StructuralEq for Runtime {}

#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for Runtime {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {}
    }
}

impl core::fmt::Debug for Runtime {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_tuple("Runtime").finish()
    }
}

impl self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::GetNodeBlockType for Runtime { type NodeBlock = opaque::Block; }

impl self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::GetRuntimeBlockType for Runtime { type RuntimeBlock = Block; }

#[allow(non_camel_case_types)]
pub enum Event {
    frame_system(frame_system::Event<Runtime>),
    pallet_grandpa(pallet_grandpa::Event),
    pallet_balances(pallet_balances::Event<Runtime>),
    pallet_sudo(pallet_sudo::Event<Runtime>),
    template(template::Event<Runtime>),
}

#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::clone::Clone for Event {
    #[inline]
    fn clone(&self) -> Event {
        match (&*self, ) {
            (&Event::frame_system(ref __self_0), ) => {
                Event::frame_system(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Event::pallet_grandpa(ref __self_0), ) => {
                Event::pallet_grandpa(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Event::pallet_balances(ref __self_0), ) => {
                Event::pallet_balances(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Event::pallet_sudo(ref __self_0), ) => {
                Event::pallet_sudo(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Event::template(ref __self_0), ) => {
                Event::template(::core::clone::Clone::clone(&(*__self_0)))
            }
        }
    }
}

#[allow(non_camel_case_types)]
impl ::core::marker::StructuralPartialEq for Event {}

#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::cmp::PartialEq for Event {
    #[inline]
    fn eq(&self, other: &Event) -> bool {
        {
            let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) };
            let __arg_1_vi = unsafe { ::core::intrinsics::discriminant_value(&*other) };
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Event::frame_system(ref __self_0), &Event::frame_system(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (
                        &Event::pallet_grandpa(ref __self_0),
                        &Event::pallet_grandpa(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    (
                        &Event::pallet_balances(ref __self_0),
                        &Event::pallet_balances(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    (&Event::pallet_sudo(ref __self_0), &Event::pallet_sudo(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Event::template(ref __self_0), &Event::template(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &Event) -> bool {
        {
            let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) };
            let __arg_1_vi = unsafe { ::core::intrinsics::discriminant_value(&*other) };
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Event::frame_system(ref __self_0), &Event::frame_system(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (
                        &Event::pallet_grandpa(ref __self_0),
                        &Event::pallet_grandpa(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    (
                        &Event::pallet_balances(ref __self_0),
                        &Event::pallet_balances(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    (&Event::pallet_sudo(ref __self_0), &Event::pallet_sudo(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Event::template(ref __self_0), &Event::template(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                true
            }
        }
    }
}

#[allow(non_camel_case_types)]
impl ::core::marker::StructuralEq for Event {}

#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::cmp::Eq for Event {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<frame_system::Event<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<pallet_grandpa::Event>;
            let _: ::core::cmp::AssertParamIsEq<pallet_balances::Event<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<pallet_sudo::Event<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<template::Event<Runtime>>;
        }
    }
}

const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Encode for Event {
        fn encode_to<EncOut: _parity_scale_codec::Output>(&self, dest: &mut EncOut) {
            match *self {
                Event::frame_system(ref aa) => {
                    dest.push_byte(0usize as u8);
                    dest.push(aa);
                }
                Event::pallet_grandpa(ref aa) => {
                    dest.push_byte(1usize as u8);
                    dest.push(aa);
                }
                Event::pallet_balances(ref aa) => {
                    dest.push_byte(2usize as u8);
                    dest.push(aa);
                }
                Event::pallet_sudo(ref aa) => {
                    dest.push_byte(3usize as u8);
                    dest.push(aa);
                }
                Event::template(ref aa) => {
                    dest.push_byte(4usize as u8);
                    dest.push(aa);
                }
                _ => (),
            }
        }
    }
    impl _parity_scale_codec::EncodeLike for Event {}
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Decode for Event {
        fn decode<DecIn: _parity_scale_codec::Input>(
            input: &mut DecIn,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            match input.read_byte()? {
                x if x == 0usize as u8 => Ok(Event::frame_system({
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field Event :: frame_system.0".into()),
                        Ok(a) => a,
                    }
                })),
                x if x == 1usize as u8 => Ok(Event::pallet_grandpa({
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => {
                            return Err("Error decoding field Event :: pallet_grandpa.0".into());
                        }
                        Ok(a) => a,
                    }
                })),
                x if x == 2usize as u8 => Ok(Event::pallet_balances({
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => {
                            return Err("Error decoding field Event :: pallet_balances.0".into());
                        }
                        Ok(a) => a,
                    }
                })),
                x if x == 3usize as u8 => Ok(Event::pallet_sudo({
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field Event :: pallet_sudo.0".into()),
                        Ok(a) => a,
                    }
                })),
                x if x == 4usize as u8 => Ok(Event::template({
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field Event :: template.0".into()),
                        Ok(a) => a,
                    }
                })),
                x => Err("No such variant in enum Event".into()),
            }
        }
    }
};

impl core::fmt::Debug for Event {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::frame_system(ref a0) => fmt.debug_tuple("Event::frame_system").field(a0).finish(),
            Self::pallet_grandpa(ref a0) => {
                fmt.debug_tuple("Event::pallet_grandpa").field(a0).finish()
            }
            Self::pallet_balances(ref a0) => {
                fmt.debug_tuple("Event::pallet_balances").field(a0).finish()
            }
            Self::pallet_sudo(ref a0) => fmt.debug_tuple("Event::pallet_sudo").field(a0).finish(),
            Self::template(ref a0) => fmt.debug_tuple("Event::template").field(a0).finish(),
            _ => Ok(()),
        }
    }
}

impl From<frame_system::Event<Runtime>> for Event {
    fn from(x: frame_system::Event<Runtime>) -> Self {
        Event::frame_system(x)
    }
}

impl ::frame_support::sp_std::convert::TryInto<frame_system::Event<Runtime>> for Event {
    type Error = ();
    fn try_into(
        self,
    ) -> ::frame_support::sp_std::result::Result<frame_system::Event<Runtime>, Self::Error> {
        match self {
            Self::frame_system(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}

impl From<pallet_grandpa::Event> for Event {
    fn from(x: pallet_grandpa::Event) -> Self {
        Event::pallet_grandpa(x)
    }
}

impl ::frame_support::sp_std::convert::TryInto<pallet_grandpa::Event> for Event {
    type Error = ();
    fn try_into(
        self,
    ) -> ::frame_support::sp_std::result::Result<pallet_grandpa::Event, Self::Error> {
        match self {
            Self::pallet_grandpa(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}

impl From<pallet_balances::Event<Runtime>> for Event {
    fn from(x: pallet_balances::Event<Runtime>) -> Self {
        Event::pallet_balances(x)
    }
}

impl ::frame_support::sp_std::convert::TryInto<pallet_balances::Event<Runtime>> for Event {
    type Error = ();
    fn try_into(
        self,
    ) -> ::frame_support::sp_std::result::Result<pallet_balances::Event<Runtime>, Self::Error> {
        match self {
            Self::pallet_balances(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}

impl From<pallet_sudo::Event<Runtime>> for Event {
    fn from(x: pallet_sudo::Event<Runtime>) -> Self {
        Event::pallet_sudo(x)
    }
}

impl ::frame_support::sp_std::convert::TryInto<pallet_sudo::Event<Runtime>> for Event {
    type Error = ();
    fn try_into(
        self,
    ) -> ::frame_support::sp_std::result::Result<pallet_sudo::Event<Runtime>, Self::Error> {
        match self {
            Self::pallet_sudo(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}

impl From<template::Event<Runtime>> for Event {
    fn from(x: template::Event<Runtime>) -> Self {
        Event::template(x)
    }
}

impl ::frame_support::sp_std::convert::TryInto<template::Event<Runtime>> for Event {
    type Error = ();
    fn try_into(
        self,
    ) -> ::frame_support::sp_std::result::Result<template::Event<Runtime>, Self::Error> {
        match self {
            Self::template(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}

impl Runtime {
    #[allow(dead_code)]
    pub fn outer_event_metadata() -> ::frame_support::event::OuterEventMetadata {
        ::frame_support::event::OuterEventMetadata {
            name: ::frame_support::event::DecodeDifferent::Encode("Event"),
            events: ::frame_support::event::DecodeDifferent::Encode(&[
                (
                    "frame_system",
                    ::frame_support::event::FnEncode(frame_system::Event::<Runtime>::metadata),
                ),
                (
                    "pallet_grandpa",
                    ::frame_support::event::FnEncode(pallet_grandpa::Event::metadata),
                ),
                (
                    "pallet_balances",
                    ::frame_support::event::FnEncode(pallet_balances::Event::<Runtime>::metadata),
                ),
                (
                    "pallet_sudo",
                    ::frame_support::event::FnEncode(pallet_sudo::Event::<Runtime>::metadata),
                ),
                (
                    "template",
                    ::frame_support::event::FnEncode(template::Event::<Runtime>::metadata),
                ),
            ]),
        }
    }
    #[allow(dead_code)]
    pub fn __module_events_frame_system() -> &'static [::frame_support::event::EventMetadata] {
        frame_system::Event::<Runtime>::metadata()
    }
    #[allow(dead_code)]
    pub fn __module_events_pallet_grandpa() -> &'static [::frame_support::event::EventMetadata] {
        pallet_grandpa::Event::metadata()
    }
    #[allow(dead_code)]
    pub fn __module_events_pallet_balances() -> &'static [::frame_support::event::EventMetadata] {
        pallet_balances::Event::<Runtime>::metadata()
    }
    #[allow(dead_code)]
    pub fn __module_events_pallet_sudo() -> &'static [::frame_support::event::EventMetadata] {
        pallet_sudo::Event::<Runtime>::metadata()
    }
    #[allow(dead_code)]
    pub fn __module_events_template() -> &'static [::frame_support::event::EventMetadata] {
        template::Event::<Runtime>::metadata()
    }
}

pub struct Origin {
    caller: OriginCaller,
    // :: can also be used as a way to specify generic types when they cannot otherwise be inferred;
    // this is called the turbofish.
    filter: ::frame_support::sp_std::rc::Rc<
        Box<dyn Fn(&<Runtime as frame_system::Trait>::Call) -> bool>,
    >,
}

#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Origin {
    #[inline]
    fn clone(&self) -> Origin {
        match *self {
            Origin {
                caller: ref __self_0_0,
                filter: ref __self_0_1,
            } => Origin {
                caller: ::core::clone::Clone::clone(&(*__self_0_0)),
                filter: ::core::clone::Clone::clone(&(*__self_0_1)),
            },
        }
    }
}

#[cfg(feature = "std")]
impl ::frame_support::sp_std::fmt::Debug for Origin {
    fn fmt(
        &self,
        fmt: &mut ::frame_support::sp_std::fmt::Formatter,
    ) -> ::frame_support::sp_std::result::Result<(), ::frame_support::sp_std::fmt::Error> {
        fmt.debug_struct("Origin")
            .field("caller", &self.caller)
            .field("filter", &"[function ptr]")
            .finish()
    }
}

impl ::frame_support::traits::OriginTrait for Origin {
    type Call = <Runtime as frame_system::Trait>::Call;
    type PalletsOrigin = OriginCaller;
    fn add_filter(&mut self, filter: impl Fn(&Self::Call) -> bool + 'static) {
        let f = self.filter.clone();
        self.filter =
            ::frame_support::sp_std::rc::Rc::new(Box::new(move |call| f(call) && filter(call)));
    }
    fn reset_filter(&mut self) {
        let filter =
            <<Runtime as frame_system::Trait>::BaseCallFilter as ::frame_support::traits::Filter<
                <Runtime as frame_system::Trait>::Call,
            >>::filter;
        self.filter = ::frame_support::sp_std::rc::Rc::new(Box::new(filter));
    }
    fn set_caller_from(&mut self, other: impl Into<Self>) {
        self.caller = other.into().caller
    }
    fn filter_call(&self, call: &Self::Call) -> bool {
        (self.filter)(call)
    }
    fn caller(&self) -> &Self::PalletsOrigin {
        &self.caller
    }
}

#[allow(non_camel_case_types)]
pub enum OriginCaller {
    system(frame_system::Origin<Runtime>),
    #[allow(dead_code)]
    Void(::frame_support::Void),
}

#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::clone::Clone for OriginCaller {
    #[inline]
    fn clone(&self) -> OriginCaller {
        match (&*self, ) {
            (&OriginCaller::system(ref __self_0), ) => {
                OriginCaller::system(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&OriginCaller::Void(ref __self_0), ) => {
                OriginCaller::Void(::core::clone::Clone::clone(&(*__self_0)))
            }
        }
    }
}

#[allow(non_camel_case_types)]
impl ::core::marker::StructuralPartialEq for OriginCaller {}

#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::cmp::PartialEq for OriginCaller {
    #[inline]
    fn eq(&self, other: &OriginCaller) -> bool {
        {
            let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) };
            let __arg_1_vi = unsafe { ::core::intrinsics::discriminant_value(&*other) };
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&OriginCaller::system(ref __self_0), &OriginCaller::system(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&OriginCaller::Void(ref __self_0), &OriginCaller::Void(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &OriginCaller) -> bool {
        {
            let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) };
            let __arg_1_vi = unsafe { ::core::intrinsics::discriminant_value(&*other) };
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&OriginCaller::system(ref __self_0), &OriginCaller::system(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&OriginCaller::Void(ref __self_0), &OriginCaller::Void(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                true
            }
        }
    }
}

#[allow(non_camel_case_types)]
impl ::core::marker::StructuralEq for OriginCaller {}

#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::cmp::Eq for OriginCaller {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<frame_system::Origin<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<::frame_support::Void>;
        }
    }
}

impl core::fmt::Debug for OriginCaller {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::system(ref a0) => fmt.debug_tuple("OriginCaller::system").field(a0).finish(),
            Self::Void(ref a0) => fmt.debug_tuple("OriginCaller::Void").field(a0).finish(),
            _ => Ok(()),
        }
    }
}

const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Encode for OriginCaller {
        fn encode_to<EncOut: _parity_scale_codec::Output>(&self, dest: &mut EncOut) {
            match *self {
                OriginCaller::system(ref aa) => {
                    dest.push_byte(0usize as u8);
                    dest.push(aa);
                }
                OriginCaller::Void(ref aa) => {
                    dest.push_byte(1usize as u8);
                    dest.push(aa);
                }
                _ => (),
            }
        }
    }
    impl _parity_scale_codec::EncodeLike for OriginCaller {}
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Decode for OriginCaller {
        fn decode<DecIn: _parity_scale_codec::Input>(
            input: &mut DecIn,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            match input.read_byte()? {
                x if x == 0usize as u8 => Ok(OriginCaller::system({
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => {
                            return Err("Error decoding field OriginCaller :: system.0".into());
                        }
                        Ok(a) => a,
                    }
                })),
                x if x == 1usize as u8 => Ok(OriginCaller::Void({
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field OriginCaller :: Void.0".into()),
                        Ok(a) => a,
                    }
                })),
                x => Err("No such variant in enum OriginCaller".into()),
            }
        }
    }
};

#[allow(dead_code)]
impl Origin {
    /// Create with system none origin and `frame-system::Trait::BaseCallFilter`.
    pub fn none() -> Self {
        frame_system::RawOrigin::None.into()
    }
    /// Create with system root origin and no filter.
    pub fn root() -> Self {
        frame_system::RawOrigin::Root.into()
    }
    /// Create with system signed origin and `frame-system::Trait::BaseCallFilter`.
    pub fn signed(by: <Runtime as frame_system::Trait>::AccountId) -> Self {
        frame_system::RawOrigin::Signed(by).into()
    }
}

impl From<frame_system::Origin<Runtime>> for OriginCaller {
    fn from(x: frame_system::Origin<Runtime>) -> Self {
        OriginCaller::system(x)
    }
}

impl From<frame_system::Origin<Runtime>> for Origin {
    /// Convert to runtime origin:
    /// * root origin is built with no filter
    /// * others use `frame-system::Trait::BaseCallFilter`
    fn from(x: frame_system::Origin<Runtime>) -> Self {
        let o: OriginCaller = x.into();
        o.into()
    }
}

impl From<OriginCaller> for Origin {
    fn from(x: OriginCaller) -> Self {
        let mut o = Origin {
            caller: x,
            filter: ::frame_support::sp_std::rc::Rc::new(Box::new(|_| true)),
        };
        if !match o.caller {
            OriginCaller::system(frame_system::Origin::<Runtime>::Root) => true,
            _ => false,
        } {
            ::frame_support::traits::OriginTrait::reset_filter(&mut o);
        }
        o
    }
}

impl Into<::frame_support::sp_std::result::Result<frame_system::Origin<Runtime>, Origin>>
for Origin
{
    /// NOTE: converting to pallet origin loses the origin filter information.
    fn into(self) -> ::frame_support::sp_std::result::Result<frame_system::Origin<Runtime>, Self> {
        if let OriginCaller::system(l) = self.caller {
            Ok(l)
        } else {
            Err(self)
        }
    }
}

impl From<Option<<Runtime as frame_system::Trait>::AccountId>> for Origin {
    /// Convert to runtime origin with caller being system signed or none and use filter
    /// `frame-system::Trait::BaseCallFilter`.
    fn from(x: Option<<Runtime as frame_system::Trait>::AccountId>) -> Self {
        <frame_system::Origin<Runtime>>::from(x).into()
    }
}

pub type System = frame_system::Module<Runtime>;
pub type RandomnessCollectiveFlip = pallet_randomness_collective_flip::Module<Runtime>;
pub type Timestamp = pallet_timestamp::Module<Runtime>;
pub type Aura = pallet_aura::Module<Runtime>;
pub type Grandpa = pallet_grandpa::Module<Runtime>;
pub type Balances = pallet_balances::Module<Runtime>;
pub type TransactionPayment = pallet_transaction_payment::Module<Runtime>;
pub type Sudo = pallet_sudo::Module<Runtime>;
pub type TemplateModule = template::Module<Runtime>;
type AllModules = ((
    TemplateModule,
    (
        Sudo,
        (
            TransactionPayment,
            (
                Balances,
                (Grandpa, (Aura, (Timestamp, (RandomnessCollectiveFlip, )))),
            ),
        ),
    ),
));

/// Provides an implementation of `ModuleToIndex` to map a module
/// to its index in the runtime.
pub struct ModuleToIndex;

impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::ModuleToIndex
for ModuleToIndex
{
    fn module_to_index<M: 'static>() -> Option<usize> {
        let type_id =
            self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                M,
            >();
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<System>() { return Some(0usize); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<RandomnessCollectiveFlip>() { return Some(1usize); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<Timestamp>() { return Some(2usize); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<Aura>() { return Some(3usize); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<Grandpa>() { return Some(4usize); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<Balances>() { return Some(5usize); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<TransactionPayment>() { return Some(6usize); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<Sudo>() { return Some(7usize); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<TemplateModule>() { return Some(8usize); }
        None
    }
}

pub enum Call {
    System(::frame_support::dispatch::CallableCallFor<System, Runtime>),
    RandomnessCollectiveFlip(
        ::frame_support::dispatch::CallableCallFor<RandomnessCollectiveFlip, Runtime>,
    ),
    Timestamp(::frame_support::dispatch::CallableCallFor<Timestamp, Runtime>),
    Grandpa(::frame_support::dispatch::CallableCallFor<Grandpa, Runtime>),
    Balances(::frame_support::dispatch::CallableCallFor<Balances, Runtime>),
    Sudo(::frame_support::dispatch::CallableCallFor<Sudo, Runtime>),
    TemplateModule(::frame_support::dispatch::CallableCallFor<TemplateModule, Runtime>),
}

#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Call {
    #[inline]
    fn clone(&self) -> Call {
        match (&*self, ) {
            (&Call::System(ref __self_0), ) => {
                Call::System(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::RandomnessCollectiveFlip(ref __self_0), ) => {
                Call::RandomnessCollectiveFlip(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::Timestamp(ref __self_0), ) => {
                Call::Timestamp(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::Grandpa(ref __self_0), ) => {
                Call::Grandpa(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::Balances(ref __self_0), ) => {
                Call::Balances(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::Sudo(ref __self_0), ) => Call::Sudo(::core::clone::Clone::clone(&(*__self_0))),
            (&Call::TemplateModule(ref __self_0), ) => {
                Call::TemplateModule(::core::clone::Clone::clone(&(*__self_0)))
            }
        }
    }
}

impl ::core::marker::StructuralPartialEq for Call {}

#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for Call {
    #[inline]
    fn eq(&self, other: &Call) -> bool {
        {
            let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) };
            let __arg_1_vi = unsafe { ::core::intrinsics::discriminant_value(&*other) };
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Call::System(ref __self_0), &Call::System(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (
                        &Call::RandomnessCollectiveFlip(ref __self_0),
                        &Call::RandomnessCollectiveFlip(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    (&Call::Timestamp(ref __self_0), &Call::Timestamp(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::Grandpa(ref __self_0), &Call::Grandpa(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::Balances(ref __self_0), &Call::Balances(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::Sudo(ref __self_0), &Call::Sudo(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::TemplateModule(ref __self_0), &Call::TemplateModule(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &Call) -> bool {
        {
            let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) };
            let __arg_1_vi = unsafe { ::core::intrinsics::discriminant_value(&*other) };
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Call::System(ref __self_0), &Call::System(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (
                        &Call::RandomnessCollectiveFlip(ref __self_0),
                        &Call::RandomnessCollectiveFlip(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    (&Call::Timestamp(ref __self_0), &Call::Timestamp(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::Grandpa(ref __self_0), &Call::Grandpa(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::Balances(ref __self_0), &Call::Balances(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::Sudo(ref __self_0), &Call::Sudo(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::TemplateModule(ref __self_0), &Call::TemplateModule(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                true
            }
        }
    }
}

impl ::core::marker::StructuralEq for Call {}

#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for Call {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<
                ::frame_support::dispatch::CallableCallFor<System, Runtime>,
            >;
            let _: ::core::cmp::AssertParamIsEq<
                ::frame_support::dispatch::CallableCallFor<RandomnessCollectiveFlip, Runtime>,
            >;
            let _: ::core::cmp::AssertParamIsEq<
                ::frame_support::dispatch::CallableCallFor<Timestamp, Runtime>,
            >;
            let _: ::core::cmp::AssertParamIsEq<
                ::frame_support::dispatch::CallableCallFor<Grandpa, Runtime>,
            >;
            let _: ::core::cmp::AssertParamIsEq<
                ::frame_support::dispatch::CallableCallFor<Balances, Runtime>,
            >;
            let _: ::core::cmp::AssertParamIsEq<
                ::frame_support::dispatch::CallableCallFor<Sudo, Runtime>,
            >;
            let _: ::core::cmp::AssertParamIsEq<
                ::frame_support::dispatch::CallableCallFor<TemplateModule, Runtime>,
            >;
        }
    }
}

const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Encode for Call {
        fn encode_to<EncOut: _parity_scale_codec::Output>(&self, dest: &mut EncOut) {
            match *self {
                Call::System(ref aa) => {
                    dest.push_byte(0usize as u8);
                    dest.push(aa);
                }
                Call::RandomnessCollectiveFlip(ref aa) => {
                    dest.push_byte(1usize as u8);
                    dest.push(aa);
                }
                Call::Timestamp(ref aa) => {
                    dest.push_byte(2usize as u8);
                    dest.push(aa);
                }
                Call::Grandpa(ref aa) => {
                    dest.push_byte(3usize as u8);
                    dest.push(aa);
                }
                Call::Balances(ref aa) => {
                    dest.push_byte(4usize as u8);
                    dest.push(aa);
                }
                Call::Sudo(ref aa) => {
                    dest.push_byte(5usize as u8);
                    dest.push(aa);
                }
                Call::TemplateModule(ref aa) => {
                    dest.push_byte(6usize as u8);
                    dest.push(aa);
                }
                _ => (),
            }
        }
    }
    impl _parity_scale_codec::EncodeLike for Call {}
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Decode for Call {
        fn decode<DecIn: _parity_scale_codec::Input>(
            input: &mut DecIn,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            match input.read_byte()? {
                x if x == 0usize as u8 => Ok(Call::System({
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field Call :: System.0".into()),
                        Ok(a) => a,
                    }
                })),
                x if x == 1usize as u8 => Ok(Call::RandomnessCollectiveFlip({
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => {
                            return Err(
                                "Error decoding field Call :: RandomnessCollectiveFlip.0".into()
                            );
                        }
                        Ok(a) => a,
                    }
                })),
                x if x == 2usize as u8 => Ok(Call::Timestamp({
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field Call :: Timestamp.0".into()),
                        Ok(a) => a,
                    }
                })),
                x if x == 3usize as u8 => Ok(Call::Grandpa({
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field Call :: Grandpa.0".into()),
                        Ok(a) => a,
                    }
                })),
                x if x == 4usize as u8 => Ok(Call::Balances({
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field Call :: Balances.0".into()),
                        Ok(a) => a,
                    }
                })),
                x if x == 5usize as u8 => Ok(Call::Sudo({
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field Call :: Sudo.0".into()),
                        Ok(a) => a,
                    }
                })),
                x if x == 6usize as u8 => Ok(Call::TemplateModule({
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => {
                            return Err("Error decoding field Call :: TemplateModule.0".into());
                        }
                        Ok(a) => a,
                    }
                })),
                x => Err("No such variant in enum Call".into()),
            }
        }
    }
};

impl core::fmt::Debug for Call {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::System(ref a0) => fmt.debug_tuple("Call::System").field(a0).finish(),
            Self::RandomnessCollectiveFlip(ref a0) => fmt
                .debug_tuple("Call::RandomnessCollectiveFlip")
                .field(a0)
                .finish(),
            Self::Timestamp(ref a0) => fmt.debug_tuple("Call::Timestamp").field(a0).finish(),
            Self::Grandpa(ref a0) => fmt.debug_tuple("Call::Grandpa").field(a0).finish(),
            Self::Balances(ref a0) => fmt.debug_tuple("Call::Balances").field(a0).finish(),
            Self::Sudo(ref a0) => fmt.debug_tuple("Call::Sudo").field(a0).finish(),
            Self::TemplateModule(ref a0) => {
                fmt.debug_tuple("Call::TemplateModule").field(a0).finish()
            }
            _ => Ok(()),
        }
    }
}

impl ::frame_support::dispatch::GetDispatchInfo for Call {
    fn get_dispatch_info(&self) -> ::frame_support::dispatch::DispatchInfo {
        match self {
            Call::System(call) => call.get_dispatch_info(),
            Call::RandomnessCollectiveFlip(call) => call.get_dispatch_info(),
            Call::Timestamp(call) => call.get_dispatch_info(),
            Call::Grandpa(call) => call.get_dispatch_info(),
            Call::Balances(call) => call.get_dispatch_info(),
            Call::Sudo(call) => call.get_dispatch_info(),
            Call::TemplateModule(call) => call.get_dispatch_info(),
        }
    }
}

impl ::frame_support::dispatch::GetCallMetadata for Call {
    fn get_call_metadata(&self) -> ::frame_support::dispatch::CallMetadata {
        use ::frame_support::dispatch::GetCallName;
        match self {
            Call::System(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "System";
                ::frame_support::dispatch::CallMetadata {
                    function_name,
                    pallet_name,
                }
            }
            Call::RandomnessCollectiveFlip(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "RandomnessCollectiveFlip";
                ::frame_support::dispatch::CallMetadata {
                    function_name,
                    pallet_name,
                }
            }
            Call::Timestamp(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "Timestamp";
                ::frame_support::dispatch::CallMetadata {
                    function_name,
                    pallet_name,
                }
            }
            Call::Grandpa(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "Grandpa";
                ::frame_support::dispatch::CallMetadata {
                    function_name,
                    pallet_name,
                }
            }
            Call::Balances(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "Balances";
                ::frame_support::dispatch::CallMetadata {
                    function_name,
                    pallet_name,
                }
            }
            Call::Sudo(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "Sudo";
                ::frame_support::dispatch::CallMetadata {
                    function_name,
                    pallet_name,
                }
            }
            Call::TemplateModule(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "TemplateModule";
                ::frame_support::dispatch::CallMetadata {
                    function_name,
                    pallet_name,
                }
            }
        }
    }
    fn get_module_names() -> &'static [&'static str] {
        &[
            "System",
            "RandomnessCollectiveFlip",
            "Timestamp",
            "Grandpa",
            "Balances",
            "Sudo",
            "TemplateModule",
        ]
    }
    fn get_call_names(module: &str) -> &'static [&'static str] {
        use ::frame_support::dispatch::{Callable, GetCallName};
        match module {
            "System" => <<System as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            "RandomnessCollectiveFlip" => <<RandomnessCollectiveFlip as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            "Timestamp" => <<Timestamp as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            "Grandpa" => <<Grandpa as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            "Balances" => <<Balances as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            "Sudo" => <<Sudo as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            "TemplateModule" => <<TemplateModule as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            _ => { { ::std::rt::begin_panic("internal error: entered unreachable code") } }
        }
    }
}

impl ::frame_support::dispatch::Dispatchable for Call {
    type Origin = Origin;
    type Trait = Call;
    type Info = ::frame_support::weights::DispatchInfo;
    type PostInfo = ::frame_support::weights::PostDispatchInfo;
    fn dispatch(self, origin: Origin) -> ::frame_support::dispatch::DispatchResultWithPostInfo {
        if !<Self::Origin as ::frame_support::traits::OriginTrait>::filter_call(&origin, &self) {
            return ::frame_support::sp_std::result::Result::Err(
                ::frame_support::dispatch::DispatchError::BadOrigin.into(),
            );
        }
        ::frame_support::traits::UnfilteredDispatchable::dispatch_bypass_filter(self, origin)
    }
}

impl ::frame_support::traits::UnfilteredDispatchable for Call {
    type Origin = Origin;
    fn dispatch_bypass_filter(
        self,
        origin: Origin,
    ) -> ::frame_support::dispatch::DispatchResultWithPostInfo {
        match self {
            Call::System(call) => {
                ::frame_support::traits::UnfilteredDispatchable::dispatch_bypass_filter(
                    call, origin,
                )
            }
            Call::RandomnessCollectiveFlip(call) => {
                ::frame_support::traits::UnfilteredDispatchable::dispatch_bypass_filter(
                    call, origin,
                )
            }
            Call::Timestamp(call) => {
                ::frame_support::traits::UnfilteredDispatchable::dispatch_bypass_filter(
                    call, origin,
                )
            }
            Call::Grandpa(call) => {
                ::frame_support::traits::UnfilteredDispatchable::dispatch_bypass_filter(
                    call, origin,
                )
            }
            Call::Balances(call) => {
                ::frame_support::traits::UnfilteredDispatchable::dispatch_bypass_filter(
                    call, origin,
                )
            }
            Call::Sudo(call) => {
                ::frame_support::traits::UnfilteredDispatchable::dispatch_bypass_filter(
                    call, origin,
                )
            }
            Call::TemplateModule(call) => {
                ::frame_support::traits::UnfilteredDispatchable::dispatch_bypass_filter(
                    call, origin,
                )
            }
        }
    }
}

impl
::frame_support::dispatch::IsSubType<
    ::frame_support::dispatch::CallableCallFor<System, Runtime>,
> for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(&self) -> Option<&::frame_support::dispatch::CallableCallFor<System, Runtime>> {
        match *self {
            Call::System(ref r) => Some(r),
            _ => None,
        }
    }
}

impl From<::frame_support::dispatch::CallableCallFor<System, Runtime>> for Call {
    fn from(call: ::frame_support::dispatch::CallableCallFor<System, Runtime>) -> Self {
        Call::System(call)
    }
}

impl
::frame_support::dispatch::IsSubType<
    ::frame_support::dispatch::CallableCallFor<RandomnessCollectiveFlip, Runtime>,
> for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<&::frame_support::dispatch::CallableCallFor<RandomnessCollectiveFlip, Runtime>>
    {
        match *self {
            Call::RandomnessCollectiveFlip(ref r) => Some(r),
            _ => None,
        }
    }
}

impl From<::frame_support::dispatch::CallableCallFor<RandomnessCollectiveFlip, Runtime>> for Call {
    fn from(
        call: ::frame_support::dispatch::CallableCallFor<RandomnessCollectiveFlip, Runtime>,
    ) -> Self {
        Call::RandomnessCollectiveFlip(call)
    }
}

impl
::frame_support::dispatch::IsSubType<
    ::frame_support::dispatch::CallableCallFor<Timestamp, Runtime>,
> for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<&::frame_support::dispatch::CallableCallFor<Timestamp, Runtime>> {
        match *self {
            Call::Timestamp(ref r) => Some(r),
            _ => None,
        }
    }
}

impl From<::frame_support::dispatch::CallableCallFor<Timestamp, Runtime>> for Call {
    fn from(call: ::frame_support::dispatch::CallableCallFor<Timestamp, Runtime>) -> Self {
        Call::Timestamp(call)
    }
}

impl
::frame_support::dispatch::IsSubType<
    ::frame_support::dispatch::CallableCallFor<Grandpa, Runtime>,
> for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(&self) -> Option<&::frame_support::dispatch::CallableCallFor<Grandpa, Runtime>> {
        match *self {
            Call::Grandpa(ref r) => Some(r),
            _ => None,
        }
    }
}

impl From<::frame_support::dispatch::CallableCallFor<Grandpa, Runtime>> for Call {
    fn from(call: ::frame_support::dispatch::CallableCallFor<Grandpa, Runtime>) -> Self {
        Call::Grandpa(call)
    }
}

impl
::frame_support::dispatch::IsSubType<
    ::frame_support::dispatch::CallableCallFor<Balances, Runtime>,
> for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<&::frame_support::dispatch::CallableCallFor<Balances, Runtime>> {
        match *self {
            Call::Balances(ref r) => Some(r),
            _ => None,
        }
    }
}

impl From<::frame_support::dispatch::CallableCallFor<Balances, Runtime>> for Call {
    fn from(call: ::frame_support::dispatch::CallableCallFor<Balances, Runtime>) -> Self {
        Call::Balances(call)
    }
}

impl ::frame_support::dispatch::IsSubType<::frame_support::dispatch::CallableCallFor<Sudo, Runtime>>
for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(&self) -> Option<&::frame_support::dispatch::CallableCallFor<Sudo, Runtime>> {
        match *self {
            Call::Sudo(ref r) => Some(r),
            _ => None,
        }
    }
}

impl From<::frame_support::dispatch::CallableCallFor<Sudo, Runtime>> for Call {
    fn from(call: ::frame_support::dispatch::CallableCallFor<Sudo, Runtime>) -> Self {
        Call::Sudo(call)
    }
}

impl
::frame_support::dispatch::IsSubType<
    ::frame_support::dispatch::CallableCallFor<TemplateModule, Runtime>,
> for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<&::frame_support::dispatch::CallableCallFor<TemplateModule, Runtime>> {
        match *self {
            Call::TemplateModule(ref r) => Some(r),
            _ => None,
        }
    }
}

impl From<::frame_support::dispatch::CallableCallFor<TemplateModule, Runtime>> for Call {
    fn from(call: ::frame_support::dispatch::CallableCallFor<TemplateModule, Runtime>) -> Self {
        Call::TemplateModule(call)
    }
}

impl Runtime {
    pub fn metadata() -> ::frame_support::metadata::RuntimeMetadataPrefixed {
        ::frame_support::metadata::RuntimeMetadataLastVersion {
            modules: ::frame_support::metadata::DecodeDifferent::Encode(&[::frame_support::metadata::ModuleMetadata {
                name: ::frame_support::metadata::DecodeDifferent::Encode("System"),
                storage: Some(::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(frame_system::Module::<Runtime>::storage_metadata))),
                calls: Some(::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(frame_system::Module::<Runtime>::call_functions))),
                event: Some(::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode({
                    #[allow(dead_code)]
                    enum ProcMacroHack { Value = ("Runtime :: [< __module_events_ frame_system >]", 0).1 }
                    { Runtime::__module_events_frame_system }
                }))),
                constants: ::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(frame_system::Module::<Runtime>::module_constants_metadata)),
                errors: ::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(<frame_system::Module<Runtime> as ::frame_support::metadata::ModuleErrorMetadata>::metadata)),
            }, ::frame_support::metadata::ModuleMetadata { name: ::frame_support::metadata::DecodeDifferent::Encode("RandomnessCollectiveFlip"), storage: Some(::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(pallet_randomness_collective_flip::Module::<Runtime>::storage_metadata))), calls: Some(::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(pallet_randomness_collective_flip::Module::<Runtime>::call_functions))), event: None, constants: ::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(pallet_randomness_collective_flip::Module::<Runtime>::module_constants_metadata)), errors: ::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(<pallet_randomness_collective_flip::Module<Runtime> as ::frame_support::metadata::ModuleErrorMetadata>::metadata)) }, ::frame_support::metadata::ModuleMetadata { name: ::frame_support::metadata::DecodeDifferent::Encode("Timestamp"), storage: Some(::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(pallet_timestamp::Module::<Runtime>::storage_metadata))), calls: Some(::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(pallet_timestamp::Module::<Runtime>::call_functions))), event: None, constants: ::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(pallet_timestamp::Module::<Runtime>::module_constants_metadata)), errors: ::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(<pallet_timestamp::Module<Runtime> as ::frame_support::metadata::ModuleErrorMetadata>::metadata)) }, ::frame_support::metadata::ModuleMetadata { name: ::frame_support::metadata::DecodeDifferent::Encode("Aura"), storage: None, calls: None, event: None, constants: ::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(pallet_aura::Module::<Runtime>::module_constants_metadata)), errors: ::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(<pallet_aura::Module<Runtime> as ::frame_support::metadata::ModuleErrorMetadata>::metadata)) }, ::frame_support::metadata::ModuleMetadata {
                name: ::frame_support::metadata::DecodeDifferent::Encode("Grandpa"),
                storage: Some(::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(pallet_grandpa::Module::<Runtime>::storage_metadata))),
                calls: Some(::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(pallet_grandpa::Module::<Runtime>::call_functions))),
                event: Some(::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode({
                    #[allow(dead_code)]
                    enum ProcMacroHack { Value = ("Runtime :: [< __module_events_ pallet_grandpa >]", 0).1 }
                    { Runtime::__module_events_pallet_grandpa }
                }))),
                constants: ::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(pallet_grandpa::Module::<Runtime>::module_constants_metadata)),
                errors: ::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(<pallet_grandpa::Module<Runtime> as ::frame_support::metadata::ModuleErrorMetadata>::metadata)),
            }, ::frame_support::metadata::ModuleMetadata {
                name: ::frame_support::metadata::DecodeDifferent::Encode("Balances"),
                storage: Some(::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(pallet_balances::Module::<Runtime>::storage_metadata))),
                calls: Some(::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(pallet_balances::Module::<Runtime>::call_functions))),
                event: Some(::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode({
                    #[allow(dead_code)]
                    enum ProcMacroHack { Value = ("Runtime :: [< __module_events_ pallet_balances >]", 0).1 }
                    { Runtime::__module_events_pallet_balances }
                }))),
                constants: ::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(pallet_balances::Module::<Runtime>::module_constants_metadata)),
                errors: ::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(<pallet_balances::Module<Runtime> as ::frame_support::metadata::ModuleErrorMetadata>::metadata)),
            }, ::frame_support::metadata::ModuleMetadata { name: ::frame_support::metadata::DecodeDifferent::Encode("TransactionPayment"), storage: Some(::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(pallet_transaction_payment::Module::<Runtime>::storage_metadata))), calls: None, event: None, constants: ::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(pallet_transaction_payment::Module::<Runtime>::module_constants_metadata)), errors: ::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(<pallet_transaction_payment::Module<Runtime> as ::frame_support::metadata::ModuleErrorMetadata>::metadata)) }, ::frame_support::metadata::ModuleMetadata {
                name: ::frame_support::metadata::DecodeDifferent::Encode("Sudo"),
                storage: Some(::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(pallet_sudo::Module::<Runtime>::storage_metadata))),
                calls: Some(::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(pallet_sudo::Module::<Runtime>::call_functions))),
                event: Some(::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode({
                    #[allow(dead_code)]
                    enum ProcMacroHack { Value = ("Runtime :: [< __module_events_ pallet_sudo >]", 0).1 }
                    { Runtime::__module_events_pallet_sudo }
                }))),
                constants: ::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(pallet_sudo::Module::<Runtime>::module_constants_metadata)),
                errors: ::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(<pallet_sudo::Module<Runtime> as ::frame_support::metadata::ModuleErrorMetadata>::metadata)),
            }, ::frame_support::metadata::ModuleMetadata {
                name: ::frame_support::metadata::DecodeDifferent::Encode("TemplateModule"),
                storage: Some(::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(template::Module::<Runtime>::storage_metadata))),
                calls: Some(::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(template::Module::<Runtime>::call_functions))),
                event: Some(::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode({
                    #[allow(dead_code)]
                    enum ProcMacroHack { Value = ("Runtime :: [< __module_events_ template >]", 0).1 }
                    { Runtime::__module_events_template }
                }))),
                constants: ::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(template::Module::<Runtime>::module_constants_metadata)),
                errors: ::frame_support::metadata::DecodeDifferent::Encode(::frame_support::metadata::FnEncode(<template::Module<Runtime> as ::frame_support::metadata::ModuleErrorMetadata>::metadata)),
            }]),
            extrinsic: ::frame_support::metadata::ExtrinsicMetadata { version: <UncheckedExtrinsic as ::frame_support::sp_runtime::traits::ExtrinsicMetadata>::VERSION, signed_extensions: <<UncheckedExtrinsic as ::frame_support::sp_runtime::traits::ExtrinsicMetadata>::SignedExtensions as ::frame_support::sp_runtime::traits::SignedExtension>::identifier().into_iter().map(::frame_support::metadata::DecodeDifferent::Encode).collect() },
        }.into()
    }
}

#[cfg(any(feature = "std", test))]
pub type SystemConfig = frame_system::GenesisConfig;
#[cfg(any(feature = "std", test))]
pub type AuraConfig = pallet_aura::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type GrandpaConfig = pallet_grandpa::GenesisConfig;
#[cfg(any(feature = "std", test))]
pub type BalancesConfig = pallet_balances::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type SudoConfig = pallet_sudo::GenesisConfig<Runtime>;

#[cfg(any(feature = "std", test))]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct GenesisConfig {
    pub frame_system: Option<SystemConfig>,
    pub pallet_aura: Option<AuraConfig>,
    pub pallet_grandpa: Option<GrandpaConfig>,
    pub pallet_balances: Option<BalancesConfig>,
    pub pallet_sudo: Option<SudoConfig>,
}

#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(rust_2018_idioms, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for GenesisConfig {
        fn serialize<__S>(&self, __serializer: __S) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "GenesisConfig",
                false as usize + 1 + 1 + 1 + 1 + 1,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "frameSystem",
                &self.frame_system,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "palletAura",
                &self.pallet_aura,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "palletGrandpa",
                &self.pallet_grandpa,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "palletBalances",
                &self.pallet_balances,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "palletSudo",
                &self.pallet_sudo,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(rust_2018_idioms, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for GenesisConfig {
        fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::export::Formatter,
                ) -> _serde::export::fmt::Result {
                    _serde::export::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::export::Ok(__Field::__field0),
                        1u64 => _serde::export::Ok(__Field::__field1),
                        2u64 => _serde::export::Ok(__Field::__field2),
                        3u64 => _serde::export::Ok(__Field::__field3),
                        4u64 => _serde::export::Ok(__Field::__field4),
                        _ => _serde::export::Err(_serde::de::Error::invalid_value(
                            _serde::de::Unexpected::Unsigned(__value),
                            &"field index 0 <= i < 5",
                        )),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                {
                    match __value {
                        "frameSystem" => _serde::export::Ok(__Field::__field0),
                        "palletAura" => _serde::export::Ok(__Field::__field1),
                        "palletGrandpa" => _serde::export::Ok(__Field::__field2),
                        "palletBalances" => _serde::export::Ok(__Field::__field3),
                        "palletSudo" => _serde::export::Ok(__Field::__field4),
                        _ => _serde::export::Err(_serde::de::Error::unknown_field(__value, FIELDS)),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                {
                    match __value {
                        b"frameSystem" => _serde::export::Ok(__Field::__field0),
                        b"palletAura" => _serde::export::Ok(__Field::__field1),
                        b"palletGrandpa" => _serde::export::Ok(__Field::__field2),
                        b"palletBalances" => _serde::export::Ok(__Field::__field3),
                        b"palletSudo" => _serde::export::Ok(__Field::__field4),
                        _ => {
                            let __value = &_serde::export::from_utf8_lossy(__value);
                            _serde::export::Err(_serde::de::Error::unknown_field(__value, FIELDS))
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct __Visitor<'de> {
                marker: _serde::export::PhantomData<GenesisConfig>,
                lifetime: _serde::export::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = GenesisConfig;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::export::Formatter,
                ) -> _serde::export::fmt::Result {
                    _serde::export::Formatter::write_str(__formatter, "struct GenesisConfig")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match match _serde::de::SeqAccess::next_element::<
                        Option<SystemConfig>,
                    >(&mut __seq)
                    {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    } {
                        _serde::export::Some(__value) => __value,
                        _serde::export::None => {
                            return _serde::export::Err(_serde::de::Error::invalid_length(
                                0usize,
                                &"struct GenesisConfig with 5 elements",
                            ));
                        }
                    };
                    let __field1 = match match _serde::de::SeqAccess::next_element::<
                        Option<AuraConfig>,
                    >(&mut __seq)
                    {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    } {
                        _serde::export::Some(__value) => __value,
                        _serde::export::None => {
                            return _serde::export::Err(_serde::de::Error::invalid_length(
                                1usize,
                                &"struct GenesisConfig with 5 elements",
                            ));
                        }
                    };
                    let __field2 = match match _serde::de::SeqAccess::next_element::<
                        Option<GrandpaConfig>,
                    >(&mut __seq)
                    {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    } {
                        _serde::export::Some(__value) => __value,
                        _serde::export::None => {
                            return _serde::export::Err(_serde::de::Error::invalid_length(
                                2usize,
                                &"struct GenesisConfig with 5 elements",
                            ));
                        }
                    };
                    let __field3 = match match _serde::de::SeqAccess::next_element::<
                        Option<BalancesConfig>,
                    >(&mut __seq)
                    {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    } {
                        _serde::export::Some(__value) => __value,
                        _serde::export::None => {
                            return _serde::export::Err(_serde::de::Error::invalid_length(
                                3usize,
                                &"struct GenesisConfig with 5 elements",
                            ));
                        }
                    };
                    let __field4 = match match _serde::de::SeqAccess::next_element::<
                        Option<SudoConfig>,
                    >(&mut __seq)
                    {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    } {
                        _serde::export::Some(__value) => __value,
                        _serde::export::None => {
                            return _serde::export::Err(_serde::de::Error::invalid_length(
                                4usize,
                                &"struct GenesisConfig with 5 elements",
                            ));
                        }
                    };
                    _serde::export::Ok(GenesisConfig {
                        frame_system: __field0,
                        pallet_aura: __field1,
                        pallet_grandpa: __field2,
                        pallet_balances: __field3,
                        pallet_sudo: __field4,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::export::Option<Option<SystemConfig>> =
                        _serde::export::None;
                    let mut __field1: _serde::export::Option<Option<AuraConfig>> =
                        _serde::export::None;
                    let mut __field2: _serde::export::Option<Option<GrandpaConfig>> =
                        _serde::export::None;
                    let mut __field3: _serde::export::Option<Option<BalancesConfig>> =
                        _serde::export::None;
                    let mut __field4: _serde::export::Option<Option<SudoConfig>> =
                        _serde::export::None;
                    while let _serde::export::Some(__key) =
                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    }
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::export::Option::is_some(&__field0) {
                                    return _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "frameSystem",
                                        ),
                                    );
                                }
                                __field0 = _serde::export::Some(
                                    match _serde::de::MapAccess::next_value::<Option<SystemConfig>>(
                                        &mut __map,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field1 => {
                                if _serde::export::Option::is_some(&__field1) {
                                    return _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "palletAura",
                                        ),
                                    );
                                }
                                __field1 = _serde::export::Some(
                                    match _serde::de::MapAccess::next_value::<Option<AuraConfig>>(
                                        &mut __map,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field2 => {
                                if _serde::export::Option::is_some(&__field2) {
                                    return _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "palletGrandpa",
                                        ),
                                    );
                                }
                                __field2 = _serde::export::Some(
                                    match _serde::de::MapAccess::next_value::<Option<GrandpaConfig>>(
                                        &mut __map,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field3 => {
                                if _serde::export::Option::is_some(&__field3) {
                                    return _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "palletBalances",
                                        ),
                                    );
                                }
                                __field3 = _serde::export::Some(
                                    match _serde::de::MapAccess::next_value::<Option<BalancesConfig>>(
                                        &mut __map,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field4 => {
                                if _serde::export::Option::is_some(&__field4) {
                                    return _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "palletSudo",
                                        ),
                                    );
                                }
                                __field4 = _serde::export::Some(
                                    match _serde::de::MapAccess::next_value::<Option<SudoConfig>>(
                                        &mut __map,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                );
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::export::Some(__field0) => __field0,
                        _serde::export::None => {
                            match _serde::private::de::missing_field("frameSystem") {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::export::Some(__field1) => __field1,
                        _serde::export::None => {
                            match _serde::private::de::missing_field("palletAura") {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::export::Some(__field2) => __field2,
                        _serde::export::None => {
                            match _serde::private::de::missing_field("palletGrandpa") {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        }
                    };
                    let __field3 = match __field3 {
                        _serde::export::Some(__field3) => __field3,
                        _serde::export::None => {
                            match _serde::private::de::missing_field("palletBalances") {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        }
                    };
                    let __field4 = match __field4 {
                        _serde::export::Some(__field4) => __field4,
                        _serde::export::None => {
                            match _serde::private::de::missing_field("palletSudo") {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::export::Ok(GenesisConfig {
                        frame_system: __field0,
                        pallet_aura: __field1,
                        pallet_grandpa: __field2,
                        pallet_balances: __field3,
                        pallet_sudo: __field4,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &[
                "frameSystem",
                "palletAura",
                "palletGrandpa",
                "palletBalances",
                "palletSudo",
            ];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "GenesisConfig",
                FIELDS,
                __Visitor {
                    marker: _serde::export::PhantomData::<GenesisConfig>,
                    lifetime: _serde::export::PhantomData,
                },
            )
        }
    }
};

#[cfg(any(feature = "std", test))]
impl ::sp_runtime::BuildStorage for GenesisConfig {
    fn assimilate_storage(
        &self,
        storage: &mut ::sp_runtime::Storage,
    ) -> std::result::Result<(), String> {
        if let Some(ref extra) = self.frame_system {
            ::sp_runtime::BuildModuleGenesisStorage::<
                Runtime,
                frame_system::__InherentHiddenInstance,
            >::build_module_genesis_storage(extra, storage)?;
        }
        if let Some(ref extra) = self.pallet_aura {
            ::sp_runtime::BuildModuleGenesisStorage::<
                Runtime,
                pallet_aura::__InherentHiddenInstance,
            >::build_module_genesis_storage(extra, storage)?;
        }
        if let Some(ref extra) = self.pallet_grandpa {
            ::sp_runtime::BuildModuleGenesisStorage::<
                Runtime,
                pallet_grandpa::__InherentHiddenInstance,
            >::build_module_genesis_storage(extra, storage)?;
        }
        if let Some(ref extra) = self.pallet_balances {
            ::sp_runtime::BuildModuleGenesisStorage::<
                Runtime,
                pallet_balances::__InherentHiddenInstance,
            >::build_module_genesis_storage(extra, storage)?;
        }
        if let Some(ref extra) = self.pallet_sudo {
            ::sp_runtime::BuildModuleGenesisStorage::<
                Runtime,
                pallet_sudo::__InherentHiddenInstance,
            >::build_module_genesis_storage(extra, storage)?;
        }
        Ok(())
    }
}

trait InherentDataExt {
    fn create_extrinsics(
        &self,
    ) -> ::frame_support::inherent::Vec<<Block as ::frame_support::inherent::BlockT>::Extrinsic>;
    fn check_extrinsics(&self, block: &Block) -> ::frame_support::inherent::CheckInherentsResult;
}

impl InherentDataExt for ::frame_support::inherent::InherentData {
    fn create_extrinsics(
        &self,
    ) -> ::frame_support::inherent::Vec<<Block as ::frame_support::inherent::BlockT>::Extrinsic>
    {
        use ::frame_support::inherent::{ProvideInherent, Extrinsic};
        let mut inherents = Vec::new();
        if let Some(inherent) = Timestamp::create_inherent(self) {
            inherents.push(UncheckedExtrinsic::new(inherent.into(), None).expect(
                "Runtime UncheckedExtrinsic is not Opaque, so it has to return `Some`; qed",
            ));
        }
        if let Some(inherent) = Aura::create_inherent(self) {
            inherents.push(UncheckedExtrinsic::new(inherent.into(), None).expect(
                "Runtime UncheckedExtrinsic is not Opaque, so it has to return `Some`; qed",
            ));
        }
        inherents
    }
    fn check_extrinsics(&self, block: &Block) -> ::frame_support::inherent::CheckInherentsResult {
        use ::frame_support::inherent::{ProvideInherent, IsFatalError};
        use ::frame_support::dispatch::IsSubType;
        let mut result = ::frame_support::inherent::CheckInherentsResult::new();
        for xt in block.extrinsics() {
            if ::frame_support::inherent::Extrinsic::is_signed(xt).unwrap_or(false) {
                break;
            }
            {
                if let Some(call) = IsSubType::<_>::is_sub_type(&xt.function) {
                    if let Err(e) = Timestamp::check_inherent(call, self) {
                        result
                            .put_error(Timestamp::INHERENT_IDENTIFIER, &e)
                            .expect("There is only one fatal error; qed");
                        if e.is_fatal_error() {
                            return result;
                        }
                    }
                }
            }
            {
                if let Some(call) = IsSubType::<_>::is_sub_type(&xt.function) {
                    if let Err(e) = Aura::check_inherent(call, self) {
                        result
                            .put_error(Aura::INHERENT_IDENTIFIER, &e)
                            .expect("There is only one fatal error; qed");
                        if e.is_fatal_error() {
                            return result;
                        }
                    }
                }
            }
        }
        match Timestamp::is_inherent_required(self) {
            Ok(Some(e)) => {
                let found = block.extrinsics().iter().any(|xt| {
                    if ::frame_support::inherent::Extrinsic::is_signed(xt).unwrap_or(false) {
                        return false;
                    }
                    let call: Option<&<Timestamp as ProvideInherent>::Call> =
                        xt.function.is_sub_type();
                    call.is_some()
                });
                if !found {
                    result
                        .put_error(Timestamp::INHERENT_IDENTIFIER, &e)
                        .expect("There is only one fatal error; qed");
                    if e.is_fatal_error() {
                        return result;
                    }
                }
            }
            Ok(None) => (),
            Err(e) => {
                result
                    .put_error(Timestamp::INHERENT_IDENTIFIER, &e)
                    .expect("There is only one fatal error; qed");
                if e.is_fatal_error() {
                    return result;
                }
            }
        }
        match Aura::is_inherent_required(self) {
            Ok(Some(e)) => {
                let found = block.extrinsics().iter().any(|xt| {
                    if ::frame_support::inherent::Extrinsic::is_signed(xt).unwrap_or(false) {
                        return false;
                    }
                    let call: Option<&<Aura as ProvideInherent>::Call> = xt.function.is_sub_type();
                    call.is_some()
                });
                if !found {
                    result
                        .put_error(Aura::INHERENT_IDENTIFIER, &e)
                        .expect("There is only one fatal error; qed");
                    if e.is_fatal_error() {
                        return result;
                    }
                }
            }
            Ok(None) => (),
            Err(e) => {
                result
                    .put_error(Aura::INHERENT_IDENTIFIER, &e)
                    .expect("There is only one fatal error; qed");
                if e.is_fatal_error() {
                    return result;
                }
            }
        }
        result
    }
}

impl ::frame_support::unsigned::ValidateUnsigned for Runtime {
    type Call = Call;
    fn pre_dispatch(
        call: &Self::Call,
    ) -> Result<(), ::frame_support::unsigned::TransactionValidityError> {
        #[allow(unreachable_patterns)]
        match call {
            _ => Ok(()),
        }
    }
    fn validate_unsigned(
        #[allow(unused_variables)] source: ::frame_support::unsigned::TransactionSource,
        call: &Self::Call,
    ) -> ::frame_support::unsigned::TransactionValidity {
        #[allow(unreachable_patterns)]
        match call {
            _ => ::frame_support::unsigned::UnknownTransaction::NoUnsignedValidator.into(),
        }
    }
}

/// The address format for describing accounts.
pub type Address = AccountId;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Call, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllModules,
>;

#[doc(hidden)]
mod sp_api_hidden_includes_IMPL_RUNTIME_APIS {
    pub extern crate sp_api as sp_api;
}

pub struct RuntimeApi {}

/// Implements all runtime apis for the client side.
#[cfg(any(feature = "std", test))]
pub struct RuntimeApiImpl<
    Block: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT,
    C: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<Block> + 'static,
> where
    C::StateBackend: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<Block>,
    >,
{
    call: &'static C,
    commit_on_success: std::cell::RefCell<bool>,
    initialized_block: std::cell::RefCell<
        Option<self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<Block>>,
    >,
    changes: std::cell::RefCell<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::OverlayedChanges,
    >,
    offchain_changes: std::cell::RefCell<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::OffchainOverlayedChanges,
    >,
    storage_transaction_cache: std::cell::RefCell<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StorageTransactionCache<
            Block,
            C::StateBackend,
        >,
    >,
    recorder: Option<self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ProofRecorder<Block>>,
}

#[cfg(any(feature = "std", test))]
unsafe impl<
    Block: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT,
    C: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<Block>,
> Send for RuntimeApiImpl<Block, C>
    where
        C::StateBackend: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<Block>,
        >,
{}

#[cfg(any(feature = "std", test))]
unsafe impl<
    Block: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT,
    C: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<Block>,
> Sync for RuntimeApiImpl<Block, C>
    where
        C::StateBackend: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<Block>,
        >,
{}

#[cfg(any(feature = "std", test))]
impl<
    Block: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT,
    C: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<Block>,
> self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiErrorExt
for RuntimeApiImpl<Block, C>
    where
        C::StateBackend: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<Block>,
        >,
{
    type Error = C::Error;
}

#[cfg(any(feature = "std", test))]
impl<
    Block: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT,
    C: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<Block>,
> self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiExt<Block>
for RuntimeApiImpl<Block, C>
    where
        C::StateBackend: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<Block>,
        >,
{
    type StateBackend = C::StateBackend;
    fn execute_in_transaction<
        F: FnOnce(
            &Self,
        )
            -> self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::TransactionOutcome<R>,
        R,
    >(
        &self,
        call: F,
    ) -> R
        where
            Self: Sized,
    {
        self.changes.borrow_mut().start_transaction();
        *self.commit_on_success.borrow_mut() = false;
        let res = call(self);
        *self.commit_on_success.borrow_mut() = true;
        self.commit_or_rollback(match res {
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::TransactionOutcome::Commit(
                _,
            ) => true,
            _ => false,
        });
        res.into_inner()
    }
    fn has_api<A: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::RuntimeApiInfo + ?Sized>(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<Block>,
    ) -> std::result::Result<bool, C::Error>
        where
            Self: Sized,
    {
        self.call
            .runtime_version_at(at)
            .map(|v| v.has_api_with(&A::ID, |v| v == A::VERSION))
    }
    fn has_api_with<
        A: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::RuntimeApiInfo + ?Sized,
        P: Fn(u32) -> bool,
    >(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<Block>,
        pred: P,
    ) -> std::result::Result<bool, C::Error>
        where
            Self: Sized,
    {
        self.call
            .runtime_version_at(at)
            .map(|v| v.has_api_with(&A::ID, pred))
    }
    fn record_proof(&mut self) {
        self.recorder = Some(Default::default());
    }
    fn extract_proof(
        &mut self,
    ) -> Option<self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StorageProof> {
        self.recorder.take().map(|recorder| {
            let trie_nodes = recorder
                .read()
                .iter()
                .filter_map(|(_k, v)| v.as_ref().map(|v| v.to_vec()))
                .collect();
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StorageProof::new(trie_nodes)
        })
    }
    fn into_storage_changes(
        &self,
        backend: &Self::StateBackend,
        changes_trie_state: Option<
            &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ChangesTrieState<
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<Block>,
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NumberFor<Block>,
            >,
        >,
        parent_hash: Block::Hash,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StorageChanges<
            Self::StateBackend,
            Block,
        >,
        String,
    >
        where
            Self: Sized,
    {
        self.initialized_block.borrow_mut().take();
        self.changes
            .replace(Default::default())
            .into_storage_changes(
                backend,
                changes_trie_state,
                parent_hash,
                self.storage_transaction_cache.replace(Default::default()),
            )
    }
}

#[cfg(any(feature = "std", test))]
impl<Block: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT, C>
self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ConstructRuntimeApi<Block, C>
for RuntimeApi
    where
        C: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<Block> + 'static,
        C::StateBackend: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<Block>,
        >,
{
    type RuntimeApi = RuntimeApiImpl<Block, C>;
    fn construct_runtime_api<'a>(
        call: &'a C,
    ) -> self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiRef<'a, Self::RuntimeApi> {
        RuntimeApiImpl {
            call: unsafe { std::mem::transmute(call) },
            commit_on_success: true.into(),
            initialized_block: None.into(),
            changes: Default::default(),
            offchain_changes: Default::default(),
            recorder: Default::default(),
            storage_transaction_cache: Default::default(),
        }
            .into()
    }
}

#[cfg(any(feature = "std", test))]
impl<
    Block: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT,
    C: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<Block>,
> RuntimeApiImpl<Block, C>
    where
        C::StateBackend: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<Block>,
        >,
{
    fn call_api_at<
        R: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode
        + self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Decode
        + PartialEq,
        F: FnOnce(
            &C,
            &Self,
            &std::cell::RefCell<
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::OverlayedChanges,
            >,
            &std::cell::RefCell<
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::OffchainOverlayedChanges,
            >,
            &std::cell::RefCell<
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StorageTransactionCache<
                    Block,
                    C::StateBackend,
                >,
            >,
            &std::cell::RefCell<
                Option<self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<Block>>,
            >,
            &Option<self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ProofRecorder<Block>>,
        ) -> std::result::Result<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<R>,
            E,
        >,
        E,
    >(
        &self,
        call_api_at: F,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<R>,
        E,
    > {
        if *self.commit_on_success.borrow() {
            self.changes.borrow_mut().start_transaction();
        }
        let res = call_api_at(
            &self.call,
            self,
            &self.changes,
            &self.offchain_changes,
            &self.storage_transaction_cache,
            &self.initialized_block,
            &self.recorder,
        );
        self.commit_or_rollback(res.is_ok());
        res
    }
    fn commit_or_rollback(&self, commit: bool) {
        let proof = "\
					We only close a transaction when we opened one ourself.
					Other parts of the runtime that make use of transactions (state-machine)
					also balance their transactions. The runtime cannot close client initiated
					transactions. qed";
        if *self.commit_on_success.borrow() {
            if commit {
                self.changes.borrow_mut().commit_transaction().expect(proof);
            } else {
                self.changes
                    .borrow_mut()
                    .rollback_transaction()
                    .expect(proof);
            }
        }
    }
}

impl sp_api::runtime_decl_for_Core::Core<Block> for Runtime {
    fn version() -> RuntimeVersion {
        VERSION
    }
    fn execute_block(block: Block) {
        Executive::execute_block(block)
    }
    fn initialize_block(header: &<Block as BlockT>::Header) {
        Executive::initialize_block(header)
    }
}

impl sp_api::runtime_decl_for_Metadata::Metadata<Block> for Runtime {
    fn metadata() -> OpaqueMetadata {
        Runtime::metadata().into()
    }
}

impl sp_block_builder::runtime_decl_for_BlockBuilder::BlockBuilder<Block> for Runtime {
    fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
        Executive::apply_extrinsic(extrinsic)
    }
    fn finalize_block() -> <Block as BlockT>::Header {
        Executive::finalize_block()
    }
    fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
        data.create_extrinsics()
    }
    fn check_inherents(
        block: Block,
        data: sp_inherents::InherentData,
    ) -> sp_inherents::CheckInherentsResult {
        data.check_extrinsics(&block)
    }
    fn random_seed() -> <Block as BlockT>::Hash {
        RandomnessCollectiveFlip::random_seed()
    }
}

impl sp_transaction_pool::runtime_api::runtime_decl_for_TaggedTransactionQueue::TaggedTransactionQueue<Block> for Runtime { fn validate_transaction(source: TransactionSource, tx: <Block as BlockT>::Extrinsic) -> TransactionValidity { Executive::validate_transaction(source, tx) } }

impl sp_offchain::runtime_decl_for_OffchainWorkerApi::OffchainWorkerApi<Block> for Runtime {
    fn offchain_worker(header: &<Block as BlockT>::Header) {
        Executive::offchain_worker(header)
    }
}

impl sp_consensus_aura::runtime_decl_for_AuraApi::AuraApi<Block, AuraId> for Runtime {
    fn slot_duration() -> u64 {
        Aura::slot_duration()
    }
    fn authorities() -> Vec<AuraId> {
        Aura::authorities()
    }
}

impl sp_session::runtime_decl_for_SessionKeys::SessionKeys<Block> for Runtime {
    fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
        opaque::SessionKeys::generate(seed)
    }
    fn decode_session_keys(encoded: Vec<u8>) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
        opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
    }
}

impl fg_primitives::runtime_decl_for_GrandpaApi::GrandpaApi<Block> for Runtime {
    fn grandpa_authorities() -> GrandpaAuthorityList {
        Grandpa::grandpa_authorities()
    }
    fn submit_report_equivocation_unsigned_extrinsic(
        _equivocation_proof: fg_primitives::EquivocationProof<
            <Block as BlockT>::Hash,
            NumberFor<Block>,
        >,
        _key_owner_proof: fg_primitives::OpaqueKeyOwnershipProof,
    ) -> Option<()> {
        None
    }
    fn generate_key_ownership_proof(
        _set_id: fg_primitives::SetId,
        _authority_id: GrandpaId,
    ) -> Option<fg_primitives::OpaqueKeyOwnershipProof> {
        None
    }
}

impl
frame_system_rpc_runtime_api::runtime_decl_for_AccountNonceApi::AccountNonceApi<
    Block,
    AccountId,
    Index,
> for Runtime
{
    fn account_nonce(account: AccountId) -> Index {
        System::account_nonce(account)
    }
}

impl pallet_transaction_payment_rpc_runtime_api::runtime_decl_for_TransactionPaymentApi::TransactionPaymentApi<Block, Balance> for Runtime { fn query_info(uxt: <Block as BlockT>::Extrinsic, len: u32) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> { TransactionPayment::query_info(uxt, len) } }

#[cfg(any(feature = "std", test))]
impl<
    __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
    + std::panic::UnwindSafe
    + std::panic::RefUnwindSafe,
    RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
    + 'static,
> sp_api::Core<__SR_API_BLOCK__> for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
    where
        RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
        RuntimeVersion: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        __SR_API_BLOCK__: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        <__SR_API_BLOCK__ as BlockT>::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn Core_version_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<()>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<RuntimeVersion>,
        RuntimeApiImplCall::Error,
    > {
        self.call_api_at(
            |call_runtime_at,
             core_api,
             changes,
             offchain_changes,
             storage_transaction_cache,
             initialized_block,
             recorder| {
                sp_api::runtime_decl_for_Core::version_call_api_at(
                    call_runtime_at,
                    core_api,
                    at,
                    params_encoded,
                    changes,
                    offchain_changes,
                    storage_transaction_cache,
                    initialized_block,
                    params.map(|p| {
                        sp_api::runtime_decl_for_Core::version_native_call_generator::<
                            Runtime,
                            __SR_API_BLOCK__,
                            Block,
                        >()
                    }),
                    context,
                    recorder,
                )
            },
        )
    }
    fn Core_execute_block_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(__SR_API_BLOCK__)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<()>,
        RuntimeApiImplCall::Error,
    > {
        self.call_api_at(
            |call_runtime_at,
             core_api,
             changes,
             offchain_changes,
             storage_transaction_cache,
             initialized_block,
             recorder| {
                sp_api::runtime_decl_for_Core::execute_block_call_api_at(
                    call_runtime_at,
                    core_api,
                    at,
                    params_encoded,
                    changes,
                    offchain_changes,
                    storage_transaction_cache,
                    initialized_block,
                    params.map(|p| {
                        sp_api::runtime_decl_for_Core::execute_block_native_call_generator::<
                            Runtime,
                            __SR_API_BLOCK__,
                            Block,
                        >(p)
                    }),
                    context,
                    recorder,
                )
            },
        )
    }
    fn Core_initialize_block_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(&<__SR_API_BLOCK__ as BlockT>::Header)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<()>,
        RuntimeApiImplCall::Error,
    > {
        self.call_api_at(
            |call_runtime_at,
             core_api,
             changes,
             offchain_changes,
             storage_transaction_cache,
             initialized_block,
             recorder| {
                sp_api::runtime_decl_for_Core::initialize_block_call_api_at(
                    call_runtime_at,
                    core_api,
                    at,
                    params_encoded,
                    changes,
                    offchain_changes,
                    storage_transaction_cache,
                    initialized_block,
                    params.map(|p| {
                        sp_api::runtime_decl_for_Core::initialize_block_native_call_generator::<
                            Runtime,
                            __SR_API_BLOCK__,
                            Block,
                        >(p)
                    }),
                    context,
                    recorder,
                )
            },
        )
    }
}

#[cfg(any(feature = "std", test))]
impl<
    __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
    + std::panic::UnwindSafe
    + std::panic::RefUnwindSafe,
    RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
    + 'static,
> sp_api::Metadata<__SR_API_BLOCK__> for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
    where
        RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
        OpaqueMetadata: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn Metadata_metadata_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<()>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<OpaqueMetadata>,
        RuntimeApiImplCall::Error,
    > {
        self.call_api_at(
            |call_runtime_at,
             core_api,
             changes,
             offchain_changes,
             storage_transaction_cache,
             initialized_block,
             recorder| {
                sp_api::runtime_decl_for_Metadata::metadata_call_api_at(
                    call_runtime_at,
                    core_api,
                    at,
                    params_encoded,
                    changes,
                    offchain_changes,
                    storage_transaction_cache,
                    initialized_block,
                    params.map(|p| {
                        sp_api::runtime_decl_for_Metadata::metadata_native_call_generator::<
                            Runtime,
                            __SR_API_BLOCK__,
                            Block,
                        >()
                    }),
                    context,
                    recorder,
                )
            },
        )
    }
}

#[cfg(any(feature = "std", test))]
impl<
    __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
    + std::panic::UnwindSafe
    + std::panic::RefUnwindSafe,
    RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
    + 'static,
> sp_block_builder::BlockBuilder<__SR_API_BLOCK__>
for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
    where
        RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
        <__SR_API_BLOCK__ as BlockT>::Extrinsic: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        ApplyExtrinsicResult: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        <__SR_API_BLOCK__ as BlockT>::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        sp_inherents::InherentData: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        Vec<<__SR_API_BLOCK__ as BlockT>::Extrinsic>:
        std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        __SR_API_BLOCK__: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        sp_inherents::InherentData: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        sp_inherents::CheckInherentsResult: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        <__SR_API_BLOCK__ as BlockT>::Hash: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn BlockBuilder_apply_extrinsic_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(<__SR_API_BLOCK__ as BlockT>::Extrinsic)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            ApplyExtrinsicResult,
        >,
        RuntimeApiImplCall::Error,
    > {
        self.call_api_at(|call_runtime_at, core_api, changes, offchain_changes, storage_transaction_cache, initialized_block, recorder| { sp_block_builder::runtime_decl_for_BlockBuilder::apply_extrinsic_call_api_at(call_runtime_at, core_api, at, params_encoded, changes, offchain_changes, storage_transaction_cache, initialized_block, params.map(|p| { sp_block_builder::runtime_decl_for_BlockBuilder::apply_extrinsic_native_call_generator::<Runtime, __SR_API_BLOCK__, Block>(p) }), context, recorder) })
    }
    fn BlockBuilder_finalize_block_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<()>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            <__SR_API_BLOCK__ as BlockT>::Header,
        >,
        RuntimeApiImplCall::Error,
    > {
        self.call_api_at(|call_runtime_at, core_api, changes, offchain_changes, storage_transaction_cache, initialized_block, recorder| { sp_block_builder::runtime_decl_for_BlockBuilder::finalize_block_call_api_at(call_runtime_at, core_api, at, params_encoded, changes, offchain_changes, storage_transaction_cache, initialized_block, params.map(|p| { sp_block_builder::runtime_decl_for_BlockBuilder::finalize_block_native_call_generator::<Runtime, __SR_API_BLOCK__, Block>() }), context, recorder) })
    }
    fn BlockBuilder_inherent_extrinsics_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(sp_inherents::InherentData)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            Vec<<__SR_API_BLOCK__ as BlockT>::Extrinsic>,
        >,
        RuntimeApiImplCall::Error,
    > {
        self.call_api_at(|call_runtime_at, core_api, changes, offchain_changes, storage_transaction_cache, initialized_block, recorder| { sp_block_builder::runtime_decl_for_BlockBuilder::inherent_extrinsics_call_api_at(call_runtime_at, core_api, at, params_encoded, changes, offchain_changes, storage_transaction_cache, initialized_block, params.map(|p| { sp_block_builder::runtime_decl_for_BlockBuilder::inherent_extrinsics_native_call_generator::<Runtime, __SR_API_BLOCK__, Block>(p) }), context, recorder) })
    }
    fn BlockBuilder_check_inherents_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(__SR_API_BLOCK__, sp_inherents::InherentData)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            sp_inherents::CheckInherentsResult,
        >,
        RuntimeApiImplCall::Error,
    > {
        self.call_api_at(|call_runtime_at, core_api, changes, offchain_changes, storage_transaction_cache, initialized_block, recorder| { sp_block_builder::runtime_decl_for_BlockBuilder::check_inherents_call_api_at(call_runtime_at, core_api, at, params_encoded, changes, offchain_changes, storage_transaction_cache, initialized_block, params.map(|p| { sp_block_builder::runtime_decl_for_BlockBuilder::check_inherents_native_call_generator::<Runtime, __SR_API_BLOCK__, Block>(p.0, p.1) }), context, recorder) })
    }
    fn BlockBuilder_random_seed_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<()>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            <__SR_API_BLOCK__ as BlockT>::Hash,
        >,
        RuntimeApiImplCall::Error,
    > {
        self.call_api_at(|call_runtime_at, core_api, changes, offchain_changes, storage_transaction_cache, initialized_block, recorder| { sp_block_builder::runtime_decl_for_BlockBuilder::random_seed_call_api_at(call_runtime_at, core_api, at, params_encoded, changes, offchain_changes, storage_transaction_cache, initialized_block, params.map(|p| { sp_block_builder::runtime_decl_for_BlockBuilder::random_seed_native_call_generator::<Runtime, __SR_API_BLOCK__, Block>() }), context, recorder) })
    }
}

#[cfg(any(feature = "std", test))]
impl<
    __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
    + std::panic::UnwindSafe
    + std::panic::RefUnwindSafe,
    RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
    + 'static,
> sp_transaction_pool::runtime_api::TaggedTransactionQueue<__SR_API_BLOCK__>
for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
    where
        RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
        TransactionSource: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        <__SR_API_BLOCK__ as BlockT>::Extrinsic: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        TransactionValidity: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn TaggedTransactionQueue_validate_transaction_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(TransactionSource, <__SR_API_BLOCK__ as BlockT>::Extrinsic)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            TransactionValidity,
        >,
        RuntimeApiImplCall::Error,
    > {
        self.call_api_at(|call_runtime_at, core_api, changes, offchain_changes, storage_transaction_cache, initialized_block, recorder| { sp_transaction_pool::runtime_api::runtime_decl_for_TaggedTransactionQueue::validate_transaction_call_api_at(call_runtime_at, core_api, at, params_encoded, changes, offchain_changes, storage_transaction_cache, initialized_block, params.map(|p| { sp_transaction_pool::runtime_api::runtime_decl_for_TaggedTransactionQueue::validate_transaction_native_call_generator::<Runtime, __SR_API_BLOCK__, Block>(p.0, p.1) }), context, recorder) })
    }
}

#[cfg(any(feature = "std", test))]
impl<
    __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
    + std::panic::UnwindSafe
    + std::panic::RefUnwindSafe,
    RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
    + 'static,
> sp_offchain::OffchainWorkerApi<__SR_API_BLOCK__>
for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
    where
        RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
        <__SR_API_BLOCK__ as BlockT>::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn OffchainWorkerApi_offchain_worker_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(&<__SR_API_BLOCK__ as BlockT>::Header)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<()>,
        RuntimeApiImplCall::Error,
    > {
        self.call_api_at(|call_runtime_at, core_api, changes, offchain_changes, storage_transaction_cache, initialized_block, recorder| { sp_offchain::runtime_decl_for_OffchainWorkerApi::offchain_worker_call_api_at(call_runtime_at, core_api, at, params_encoded, changes, offchain_changes, storage_transaction_cache, initialized_block, params.map(|p| { sp_offchain::runtime_decl_for_OffchainWorkerApi::offchain_worker_native_call_generator::<Runtime, __SR_API_BLOCK__, Block>(p) }), context, recorder) })
    }
}

#[cfg(any(feature = "std", test))]
impl<
    __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
    + std::panic::UnwindSafe
    + std::panic::RefUnwindSafe,
    RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
    + 'static,
> sp_consensus_aura::AuraApi<__SR_API_BLOCK__, AuraId>
for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
    where
        RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
        u64: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        Vec<AuraId>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn AuraApi_slot_duration_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<()>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<u64>,
        RuntimeApiImplCall::Error,
    > {
        self.call_api_at(|call_runtime_at, core_api, changes, offchain_changes, storage_transaction_cache, initialized_block, recorder| { sp_consensus_aura::runtime_decl_for_AuraApi::slot_duration_call_api_at(call_runtime_at, core_api, at, params_encoded, changes, offchain_changes, storage_transaction_cache, initialized_block, params.map(|p| { sp_consensus_aura::runtime_decl_for_AuraApi::slot_duration_native_call_generator::<Runtime, __SR_API_BLOCK__, Block, AuraId>() }), context, recorder) })
    }
    fn AuraApi_authorities_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<()>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<Vec<AuraId>>,
        RuntimeApiImplCall::Error,
    > {
        self.call_api_at(|call_runtime_at, core_api, changes, offchain_changes, storage_transaction_cache, initialized_block, recorder| { sp_consensus_aura::runtime_decl_for_AuraApi::authorities_call_api_at(call_runtime_at, core_api, at, params_encoded, changes, offchain_changes, storage_transaction_cache, initialized_block, params.map(|p| { sp_consensus_aura::runtime_decl_for_AuraApi::authorities_native_call_generator::<Runtime, __SR_API_BLOCK__, Block, AuraId>() }), context, recorder) })
    }
}

#[cfg(any(feature = "std", test))]
impl<
    __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
    + std::panic::UnwindSafe
    + std::panic::RefUnwindSafe,
    RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
    + 'static,
> sp_session::SessionKeys<__SR_API_BLOCK__>
for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
    where
        RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
        Option<Vec<u8>>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        Vec<u8>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        Vec<u8>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        Option<Vec<(Vec<u8>, KeyTypeId)>>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn SessionKeys_generate_session_keys_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(Option<Vec<u8>>)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<Vec<u8>>,
        RuntimeApiImplCall::Error,
    > {
        self.call_api_at(|call_runtime_at, core_api, changes, offchain_changes, storage_transaction_cache, initialized_block, recorder| { sp_session::runtime_decl_for_SessionKeys::generate_session_keys_call_api_at(call_runtime_at, core_api, at, params_encoded, changes, offchain_changes, storage_transaction_cache, initialized_block, params.map(|p| { sp_session::runtime_decl_for_SessionKeys::generate_session_keys_native_call_generator::<Runtime, __SR_API_BLOCK__, Block>(p) }), context, recorder) })
    }
    fn SessionKeys_decode_session_keys_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(Vec<u8>)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            Option<Vec<(Vec<u8>, KeyTypeId)>>,
        >,
        RuntimeApiImplCall::Error,
    > {
        self.call_api_at(|call_runtime_at, core_api, changes, offchain_changes, storage_transaction_cache, initialized_block, recorder| { sp_session::runtime_decl_for_SessionKeys::decode_session_keys_call_api_at(call_runtime_at, core_api, at, params_encoded, changes, offchain_changes, storage_transaction_cache, initialized_block, params.map(|p| { sp_session::runtime_decl_for_SessionKeys::decode_session_keys_native_call_generator::<Runtime, __SR_API_BLOCK__, Block>(p) }), context, recorder) })
    }
}

#[cfg(any(feature = "std", test))]
impl<
    __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
    + std::panic::UnwindSafe
    + std::panic::RefUnwindSafe,
    RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
    + 'static,
> fg_primitives::GrandpaApi<__SR_API_BLOCK__>
for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
    where
        RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
        GrandpaAuthorityList: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        fg_primitives::EquivocationProof<
            <__SR_API_BLOCK__ as BlockT>::Hash,
            NumberFor<__SR_API_BLOCK__>,
        >: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        fg_primitives::OpaqueKeyOwnershipProof: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        Option<()>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        fg_primitives::SetId: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        GrandpaId: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        Option<fg_primitives::OpaqueKeyOwnershipProof>:
        std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn GrandpaApi_grandpa_authorities_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<()>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            GrandpaAuthorityList,
        >,
        RuntimeApiImplCall::Error,
    > {
        self.call_api_at(|call_runtime_at, core_api, changes, offchain_changes, storage_transaction_cache, initialized_block, recorder| { fg_primitives::runtime_decl_for_GrandpaApi::grandpa_authorities_call_api_at(call_runtime_at, core_api, at, params_encoded, changes, offchain_changes, storage_transaction_cache, initialized_block, params.map(|p| { fg_primitives::runtime_decl_for_GrandpaApi::grandpa_authorities_native_call_generator::<Runtime, __SR_API_BLOCK__, Block>() }), context, recorder) })
    }
    fn GrandpaApi_submit_report_equivocation_unsigned_extrinsic_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(
            fg_primitives::EquivocationProof<
                <__SR_API_BLOCK__ as BlockT>::Hash,
                NumberFor<__SR_API_BLOCK__>,
            >,
            fg_primitives::OpaqueKeyOwnershipProof,
        )>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<Option<()>>,
        RuntimeApiImplCall::Error,
    > {
        self.call_api_at(|call_runtime_at, core_api, changes, offchain_changes, storage_transaction_cache, initialized_block, recorder| { fg_primitives::runtime_decl_for_GrandpaApi::submit_report_equivocation_unsigned_extrinsic_call_api_at(call_runtime_at, core_api, at, params_encoded, changes, offchain_changes, storage_transaction_cache, initialized_block, params.map(|p| { fg_primitives::runtime_decl_for_GrandpaApi::submit_report_equivocation_unsigned_extrinsic_native_call_generator::<Runtime, __SR_API_BLOCK__, Block>(p.0, p.1) }), context, recorder) })
    }
    fn GrandpaApi_generate_key_ownership_proof_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(fg_primitives::SetId, GrandpaId)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            Option<fg_primitives::OpaqueKeyOwnershipProof>,
        >,
        RuntimeApiImplCall::Error,
    > {
        self.call_api_at(|call_runtime_at, core_api, changes, offchain_changes, storage_transaction_cache, initialized_block, recorder| { fg_primitives::runtime_decl_for_GrandpaApi::generate_key_ownership_proof_call_api_at(call_runtime_at, core_api, at, params_encoded, changes, offchain_changes, storage_transaction_cache, initialized_block, params.map(|p| { fg_primitives::runtime_decl_for_GrandpaApi::generate_key_ownership_proof_native_call_generator::<Runtime, __SR_API_BLOCK__, Block>(p.0, p.1) }), context, recorder) })
    }
}

#[cfg(any(feature = "std", test))]
impl<
    __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
    + std::panic::UnwindSafe
    + std::panic::RefUnwindSafe,
    RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
    + 'static,
> frame_system_rpc_runtime_api::AccountNonceApi<__SR_API_BLOCK__, AccountId, Index>
for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
    where
        RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
        AccountId: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        Index: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn AccountNonceApi_account_nonce_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(AccountId)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<Index>,
        RuntimeApiImplCall::Error,
    > {
        self.call_api_at(|call_runtime_at, core_api, changes, offchain_changes, storage_transaction_cache, initialized_block, recorder| { frame_system_rpc_runtime_api::runtime_decl_for_AccountNonceApi::account_nonce_call_api_at(call_runtime_at, core_api, at, params_encoded, changes, offchain_changes, storage_transaction_cache, initialized_block, params.map(|p| { frame_system_rpc_runtime_api::runtime_decl_for_AccountNonceApi::account_nonce_native_call_generator::<Runtime, __SR_API_BLOCK__, Block, AccountId, Index>(p) }), context, recorder) })
    }
}

#[cfg(any(feature = "std", test))]
impl<
    __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
    + std::panic::UnwindSafe
    + std::panic::RefUnwindSafe,
    RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
    + 'static,
> pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<__SR_API_BLOCK__, Balance>
for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
    where
        RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
        <__SR_API_BLOCK__ as BlockT>::Extrinsic: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        u32: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance>:
        std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn TransactionPaymentApi_query_info_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(<__SR_API_BLOCK__ as BlockT>::Extrinsic, u32)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance>,
        >,
        RuntimeApiImplCall::Error,
    > {
        self.call_api_at(|call_runtime_at, core_api, changes, offchain_changes, storage_transaction_cache, initialized_block, recorder| { pallet_transaction_payment_rpc_runtime_api::runtime_decl_for_TransactionPaymentApi::query_info_call_api_at(call_runtime_at, core_api, at, params_encoded, changes, offchain_changes, storage_transaction_cache, initialized_block, params.map(|p| { pallet_transaction_payment_rpc_runtime_api::runtime_decl_for_TransactionPaymentApi::query_info_native_call_generator::<Runtime, __SR_API_BLOCK__, Block, Balance>(p.0, p.1) }), context, recorder) })
    }
}

const RUNTIME_API_VERSIONS: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApisVec = ::sp_version::sp_std::borrow::Cow::Borrowed(&[(sp_api::runtime_decl_for_Core::ID, sp_api::runtime_decl_for_Core::VERSION), (sp_api::runtime_decl_for_Metadata::ID, sp_api::runtime_decl_for_Metadata::VERSION), (sp_block_builder::runtime_decl_for_BlockBuilder::ID, sp_block_builder::runtime_decl_for_BlockBuilder::VERSION), (sp_transaction_pool::runtime_api::runtime_decl_for_TaggedTransactionQueue::ID, sp_transaction_pool::runtime_api::runtime_decl_for_TaggedTransactionQueue::VERSION), (sp_offchain::runtime_decl_for_OffchainWorkerApi::ID, sp_offchain::runtime_decl_for_OffchainWorkerApi::VERSION), (sp_consensus_aura::runtime_decl_for_AuraApi::ID, sp_consensus_aura::runtime_decl_for_AuraApi::VERSION), (sp_session::runtime_decl_for_SessionKeys::ID, sp_session::runtime_decl_for_SessionKeys::VERSION), (fg_primitives::runtime_decl_for_GrandpaApi::ID, fg_primitives::runtime_decl_for_GrandpaApi::VERSION), (frame_system_rpc_runtime_api::runtime_decl_for_AccountNonceApi::ID, frame_system_rpc_runtime_api::runtime_decl_for_AccountNonceApi::VERSION), (pallet_transaction_payment_rpc_runtime_api::runtime_decl_for_TransactionPaymentApi::ID, pallet_transaction_payment_rpc_runtime_api::runtime_decl_for_TransactionPaymentApi::VERSION)]);

pub mod api {
    use super::*;

    #[cfg(feature = "std")]
    pub fn dispatch(method: &str, mut __sp_api__input_data: &[u8]) -> Option<Vec<u8>> {
        match method {
            "Core_version" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (): () = match self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::DecodeLimit::decode_all_with_depth_limit(self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::MAX_EXTRINSIC_DEPTH, &__sp_api__input_data) {
                        Ok(res) => res,
                        Err(e) => { ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(&["Bad input data provided to ", ": "], &match (&"version", &e.what()) { (arg0, arg1) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt), ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt)], })) }
                    };
                    #[allow(deprecated)]
                        <Runtime as sp_api::runtime_decl_for_Core::Core<Block>>::version()
                }),
            ),
            "Core_execute_block" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (block): ( Block ) = match self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::DecodeLimit::decode_all_with_depth_limit(self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::MAX_EXTRINSIC_DEPTH, &__sp_api__input_data) {
                        Ok(res) => res,
                        Err(e) => { ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(&["Bad input data provided to ", ": "], &match (&"execute_block", &e.what()) { (arg0, arg1) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt), ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt)], })) }
                    };
                    #[allow(deprecated)]
                        <Runtime as sp_api::runtime_decl_for_Core::Core<Block>>::execute_block(block)
                }),
            ),
            "Core_initialize_block" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (header): ( <Block as BlockT>::Header ) = match self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::DecodeLimit::decode_all_with_depth_limit(self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::MAX_EXTRINSIC_DEPTH, &__sp_api__input_data) {
                        Ok(res) => res,
                        Err(e) => { ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(&["Bad input data provided to ", ": "], &match (&"initialize_block", &e.what()) { (arg0, arg1) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt), ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt)], })) }
                    };
                    #[allow(deprecated)]
                        <Runtime as sp_api::runtime_decl_for_Core::Core<Block>>::initialize_block(
                        &header,
                    )
                }),
            ),
            "Metadata_metadata" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (): () = match self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::DecodeLimit::decode_all_with_depth_limit(self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::MAX_EXTRINSIC_DEPTH, &__sp_api__input_data) {
                        Ok(res) => res,
                        Err(e) => { ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(&["Bad input data provided to ", ": "], &match (&"metadata", &e.what()) { (arg0, arg1) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt), ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt)], })) }
                    };
                    #[allow(deprecated)]
                        <Runtime as sp_api::runtime_decl_for_Metadata::Metadata<Block>>::metadata()
                }),
            ),
            "BlockBuilder_apply_extrinsic" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (extrinsic): ( <Block as BlockT>::Extrinsic ) = match self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::DecodeLimit::decode_all_with_depth_limit(self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::MAX_EXTRINSIC_DEPTH, &__sp_api__input_data) {
                        Ok(res) => res,
                        Err(e) => { ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(&["Bad input data provided to ", ": "], &match (&"apply_extrinsic", &e.what()) { (arg0, arg1) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt), ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt)], })) }
                    };
                    #[allow(deprecated)]
                        <Runtime as sp_block_builder::runtime_decl_for_BlockBuilder::BlockBuilder<
                            Block,
                        >>::apply_extrinsic(extrinsic)
                }),
            ),
            "BlockBuilder_finalize_block" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (): () = match self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::DecodeLimit::decode_all_with_depth_limit(self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::MAX_EXTRINSIC_DEPTH, &__sp_api__input_data) {
                        Ok(res) => res,
                        Err(e) => { ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(&["Bad input data provided to ", ": "], &match (&"finalize_block", &e.what()) { (arg0, arg1) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt), ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt)], })) }
                    };
                    #[allow(deprecated)]
                        <Runtime as sp_block_builder::runtime_decl_for_BlockBuilder::BlockBuilder<
                            Block,
                        >>::finalize_block()
                }),
            ),
            "BlockBuilder_inherent_extrinsics" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (data): ( sp_inherents::InherentData ) = match self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::DecodeLimit::decode_all_with_depth_limit(self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::MAX_EXTRINSIC_DEPTH, &__sp_api__input_data) {
                        Ok(res) => res,
                        Err(e) => { ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(&["Bad input data provided to ", ": "], &match (&"inherent_extrinsics", &e.what()) { (arg0, arg1) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt), ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt)], })) }
                    };
                    #[allow(deprecated)]
                        <Runtime as sp_block_builder::runtime_decl_for_BlockBuilder::BlockBuilder<
                            Block,
                        >>::inherent_extrinsics(data)
                }),
            ),
            "BlockBuilder_check_inherents" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (block, data): (Block, sp_inherents::InherentData) = match self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::DecodeLimit::decode_all_with_depth_limit(self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::MAX_EXTRINSIC_DEPTH, &__sp_api__input_data) {
                        Ok(res) => res,
                        Err(e) => { ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(&["Bad input data provided to ", ": "], &match (&"check_inherents", &e.what()) { (arg0, arg1) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt), ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt)], })) }
                    };
                    #[allow(deprecated)]
                        <Runtime as sp_block_builder::runtime_decl_for_BlockBuilder::BlockBuilder<
                            Block,
                        >>::check_inherents(block, data)
                }),
            ),
            "BlockBuilder_random_seed" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (): () = match self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::DecodeLimit::decode_all_with_depth_limit(self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::MAX_EXTRINSIC_DEPTH, &__sp_api__input_data) {
                        Ok(res) => res,
                        Err(e) => { ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(&["Bad input data provided to ", ": "], &match (&"random_seed", &e.what()) { (arg0, arg1) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt), ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt)], })) }
                    };
                    #[allow(deprecated)]
                        <Runtime as sp_block_builder::runtime_decl_for_BlockBuilder::BlockBuilder<
                            Block,
                        >>::random_seed()
                }),
            ),
            "TaggedTransactionQueue_validate_transaction" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (source, tx): (TransactionSource, <Block as BlockT>::Extrinsic) = match self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::DecodeLimit::decode_all_with_depth_limit(self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::MAX_EXTRINSIC_DEPTH, &__sp_api__input_data) {
                        Ok(res) => res,
                        Err(e) => { ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(&["Bad input data provided to ", ": "], &match (&"validate_transaction", &e.what()) { (arg0, arg1) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt), ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt)], })) }
                    };
                    #[allow(deprecated)] <Runtime as sp_transaction_pool::runtime_api::runtime_decl_for_TaggedTransactionQueue::TaggedTransactionQueue<Block>>::validate_transaction(source, tx)
                }),
            ),
            "OffchainWorkerApi_offchain_worker" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (header): ( <Block as BlockT>::Header ) = match self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::DecodeLimit::decode_all_with_depth_limit(self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::MAX_EXTRINSIC_DEPTH, &__sp_api__input_data) {
                        Ok(res) => res,
                        Err(e) => { ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(&["Bad input data provided to ", ": "], &match (&"offchain_worker", &e.what()) { (arg0, arg1) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt), ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt)], })) }
                    };
                    #[allow(deprecated)] <Runtime as sp_offchain::runtime_decl_for_OffchainWorkerApi::OffchainWorkerApi<Block>>::offchain_worker(&header)
                }),
            ),
            "AuraApi_slot_duration" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (): () = match self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::DecodeLimit::decode_all_with_depth_limit(self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::MAX_EXTRINSIC_DEPTH, &__sp_api__input_data) {
                        Ok(res) => res,
                        Err(e) => { ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(&["Bad input data provided to ", ": "], &match (&"slot_duration", &e.what()) { (arg0, arg1) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt), ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt)], })) }
                    };
                    #[allow(deprecated)]
                        <Runtime as sp_consensus_aura::runtime_decl_for_AuraApi::AuraApi<
                            Block,
                            AuraId,
                        >>::slot_duration()
                }),
            ),
            "AuraApi_authorities" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (): () = match self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::DecodeLimit::decode_all_with_depth_limit(self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::MAX_EXTRINSIC_DEPTH, &__sp_api__input_data) {
                        Ok(res) => res,
                        Err(e) => { ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(&["Bad input data provided to ", ": "], &match (&"authorities", &e.what()) { (arg0, arg1) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt), ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt)], })) }
                    };
                    #[allow(deprecated)]
                        <Runtime as sp_consensus_aura::runtime_decl_for_AuraApi::AuraApi<
                            Block,
                            AuraId,
                        >>::authorities()
                }),
            ),
            "SessionKeys_generate_session_keys" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (seed): ( Option<Vec<u8>> ) = match self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::DecodeLimit::decode_all_with_depth_limit(self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::MAX_EXTRINSIC_DEPTH, &__sp_api__input_data) {
                        Ok(res) => res,
                        Err(e) => { ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(&["Bad input data provided to ", ": "], &match (&"generate_session_keys", &e.what()) { (arg0, arg1) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt), ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt)], })) }
                    };
                    #[allow(deprecated)] <Runtime as sp_session::runtime_decl_for_SessionKeys::SessionKeys<Block>>::generate_session_keys(seed)
                }),
            ),
            "SessionKeys_decode_session_keys" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (encoded): ( Vec<u8> ) = match self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::DecodeLimit::decode_all_with_depth_limit(self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::MAX_EXTRINSIC_DEPTH, &__sp_api__input_data) {
                        Ok(res) => res,
                        Err(e) => { ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(&["Bad input data provided to ", ": "], &match (&"decode_session_keys", &e.what()) { (arg0, arg1) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt), ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt)], })) }
                    };
                    #[allow(deprecated)] <Runtime as sp_session::runtime_decl_for_SessionKeys::SessionKeys<Block>>::decode_session_keys(encoded)
                }),
            ),
            "GrandpaApi_grandpa_authorities" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (): () = match self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::DecodeLimit::decode_all_with_depth_limit(self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::MAX_EXTRINSIC_DEPTH, &__sp_api__input_data) {
                        Ok(res) => res,
                        Err(e) => { ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(&["Bad input data provided to ", ": "], &match (&"grandpa_authorities", &e.what()) { (arg0, arg1) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt), ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt)], })) }
                    };
                    #[allow(deprecated)] <Runtime as fg_primitives::runtime_decl_for_GrandpaApi::GrandpaApi<Block>>::grandpa_authorities()
                }),
            ),
            "GrandpaApi_submit_report_equivocation_unsigned_extrinsic" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (_equivocation_proof, _key_owner_proof): (fg_primitives::EquivocationProof<<Block as BlockT>::Hash, NumberFor<Block>>, fg_primitives::OpaqueKeyOwnershipProof) = match self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::DecodeLimit::decode_all_with_depth_limit(self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::MAX_EXTRINSIC_DEPTH, &__sp_api__input_data) {
                        Ok(res) => res,
                        Err(e) => { ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(&["Bad input data provided to ", ": "], &match (&"submit_report_equivocation_unsigned_extrinsic", &e.what()) { (arg0, arg1) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt), ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt)], })) }
                    };
                    #[allow(deprecated)] <Runtime as fg_primitives::runtime_decl_for_GrandpaApi::GrandpaApi<Block>>::submit_report_equivocation_unsigned_extrinsic(_equivocation_proof, _key_owner_proof)
                }),
            ),
            "GrandpaApi_generate_key_ownership_proof" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (_set_id, _authority_id): (fg_primitives::SetId, GrandpaId) = match self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::DecodeLimit::decode_all_with_depth_limit(self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::MAX_EXTRINSIC_DEPTH, &__sp_api__input_data) {
                        Ok(res) => res,
                        Err(e) => { ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(&["Bad input data provided to ", ": "], &match (&"generate_key_ownership_proof", &e.what()) { (arg0, arg1) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt), ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt)], })) }
                    };
                    #[allow(deprecated)] <Runtime as fg_primitives::runtime_decl_for_GrandpaApi::GrandpaApi<Block>>::generate_key_ownership_proof(_set_id, _authority_id)
                }),
            ),
            "AccountNonceApi_account_nonce" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (account): ( AccountId ) = match self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::DecodeLimit::decode_all_with_depth_limit(self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::MAX_EXTRINSIC_DEPTH, &__sp_api__input_data) {
                        Ok(res) => res,
                        Err(e) => { ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(&["Bad input data provided to ", ": "], &match (&"account_nonce", &e.what()) { (arg0, arg1) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt), ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt)], })) }
                    };
                    #[allow(deprecated)] <Runtime as frame_system_rpc_runtime_api::runtime_decl_for_AccountNonceApi::AccountNonceApi<Block, AccountId, Index>>::account_nonce(account)
                }),
            ),
            "TransactionPaymentApi_query_info" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (uxt, len): (<Block as BlockT>::Extrinsic, u32) = match self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::DecodeLimit::decode_all_with_depth_limit(self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::MAX_EXTRINSIC_DEPTH, &__sp_api__input_data) {
                        Ok(res) => res,
                        Err(e) => { ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(&["Bad input data provided to ", ": "], &match (&"query_info", &e.what()) { (arg0, arg1) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt), ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt)], })) }
                    };
                    #[allow(deprecated)] <Runtime as pallet_transaction_payment_rpc_runtime_api::runtime_decl_for_TransactionPaymentApi::TransactionPaymentApi<Block, Balance>>::query_info(uxt, len)
                }),
            ),
            _ => None,
        }
    }
}
