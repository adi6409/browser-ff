//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/js/xpconnect/idl/nsIXPConnect.idl
//


/// `interface nsIXPConnectJSObjectHolder : nsISupports`
///

/// ```text
/// /***************************************************************************/
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIXPConnectJSObjectHolder {
    vtable: *const nsIXPConnectJSObjectHolderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIXPConnectJSObjectHolder.
unsafe impl XpCom for nsIXPConnectJSObjectHolder {
    const IID: nsIID = nsID(0x73e6ff4a, 0xab99, 0x4d99,
        [0xac, 0x00, 0xba, 0x39, 0xcc, 0xb8, 0xe4, 0xd7]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIXPConnectJSObjectHolder {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIXPConnectJSObjectHolder.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIXPConnectJSObjectHolderCoerce {
    /// Cheaply cast a value of this type from a `nsIXPConnectJSObjectHolder`.
    fn coerce_from(v: &nsIXPConnectJSObjectHolder) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIXPConnectJSObjectHolderCoerce for nsIXPConnectJSObjectHolder {
    #[inline]
    fn coerce_from(v: &nsIXPConnectJSObjectHolder) -> &Self {
        v
    }
}

impl nsIXPConnectJSObjectHolder {
    /// Cast this `nsIXPConnectJSObjectHolder` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIXPConnectJSObjectHolderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIXPConnectJSObjectHolder {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsCoerce> nsIXPConnectJSObjectHolderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPConnectJSObjectHolder) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIXPConnectJSObjectHolder
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIXPConnectJSObjectHolderVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [nostdcall,notxpcom] JSObjectPtr GetJSObject (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetJSObject: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIXPConnectJSObjectHolder {


    /// `[nostdcall,notxpcom] JSObjectPtr GetJSObject ();`
    const _GetJSObject: () = ();

}


/// `interface nsIXPConnectWrappedNative : nsIXPConnectJSObjectHolder`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIXPConnectWrappedNative {
    vtable: *const nsIXPConnectWrappedNativeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIXPConnectWrappedNative.
unsafe impl XpCom for nsIXPConnectWrappedNative {
    const IID: nsIID = nsID(0xe787be29, 0xdb5d, 0x4a45,
        [0xa3, 0xd6, 0x1d, 0xe1, 0xd6, 0xb8, 0x5c, 0x30]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIXPConnectWrappedNative {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIXPConnectWrappedNative.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIXPConnectWrappedNativeCoerce {
    /// Cheaply cast a value of this type from a `nsIXPConnectWrappedNative`.
    fn coerce_from(v: &nsIXPConnectWrappedNative) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIXPConnectWrappedNativeCoerce for nsIXPConnectWrappedNative {
    #[inline]
    fn coerce_from(v: &nsIXPConnectWrappedNative) -> &Self {
        v
    }
}

impl nsIXPConnectWrappedNative {
    /// Cast this `nsIXPConnectWrappedNative` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIXPConnectWrappedNativeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIXPConnectWrappedNative {
    type Target = nsIXPConnectJSObjectHolder;
    #[inline]
    fn deref(&self) -> &nsIXPConnectJSObjectHolder {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIXPConnectJSObjectHolderCoerce> nsIXPConnectWrappedNativeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPConnectWrappedNative) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIXPConnectWrappedNative
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIXPConnectWrappedNativeVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIXPConnectJSObjectHolderVTable,

    /* void debugDump (in short depth); */
    pub DebugDump: unsafe extern "system" fn (this: *const nsIXPConnectWrappedNative, depth: i16) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIXPConnectWrappedNative {


    /// `void debugDump (in short depth);`
    #[inline]
    pub unsafe fn DebugDump(&self, depth: i16) -> ::nserror::nsresult {
        ((*self.vtable).DebugDump)(self, depth)
    }


}


/// `interface nsIXPConnectWrappedJS : nsIXPConnectJSObjectHolder`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIXPConnectWrappedJS {
    vtable: *const nsIXPConnectWrappedJSVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIXPConnectWrappedJS.
unsafe impl XpCom for nsIXPConnectWrappedJS {
    const IID: nsIID = nsID(0x3a01b0d6, 0x074b, 0x49ed,
        [0xba, 0xc3, 0x08, 0xc7, 0x63, 0x66, 0xca, 0xe4]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIXPConnectWrappedJS {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIXPConnectWrappedJS.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIXPConnectWrappedJSCoerce {
    /// Cheaply cast a value of this type from a `nsIXPConnectWrappedJS`.
    fn coerce_from(v: &nsIXPConnectWrappedJS) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIXPConnectWrappedJSCoerce for nsIXPConnectWrappedJS {
    #[inline]
    fn coerce_from(v: &nsIXPConnectWrappedJS) -> &Self {
        v
    }
}

impl nsIXPConnectWrappedJS {
    /// Cast this `nsIXPConnectWrappedJS` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIXPConnectWrappedJSCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIXPConnectWrappedJS {
    type Target = nsIXPConnectJSObjectHolder;
    #[inline]
    fn deref(&self) -> &nsIXPConnectJSObjectHolder {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIXPConnectJSObjectHolderCoerce> nsIXPConnectWrappedJSCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPConnectWrappedJS) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIXPConnectWrappedJS
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIXPConnectWrappedJSVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIXPConnectJSObjectHolderVTable,

    /* readonly attribute nsIIDPtr InterfaceIID; */
    pub GetInterfaceIID: unsafe extern "system" fn (this: *const nsIXPConnectWrappedJS, aInterfaceIID: *mut *mut nsIID) -> ::nserror::nsresult,

    /* [nostdcall,notxpcom] JSObjectPtr GetJSObjectGlobal (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetJSObjectGlobal: *const ::libc::c_void,

    /* void debugDump (in short depth); */
    pub DebugDump: unsafe extern "system" fn (this: *const nsIXPConnectWrappedJS, depth: i16) -> ::nserror::nsresult,

    /* void aggregatedQueryInterface (in nsIIDRef uuid, [iid_is (uuid), retval] out nsQIResult result); */
    pub AggregatedQueryInterface: unsafe extern "system" fn (this: *const nsIXPConnectWrappedJS, uuid: *const nsIID, result: *mut *mut libc::c_void) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIXPConnectWrappedJS {


    /// `readonly attribute nsIIDPtr InterfaceIID;`
    #[inline]
    pub unsafe fn GetInterfaceIID(&self, aInterfaceIID: *mut *mut nsIID) -> ::nserror::nsresult {
        ((*self.vtable).GetInterfaceIID)(self, aInterfaceIID)
    }



    /// `[nostdcall,notxpcom] JSObjectPtr GetJSObjectGlobal ();`
    const _GetJSObjectGlobal: () = ();


    /// `void debugDump (in short depth);`
    #[inline]
    pub unsafe fn DebugDump(&self, depth: i16) -> ::nserror::nsresult {
        ((*self.vtable).DebugDump)(self, depth)
    }



    /// `void aggregatedQueryInterface (in nsIIDRef uuid, [iid_is (uuid), retval] out nsQIResult result);`
    #[inline]
    pub unsafe fn AggregatedQueryInterface(&self, uuid: *const nsIID, result: *mut *mut libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).AggregatedQueryInterface)(self, uuid, result)
    }


}


/// `interface nsIXPConnectWrappedJSUnmarkGray : nsIXPConnectWrappedJS`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIXPConnectWrappedJSUnmarkGray {
    vtable: *const nsIXPConnectWrappedJSUnmarkGrayVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIXPConnectWrappedJSUnmarkGray.
unsafe impl XpCom for nsIXPConnectWrappedJSUnmarkGray {
    const IID: nsIID = nsID(0xc02a0ce6, 0x275f, 0x4ea1,
        [0x9c, 0x23, 0x08, 0x49, 0x48, 0x98, 0xb0, 0x70]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIXPConnectWrappedJSUnmarkGray {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIXPConnectWrappedJSUnmarkGray.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIXPConnectWrappedJSUnmarkGrayCoerce {
    /// Cheaply cast a value of this type from a `nsIXPConnectWrappedJSUnmarkGray`.
    fn coerce_from(v: &nsIXPConnectWrappedJSUnmarkGray) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIXPConnectWrappedJSUnmarkGrayCoerce for nsIXPConnectWrappedJSUnmarkGray {
    #[inline]
    fn coerce_from(v: &nsIXPConnectWrappedJSUnmarkGray) -> &Self {
        v
    }
}

impl nsIXPConnectWrappedJSUnmarkGray {
    /// Cast this `nsIXPConnectWrappedJSUnmarkGray` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIXPConnectWrappedJSUnmarkGrayCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIXPConnectWrappedJSUnmarkGray {
    type Target = nsIXPConnectWrappedJS;
    #[inline]
    fn deref(&self) -> &nsIXPConnectWrappedJS {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIXPConnectWrappedJSCoerce> nsIXPConnectWrappedJSUnmarkGrayCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPConnectWrappedJSUnmarkGray) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIXPConnectWrappedJSUnmarkGray
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIXPConnectWrappedJSUnmarkGrayVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIXPConnectWrappedJSVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIXPConnectWrappedJSUnmarkGray {


}


/// `interface nsIXPConnect : nsISupports`
///

/// ```text
/// /***************************************************************************/
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIXPConnect {
    vtable: *const nsIXPConnectVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIXPConnect.
unsafe impl XpCom for nsIXPConnect {
    const IID: nsIID = nsID(0x768507b5, 0xb981, 0x40c7,
        [0x82, 0x76, 0xf6, 0xa1, 0xda, 0x50, 0x2a, 0x24]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIXPConnect {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIXPConnect.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIXPConnectCoerce {
    /// Cheaply cast a value of this type from a `nsIXPConnect`.
    fn coerce_from(v: &nsIXPConnect) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIXPConnectCoerce for nsIXPConnect {
    #[inline]
    fn coerce_from(v: &nsIXPConnect) -> &Self {
        v
    }
}

impl nsIXPConnect {
    /// Cast this `nsIXPConnect` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIXPConnectCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIXPConnect {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsCoerce> nsIXPConnectCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPConnect) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIXPConnect
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIXPConnectVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* JSObjectPtr wrapNative (in JSContextPtr aJSContext, in JSObjectPtr aScope, in nsISupports aCOMObj, in nsIIDRef aIID); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub WrapNative: *const ::libc::c_void,

    /* void wrapNativeToJSVal (in JSContextPtr aJSContext, in JSObjectPtr aScope, in nsISupports aCOMObj, in nsWrapperCachePtr aCache, in nsIIDPtr aIID, in boolean aAllowWrapper, out jsval aVal); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub WrapNativeToJSVal: *const ::libc::c_void,

    /* void wrapJS (in JSContextPtr aJSContext, in JSObjectPtr aJSObj, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub WrapJS: *const ::libc::c_void,

    /* nsIVariant jSValToVariant (in JSContextPtr cx, in jsval aJSVal); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub JSValToVariant: *const ::libc::c_void,

    /* nsIXPConnectWrappedNative getWrappedNativeOfJSObject (in JSContextPtr aJSContext, in JSObjectPtr aJSObj); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub GetWrappedNativeOfJSObject: *const ::libc::c_void,

    /* void debugDump (in short depth); */
    pub DebugDump: unsafe extern "system" fn (this: *const nsIXPConnect, depth: i16) -> ::nserror::nsresult,

    /* void debugDumpObject (in nsISupports aCOMObj, in short depth); */
    pub DebugDumpObject: unsafe extern "system" fn (this: *const nsIXPConnect, aCOMObj: *const nsISupports, depth: i16) -> ::nserror::nsresult,

    /* void debugDumpJSStack (in boolean showArgs, in boolean showLocals, in boolean showThisProps); */
    pub DebugDumpJSStack: unsafe extern "system" fn (this: *const nsIXPConnect, showArgs: bool, showLocals: bool, showThisProps: bool) -> ::nserror::nsresult,

    /* void wrapJSAggregatedToNative (in nsISupports aOuter, in JSContextPtr aJSContext, in JSObjectPtr aJSObj, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub WrapJSAggregatedToNative: *const ::libc::c_void,

    /* jsval variantToJS (in JSContextPtr ctx, in JSObjectPtr scope, in nsIVariant value); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub VariantToJS: *const ::libc::c_void,

    /* nsIVariant JSToVariant (in JSContextPtr ctx, in jsval value); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub JSToVariant: *const ::libc::c_void,

    /* [noscript] JSObjectPtr createSandbox (in JSContextPtr cx, in nsIPrincipal principal); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub CreateSandbox: *const ::libc::c_void,

    /* [noscript] jsval evalInSandboxObject (in AString source, in string filename, in JSContextPtr cx, in JSObjectPtr sandbox); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub EvalInSandboxObject: *const ::libc::c_void,

    /* [noscript] void writeScript (in nsIObjectOutputStream aStream, in JSContextPtr aJSContext, in JSScriptPtr aJSScript); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub WriteScript: *const ::libc::c_void,

    /* [noscript] JSScriptPtr readScript (in nsIObjectInputStream aStream, in JSContextPtr aJSContext, in const_JSReadOnlyCompileOptionsRef aOptions); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub ReadScript: *const ::libc::c_void,

    /* [infallible] readonly attribute boolean isShuttingDown; */
    pub GetIsShuttingDown: unsafe extern "system" fn (this: *const nsIXPConnect, aIsShuttingDown: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIXPConnect {

    /// ```text
    /// /**
    ///     * wrapNative will create a new JSObject or return an existing one.
    ///     *
    ///     * This method now correctly deals with cases where the passed in xpcom
    ///     * object already has an associated JSObject for the cases:
    ///     *  1) The xpcom object has already been wrapped for use in the same scope
///     *     as an nsIXPConnectWrappedNative.
///     *  2) The xpcom object is in fact a nsIXPConnectWrappedJS and thus already
///     *     has an underlying JSObject.
///     *
///     * It *might* be possible to QueryInterface the nsIXPConnectJSObjectHolder
///     * returned by the method into a nsIXPConnectWrappedNative or a
///     * nsIXPConnectWrappedJS.
///     *
///     * This method will never wrap the JSObject involved in an
///     * XPCNativeWrapper before returning.
///     *
///     * Returns:
///     *    success:
///     *       NS_OK
///     *    failure:
///     *       NS_ERROR_XPC_BAD_CONVERT_NATIVE
///     *       NS_ERROR_XPC_CANT_GET_JSOBJECT_OF_DOM_OBJECT
///     *       NS_ERROR_FAILURE
///     */
/// ```
///

/// `JSObjectPtr wrapNative (in JSContextPtr aJSContext, in JSObjectPtr aScope, in nsISupports aCOMObj, in nsIIDRef aIID);`
const _WrapNative: () = ();

/// ```text
/// /**
///      * Same as wrapNative, but it returns the JSObject in aVal. C++ callers
///      * must ensure that aVal is rooted.
///      * aIID may be null, it means the same as passing in
///      * &NS_GET_IID(nsISupports) but when passing in null certain shortcuts
///      * can be taken because we know without comparing IIDs that the caller is
///      * asking for an nsISupports wrapper.
///      * If aAllowWrapper, then the returned value will be wrapped in the proper
///      * type of security wrapper on top of the XPCWrappedNative (if needed).
///      * This method doesn't push aJSContext on the context stack, so the caller
///      * is required to push it if the top of the context stack is not equal to
///      * aJSContext.
///      */
/// ```
///

/// `void wrapNativeToJSVal (in JSContextPtr aJSContext, in JSObjectPtr aScope, in nsISupports aCOMObj, in nsWrapperCachePtr aCache, in nsIIDPtr aIID, in boolean aAllowWrapper, out jsval aVal);`
const _WrapNativeToJSVal: () = ();

/// ```text
/// /**
///     * wrapJS will yield a new or previously existing xpcom interface pointer
///     * to represent the JSObject passed in.
///     *
///     * This method now correctly deals with cases where the passed in JSObject
///     * already has an associated xpcom interface for the cases:
///     *  1) The JSObject has already been wrapped as a nsIXPConnectWrappedJS.
///     *  2) The JSObject is in fact a nsIXPConnectWrappedNative and thus already
///     *     has an underlying xpcom object.
///     *  3) The JSObject is of a jsclass which supports getting the nsISupports
///     *     from the JSObject directly. This is used for idlc style objects
///     *     (e.g. DOM objects).
///     *
///     * It *might* be possible to QueryInterface the resulting interface pointer
///     * to nsIXPConnectWrappedJS.
///     *
///     * Returns:
///     *   success:
///     *     NS_OK
///     *    failure:
///     *       NS_ERROR_XPC_BAD_CONVERT_JS
///     *       NS_ERROR_FAILURE
///     */
/// ```
///

/// `void wrapJS (in JSContextPtr aJSContext, in JSObjectPtr aJSObj, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result);`
const _WrapJS: () = ();

/// ```text
/// /**
///      * Wraps the given jsval in a nsIVariant and returns the new variant.
///      */
/// ```
///

/// `nsIVariant jSValToVariant (in JSContextPtr cx, in jsval aJSVal);`
const _JSValToVariant: () = ();

/// ```text
/// /**
///     * This only succeeds if the JSObject is a nsIXPConnectWrappedNative.
///     * A new wrapper is *never* constructed.
///     */
/// ```
///

/// `nsIXPConnectWrappedNative getWrappedNativeOfJSObject (in JSContextPtr aJSContext, in JSObjectPtr aJSObj);`
const _GetWrappedNativeOfJSObject: () = ();


/// `void debugDump (in short depth);`
#[inline]
pub unsafe fn DebugDump(&self, depth: i16) -> ::nserror::nsresult {
((*self.vtable).DebugDump)(self, depth)
}



/// `void debugDumpObject (in nsISupports aCOMObj, in short depth);`
#[inline]
pub unsafe fn DebugDumpObject(&self, aCOMObj: *const nsISupports, depth: i16) -> ::nserror::nsresult {
((*self.vtable).DebugDumpObject)(self, aCOMObj, depth)
}



/// `void debugDumpJSStack (in boolean showArgs, in boolean showLocals, in boolean showThisProps);`
#[inline]
pub unsafe fn DebugDumpJSStack(&self, showArgs: bool, showLocals: bool, showThisProps: bool) -> ::nserror::nsresult {
((*self.vtable).DebugDumpJSStack)(self, showArgs, showLocals, showThisProps)
}


/// ```text
/// /**
///     * wrapJSAggregatedToNative is just like wrapJS except it is used in cases
///     * where the JSObject is also aggregated to some native xpcom Object.
///     * At present XBL is the only system that might want to do this.
///     *
///     * XXX write more!
///     *
///     * Returns:
///     *   success:
///     *     NS_OK
///     *    failure:
///     *       NS_ERROR_XPC_BAD_CONVERT_JS
///     *       NS_ERROR_FAILURE
///     */
/// ```
///

/// `void wrapJSAggregatedToNative (in nsISupports aOuter, in JSContextPtr aJSContext, in JSObjectPtr aJSObj, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result);`
const _WrapJSAggregatedToNative: () = ();


/// `jsval variantToJS (in JSContextPtr ctx, in JSObjectPtr scope, in nsIVariant value);`
const _VariantToJS: () = ();


/// `nsIVariant JSToVariant (in JSContextPtr ctx, in jsval value);`
const _JSToVariant: () = ();

/// ```text
/// /**
///      * Create a sandbox for evaluating code in isolation using
///      * evalInSandboxObject().
///      *
///      * @param cx A context to use when creating the sandbox object.
///      * @param principal The principal (or NULL to use the null principal)
///      *                  to use when evaluating code in this sandbox.
///      */
/// ```
///

/// `[noscript] JSObjectPtr createSandbox (in JSContextPtr cx, in nsIPrincipal principal);`
const _CreateSandbox: () = ();

/// ```text
/// /**
///      * Evaluate script in a sandbox, completely isolated from all
///      * other running scripts.
///      *
///      * @param source The source of the script to evaluate.
///      * @param filename The filename of the script. May be null.
///      * @param cx The context to use when setting up the evaluation of
///      *           the script. The actual evaluation will happen on a new
///      *           temporary context.
///      * @param sandbox The sandbox object to evaluate the script in.
///      * @return The result of the evaluation as a jsval. If the caller
///      *         intends to use the return value from this call the caller
///      *         is responsible for rooting the jsval before making a call
///      *         to this method.
///      */
/// ```
///

/// `[noscript] jsval evalInSandboxObject (in AString source, in string filename, in JSContextPtr cx, in JSObjectPtr sandbox);`
const _EvalInSandboxObject: () = ();


/// `[noscript] void writeScript (in nsIObjectOutputStream aStream, in JSContextPtr aJSContext, in JSScriptPtr aJSScript);`
const _WriteScript: () = ();


/// `[noscript] JSScriptPtr readScript (in nsIObjectInputStream aStream, in JSContextPtr aJSContext, in const_JSReadOnlyCompileOptionsRef aOptions);`
const _ReadScript: () = ();


/// `[infallible] readonly attribute boolean isShuttingDown;`
#[inline]
pub unsafe fn GetIsShuttingDown(&self) -> bool {
let mut result = <bool as ::std::default::Default>::default();
let _rv = ((*self.vtable).GetIsShuttingDown)(self, &mut result);
debug_assert!(_rv.succeeded());
result
}


}


