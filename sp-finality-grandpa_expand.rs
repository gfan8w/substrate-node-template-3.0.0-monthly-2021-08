#![feature(prelude_import)]
//! Primitives for GRANDPA integration, suitable for WASM compilation.
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
#[cfg(feature = "std")]
use serde::Serialize;
use codec::{Codec, Decode, Encode, Input};
#[cfg(feature = "std")]
use sp_keystore::{SyncCryptoStore, SyncCryptoStorePtr};
use sp_runtime::{traits::NumberFor, ConsensusEngineId, RuntimeDebug};
use sp_std::{borrow::Cow, vec::Vec};
#[cfg(feature = "std")]
use log::debug;
/// Key type for GRANDPA module.
pub const KEY_TYPE: sp_core::crypto::KeyTypeId = sp_application_crypto::key_types::GRANDPA;
mod app {
    use sp_application_crypto::{app_crypto, ed25519, key_types::GRANDPA};
    /// A generic `AppPublic` wrapper type over $public crypto; this has no specific App.
    # [codec (crate = :: sp_application_crypto :: codec)]
    pub struct Public(ed25519::Public);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Public {
        #[inline]
        fn clone(&self) -> Public {
            match *self {
                Public(ref __self_0_0) => Public(::core::clone::Clone::clone(&(*__self_0_0))),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for Public {
        #[inline]
        fn default() -> Public {
            Public(::core::default::Default::default())
        }
    }
    impl ::core::marker::StructuralEq for Public {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for Public {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<ed25519::Public>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::hash::Hash for Public {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                Public(ref __self_0_0) => ::core::hash::Hash::hash(&(*__self_0_0), state),
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for Public {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Public {
        #[inline]
        fn eq(&self, other: &Public) -> bool {
            match *other {
                Public(ref __self_1_0) => match *self {
                    Public(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Public) -> bool {
            match *other {
                Public(ref __self_1_0) => match *self {
                    Public(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialOrd for Public {
        #[inline]
        fn partial_cmp(&self, other: &Public) -> ::core::option::Option<::core::cmp::Ordering> {
            match *other {
                Public(ref __self_1_0) => match *self {
                    Public(ref __self_0_0) => {
                        match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)) {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                            }
                            cmp => cmp,
                        }
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Ord for Public {
        #[inline]
        fn cmp(&self, other: &Public) -> ::core::cmp::Ordering {
            match *other {
                Public(ref __self_1_0) => match *self {
                    Public(ref __self_0_0) => {
                        match ::core::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                            ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                            cmp => cmp,
                        }
                    }
                },
            }
        }
    }
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl _parity_scale_codec::Encode for Public {
            fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                _parity_scale_codec::Encode::encode_to(&&self.0, __codec_dest_edqy)
            }
            fn encode(&self) -> _parity_scale_codec::alloc::vec::Vec<u8> {
                _parity_scale_codec::Encode::encode(&&self.0)
            }
            fn using_encoded<R, F: ::core::ops::FnOnce(&[::core::primitive::u8]) -> R>(
                &self,
                f: F,
            ) -> R {
                _parity_scale_codec::Encode::using_encoded(&&self.0, f)
            }
        }
        impl _parity_scale_codec::EncodeLike for Public {}
    };
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl _parity_scale_codec::Decode for Public {
            fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, _parity_scale_codec::Error> {
                ::core::result::Result::Ok(Public({
                    let __codec_res_edqy = <ed25519::Public as _parity_scale_codec::Decode>::decode(
                        __codec_input_edqy,
                    );
                    match __codec_res_edqy {
                        ::core::result::Result::Err(e) => {
                            return ::core::result::Result::Err(
                                e.chain("Could not decode `Public.0`"),
                            )
                        }
                        ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                }))
            }
        }
    };
    impl core::fmt::Debug for Public {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            fmt.debug_tuple("Public").field(&self.0).finish()
        }
    }
    const _: () = {
        impl ::sp_application_crypto::codec::MaxEncodedLen for Public {
            fn max_encoded_len() -> usize {
                0_usize.saturating_add(<ed25519::Public>::max_encoded_len())
            }
        }
    };
    impl ::sp_application_crypto::Wraps for Public {
        type Inner = ed25519::Public;
    }
    impl From<ed25519::Public> for Public {
        fn from(inner: ed25519::Public) -> Self {
            Self(inner)
        }
    }
    impl From<Public> for ed25519::Public {
        fn from(outer: Public) -> Self {
            outer.0
        }
    }
    impl AsRef<ed25519::Public> for Public {
        fn as_ref(&self) -> &ed25519::Public {
            &self.0
        }
    }
    impl AsMut<ed25519::Public> for Public {
        fn as_mut(&mut self) -> &mut ed25519::Public {
            &mut self.0
        }
    }
    impl ::sp_application_crypto::CryptoType for Public {
        type Pair = Pair;
    }
    impl ::sp_application_crypto::AppKey for Public {
        type UntypedGeneric = ed25519::Public;
        type Public = Public;
        type Pair = Pair;
        type Signature = Signature;
        const ID: ::sp_application_crypto::KeyTypeId = GRANDPA;
        const CRYPTO_ID: ::sp_application_crypto::CryptoTypeId = ed25519::CRYPTO_ID;
    }
    impl ::sp_application_crypto::Derive for Public {
        fn derive<Iter: Iterator<Item = ::sp_application_crypto::DeriveJunction>>(
            &self,
            path: Iter,
        ) -> Option<Self> {
            self.0.derive(path).map(Self)
        }
    }
    impl std::fmt::Display for Public {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            use ::sp_application_crypto::Ss58Codec;
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &[""],
                &match (&self.0.to_ss58check(),) {
                    _args => [::core::fmt::ArgumentV1::new(
                        _args.0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            ))
        }
    }
    impl ::sp_application_crypto::serde::Serialize for Public {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: ::sp_application_crypto::serde::Serializer,
        {
            use ::sp_application_crypto::Ss58Codec;
            serializer.serialize_str(&self.to_ss58check())
        }
    }
    impl<'de> ::sp_application_crypto::serde::Deserialize<'de> for Public {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: ::sp_application_crypto::serde::Deserializer<'de>,
        {
            use ::sp_application_crypto::Ss58Codec;
            Public::from_ss58check(&String::deserialize(deserializer)?).map_err(|e| {
                ::sp_application_crypto::serde::de::Error::custom({
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &[""],
                        &match (&e,) {
                            _args => [::core::fmt::ArgumentV1::new(
                                _args.0,
                                ::core::fmt::Debug::fmt,
                            )],
                        },
                    ));
                    res
                })
            })
        }
    }
    impl AsRef<[u8]> for Public {
        fn as_ref(&self) -> &[u8] {
            self.0.as_ref()
        }
    }
    impl AsMut<[u8]> for Public {
        fn as_mut(&mut self) -> &mut [u8] {
            self.0.as_mut()
        }
    }
    impl ::sp_application_crypto::Public for Public {
        fn from_slice(x: &[u8]) -> Self {
            Self(<ed25519::Public>::from_slice(x))
        }
        fn to_public_crypto_pair(&self) -> ::sp_application_crypto::CryptoTypePublicPair {
            ::sp_application_crypto::CryptoTypePublicPair(ed25519::CRYPTO_ID, self.to_raw_vec())
        }
    }
    impl ::sp_application_crypto::AppPublic for Public {
        type Generic = ed25519::Public;
    }
    impl ::sp_application_crypto::RuntimeAppPublic for Public
    where
        ed25519::Public: ::sp_application_crypto::RuntimePublic<Signature = ed25519::Signature>,
    {
        const ID: ::sp_application_crypto::KeyTypeId = GRANDPA;
        const CRYPTO_ID: ::sp_application_crypto::CryptoTypeId = ed25519::CRYPTO_ID;
        type Signature = Signature;
        fn all() -> ::sp_application_crypto::Vec<Self> {
            <ed25519::Public as ::sp_application_crypto::RuntimePublic>::all(GRANDPA)
                .into_iter()
                .map(Self)
                .collect()
        }
        fn generate_pair(seed: Option<::sp_application_crypto::Vec<u8>>) -> Self {
            Self(
                <ed25519::Public as ::sp_application_crypto::RuntimePublic>::generate_pair(
                    GRANDPA, seed,
                ),
            )
        }
        fn sign<M: AsRef<[u8]>>(&self, msg: &M) -> Option<Self::Signature> {
            <ed25519::Public as ::sp_application_crypto::RuntimePublic>::sign(
                self.as_ref(),
                GRANDPA,
                msg,
            )
            .map(Signature)
        }
        fn verify<M: AsRef<[u8]>>(&self, msg: &M, signature: &Self::Signature) -> bool {
            <ed25519::Public as ::sp_application_crypto::RuntimePublic>::verify(
                self.as_ref(),
                msg,
                &signature.as_ref(),
            )
        }
        fn to_raw_vec(&self) -> ::sp_application_crypto::Vec<u8> {
            <ed25519::Public as ::sp_application_crypto::RuntimePublic>::to_raw_vec(&self.0)
        }
    }
    impl From<Public> for ::sp_application_crypto::CryptoTypePublicPair {
        fn from(key: Public) -> Self {
            (&key).into()
        }
    }
    impl From<&Public> for ::sp_application_crypto::CryptoTypePublicPair {
        fn from(key: &Public) -> Self {
            ::sp_application_crypto::CryptoTypePublicPair(
                ed25519::CRYPTO_ID,
                ::sp_application_crypto::Public::to_raw_vec(key),
            )
        }
    }
    impl<'a> ::sp_application_crypto::TryFrom<&'a [u8]> for Public {
        type Error = ();
        fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
            <ed25519::Public>::try_from(data).map(Into::into)
        }
    }
    /// A generic `AppPublic` wrapper type over $public crypto; this has no specific App.
    pub struct Signature(ed25519::Signature);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::hash::Hash for Signature {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                Signature(ref __self_0_0) => ::core::hash::Hash::hash(&(*__self_0_0), state),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Signature {
        #[inline]
        fn clone(&self) -> Signature {
            match *self {
                Signature(ref __self_0_0) => Signature(::core::clone::Clone::clone(&(*__self_0_0))),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for Signature {
        #[inline]
        fn default() -> Signature {
            Signature(::core::default::Default::default())
        }
    }
    impl ::core::marker::StructuralEq for Signature {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for Signature {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<ed25519::Signature>;
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for Signature {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Signature {
        #[inline]
        fn eq(&self, other: &Signature) -> bool {
            match *other {
                Signature(ref __self_1_0) => match *self {
                    Signature(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Signature) -> bool {
            match *other {
                Signature(ref __self_1_0) => match *self {
                    Signature(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl _parity_scale_codec::Encode for Signature {
            fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                _parity_scale_codec::Encode::encode_to(&&self.0, __codec_dest_edqy)
            }
            fn encode(&self) -> _parity_scale_codec::alloc::vec::Vec<u8> {
                _parity_scale_codec::Encode::encode(&&self.0)
            }
            fn using_encoded<R, F: ::core::ops::FnOnce(&[::core::primitive::u8]) -> R>(
                &self,
                f: F,
            ) -> R {
                _parity_scale_codec::Encode::using_encoded(&&self.0, f)
            }
        }
        impl _parity_scale_codec::EncodeLike for Signature {}
    };
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl _parity_scale_codec::Decode for Signature {
            fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, _parity_scale_codec::Error> {
                ::core::result::Result::Ok(Signature({
                    let __codec_res_edqy =
                        <ed25519::Signature as _parity_scale_codec::Decode>::decode(
                            __codec_input_edqy,
                        );
                    match __codec_res_edqy {
                        ::core::result::Result::Err(e) => {
                            return ::core::result::Result::Err(
                                e.chain("Could not decode `Signature.0`"),
                            )
                        }
                        ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                }))
            }
        }
    };
    impl core::fmt::Debug for Signature {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            fmt.debug_tuple("Signature").field(&self.0).finish()
        }
    }
    impl ::sp_application_crypto::Wraps for Signature {
        type Inner = ed25519::Signature;
    }
    impl From<ed25519::Signature> for Signature {
        fn from(inner: ed25519::Signature) -> Self {
            Self(inner)
        }
    }
    impl From<Signature> for ed25519::Signature {
        fn from(outer: Signature) -> Self {
            outer.0
        }
    }
    impl AsRef<ed25519::Signature> for Signature {
        fn as_ref(&self) -> &ed25519::Signature {
            &self.0
        }
    }
    impl AsMut<ed25519::Signature> for Signature {
        fn as_mut(&mut self) -> &mut ed25519::Signature {
            &mut self.0
        }
    }
    impl ::sp_application_crypto::CryptoType for Signature {
        type Pair = Pair;
    }
    impl ::sp_application_crypto::AppKey for Signature {
        type UntypedGeneric = ed25519::Signature;
        type Public = Public;
        type Pair = Pair;
        type Signature = Signature;
        const ID: ::sp_application_crypto::KeyTypeId = GRANDPA;
        const CRYPTO_ID: ::sp_application_crypto::CryptoTypeId = ed25519::CRYPTO_ID;
    }
    impl ::sp_application_crypto::Deref for Signature {
        type Target = [u8];
        fn deref(&self) -> &Self::Target {
            self.0.as_ref()
        }
    }
    impl AsRef<[u8]> for Signature {
        fn as_ref(&self) -> &[u8] {
            self.0.as_ref()
        }
    }
    impl ::sp_application_crypto::AppSignature for Signature {
        type Generic = ed25519::Signature;
    }
    impl ::sp_application_crypto::TryFrom<::sp_application_crypto::Vec<u8>> for Signature {
        type Error = ();
        fn try_from(data: ::sp_application_crypto::Vec<u8>) -> Result<Self, Self::Error> {
            Ok(<ed25519::Signature>::try_from(data.as_slice())?.into())
        }
    }
    /// A generic `AppPublic` wrapper type over $pair crypto; this has no specific App.
    pub struct Pair(ed25519::Pair);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Pair {
        #[inline]
        fn clone(&self) -> Pair {
            match *self {
                Pair(ref __self_0_0) => Pair(::core::clone::Clone::clone(&(*__self_0_0))),
            }
        }
    }
    impl ::sp_application_crypto::Wraps for Pair {
        type Inner = ed25519::Pair;
    }
    impl From<ed25519::Pair> for Pair {
        fn from(inner: ed25519::Pair) -> Self {
            Self(inner)
        }
    }
    impl From<Pair> for ed25519::Pair {
        fn from(outer: Pair) -> Self {
            outer.0
        }
    }
    impl AsRef<ed25519::Pair> for Pair {
        fn as_ref(&self) -> &ed25519::Pair {
            &self.0
        }
    }
    impl AsMut<ed25519::Pair> for Pair {
        fn as_mut(&mut self) -> &mut ed25519::Pair {
            &mut self.0
        }
    }
    impl ::sp_application_crypto::CryptoType for Pair {
        type Pair = Pair;
    }
    impl ::sp_application_crypto::Pair for Pair {
        type Public = Public;
        type Seed = <ed25519::Pair as ::sp_application_crypto::Pair>::Seed;
        type Signature = Signature;
        type DeriveError = <ed25519::Pair as ::sp_application_crypto::Pair>::DeriveError;
        fn generate_with_phrase(password: Option<&str>) -> (Self, String, Self::Seed) {
            let r = <ed25519::Pair>::generate_with_phrase(password);
            (Self(r.0), r.1, r.2)
        }
        fn from_phrase(
            phrase: &str,
            password: Option<&str>,
        ) -> Result<(Self, Self::Seed), ::sp_application_crypto::SecretStringError> {
            <ed25519::Pair>::from_phrase(phrase, password).map(|r| (Self(r.0), r.1))
        }
        fn derive<Iter: Iterator<Item = ::sp_application_crypto::DeriveJunction>>(
            &self,
            path: Iter,
            seed: Option<Self::Seed>,
        ) -> Result<(Self, Option<Self::Seed>), Self::DeriveError> {
            self.0.derive(path, seed).map(|x| (Self(x.0), x.1))
        }
        fn from_seed(seed: &Self::Seed) -> Self {
            Self(<ed25519::Pair>::from_seed(seed))
        }
        fn from_seed_slice(
            seed: &[u8],
        ) -> Result<Self, ::sp_application_crypto::SecretStringError> {
            <ed25519::Pair>::from_seed_slice(seed).map(Self)
        }
        fn sign(&self, msg: &[u8]) -> Self::Signature {
            Signature(self.0.sign(msg))
        }
        fn verify<M: AsRef<[u8]>>(
            sig: &Self::Signature,
            message: M,
            pubkey: &Self::Public,
        ) -> bool {
            <ed25519::Pair>::verify(&sig.0, message, pubkey.as_ref())
        }
        fn verify_weak<P: AsRef<[u8]>, M: AsRef<[u8]>>(sig: &[u8], message: M, pubkey: P) -> bool {
            <ed25519::Pair>::verify_weak(sig, message, pubkey)
        }
        fn public(&self) -> Self::Public {
            Public(self.0.public())
        }
        fn to_raw_vec(&self) -> ::sp_application_crypto::Vec<u8> {
            self.0.to_raw_vec()
        }
    }
    impl ::sp_application_crypto::AppKey for Pair {
        type UntypedGeneric = ed25519::Pair;
        type Public = Public;
        type Pair = Pair;
        type Signature = Signature;
        const ID: ::sp_application_crypto::KeyTypeId = GRANDPA;
        const CRYPTO_ID: ::sp_application_crypto::CryptoTypeId = ed25519::CRYPTO_ID;
    }
    impl ::sp_application_crypto::AppPair for Pair {
        type Generic = ed25519::Pair;
    }
}
/// The grandpa crypto scheme defined via the keypair type.
pub type AuthorityPair = app::Pair;
/// Identity of a Grandpa authority.
pub type AuthorityId = app::Public;
/// Signature for a Grandpa authority.
pub type AuthoritySignature = app::Signature;
/// The `ConsensusEngineId` of GRANDPA.
pub const GRANDPA_ENGINE_ID: ConsensusEngineId = *b"FRNK";
/// The storage key for the current set of weighted Grandpa authorities.
/// The value stored is an encoded VersionedAuthorityList.
pub const GRANDPA_AUTHORITIES_KEY: &'static [u8] = b":grandpa_authorities";
/// The weight of an authority.
pub type AuthorityWeight = u64;
/// The index of an authority.
pub type AuthorityIndex = u64;
/// The monotonic identifier of a GRANDPA set of authorities.
pub type SetId = u64;
/// The round indicator.
pub type RoundNumber = u64;
/// A list of Grandpa authorities with associated weights.
pub type AuthorityList = Vec<(AuthorityId, AuthorityWeight)>;
/// A scheduled change of authority set.
pub struct ScheduledChange<N> {
    /// The new authorities after the change, along with their respective weights.
    pub next_authorities: AuthorityList,
    /// The number of blocks to delay.
    pub delay: N,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<N: ::core::clone::Clone> ::core::clone::Clone for ScheduledChange<N> {
    #[inline]
    fn clone(&self) -> ScheduledChange<N> {
        match *self {
            ScheduledChange {
                next_authorities: ref __self_0_0,
                delay: ref __self_0_1,
            } => ScheduledChange {
                next_authorities: ::core::clone::Clone::clone(&(*__self_0_0)),
                delay: ::core::clone::Clone::clone(&(*__self_0_1)),
            },
        }
    }
}
impl<N> ::core::marker::StructuralEq for ScheduledChange<N> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<N: ::core::cmp::Eq> ::core::cmp::Eq for ScheduledChange<N> {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<AuthorityList>;
            let _: ::core::cmp::AssertParamIsEq<N>;
        }
    }
}
impl<N> ::core::marker::StructuralPartialEq for ScheduledChange<N> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<N: ::core::cmp::PartialEq> ::core::cmp::PartialEq for ScheduledChange<N> {
    #[inline]
    fn eq(&self, other: &ScheduledChange<N>) -> bool {
        match *other {
            ScheduledChange {
                next_authorities: ref __self_1_0,
                delay: ref __self_1_1,
            } => match *self {
                ScheduledChange {
                    next_authorities: ref __self_0_0,
                    delay: ref __self_0_1,
                } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &ScheduledChange<N>) -> bool {
        match *other {
            ScheduledChange {
                next_authorities: ref __self_1_0,
                delay: ref __self_1_1,
            } => match *self {
                ScheduledChange {
                    next_authorities: ref __self_0_0,
                    delay: ref __self_0_1,
                } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
            },
        }
    }
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<N> _parity_scale_codec::Encode for ScheduledChange<N>
    where
        N: _parity_scale_codec::Encode,
        N: _parity_scale_codec::Encode,
    {
        fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            _parity_scale_codec::Encode::encode_to(&self.next_authorities, __codec_dest_edqy);
            _parity_scale_codec::Encode::encode_to(&self.delay, __codec_dest_edqy);
        }
    }
    impl<N> _parity_scale_codec::EncodeLike for ScheduledChange<N>
    where
        N: _parity_scale_codec::Encode,
        N: _parity_scale_codec::Encode,
    {
    }
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<N> _parity_scale_codec::Decode for ScheduledChange<N>
    where
        N: _parity_scale_codec::Decode,
        N: _parity_scale_codec::Decode,
    {
        fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, _parity_scale_codec::Error> {
            ::core::result::Result::Ok(ScheduledChange::<N> {
                next_authorities: {
                    let __codec_res_edqy =
                        <AuthorityList as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                    match __codec_res_edqy {
                        ::core::result::Result::Err(e) => {
                            return ::core::result::Result::Err(
                                e.chain("Could not decode `ScheduledChange::next_authorities`"),
                            )
                        }
                        ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                },
                delay: {
                    let __codec_res_edqy =
                        <N as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                    match __codec_res_edqy {
                        ::core::result::Result::Err(e) => {
                            return ::core::result::Result::Err(
                                e.chain("Could not decode `ScheduledChange::delay`"),
                            )
                        }
                        ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                },
            })
        }
    }
};
impl<N> core::fmt::Debug for ScheduledChange<N>
where
    N: core::fmt::Debug,
{
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_struct("ScheduledChange")
            .field("next_authorities", &self.next_authorities)
            .field("delay", &self.delay)
            .finish()
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<N> _serde::Serialize for ScheduledChange<N>
    where
        N: _serde::Serialize,
    {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "ScheduledChange",
                false as usize + 1 + 1,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "next_authorities",
                &self.next_authorities,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "delay",
                &self.delay,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
/// An consensus log item for GRANDPA.
pub enum ConsensusLog<N: Codec> {
    /// Schedule an authority set change.
    ///
    /// The earliest digest of this type in a single block will be respected,
    /// provided that there is no `ForcedChange` digest. If there is, then the
    /// `ForcedChange` will take precedence.
    ///
    /// No change should be scheduled if one is already and the delay has not
    /// passed completely.
    ///
    /// This should be a pure function: i.e. as long as the runtime can interpret
    /// the digest type it should return the same result regardless of the current
    /// state.
    #[codec(index = 1)]
    ScheduledChange(ScheduledChange<N>),
    /// Force an authority set change.
    ///
    /// Forced changes are applied after a delay of _imported_ blocks,
    /// while pending changes are applied after a delay of _finalized_ blocks.
    ///
    /// The earliest digest of this type in a single block will be respected,
    /// with others ignored.
    ///
    /// No change should be scheduled if one is already and the delay has not
    /// passed completely.
    ///
    /// This should be a pure function: i.e. as long as the runtime can interpret
    /// the digest type it should return the same result regardless of the current
    /// state.
    #[codec(index = 2)]
    ForcedChange(N, ScheduledChange<N>),
    /// Note that the authority with given index is disabled until the next change.
    #[codec(index = 3)]
    OnDisabled(AuthorityIndex),
    /// A signal to pause the current authority set after the given delay.
    /// After finalizing the block at _delay_ the authorities should stop voting.
    #[codec(index = 4)]
    Pause(N),
    /// A signal to resume the current authority set after the given delay.
    /// After authoring the block at _delay_ the authorities should resume voting.
    #[codec(index = 5)]
    Resume(N),
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<N: Codec> _parity_scale_codec::Decode for ConsensusLog<N>
    where
        ScheduledChange<N>: _parity_scale_codec::Decode,
        ScheduledChange<N>: _parity_scale_codec::Decode,
        N: _parity_scale_codec::Decode,
        N: _parity_scale_codec::Decode,
        ScheduledChange<N>: _parity_scale_codec::Decode,
        ScheduledChange<N>: _parity_scale_codec::Decode,
        N: _parity_scale_codec::Decode,
        N: _parity_scale_codec::Decode,
        N: _parity_scale_codec::Decode,
        N: _parity_scale_codec::Decode,
    {
        fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, _parity_scale_codec::Error> {
            match __codec_input_edqy.read_byte().map_err(|e| {
                e.chain("Could not decode `ConsensusLog`, failed to read variant byte")
            })? {
                __codec_x_edqy if __codec_x_edqy == 1u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(ConsensusLog::<N>::ScheduledChange({
                        let __codec_res_edqy =
                            <ScheduledChange<N> as _parity_scale_codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `ConsensusLog::ScheduledChange.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 2u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(ConsensusLog::<N>::ForcedChange(
                        {
                            let __codec_res_edqy =
                                <N as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `ConsensusLog::ForcedChange.0`"),
                                    )
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                        {
                            let __codec_res_edqy =
                                <ScheduledChange<N> as _parity_scale_codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `ConsensusLog::ForcedChange.1`"),
                                    )
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                    ))
                }
                __codec_x_edqy if __codec_x_edqy == 3u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(ConsensusLog::<N>::OnDisabled({
                        let __codec_res_edqy =
                            <AuthorityIndex as _parity_scale_codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `ConsensusLog::OnDisabled.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 4u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(ConsensusLog::<N>::Pause({
                        let __codec_res_edqy =
                            <N as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `ConsensusLog::Pause.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 5u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(ConsensusLog::<N>::Resume({
                        let __codec_res_edqy =
                            <N as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `ConsensusLog::Resume.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                _ => ::core::result::Result::Err(
                    "Could not decode `ConsensusLog`, variant doesn\'t exist".into(),
                ),
            }
        }
    }
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<N: Codec> _parity_scale_codec::Encode for ConsensusLog<N>
    where
        ScheduledChange<N>: _parity_scale_codec::Encode,
        ScheduledChange<N>: _parity_scale_codec::Encode,
        N: _parity_scale_codec::Encode,
        N: _parity_scale_codec::Encode,
        ScheduledChange<N>: _parity_scale_codec::Encode,
        ScheduledChange<N>: _parity_scale_codec::Encode,
        N: _parity_scale_codec::Encode,
        N: _parity_scale_codec::Encode,
        N: _parity_scale_codec::Encode,
        N: _parity_scale_codec::Encode,
    {
        fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            match *self {
                ConsensusLog::ScheduledChange(ref aa) => {
                    __codec_dest_edqy.push_byte(1u8 as ::core::primitive::u8);
                    _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                ConsensusLog::ForcedChange(ref aa, ref ba) => {
                    __codec_dest_edqy.push_byte(2u8 as ::core::primitive::u8);
                    _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                    _parity_scale_codec::Encode::encode_to(ba, __codec_dest_edqy);
                }
                ConsensusLog::OnDisabled(ref aa) => {
                    __codec_dest_edqy.push_byte(3u8 as ::core::primitive::u8);
                    _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                ConsensusLog::Pause(ref aa) => {
                    __codec_dest_edqy.push_byte(4u8 as ::core::primitive::u8);
                    _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                ConsensusLog::Resume(ref aa) => {
                    __codec_dest_edqy.push_byte(5u8 as ::core::primitive::u8);
                    _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                _ => (),
            }
        }
    }
    impl<N: Codec> _parity_scale_codec::EncodeLike for ConsensusLog<N>
    where
        ScheduledChange<N>: _parity_scale_codec::Encode,
        ScheduledChange<N>: _parity_scale_codec::Encode,
        N: _parity_scale_codec::Encode,
        N: _parity_scale_codec::Encode,
        ScheduledChange<N>: _parity_scale_codec::Encode,
        ScheduledChange<N>: _parity_scale_codec::Encode,
        N: _parity_scale_codec::Encode,
        N: _parity_scale_codec::Encode,
        N: _parity_scale_codec::Encode,
        N: _parity_scale_codec::Encode,
    {
    }
};
impl<N: Codec> ::core::marker::StructuralPartialEq for ConsensusLog<N> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<N: ::core::cmp::PartialEq + Codec> ::core::cmp::PartialEq for ConsensusLog<N> {
    #[inline]
    fn eq(&self, other: &ConsensusLog<N>) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (
                        &ConsensusLog::ScheduledChange(ref __self_0),
                        &ConsensusLog::ScheduledChange(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    (
                        &ConsensusLog::ForcedChange(ref __self_0, ref __self_1),
                        &ConsensusLog::ForcedChange(ref __arg_1_0, ref __arg_1_1),
                    ) => (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1),
                    (
                        &ConsensusLog::OnDisabled(ref __self_0),
                        &ConsensusLog::OnDisabled(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    (&ConsensusLog::Pause(ref __self_0), &ConsensusLog::Pause(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&ConsensusLog::Resume(ref __self_0), &ConsensusLog::Resume(ref __arg_1_0)) => {
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
    fn ne(&self, other: &ConsensusLog<N>) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (
                        &ConsensusLog::ScheduledChange(ref __self_0),
                        &ConsensusLog::ScheduledChange(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    (
                        &ConsensusLog::ForcedChange(ref __self_0, ref __self_1),
                        &ConsensusLog::ForcedChange(ref __arg_1_0, ref __arg_1_1),
                    ) => (*__self_0) != (*__arg_1_0) || (*__self_1) != (*__arg_1_1),
                    (
                        &ConsensusLog::OnDisabled(ref __self_0),
                        &ConsensusLog::OnDisabled(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    (&ConsensusLog::Pause(ref __self_0), &ConsensusLog::Pause(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&ConsensusLog::Resume(ref __self_0), &ConsensusLog::Resume(ref __arg_1_0)) => {
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
impl<N: Codec> ::core::marker::StructuralEq for ConsensusLog<N> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<N: ::core::cmp::Eq + Codec> ::core::cmp::Eq for ConsensusLog<N> {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<ScheduledChange<N>>;
            let _: ::core::cmp::AssertParamIsEq<N>;
            let _: ::core::cmp::AssertParamIsEq<ScheduledChange<N>>;
            let _: ::core::cmp::AssertParamIsEq<AuthorityIndex>;
            let _: ::core::cmp::AssertParamIsEq<N>;
            let _: ::core::cmp::AssertParamIsEq<N>;
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<N: ::core::clone::Clone + Codec> ::core::clone::Clone for ConsensusLog<N> {
    #[inline]
    fn clone(&self) -> ConsensusLog<N> {
        match (&*self,) {
            (&ConsensusLog::ScheduledChange(ref __self_0),) => {
                ConsensusLog::ScheduledChange(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&ConsensusLog::ForcedChange(ref __self_0, ref __self_1),) => {
                ConsensusLog::ForcedChange(
                    ::core::clone::Clone::clone(&(*__self_0)),
                    ::core::clone::Clone::clone(&(*__self_1)),
                )
            }
            (&ConsensusLog::OnDisabled(ref __self_0),) => {
                ConsensusLog::OnDisabled(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&ConsensusLog::Pause(ref __self_0),) => {
                ConsensusLog::Pause(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&ConsensusLog::Resume(ref __self_0),) => {
                ConsensusLog::Resume(::core::clone::Clone::clone(&(*__self_0)))
            }
        }
    }
}
impl<N: Codec> core::fmt::Debug for ConsensusLog<N>
where
    N: core::fmt::Debug,
{
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ScheduledChange(ref a0) => fmt
                .debug_tuple("ConsensusLog::ScheduledChange")
                .field(a0)
                .finish(),
            Self::ForcedChange(ref a0, ref a1) => fmt
                .debug_tuple("ConsensusLog::ForcedChange")
                .field(a0)
                .field(a1)
                .finish(),
            Self::OnDisabled(ref a0) => fmt
                .debug_tuple("ConsensusLog::OnDisabled")
                .field(a0)
                .finish(),
            Self::Pause(ref a0) => fmt.debug_tuple("ConsensusLog::Pause").field(a0).finish(),
            Self::Resume(ref a0) => fmt.debug_tuple("ConsensusLog::Resume").field(a0).finish(),
            _ => Ok(()),
        }
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<N: Codec> _serde::Serialize for ConsensusLog<N>
    where
        N: _serde::Serialize,
    {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                ConsensusLog::ScheduledChange(ref __field0) => {
                    _serde::Serializer::serialize_newtype_variant(
                        __serializer,
                        "ConsensusLog",
                        0u32,
                        "ScheduledChange",
                        __field0,
                    )
                }
                ConsensusLog::ForcedChange(ref __field0, ref __field1) => {
                    let mut __serde_state = match _serde::Serializer::serialize_tuple_variant(
                        __serializer,
                        "ConsensusLog",
                        1u32,
                        "ForcedChange",
                        0 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeTupleVariant::serialize_field(
                        &mut __serde_state,
                        __field0,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeTupleVariant::serialize_field(
                        &mut __serde_state,
                        __field1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeTupleVariant::end(__serde_state)
                }
                ConsensusLog::OnDisabled(ref __field0) => {
                    _serde::Serializer::serialize_newtype_variant(
                        __serializer,
                        "ConsensusLog",
                        2u32,
                        "OnDisabled",
                        __field0,
                    )
                }
                ConsensusLog::Pause(ref __field0) => _serde::Serializer::serialize_newtype_variant(
                    __serializer,
                    "ConsensusLog",
                    3u32,
                    "Pause",
                    __field0,
                ),
                ConsensusLog::Resume(ref __field0) => {
                    _serde::Serializer::serialize_newtype_variant(
                        __serializer,
                        "ConsensusLog",
                        4u32,
                        "Resume",
                        __field0,
                    )
                }
            }
        }
    }
};
impl<N: Codec> ConsensusLog<N> {
    /// Try to cast the log entry as a contained signal.
    pub fn try_into_change(self) -> Option<ScheduledChange<N>> {
        match self {
            ConsensusLog::ScheduledChange(change) => Some(change),
            _ => None,
        }
    }
    /// Try to cast the log entry as a contained forced signal.
    pub fn try_into_forced_change(self) -> Option<(N, ScheduledChange<N>)> {
        match self {
            ConsensusLog::ForcedChange(median, change) => Some((median, change)),
            _ => None,
        }
    }
    /// Try to cast the log entry as a contained pause signal.
    pub fn try_into_pause(self) -> Option<N> {
        match self {
            ConsensusLog::Pause(delay) => Some(delay),
            _ => None,
        }
    }
    /// Try to cast the log entry as a contained resume signal.
    pub fn try_into_resume(self) -> Option<N> {
        match self {
            ConsensusLog::Resume(delay) => Some(delay),
            _ => None,
        }
    }
}
/// Proof of voter misbehavior on a given set id. Misbehavior/equivocation in
/// GRANDPA happens when a voter votes on the same round (either at prevote or
/// precommit stage) for different blocks. Proving is achieved by collecting the
/// signed messages of conflicting votes.
pub struct EquivocationProof<H, N> {
    set_id: SetId,
    equivocation: Equivocation<H, N>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<H: ::core::clone::Clone, N: ::core::clone::Clone> ::core::clone::Clone
    for EquivocationProof<H, N>
{
    #[inline]
    fn clone(&self) -> EquivocationProof<H, N> {
        match *self {
            EquivocationProof {
                set_id: ref __self_0_0,
                equivocation: ref __self_0_1,
            } => EquivocationProof {
                set_id: ::core::clone::Clone::clone(&(*__self_0_0)),
                equivocation: ::core::clone::Clone::clone(&(*__self_0_1)),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<H: ::core::fmt::Debug, N: ::core::fmt::Debug> ::core::fmt::Debug for EquivocationProof<H, N> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            EquivocationProof {
                set_id: ref __self_0_0,
                equivocation: ref __self_0_1,
            } => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_struct(f, "EquivocationProof");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "set_id", &&(*__self_0_0));
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "equivocation",
                    &&(*__self_0_1),
                );
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<H, N> _parity_scale_codec::Decode for EquivocationProof<H, N>
    where
        Equivocation<H, N>: _parity_scale_codec::Decode,
        Equivocation<H, N>: _parity_scale_codec::Decode,
    {
        fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, _parity_scale_codec::Error> {
            ::core::result::Result::Ok(EquivocationProof::<H, N> {
                set_id: {
                    let __codec_res_edqy =
                        <SetId as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                    match __codec_res_edqy {
                        ::core::result::Result::Err(e) => {
                            return ::core::result::Result::Err(
                                e.chain("Could not decode `EquivocationProof::set_id`"),
                            )
                        }
                        ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                },
                equivocation: {
                    let __codec_res_edqy =
                        <Equivocation<H, N> as _parity_scale_codec::Decode>::decode(
                            __codec_input_edqy,
                        );
                    match __codec_res_edqy {
                        ::core::result::Result::Err(e) => {
                            return ::core::result::Result::Err(
                                e.chain("Could not decode `EquivocationProof::equivocation`"),
                            )
                        }
                        ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                },
            })
        }
    }
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<H, N> _parity_scale_codec::Encode for EquivocationProof<H, N>
    where
        Equivocation<H, N>: _parity_scale_codec::Encode,
        Equivocation<H, N>: _parity_scale_codec::Encode,
    {
        fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            _parity_scale_codec::Encode::encode_to(&self.set_id, __codec_dest_edqy);
            _parity_scale_codec::Encode::encode_to(&self.equivocation, __codec_dest_edqy);
        }
    }
    impl<H, N> _parity_scale_codec::EncodeLike for EquivocationProof<H, N>
    where
        Equivocation<H, N>: _parity_scale_codec::Encode,
        Equivocation<H, N>: _parity_scale_codec::Encode,
    {
    }
};
impl<H, N> ::core::marker::StructuralPartialEq for EquivocationProof<H, N> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<H: ::core::cmp::PartialEq, N: ::core::cmp::PartialEq> ::core::cmp::PartialEq
    for EquivocationProof<H, N>
{
    #[inline]
    fn eq(&self, other: &EquivocationProof<H, N>) -> bool {
        match *other {
            EquivocationProof {
                set_id: ref __self_1_0,
                equivocation: ref __self_1_1,
            } => match *self {
                EquivocationProof {
                    set_id: ref __self_0_0,
                    equivocation: ref __self_0_1,
                } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &EquivocationProof<H, N>) -> bool {
        match *other {
            EquivocationProof {
                set_id: ref __self_1_0,
                equivocation: ref __self_1_1,
            } => match *self {
                EquivocationProof {
                    set_id: ref __self_0_0,
                    equivocation: ref __self_0_1,
                } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
            },
        }
    }
}
impl<H, N> EquivocationProof<H, N> {
    /// Create a new `EquivocationProof` for the given set id and using the
    /// given equivocation as proof.
    pub fn new(set_id: SetId, equivocation: Equivocation<H, N>) -> Self {
        EquivocationProof {
            set_id,
            equivocation,
        }
    }
    /// Returns the set id at which the equivocation occurred.
    pub fn set_id(&self) -> SetId {
        self.set_id
    }
    /// Returns the round number at which the equivocation occurred.
    pub fn round(&self) -> RoundNumber {
        match self.equivocation {
            Equivocation::Prevote(ref equivocation) => equivocation.round_number,
            Equivocation::Precommit(ref equivocation) => equivocation.round_number,
        }
    }
    /// Returns the authority id of the equivocator.
    pub fn offender(&self) -> &AuthorityId {
        self.equivocation.offender()
    }
}
/// Wrapper object for GRANDPA equivocation proofs, useful for unifying prevote
/// and precommit equivocations under a common type.
pub enum Equivocation<H, N> {
    /// Proof of equivocation at prevote stage.
    Prevote(grandpa::Equivocation<AuthorityId, grandpa::Prevote<H, N>, AuthoritySignature>),
    /// Proof of equivocation at precommit stage.
    Precommit(grandpa::Equivocation<AuthorityId, grandpa::Precommit<H, N>, AuthoritySignature>),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<H: ::core::clone::Clone, N: ::core::clone::Clone> ::core::clone::Clone for Equivocation<H, N> {
    #[inline]
    fn clone(&self) -> Equivocation<H, N> {
        match (&*self,) {
            (&Equivocation::Prevote(ref __self_0),) => {
                Equivocation::Prevote(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Equivocation::Precommit(ref __self_0),) => {
                Equivocation::Precommit(::core::clone::Clone::clone(&(*__self_0)))
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<H: ::core::fmt::Debug, N: ::core::fmt::Debug> ::core::fmt::Debug for Equivocation<H, N> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&Equivocation::Prevote(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Prevote");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&Equivocation::Precommit(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Precommit");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
        }
    }
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<H, N> _parity_scale_codec::Decode for Equivocation<H, N>
    where
        grandpa::Equivocation<AuthorityId, grandpa::Prevote<H, N>, AuthoritySignature>:
            _parity_scale_codec::Decode,
        grandpa::Equivocation<AuthorityId, grandpa::Prevote<H, N>, AuthoritySignature>:
            _parity_scale_codec::Decode,
        grandpa::Equivocation<AuthorityId, grandpa::Precommit<H, N>, AuthoritySignature>:
            _parity_scale_codec::Decode,
        grandpa::Equivocation<AuthorityId, grandpa::Precommit<H, N>, AuthoritySignature>:
            _parity_scale_codec::Decode,
    {
        fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, _parity_scale_codec::Error> {
            match __codec_input_edqy.read_byte().map_err(|e| {
                e.chain("Could not decode `Equivocation`, failed to read variant byte")
            })? {
                __codec_x_edqy if __codec_x_edqy == 0usize as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Equivocation::<H, N>::Prevote({
                        let __codec_res_edqy = <grandpa::Equivocation<
                            AuthorityId,
                            grandpa::Prevote<H, N>,
                            AuthoritySignature,
                        > as _parity_scale_codec::Decode>::decode(
                            __codec_input_edqy
                        );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Equivocation::Prevote.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 1usize as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Equivocation::<H, N>::Precommit({
                        let __codec_res_edqy = <grandpa::Equivocation<
                            AuthorityId,
                            grandpa::Precommit<H, N>,
                            AuthoritySignature,
                        > as _parity_scale_codec::Decode>::decode(
                            __codec_input_edqy
                        );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Equivocation::Precommit.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                _ => ::core::result::Result::Err(
                    "Could not decode `Equivocation`, variant doesn\'t exist".into(),
                ),
            }
        }
    }
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<H, N> _parity_scale_codec::Encode for Equivocation<H, N>
    where
        grandpa::Equivocation<AuthorityId, grandpa::Prevote<H, N>, AuthoritySignature>:
            _parity_scale_codec::Encode,
        grandpa::Equivocation<AuthorityId, grandpa::Prevote<H, N>, AuthoritySignature>:
            _parity_scale_codec::Encode,
        grandpa::Equivocation<AuthorityId, grandpa::Precommit<H, N>, AuthoritySignature>:
            _parity_scale_codec::Encode,
        grandpa::Equivocation<AuthorityId, grandpa::Precommit<H, N>, AuthoritySignature>:
            _parity_scale_codec::Encode,
    {
        fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            match *self {
                Equivocation::Prevote(ref aa) => {
                    __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                    _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Equivocation::Precommit(ref aa) => {
                    __codec_dest_edqy.push_byte(1usize as ::core::primitive::u8);
                    _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                _ => (),
            }
        }
    }
    impl<H, N> _parity_scale_codec::EncodeLike for Equivocation<H, N>
    where
        grandpa::Equivocation<AuthorityId, grandpa::Prevote<H, N>, AuthoritySignature>:
            _parity_scale_codec::Encode,
        grandpa::Equivocation<AuthorityId, grandpa::Prevote<H, N>, AuthoritySignature>:
            _parity_scale_codec::Encode,
        grandpa::Equivocation<AuthorityId, grandpa::Precommit<H, N>, AuthoritySignature>:
            _parity_scale_codec::Encode,
        grandpa::Equivocation<AuthorityId, grandpa::Precommit<H, N>, AuthoritySignature>:
            _parity_scale_codec::Encode,
    {
    }
};
impl<H, N> ::core::marker::StructuralPartialEq for Equivocation<H, N> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<H: ::core::cmp::PartialEq, N: ::core::cmp::PartialEq> ::core::cmp::PartialEq
    for Equivocation<H, N>
{
    #[inline]
    fn eq(&self, other: &Equivocation<H, N>) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (
                        &Equivocation::Prevote(ref __self_0),
                        &Equivocation::Prevote(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    (
                        &Equivocation::Precommit(ref __self_0),
                        &Equivocation::Precommit(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &Equivocation<H, N>) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (
                        &Equivocation::Prevote(ref __self_0),
                        &Equivocation::Prevote(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    (
                        &Equivocation::Precommit(ref __self_0),
                        &Equivocation::Precommit(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                true
            }
        }
    }
}
impl<H, N> From<grandpa::Equivocation<AuthorityId, grandpa::Prevote<H, N>, AuthoritySignature>>
    for Equivocation<H, N>
{
    fn from(
        equivocation: grandpa::Equivocation<
            AuthorityId,
            grandpa::Prevote<H, N>,
            AuthoritySignature,
        >,
    ) -> Self {
        Equivocation::Prevote(equivocation)
    }
}
impl<H, N> From<grandpa::Equivocation<AuthorityId, grandpa::Precommit<H, N>, AuthoritySignature>>
    for Equivocation<H, N>
{
    fn from(
        equivocation: grandpa::Equivocation<
            AuthorityId,
            grandpa::Precommit<H, N>,
            AuthoritySignature,
        >,
    ) -> Self {
        Equivocation::Precommit(equivocation)
    }
}
impl<H, N> Equivocation<H, N> {
    /// Returns the authority id of the equivocator.
    pub fn offender(&self) -> &AuthorityId {
        match self {
            Equivocation::Prevote(ref equivocation) => &equivocation.identity,
            Equivocation::Precommit(ref equivocation) => &equivocation.identity,
        }
    }
    /// Returns the round number when the equivocation happened.
    pub fn round_number(&self) -> RoundNumber {
        match self {
            Equivocation::Prevote(ref equivocation) => equivocation.round_number,
            Equivocation::Precommit(ref equivocation) => equivocation.round_number,
        }
    }
}
/// Verifies the equivocation proof by making sure that both votes target
/// different blocks and that its signatures are valid.
pub fn check_equivocation_proof<H, N>(report: EquivocationProof<H, N>) -> bool
where
    H: Clone + Encode + PartialEq,
    N: Clone + Encode + PartialEq,
{
    match report.equivocation {
        Equivocation::Prevote(equivocation) => {
            if equivocation.first.0.target_hash == equivocation.second.0.target_hash
                && equivocation.first.0.target_number == equivocation.second.0.target_number
            {
                return false;
            }
            let valid_first = check_message_signature(
                &grandpa::Message::Prevote(equivocation.first.0),
                &equivocation.identity,
                &equivocation.first.1,
                equivocation.round_number,
                report.set_id,
            );
            let valid_second = check_message_signature(
                &grandpa::Message::Prevote(equivocation.second.0),
                &equivocation.identity,
                &equivocation.second.1,
                equivocation.round_number,
                report.set_id,
            );
            return valid_first && valid_second;
        }
        Equivocation::Precommit(equivocation) => {
            if equivocation.first.0.target_hash == equivocation.second.0.target_hash
                && equivocation.first.0.target_number == equivocation.second.0.target_number
            {
                return false;
            }
            let valid_first = check_message_signature(
                &grandpa::Message::Precommit(equivocation.first.0),
                &equivocation.identity,
                &equivocation.first.1,
                equivocation.round_number,
                report.set_id,
            );
            let valid_second = check_message_signature(
                &grandpa::Message::Precommit(equivocation.second.0),
                &equivocation.identity,
                &equivocation.second.1,
                equivocation.round_number,
                report.set_id,
            );
            return valid_first && valid_second;
        }
    }
}
/// Encode round message localized to a given round and set id.
pub fn localized_payload<E: Encode>(round: RoundNumber, set_id: SetId, message: &E) -> Vec<u8> {
    let mut buf = Vec::new();
    localized_payload_with_buffer(round, set_id, message, &mut buf);
    buf
}
/// Encode round message localized to a given round and set id using the given
/// buffer. The given buffer will be cleared and the resulting encoded payload
/// will always be written to the start of the buffer.
pub fn localized_payload_with_buffer<E: Encode>(
    round: RoundNumber,
    set_id: SetId,
    message: &E,
    buf: &mut Vec<u8>,
) {
    buf.clear();
    (message, round, set_id).encode_to(buf)
}
/// Check a message signature by encoding the message as a localized payload and
/// verifying the provided signature using the expected authority id.
pub fn check_message_signature<H, N>(
    message: &grandpa::Message<H, N>,
    id: &AuthorityId,
    signature: &AuthoritySignature,
    round: RoundNumber,
    set_id: SetId,
) -> bool
where
    H: Encode,
    N: Encode,
{
    check_message_signature_with_buffer(message, id, signature, round, set_id, &mut Vec::new())
}
/// Check a message signature by encoding the message as a localized payload and
/// verifying the provided signature using the expected authority id.
/// The encoding necessary to verify the signature will be done using the given
/// buffer, the original content of the buffer will be cleared.
pub fn check_message_signature_with_buffer<H, N>(
    message: &grandpa::Message<H, N>,
    id: &AuthorityId,
    signature: &AuthoritySignature,
    round: RoundNumber,
    set_id: SetId,
    buf: &mut Vec<u8>,
) -> bool
where
    H: Encode,
    N: Encode,
{
    use sp_application_crypto::RuntimeAppPublic;
    localized_payload_with_buffer(round, set_id, message, buf);
    let valid = id.verify(&buf, signature);
    if !valid {
        {
            let lvl = ::log::Level::Debug;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                :: log :: __private_api_log (:: core :: fmt :: Arguments :: new_v1 (& ["Bad signature on message from "] , & match (& id ,) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Debug :: fmt)] , }) , lvl , & ("afg" , "sp_finality_grandpa" , "/Users/mmac/.cargo/git/checkouts/substrate-7e08433d4c370a21/4d28ebe/primitives/finality-grandpa/src/lib.rs" , 370u32)) ;
            }
        };
    }
    valid
}
/// Localizes the message to the given set and round and signs the payload.
#[cfg(feature = "std")]
pub fn sign_message<H, N>(
    keystore: SyncCryptoStorePtr,
    message: grandpa::Message<H, N>,
    public: AuthorityId,
    round: RoundNumber,
    set_id: SetId,
) -> Option<grandpa::SignedMessage<H, N, AuthoritySignature, AuthorityId>>
where
    H: Encode,
    N: Encode,
{
    use sp_application_crypto::AppKey;
    use sp_core::crypto::Public;
    use sp_std::convert::TryInto;
    let encoded = localized_payload(round, set_id, &message);
    let signature = SyncCryptoStore::sign_with(
        &*keystore,
        AuthorityId::ID,
        &public.to_public_crypto_pair(),
        &encoded[..],
    )
    .ok()
    .flatten()?
    .try_into()
    .ok()?;
    Some(grandpa::SignedMessage {
        message,
        signature,
        id: public,
    })
}
/// WASM function call to check for pending changes.
pub const PENDING_CHANGE_CALL: &str = "grandpa_pending_change";
/// WASM function call to get current GRANDPA authorities.
pub const AUTHORITIES_CALL: &str = "grandpa_authorities";
/// The current version of the stored AuthorityList type. The encoding version MUST be updated any
/// time the AuthorityList type changes.
const AUTHORITIES_VERSION: u8 = 1;
/// An AuthorityList that is encoded with a version specifier. The encoding version is updated any
/// time the AuthorityList type changes. This ensures that encodings of different versions of an
/// AuthorityList are differentiable. Attempting to decode an authority list with an unknown
/// version will fail.
pub struct VersionedAuthorityList<'a>(Cow<'a, AuthorityList>);
#[automatically_derived]
#[allow(unused_qualifications)]
impl<'a> ::core::default::Default for VersionedAuthorityList<'a> {
    #[inline]
    fn default() -> VersionedAuthorityList<'a> {
        VersionedAuthorityList(::core::default::Default::default())
    }
}
impl<'a> From<AuthorityList> for VersionedAuthorityList<'a> {
    fn from(authorities: AuthorityList) -> Self {
        VersionedAuthorityList(Cow::Owned(authorities))
    }
}
impl<'a> From<&'a AuthorityList> for VersionedAuthorityList<'a> {
    fn from(authorities: &'a AuthorityList) -> Self {
        VersionedAuthorityList(Cow::Borrowed(authorities))
    }
}
impl<'a> Into<AuthorityList> for VersionedAuthorityList<'a> {
    fn into(self) -> AuthorityList {
        self.0.into_owned()
    }
}
impl<'a> Encode for VersionedAuthorityList<'a> {
    fn size_hint(&self) -> usize {
        (AUTHORITIES_VERSION, self.0.as_ref()).size_hint()
    }
    fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R {
        (AUTHORITIES_VERSION, self.0.as_ref()).using_encoded(f)
    }
}
impl<'a> Decode for VersionedAuthorityList<'a> {
    fn decode<I: Input>(value: &mut I) -> Result<Self, codec::Error> {
        let (version, authorities): (u8, AuthorityList) = Decode::decode(value)?;
        if version != AUTHORITIES_VERSION {
            return Err("unknown Grandpa authorities version".into());
        }
        Ok(authorities.into())
    }
}
/// An opaque type used to represent the key ownership proof at the runtime API
/// boundary. The inner value is an encoded representation of the actual key
/// ownership proof which will be parameterized when defining the runtime. At
/// the runtime API boundary this type is unknown and as such we keep this
/// opaque representation, implementors of the runtime API will have to make
/// sure that all usages of `OpaqueKeyOwnershipProof` refer to the same type.
pub struct OpaqueKeyOwnershipProof(Vec<u8>);
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Decode for OpaqueKeyOwnershipProof {
        fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, _parity_scale_codec::Error> {
            ::core::result::Result::Ok(OpaqueKeyOwnershipProof({
                let __codec_res_edqy =
                    <Vec<u8> as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                match __codec_res_edqy {
                    ::core::result::Result::Err(e) => {
                        return ::core::result::Result::Err(
                            e.chain("Could not decode `OpaqueKeyOwnershipProof.0`"),
                        )
                    }
                    ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                }
            }))
        }
    }
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Encode for OpaqueKeyOwnershipProof {
        fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            _parity_scale_codec::Encode::encode_to(&&self.0, __codec_dest_edqy)
        }
        fn encode(&self) -> _parity_scale_codec::alloc::vec::Vec<u8> {
            _parity_scale_codec::Encode::encode(&&self.0)
        }
        fn using_encoded<R, F: ::core::ops::FnOnce(&[::core::primitive::u8]) -> R>(
            &self,
            f: F,
        ) -> R {
            _parity_scale_codec::Encode::using_encoded(&&self.0, f)
        }
    }
    impl _parity_scale_codec::EncodeLike for OpaqueKeyOwnershipProof {}
};
impl ::core::marker::StructuralPartialEq for OpaqueKeyOwnershipProof {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for OpaqueKeyOwnershipProof {
    #[inline]
    fn eq(&self, other: &OpaqueKeyOwnershipProof) -> bool {
        match *other {
            OpaqueKeyOwnershipProof(ref __self_1_0) => match *self {
                OpaqueKeyOwnershipProof(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &OpaqueKeyOwnershipProof) -> bool {
        match *other {
            OpaqueKeyOwnershipProof(ref __self_1_0) => match *self {
                OpaqueKeyOwnershipProof(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
impl OpaqueKeyOwnershipProof {
    /// Create a new `OpaqueKeyOwnershipProof` using the given encoded
    /// representation.
    pub fn new(inner: Vec<u8>) -> OpaqueKeyOwnershipProof {
        OpaqueKeyOwnershipProof(inner)
    }
    /// Try to decode this `OpaqueKeyOwnershipProof` into the given concrete key
    /// ownership proof type.
    pub fn decode<T: Decode>(self) -> Option<T> {
        codec::Decode::decode(&mut &self.0[..]).ok()
    }
}
#[doc(hidden)]
mod sp_api_hidden_includes_DECL_RUNTIME_APIS {
    pub extern crate sp_api as sp_api;
}
#[doc(hidden)]
#[allow(dead_code)]
#[allow(deprecated)]
pub mod runtime_decl_for_GrandpaApi {
    use super::*;
    /// APIs for integrating the GRANDPA finality gadget into runtimes.
    /// This should be implemented on the runtime side.
    ///
    /// This is primarily used for negotiating authority-set changes for the
    /// gadget. GRANDPA uses a signaling model of changing authority sets:
    /// changes should be signaled with a delay of N blocks, and then automatically
    /// applied in the runtime after those N blocks have passed.
    ///
    /// The consensus protocol will coordinate the handoff externally.
    pub trait GrandpaApi<Block: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT> {
        /// Get the current GRANDPA authorities and weights. This should not change except
        /// for when changes are scheduled and the corresponding delay has passed.
        ///
        /// When called at block B, it will return the set of authorities that should be
        /// used to finalize descendants of this block (B+1, B+2, ...). The block B itself
        /// is finalized by the authorities from block B-1.
        fn grandpa_authorities() -> AuthorityList;
        /// Submits an unsigned extrinsic to report an equivocation. The caller
        /// must provide the equivocation proof and a key ownership proof
        /// (should be obtained using `generate_key_ownership_proof`). The
        /// extrinsic will be unsigned and should only be accepted for local
        /// authorship (not to be broadcast to the network). This method returns
        /// `None` when creation of the extrinsic fails, e.g. if equivocation
        /// reporting is disabled for the given runtime (i.e. this method is
        /// hardcoded to return `None`). Only useful in an offchain context.
        fn submit_report_equivocation_unsigned_extrinsic(
            equivocation_proof: EquivocationProof<Block::Hash, NumberFor<Block>>,
            key_owner_proof: OpaqueKeyOwnershipProof,
        ) -> Option<()>;
        /// Generates a proof of key ownership for the given authority in the
        /// given set. An example usage of this module is coupled with the
        /// session historical module to prove that a given authority key is
        /// tied to a given staking identity during a specific session. Proofs
        /// of key ownership are necessary for submitting equivocation reports.
        /// NOTE: even though the API takes a `set_id` as parameter the current
        /// implementations ignore this parameter and instead rely on this
        /// method being called at the correct block height, i.e. any point at
        /// which the given set id is live on-chain. Future implementations will
        /// instead use indexed data through an offchain worker, not requiring
        /// older states to be available.
        fn generate_key_ownership_proof(
            set_id: SetId,
            authority_id: AuthorityId,
        ) -> Option<OpaqueKeyOwnershipProof>;
    }
    pub const VERSION: u32 = 2u32;
    pub const ID: [u8; 8] = [237u8, 153u8, 197u8, 172u8, 178u8, 94u8, 237u8, 245u8];
    #[cfg(any(feature = "std", test))]
    fn convert_between_block_types<
        I: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Encode,
        R: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Decode,
        F: FnOnce(
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::codec::Error,
        ) -> self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError,
    >(
        input: &I,
        map_error: F,
    ) -> std::result::Result<R, self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError>
    {
        < R as self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: DecodeLimit > :: decode_with_depth_limit (self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & mut & self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: Encode :: encode (input) [..]) . map_err (map_error)
    }
    #[cfg(any(feature = "std", test))]
    pub fn grandpa_authorities_native_call_generator<
        'a,
        ApiImpl: GrandpaApi<Block>,
        NodeBlock: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT,
        Block: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT + 'a,
    >() -> impl FnOnce() -> std::result::Result<
        AuthorityList,
        self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError,
    > + 'a {
        move || {
            let res = ApiImpl::grandpa_authorities();
            Ok(res)
        }
    }
    #[cfg(any(feature = "std", test))]
    pub fn submit_report_equivocation_unsigned_extrinsic_native_call_generator<
        'a,
        ApiImpl: GrandpaApi<Block>,
        NodeBlock: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT,
        Block: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT + 'a,
    >(
        equivocation_proof: EquivocationProof<NodeBlock::Hash, NumberFor<NodeBlock>>,
        key_owner_proof: OpaqueKeyOwnershipProof,
    ) -> impl FnOnce() -> std::result::Result<
        Option<()>,
        self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError,
    > + 'a {
        move || {
            let equivocation_proof: EquivocationProof<Block::Hash, NumberFor<Block>> =
                convert_between_block_types(&equivocation_proof, |e| {
                    self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: ApiError :: FailedToConvertParameter { function : "submit_report_equivocation_unsigned_extrinsic" , parameter : "equivocation_proof" , error : e , }
                })?;
            let res = ApiImpl::submit_report_equivocation_unsigned_extrinsic(
                equivocation_proof,
                key_owner_proof,
            );
            Ok(res)
        }
    }
    #[cfg(any(feature = "std", test))]
    pub fn generate_key_ownership_proof_native_call_generator<
        'a,
        ApiImpl: GrandpaApi<Block>,
        NodeBlock: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT,
        Block: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT + 'a,
    >(
        set_id: SetId,
        authority_id: AuthorityId,
    ) -> impl FnOnce() -> std::result::Result<
        Option<OpaqueKeyOwnershipProof>,
        self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError,
    > + 'a {
        move || {
            let res = ApiImpl::generate_key_ownership_proof(set_id, authority_id);
            Ok(res)
        }
    }
    #[cfg(any(feature = "std", test))]
    pub fn grandpa_authorities_call_api_at<
        R: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Encode
            + self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Decode
            + PartialEq,
        NC: FnOnce() -> std::result::Result<
                R,
                self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError,
            > + std::panic::UnwindSafe,
        Block: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT,
        T: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::CallApiAt<Block>,
    >(
        call_runtime_at: &T,
        at: &self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockId<Block>,
        args: Vec<u8>,
        changes: &std::cell::RefCell<
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::OverlayedChanges,
        >,
        storage_transaction_cache: &std::cell::RefCell<
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::StorageTransactionCache<
                Block,
                T::StateBackend,
            >,
        >,
        native_call: Option<NC>,
        context: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ExecutionContext,
        recorder: &Option<
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ProofRecorder<Block>,
        >,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::NativeOrEncoded<R>,
        self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError,
    > {
        let version = call_runtime_at.runtime_version_at(at)?;
        let params = self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::CallApiAtParams {
            at,
            function: "GrandpaApi_grandpa_authorities",
            native_call,
            arguments: args,
            overlayed_changes: changes,
            storage_transaction_cache,
            context,
            recorder,
        };
        call_runtime_at.call_api_at(params)
    }
    #[cfg(any(feature = "std", test))]
    pub fn submit_report_equivocation_unsigned_extrinsic_call_api_at<
        R: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Encode
            + self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Decode
            + PartialEq,
        NC: FnOnce() -> std::result::Result<
                R,
                self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError,
            > + std::panic::UnwindSafe,
        Block: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT,
        T: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::CallApiAt<Block>,
    >(
        call_runtime_at: &T,
        at: &self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockId<Block>,
        args: Vec<u8>,
        changes: &std::cell::RefCell<
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::OverlayedChanges,
        >,
        storage_transaction_cache: &std::cell::RefCell<
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::StorageTransactionCache<
                Block,
                T::StateBackend,
            >,
        >,
        native_call: Option<NC>,
        context: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ExecutionContext,
        recorder: &Option<
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ProofRecorder<Block>,
        >,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::NativeOrEncoded<R>,
        self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError,
    > {
        let version = call_runtime_at.runtime_version_at(at)?;
        let params = self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::CallApiAtParams {
            at,
            function: "GrandpaApi_submit_report_equivocation_unsigned_extrinsic",
            native_call,
            arguments: args,
            overlayed_changes: changes,
            storage_transaction_cache,
            context,
            recorder,
        };
        call_runtime_at.call_api_at(params)
    }
    #[cfg(any(feature = "std", test))]
    pub fn generate_key_ownership_proof_call_api_at<
        R: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Encode
            + self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Decode
            + PartialEq,
        NC: FnOnce() -> std::result::Result<
                R,
                self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError,
            > + std::panic::UnwindSafe,
        Block: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT,
        T: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::CallApiAt<Block>,
    >(
        call_runtime_at: &T,
        at: &self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockId<Block>,
        args: Vec<u8>,
        changes: &std::cell::RefCell<
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::OverlayedChanges,
        >,
        storage_transaction_cache: &std::cell::RefCell<
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::StorageTransactionCache<
                Block,
                T::StateBackend,
            >,
        >,
        native_call: Option<NC>,
        context: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ExecutionContext,
        recorder: &Option<
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ProofRecorder<Block>,
        >,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::NativeOrEncoded<R>,
        self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError,
    > {
        let version = call_runtime_at.runtime_version_at(at)?;
        let params = self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::CallApiAtParams {
            at,
            function: "GrandpaApi_generate_key_ownership_proof",
            native_call,
            arguments: args,
            overlayed_changes: changes,
            storage_transaction_cache,
            context,
            recorder,
        };
        call_runtime_at.call_api_at(params)
    }
}
/// APIs for integrating the GRANDPA finality gadget into runtimes.
/// This should be implemented on the runtime side.
///
/// This is primarily used for negotiating authority-set changes for the
/// gadget. GRANDPA uses a signaling model of changing authority sets:
/// changes should be signaled with a delay of N blocks, and then automatically
/// applied in the runtime after those N blocks have passed.
///
/// The consensus protocol will coordinate the handoff externally.
#[cfg(any(feature = "std", test))]
pub trait GrandpaApi<Block: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT>:
    self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Core<Block>
{
    /// Get the current GRANDPA authorities and weights. This should not change except
    /// for when changes are scheduled and the corresponding delay has passed.
    ///
    /// When called at block B, it will return the set of authorities that should be
    /// used to finalize descendants of this block (B+1, B+2, ...). The block B itself
    /// is finalized by the authorities from block B-1.
    fn grandpa_authorities(
        &self,
        __runtime_api_at_param__: &self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockId<
            Block,
        >,
    ) -> std::result::Result<
        AuthorityList,
        self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError,
    > {
        let runtime_api_impl_params_encoded =
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Encode::encode(&());
        self . GrandpaApi_grandpa_authorities_runtime_api_impl (__runtime_api_at_param__ , self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: ExecutionContext :: OffchainCall (None) , Some (()) , runtime_api_impl_params_encoded) . and_then (| r | match r { self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: NativeOrEncoded :: Native (n) => { Ok (n) } self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: NativeOrEncoded :: Encoded (r) => { < AuthorityList as self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: Decode > :: decode (& mut & r [..]) . map_err (| err | self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: ApiError :: FailedToDecodeReturnValue { function : "grandpa_authorities" , error : err , }) } })
    }
    /// Get the current GRANDPA authorities and weights. This should not change except
    /// for when changes are scheduled and the corresponding delay has passed.
    ///
    /// When called at block B, it will return the set of authorities that should be
    /// used to finalize descendants of this block (B+1, B+2, ...). The block B itself
    /// is finalized by the authorities from block B-1.
    fn grandpa_authorities_with_context(
        &self,
        __runtime_api_at_param__: &self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockId<
            Block,
        >,
        context: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ExecutionContext,
    ) -> std::result::Result<
        AuthorityList,
        self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError,
    > {
        let runtime_api_impl_params_encoded =
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Encode::encode(&());
        self . GrandpaApi_grandpa_authorities_runtime_api_impl (__runtime_api_at_param__ , context , Some (()) , runtime_api_impl_params_encoded) . and_then (| r | match r { self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: NativeOrEncoded :: Native (n) => { Ok (n) } self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: NativeOrEncoded :: Encoded (r) => { < AuthorityList as self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: Decode > :: decode (& mut & r [..]) . map_err (| err | self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: ApiError :: FailedToDecodeReturnValue { function : "grandpa_authorities" , error : err , }) } })
    }
    #[doc(hidden)]
    fn GrandpaApi_grandpa_authorities_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockId<Block>,
        context: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<()>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::NativeOrEncoded<AuthorityList>,
        self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError,
    >;
    /// Submits an unsigned extrinsic to report an equivocation. The caller
    /// must provide the equivocation proof and a key ownership proof
    /// (should be obtained using `generate_key_ownership_proof`). The
    /// extrinsic will be unsigned and should only be accepted for local
    /// authorship (not to be broadcast to the network). This method returns
    /// `None` when creation of the extrinsic fails, e.g. if equivocation
    /// reporting is disabled for the given runtime (i.e. this method is
    /// hardcoded to return `None`). Only useful in an offchain context.
    fn submit_report_equivocation_unsigned_extrinsic(
        &self,
        __runtime_api_at_param__: &self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockId<
            Block,
        >,
        equivocation_proof: EquivocationProof<Block::Hash, NumberFor<Block>>,
        key_owner_proof: OpaqueKeyOwnershipProof,
    ) -> std::result::Result<
        Option<()>,
        self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError,
    > {
        let runtime_api_impl_params_encoded =
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Encode::encode(&(
                &equivocation_proof,
                &key_owner_proof,
            ));
        self . GrandpaApi_submit_report_equivocation_unsigned_extrinsic_runtime_api_impl (__runtime_api_at_param__ , self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: ExecutionContext :: OffchainCall (None) , Some ((equivocation_proof , key_owner_proof)) , runtime_api_impl_params_encoded) . and_then (| r | match r { self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: NativeOrEncoded :: Native (n) => { Ok (n) } self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: NativeOrEncoded :: Encoded (r) => { < Option < () > as self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: Decode > :: decode (& mut & r [..]) . map_err (| err | self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: ApiError :: FailedToDecodeReturnValue { function : "submit_report_equivocation_unsigned_extrinsic" , error : err , }) } })
    }
    /// Submits an unsigned extrinsic to report an equivocation. The caller
    /// must provide the equivocation proof and a key ownership proof
    /// (should be obtained using `generate_key_ownership_proof`). The
    /// extrinsic will be unsigned and should only be accepted for local
    /// authorship (not to be broadcast to the network). This method returns
    /// `None` when creation of the extrinsic fails, e.g. if equivocation
    /// reporting is disabled for the given runtime (i.e. this method is
    /// hardcoded to return `None`). Only useful in an offchain context.
    fn submit_report_equivocation_unsigned_extrinsic_with_context(
        &self,
        __runtime_api_at_param__: &self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockId<
            Block,
        >,
        context: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ExecutionContext,
        equivocation_proof: EquivocationProof<Block::Hash, NumberFor<Block>>,
        key_owner_proof: OpaqueKeyOwnershipProof,
    ) -> std::result::Result<
        Option<()>,
        self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError,
    > {
        let runtime_api_impl_params_encoded =
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Encode::encode(&(
                &equivocation_proof,
                &key_owner_proof,
            ));
        self . GrandpaApi_submit_report_equivocation_unsigned_extrinsic_runtime_api_impl (__runtime_api_at_param__ , context , Some ((equivocation_proof , key_owner_proof)) , runtime_api_impl_params_encoded) . and_then (| r | match r { self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: NativeOrEncoded :: Native (n) => { Ok (n) } self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: NativeOrEncoded :: Encoded (r) => { < Option < () > as self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: Decode > :: decode (& mut & r [..]) . map_err (| err | self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: ApiError :: FailedToDecodeReturnValue { function : "submit_report_equivocation_unsigned_extrinsic" , error : err , }) } })
    }
    #[doc(hidden)]
    fn GrandpaApi_submit_report_equivocation_unsigned_extrinsic_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockId<Block>,
        context: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(
            EquivocationProof<Block::Hash, NumberFor<Block>>,
            OpaqueKeyOwnershipProof,
        )>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::NativeOrEncoded<Option<()>>,
        self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError,
    >;
    /// Generates a proof of key ownership for the given authority in the
    /// given set. An example usage of this module is coupled with the
    /// session historical module to prove that a given authority key is
    /// tied to a given staking identity during a specific session. Proofs
    /// of key ownership are necessary for submitting equivocation reports.
    /// NOTE: even though the API takes a `set_id` as parameter the current
    /// implementations ignore this parameter and instead rely on this
    /// method being called at the correct block height, i.e. any point at
    /// which the given set id is live on-chain. Future implementations will
    /// instead use indexed data through an offchain worker, not requiring
    /// older states to be available.
    fn generate_key_ownership_proof(
        &self,
        __runtime_api_at_param__: &self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockId<
            Block,
        >,
        set_id: SetId,
        authority_id: AuthorityId,
    ) -> std::result::Result<
        Option<OpaqueKeyOwnershipProof>,
        self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError,
    > {
        let runtime_api_impl_params_encoded =
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Encode::encode(&(
                &set_id,
                &authority_id,
            ));
        self . GrandpaApi_generate_key_ownership_proof_runtime_api_impl (__runtime_api_at_param__ , self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: ExecutionContext :: OffchainCall (None) , Some ((set_id , authority_id)) , runtime_api_impl_params_encoded) . and_then (| r | match r { self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: NativeOrEncoded :: Native (n) => { Ok (n) } self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: NativeOrEncoded :: Encoded (r) => { < Option < OpaqueKeyOwnershipProof > as self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: Decode > :: decode (& mut & r [..]) . map_err (| err | self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: ApiError :: FailedToDecodeReturnValue { function : "generate_key_ownership_proof" , error : err , }) } })
    }
    /// Generates a proof of key ownership for the given authority in the
    /// given set. An example usage of this module is coupled with the
    /// session historical module to prove that a given authority key is
    /// tied to a given staking identity during a specific session. Proofs
    /// of key ownership are necessary for submitting equivocation reports.
    /// NOTE: even though the API takes a `set_id` as parameter the current
    /// implementations ignore this parameter and instead rely on this
    /// method being called at the correct block height, i.e. any point at
    /// which the given set id is live on-chain. Future implementations will
    /// instead use indexed data through an offchain worker, not requiring
    /// older states to be available.
    fn generate_key_ownership_proof_with_context(
        &self,
        __runtime_api_at_param__: &self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockId<
            Block,
        >,
        context: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ExecutionContext,
        set_id: SetId,
        authority_id: AuthorityId,
    ) -> std::result::Result<
        Option<OpaqueKeyOwnershipProof>,
        self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError,
    > {
        let runtime_api_impl_params_encoded =
            self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::Encode::encode(&(
                &set_id,
                &authority_id,
            ));
        self . GrandpaApi_generate_key_ownership_proof_runtime_api_impl (__runtime_api_at_param__ , context , Some ((set_id , authority_id)) , runtime_api_impl_params_encoded) . and_then (| r | match r { self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: NativeOrEncoded :: Native (n) => { Ok (n) } self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: NativeOrEncoded :: Encoded (r) => { < Option < OpaqueKeyOwnershipProof > as self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: Decode > :: decode (& mut & r [..]) . map_err (| err | self :: sp_api_hidden_includes_DECL_RUNTIME_APIS :: sp_api :: ApiError :: FailedToDecodeReturnValue { function : "generate_key_ownership_proof" , error : err , }) } })
    }
    #[doc(hidden)]
    fn GrandpaApi_generate_key_ownership_proof_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockId<Block>,
        context: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(SetId, AuthorityId)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            Option<OpaqueKeyOwnershipProof>,
        >,
        self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::ApiError,
    >;
}
#[cfg(any(feature = "std", test))]
impl<Block: self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::BlockT>
    self::sp_api_hidden_includes_DECL_RUNTIME_APIS::sp_api::RuntimeApiInfo for GrandpaApi<Block>
{
    const ID: [u8; 8] = [237u8, 153u8, 197u8, 172u8, 178u8, 94u8, 237u8, 245u8];
    const VERSION: u32 = 2u32;
}
