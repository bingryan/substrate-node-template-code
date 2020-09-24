#![feature(prelude_import)]
//! # Nicks Module
//!
//! - [`nicks::Trait`](./trait.Trait.html)
//! - [`Call`](./enum.Call.html)
//!
//! ## Overview
//!
//! Nicks is an example module for keeping track of account names on-chain. It makes no effort to
//! create a name hierarchy, be a DNS replacement or provide reverse lookups. Furthermore, the
//! weights attached to this module's dispatchable functions are for demonstration purposes only and
//! have not been designed to be economically secure. Do not use this pallet as-is in production.
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! * `set_name` - Set the associated name of an account; a small deposit is reserved if not already
//!   taken.
//! * `clear_name` - Remove an account's associated name; the deposit is returned.
//! * `kill_name` - Forcibly remove the associated name; the deposit is lost.
//!
//! [`Call`]: ./enum.Call.html
//! [`Trait`]: ./trait.Trait.html
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use sp_std::prelude::*;
use sp_runtime::{
    traits::{StaticLookup, Zero},
};
use frame_support::{
    decl_module, decl_event, decl_storage, ensure, decl_error,
    traits::{Currency, EnsureOrigin, ReservableCurrency, OnUnbalanced, Get},
};
use frame_system::ensure_signed;
type BalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;
type NegativeImbalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::NegativeImbalance;
pub trait Trait: frame_system::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    /// The currency trait.
    type Currency: ReservableCurrency<Self::AccountId>;
    /// Reservation fee.
    type ReservationFee: Get<BalanceOf<Self>>;
    /// What to do with slashed funds.
    type Slashed: OnUnbalanced<NegativeImbalanceOf<Self>>;
    /// The origin which may forcibly set or remove a name. Root can always do this.
    type ForceOrigin: EnsureOrigin<Self::Origin>;
    /// The minimum length a name may be.
    type MinLength: Get<usize>;
    /// The maximum length a name may be.
    type MaxLength: Get<usize>;
}

// "impl-only-use · Issue #2166 · rust-lang/rfcs" https://github.com/rust-lang/rfcs/pull/2166
use self::sp_api_hidden_includes_decl_storage::hidden_include::{
    StorageValue as _, StorageMap as _, StorageDoubleMap as _, StoragePrefixedMap as _,
    IterableStorageMap as _, IterableStorageDoubleMap as _,
};
#[doc(hidden)]
mod sp_api_hidden_includes_decl_storage {
    pub extern crate frame_support as hidden_include;
}
trait Store {
    type NameOf;
}
impl<T: Trait + 'static> Store for Module<T> {
    type NameOf = NameOf<T>;
}
impl<T: Trait + 'static> Module<T> {}
#[doc(hidden)]
pub struct __GetByteStructNameOf<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_NameOf:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Trait> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructNameOf<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_NameOf
            .get_or_init(|| {
                let def_val: Option<(Vec<u8>, BalanceOf<T>)> = Default::default();
                <Option<(Vec<u8>, BalanceOf<T>)> as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Trait> Send for __GetByteStructNameOf<T> {}
unsafe impl<T: Trait> Sync for __GetByteStructNameOf<T> {}
impl<T: Trait + 'static> Module<T> {
    #[doc(hidden)]
    pub fn storage_metadata(
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata {
        self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageMetadata {
            prefix : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "Nicks" ) ,
            entries : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (
                & [ self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata {
                    name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "NameOf" ) ,
                    modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Optional ,
                    ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map {
                        hasher : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Twox64Concat ,
                        key : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::AccountId" ) ,
                        value : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "(Vec<u8>, BalanceOf<T>)" ) ,
                        unused : false , } , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (
                        self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructNameOf :: < T > (
                            self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData ) ) ) ,
                    documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ " The lookup table for names." ] ) , } ] [ .. ] ) , }
    }
}
/// Hidden instance generated to be internally used when module is used without
/// instance.
#[doc(hidden)]
pub struct __InherentHiddenInstance;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for __InherentHiddenInstance {
    #[inline]
    fn clone(&self) -> __InherentHiddenInstance {
        match *self {
            __InherentHiddenInstance => __InherentHiddenInstance,
        }
    }
}
impl ::core::marker::StructuralEq for __InherentHiddenInstance {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for __InherentHiddenInstance {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {}
    }
}
impl ::core::marker::StructuralPartialEq for __InherentHiddenInstance {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for __InherentHiddenInstance {
    #[inline]
    fn eq(&self, other: &__InherentHiddenInstance) -> bool {
        match *other {
            __InherentHiddenInstance => match *self {
                __InherentHiddenInstance => true,
            },
        }
    }
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Encode for __InherentHiddenInstance {
        fn encode_to<EncOut: _parity_scale_codec::Output>(&self, dest: &mut EncOut) {}
    }
    impl _parity_scale_codec::EncodeLike for __InherentHiddenInstance {}
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Decode for __InherentHiddenInstance {
        fn decode<DecIn: _parity_scale_codec::Input>(
            input: &mut DecIn,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            Ok(__InherentHiddenInstance)
        }
    }
};
impl core::fmt::Debug for __InherentHiddenInstance {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_tuple("__InherentHiddenInstance").finish()
    }
}
impl self::sp_api_hidden_includes_decl_storage::hidden_include::traits::Instance
    for __InherentHiddenInstance
{
    const PREFIX: &'static str = "Nicks";
}
/// The lookup table for names.
struct NameOf<T: Trait>(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T,)>,
);
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StoragePrefixedMap<(
        Vec<u8>,
        BalanceOf<T>,
    )> for NameOf<T>
{
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ( )
    }
    fn storage_prefix() -> &'static [u8] {
        b"NameOf"
    }
}
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        T::AccountId,
        (Vec<u8>, BalanceOf<T>),
    > for NameOf<T>
{
    type Query = Option<(Vec<u8>, BalanceOf<T>)>;
    type Hasher = self::sp_api_hidden_includes_decl_storage::hidden_include::Twox64Concat;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ( )
    }
    fn storage_prefix() -> &'static [u8] {
        b"NameOf"
    }
    fn from_optional_value_to_query(v: Option<(Vec<u8>, BalanceOf<T>)>) -> Self::Query {
        v.or_else(|| Default::default())
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<(Vec<u8>, BalanceOf<T>)> {
        v
    }
}
/// [`RawEvent`] specialized for the configuration [`Trait`]
///
/// [`RawEvent`]: enum.RawEvent.html
/// [`Trait`]: trait.Trait.html
pub type Event<T> = RawEvent<<T as frame_system::Trait>::AccountId, BalanceOf<T>>;
/// Events for this module.
///
pub enum RawEvent<AccountId, Balance> {
    /// A name was set. [who]
    NameSet(AccountId),
    /// A name was forcibly set. [target]
    NameForced(AccountId),
    /// A name was changed. [who]
    NameChanged(AccountId),
    /// A name was cleared, and the given balance returned. [who, deposit]
    NameCleared(AccountId, Balance),
    /// A name was removed and the given balance slashed. [target, deposit]
    NameKilled(AccountId, Balance),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<AccountId: ::core::clone::Clone, Balance: ::core::clone::Clone> ::core::clone::Clone
    for RawEvent<AccountId, Balance>
{
    #[inline]
    fn clone(&self) -> RawEvent<AccountId, Balance> {
        match (&*self,) {
            (&RawEvent::NameSet(ref __self_0),) => {
                RawEvent::NameSet(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&RawEvent::NameForced(ref __self_0),) => {
                RawEvent::NameForced(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&RawEvent::NameChanged(ref __self_0),) => {
                RawEvent::NameChanged(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&RawEvent::NameCleared(ref __self_0, ref __self_1),) => RawEvent::NameCleared(
                ::core::clone::Clone::clone(&(*__self_0)),
                ::core::clone::Clone::clone(&(*__self_1)),
            ),
            (&RawEvent::NameKilled(ref __self_0, ref __self_1),) => RawEvent::NameKilled(
                ::core::clone::Clone::clone(&(*__self_0)),
                ::core::clone::Clone::clone(&(*__self_1)),
            ),
        }
    }
}
impl<AccountId, Balance> ::core::marker::StructuralPartialEq for RawEvent<AccountId, Balance> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<AccountId: ::core::cmp::PartialEq, Balance: ::core::cmp::PartialEq> ::core::cmp::PartialEq
    for RawEvent<AccountId, Balance>
{
    #[inline]
    fn eq(&self, other: &RawEvent<AccountId, Balance>) -> bool {
        {
            let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) };
            let __arg_1_vi = unsafe { ::core::intrinsics::discriminant_value(&*other) };
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&RawEvent::NameSet(ref __self_0), &RawEvent::NameSet(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&RawEvent::NameForced(ref __self_0), &RawEvent::NameForced(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (
                        &RawEvent::NameChanged(ref __self_0),
                        &RawEvent::NameChanged(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    (
                        &RawEvent::NameCleared(ref __self_0, ref __self_1),
                        &RawEvent::NameCleared(ref __arg_1_0, ref __arg_1_1),
                    ) => (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1),
                    (
                        &RawEvent::NameKilled(ref __self_0, ref __self_1),
                        &RawEvent::NameKilled(ref __arg_1_0, ref __arg_1_1),
                    ) => (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1),
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &RawEvent<AccountId, Balance>) -> bool {
        {
            let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) };
            let __arg_1_vi = unsafe { ::core::intrinsics::discriminant_value(&*other) };
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&RawEvent::NameSet(ref __self_0), &RawEvent::NameSet(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&RawEvent::NameForced(ref __self_0), &RawEvent::NameForced(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (
                        &RawEvent::NameChanged(ref __self_0),
                        &RawEvent::NameChanged(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    (
                        &RawEvent::NameCleared(ref __self_0, ref __self_1),
                        &RawEvent::NameCleared(ref __arg_1_0, ref __arg_1_1),
                    ) => (*__self_0) != (*__arg_1_0) || (*__self_1) != (*__arg_1_1),
                    (
                        &RawEvent::NameKilled(ref __self_0, ref __self_1),
                        &RawEvent::NameKilled(ref __arg_1_0, ref __arg_1_1),
                    ) => (*__self_0) != (*__arg_1_0) || (*__self_1) != (*__arg_1_1),
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                true
            }
        }
    }
}
impl<AccountId, Balance> ::core::marker::StructuralEq for RawEvent<AccountId, Balance> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<AccountId: ::core::cmp::Eq, Balance: ::core::cmp::Eq> ::core::cmp::Eq
    for RawEvent<AccountId, Balance>
{
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<Balance>;
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<Balance>;
        }
    }
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<AccountId, Balance> _parity_scale_codec::Encode for RawEvent<AccountId, Balance>
    where
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        Balance: _parity_scale_codec::Encode,
        Balance: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        Balance: _parity_scale_codec::Encode,
        Balance: _parity_scale_codec::Encode,
    {
        fn encode_to<EncOut: _parity_scale_codec::Output>(&self, dest: &mut EncOut) {
            match *self {
                RawEvent::NameSet(ref aa) => {
                    dest.push_byte(0usize as u8);
                    dest.push(aa);
                }
                RawEvent::NameForced(ref aa) => {
                    dest.push_byte(1usize as u8);
                    dest.push(aa);
                }
                RawEvent::NameChanged(ref aa) => {
                    dest.push_byte(2usize as u8);
                    dest.push(aa);
                }
                RawEvent::NameCleared(ref aa, ref ba) => {
                    dest.push_byte(3usize as u8);
                    dest.push(aa);
                    dest.push(ba);
                }
                RawEvent::NameKilled(ref aa, ref ba) => {
                    dest.push_byte(4usize as u8);
                    dest.push(aa);
                    dest.push(ba);
                }
                _ => (),
            }
        }
    }
    impl<AccountId, Balance> _parity_scale_codec::EncodeLike for RawEvent<AccountId, Balance>
    where
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        Balance: _parity_scale_codec::Encode,
        Balance: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        Balance: _parity_scale_codec::Encode,
        Balance: _parity_scale_codec::Encode,
    {
    }
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<AccountId, Balance> _parity_scale_codec::Decode for RawEvent<AccountId, Balance>
    where
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        Balance: _parity_scale_codec::Decode,
        Balance: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        Balance: _parity_scale_codec::Decode,
        Balance: _parity_scale_codec::Decode,
    {
        fn decode<DecIn: _parity_scale_codec::Input>(
            input: &mut DecIn,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            match input.read_byte()? {
                x if x == 0usize as u8 => Ok(RawEvent::NameSet({
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field RawEvent :: NameSet.0".into()),
                        Ok(a) => a,
                    }
                })),
                x if x == 1usize as u8 => Ok(RawEvent::NameForced({
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => {
                            return Err("Error decoding field RawEvent :: NameForced.0".into())
                        }
                        Ok(a) => a,
                    }
                })),
                x if x == 2usize as u8 => Ok(RawEvent::NameChanged({
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => {
                            return Err("Error decoding field RawEvent :: NameChanged.0".into())
                        }
                        Ok(a) => a,
                    }
                })),
                x if x == 3usize as u8 => Ok(RawEvent::NameCleared(
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field RawEvent :: NameCleared.0".into())
                            }
                            Ok(a) => a,
                        }
                    },
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field RawEvent :: NameCleared.1".into())
                            }
                            Ok(a) => a,
                        }
                    },
                )),
                x if x == 4usize as u8 => Ok(RawEvent::NameKilled(
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field RawEvent :: NameKilled.0".into())
                            }
                            Ok(a) => a,
                        }
                    },
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field RawEvent :: NameKilled.1".into())
                            }
                            Ok(a) => a,
                        }
                    },
                )),
                x => Err("No such variant in enum RawEvent".into()),
            }
        }
    }
};
impl<AccountId, Balance> core::fmt::Debug for RawEvent<AccountId, Balance>
where
    AccountId: core::fmt::Debug,
    Balance: core::fmt::Debug,
{
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::NameSet(ref a0) => fmt.debug_tuple("RawEvent::NameSet").field(a0).finish(),
            Self::NameForced(ref a0) => fmt.debug_tuple("RawEvent::NameForced").field(a0).finish(),
            Self::NameChanged(ref a0) => {
                fmt.debug_tuple("RawEvent::NameChanged").field(a0).finish()
            }
            Self::NameCleared(ref a0, ref a1) => fmt
                .debug_tuple("RawEvent::NameCleared")
                .field(a0)
                .field(a1)
                .finish(),
            Self::NameKilled(ref a0, ref a1) => fmt
                .debug_tuple("RawEvent::NameKilled")
                .field(a0)
                .field(a1)
                .finish(),
            _ => Ok(()),
        }
    }
}
impl<AccountId, Balance> From<RawEvent<AccountId, Balance>> for () {
    fn from(_: RawEvent<AccountId, Balance>) -> () {
        ()
    }
}
impl<AccountId, Balance> RawEvent<AccountId, Balance> {
    #[allow(dead_code)]
    #[doc(hidden)]
    pub fn metadata() -> &'static [::frame_support::event::EventMetadata] {
        &[
            ::frame_support::event::EventMetadata {
                name: ::frame_support::event::DecodeDifferent::Encode("NameSet"),
                arguments: ::frame_support::event::DecodeDifferent::Encode(&["AccountId"]),
                documentation: ::frame_support::event::DecodeDifferent::Encode(&[
                    r" A name was set. [who]",
                ]),
            },
            ::frame_support::event::EventMetadata {
                name: ::frame_support::event::DecodeDifferent::Encode("NameForced"),
                arguments: ::frame_support::event::DecodeDifferent::Encode(&["AccountId"]),
                documentation: ::frame_support::event::DecodeDifferent::Encode(&[
                    r" A name was forcibly set. [target]",
                ]),
            },
            ::frame_support::event::EventMetadata {
                name: ::frame_support::event::DecodeDifferent::Encode("NameChanged"),
                arguments: ::frame_support::event::DecodeDifferent::Encode(&["AccountId"]),
                documentation: ::frame_support::event::DecodeDifferent::Encode(&[
                    r" A name was changed. [who]",
                ]),
            },
            ::frame_support::event::EventMetadata {
                name: ::frame_support::event::DecodeDifferent::Encode("NameCleared"),
                arguments: ::frame_support::event::DecodeDifferent::Encode(&[
                    "AccountId",
                    "Balance",
                ]),
                documentation: ::frame_support::event::DecodeDifferent::Encode(&[
                    r" A name was cleared, and the given balance returned. [who, deposit]",
                ]),
            },
            ::frame_support::event::EventMetadata {
                name: ::frame_support::event::DecodeDifferent::Encode("NameKilled"),
                arguments: ::frame_support::event::DecodeDifferent::Encode(&[
                    "AccountId",
                    "Balance",
                ]),
                documentation: ::frame_support::event::DecodeDifferent::Encode(&[
                    r" A name was removed and the given balance slashed. [target, deposit]",
                ]),
            },
        ]
    }
}
/// Error for the nicks module.
pub enum Error<T: Trait> {
    #[doc(hidden)]
    __Ignore(
        ::frame_support::sp_std::marker::PhantomData<(T,)>,
        ::frame_support::Never,
    ),
    /// A name is too short.
    TooShort,
    /// A name is too long.
    TooLong,
    /// An account isn't named.
    Unnamed,
}
impl<T: Trait> ::frame_support::sp_std::fmt::Debug for Error<T> {
    fn fmt(
        &self,
        f: &mut ::frame_support::sp_std::fmt::Formatter<'_>,
    ) -> ::frame_support::sp_std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl<T: Trait> Error<T> {
    fn as_u8(&self) -> u8 {
        match self {
            Error::__Ignore(_, _) => ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                &["internal error: entered unreachable code: "],
                &match (&"`__Ignore` can never be constructed",) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            )),
            Error::TooShort => 0,
            Error::TooLong => 0 + 1,
            Error::Unnamed => 0 + 1 + 1,
        }
    }
    fn as_str(&self) -> &'static str {
        match self {
            Self::__Ignore(_, _) => ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                &["internal error: entered unreachable code: "],
                &match (&"`__Ignore` can never be constructed",) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            )),
            Error::TooShort => "TooShort",
            Error::TooLong => "TooLong",
            Error::Unnamed => "Unnamed",
        }
    }
}
impl<T: Trait> From<Error<T>> for &'static str {
    fn from(err: Error<T>) -> &'static str {
        err.as_str()
    }
}
impl<T: Trait> From<Error<T>> for ::frame_support::sp_runtime::DispatchError {
    fn from(err: Error<T>) -> Self {
        let index = <T::ModuleToIndex as ::frame_support::traits::ModuleToIndex>::module_to_index::<
            Module<T>,
        >()
        .expect("Every active module has an index in the runtime; qed") as u8;
        ::frame_support::sp_runtime::DispatchError::Module {
            index,
            error: err.as_u8(),
            message: Some(err.as_str()),
        }
    }
}
impl<T: Trait> ::frame_support::error::ModuleErrorMetadata for Error<T> {
    fn metadata() -> &'static [::frame_support::error::ErrorMetadata] {
        &[
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("TooShort"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[
                    r" A name is too short.",
                ]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("TooLong"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[
                    r" A name is too long.",
                ]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("Unnamed"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[
                    r" An account isn't named.",
                ]),
            },
        ]
    }
}
/// Nicks module declaration.
pub struct Module<T: Trait>(::frame_support::sp_std::marker::PhantomData<(T,)>);
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::clone::Clone + Trait> ::core::clone::Clone for Module<T> {
    #[inline]
    fn clone(&self) -> Module<T> {
        match *self {
            Module(ref __self_0_0) => Module(::core::clone::Clone::clone(&(*__self_0_0))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::marker::Copy + Trait> ::core::marker::Copy for Module<T> {}
impl<T: Trait> ::core::marker::StructuralPartialEq for Module<T> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::cmp::PartialEq + Trait> ::core::cmp::PartialEq for Module<T> {
    #[inline]
    fn eq(&self, other: &Module<T>) -> bool {
        match *other {
            Module(ref __self_1_0) => match *self {
                Module(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Module<T>) -> bool {
        match *other {
            Module(ref __self_1_0) => match *self {
                Module(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
impl<T: Trait> ::core::marker::StructuralEq for Module<T> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::cmp::Eq + Trait> ::core::cmp::Eq for Module<T> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<
                ::frame_support::sp_std::marker::PhantomData<(T,)>,
            >;
        }
    }
}
impl<T: Trait> core::fmt::Debug for Module<T>
where
    T: core::fmt::Debug,
{
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_tuple("Module").field(&self.0).finish()
    }
}
impl<T: Trait> ::frame_support::traits::OnInitialize<T::BlockNumber> for Module<T> {}
impl<T: Trait> ::frame_support::traits::OnRuntimeUpgrade for Module<T> {}
impl<T: Trait> ::frame_support::traits::OnFinalize<T::BlockNumber> for Module<T> {}
impl<T: Trait> ::frame_support::traits::OffchainWorker<T::BlockNumber> for Module<T> {}
impl<T: Trait> Module<T> {
    /// Deposits an event using `frame_system::Module::deposit_event`.
    fn deposit_event(event: impl Into<<T as Trait>::Event>) {
        <frame_system::Module<T>>::deposit_event(event.into())
    }
}
#[cfg(feature = "std")]
impl<T: Trait> ::frame_support::traits::IntegrityTest for Module<T> {}
/// Can also be called using [`Call`].
///
/// [`Call`]: enum.Call.html
impl<T: Trait> Module<T> {
    #[allow(unreachable_code)]
    /// Set an account's name. The name should be a UTF-8-encoded string by convention, though
    /// we don't check it.
    ///
    /// The name may not be more than `T::MaxLength` bytes, nor less than `T::MinLength` bytes.
    ///
    /// If the account doesn't already have a name, then a fee of `ReservationFee` is reserved
    /// in the account.
    ///
    /// The dispatch origin for this call must be _Signed_.
    ///
    /// # <weight>
    /// - O(1).
    /// - At most one balance operation.
    /// - One storage read/write.
    /// - One event.
    /// # </weight>
    ///
    /// NOTE: Calling this function will bypass origin filters.
    fn set_name(origin: T::Origin, name: Vec<u8>) -> ::frame_support::dispatch::DispatchResult {
        let __tracing_span__ = {
            {
                if ::sp_tracing::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::sp_tracing::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                {
                    use ::tracing::__macro_support::*;
                    let callsite = {
                        use ::tracing::__macro_support::MacroCallsite;
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "set_name",
                                "pallet_template",
                                ::sp_tracing::tracing::Level::TRACE,
                                Some("pallets/template/src/lib.rs"),
                                Some(114u32),
                                Some("pallet_template"),
                                ::tracing_core::field::FieldSet::new(
                                    &[],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::SPAN,
                            )
                        };
                        static CALLSITE: MacroCallsite = MacroCallsite::new(&META);
                        CALLSITE.register();
                        &CALLSITE
                    };
                    if callsite.is_enabled() {
                        let meta = callsite.metadata();
                        ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                    } else {
                        ::tracing::Span::none()
                    }
                } else {
                    ::tracing::Span::none()
                }
            }
        };
        let __tracing_guard__ = { __tracing_span__.enter() };
        {
            let sender = ensure_signed(origin)?;
            {
                if !(name.len() >= T::MinLength::get()) {
                    {
                        return Err(Error::<T>::TooShort.into());
                    };
                }
            };
            {
                if !(name.len() <= T::MaxLength::get()) {
                    {
                        return Err(Error::<T>::TooLong.into());
                    };
                }
            };
            let deposit = if let Some((_, deposit)) = <NameOf<T>>::get(&sender) {
                Self::deposit_event(RawEvent::NameChanged(sender.clone()));
                deposit
            } else {
                let deposit = T::ReservationFee::get();
                T::Currency::reserve(&sender, deposit.clone())?;
                Self::deposit_event(RawEvent::NameSet(sender.clone()));
                deposit
            };
            <NameOf<T>>::insert(&sender, (name, deposit));
        }
        Ok(())
    }
    #[allow(unreachable_code)]
    /// Clear an account's name and return the deposit. Fails if the account was not named.
    ///
    /// The dispatch origin for this call must be _Signed_.
    ///
    /// # <weight>
    /// - O(1).
    /// - One balance operation.
    /// - One storage read/write.
    /// - One event.
    /// # </weight>
    ///
    /// NOTE: Calling this function will bypass origin filters.
    fn clear_name(origin: T::Origin) -> ::frame_support::dispatch::DispatchResult {
        let __tracing_span__ = {
            {
                if ::sp_tracing::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::sp_tracing::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                {
                    use ::tracing::__macro_support::*;
                    let callsite = {
                        use ::tracing::__macro_support::MacroCallsite;
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "clear_name",
                                "pallet_template",
                                ::sp_tracing::tracing::Level::TRACE,
                                Some("pallets/template/src/lib.rs"),
                                Some(114u32),
                                Some("pallet_template"),
                                ::tracing_core::field::FieldSet::new(
                                    &[],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::SPAN,
                            )
                        };
                        static CALLSITE: MacroCallsite = MacroCallsite::new(&META);
                        CALLSITE.register();
                        &CALLSITE
                    };
                    if callsite.is_enabled() {
                        let meta = callsite.metadata();
                        ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                    } else {
                        ::tracing::Span::none()
                    }
                } else {
                    ::tracing::Span::none()
                }
            }
        };
        let __tracing_guard__ = { __tracing_span__.enter() };
        {
            let sender = ensure_signed(origin)?;
            let deposit = <NameOf<T>>::take(&sender).ok_or(Error::<T>::Unnamed)?.1;
            let _ = T::Currency::unreserve(&sender, deposit.clone());
            Self::deposit_event(RawEvent::NameCleared(sender, deposit));
        }
        Ok(())
    }
    #[allow(unreachable_code)]
    /// Remove an account's name and take charge of the deposit.
    ///
    /// Fails if `who` has not been named. The deposit is dealt with through `T::Slashed`
    /// imbalance handler.
    ///
    /// The dispatch origin for this call must match `T::ForceOrigin`.
    ///
    /// # <weight>
    /// - O(1).
    /// - One unbalanced handler (probably a balance transfer)
    /// - One storage read/write.
    /// - One event.
    /// # </weight>
    ///
    /// NOTE: Calling this function will bypass origin filters.
    fn kill_name(
        origin: T::Origin,
        target: <T::Lookup as StaticLookup>::Source,
    ) -> ::frame_support::dispatch::DispatchResult {
        let __tracing_span__ = {
            {
                if ::sp_tracing::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::sp_tracing::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                {
                    use ::tracing::__macro_support::*;
                    let callsite = {
                        use ::tracing::__macro_support::MacroCallsite;
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "kill_name",
                                "pallet_template",
                                ::sp_tracing::tracing::Level::TRACE,
                                Some("pallets/template/src/lib.rs"),
                                Some(114u32),
                                Some("pallet_template"),
                                ::tracing_core::field::FieldSet::new(
                                    &[],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::SPAN,
                            )
                        };
                        static CALLSITE: MacroCallsite = MacroCallsite::new(&META);
                        CALLSITE.register();
                        &CALLSITE
                    };
                    if callsite.is_enabled() {
                        let meta = callsite.metadata();
                        ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                    } else {
                        ::tracing::Span::none()
                    }
                } else {
                    ::tracing::Span::none()
                }
            }
        };
        let __tracing_guard__ = { __tracing_span__.enter() };
        {
            T::ForceOrigin::ensure_origin(origin)?;
            let target = T::Lookup::lookup(target)?;
            let deposit = <NameOf<T>>::take(&target).ok_or(Error::<T>::Unnamed)?.1;
            T::Slashed::on_unbalanced(T::Currency::slash_reserved(&target, deposit.clone()).0);
            Self::deposit_event(RawEvent::NameKilled(target, deposit));
        }
        Ok(())
    }
    #[allow(unreachable_code)]
    /// Set a third-party account's name with no deposit.
    ///
    /// No length checking is done on the name.
    ///
    /// The dispatch origin for this call must match `T::ForceOrigin`.
    ///
    /// # <weight>
    /// - O(1).
    /// - At most one balance operation.
    /// - One storage read/write.
    /// - One event.
    /// # </weight>
    ///
    /// NOTE: Calling this function will bypass origin filters.
    fn force_name(
        origin: T::Origin,
        target: <T::Lookup as StaticLookup>::Source,
        name: Vec<u8>,
    ) -> ::frame_support::dispatch::DispatchResult {
        let __tracing_span__ = {
            {
                if ::sp_tracing::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::sp_tracing::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                {
                    use ::tracing::__macro_support::*;
                    let callsite = {
                        use ::tracing::__macro_support::MacroCallsite;
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "force_name",
                                "pallet_template",
                                ::sp_tracing::tracing::Level::TRACE,
                                Some("pallets/template/src/lib.rs"),
                                Some(114u32),
                                Some("pallet_template"),
                                ::tracing_core::field::FieldSet::new(
                                    &[],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::SPAN,
                            )
                        };
                        static CALLSITE: MacroCallsite = MacroCallsite::new(&META);
                        CALLSITE.register();
                        &CALLSITE
                    };
                    if callsite.is_enabled() {
                        let meta = callsite.metadata();
                        ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                    } else {
                        ::tracing::Span::none()
                    }
                } else {
                    ::tracing::Span::none()
                }
            }
        };
        let __tracing_guard__ = { __tracing_span__.enter() };
        {
            T::ForceOrigin::ensure_origin(origin)?;
            let target = T::Lookup::lookup(target)?;
            let deposit = <NameOf<T>>::get(&target)
                .map(|x| x.1)
                .unwrap_or_else(Zero::zero);
            <NameOf<T>>::insert(&target, (name, deposit));
            Self::deposit_event(RawEvent::NameForced(target));
        }
        Ok(())
    }
}
/// Dispatchable calls.
///
/// Each variant of this enum maps to a dispatchable function from the associated module.
pub enum Call<T: Trait> {
    #[doc(hidden)]
    #[codec(skip)]
    __PhantomItem(
        ::frame_support::sp_std::marker::PhantomData<(T,)>,
        ::frame_support::Never,
    ),
    #[allow(non_camel_case_types)]
    /// Set an account's name. The name should be a UTF-8-encoded string by convention, though
    /// we don't check it.
    ///
    /// The name may not be more than `T::MaxLength` bytes, nor less than `T::MinLength` bytes.
    ///
    /// If the account doesn't already have a name, then a fee of `ReservationFee` is reserved
    /// in the account.
    ///
    /// The dispatch origin for this call must be _Signed_.
    ///
    /// # <weight>
    /// - O(1).
    /// - At most one balance operation.
    /// - One storage read/write.
    /// - One event.
    /// # </weight>
    set_name(Vec<u8>),
    #[allow(non_camel_case_types)]
    /// Clear an account's name and return the deposit. Fails if the account was not named.
    ///
    /// The dispatch origin for this call must be _Signed_.
    ///
    /// # <weight>
    /// - O(1).
    /// - One balance operation.
    /// - One storage read/write.
    /// - One event.
    /// # </weight>
    clear_name(),
    #[allow(non_camel_case_types)]
    /// Remove an account's name and take charge of the deposit.
    ///
    /// Fails if `who` has not been named. The deposit is dealt with through `T::Slashed`
    /// imbalance handler.
    ///
    /// The dispatch origin for this call must match `T::ForceOrigin`.
    ///
    /// # <weight>
    /// - O(1).
    /// - One unbalanced handler (probably a balance transfer)
    /// - One storage read/write.
    /// - One event.
    /// # </weight>
    kill_name(<T::Lookup as StaticLookup>::Source),
    #[allow(non_camel_case_types)]
    /// Set a third-party account's name with no deposit.
    ///
    /// No length checking is done on the name.
    ///
    /// The dispatch origin for this call must match `T::ForceOrigin`.
    ///
    /// # <weight>
    /// - O(1).
    /// - At most one balance operation.
    /// - One storage read/write.
    /// - One event.
    /// # </weight>
    force_name(<T::Lookup as StaticLookup>::Source, Vec<u8>),
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<T: Trait> _parity_scale_codec::Encode for Call<T>
    where
        <T::Lookup as StaticLookup>::Source: _parity_scale_codec::Encode,
        <T::Lookup as StaticLookup>::Source: _parity_scale_codec::Encode,
        <T::Lookup as StaticLookup>::Source: _parity_scale_codec::Encode,
        <T::Lookup as StaticLookup>::Source: _parity_scale_codec::Encode,
    {
        fn encode_to<EncOut: _parity_scale_codec::Output>(&self, dest: &mut EncOut) {
            match *self {
                Call::set_name(ref aa) => {
                    dest.push_byte(0usize as u8);
                    dest.push(aa);
                }
                Call::clear_name() => {
                    dest.push_byte(1usize as u8);
                }
                Call::kill_name(ref aa) => {
                    dest.push_byte(2usize as u8);
                    dest.push(aa);
                }
                Call::force_name(ref aa, ref ba) => {
                    dest.push_byte(3usize as u8);
                    dest.push(aa);
                    dest.push(ba);
                }
                _ => (),
            }
        }
    }
    impl<T: Trait> _parity_scale_codec::EncodeLike for Call<T>
    where
        <T::Lookup as StaticLookup>::Source: _parity_scale_codec::Encode,
        <T::Lookup as StaticLookup>::Source: _parity_scale_codec::Encode,
        <T::Lookup as StaticLookup>::Source: _parity_scale_codec::Encode,
        <T::Lookup as StaticLookup>::Source: _parity_scale_codec::Encode,
    {
    }
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<T: Trait> _parity_scale_codec::Decode for Call<T>
    where
        <T::Lookup as StaticLookup>::Source: _parity_scale_codec::Decode,
        <T::Lookup as StaticLookup>::Source: _parity_scale_codec::Decode,
        <T::Lookup as StaticLookup>::Source: _parity_scale_codec::Decode,
        <T::Lookup as StaticLookup>::Source: _parity_scale_codec::Decode,
    {
        fn decode<DecIn: _parity_scale_codec::Input>(
            input: &mut DecIn,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            match input.read_byte()? {
                x if x == 0usize as u8 => Ok(Call::set_name({
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field Call :: set_name.0".into()),
                        Ok(a) => a,
                    }
                })),
                x if x == 1usize as u8 => Ok(Call::clear_name()),
                x if x == 2usize as u8 => Ok(Call::kill_name({
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field Call :: kill_name.0".into()),
                        Ok(a) => a,
                    }
                })),
                x if x == 3usize as u8 => Ok(Call::force_name(
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field Call :: force_name.0".into())
                            }
                            Ok(a) => a,
                        }
                    },
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field Call :: force_name.1".into())
                            }
                            Ok(a) => a,
                        }
                    },
                )),
                x => Err("No such variant in enum Call".into()),
            }
        }
    }
};
impl<T: Trait> ::frame_support::dispatch::GetDispatchInfo for Call<T> {
    fn get_dispatch_info(&self) -> ::frame_support::dispatch::DispatchInfo {
        match *self {
            Call::set_name(ref name) => {
                let base_weight = 50_000_000;
                let weight = <dyn ::frame_support::dispatch::WeighData<(&Vec<u8>,)>>::weigh_data(
                    &base_weight,
                    (name,),
                );
                let class = < dyn :: frame_support :: dispatch :: ClassifyDispatch < ( & Vec < u8 > , ) > > :: classify_dispatch ( & base_weight , ( name , ) ) ;
                let pays_fee = <dyn ::frame_support::dispatch::PaysFee<(&Vec<u8>,)>>::pays_fee(
                    &base_weight,
                    (name,),
                );
                ::frame_support::dispatch::DispatchInfo {
                    weight,
                    class,
                    pays_fee,
                }
            }
            Call::clear_name() => {
                let base_weight = 70_000_000;
                let weight =
                    <dyn ::frame_support::dispatch::WeighData<()>>::weigh_data(&base_weight, ());
                let class =
                    <dyn ::frame_support::dispatch::ClassifyDispatch<()>>::classify_dispatch(
                        &base_weight,
                        (),
                    );
                let pays_fee =
                    <dyn ::frame_support::dispatch::PaysFee<()>>::pays_fee(&base_weight, ());
                ::frame_support::dispatch::DispatchInfo {
                    weight,
                    class,
                    pays_fee,
                }
            }
            Call::kill_name(ref target) => {
                let base_weight = 70_000_000;
                let weight = <dyn ::frame_support::dispatch::WeighData<(
                    &<T::Lookup as StaticLookup>::Source,
                )>>::weigh_data(&base_weight, (target,));
                let class = <dyn ::frame_support::dispatch::ClassifyDispatch<(
                    &<T::Lookup as StaticLookup>::Source,
                )>>::classify_dispatch(&base_weight, (target,));
                let pays_fee = <dyn ::frame_support::dispatch::PaysFee<(
                    &<T::Lookup as StaticLookup>::Source,
                )>>::pays_fee(&base_weight, (target,));
                ::frame_support::dispatch::DispatchInfo {
                    weight,
                    class,
                    pays_fee,
                }
            }
            Call::force_name(ref target, ref name) => {
                let base_weight = 70_000_000;
                let weight = <dyn ::frame_support::dispatch::WeighData<(
                    &<T::Lookup as StaticLookup>::Source,
                    &Vec<u8>,
                )>>::weigh_data(&base_weight, (target, name));
                let class = <dyn ::frame_support::dispatch::ClassifyDispatch<(
                    &<T::Lookup as StaticLookup>::Source,
                    &Vec<u8>,
                )>>::classify_dispatch(&base_weight, (target, name));
                let pays_fee = <dyn ::frame_support::dispatch::PaysFee<(
                    &<T::Lookup as StaticLookup>::Source,
                    &Vec<u8>,
                )>>::pays_fee(&base_weight, (target, name));
                ::frame_support::dispatch::DispatchInfo {
                    weight,
                    class,
                    pays_fee,
                }
            }
            Call::__PhantomItem(_, _) => {
                ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                    &["internal error: entered unreachable code: "],
                    &match (&"__PhantomItem should never be used.",) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ))
            }
        }
    }
}
impl<T: Trait> ::frame_support::dispatch::GetCallName for Call<T> {
    fn get_call_name(&self) -> &'static str {
        match *self {
            Call::set_name(ref name) => {
                let _ = (name);
                "set_name"
            }
            Call::clear_name() => {
                let _ = ();
                "clear_name"
            }
            Call::kill_name(ref target) => {
                let _ = (target);
                "kill_name"
            }
            Call::force_name(ref target, ref name) => {
                let _ = (target, name);
                "force_name"
            }
            Call::__PhantomItem(_, _) => {
                ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                    &["internal error: entered unreachable code: "],
                    &match (&"__PhantomItem should never be used.",) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ))
            }
        }
    }
    fn get_call_names() -> &'static [&'static str] {
        &["set_name", "clear_name", "kill_name", "force_name"]
    }
}
impl<T: Trait> ::frame_support::dispatch::Clone for Call<T> {
    fn clone(&self) -> Self {
        match *self {
            Call::set_name(ref name) => Call::set_name((*name).clone()),
            Call::clear_name() => Call::clear_name(),
            Call::kill_name(ref target) => Call::kill_name((*target).clone()),
            Call::force_name(ref target, ref name) => {
                Call::force_name((*target).clone(), (*name).clone())
            }
            _ => ::std::rt::begin_panic("internal error: entered unreachable code"),
        }
    }
}
impl<T: Trait> ::frame_support::dispatch::PartialEq for Call<T> {
    fn eq(&self, _other: &Self) -> bool {
        match *self {
            Call::set_name(ref name) => {
                let self_params = (name,);
                if let Call::set_name(ref name) = *_other {
                    self_params == (name,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            ::std::rt::begin_panic("internal error: entered unreachable code")
                        }
                        _ => false,
                    }
                }
            }
            Call::clear_name() => {
                let self_params = ();
                if let Call::clear_name() = *_other {
                    self_params == ()
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            ::std::rt::begin_panic("internal error: entered unreachable code")
                        }
                        _ => false,
                    }
                }
            }
            Call::kill_name(ref target) => {
                let self_params = (target,);
                if let Call::kill_name(ref target) = *_other {
                    self_params == (target,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            ::std::rt::begin_panic("internal error: entered unreachable code")
                        }
                        _ => false,
                    }
                }
            }
            Call::force_name(ref target, ref name) => {
                let self_params = (target, name);
                if let Call::force_name(ref target, ref name) = *_other {
                    self_params == (target, name)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            ::std::rt::begin_panic("internal error: entered unreachable code")
                        }
                        _ => false,
                    }
                }
            }
            _ => ::std::rt::begin_panic("internal error: entered unreachable code"),
        }
    }
}
impl<T: Trait> ::frame_support::dispatch::Eq for Call<T> {}
impl<T: Trait> ::frame_support::dispatch::fmt::Debug for Call<T> {
    fn fmt(
        &self,
        _f: &mut ::frame_support::dispatch::fmt::Formatter,
    ) -> ::frame_support::dispatch::result::Result<(), ::frame_support::dispatch::fmt::Error> {
        match *self {
            Call::set_name(ref name) => _f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", ""],
                &match (&"set_name", &(name.clone(),)) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                    ],
                },
            )),
            Call::clear_name() => _f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", ""],
                &match (&"clear_name", &()) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                    ],
                },
            )),
            Call::kill_name(ref target) => _f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", ""],
                &match (&"kill_name", &(target.clone(),)) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                    ],
                },
            )),
            Call::force_name(ref target, ref name) => _f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", ""],
                &match (&"force_name", &(target.clone(), name.clone())) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                    ],
                },
            )),
            _ => ::std::rt::begin_panic("internal error: entered unreachable code"),
        }
    }
}
impl<T: Trait> ::frame_support::traits::UnfilteredDispatchable for Call<T> {
    type Origin = T::Origin;
    fn dispatch_bypass_filter(
        self,
        _origin: Self::Origin,
    ) -> ::frame_support::dispatch::DispatchResultWithPostInfo {
        match self {
            Call::set_name(name) => <Module<T>>::set_name(_origin, name)
                .map(Into::into)
                .map_err(Into::into),
            Call::clear_name() => <Module<T>>::clear_name(_origin)
                .map(Into::into)
                .map_err(Into::into),
            Call::kill_name(target) => <Module<T>>::kill_name(_origin, target)
                .map(Into::into)
                .map_err(Into::into),
            Call::force_name(target, name) => <Module<T>>::force_name(_origin, target, name)
                .map(Into::into)
                .map_err(Into::into),
            Call::__PhantomItem(_, _) => {
                ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                    &["internal error: entered unreachable code: "],
                    &match (&"__PhantomItem should never be used.",) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ))
            }
        }
    }
}
impl<T: Trait> ::frame_support::dispatch::Callable<T> for Module<T> {
    type Call = Call<T>;
}
impl<T: Trait> Module<T> {
    #[doc(hidden)]
    pub fn call_functions() -> &'static [::frame_support::dispatch::FunctionMetadata] {
        &[
            ::frame_support::dispatch::FunctionMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode("set_name"),
                arguments: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("name"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("Vec<u8>"),
                    },
                ]),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    r" Set an account's name. The name should be a UTF-8-encoded string by convention, though",
                    r" we don't check it.",
                    r"",
                    r" The name may not be more than `T::MaxLength` bytes, nor less than `T::MinLength` bytes.",
                    r"",
                    r" If the account doesn't already have a name, then a fee of `ReservationFee` is reserved",
                    r" in the account.",
                    r"",
                    r" The dispatch origin for this call must be _Signed_.",
                    r"",
                    r" # <weight>",
                    r" - O(1).",
                    r" - At most one balance operation.",
                    r" - One storage read/write.",
                    r" - One event.",
                    r" # </weight>",
                ]),
            },
            ::frame_support::dispatch::FunctionMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode("clear_name"),
                arguments: ::frame_support::dispatch::DecodeDifferent::Encode(&[]),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    r" Clear an account's name and return the deposit. Fails if the account was not named.",
                    r"",
                    r" The dispatch origin for this call must be _Signed_.",
                    r"",
                    r" # <weight>",
                    r" - O(1).",
                    r" - One balance operation.",
                    r" - One storage read/write.",
                    r" - One event.",
                    r" # </weight>",
                ]),
            },
            ::frame_support::dispatch::FunctionMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode("kill_name"),
                arguments: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("target"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode(
                            "<T::Lookup as StaticLookup>::Source",
                        ),
                    },
                ]),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    r" Remove an account's name and take charge of the deposit.",
                    r"",
                    r" Fails if `who` has not been named. The deposit is dealt with through `T::Slashed`",
                    r" imbalance handler.",
                    r"",
                    r" The dispatch origin for this call must match `T::ForceOrigin`.",
                    r"",
                    r" # <weight>",
                    r" - O(1).",
                    r" - One unbalanced handler (probably a balance transfer)",
                    r" - One storage read/write.",
                    r" - One event.",
                    r" # </weight>",
                ]),
            },
            ::frame_support::dispatch::FunctionMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode("force_name"),
                arguments: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("target"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode(
                            "<T::Lookup as StaticLookup>::Source",
                        ),
                    },
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("name"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("Vec<u8>"),
                    },
                ]),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    r" Set a third-party account's name with no deposit.",
                    r"",
                    r" No length checking is done on the name.",
                    r"",
                    r" The dispatch origin for this call must match `T::ForceOrigin`.",
                    r"",
                    r" # <weight>",
                    r" - O(1).",
                    r" - At most one balance operation.",
                    r" - One storage read/write.",
                    r" - One event.",
                    r" # </weight>",
                ]),
            },
        ]
    }
}
impl<T: 'static + Trait> Module<T> {
    #[doc(hidden)]
    pub fn module_constants_metadata(
    ) -> &'static [::frame_support::dispatch::ModuleConstantMetadata] {
        #[allow(non_upper_case_types)]
        #[allow(non_camel_case_types)]
        struct ReservationFeeDefaultByteGetter<T: Trait>(
            ::frame_support::dispatch::marker::PhantomData<(T,)>,
        );
        impl<T: 'static + Trait> ::frame_support::dispatch::DefaultByte
            for ReservationFeeDefaultByteGetter<T>
        {
            fn default_byte(&self) -> ::frame_support::dispatch::Vec<u8> {
                let value: BalanceOf<T> = T::ReservationFee::get();
                ::frame_support::dispatch::Encode::encode(&value)
            }
        }
        unsafe impl<T: 'static + Trait> Send for ReservationFeeDefaultByteGetter<T> {}
        unsafe impl<T: 'static + Trait> Sync for ReservationFeeDefaultByteGetter<T> {}
        #[allow(non_upper_case_types)]
        #[allow(non_camel_case_types)]
        struct MinLengthDefaultByteGetter<T: Trait>(
            ::frame_support::dispatch::marker::PhantomData<(T,)>,
        );
        impl<T: 'static + Trait> ::frame_support::dispatch::DefaultByte for MinLengthDefaultByteGetter<T> {
            fn default_byte(&self) -> ::frame_support::dispatch::Vec<u8> {
                let value: u32 = T::MinLength::get() as u32;
                ::frame_support::dispatch::Encode::encode(&value)
            }
        }
        unsafe impl<T: 'static + Trait> Send for MinLengthDefaultByteGetter<T> {}
        unsafe impl<T: 'static + Trait> Sync for MinLengthDefaultByteGetter<T> {}
        #[allow(non_upper_case_types)]
        #[allow(non_camel_case_types)]
        struct MaxLengthDefaultByteGetter<T: Trait>(
            ::frame_support::dispatch::marker::PhantomData<(T,)>,
        );
        impl<T: 'static + Trait> ::frame_support::dispatch::DefaultByte for MaxLengthDefaultByteGetter<T> {
            fn default_byte(&self) -> ::frame_support::dispatch::Vec<u8> {
                let value: u32 = T::MaxLength::get() as u32;
                ::frame_support::dispatch::Encode::encode(&value)
            }
        }
        unsafe impl<T: 'static + Trait> Send for MaxLengthDefaultByteGetter<T> {}
        unsafe impl<T: 'static + Trait> Sync for MaxLengthDefaultByteGetter<T> {}
        &[
            ::frame_support::dispatch::ModuleConstantMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode("ReservationFee"),
                ty: ::frame_support::dispatch::DecodeDifferent::Encode("BalanceOf<T>"),
                value: ::frame_support::dispatch::DecodeDifferent::Encode(
                    ::frame_support::dispatch::DefaultByteGetter(
                        &ReservationFeeDefaultByteGetter::<T>(
                            ::frame_support::dispatch::marker::PhantomData,
                        ),
                    ),
                ),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    r" Reservation fee.",
                ]),
            },
            ::frame_support::dispatch::ModuleConstantMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode("MinLength"),
                ty: ::frame_support::dispatch::DecodeDifferent::Encode("u32"),
                value: ::frame_support::dispatch::DecodeDifferent::Encode(
                    ::frame_support::dispatch::DefaultByteGetter(&MinLengthDefaultByteGetter::<T>(
                        ::frame_support::dispatch::marker::PhantomData,
                    )),
                ),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    r" The minimum length a name may be.",
                ]),
            },
            ::frame_support::dispatch::ModuleConstantMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode("MaxLength"),
                ty: ::frame_support::dispatch::DecodeDifferent::Encode("u32"),
                value: ::frame_support::dispatch::DecodeDifferent::Encode(
                    ::frame_support::dispatch::DefaultByteGetter(&MaxLengthDefaultByteGetter::<T>(
                        ::frame_support::dispatch::marker::PhantomData,
                    )),
                ),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    r" The maximum length a name may be.",
                ]),
            },
        ]
    }
}
impl<T: Trait> ::frame_support::dispatch::ModuleErrorMetadata for Module<T> {
    fn metadata() -> &'static [::frame_support::dispatch::ErrorMetadata] {
        <Error<T> as ::frame_support::dispatch::ModuleErrorMetadata>::metadata()
    }
}
