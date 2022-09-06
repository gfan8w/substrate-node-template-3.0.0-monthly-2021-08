#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
#[cfg(test)]
mod mock {
    use crate as pallet_poe;
    use frame_support::parameter_types;
    use frame_support::traits::ConstU32;
    use frame_system as system;
    use sp_core::H256;
    use sp_runtime::{
        testing::Header,
        traits::{BlakeTwo256, IdentityLookup},
    };
    type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
    type Block = frame_system::mocking::MockBlock<Test>;
    pub const MAX_CLAIM_LENGTH: u32 = 5;
    #[doc(hidden)]
    mod sp_api_hidden_includes_construct_runtime {
        pub extern crate frame_support as hidden_include;
    }
    const _: () = {
        #[allow(unused)]
        type __hidden_use_of_unchecked_extrinsic = UncheckedExtrinsic;
    };
    pub struct Test;
    #[automatically_derived]
    impl ::core::clone::Clone for Test {
        #[inline]
        fn clone(&self) -> Test {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Test {}
    impl ::core::marker::StructuralPartialEq for Test {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Test {
        #[inline]
        fn eq(&self, other: &Test) -> bool {
            true
        }
    }
    impl ::core::marker::StructuralEq for Test {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Test {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    impl core::fmt::Debug for Test {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            fmt.debug_tuple("Test").finish()
        }
    }
    impl self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: traits :: GetNodeBlockType for Test { type NodeBlock = Block ; }
    impl self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: traits :: GetRuntimeBlockType for Test { type RuntimeBlock = Block ; }
    #[allow(non_camel_case_types)]
    pub enum Event {
        #[codec(index = 0u8)]
        System(frame_system::Event<Test>),
        #[codec(index = 1u8)]
        PoeModule(pallet_poe::Event<Test>),
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::clone::Clone for Event {
        #[inline]
        fn clone(&self) -> Event {
            match self {
                Event::System(__self_0) => Event::System(::core::clone::Clone::clone(__self_0)),
                Event::PoeModule(__self_0) => {
                    Event::PoeModule(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    #[allow(non_camel_case_types)]
    impl ::core::marker::StructuralPartialEq for Event {}
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::cmp::PartialEq for Event {
        #[inline]
        fn eq(&self, other: &Event) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (Event::System(__self_0), Event::System(__arg1_0)) => *__self_0 == *__arg1_0,
                    (Event::PoeModule(__self_0), Event::PoeModule(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
        }
    }
    #[allow(non_camel_case_types)]
    impl ::core::marker::StructuralEq for Event {}
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::cmp::Eq for Event {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<frame_system::Event<Test>>;
            let _: ::core::cmp::AssertParamIsEq<pallet_poe::Event<Test>>;
        }
    }
    const _: () = {
        #[allow(non_camel_case_types)]
        impl ::codec::Encode for Event {
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Event::System(ref aa) => {
                        __codec_dest_edqy.push_byte(0u8 as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    Event::PoeModule(ref aa) => {
                        __codec_dest_edqy.push_byte(1u8 as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    _ => (),
                }
            }
        }
        impl ::codec::EncodeLike for Event {}
    };
    const _: () = {
        #[allow(non_camel_case_types)]
        impl ::codec::Decode for Event {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| e.chain("Could not decode `Event`, failed to read variant byte"))?
                {
                    __codec_x_edqy if __codec_x_edqy == 0u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Event::System({
                            let __codec_res_edqy =
                                <frame_system::Event<Test> as ::codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `Event::System.0`"),
                                    )
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        }))
                    }
                    __codec_x_edqy if __codec_x_edqy == 1u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Event::PoeModule({
                            let __codec_res_edqy =
                                <pallet_poe::Event<Test> as ::codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `Event::PoeModule.0`"),
                                    )
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        }))
                    }
                    _ => ::core::result::Result::Err(<_ as ::core::convert::Into<_>>::into(
                        "Could not decode `Event`, variant doesn't exist",
                    )),
                }
            }
        }
    };
    impl core::fmt::Debug for Event {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self {
                Self::System(ref a0) => fmt.debug_tuple("Event::System").field(a0).finish(),
                Self::PoeModule(ref a0) => fmt.debug_tuple("Event::PoeModule").field(a0).finish(),
                _ => Ok(()),
            }
        }
    }
    impl From<frame_system::Event<Test>> for Event {
        fn from(x: frame_system::Event<Test>) -> Self {
            Event::System(x)
        }
    }
    impl
        self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::convert::TryInto<
            frame_system::Event<Test>,
        > for Event
    {
        type Error = ();        fn try_into (self) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: result :: Result < frame_system :: Event < Test > , Self :: Error >{
            match self {
                Self::System(evt) => Ok(evt),
                _ => Err(()),
            }
        }
    }
    impl From<pallet_poe::Event<Test>> for Event {
        fn from(x: pallet_poe::Event<Test>) -> Self {
            Event::PoeModule(x)
        }
    }
    impl
        self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::convert::TryInto<
            pallet_poe::Event<Test>,
        > for Event
    {
        type Error = ();        fn try_into (self) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: result :: Result < pallet_poe :: Event < Test > , Self :: Error >{
            match self {
                Self::PoeModule(evt) => Ok(evt),
                _ => Err(()),
            }
        }
    }
    pub struct Origin {
        caller: OriginCaller,
        filter: self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::rc::Rc<
            Box<dyn Fn(&<Test as frame_system::Config>::Call) -> bool>,
        >,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Origin {
        #[inline]
        fn clone(&self) -> Origin {
            Origin {
                caller: ::core::clone::Clone::clone(&self.caller),
                filter: ::core::clone::Clone::clone(&self.filter),
            }
        }
    }
    #[cfg(feature = "std")]
    impl self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::fmt::Debug for Origin {
        fn fmt (& self , fmt : & mut self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: fmt :: Formatter) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: result :: Result < () , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: fmt :: Error >{
            fmt.debug_struct("Origin")
                .field("caller", &self.caller)
                .field("filter", &"[function ptr]")
                .finish()
        }
    }
    impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OriginTrait
        for Origin
    {
        type Call = <Test as frame_system::Config>::Call;
        type PalletsOrigin = OriginCaller;
        type AccountId = <Test as frame_system::Config>::AccountId;
        fn add_filter(&mut self, filter: impl Fn(&Self::Call) -> bool + 'static) {
            let f = self.filter.clone();
            self.filter =
                self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::rc::Rc::new(
                    Box::new(move |call| f(call) && filter(call)),
                );
        }
        fn reset_filter(&mut self) {
            let filter = < < Test as frame_system :: Config > :: BaseCallFilter as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: Filter < < Test as frame_system :: Config > :: Call > > :: filter ;
            self.filter =
                self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::rc::Rc::new(
                    Box::new(filter),
                );
        }
        fn set_caller_from(&mut self, other: impl Into<Self>) {
            self.caller = other.into().caller;
        }
        fn filter_call(&self, call: &Self::Call) -> bool {
            (self.filter)(call)
        }
        fn caller(&self) -> &Self::PalletsOrigin {
            &self.caller
        }
        fn try_with_caller<R>(
            mut self,
            f: impl FnOnce(Self::PalletsOrigin) -> Result<R, Self::PalletsOrigin>,
        ) -> Result<R, Self> {
            match f(self.caller) {
                Ok(r) => Ok(r),
                Err(caller) => {
                    self.caller = caller;
                    Err(self)
                }
            }
        }
        /// Create with system none origin and `frame-system::Config::BaseCallFilter`.
        fn none() -> Self {
            frame_system::RawOrigin::None.into()
        }
        /// Create with system root origin and no filter.
        fn root() -> Self {
            frame_system::RawOrigin::Root.into()
        }
        /// Create with system signed origin and `frame-system::Config::BaseCallFilter`.
        fn signed(by: <Test as frame_system::Config>::AccountId) -> Self {
            frame_system::RawOrigin::Signed(by).into()
        }
    }
    #[allow(non_camel_case_types)]
    pub enum OriginCaller {
        #[codec(index = 0u8)]
        system(frame_system::Origin<Test>),
        #[allow(dead_code)]
        Void(self::sp_api_hidden_includes_construct_runtime::hidden_include::Void),
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::clone::Clone for OriginCaller {
        #[inline]
        fn clone(&self) -> OriginCaller {
            match self {
                OriginCaller::system(__self_0) => {
                    OriginCaller::system(::core::clone::Clone::clone(__self_0))
                }
                OriginCaller::Void(__self_0) => {
                    OriginCaller::Void(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    #[allow(non_camel_case_types)]
    impl ::core::marker::StructuralPartialEq for OriginCaller {}
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::cmp::PartialEq for OriginCaller {
        #[inline]
        fn eq(&self, other: &OriginCaller) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (OriginCaller::system(__self_0), OriginCaller::system(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (OriginCaller::Void(__self_0), OriginCaller::Void(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
        }
    }
    #[allow(non_camel_case_types)]
    impl ::core::marker::StructuralEq for OriginCaller {}
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::cmp::Eq for OriginCaller {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<frame_system::Origin<Test>>;
            let _: ::core::cmp::AssertParamIsEq<
                self::sp_api_hidden_includes_construct_runtime::hidden_include::Void,
            >;
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
        #[allow(non_camel_case_types)]
        impl ::codec::Encode for OriginCaller {
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    OriginCaller::system(ref aa) => {
                        __codec_dest_edqy.push_byte(0u8 as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    OriginCaller::Void(ref aa) => {
                        __codec_dest_edqy.push_byte(1usize as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    _ => (),
                }
            }
        }
        impl ::codec::EncodeLike for OriginCaller {}
    };
    const _: () = {
        #[allow(non_camel_case_types)]
        impl ::codec::Decode for OriginCaller {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                match __codec_input_edqy.read_byte().map_err(|e| {
                    e.chain("Could not decode `OriginCaller`, failed to read variant byte")
                })? {
                    __codec_x_edqy if __codec_x_edqy == 0u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(OriginCaller::system({
                            let __codec_res_edqy =
                                <frame_system::Origin<Test> as ::codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `OriginCaller::system.0`"),
                                    )
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        }))
                    }
                    __codec_x_edqy if __codec_x_edqy == 1usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(OriginCaller::Void({
                            let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: Void as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `OriginCaller::Void.0`"),
                                    )
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        }))
                    }
                    _ => ::core::result::Result::Err(<_ as ::core::convert::Into<_>>::into(
                        "Could not decode `OriginCaller`, variant doesn't exist",
                    )),
                }
            }
        }
    };
    #[allow(dead_code)]
    impl Origin {
        /// Create with system none origin and `frame-system::Config::BaseCallFilter`.
        pub fn none() -> Self {
            < Origin as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: OriginTrait > :: none ()
        }
        /// Create with system root origin and no filter.
        pub fn root() -> Self {
            < Origin as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: OriginTrait > :: root ()
        }
        /// Create with system signed origin and `frame-system::Config::BaseCallFilter`.
        pub fn signed(by: <Test as frame_system::Config>::AccountId) -> Self {
            < Origin as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: OriginTrait > :: signed (by)
        }
    }
    impl From<frame_system::Origin<Test>> for OriginCaller {
        fn from(x: frame_system::Origin<Test>) -> Self {
            OriginCaller::system(x)
        }
    }
    impl
        self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::convert::TryFrom<
            OriginCaller,
        > for frame_system::Origin<Test>
    {
        type Error = OriginCaller;        fn try_from (x : OriginCaller) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: result :: Result < frame_system :: Origin < Test > , OriginCaller >{
            if let OriginCaller::system(l) = x {
                Ok(l)
            } else {
                Err(x)
            }
        }
    }
    impl From<frame_system::Origin<Test>> for Origin {
        /// Convert to runtime origin:
        /// * root origin is built with no filter
        /// * others use `frame-system::Config::BaseCallFilter`
        fn from(x: frame_system::Origin<Test>) -> Self {
            let o: OriginCaller = x.into();
            o.into()
        }
    }
    impl From<OriginCaller> for Origin {
        fn from(x: OriginCaller) -> Self {
            let mut o = Origin { caller : x , filter : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: rc :: Rc :: new (Box :: new (| _ | true)) , } ;
            if !match o.caller {
                OriginCaller::system(frame_system::Origin::<Test>::Root) => true,
                _ => false,
            } {
                self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: OriginTrait :: reset_filter (& mut o) ;
            }
            o
        }
    }
    impl From<Origin>
        for self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
            frame_system::Origin<Test>,
            Origin,
        >
    {
        /// NOTE: converting to pallet origin loses the origin filter information.
        fn from(val: Origin) -> Self {
            if let OriginCaller::system(l) = val.caller {
                Ok(l)
            } else {
                Err(val)
            }
        }
    }
    impl From<Option<<Test as frame_system::Config>::AccountId>> for Origin {
        /// Convert to runtime origin with caller being system signed or none and use filter
        /// `frame-system::Config::BaseCallFilter`.
        fn from(x: Option<<Test as frame_system::Config>::AccountId>) -> Self {
            <frame_system::Origin<Test>>::from(x).into()
        }
    }
    pub type System = frame_system::Pallet<Test>;
    pub type PoeModule = pallet_poe::Pallet<Test>;
    /// All pallets included in the runtime as a nested tuple of types.
    /// Excludes the System pallet.
    pub type AllPallets = ((PoeModule,));
    /// All pallets included in the runtime as a nested tuple of types.
    pub type AllPalletsWithSystem = ((PoeModule, (System,)));
    /// All modules included in the runtime as a nested tuple of types.
    /// Excludes the System pallet.
    #[deprecated(note = "use `AllPallets` instead")]
    #[allow(dead_code)]
    pub type AllModules = ((PoeModule,));
    /// All modules included in the runtime as a nested tuple of types.
    #[deprecated(note = "use `AllPalletsWithSystem` instead")]
    #[allow(dead_code)]
    pub type AllModulesWithSystem = ((PoeModule, (System,)));
    /// Provides an implementation of `PalletInfo` to provide information
    /// about the pallet setup in the runtime.
    pub struct PalletInfo;
    impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfo
        for PalletInfo
    {
        fn index<P: 'static>() -> Option<usize> {
            let type_id = self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < P > () ;
            if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < System > () { return Some (0usize) }
            if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < PoeModule > () { return Some (1usize) }
            None
        }
        fn name<P: 'static>() -> Option<&'static str> {
            let type_id = self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < P > () ;
            if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < System > () { return Some ("System") }
            if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < PoeModule > () { return Some ("PoeModule") }
            None
        }
    }
    pub enum Call {
        # [codec (index = 0u8)] System (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < System , Test >) , # [codec (index = 1u8)] PoeModule (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < PoeModule , Test >) , }
    #[automatically_derived]
    impl ::core::clone::Clone for Call {
        #[inline]
        fn clone(&self) -> Call {
            match self {
                Call::System(__self_0) => Call::System(::core::clone::Clone::clone(__self_0)),
                Call::PoeModule(__self_0) => Call::PoeModule(::core::clone::Clone::clone(__self_0)),
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for Call {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Call {
        #[inline]
        fn eq(&self, other: &Call) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (Call::System(__self_0), Call::System(__arg1_0)) => *__self_0 == *__arg1_0,
                    (Call::PoeModule(__self_0), Call::PoeModule(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
        }
    }
    impl ::core::marker::StructuralEq for Call {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Call {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _ : :: core :: cmp :: AssertParamIsEq < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < System , Test > > ;
            let _ : :: core :: cmp :: AssertParamIsEq < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < PoeModule , Test > > ;
        }
    }
    const _: () = {
        impl ::codec::Encode for Call {
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Call::System(ref aa) => {
                        __codec_dest_edqy.push_byte(0u8 as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    Call::PoeModule(ref aa) => {
                        __codec_dest_edqy.push_byte(1u8 as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    _ => (),
                }
            }
        }
        impl ::codec::EncodeLike for Call {}
    };
    const _: () = {
        impl ::codec::Decode for Call {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| e.chain("Could not decode `Call`, failed to read variant byte"))?
                {
                    __codec_x_edqy if __codec_x_edqy == 0u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Call::System({
                            let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < System , Test > as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `Call::System.0`"),
                                    )
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        }))
                    }
                    __codec_x_edqy if __codec_x_edqy == 1u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Call::PoeModule({
                            let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < PoeModule , Test > as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `Call::PoeModule.0`"),
                                    )
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        }))
                    }
                    _ => ::core::result::Result::Err(<_ as ::core::convert::Into<_>>::into(
                        "Could not decode `Call`, variant doesn't exist",
                    )),
                }
            }
        }
    };
    impl core::fmt::Debug for Call {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self {
                Self::System(ref a0) => fmt.debug_tuple("Call::System").field(a0).finish(),
                Self::PoeModule(ref a0) => fmt.debug_tuple("Call::PoeModule").field(a0).finish(),
                _ => Ok(()),
            }
        }
    }
    impl self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::GetDispatchInfo
        for Call
    {
        fn get_dispatch_info(
            &self,
        ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::DispatchInfo
        {
            match self {
                Call::System(call) => call.get_dispatch_info(),
                Call::PoeModule(call) => call.get_dispatch_info(),
            }
        }
    }
    impl self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::GetCallMetadata
        for Call
    {
        fn get_call_metadata(
            &self,
        ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallMetadata
        {
            use self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::GetCallName;
            match self {
                Call::System(call) => {
                    let function_name = call.get_call_name();
                    let pallet_name = "System";
                    self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallMetadata { function_name , pallet_name , }
                }
                Call::PoeModule(call) => {
                    let function_name = call.get_call_name();
                    let pallet_name = "PoeModule";
                    self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallMetadata { function_name , pallet_name , }
                }
            }
        }
        fn get_module_names() -> &'static [&'static str] {
            &["System", "PoeModule"]
        }
        fn get_call_names(module: &str) -> &'static [&'static str] {
            use self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::{
                Callable, GetCallName,
            };
            match module {
                "System" => <<System as Callable<Test>>::Call as GetCallName>::get_call_names(),
                "PoeModule" => {
                    <<PoeModule as Callable<Test>>::Call as GetCallName>::get_call_names()
                }
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }
        }
    }
    impl self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::Dispatchable
        for Call
    {
        type Origin = Origin;
        type Config = Call;
        type Info =
            self::sp_api_hidden_includes_construct_runtime::hidden_include::weights::DispatchInfo;
        type PostInfo = self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: weights :: PostDispatchInfo ;        fn dispatch (self , origin : Origin) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: DispatchResultWithPostInfo{
            if ! < Self :: Origin as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: OriginTrait > :: filter_call (& origin , & self) { return self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: result :: Result :: Err (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: DispatchError :: BadOrigin . into ()) ; }
            self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (self , origin)
        }
    }
    impl self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable for Call { type Origin = Origin ; fn dispatch_bypass_filter (self , origin : Origin) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: DispatchResultWithPostInfo { match self { Call :: System (call) => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) , Call :: PoeModule (call) => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) , } } }
    impl self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: IsSubType < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < System , Test > > for Call { # [allow (unreachable_patterns)] fn is_sub_type (& self) -> Option < & self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < System , Test > > { match self { Call :: System (call) => Some (call) , _ => None , } } }
    impl From < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < System , Test > > for Call { fn from (call : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < System , Test >) -> Self { Call :: System (call) } }
    impl self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: IsSubType < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < PoeModule , Test > > for Call { # [allow (unreachable_patterns)] fn is_sub_type (& self) -> Option < & self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < PoeModule , Test > > { match self { Call :: PoeModule (call) => Some (call) , _ => None , } } }
    impl From < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < PoeModule , Test > > for Call { fn from (call : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < PoeModule , Test >) -> Self { Call :: PoeModule (call) } }
    impl Test {
        pub fn metadata () -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: RuntimeMetadataPrefixed{
            self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: RuntimeMetadataLastVersion { modules : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (& [self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleMetadata { name : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode ("System") , index : 0u8 , storage : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (frame_system :: Pallet :: < Test > :: storage_metadata))) , calls : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (frame_system :: Pallet :: < Test > :: call_functions))) , event : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (frame_system :: Event :: < Test > :: metadata))) , constants : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (frame_system :: Pallet :: < Test > :: module_constants_metadata)) , errors : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (< frame_system :: Pallet < Test > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleErrorMetadata > :: metadata)) , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleMetadata { name : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode ("PoeModule") , index : 1u8 , storage : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_poe :: Pallet :: < Test > :: storage_metadata))) , calls : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_poe :: Pallet :: < Test > :: call_functions))) , event : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_poe :: Event :: < Test > :: metadata))) , constants : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_poe :: Pallet :: < Test > :: module_constants_metadata)) , errors : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (< pallet_poe :: Pallet < Test > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleErrorMetadata > :: metadata)) , }]) , extrinsic : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ExtrinsicMetadata { version : < UncheckedExtrinsic as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: traits :: ExtrinsicMetadata > :: VERSION , signed_extensions : < < UncheckedExtrinsic as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: traits :: ExtrinsicMetadata > :: SignedExtensions as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: traits :: SignedExtension > :: identifier () . into_iter () . map (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode) . collect () , } , } . into ()
        }
    }
    #[cfg(any(feature = "std", test))]
    pub type SystemConfig = frame_system::GenesisConfig;
    #[cfg(any(feature = "std", test))]
    use self::sp_api_hidden_includes_construct_runtime::hidden_include::serde as __genesis_config_serde_import__;
    #[cfg(any(feature = "std", test))]
    #[serde(rename_all = "camelCase")]
    #[serde(deny_unknown_fields)]
    #[serde(crate = "__genesis_config_serde_import__")]
    pub struct GenesisConfig {
        pub system: SystemConfig,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use __genesis_config_serde_import__ as _serde;
        #[automatically_derived]
        impl __genesis_config_serde_import__::Serialize for GenesisConfig {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> __genesis_config_serde_import__::__private::Result<__S::Ok, __S::Error>
            where
                __S: __genesis_config_serde_import__::Serializer,
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
                    "system",
                    &self.system,
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
        use __genesis_config_serde_import__ as _serde;
        #[automatically_derived]
        impl<'de> __genesis_config_serde_import__::Deserialize<'de> for GenesisConfig {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> __genesis_config_serde_import__::__private::Result<Self, __D::Error>
            where
                __D: __genesis_config_serde_import__::Deserializer<'de>,
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
                            "system" => _serde::__private::Ok(__Field::__field0),
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
                            b"system" => _serde::__private::Ok(__Field::__field0),
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
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<GenesisConfig>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = GenesisConfig;
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
                        let __field0 = match match _serde::de::SeqAccess::next_element::<SystemConfig>(
                            &mut __seq,
                        ) {
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
                        _serde::__private::Ok(GenesisConfig { system: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<SystemConfig> =
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
                                                "system",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<SystemConfig>(
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
                                match _serde::__private::de::missing_field("system") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(GenesisConfig { system: __field0 })
                    }
                }
                const FIELDS: &'static [&'static str] = &["system"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "GenesisConfig",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<GenesisConfig>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::default::Default for GenesisConfig {
        #[inline]
        fn default() -> GenesisConfig {
            GenesisConfig {
                system: ::core::default::Default::default(),
            }
        }
    }
    #[cfg(any(feature = "std", test))]
    impl self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::BuildStorage
        for GenesisConfig
    {
        fn assimilate_storage(
            &self,
            storage : & mut self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: Storage,
        ) -> std::result::Result<(), String> {
            self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: BuildModuleGenesisStorage :: < Test , frame_system :: __InherentHiddenInstance > :: build_module_genesis_storage (& self . system , storage) ? ;
            self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: BasicExternalities :: execute_with_storage (storage , | | { < AllPalletsWithSystem as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: OnGenesis > :: on_genesis () ; }) ;
            Ok(())
        }
    }
    trait InherentDataExt {
        fn create_extrinsics (& self) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: Vec < < Block as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: BlockT > :: Extrinsic > ;
        fn check_extrinsics (& self , block : & Block) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: CheckInherentsResult ;
    }
    impl InherentDataExt
        for self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::InherentData
    {
        fn create_extrinsics (& self) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: Vec < < Block as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: BlockT > :: Extrinsic >{
            use self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::ProvideInherent;
            let mut inherents = Vec::new();
            inherents
        }        fn check_extrinsics (& self , block : & Block) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: CheckInherentsResult{
            use self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::{
                ProvideInherent, IsFatalError,
            };
            use self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::{
                IsSubType, ExtrinsicCall,
            };
            use self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::Block as _;
            let mut result = self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: CheckInherentsResult :: new () ;
            for xt in block.extrinsics() {
                if self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: Extrinsic :: is_signed (xt) . unwrap_or (false) { break }
                let mut is_inherent = false;
                if !is_inherent {
                    break;
                }
            }
            result
        }
    }
    impl self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: EnsureInherentsAreFirst < Block > for Test { fn ensure_inherents_are_first (block : & Block) -> Result < () , u32 > { use self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: ProvideInherent ; use self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: { IsSubType , ExtrinsicCall , } ; use self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: traits :: Block as _ ; let mut first_signed_observed = false ; for (i , xt) in block . extrinsics () . iter () . enumerate () { let is_signed = self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: Extrinsic :: is_signed (xt) . unwrap_or (false) ; let is_inherent = if is_signed { false } else { let mut is_inherent = false ; is_inherent } ; if ! is_inherent { first_signed_observed = true ; } if first_signed_observed && is_inherent { return Err (i as u32) } } Ok (()) } }
    impl self::sp_api_hidden_includes_construct_runtime::hidden_include::unsigned::ValidateUnsigned
        for Test
    {
        type Call = Call;        fn pre_dispatch (call : & Self :: Call) -> Result < () , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: unsigned :: TransactionValidityError >{
            #[allow(unreachable_patterns)]
            match call {
                _ => Ok(()),
            }
        }        fn validate_unsigned (# [allow (unused_variables)] source : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: unsigned :: TransactionSource , call : & Self :: Call) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: unsigned :: TransactionValidity{
            # [allow (unreachable_patterns)] match call { _ => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: unsigned :: UnknownTransaction :: NoUnsignedValidator . into () , }
        }
    }
    #[cfg(test)]
    mod __construct_runtime_integrity_test {
        use super::*;
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker]
        pub const runtime_integrity_tests: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName(
                    "mock::__construct_runtime_integrity_test::runtime_integrity_tests",
                ),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::UnitTest,
            },
            testfn: test::StaticTestFn(|| test::assert_test_result(runtime_integrity_tests())),
        };
        pub fn runtime_integrity_tests() {
            < AllPalletsWithSystem as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: IntegrityTest > :: integrity_test () ;
        }
    }
    pub struct BlockHashCount;
    impl BlockHashCount {
        /// Returns the value of this parameter type.
        #[allow(unused)]
        pub const fn get() -> u64 {
            250
        }
    }
    impl<I: From<u64>> ::frame_support::traits::Get<I> for BlockHashCount {
        fn get() -> I {
            I::from(250)
        }
    }
    pub struct SS58Prefix;
    impl SS58Prefix {
        /// Returns the value of this parameter type.
        #[allow(unused)]
        pub const fn get() -> u8 {
            42
        }
    }
    impl<I: From<u8>> ::frame_support::traits::Get<I> for SS58Prefix {
        fn get() -> I {
            I::from(42)
        }
    }
    pub struct MaxClaimLengthDefinedInMock;
    impl MaxClaimLengthDefinedInMock {
        /// Returns the value of this parameter type.
        #[allow(unused)]
        pub const fn get() -> u32 {
            5
        }
    }
    impl<I: From<u32>> ::frame_support::traits::Get<I> for MaxClaimLengthDefinedInMock {
        fn get() -> I {
            I::from(5)
        }
    }
    impl system::Config for Test {
        type BaseCallFilter = frame_support::traits::AllowAll;
        type BlockWeights = ();
        type BlockLength = ();
        type DbWeight = ();
        type Origin = Origin;
        type Call = Call;
        type Index = u64;
        type BlockNumber = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Header = Header;
        type Event = Event;
        type BlockHashCount = BlockHashCount;
        type Version = ();
        type PalletInfo = PalletInfo;
        type AccountData = ();
        type OnNewAccount = ();
        type OnKilledAccount = ();
        type SystemWeightInfo = ();
        type SS58Prefix = SS58Prefix;
        type OnSetCode = ();
    }
    impl pallet_poe::Config for Test {
        type Event = Event;
        type MaxClaimLength = ConstU32<MAX_CLAIM_LENGTH>;
        type WeightInfo = ();
    }
    pub fn new_test_ext() -> sp_io::TestExternalities {
        let mut t = system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap()
            .into();
        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}
#[cfg(test)]
mod tests {
    use crate::{Error, mock::*, Proofs};
    use frame_support::{assert_ok, assert_noop, BoundedVec};
    use super::*;
    use frame_system as system;
    use core::convert::TryFrom;
    use crate::frame_support::traits::Get;
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker]
    pub const create_claim_ok: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::create_claim_ok"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(create_claim_ok())),
    };
    fn create_claim_ok() {
        new_test_ext().execute_with(|| {
            let claim = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([0, 1]),
            );
            let is = PoeModule::create_claim(Origin::signed(1), claim.clone());
            match is {
                Ok(_) => (),
                _ => {
                    if !false {
                        {
                            ::std::rt::panic_fmt(::core::fmt::Arguments::new_v1_formatted(
                                &["Expected Ok(_). Got "],
                                &[::core::fmt::ArgumentV1::new_debug(&is)],
                                &[::core::fmt::rt::v1::Argument {
                                    position: 0usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 4u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                }],
                                unsafe { ::core::fmt::UnsafeArg::new() },
                            ))
                        }
                    }
                }
            };
            let bounded_vec =
                BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone())
                    .unwrap();
            match (
                &Proofs::<Test>::get(&bounded_vec),
                &Some((1, frame_system::Pallet::<Test>::block_number())),
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
        });
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker]
    pub const create_claim_ok_verify_event_claim_created: test::TestDescAndFn =
        test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("tests::create_claim_ok_verify_event_claim_created"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::UnitTest,
            },
            testfn: test::StaticTestFn(|| {
                test::assert_test_result(create_claim_ok_verify_event_claim_created())
            }),
        };
    fn create_claim_ok_verify_event_claim_created() {
        new_test_ext().execute_with(|| {
            let claim = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([0, 1]),
            );
            let alice: u64 = 1;
            let is = PoeModule::create_claim(Origin::signed(alice), claim.clone());
            match is {
                Ok(_) => (),
                _ => {
                    if !false {
                        {
                            ::std::rt::panic_fmt(::core::fmt::Arguments::new_v1_formatted(
                                &["Expected Ok(_). Got "],
                                &[::core::fmt::ArgumentV1::new_debug(&is)],
                                &[::core::fmt::rt::v1::Argument {
                                    position: 0usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 4u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                }],
                                unsafe { ::core::fmt::UnsafeArg::new() },
                            ))
                        }
                    }
                }
            };
            PoeModule::create_claim(Origin::signed(alice), claim.clone());
            System::assert_last_event(mock::Event::PoeModule(crate::Event::ClaimCreated(
                alice,
                claim.clone(),
            )));
            let bounded_vec =
                BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone())
                    .unwrap();
            let rest = Proofs::<Test>::try_get(&bounded_vec);
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(
                    &["", ",", "\n"],
                    &[
                        ::core::fmt::ArgumentV1::new_display(&rest.unwrap().0),
                        ::core::fmt::ArgumentV1::new_display(&rest.unwrap().1),
                    ],
                ));
            };
            match (
                &rest.unwrap(),
                &(alice, frame_system::Pallet::<Test>::block_number()),
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
        });
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker]
    pub const create_claim_failed_when_claim_already_exist: test::TestDescAndFn =
        test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("tests::create_claim_failed_when_claim_already_exist"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::UnitTest,
            },
            testfn: test::StaticTestFn(|| {
                test::assert_test_result(create_claim_failed_when_claim_already_exist())
            }),
        };
    fn create_claim_failed_when_claim_already_exist() {
        new_test_ext().execute_with(|| {
            let claim = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([0, 1]),
            );
            let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
            let h = ::frame_support::storage_root();
            match (
                &PoeModule::create_claim(Origin::signed(1), claim.clone()),
                &Err(Error::<Test>::ProofAlreadyExist.into()),
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            match (&h, &::frame_support::storage_root()) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
        });
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker]
    pub const revoke_claim_ok: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::revoke_claim_ok"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(revoke_claim_ok())),
    };
    fn revoke_claim_ok() {
        new_test_ext().execute_with(|| {
            let claim = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([0, 1]),
            );
            let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
            let is = PoeModule::revoke_claim(Origin::signed(1), claim.clone());
            match is {
                Ok(_) => (),
                _ => {
                    if !false {
                        {
                            ::std::rt::panic_fmt(::core::fmt::Arguments::new_v1_formatted(
                                &["Expected Ok(_). Got "],
                                &[::core::fmt::ArgumentV1::new_debug(&is)],
                                &[::core::fmt::rt::v1::Argument {
                                    position: 0usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 4u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                }],
                                unsafe { ::core::fmt::UnsafeArg::new() },
                            ))
                        }
                    }
                }
            };
            let bounded_vec =
                BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone())
                    .unwrap();
            match (&Proofs::<Test>::get(&bounded_vec), &None) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
        });
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker]
    pub const revoke_claim_failed_when_no_claim_exist: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::revoke_claim_failed_when_no_claim_exist"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(|| {
            test::assert_test_result(revoke_claim_failed_when_no_claim_exist())
        }),
    };
    fn revoke_claim_failed_when_no_claim_exist() {
        new_test_ext().execute_with(|| {
            let claim = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([0, 1]),
            );
            let h = ::frame_support::storage_root();
            match (
                &PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
                &Err(Error::<Test>::ProofNotExist.into()),
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            match (&h, &::frame_support::storage_root()) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
        });
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker]
    pub const transfer_claim_failed_when_no_claim_exist: test::TestDescAndFn =
        test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("tests::transfer_claim_failed_when_no_claim_exist"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::UnitTest,
            },
            testfn: test::StaticTestFn(|| {
                test::assert_test_result(transfer_claim_failed_when_no_claim_exist())
            }),
        };
    fn transfer_claim_failed_when_no_claim_exist() {
        new_test_ext().execute_with(|| {
            let claim = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([0, 1]),
            );
            let h = ::frame_support::storage_root();
            match (
                &PoeModule::transfer_claim(Origin::signed(1), 2, claim.clone()),
                &Err(Error::<Test>::ProofNotExist.into()),
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            match (&h, &::frame_support::storage_root()) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
        });
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker]
    pub const transfer_claim_to_b: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::transfer_claim_to_b"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(transfer_claim_to_b())),
    };
    fn transfer_claim_to_b() {
        new_test_ext().execute_with(|| {
            let claim = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([0, 1]),
            );
            let bounded_vec =
                BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone())
                    .unwrap();
            let user_b = 2 as u64;
            let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
            let is = PoeModule::transfer_claim(Origin::signed(1), user_b, claim.clone());
            match is {
                Ok(_) => (),
                _ => {
                    if !false {
                        {
                            ::std::rt::panic_fmt(::core::fmt::Arguments::new_v1_formatted(
                                &["Expected Ok(_). Got "],
                                &[::core::fmt::ArgumentV1::new_debug(&is)],
                                &[::core::fmt::rt::v1::Argument {
                                    position: 0usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 4u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                }],
                                unsafe { ::core::fmt::UnsafeArg::new() },
                            ))
                        }
                    }
                }
            };
            match (
                &Proofs::<Test>::get(&bounded_vec),
                &Some((2, frame_system::Pallet::<Test>::block_number())),
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
        });
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker]
    pub const transfer_claim_to_b_failed_not_owner: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::transfer_claim_to_b_failed_not_owner"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(|| {
            test::assert_test_result(transfer_claim_to_b_failed_not_owner())
        }),
    };
    fn transfer_claim_to_b_failed_not_owner() {
        new_test_ext().execute_with(|| {
            let claim = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([0, 1]),
            );
            let user_b: <Test as system::Config>::AccountId = 2;
            let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
            let h = ::frame_support::storage_root();
            match (
                &PoeModule::transfer_claim(Origin::signed(3), user_b, claim.clone()),
                &Err(Error::<Test>::NotClaimOwner.into()),
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            match (&h, &::frame_support::storage_root()) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
        });
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker]
    pub const crete_claim_failed_when_too_large_claim: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::crete_claim_failed_when_too_large_claim"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(|| {
            test::assert_test_result(crete_claim_failed_when_too_large_claim())
        }),
    };
    fn crete_claim_failed_when_too_large_claim() {
        new_test_ext().execute_with(|| {
            let aa: u32 = <Test as Config>::MaxClaimLength::get();
            let max_len = <<Test as Config>::MaxClaimLength as Get<u32>>::get();
            let claim = ::alloc::vec::from_elem(0, (max_len + 1) as usize);
            let h = ::frame_support::storage_root();
            match (
                &PoeModule::create_claim(Origin::signed(1), claim.clone()),
                &Err(Error::<Test>::ClaimTooLarge.into()),
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            match (&h, &::frame_support::storage_root()) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
        });
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker]
    pub const transfer_claim_not_claim_owner: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::transfer_claim_not_claim_owner"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(transfer_claim_not_claim_owner())),
    };
    fn transfer_claim_not_claim_owner() {
        new_test_ext().execute_with(|| {
            let claim = "hello".as_bytes().to_vec();
            let user_a: <Test as frame_system::Config>::AccountId = 1;
            let user_b: <Test as frame_system::Config>::AccountId = 2;
            let user_ghost: <Test as frame_system::Config>::AccountId = 100;
            PoeModule::create_claim(Origin::signed(user_a), claim.clone());
            let h = ::frame_support::storage_root();
            match (
                &PoeModule::transfer_claim(Origin::signed(user_ghost), user_b, claim.clone()),
                &Err(Error::<Test>::NotClaimOwner.into()),
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            match (&h, &::frame_support::storage_root()) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
        });
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker]
    pub const transfer_claim_claim_ok: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::transfer_claim_claim_ok"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(transfer_claim_claim_ok())),
    };
    fn transfer_claim_claim_ok() {
        new_test_ext().execute_with(|| {
            let claim = "hello".as_bytes().to_vec();
            let bounded_vec =
                BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone())
                    .unwrap();
            let user_a: <Test as frame_system::Config>::AccountId = 1u64;
            let user_b: <Test as frame_system::Config>::AccountId = 2u64;
            let _ = PoeModule::create_claim(Origin::signed(user_a), claim.clone());
            let is = PoeModule::transfer_claim(Origin::signed(user_a), user_b, claim.clone());
            match is {
                Ok(_) => (),
                _ => {
                    if !false {
                        {
                            ::std::rt::panic_fmt(::core::fmt::Arguments::new_v1_formatted(
                                &["Expected Ok(_). Got "],
                                &[::core::fmt::ArgumentV1::new_debug(&is)],
                                &[::core::fmt::rt::v1::Argument {
                                    position: 0usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 4u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                }],
                                unsafe { ::core::fmt::UnsafeArg::new() },
                            ))
                        }
                    }
                }
            };
            match (
                &Proofs::<Test>::get(&bounded_vec),
                &Some((user_b, frame_system::Pallet::<Test>::block_number())),
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        });
    }
}
pub mod weights {
    //! Autogenerated weights for pallet_poe
    //!
    //! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
    //! DATE: 2021-09-28, STEPS: [50, ], REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
    //! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128
    #![allow(unused_parens)]
    #![allow(unused_imports)]
    use frame_support::{
        traits::Get,
        weights::{Weight, constants::RocksDbWeight},
    };
    use sp_std::marker::PhantomData;
    use sp_std::vec::Vec;
    /// Weight functions needed for pallet_template.
    pub trait WeightInfo {
        fn create_claim_benchmark(s: Vec<u8>) -> Weight;
    }
    /// Weight functions for pallet_poe.
    pub struct SubstrateWeight<T>(PhantomData<T>);
    impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
        fn create_claim_benchmark(_s: Vec<u8>) -> Weight {
            (54_000_000 as Weight)
                .saturating_add(T::DbWeight::get().reads(1 as Weight))
                .saturating_add(T::DbWeight::get().writes(1 as Weight))
        }
    }
    impl WeightInfo for () {
        fn create_claim_benchmark(_s: Vec<u8>) -> Weight {
            (54_000_000 as Weight)
                .saturating_add(RocksDbWeight::get().reads(1 as Weight))
                .saturating_add(RocksDbWeight::get().writes(1 as Weight))
        }
    }
}
extern crate frame_support;
extern crate frame_system;
/// 一个简单的开始
/// 学习写一个 poe
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;
///
///			The module that hosts all the
///			[FRAME](https://substrate.dev/docs/en/knowledgebase/runtime/frame)
///			types needed to add this pallet to a
///			[runtime](https://substrate.dev/docs/en/knowledgebase/runtime/).
///
pub mod pallet {
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    pub use crate::weights::WeightInfo;
    use sp_std::vec::Vec;
    use core::convert::TryFrom;
    /// Configure the pallet by specifying the parameters and types on which it depends.
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type MaxClaimLength: Get<u32>;
        /// Information on runtime weights.
        type WeightInfo: WeightInfo;
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
    pub type Proofs<T: Config> = StorageMap<
        _GeneratedPrefixForStorageProofs<T>,
        Blake2_128Concat,
        BoundedVec<u8, T::MaxClaimLength>,
        (T::AccountId, T::BlockNumber),
    >;
    ///
    ///			The [event](https://substrate.dev/docs/en/knowledgebase/runtime/events) emitted
    ///			by this pallet.
    ///
    pub enum Event<T: Config> {
        ClaimCreated(T::AccountId, Vec<u8>),
        ClaimRevoked(T::AccountId, Vec<u8>),
        ClaimTransfered(T::AccountId, T::AccountId, Vec<u8>),
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
                    Self::ClaimCreated(ref _0, ref _1) => Self::ClaimCreated(
                        core::clone::Clone::clone(_0),
                        core::clone::Clone::clone(_1),
                    ),
                    Self::ClaimRevoked(ref _0, ref _1) => Self::ClaimRevoked(
                        core::clone::Clone::clone(_0),
                        core::clone::Clone::clone(_1),
                    ),
                    Self::ClaimTransfered(ref _0, ref _1, ref _2) => Self::ClaimTransfered(
                        core::clone::Clone::clone(_0),
                        core::clone::Clone::clone(_1),
                        core::clone::Clone::clone(_2),
                    ),
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
                    (Self::ClaimCreated(_0, _1), Self::ClaimCreated(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::ClaimRevoked(_0, _1), Self::ClaimRevoked(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (
                        Self::ClaimTransfered(_0, _1, _2),
                        Self::ClaimTransfered(_0_other, _1_other, _2_other),
                    ) => true && _0 == _0_other && _1 == _1_other && _2 == _2_other,
                    (Self::__Ignore(_0, _1), Self::__Ignore(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::ClaimCreated { .. }, Self::ClaimRevoked { .. }) => false,
                    (Self::ClaimCreated { .. }, Self::ClaimTransfered { .. }) => false,
                    (Self::ClaimCreated { .. }, Self::__Ignore { .. }) => false,
                    (Self::ClaimRevoked { .. }, Self::ClaimCreated { .. }) => false,
                    (Self::ClaimRevoked { .. }, Self::ClaimTransfered { .. }) => false,
                    (Self::ClaimRevoked { .. }, Self::__Ignore { .. }) => false,
                    (Self::ClaimTransfered { .. }, Self::ClaimCreated { .. }) => false,
                    (Self::ClaimTransfered { .. }, Self::ClaimRevoked { .. }) => false,
                    (Self::ClaimTransfered { .. }, Self::__Ignore { .. }) => false,
                    (Self::__Ignore { .. }, Self::ClaimCreated { .. }) => false,
                    (Self::__Ignore { .. }, Self::ClaimRevoked { .. }) => false,
                    (Self::__Ignore { .. }, Self::ClaimTransfered { .. }) => false,
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::fmt::Debug for Event<T> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                match *self {
                    Self::ClaimCreated(ref _0, ref _1) => fmt
                        .debug_tuple("Event::ClaimCreated")
                        .field(&_0)
                        .field(&_1)
                        .finish(),
                    Self::ClaimRevoked(ref _0, ref _1) => fmt
                        .debug_tuple("Event::ClaimRevoked")
                        .field(&_0)
                        .field(&_1)
                        .finish(),
                    Self::ClaimTransfered(ref _0, ref _1, ref _2) => fmt
                        .debug_tuple("Event::ClaimTransfered")
                        .field(&_0)
                        .field(&_1)
                        .field(&_2)
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
        impl<T: Config> ::codec::Encode for Event<T>
        where
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
        {
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Event::ClaimCreated(ref aa, ref ba) => {
                        __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                        ::codec::Encode::encode_to(ba, __codec_dest_edqy);
                    }
                    Event::ClaimRevoked(ref aa, ref ba) => {
                        __codec_dest_edqy.push_byte(1usize as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                        ::codec::Encode::encode_to(ba, __codec_dest_edqy);
                    }
                    Event::ClaimTransfered(ref aa, ref ba, ref ca) => {
                        __codec_dest_edqy.push_byte(2usize as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                        ::codec::Encode::encode_to(ba, __codec_dest_edqy);
                        ::codec::Encode::encode_to(ca, __codec_dest_edqy);
                    }
                    _ => (),
                }
            }
        }
        impl<T: Config> ::codec::EncodeLike for Event<T>
        where
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
        {
        }
    };
    const _: () = {
        impl<T: Config> ::codec::Decode for Event<T>
        where
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
        {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| e.chain("Could not decode `Event`, failed to read variant byte"))?
                {
                    __codec_x_edqy if __codec_x_edqy == 0usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Event::<T>::ClaimCreated(
                            {
                                let __codec_res_edqy =
                                    <T::AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::ClaimCreated.0`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            {
                                let __codec_res_edqy =
                                    <Vec<u8> as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::ClaimCreated.1`"),
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
                        ::core::result::Result::Ok(Event::<T>::ClaimRevoked(
                            {
                                let __codec_res_edqy =
                                    <T::AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::ClaimRevoked.0`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            {
                                let __codec_res_edqy =
                                    <Vec<u8> as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::ClaimRevoked.1`"),
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
                        ::core::result::Result::Ok(Event::<T>::ClaimTransfered(
                            {
                                let __codec_res_edqy =
                                    <T::AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::ClaimTransfered.0`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            {
                                let __codec_res_edqy =
                                    <T::AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::ClaimTransfered.1`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            {
                                let __codec_res_edqy =
                                    <Vec<u8> as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::ClaimTransfered.2`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        ))
                    }
                    _ => ::core::result::Result::Err(<_ as ::core::convert::Into<_>>::into(
                        "Could not decode `Event`, variant doesn't exist",
                    )),
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
        ///存证已经存在
        ProofAlreadyExist,
        ///存证不存在
        ProofNotExist,
        ///不是存证的拥有者
        NotClaimOwner,
        ///存证太长
        ClaimTooLarge,
    }
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
    impl<T: Config> Pallet<T> {
        pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            let bounded_vec = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
                .map_err(|_| Error::<T>::ClaimTooLarge)?;
            {
                if !!Proofs::<T>::contains_key(&bounded_vec) {
                    {
                        return Err(Error::<T>::ProofAlreadyExist.into());
                    };
                }
            };
            Proofs::<T>::insert(
                &bounded_vec,
                (sender.clone(), frame_system::Pallet::<T>::block_number()),
            );
            Self::deposit_event(Event::ClaimCreated(sender, claim));
            Ok(().into())
        }
        pub fn revoke_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            let bounded_vec = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
                .map_err(|_| Error::<T>::ClaimTooLarge)?;
            let (owner, _) = Proofs::<T>::get(&bounded_vec).ok_or(Error::<T>::ProofNotExist)?;
            {
                if !(owner == sender) {
                    {
                        return Err(Error::<T>::NotClaimOwner.into());
                    };
                }
            };
            Proofs::<T>::remove(&bounded_vec);
            Self::deposit_event(Event::ClaimRevoked(sender, claim));
            Ok(().into())
        }
        pub fn transfer_claim(
            origin: OriginFor<T>,
            target: T::AccountId,
            claim: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            let bounded_vec = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
                .map_err(|_| Error::<T>::ClaimTooLarge)?;
            let (owner, _) = Proofs::<T>::get(&bounded_vec).ok_or(Error::<T>::ProofNotExist)?;
            {
                if !(owner == sender) {
                    {
                        return Err(Error::<T>::NotClaimOwner.into());
                    };
                }
            };
            Proofs::<T>::remove(&bounded_vec);
            Proofs::<T>::insert(
                &bounded_vec,
                (target.clone(), frame_system::Pallet::<T>::block_number()),
            );
            Proofs::<T>::mutate(&bounded_vec, |value| {
                let mut v = value.as_mut().unwrap();
                v.0 = target.clone();
                v.1 = frame_system::Pallet::<T>::block_number();
            });
            Self::deposit_event(Event::ClaimTransfered(sender, target, claim));
            Ok(().into())
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn module_constants_metadata(
        ) -> &'static [frame_support::dispatch::ModuleConstantMetadata] {
            &[{
                #[allow(non_upper_case_types)]
                #[allow(non_camel_case_types)]
                struct MaxClaimLengthDefaultByteGetter<T>(
                    frame_support::sp_std::marker::PhantomData<(T)>,
                );
                impl<T: Config> frame_support::dispatch::DefaultByte for MaxClaimLengthDefaultByteGetter<T> {
                    fn default_byte(&self) -> frame_support::sp_std::vec::Vec<u8> {
                        let value = <T::MaxClaimLength as frame_support::traits::Get<u32>>::get();
                        frame_support::codec::Encode::encode(&value)
                    }
                }
                unsafe impl<T: Config> Send for MaxClaimLengthDefaultByteGetter<T> {}
                unsafe impl<T: Config> Sync for MaxClaimLengthDefaultByteGetter<T> {}
                frame_support::dispatch::ModuleConstantMetadata {
                    name: frame_support::dispatch::DecodeDifferent::Encode("MaxClaimLength"),
                    ty: frame_support::dispatch::DecodeDifferent::Encode("u32"),
                    value: frame_support::dispatch::DecodeDifferent::Encode(
                        frame_support::dispatch::DefaultByteGetter(
                            &MaxClaimLengthDefaultByteGetter::<T>(
                                frame_support::sp_std::marker::PhantomData,
                            ),
                        ),
                    ),
                    documentation: frame_support::dispatch::DecodeDifferent::Encode(&[]),
                }
            }]
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
                let mut storage_info = < Proofs < T > as frame_support :: traits :: PartialStorageInfoTrait > :: partial_storage_info () ;
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
        create_claim(Vec<u8>),
        revoke_claim(Vec<u8>),
        transfer_claim(T::AccountId, Vec<u8>),
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
                    Self::create_claim(ref _0) => {
                        fmt.debug_tuple("Call::create_claim").field(&_0).finish()
                    }
                    Self::revoke_claim(ref _0) => {
                        fmt.debug_tuple("Call::revoke_claim").field(&_0).finish()
                    }
                    Self::transfer_claim(ref _0, ref _1) => fmt
                        .debug_tuple("Call::transfer_claim")
                        .field(&_0)
                        .field(&_1)
                        .finish(),
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
                    Self::create_claim(ref _0) => Self::create_claim(core::clone::Clone::clone(_0)),
                    Self::revoke_claim(ref _0) => Self::revoke_claim(core::clone::Clone::clone(_0)),
                    Self::transfer_claim(ref _0, ref _1) => Self::transfer_claim(
                        core::clone::Clone::clone(_0),
                        core::clone::Clone::clone(_1),
                    ),
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
                    (Self::create_claim(_0), Self::create_claim(_0_other)) => {
                        true && _0 == _0_other
                    }
                    (Self::revoke_claim(_0), Self::revoke_claim(_0_other)) => {
                        true && _0 == _0_other
                    }
                    (Self::transfer_claim(_0, _1), Self::transfer_claim(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::__Ignore { .. }, Self::create_claim { .. }) => false,
                    (Self::__Ignore { .. }, Self::revoke_claim { .. }) => false,
                    (Self::__Ignore { .. }, Self::transfer_claim { .. }) => false,
                    (Self::create_claim { .. }, Self::__Ignore { .. }) => false,
                    (Self::create_claim { .. }, Self::revoke_claim { .. }) => false,
                    (Self::create_claim { .. }, Self::transfer_claim { .. }) => false,
                    (Self::revoke_claim { .. }, Self::__Ignore { .. }) => false,
                    (Self::revoke_claim { .. }, Self::create_claim { .. }) => false,
                    (Self::revoke_claim { .. }, Self::transfer_claim { .. }) => false,
                    (Self::transfer_claim { .. }, Self::__Ignore { .. }) => false,
                    (Self::transfer_claim { .. }, Self::create_claim { .. }) => false,
                    (Self::transfer_claim { .. }, Self::revoke_claim { .. }) => false,
                }
            }
        }
    };
    const _: () = {
        #[allow(non_camel_case_types)]
        impl<T: Config> ::codec::Encode for Call<T> {
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Call::create_claim(ref aa) => {
                        __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    Call::revoke_claim(ref aa) => {
                        __codec_dest_edqy.push_byte(1usize as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    Call::transfer_claim(ref aa, ref ba) => {
                        __codec_dest_edqy.push_byte(2usize as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                        ::codec::Encode::encode_to(ba, __codec_dest_edqy);
                    }
                    _ => (),
                }
            }
        }
        impl<T: Config> ::codec::EncodeLike for Call<T> {}
    };
    const _: () = {
        #[allow(non_camel_case_types)]
        impl<T: Config> ::codec::Decode for Call<T> {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| e.chain("Could not decode `Call`, failed to read variant byte"))?
                {
                    __codec_x_edqy if __codec_x_edqy == 0usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Call::<T>::create_claim({
                            let __codec_res_edqy =
                                <Vec<u8> as ::codec::Decode>::decode(__codec_input_edqy);
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `Call::create_claim.0`"),
                                    )
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        }))
                    }
                    __codec_x_edqy if __codec_x_edqy == 1usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Call::<T>::revoke_claim({
                            let __codec_res_edqy =
                                <Vec<u8> as ::codec::Decode>::decode(__codec_input_edqy);
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `Call::revoke_claim.0`"),
                                    )
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        }))
                    }
                    __codec_x_edqy if __codec_x_edqy == 2usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Call::<T>::transfer_claim(
                            {
                                let __codec_res_edqy =
                                    <T::AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Call::transfer_claim.0`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            {
                                let __codec_res_edqy =
                                    <Vec<u8> as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Call::transfer_claim.1`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        ))
                    }
                    _ => ::core::result::Result::Err(<_ as ::core::convert::Into<_>>::into(
                        "Could not decode `Call`, variant doesn't exist",
                    )),
                }
            }
        }
    };
    impl<T: Config> frame_support::dispatch::GetDispatchInfo for Call<T> {
        fn get_dispatch_info(&self) -> frame_support::dispatch::DispatchInfo {
            match *self {
                Self::create_claim(ref claim) => {
                    let __pallet_base_weight =
                        T::WeightInfo::create_claim_benchmark(claim.to_vec());
                    let __pallet_weight =
                        <dyn frame_support::dispatch::WeighData<(&Vec<u8>,)>>::weigh_data(
                            &__pallet_base_weight,
                            (claim,),
                        );
                    let __pallet_class = < dyn frame_support :: dispatch :: ClassifyDispatch < (& Vec < u8 > ,) > > :: classify_dispatch (& __pallet_base_weight , (claim ,)) ;
                    let __pallet_pays_fee =
                        <dyn frame_support::dispatch::PaysFee<(&Vec<u8>,)>>::pays_fee(
                            &__pallet_base_weight,
                            (claim,),
                        );
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::revoke_claim(ref claim) => {
                    let __pallet_base_weight = 0;
                    let __pallet_weight =
                        <dyn frame_support::dispatch::WeighData<(&Vec<u8>,)>>::weigh_data(
                            &__pallet_base_weight,
                            (claim,),
                        );
                    let __pallet_class = < dyn frame_support :: dispatch :: ClassifyDispatch < (& Vec < u8 > ,) > > :: classify_dispatch (& __pallet_base_weight , (claim ,)) ;
                    let __pallet_pays_fee =
                        <dyn frame_support::dispatch::PaysFee<(&Vec<u8>,)>>::pays_fee(
                            &__pallet_base_weight,
                            (claim,),
                        );
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::transfer_claim(ref target, ref claim) => {
                    let __pallet_base_weight = 0;
                    let __pallet_weight = <dyn frame_support::dispatch::WeighData<(
                        &T::AccountId,
                        &Vec<u8>,
                    )>>::weigh_data(
                        &__pallet_base_weight, (target, claim)
                    );
                    let __pallet_class = <dyn frame_support::dispatch::ClassifyDispatch<(
                        &T::AccountId,
                        &Vec<u8>,
                    )>>::classify_dispatch(
                        &__pallet_base_weight, (target, claim)
                    );
                    let __pallet_pays_fee = <dyn frame_support::dispatch::PaysFee<(
                        &T::AccountId,
                        &Vec<u8>,
                    )>>::pays_fee(
                        &__pallet_base_weight, (target, claim)
                    );
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::__Ignore(_, _) => {
                    ::core::panicking::unreachable_display(&"__Ignore cannot be used")
                }
            }
        }
    }
    impl<T: Config> frame_support::dispatch::GetCallName for Call<T> {
        fn get_call_name(&self) -> &'static str {
            match *self {
                Self::create_claim(..) => "create_claim",
                Self::revoke_claim(..) => "revoke_claim",
                Self::transfer_claim(..) => "transfer_claim",
                Self::__Ignore(_, _) => {
                    ::core::panicking::unreachable_display(&"__PhantomItem cannot be used.")
                }
            }
        }
        fn get_call_names() -> &'static [&'static str] {
            &["create_claim", "revoke_claim", "transfer_claim"]
        }
    }
    impl<T: Config> frame_support::traits::UnfilteredDispatchable for Call<T> {
        type Origin = frame_system::pallet_prelude::OriginFor<T>;
        fn dispatch_bypass_filter(
            self,
            origin: Self::Origin,
        ) -> frame_support::dispatch::DispatchResultWithPostInfo {
            match self {
                Self::create_claim(claim) => {
                    let __within_span__ = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "create_claim",
                                    "pallet_poe::pallet",
                                    ::tracing::Level::TRACE,
                                    Some("pallets/poe/src/lib.rs"),
                                    Some(25u32),
                                    Some("pallet_poe::pallet"),
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
                    <Pallet<T>>::create_claim(origin, claim)
                        .map(Into::into)
                        .map_err(Into::into)
                }
                Self::revoke_claim(claim) => {
                    let __within_span__ = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "revoke_claim",
                                    "pallet_poe::pallet",
                                    ::tracing::Level::TRACE,
                                    Some("pallets/poe/src/lib.rs"),
                                    Some(25u32),
                                    Some("pallet_poe::pallet"),
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
                    <Pallet<T>>::revoke_claim(origin, claim)
                        .map(Into::into)
                        .map_err(Into::into)
                }
                Self::transfer_claim(target, claim) => {
                    let __within_span__ = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "transfer_claim",
                                    "pallet_poe::pallet",
                                    ::tracing::Level::TRACE,
                                    Some("pallets/poe/src/lib.rs"),
                                    Some(25u32),
                                    Some("pallet_poe::pallet"),
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
                    <Pallet<T>>::transfer_claim(origin, target, claim)
                        .map(Into::into)
                        .map_err(Into::into)
                }
                Self::__Ignore(_, _) => {
                    let _ = origin;
                    ::core::panicking::unreachable_display(&"__PhantomItem cannot be used.");
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
                    name: frame_support::dispatch::DecodeDifferent::Encode("create_claim"),
                    arguments: frame_support::dispatch::DecodeDifferent::Encode(&[
                        frame_support::dispatch::FunctionArgumentMetadata {
                            name: frame_support::dispatch::DecodeDifferent::Encode("claim"),
                            ty: frame_support::dispatch::DecodeDifferent::Encode("Vec<u8>"),
                        },
                    ]),
                    documentation: frame_support::dispatch::DecodeDifferent::Encode(&[]),
                },
                frame_support::dispatch::FunctionMetadata {
                    name: frame_support::dispatch::DecodeDifferent::Encode("revoke_claim"),
                    arguments: frame_support::dispatch::DecodeDifferent::Encode(&[
                        frame_support::dispatch::FunctionArgumentMetadata {
                            name: frame_support::dispatch::DecodeDifferent::Encode("claim"),
                            ty: frame_support::dispatch::DecodeDifferent::Encode("Vec<u8>"),
                        },
                    ]),
                    documentation: frame_support::dispatch::DecodeDifferent::Encode(&[]),
                },
                frame_support::dispatch::FunctionMetadata {
                    name: frame_support::dispatch::DecodeDifferent::Encode("transfer_claim"),
                    arguments: frame_support::dispatch::DecodeDifferent::Encode(&[
                        frame_support::dispatch::FunctionArgumentMetadata {
                            name: frame_support::dispatch::DecodeDifferent::Encode("target"),
                            ty: frame_support::dispatch::DecodeDifferent::Encode("T::AccountId"),
                        },
                        frame_support::dispatch::FunctionArgumentMetadata {
                            name: frame_support::dispatch::DecodeDifferent::Encode("claim"),
                            ty: frame_support::dispatch::DecodeDifferent::Encode("Vec<u8>"),
                        },
                    ]),
                    documentation: frame_support::dispatch::DecodeDifferent::Encode(&[]),
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
                    ::core::panicking::unreachable_display(&"`__Ignore` can never be constructed")
                }
                Self::ProofAlreadyExist => 0usize as u8,
                Self::ProofNotExist => 1usize as u8,
                Self::NotClaimOwner => 2usize as u8,
                Self::ClaimTooLarge => 3usize as u8,
            }
        }
        pub fn as_str(&self) -> &'static str {
            match &self {
                Self::__Ignore(_, _) => {
                    ::core::panicking::unreachable_display(&"`__Ignore` can never be constructed")
                }
                Self::ProofAlreadyExist => "ProofAlreadyExist",
                Self::ProofNotExist => "ProofNotExist",
                Self::NotClaimOwner => "NotClaimOwner",
                Self::ClaimTooLarge => "ClaimTooLarge",
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
            &[
                frame_support::error::ErrorMetadata {
                    name: frame_support::error::DecodeDifferent::Encode("ProofAlreadyExist"),
                    documentation: frame_support::error::DecodeDifferent::Encode(&["存证已经存在"]),
                },
                frame_support::error::ErrorMetadata {
                    name: frame_support::error::DecodeDifferent::Encode("ProofNotExist"),
                    documentation: frame_support::error::DecodeDifferent::Encode(&["存证不存在"]),
                },
                frame_support::error::ErrorMetadata {
                    name: frame_support::error::DecodeDifferent::Encode("NotClaimOwner"),
                    documentation: frame_support::error::DecodeDifferent::Encode(&[
                        "不是存证的拥有者",
                    ]),
                },
                frame_support::error::ErrorMetadata {
                    name: frame_support::error::DecodeDifferent::Encode("ClaimTooLarge"),
                    documentation: frame_support::error::DecodeDifferent::Encode(&["存证太长"]),
                },
            ]
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
                    name: frame_support::event::DecodeDifferent::Encode("ClaimCreated"),
                    arguments: frame_support::event::DecodeDifferent::Encode(&[
                        "AccountId",
                        "Vec<u8>",
                    ]),
                    documentation: frame_support::event::DecodeDifferent::Encode(&[]),
                },
                frame_support::event::EventMetadata {
                    name: frame_support::event::DecodeDifferent::Encode("ClaimRevoked"),
                    arguments: frame_support::event::DecodeDifferent::Encode(&[
                        "AccountId",
                        "Vec<u8>",
                    ]),
                    documentation: frame_support::event::DecodeDifferent::Encode(&[]),
                },
                frame_support::event::EventMetadata {
                    name: frame_support::event::DecodeDifferent::Encode("ClaimTransfered"),
                    arguments: frame_support::event::DecodeDifferent::Encode(&[
                        "AccountId",
                        "AccountId",
                        "Vec<u8>",
                    ]),
                    documentation: frame_support::event::DecodeDifferent::Encode(&[]),
                },
            ]
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn storage_metadata() -> frame_support::metadata::StorageMetadata {
            frame_support :: metadata :: StorageMetadata { prefix : frame_support :: metadata :: DecodeDifferent :: Encode (< < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: name :: < Pallet < T > > () . expect ("Every active pallet has a name in the runtime; qed")) , entries : frame_support :: metadata :: DecodeDifferent :: Encode (& [frame_support :: metadata :: StorageEntryMetadata { name : frame_support :: metadata :: DecodeDifferent :: Encode (< Proofs < T > as frame_support :: storage :: types :: StorageMapMetadata > :: NAME) , modifier : < Proofs < T > as frame_support :: storage :: types :: StorageMapMetadata > :: MODIFIER , ty : frame_support :: metadata :: StorageEntryType :: Map { hasher : < Proofs < T > as frame_support :: storage :: types :: StorageMapMetadata > :: HASHER , key : frame_support :: metadata :: DecodeDifferent :: Encode ("BoundedVec<u8, T::MaxClaimLength>") , value : frame_support :: metadata :: DecodeDifferent :: Encode ("(T::AccountId, T::BlockNumber)") , unused : false , } , default : frame_support :: metadata :: DecodeDifferent :: Encode (< Proofs < T > as frame_support :: storage :: types :: StorageMapMetadata > :: DEFAULT) , documentation : frame_support :: metadata :: DecodeDifferent :: Encode (& []) , }]) , }
        }
    }
    impl<T: Config> Pallet<T> {
        pub fn proofs<KArg>(k: KArg) -> Option<(T::AccountId, T::BlockNumber)>
        where
            KArg: frame_support::codec::EncodeLike<BoundedVec<u8, T::MaxClaimLength>>,
        {
            <Proofs<T> as frame_support::storage::StorageMap<
                BoundedVec<u8, T::MaxClaimLength>,
                (T::AccountId, T::BlockNumber),
            >>::get(k)
        }
    }
    pub struct _GeneratedPrefixForStorageProofs<T>(core::marker::PhantomData<(T,)>);
    impl<T: Config> frame_support::traits::StorageInstance for _GeneratedPrefixForStorageProofs<T> {
        fn pallet_prefix() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Pallet<T>,
            >()
            .expect("Every active pallet has a name in the runtime; qed")
        }
        const STORAGE_PREFIX: &'static str = "Proofs";
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
        type Proofs;
    }
    impl<T: Config> Store for Pallet<T> {
        type Proofs = Proofs<T>;
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
                            "pallet_poe::pallet",
                            ::tracing::Level::TRACE,
                            Some("pallets/poe/src/lib.rs"),
                            Some(25u32),
                            Some("pallet_poe::pallet"),
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
                            "pallet_poe::pallet",
                            ::tracing::Level::TRACE,
                            Some("pallets/poe/src/lib.rs"),
                            Some(25u32),
                            Some("pallet_poe::pallet"),
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
                            "pallet_poe::pallet",
                            ::tracing::Level::TRACE,
                            Some("pallets/poe/src/lib.rs"),
                            Some(25u32),
                            Some("pallet_poe::pallet"),
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
                            &[::core::fmt::ArgumentV1::new_display(&pallet_name)],
                        ),
                        lvl,
                        &(
                            frame_support::LOG_TARGET,
                            "pallet_poe::pallet",
                            "pallets/poe/src/lib.rs",
                            25u32,
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
    #[doc(hidden)]
    pub mod __substrate_genesis_config_check {
        #[doc(hidden)]
        pub use __is_genesis_config_defined_3 as is_genesis_config_defined;
        #[doc(hidden)]
        pub use __is_std_enabled_for_genesis_3 as is_std_enabled_for_genesis;
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
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[
        &runtime_integrity_tests,
        &create_claim_ok,
        &create_claim_ok_verify_event_claim_created,
        &create_claim_failed_when_claim_already_exist,
        &revoke_claim_ok,
        &revoke_claim_failed_when_no_claim_exist,
        &transfer_claim_failed_when_no_claim_exist,
        &transfer_claim_to_b,
        &transfer_claim_to_b_failed_not_owner,
        &crete_claim_failed_when_too_large_claim,
        &transfer_claim_not_claim_owner,
        &transfer_claim_claim_ok,
    ])
}
