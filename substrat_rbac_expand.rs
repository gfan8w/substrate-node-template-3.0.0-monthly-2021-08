#![feature(prelude_import)]
//! # Role-based Access Control (RBAC) Pallet
//!
//! The RBAC Pallet implements role-based access control and permissions for Substrate extrinsic calls.
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use codec::{Decode, Encode};
use frame_support::{traits::GetCallMetadata, weights::DispatchInfo};
pub use pallet::*;
use scale_info::TypeInfo;
use sp_runtime::{
    print,
    traits::{DispatchInfoOf, Dispatchable, SignedExtension},
    transaction_validity::{InvalidTransaction, TransactionValidity, TransactionValidityError},
    RuntimeDebug,
};
use sp_std::prelude::*;
///
///			The module that hosts all the
///			[FRAME](https://substrate.dev/docs/en/knowledgebase/runtime/frame)
///			types needed to add this pallet to a
///			[runtime](https://substrate.dev/docs/en/knowledgebase/runtime/).
///
pub mod pallet {
    use super::*;
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    ///
    ///			Configuration trait of this pallet.
    ///
    ///			Implement this type for a runtime in order to customize this pallet.
    ///
    pub trait Config: frame_system::Config {
        /// The Event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// Origin for adding or removing a roles and permissions.
        type RbacAdminOrigin: EnsureOrigin<Self::Origin>;
    }
    ///
    ///			The [pallet](https://substrate.dev/docs/en/knowledgebase/runtime/pallets) implementing
    ///			the on-chain logic.
    ///
    pub struct Pallet<T>(frame_support::sp_std::marker::PhantomData<(T)>);
    const _: () = {
        impl<T> core::clone::Clone for Pallet<T> {
            fn clone(&self) -> Self {
                Self(core::clone::Clone::clone(&self.0))
            }
        }
    };
    const _: () = {
        impl<T> core::cmp::Eq for Pallet<T> {}
    };
    const _: () = {
        impl<T> core::cmp::PartialEq for Pallet<T> {
            fn eq(&self, other: &Self) -> bool {
                true && self.0 == other.0
            }
        }
    };
    const _: () = {
        impl<T> core::fmt::Debug for Pallet<T> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_tuple("Pallet").field(&self.0).finish()
            }
        }
    };
    #[allow(type_alias_bounds)]
    pub type SuperAdmins<T: Config> =
        StorageMap<_GeneratedPrefixForStorageSuperAdmins<T>, Blake2_128Concat, T::AccountId, ()>;
    #[allow(type_alias_bounds)]
    pub type Permissions<T: Config> = StorageMap<
        _GeneratedPrefixForStoragePermissions<T>,
        Blake2_128Concat,
        (T::AccountId, Role),
        (),
    >;
    #[allow(type_alias_bounds)]
    pub type Roles<T: Config> =
        StorageMap<_GeneratedPrefixForStorageRoles<T>, Blake2_128Concat, Role, ()>;
    ///
    ///					Can be used to configure the
    ///					[genesis state](https://substrate.dev/docs/en/knowledgebase/integrate/chain-spec#the-genesis-state)
    ///					of this pallet.
    ///
    #[cfg(feature = "std")]
    #[serde(rename_all = "camelCase")]
    #[serde(deny_unknown_fields)]
    #[serde(bound(serialize = ""))]
    #[serde(bound(deserialize = ""))]
    #[serde(crate = "frame_support::serde")]
    pub struct GenesisConfig<T: Config> {
        pub super_admins: Vec<T::AccountId>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use frame_support::serde as _serde;
        #[automatically_derived]
        impl<T: Config> frame_support::serde::Serialize for GenesisConfig<T> {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> frame_support::serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: frame_support::serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "GenesisConfig",
                    false as usize + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "superAdmins",
                    &self.super_admins,
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
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use frame_support::serde as _serde;
        #[automatically_derived]
        impl<'de, T: Config> frame_support::serde::Deserialize<'de> for GenesisConfig<T> {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> frame_support::serde::__private::Result<Self, __D::Error>
            where
                __D: frame_support::serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 1",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "superAdmins" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Err(_serde::de::Error::unknown_field(
                                __value, FIELDS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"superAdmins" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_field(
                                    __value, FIELDS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de, T: Config> {
                    marker: _serde::__private::PhantomData<GenesisConfig<T>>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de, T: Config> _serde::de::Visitor<'de> for __Visitor<'de, T> {
                    type Value = GenesisConfig<T>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct GenesisConfig")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            Vec<T::AccountId>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct GenesisConfig with 1 element",
                                ));
                            }
                        };
                        _serde::__private::Ok(GenesisConfig {
                            super_admins: __field0,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Vec<T::AccountId>> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "superAdmins",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Vec<T::AccountId>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("superAdmins") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(GenesisConfig {
                            super_admins: __field0,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["superAdmins"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "GenesisConfig",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<GenesisConfig<T>>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                super_admins: Vec::new(),
            }
        }
    }
    #[cfg(feature = "std")]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            for admin in &self.super_admins {
                <SuperAdmins<T>>::insert(admin, ());
            }
        }
    }
    ///
    ///			The [event](https://substrate.dev/docs/en/knowledgebase/runtime/events) emitted
    ///			by this pallet.
    ///
    pub enum Event<T: Config> {
        AccessRevoked(T::AccountId, Vec<u8>),
        AccessGranted(T::AccountId, Vec<u8>),
        SuperAdminAdded(T::AccountId),
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(
            frame_support::sp_std::marker::PhantomData<(T)>,
            frame_support::Never,
        ),
    }
    const _: () = {
        impl<T: Config> core::clone::Clone for Event<T> {
            fn clone(&self) -> Self {
                match self {
                    Self::AccessRevoked(ref _0, ref _1) => Self::AccessRevoked(
                        core::clone::Clone::clone(_0),
                        core::clone::Clone::clone(_1),
                    ),
                    Self::AccessGranted(ref _0, ref _1) => Self::AccessGranted(
                        core::clone::Clone::clone(_0),
                        core::clone::Clone::clone(_1),
                    ),
                    Self::SuperAdminAdded(ref _0) => {
                        Self::SuperAdminAdded(core::clone::Clone::clone(_0))
                    }
                    Self::__Ignore(ref _0, ref _1) => {
                        Self::__Ignore(core::clone::Clone::clone(_0), core::clone::Clone::clone(_1))
                    }
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::cmp::Eq for Event<T> {}
    };
    const _: () = {
        impl<T: Config> core::cmp::PartialEq for Event<T> {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (Self::AccessRevoked(_0, _1), Self::AccessRevoked(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::AccessGranted(_0, _1), Self::AccessGranted(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::SuperAdminAdded(_0), Self::SuperAdminAdded(_0_other)) => {
                        true && _0 == _0_other
                    }
                    (Self::__Ignore(_0, _1), Self::__Ignore(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::AccessRevoked { .. }, Self::AccessGranted { .. }) => false,
                    (Self::AccessRevoked { .. }, Self::SuperAdminAdded { .. }) => false,
                    (Self::AccessRevoked { .. }, Self::__Ignore { .. }) => false,
                    (Self::AccessGranted { .. }, Self::AccessRevoked { .. }) => false,
                    (Self::AccessGranted { .. }, Self::SuperAdminAdded { .. }) => false,
                    (Self::AccessGranted { .. }, Self::__Ignore { .. }) => false,
                    (Self::SuperAdminAdded { .. }, Self::AccessRevoked { .. }) => false,
                    (Self::SuperAdminAdded { .. }, Self::AccessGranted { .. }) => false,
                    (Self::SuperAdminAdded { .. }, Self::__Ignore { .. }) => false,
                    (Self::__Ignore { .. }, Self::AccessRevoked { .. }) => false,
                    (Self::__Ignore { .. }, Self::AccessGranted { .. }) => false,
                    (Self::__Ignore { .. }, Self::SuperAdminAdded { .. }) => false,
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::fmt::Debug for Event<T> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                match *self {
                    Self::AccessRevoked(ref _0, ref _1) => fmt
                        .debug_tuple("Event::AccessRevoked")
                        .field(&_0)
                        .field(&_1)
                        .finish(),
                    Self::AccessGranted(ref _0, ref _1) => fmt
                        .debug_tuple("Event::AccessGranted")
                        .field(&_0)
                        .field(&_1)
                        .finish(),
                    Self::SuperAdminAdded(ref _0) => fmt
                        .debug_tuple("Event::SuperAdminAdded")
                        .field(&_0)
                        .finish(),
                    Self::__Ignore(ref _0, ref _1) => fmt
                        .debug_tuple("Event::__Ignore")
                        .field(&_0)
                        .field(&_1)
                        .finish(),
                }
            }
        }
    };
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl<T: Config> _parity_scale_codec::Encode for Event<T>
        where
            T::AccountId: _parity_scale_codec::Encode,
            T::AccountId: _parity_scale_codec::Encode,
            T::AccountId: _parity_scale_codec::Encode,
            T::AccountId: _parity_scale_codec::Encode,
            T::AccountId: _parity_scale_codec::Encode,
            T::AccountId: _parity_scale_codec::Encode,
        {
            fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Event::AccessRevoked(ref aa, ref ba) => {
                        __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                        _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                        _parity_scale_codec::Encode::encode_to(ba, __codec_dest_edqy);
                    }
                    Event::AccessGranted(ref aa, ref ba) => {
                        __codec_dest_edqy.push_byte(1usize as ::core::primitive::u8);
                        _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                        _parity_scale_codec::Encode::encode_to(ba, __codec_dest_edqy);
                    }
                    Event::SuperAdminAdded(ref aa) => {
                        __codec_dest_edqy.push_byte(2usize as ::core::primitive::u8);
                        _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    _ => (),
                }
            }
        }
        impl<T: Config> _parity_scale_codec::EncodeLike for Event<T>
        where
            T::AccountId: _parity_scale_codec::Encode,
            T::AccountId: _parity_scale_codec::Encode,
            T::AccountId: _parity_scale_codec::Encode,
            T::AccountId: _parity_scale_codec::Encode,
            T::AccountId: _parity_scale_codec::Encode,
            T::AccountId: _parity_scale_codec::Encode,
        {
        }
    };
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl<T: Config> _parity_scale_codec::Decode for Event<T>
        where
            T::AccountId: _parity_scale_codec::Decode,
            T::AccountId: _parity_scale_codec::Decode,
            T::AccountId: _parity_scale_codec::Decode,
            T::AccountId: _parity_scale_codec::Decode,
            T::AccountId: _parity_scale_codec::Decode,
            T::AccountId: _parity_scale_codec::Decode,
        {
            fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, _parity_scale_codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| e.chain("Could not decode `Event`, failed to read variant byte"))?
                {
                    __codec_x_edqy if __codec_x_edqy == 0usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Event::<T>::AccessRevoked(
                            {
                                let __codec_res_edqy =
                                    <T::AccountId as _parity_scale_codec::Decode>::decode(
                                        __codec_input_edqy,
                                    );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::AccessRevoked.0`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            {
                                let __codec_res_edqy =
                                    <Vec<u8> as _parity_scale_codec::Decode>::decode(
                                        __codec_input_edqy,
                                    );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::AccessRevoked.1`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        ))
                    }
                    __codec_x_edqy if __codec_x_edqy == 1usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Event::<T>::AccessGranted(
                            {
                                let __codec_res_edqy =
                                    <T::AccountId as _parity_scale_codec::Decode>::decode(
                                        __codec_input_edqy,
                                    );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::AccessGranted.0`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            {
                                let __codec_res_edqy =
                                    <Vec<u8> as _parity_scale_codec::Decode>::decode(
                                        __codec_input_edqy,
                                    );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::AccessGranted.1`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        ))
                    }
                    __codec_x_edqy if __codec_x_edqy == 2usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Event::<T>::SuperAdminAdded({
                            let __codec_res_edqy =
                                <T::AccountId as _parity_scale_codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `Event::SuperAdminAdded.0`"),
                                    )
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        }))
                    }
                    _ => ::core::result::Result::Err(
                        "Could not decode `Event`, variant doesn\'t exist".into(),
                    ),
                }
            }
        }
    };
    ///
    ///			Custom [dispatch errors](https://substrate.dev/docs/en/knowledgebase/runtime/errors)
    ///			of this pallet.
    ///
    pub enum Error<T> {
        #[doc(hidden)]
        __Ignore(
            frame_support::sp_std::marker::PhantomData<(T)>,
            frame_support::Never,
        ),
        AccessDenied,
    }
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
    impl<T: Config> Pallet<T> {
        pub fn create_role(
            origin: OriginFor<T>,
            pallet_name: Vec<u8>,
            permission: Permission,
        ) -> DispatchResult {
            ensure_signed(origin)?;
            let role = Role {
                pallet: pallet_name,
                permission,
            };
            Roles::<T>::insert(role, ());
            Ok(())
        }
        pub fn assign_role(
            origin: OriginFor<T>,
            account_id: T::AccountId,
            role: Role,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            if Self::verify_manage_access(who, role.pallet.clone()) {
                Self::deposit_event(Event::AccessGranted(
                    account_id.clone(),
                    role.pallet.clone(),
                ));
                <Permissions<T>>::insert((account_id, role), ());
            } else {
                return Err(Error::<T>::AccessDenied.into());
            }
            Ok(())
        }
        pub fn revoke_access(
            origin: OriginFor<T>,
            account_id: T::AccountId,
            role: Role,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            if Self::verify_manage_access(who, role.pallet.clone()) {
                Self::deposit_event(Event::AccessRevoked(
                    account_id.clone(),
                    role.pallet.clone(),
                ));
                <Permissions<T>>::remove((account_id, role));
            } else {
                return Err(Error::<T>::AccessDenied.into());
            }
            Ok(())
        }
        /// Add a new Super Admin.
        /// Super Admins have access to execute and manage all pallets.
        ///
        /// Only _root_ can add a Super Admin.
        pub fn add_super_admin(origin: OriginFor<T>, account_id: T::AccountId) -> DispatchResult {
            T::RbacAdminOrigin::ensure_origin(origin)?;
            <SuperAdmins<T>>::insert(&account_id, ());
            Self::deposit_event(Event::SuperAdminAdded(account_id));
            Ok(())
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn module_constants_metadata(
        ) -> &'static [frame_support::dispatch::ModuleConstantMetadata] {
            &[]
        }
    }
    impl<T: Config> frame_support::error::ModuleErrorMetadata for Pallet<T> {
        fn metadata() -> &'static [frame_support::error::ErrorMetadata] {
            <Error<T> as frame_support::error::ModuleErrorMetadata>::metadata()
        }
    }
    /// Type alias to `Pallet`, to be used by `construct_runtime`.
    ///
    /// Generated by `pallet` attribute macro.
    #[deprecated(note = "use `Pallet` instead")]
    #[allow(dead_code)]
    pub type Module<T> = Pallet<T>;
    impl<T: Config> frame_support::traits::GetStorageVersion for Pallet<T> {
        fn current_storage_version() -> frame_support::traits::StorageVersion {
            frame_support::traits::StorageVersion::default()
        }
        fn on_chain_storage_version() -> frame_support::traits::StorageVersion {
            frame_support::traits::StorageVersion::get::<Self>()
        }
    }
    impl<T: Config> frame_support::traits::OnGenesis for Pallet<T> {
        fn on_genesis() {
            let storage_version = frame_support::traits::StorageVersion::default();
            storage_version.put::<Self>();
        }
    }
    impl<T: Config> frame_support::traits::PalletInfoAccess for Pallet<T> {
        fn index() -> usize {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::index::<
                Self,
            >()
            .expect(
                "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
            )
        }
        fn name() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Self,
            >()
            .expect(
                "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
            )
        }
    }
    impl<T: Config> frame_support::traits::StorageInfoTrait for Pallet<T> {
        fn storage_info() -> frame_support::sp_std::vec::Vec<frame_support::traits::StorageInfo> {
            #[allow(unused_mut)]
            let mut res = ::alloc::vec::Vec::new();
            {
                let mut storage_info = < SuperAdmins < T > as frame_support :: traits :: PartialStorageInfoTrait > :: partial_storage_info () ;
                res.append(&mut storage_info);
            }
            {
                let mut storage_info = < Permissions < T > as frame_support :: traits :: PartialStorageInfoTrait > :: partial_storage_info () ;
                res.append(&mut storage_info);
            }
            {
                let mut storage_info = < Roles < T > as frame_support :: traits :: PartialStorageInfoTrait > :: partial_storage_info () ;
                res.append(&mut storage_info);
            }
            res
        }
    }
    #[doc(hidden)]
    pub mod __substrate_call_check {
        #[doc(hidden)]
        pub use __is_call_part_defined_0 as is_call_part_defined;
    }
    ///Contains one variant per dispatchable that can be called by an extrinsic.
    #[codec(encode_bound())]
    #[codec(decode_bound())]
    #[allow(non_camel_case_types)]
    pub enum Call<T: Config> {
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(
            frame_support::sp_std::marker::PhantomData<(T,)>,
            frame_support::Never,
        ),
        create_role(Vec<u8>, Permission),
        assign_role(T::AccountId, Role),
        revoke_access(T::AccountId, Role),
        /// Add a new Super Admin.
        /// Super Admins have access to execute and manage all pallets.
        ///
        /// Only _root_ can add a Super Admin.
        add_super_admin(T::AccountId),
    }
    const _: () = {
        impl<T: Config> core::fmt::Debug for Call<T> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                match *self {
                    Self::__Ignore(ref _0, ref _1) => fmt
                        .debug_tuple("Call::__Ignore")
                        .field(&_0)
                        .field(&_1)
                        .finish(),
                    Self::create_role(ref _0, ref _1) => fmt
                        .debug_tuple("Call::create_role")
                        .field(&_0)
                        .field(&_1)
                        .finish(),
                    Self::assign_role(ref _0, ref _1) => fmt
                        .debug_tuple("Call::assign_role")
                        .field(&_0)
                        .field(&_1)
                        .finish(),
                    Self::revoke_access(ref _0, ref _1) => fmt
                        .debug_tuple("Call::revoke_access")
                        .field(&_0)
                        .field(&_1)
                        .finish(),
                    Self::add_super_admin(ref _0) => {
                        fmt.debug_tuple("Call::add_super_admin").field(&_0).finish()
                    }
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::clone::Clone for Call<T> {
            fn clone(&self) -> Self {
                match self {
                    Self::__Ignore(ref _0, ref _1) => {
                        Self::__Ignore(core::clone::Clone::clone(_0), core::clone::Clone::clone(_1))
                    }
                    Self::create_role(ref _0, ref _1) => Self::create_role(
                        core::clone::Clone::clone(_0),
                        core::clone::Clone::clone(_1),
                    ),
                    Self::assign_role(ref _0, ref _1) => Self::assign_role(
                        core::clone::Clone::clone(_0),
                        core::clone::Clone::clone(_1),
                    ),
                    Self::revoke_access(ref _0, ref _1) => Self::revoke_access(
                        core::clone::Clone::clone(_0),
                        core::clone::Clone::clone(_1),
                    ),
                    Self::add_super_admin(ref _0) => {
                        Self::add_super_admin(core::clone::Clone::clone(_0))
                    }
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::cmp::Eq for Call<T> {}
    };
    const _: () = {
        impl<T: Config> core::cmp::PartialEq for Call<T> {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (Self::__Ignore(_0, _1), Self::__Ignore(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::create_role(_0, _1), Self::create_role(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::assign_role(_0, _1), Self::assign_role(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::revoke_access(_0, _1), Self::revoke_access(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::add_super_admin(_0), Self::add_super_admin(_0_other)) => {
                        true && _0 == _0_other
                    }
                    (Self::__Ignore { .. }, Self::create_role { .. }) => false,
                    (Self::__Ignore { .. }, Self::assign_role { .. }) => false,
                    (Self::__Ignore { .. }, Self::revoke_access { .. }) => false,
                    (Self::__Ignore { .. }, Self::add_super_admin { .. }) => false,
                    (Self::create_role { .. }, Self::__Ignore { .. }) => false,
                    (Self::create_role { .. }, Self::assign_role { .. }) => false,
                    (Self::create_role { .. }, Self::revoke_access { .. }) => false,
                    (Self::create_role { .. }, Self::add_super_admin { .. }) => false,
                    (Self::assign_role { .. }, Self::__Ignore { .. }) => false,
                    (Self::assign_role { .. }, Self::create_role { .. }) => false,
                    (Self::assign_role { .. }, Self::revoke_access { .. }) => false,
                    (Self::assign_role { .. }, Self::add_super_admin { .. }) => false,
                    (Self::revoke_access { .. }, Self::__Ignore { .. }) => false,
                    (Self::revoke_access { .. }, Self::create_role { .. }) => false,
                    (Self::revoke_access { .. }, Self::assign_role { .. }) => false,
                    (Self::revoke_access { .. }, Self::add_super_admin { .. }) => false,
                    (Self::add_super_admin { .. }, Self::__Ignore { .. }) => false,
                    (Self::add_super_admin { .. }, Self::create_role { .. }) => false,
                    (Self::add_super_admin { .. }, Self::assign_role { .. }) => false,
                    (Self::add_super_admin { .. }, Self::revoke_access { .. }) => false,
                }
            }
        }
    };
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        #[allow(non_camel_case_types)]
        impl<T: Config> _parity_scale_codec::Encode for Call<T> {
            fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Call::create_role(ref aa, ref ba) => {
                        __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                        _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                        _parity_scale_codec::Encode::encode_to(ba, __codec_dest_edqy);
                    }
                    Call::assign_role(ref aa, ref ba) => {
                        __codec_dest_edqy.push_byte(1usize as ::core::primitive::u8);
                        _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                        _parity_scale_codec::Encode::encode_to(ba, __codec_dest_edqy);
                    }
                    Call::revoke_access(ref aa, ref ba) => {
                        __codec_dest_edqy.push_byte(2usize as ::core::primitive::u8);
                        _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                        _parity_scale_codec::Encode::encode_to(ba, __codec_dest_edqy);
                    }
                    Call::add_super_admin(ref aa) => {
                        __codec_dest_edqy.push_byte(3usize as ::core::primitive::u8);
                        _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    _ => (),
                }
            }
        }
        impl<T: Config> _parity_scale_codec::EncodeLike for Call<T> {}
    };
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        #[allow(non_camel_case_types)]
        impl<T: Config> _parity_scale_codec::Decode for Call<T> {
            fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, _parity_scale_codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| e.chain("Could not decode `Call`, failed to read variant byte"))?
                {
                    __codec_x_edqy if __codec_x_edqy == 0usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Call::<T>::create_role(
                            {
                                let __codec_res_edqy =
                                    <Vec<u8> as _parity_scale_codec::Decode>::decode(
                                        __codec_input_edqy,
                                    );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Call::create_role.0`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            {
                                let __codec_res_edqy =
                                    <Permission as _parity_scale_codec::Decode>::decode(
                                        __codec_input_edqy,
                                    );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Call::create_role.1`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        ))
                    }
                    __codec_x_edqy if __codec_x_edqy == 1usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Call::<T>::assign_role(
                            {
                                let __codec_res_edqy =
                                    <T::AccountId as _parity_scale_codec::Decode>::decode(
                                        __codec_input_edqy,
                                    );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Call::assign_role.0`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            {
                                let __codec_res_edqy =
                                    <Role as _parity_scale_codec::Decode>::decode(
                                        __codec_input_edqy,
                                    );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Call::assign_role.1`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        ))
                    }
                    __codec_x_edqy if __codec_x_edqy == 2usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Call::<T>::revoke_access(
                            {
                                let __codec_res_edqy =
                                    <T::AccountId as _parity_scale_codec::Decode>::decode(
                                        __codec_input_edqy,
                                    );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Call::revoke_access.0`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            {
                                let __codec_res_edqy =
                                    <Role as _parity_scale_codec::Decode>::decode(
                                        __codec_input_edqy,
                                    );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Call::revoke_access.1`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        ))
                    }
                    __codec_x_edqy if __codec_x_edqy == 3usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Call::<T>::add_super_admin({
                            let __codec_res_edqy =
                                <T::AccountId as _parity_scale_codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `Call::add_super_admin.0`"),
                                    )
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        }))
                    }
                    _ => ::core::result::Result::Err(
                        "Could not decode `Call`, variant doesn\'t exist".into(),
                    ),
                }
            }
        }
    };
    impl<T: Config> frame_support::dispatch::GetDispatchInfo for Call<T> {
        fn get_dispatch_info(&self) -> frame_support::dispatch::DispatchInfo {
            match *self {
                Self::create_role(ref pallet_name, ref permission) => {
                    let __pallet_base_weight = 0;
                    let __pallet_weight = <dyn frame_support::dispatch::WeighData<(
                        &Vec<u8>,
                        &Permission,
                    )>>::weigh_data(
                        &__pallet_base_weight, (pallet_name, permission)
                    );
                    let __pallet_class = <dyn frame_support::dispatch::ClassifyDispatch<(
                        &Vec<u8>,
                        &Permission,
                    )>>::classify_dispatch(
                        &__pallet_base_weight, (pallet_name, permission)
                    );
                    let __pallet_pays_fee =
                        <dyn frame_support::dispatch::PaysFee<(&Vec<u8>, &Permission)>>::pays_fee(
                            &__pallet_base_weight,
                            (pallet_name, permission),
                        );
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::assign_role(ref account_id, ref role) => {
                    let __pallet_base_weight = 0;
                    let __pallet_weight = <dyn frame_support::dispatch::WeighData<(
                        &T::AccountId,
                        &Role,
                    )>>::weigh_data(
                        &__pallet_base_weight, (account_id, role)
                    );
                    let __pallet_class = <dyn frame_support::dispatch::ClassifyDispatch<(
                        &T::AccountId,
                        &Role,
                    )>>::classify_dispatch(
                        &__pallet_base_weight, (account_id, role)
                    );
                    let __pallet_pays_fee =
                        <dyn frame_support::dispatch::PaysFee<(&T::AccountId, &Role)>>::pays_fee(
                            &__pallet_base_weight,
                            (account_id, role),
                        );
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::revoke_access(ref account_id, ref role) => {
                    let __pallet_base_weight = 0;
                    let __pallet_weight = <dyn frame_support::dispatch::WeighData<(
                        &T::AccountId,
                        &Role,
                    )>>::weigh_data(
                        &__pallet_base_weight, (account_id, role)
                    );
                    let __pallet_class = <dyn frame_support::dispatch::ClassifyDispatch<(
                        &T::AccountId,
                        &Role,
                    )>>::classify_dispatch(
                        &__pallet_base_weight, (account_id, role)
                    );
                    let __pallet_pays_fee =
                        <dyn frame_support::dispatch::PaysFee<(&T::AccountId, &Role)>>::pays_fee(
                            &__pallet_base_weight,
                            (account_id, role),
                        );
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::add_super_admin(ref account_id) => {
                    let __pallet_base_weight = 0;
                    let __pallet_weight =
                        <dyn frame_support::dispatch::WeighData<(&T::AccountId,)>>::weigh_data(
                            &__pallet_base_weight,
                            (account_id,),
                        );
                    let __pallet_class = <dyn frame_support::dispatch::ClassifyDispatch<(
                        &T::AccountId,
                    )>>::classify_dispatch(
                        &__pallet_base_weight, (account_id,)
                    );
                    let __pallet_pays_fee =
                        <dyn frame_support::dispatch::PaysFee<(&T::AccountId,)>>::pays_fee(
                            &__pallet_base_weight,
                            (account_id,),
                        );
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &match (&"__Ignore cannot be used",) {
                            _args => [::core::fmt::ArgumentV1::new(
                                _args.0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ))
                }
            }
        }
    }
    impl<T: Config> frame_support::dispatch::GetCallName for Call<T> {
        fn get_call_name(&self) -> &'static str {
            match *self {
                Self::create_role(..) => "create_role",
                Self::assign_role(..) => "assign_role",
                Self::revoke_access(..) => "revoke_access",
                Self::add_super_admin(..) => "add_super_admin",
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &match (&"__PhantomItem cannot be used.",) {
                            _args => [::core::fmt::ArgumentV1::new(
                                _args.0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ))
                }
            }
        }
        fn get_call_names() -> &'static [&'static str] {
            &[
                "create_role",
                "assign_role",
                "revoke_access",
                "add_super_admin",
            ]
        }
    }
    impl<T: Config> frame_support::traits::UnfilteredDispatchable for Call<T> {
        type Origin = frame_system::pallet_prelude::OriginFor<T>;
        fn dispatch_bypass_filter(
            self,
            origin: Self::Origin,
        ) -> frame_support::dispatch::DispatchResultWithPostInfo {
            match self {
                Self::create_role(pallet_name, permission) => {
                    let __within_span__ = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "create_role",
                                    "substrate_rbac::pallet",
                                    ::tracing::Level::TRACE,
                                    Some("pallets/substrate-rbac/src/lib.rs"),
                                    Some(19u32),
                                    Some("substrate_rbac::pallet"),
                                    ::tracing_core::field::FieldSet::new(
                                        &[],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                        } else {
                            let span = CALLSITE.disabled_span();
                            {};
                            span
                        }
                    };
                    let __tracing_guard__ = __within_span__.enter();
                    <Pallet<T>>::create_role(origin, pallet_name, permission)
                        .map(Into::into)
                        .map_err(Into::into)
                }
                Self::assign_role(account_id, role) => {
                    let __within_span__ = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "assign_role",
                                    "substrate_rbac::pallet",
                                    ::tracing::Level::TRACE,
                                    Some("pallets/substrate-rbac/src/lib.rs"),
                                    Some(19u32),
                                    Some("substrate_rbac::pallet"),
                                    ::tracing_core::field::FieldSet::new(
                                        &[],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                        } else {
                            let span = CALLSITE.disabled_span();
                            {};
                            span
                        }
                    };
                    let __tracing_guard__ = __within_span__.enter();
                    <Pallet<T>>::assign_role(origin, account_id, role)
                        .map(Into::into)
                        .map_err(Into::into)
                }
                Self::revoke_access(account_id, role) => {
                    let __within_span__ = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "revoke_access",
                                    "substrate_rbac::pallet",
                                    ::tracing::Level::TRACE,
                                    Some("pallets/substrate-rbac/src/lib.rs"),
                                    Some(19u32),
                                    Some("substrate_rbac::pallet"),
                                    ::tracing_core::field::FieldSet::new(
                                        &[],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                        } else {
                            let span = CALLSITE.disabled_span();
                            {};
                            span
                        }
                    };
                    let __tracing_guard__ = __within_span__.enter();
                    <Pallet<T>>::revoke_access(origin, account_id, role)
                        .map(Into::into)
                        .map_err(Into::into)
                }
                Self::add_super_admin(account_id) => {
                    let __within_span__ = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "add_super_admin",
                                    "substrate_rbac::pallet",
                                    ::tracing::Level::TRACE,
                                    Some("pallets/substrate-rbac/src/lib.rs"),
                                    Some(19u32),
                                    Some("substrate_rbac::pallet"),
                                    ::tracing_core::field::FieldSet::new(
                                        &[],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                        } else {
                            let span = CALLSITE.disabled_span();
                            {};
                            span
                        }
                    };
                    let __tracing_guard__ = __within_span__.enter();
                    <Pallet<T>>::add_super_admin(origin, account_id)
                        .map(Into::into)
                        .map_err(Into::into)
                }
                Self::__Ignore(_, _) => {
                    let _ = origin;
                    {
                        {
                            ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                                &["internal error: entered unreachable code: "],
                                &match (&"__PhantomItem cannot be used.",) {
                                    _args => [::core::fmt::ArgumentV1::new(
                                        _args.0,
                                        ::core::fmt::Display::fmt,
                                    )],
                                },
                            ))
                        }
                    };
                }
            }
        }
    }
    impl<T: Config> frame_support::dispatch::Callable<T> for Pallet<T> {
        type Call = Call<T>;
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        #[allow(dead_code)]
        pub fn call_functions() -> &'static [frame_support::dispatch::FunctionMetadata] {
            &[
                frame_support::dispatch::FunctionMetadata {
                    name: frame_support::dispatch::DecodeDifferent::Encode("create_role"),
                    arguments: frame_support::dispatch::DecodeDifferent::Encode(&[
                        frame_support::dispatch::FunctionArgumentMetadata {
                            name: frame_support::dispatch::DecodeDifferent::Encode("pallet_name"),
                            ty: frame_support::dispatch::DecodeDifferent::Encode("Vec<u8>"),
                        },
                        frame_support::dispatch::FunctionArgumentMetadata {
                            name: frame_support::dispatch::DecodeDifferent::Encode("permission"),
                            ty: frame_support::dispatch::DecodeDifferent::Encode("Permission"),
                        },
                    ]),
                    documentation: frame_support::dispatch::DecodeDifferent::Encode(&[]),
                },
                frame_support::dispatch::FunctionMetadata {
                    name: frame_support::dispatch::DecodeDifferent::Encode("assign_role"),
                    arguments: frame_support::dispatch::DecodeDifferent::Encode(&[
                        frame_support::dispatch::FunctionArgumentMetadata {
                            name: frame_support::dispatch::DecodeDifferent::Encode("account_id"),
                            ty: frame_support::dispatch::DecodeDifferent::Encode("T::AccountId"),
                        },
                        frame_support::dispatch::FunctionArgumentMetadata {
                            name: frame_support::dispatch::DecodeDifferent::Encode("role"),
                            ty: frame_support::dispatch::DecodeDifferent::Encode("Role"),
                        },
                    ]),
                    documentation: frame_support::dispatch::DecodeDifferent::Encode(&[]),
                },
                frame_support::dispatch::FunctionMetadata {
                    name: frame_support::dispatch::DecodeDifferent::Encode("revoke_access"),
                    arguments: frame_support::dispatch::DecodeDifferent::Encode(&[
                        frame_support::dispatch::FunctionArgumentMetadata {
                            name: frame_support::dispatch::DecodeDifferent::Encode("account_id"),
                            ty: frame_support::dispatch::DecodeDifferent::Encode("T::AccountId"),
                        },
                        frame_support::dispatch::FunctionArgumentMetadata {
                            name: frame_support::dispatch::DecodeDifferent::Encode("role"),
                            ty: frame_support::dispatch::DecodeDifferent::Encode("Role"),
                        },
                    ]),
                    documentation: frame_support::dispatch::DecodeDifferent::Encode(&[]),
                },
                frame_support::dispatch::FunctionMetadata {
                    name: frame_support::dispatch::DecodeDifferent::Encode("add_super_admin"),
                    arguments: frame_support::dispatch::DecodeDifferent::Encode(&[
                        frame_support::dispatch::FunctionArgumentMetadata {
                            name: frame_support::dispatch::DecodeDifferent::Encode("account_id"),
                            ty: frame_support::dispatch::DecodeDifferent::Encode("T::AccountId"),
                        },
                    ]),
                    documentation: frame_support::dispatch::DecodeDifferent::Encode(&[
                        " Add a new Super Admin.",
                        " Super Admins have access to execute and manage all pallets.",
                        "",
                        " Only _root_ can add a Super Admin.",
                    ]),
                },
            ]
        }
    }
    impl<T: Config> frame_support::sp_std::fmt::Debug for Error<T> {
        fn fmt(
            &self,
            f: &mut frame_support::sp_std::fmt::Formatter<'_>,
        ) -> frame_support::sp_std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl<T: Config> Error<T> {
        pub fn as_u8(&self) -> u8 {
            match &self {
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &match (&"`__Ignore` can never be constructed",) {
                            _args => [::core::fmt::ArgumentV1::new(
                                _args.0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ))
                }
                Self::AccessDenied => 0usize as u8,
            }
        }
        pub fn as_str(&self) -> &'static str {
            match &self {
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &match (&"`__Ignore` can never be constructed",) {
                            _args => [::core::fmt::ArgumentV1::new(
                                _args.0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ))
                }
                Self::AccessDenied => "AccessDenied",
            }
        }
    }
    impl<T: Config> From<Error<T>> for &'static str {
        fn from(err: Error<T>) -> &'static str {
            err.as_str()
        }
    }
    impl<T: Config> From<Error<T>> for frame_support::sp_runtime::DispatchError {
        fn from(err: Error<T>) -> Self {
            let index = < < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: index :: < Pallet < T > > () . expect ("Every active module has an index in the runtime; qed") as u8 ;
            frame_support::sp_runtime::DispatchError::Module {
                index,
                error: err.as_u8(),
                message: Some(err.as_str()),
            }
        }
    }
    impl<T: Config> frame_support::error::ModuleErrorMetadata for Error<T> {
        fn metadata() -> &'static [frame_support::error::ErrorMetadata] {
            &[frame_support::error::ErrorMetadata {
                name: frame_support::error::DecodeDifferent::Encode("AccessDenied"),
                documentation: frame_support::error::DecodeDifferent::Encode(&[]),
            }]
        }
    }
    #[doc(hidden)]
    pub mod __substrate_event_check {
        #[doc(hidden)]
        pub use __is_event_part_defined_1 as is_event_part_defined;
    }
    impl<T: Config> Pallet<T> {
        pub(super) fn deposit_event(event: Event<T>) {
            let event = <<T as Config>::Event as From<Event<T>>>::from(event);
            let event =
                <<T as Config>::Event as Into<<T as frame_system::Config>::Event>>::into(event);
            <frame_system::Pallet<T>>::deposit_event(event)
        }
    }
    impl<T: Config> From<Event<T>> for () {
        fn from(_: Event<T>) {}
    }
    impl<T: Config> Event<T> {
        #[allow(dead_code)]
        #[doc(hidden)]
        pub fn metadata() -> &'static [frame_support::event::EventMetadata] {
            &[
                frame_support::event::EventMetadata {
                    name: frame_support::event::DecodeDifferent::Encode("AccessRevoked"),
                    arguments: frame_support::event::DecodeDifferent::Encode(&[
                        "T::AccountId",
                        "Vec<u8>",
                    ]),
                    documentation: frame_support::event::DecodeDifferent::Encode(&[]),
                },
                frame_support::event::EventMetadata {
                    name: frame_support::event::DecodeDifferent::Encode("AccessGranted"),
                    arguments: frame_support::event::DecodeDifferent::Encode(&[
                        "T::AccountId",
                        "Vec<u8>",
                    ]),
                    documentation: frame_support::event::DecodeDifferent::Encode(&[]),
                },
                frame_support::event::EventMetadata {
                    name: frame_support::event::DecodeDifferent::Encode("SuperAdminAdded"),
                    arguments: frame_support::event::DecodeDifferent::Encode(&["T::AccountId"]),
                    documentation: frame_support::event::DecodeDifferent::Encode(&[]),
                },
            ]
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn storage_metadata() -> frame_support::metadata::StorageMetadata {
            frame_support :: metadata :: StorageMetadata { prefix : frame_support :: metadata :: DecodeDifferent :: Encode (< < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: name :: < Pallet < T > > () . expect ("Every active pallet has a name in the runtime; qed")) , entries : frame_support :: metadata :: DecodeDifferent :: Encode (& [frame_support :: metadata :: StorageEntryMetadata { name : frame_support :: metadata :: DecodeDifferent :: Encode (< SuperAdmins < T > as frame_support :: storage :: types :: StorageMapMetadata > :: NAME) , modifier : < SuperAdmins < T > as frame_support :: storage :: types :: StorageMapMetadata > :: MODIFIER , ty : frame_support :: metadata :: StorageEntryType :: Map { hasher : < SuperAdmins < T > as frame_support :: storage :: types :: StorageMapMetadata > :: HASHER , key : frame_support :: metadata :: DecodeDifferent :: Encode ("T::AccountId") , value : frame_support :: metadata :: DecodeDifferent :: Encode ("()") , unused : false , } , default : frame_support :: metadata :: DecodeDifferent :: Encode (< SuperAdmins < T > as frame_support :: storage :: types :: StorageMapMetadata > :: DEFAULT) , documentation : frame_support :: metadata :: DecodeDifferent :: Encode (& []) , } , frame_support :: metadata :: StorageEntryMetadata { name : frame_support :: metadata :: DecodeDifferent :: Encode (< Permissions < T > as frame_support :: storage :: types :: StorageMapMetadata > :: NAME) , modifier : < Permissions < T > as frame_support :: storage :: types :: StorageMapMetadata > :: MODIFIER , ty : frame_support :: metadata :: StorageEntryType :: Map { hasher : < Permissions < T > as frame_support :: storage :: types :: StorageMapMetadata > :: HASHER , key : frame_support :: metadata :: DecodeDifferent :: Encode ("(T::AccountId, Role)") , value : frame_support :: metadata :: DecodeDifferent :: Encode ("()") , unused : false , } , default : frame_support :: metadata :: DecodeDifferent :: Encode (< Permissions < T > as frame_support :: storage :: types :: StorageMapMetadata > :: DEFAULT) , documentation : frame_support :: metadata :: DecodeDifferent :: Encode (& []) , } , frame_support :: metadata :: StorageEntryMetadata { name : frame_support :: metadata :: DecodeDifferent :: Encode (< Roles < T > as frame_support :: storage :: types :: StorageMapMetadata > :: NAME) , modifier : < Roles < T > as frame_support :: storage :: types :: StorageMapMetadata > :: MODIFIER , ty : frame_support :: metadata :: StorageEntryType :: Map { hasher : < Roles < T > as frame_support :: storage :: types :: StorageMapMetadata > :: HASHER , key : frame_support :: metadata :: DecodeDifferent :: Encode ("Role") , value : frame_support :: metadata :: DecodeDifferent :: Encode ("()") , unused : false , } , default : frame_support :: metadata :: DecodeDifferent :: Encode (< Roles < T > as frame_support :: storage :: types :: StorageMapMetadata > :: DEFAULT) , documentation : frame_support :: metadata :: DecodeDifferent :: Encode (& []) , }]) , }
        }
    }
    impl<T: Config> Pallet<T> {
        pub fn super_admins<KArg>(k: KArg) -> Option<()>
        where
            KArg: frame_support::codec::EncodeLike<T::AccountId>,
        {
            <SuperAdmins<T> as frame_support::storage::StorageMap<T::AccountId, ()>>::get(k)
        }
    }
    impl<T: Config> Pallet<T> {
        pub fn permissions<KArg>(k: KArg) -> Option<()>
        where
            KArg: frame_support::codec::EncodeLike<(T::AccountId, Role)>,
        {
            <Permissions<T> as frame_support::storage::StorageMap<(T::AccountId, Role), ()>>::get(k)
        }
    }
    impl<T: Config> Pallet<T> {
        pub fn roles<KArg>(k: KArg) -> Option<()>
        where
            KArg: frame_support::codec::EncodeLike<Role>,
        {
            <Roles<T> as frame_support::storage::StorageMap<Role, ()>>::get(k)
        }
    }
    pub struct _GeneratedPrefixForStorageSuperAdmins<T>(core::marker::PhantomData<(T,)>);
    impl<T: Config> frame_support::traits::StorageInstance
        for _GeneratedPrefixForStorageSuperAdmins<T>
    {
        fn pallet_prefix() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Pallet<T>,
            >()
            .expect("Every active pallet has a name in the runtime; qed")
        }
        const STORAGE_PREFIX: &'static str = "SuperAdmins";
    }
    pub struct _GeneratedPrefixForStoragePermissions<T>(core::marker::PhantomData<(T,)>);
    impl<T: Config> frame_support::traits::StorageInstance
        for _GeneratedPrefixForStoragePermissions<T>
    {
        fn pallet_prefix() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Pallet<T>,
            >()
            .expect("Every active pallet has a name in the runtime; qed")
        }
        const STORAGE_PREFIX: &'static str = "Permissions";
    }
    pub struct _GeneratedPrefixForStorageRoles<T>(core::marker::PhantomData<(T,)>);
    impl<T: Config> frame_support::traits::StorageInstance for _GeneratedPrefixForStorageRoles<T> {
        fn pallet_prefix() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Pallet<T>,
            >()
            .expect("Every active pallet has a name in the runtime; qed")
        }
        const STORAGE_PREFIX: &'static str = "Roles";
    }
    #[doc(hidden)]
    pub mod __substrate_inherent_check {
        #[doc(hidden)]
        pub use __is_inherent_part_defined_2 as is_inherent_part_defined;
    }
    /// Hidden instance generated to be internally used when module is used without
    /// instance.
    #[doc(hidden)]
    pub type __InherentHiddenInstance = ();
    pub(super) trait Store {
        type SuperAdmins;
        type Permissions;
        type Roles;
    }
    impl<T: Config> Store for Pallet<T> {
        type SuperAdmins = SuperAdmins<T>;
        type Permissions = Permissions<T>;
        type Roles = Roles<T>;
    }
    impl<T: Config> frame_support::traits::OnFinalize<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn on_finalize(n: <T as frame_system::Config>::BlockNumber) {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_finalize",
                            "substrate_rbac::pallet",
                            ::tracing::Level::TRACE,
                            Some("pallets/substrate-rbac/src/lib.rs"),
                            Some(19u32),
                            Some("substrate_rbac::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && CALLSITE.is_enabled(interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = CALLSITE.disabled_span();
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: on_finalize (n)
        }
    }
    impl<T: Config> frame_support::traits::OnIdle<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn on_idle(
            n: <T as frame_system::Config>::BlockNumber,
            remaining_weight: frame_support::weights::Weight,
        ) -> frame_support::weights::Weight {
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: on_idle (n , remaining_weight)
        }
    }
    impl<T: Config> frame_support::traits::OnInitialize<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn on_initialize(
            n: <T as frame_system::Config>::BlockNumber,
        ) -> frame_support::weights::Weight {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_initialize",
                            "substrate_rbac::pallet",
                            ::tracing::Level::TRACE,
                            Some("pallets/substrate-rbac/src/lib.rs"),
                            Some(19u32),
                            Some("substrate_rbac::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && CALLSITE.is_enabled(interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = CALLSITE.disabled_span();
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: on_initialize (n)
        }
    }
    impl<T: Config> frame_support::traits::OnRuntimeUpgrade for Pallet<T> {
        fn on_runtime_upgrade() -> frame_support::weights::Weight {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_runtime_update",
                            "substrate_rbac::pallet",
                            ::tracing::Level::TRACE,
                            Some("pallets/substrate-rbac/src/lib.rs"),
                            Some(19u32),
                            Some("substrate_rbac::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && CALLSITE.is_enabled(interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = CALLSITE.disabled_span();
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            let pallet_name = < < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: name :: < Self > () . unwrap_or ("<unknown pallet name>") ;
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["\u{2705} no migration for "],
                            &match (&pallet_name,) {
                                _args => [::core::fmt::ArgumentV1::new(
                                    _args.0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ),
                        lvl,
                        &(
                            frame_support::LOG_TARGET,
                            "substrate_rbac::pallet",
                            "pallets/substrate-rbac/src/lib.rs",
                            19u32,
                        ),
                    );
                }
            };
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: on_runtime_upgrade ()
        }
    }
    impl<T: Config> frame_support::traits::OffchainWorker<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn offchain_worker(n: <T as frame_system::Config>::BlockNumber) {
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: offchain_worker (n)
        }
    }
    impl<T: Config> frame_support::traits::IntegrityTest for Pallet<T> {
        fn integrity_test() {
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: integrity_test ()
        }
    }
    #[cfg(feature = "std")]
    impl<T: Config> frame_support::sp_runtime::BuildModuleGenesisStorage<T, ()> for GenesisConfig<T> {
        fn build_module_genesis_storage(
            &self,
            storage: &mut frame_support::sp_runtime::Storage,
        ) -> std::result::Result<(), std::string::String> {
            frame_support::BasicExternalities::execute_with_storage(storage, || {
                <Self as frame_support::traits::GenesisBuild<T>>::build(self);
                Ok(())
            })
        }
    }
    #[doc(hidden)]
    pub mod __substrate_genesis_config_check {
        #[doc(hidden)]
        pub use __is_genesis_config_defined_3 as is_genesis_config_defined;
        #[doc(hidden)]
        pub use __is_std_macro_defined_for_genesis_3 as is_std_enabled_for_genesis;
    }
    #[doc(hidden)]
    pub mod __substrate_origin_check {
        #[doc(hidden)]
        pub use __is_origin_part_defined_4 as is_origin_part_defined;
    }
    #[doc(hidden)]
    pub mod __substrate_validate_unsigned_check {
        #[doc(hidden)]
        pub use __is_validate_unsigned_part_defined_5 as is_validate_unsigned_part_defined;
    }
}
pub enum Permission {
    Execute,
    Manage,
}
impl ::core::marker::StructuralPartialEq for Permission {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for Permission {
    #[inline]
    fn eq(&self, other: &Permission) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    _ => true,
                }
            } else {
                false
            }
        }
    }
}
impl ::core::marker::StructuralEq for Permission {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for Permission {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {}
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Permission {
    #[inline]
    fn clone(&self) -> Permission {
        match (&*self,) {
            (&Permission::Execute,) => Permission::Execute,
            (&Permission::Manage,) => Permission::Manage,
        }
    }
}
impl core::fmt::Debug for Permission {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Execute => fmt.debug_tuple("Permission::Execute").finish(),
            Self::Manage => fmt.debug_tuple("Permission::Manage").finish(),
            _ => Ok(()),
        }
    }
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Encode for Permission {
        fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            match *self {
                Permission::Execute => {
                    __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                }
                Permission::Manage => {
                    __codec_dest_edqy.push_byte(1usize as ::core::primitive::u8);
                }
                _ => (),
            }
        }
    }
    impl _parity_scale_codec::EncodeLike for Permission {}
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Decode for Permission {
        fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, _parity_scale_codec::Error> {
            match __codec_input_edqy.read_byte().map_err(|e| {
                e.chain("Could not decode `Permission`, failed to read variant byte")
            })? {
                __codec_x_edqy if __codec_x_edqy == 0usize as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Permission::Execute)
                }
                __codec_x_edqy if __codec_x_edqy == 1usize as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Permission::Manage)
                }
                _ => ::core::result::Result::Err(
                    "Could not decode `Permission`, variant doesn\'t exist".into(),
                ),
            }
        }
    }
};
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for Permission {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            ::scale_info::Type::builder()
                .path(::scale_info::Path::new("Permission", "substrate_rbac"))
                .type_params(::alloc::vec::Vec::new())
                .docs(&[])
                .variant(
                    ::scale_info::build::Variants::new()
                        .variant("Execute", |v| {
                            v.index(0usize as ::core::primitive::u8).docs(&[])
                        })
                        .variant("Manage", |v| {
                            v.index(1usize as ::core::primitive::u8).docs(&[])
                        }),
                )
        }
    };
};
impl Default for Permission {
    fn default() -> Self {
        Permission::Execute
    }
}
pub struct Role {
    pallet: Vec<u8>,
    permission: Permission,
}
impl ::core::marker::StructuralPartialEq for Role {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for Role {
    #[inline]
    fn eq(&self, other: &Role) -> bool {
        match *other {
            Role {
                pallet: ref __self_1_0,
                permission: ref __self_1_1,
            } => match *self {
                Role {
                    pallet: ref __self_0_0,
                    permission: ref __self_0_1,
                } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Role) -> bool {
        match *other {
            Role {
                pallet: ref __self_1_0,
                permission: ref __self_1_1,
            } => match *self {
                Role {
                    pallet: ref __self_0_0,
                    permission: ref __self_0_1,
                } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
            },
        }
    }
}
impl ::core::marker::StructuralEq for Role {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for Role {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<Vec<u8>>;
            let _: ::core::cmp::AssertParamIsEq<Permission>;
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Role {
    #[inline]
    fn clone(&self) -> Role {
        match *self {
            Role {
                pallet: ref __self_0_0,
                permission: ref __self_0_1,
            } => Role {
                pallet: ::core::clone::Clone::clone(&(*__self_0_0)),
                permission: ::core::clone::Clone::clone(&(*__self_0_1)),
            },
        }
    }
}
impl core::fmt::Debug for Role {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_struct("Role")
            .field("pallet", &self.pallet)
            .field("permission", &self.permission)
            .finish()
    }
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Encode for Role {
        fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            _parity_scale_codec::Encode::encode_to(&self.pallet, __codec_dest_edqy);
            _parity_scale_codec::Encode::encode_to(&self.permission, __codec_dest_edqy);
        }
    }
    impl _parity_scale_codec::EncodeLike for Role {}
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Decode for Role {
        fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, _parity_scale_codec::Error> {
            ::core::result::Result::Ok(Role {
                pallet: {
                    let __codec_res_edqy =
                        <Vec<u8> as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                    match __codec_res_edqy {
                        ::core::result::Result::Err(e) => {
                            return ::core::result::Result::Err(
                                e.chain("Could not decode `Role::pallet`"),
                            )
                        }
                        ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                },
                permission: {
                    let __codec_res_edqy =
                        <Permission as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                    match __codec_res_edqy {
                        ::core::result::Result::Err(e) => {
                            return ::core::result::Result::Err(
                                e.chain("Could not decode `Role::permission`"),
                            )
                        }
                        ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                },
            })
        }
    }
};
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for Role {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            ::scale_info::Type::builder()
                .path(::scale_info::Path::new("Role", "substrate_rbac"))
                .type_params(::alloc::vec::Vec::new())
                .docs(&[])
                .composite(
                    ::scale_info::build::Fields::named()
                        .field(|f| {
                            f.ty::<Vec<u8>>()
                                .name("pallet")
                                .type_name("Vec<u8>")
                                .docs(&[])
                        })
                        .field(|f| {
                            f.ty::<Permission>()
                                .name("permission")
                                .type_name("Permission")
                                .docs(&[])
                        }),
                )
        }
    };
};
impl<T: Config> Pallet<T> {
    pub fn verify_execute_access(account_id: T::AccountId, pallet: Vec<u8>) -> bool {
        let role = Role {
            pallet,
            permission: Permission::Execute,
        };
        if <Roles<T>>::contains_key(&role) && <Permissions<T>>::contains_key((account_id, role)) {
            return true;
        }
        false
    }
    fn verify_manage_access(account_id: T::AccountId, pallet: Vec<u8>) -> bool {
        let role = Role {
            pallet,
            permission: Permission::Manage,
        };
        if <Roles<T>>::contains_key(&role) && <Permissions<T>>::contains_key((account_id, role)) {
            return true;
        }
        false
    }
}
/// The following section implements the `SignedExtension` trait
/// for the `Authorize` type.
/// `SignedExtension` is being used here to filter out the not authorized accounts
/// when they try to send extrinsics to the runtime.
/// Inside the `validate` function of the `SignedExtension` trait,
/// we check if the sender (origin) of the extrinsic has the execute permission or not.
/// The validation happens at the transaction queue level,
///  and the extrinsics are filtered out before they hit the pallet logic.
/// The `Authorize` struct.
#[scale_info(skip_type_params(T))]
pub struct Authorize<T: Config + Send + Sync>(sp_std::marker::PhantomData<T>);
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<T: Config + Send + Sync> _parity_scale_codec::Encode for Authorize<T>
    where
        sp_std::marker::PhantomData<T>: _parity_scale_codec::Encode,
        sp_std::marker::PhantomData<T>: _parity_scale_codec::Encode,
    {
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
    impl<T: Config + Send + Sync> _parity_scale_codec::EncodeLike for Authorize<T>
    where
        sp_std::marker::PhantomData<T>: _parity_scale_codec::Encode,
        sp_std::marker::PhantomData<T>: _parity_scale_codec::Encode,
    {
    }
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<T: Config + Send + Sync> _parity_scale_codec::Decode for Authorize<T>
    where
        sp_std::marker::PhantomData<T>: _parity_scale_codec::Decode,
        sp_std::marker::PhantomData<T>: _parity_scale_codec::Decode,
    {
        fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, _parity_scale_codec::Error> {
            ::core::result::Result::Ok(Authorize::<T>({
                let __codec_res_edqy =
                    <sp_std::marker::PhantomData<T> as _parity_scale_codec::Decode>::decode(
                        __codec_input_edqy,
                    );
                match __codec_res_edqy {
                    ::core::result::Result::Err(e) => {
                        return ::core::result::Result::Err(
                            e.chain("Could not decode `Authorize.0`"),
                        )
                    }
                    ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                }
            }))
        }
    }
};
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::clone::Clone + Config + Send + Sync> ::core::clone::Clone for Authorize<T> {
    #[inline]
    fn clone(&self) -> Authorize<T> {
        match *self {
            Authorize(ref __self_0_0) => Authorize(::core::clone::Clone::clone(&(*__self_0_0))),
        }
    }
}
impl<T: Config + Send + Sync> ::core::marker::StructuralEq for Authorize<T> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::cmp::Eq + Config + Send + Sync> ::core::cmp::Eq for Authorize<T> {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<sp_std::marker::PhantomData<T>>;
        }
    }
}
impl<T: Config + Send + Sync> ::core::marker::StructuralPartialEq for Authorize<T> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::cmp::PartialEq + Config + Send + Sync> ::core::cmp::PartialEq for Authorize<T> {
    #[inline]
    fn eq(&self, other: &Authorize<T>) -> bool {
        match *other {
            Authorize(ref __self_1_0) => match *self {
                Authorize(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Authorize<T>) -> bool {
        match *other {
            Authorize(ref __self_1_0) => match *self {
                Authorize(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    impl<T: Config + Send + Sync> ::scale_info::TypeInfo for Authorize<T>
    where
        sp_std::marker::PhantomData<T>: ::scale_info::TypeInfo + 'static,
        T: Config + Send + Sync + 'static,
    {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            :: scale_info :: Type :: builder () . path (:: scale_info :: Path :: new ("Authorize" , "substrate_rbac")) . type_params (< [_] > :: into_vec (box [:: scale_info :: TypeParameter :: new ("T" , :: core :: option :: Option :: None)])) . docs (& ["The following section implements the `SignedExtension` trait" , "for the `Authorize` type." , "`SignedExtension` is being used here to filter out the not authorized accounts" , "when they try to send extrinsics to the runtime." , "Inside the `validate` function of the `SignedExtension` trait," , "we check if the sender (origin) of the extrinsic has the execute permission or not." , "The validation happens at the transaction queue level," , " and the extrinsics are filtered out before they hit the pallet logic." , "The `Authorize` struct."]) . composite (:: scale_info :: build :: Fields :: unnamed () . field (| f | f . ty :: < sp_std :: marker :: PhantomData < T > > () . type_name ("sp_std::marker::PhantomData<T>") . docs (& [])))
        }
    };
};
/// Debug impl for the `Authorize` struct.
impl<T: Config + Send + Sync> sp_std::fmt::Debug for Authorize<T> {
    #[cfg(feature = "std")]
    fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
        f.write_fmt(::core::fmt::Arguments::new_v1(
            &["Authorize"],
            &match () {
                _args => [],
            },
        ))
    }
}
impl<T: Config + Send + Sync> Authorize<T> {
    pub fn new() -> Self {
        Self(sp_std::marker::PhantomData)
    }
}
impl<T: Config + Send + Sync> SignedExtension for Authorize<T>
where
    T::Call: Dispatchable<Info = DispatchInfo> + GetCallMetadata,
{
    type AccountId = T::AccountId;
    type Call = T::Call;
    type AdditionalSigned = ();
    type Pre = ();
    const IDENTIFIER: &'static str = "Authorize";
    fn additional_signed(&self) -> sp_std::result::Result<(), TransactionValidityError> {
        Ok(())
    }
    fn validate(
        &self,
        who: &Self::AccountId,
        call: &Self::Call,
        _info: &DispatchInfoOf<Self::Call>,
        _len: usize,
    ) -> TransactionValidity {
        let md = call.get_call_metadata();
        if <SuperAdmins<T>>::contains_key(who.clone()) {
            print("Access Granted!");
            Ok(Default::default())
        } else if <Pallet<T>>::verify_execute_access(
            who.clone(),
            md.pallet_name.as_bytes().to_vec(),
        ) {
            print("Access Granted!");
            Ok(Default::default())
        } else {
            print("Access Denied!");
            Err(InvalidTransaction::Call.into())
        }
    }
}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
