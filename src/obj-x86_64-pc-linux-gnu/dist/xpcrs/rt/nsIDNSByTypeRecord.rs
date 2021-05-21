//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/dns/nsIDNSByTypeRecord.idl
//


/// `interface nsIDNSByTypeRecord : nsIDNSRecord`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDNSByTypeRecord {
    vtable: *const nsIDNSByTypeRecordVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDNSByTypeRecord.
unsafe impl XpCom for nsIDNSByTypeRecord {
    const IID: nsIID = nsID(0x5d13241b, 0x9d46, 0x448a,
        [0x90, 0xd8, 0x77, 0xc4, 0x18, 0x49, 0x10, 0x26]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDNSByTypeRecord {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDNSByTypeRecord.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDNSByTypeRecordCoerce {
    /// Cheaply cast a value of this type from a `nsIDNSByTypeRecord`.
    fn coerce_from(v: &nsIDNSByTypeRecord) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDNSByTypeRecordCoerce for nsIDNSByTypeRecord {
    #[inline]
    fn coerce_from(v: &nsIDNSByTypeRecord) -> &Self {
        v
    }
}

impl nsIDNSByTypeRecord {
    /// Cast this `nsIDNSByTypeRecord` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDNSByTypeRecordCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDNSByTypeRecord {
    type Target = nsIDNSRecord;
    #[inline]
    fn deref(&self) -> &nsIDNSRecord {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIDNSRecordCoerce> nsIDNSByTypeRecordCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDNSByTypeRecord) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDNSByTypeRecord
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDNSByTypeRecordVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIDNSRecordVTable,

    /* readonly attribute unsigned long type; */
    pub GetType: unsafe extern "system" fn (this: *const nsIDNSByTypeRecord, aType: *mut u32) -> ::nserror::nsresult,

    /* [noscript] readonly attribute TypeResult results; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetResults: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDNSByTypeRecord {

    /// ```text
    /// /**
    ///    * Returns DNS request type that was made for this request.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long type;`
    #[inline]
    pub unsafe fn GetType(&self, aType: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetType)(self, aType)
    }



    /// `[noscript] readonly attribute TypeResult results;`
    const _GetResults: () = ();

}


/// `interface nsIDNSTXTRecord : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDNSTXTRecord {
    vtable: *const nsIDNSTXTRecordVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDNSTXTRecord.
unsafe impl XpCom for nsIDNSTXTRecord {
    const IID: nsIID = nsID(0x2a71750d, 0xcb21, 0x45f1,
        [0x9e, 0x1c, 0x66, 0x6d, 0x18, 0xdd, 0x76, 0x45]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDNSTXTRecord {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDNSTXTRecord.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDNSTXTRecordCoerce {
    /// Cheaply cast a value of this type from a `nsIDNSTXTRecord`.
    fn coerce_from(v: &nsIDNSTXTRecord) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDNSTXTRecordCoerce for nsIDNSTXTRecord {
    #[inline]
    fn coerce_from(v: &nsIDNSTXTRecord) -> &Self {
        v
    }
}

impl nsIDNSTXTRecord {
    /// Cast this `nsIDNSTXTRecord` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDNSTXTRecordCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDNSTXTRecord {
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
impl<T: nsISupportsCoerce> nsIDNSTXTRecordCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDNSTXTRecord) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDNSTXTRecord
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDNSTXTRecordVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* CStringArrayRef getRecords (); */
    /// Unable to generate binding because `native type CopyableTArray<nsCString> unsupported`
    pub GetRecords: *const ::libc::c_void,

    /* ACString getRecordsAsOneString (); */
    pub GetRecordsAsOneString: unsafe extern "system" fn (this: *const nsIDNSTXTRecord, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDNSTXTRecord {


    /// `CStringArrayRef getRecords ();`
    const _GetRecords: () = ();


    /// `ACString getRecordsAsOneString ();`
    #[inline]
    pub unsafe fn GetRecordsAsOneString(&self, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetRecordsAsOneString)(self, _retval)
    }


}


/// `interface nsISVCParam : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISVCParam {
    vtable: *const nsISVCParamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISVCParam.
unsafe impl XpCom for nsISVCParam {
    const IID: nsIID = nsID(0x2979ceaa, 0x9c7e, 0x49de,
        [0x84, 0xb8, 0xea, 0x81, 0xc1, 0x6a, 0xeb, 0xf1]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISVCParam {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISVCParam.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISVCParamCoerce {
    /// Cheaply cast a value of this type from a `nsISVCParam`.
    fn coerce_from(v: &nsISVCParam) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISVCParamCoerce for nsISVCParam {
    #[inline]
    fn coerce_from(v: &nsISVCParam) -> &Self {
        v
    }
}

impl nsISVCParam {
    /// Cast this `nsISVCParam` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISVCParamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISVCParam {
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
impl<T: nsISupportsCoerce> nsISVCParamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISVCParam) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISVCParam
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISVCParamVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute uint16_t type; */
    pub GetType: unsafe extern "system" fn (this: *const nsISVCParam, aType: *mut uint16_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISVCParam {


    /// `readonly attribute uint16_t type;`
    #[inline]
    pub unsafe fn GetType(&self, aType: *mut uint16_t) -> ::nserror::nsresult {
        ((*self.vtable).GetType)(self, aType)
    }


}


/// `interface nsISVCParamAlpn : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISVCParamAlpn {
    vtable: *const nsISVCParamAlpnVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISVCParamAlpn.
unsafe impl XpCom for nsISVCParamAlpn {
    const IID: nsIID = nsID(0x0dc58309, 0x4d67, 0x4fc4,
        [0xa4, 0xe3, 0x38, 0xdb, 0xde, 0x9d, 0x9f, 0x4c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISVCParamAlpn {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISVCParamAlpn.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISVCParamAlpnCoerce {
    /// Cheaply cast a value of this type from a `nsISVCParamAlpn`.
    fn coerce_from(v: &nsISVCParamAlpn) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISVCParamAlpnCoerce for nsISVCParamAlpn {
    #[inline]
    fn coerce_from(v: &nsISVCParamAlpn) -> &Self {
        v
    }
}

impl nsISVCParamAlpn {
    /// Cast this `nsISVCParamAlpn` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISVCParamAlpnCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISVCParamAlpn {
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
impl<T: nsISupportsCoerce> nsISVCParamAlpnCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISVCParamAlpn) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISVCParamAlpn
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISVCParamAlpnVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute Array<ACString> alpn; */
    pub GetAlpn: unsafe extern "system" fn (this: *const nsISVCParamAlpn, aAlpn: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISVCParamAlpn {


    /// `readonly attribute Array<ACString> alpn;`
    #[inline]
    pub unsafe fn GetAlpn(&self, aAlpn: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetAlpn)(self, aAlpn)
    }


}


/// `interface nsISVCParamNoDefaultAlpn : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISVCParamNoDefaultAlpn {
    vtable: *const nsISVCParamNoDefaultAlpnVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISVCParamNoDefaultAlpn.
unsafe impl XpCom for nsISVCParamNoDefaultAlpn {
    const IID: nsIID = nsID(0xb3ed89c3, 0x2ae6, 0x4c92,
        [0x81, 0x76, 0xb7, 0x6b, 0xc2, 0x43, 0x7f, 0xcb]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISVCParamNoDefaultAlpn {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISVCParamNoDefaultAlpn.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISVCParamNoDefaultAlpnCoerce {
    /// Cheaply cast a value of this type from a `nsISVCParamNoDefaultAlpn`.
    fn coerce_from(v: &nsISVCParamNoDefaultAlpn) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISVCParamNoDefaultAlpnCoerce for nsISVCParamNoDefaultAlpn {
    #[inline]
    fn coerce_from(v: &nsISVCParamNoDefaultAlpn) -> &Self {
        v
    }
}

impl nsISVCParamNoDefaultAlpn {
    /// Cast this `nsISVCParamNoDefaultAlpn` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISVCParamNoDefaultAlpnCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISVCParamNoDefaultAlpn {
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
impl<T: nsISupportsCoerce> nsISVCParamNoDefaultAlpnCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISVCParamNoDefaultAlpn) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISVCParamNoDefaultAlpn
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISVCParamNoDefaultAlpnVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISVCParamNoDefaultAlpn {


}


/// `interface nsISVCParamPort : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISVCParamPort {
    vtable: *const nsISVCParamPortVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISVCParamPort.
unsafe impl XpCom for nsISVCParamPort {
    const IID: nsIID = nsID(0xa37c7bcb, 0xbfcd, 0x4ab4,
        [0xb8, 0x26, 0xcc, 0x58, 0x38, 0x59, 0xba, 0x73]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISVCParamPort {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISVCParamPort.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISVCParamPortCoerce {
    /// Cheaply cast a value of this type from a `nsISVCParamPort`.
    fn coerce_from(v: &nsISVCParamPort) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISVCParamPortCoerce for nsISVCParamPort {
    #[inline]
    fn coerce_from(v: &nsISVCParamPort) -> &Self {
        v
    }
}

impl nsISVCParamPort {
    /// Cast this `nsISVCParamPort` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISVCParamPortCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISVCParamPort {
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
impl<T: nsISupportsCoerce> nsISVCParamPortCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISVCParamPort) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISVCParamPort
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISVCParamPortVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute uint16_t port; */
    pub GetPort: unsafe extern "system" fn (this: *const nsISVCParamPort, aPort: *mut uint16_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISVCParamPort {


    /// `readonly attribute uint16_t port;`
    #[inline]
    pub unsafe fn GetPort(&self, aPort: *mut uint16_t) -> ::nserror::nsresult {
        ((*self.vtable).GetPort)(self, aPort)
    }


}


/// `interface nsISVCParamIPv4Hint : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISVCParamIPv4Hint {
    vtable: *const nsISVCParamIPv4HintVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISVCParamIPv4Hint.
unsafe impl XpCom for nsISVCParamIPv4Hint {
    const IID: nsIID = nsID(0xd3163d2f, 0x0bbe, 0x47d4,
        [0xbc, 0xac, 0xdb, 0x3f, 0xb1, 0x43, 0x3b, 0x39]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISVCParamIPv4Hint {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISVCParamIPv4Hint.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISVCParamIPv4HintCoerce {
    /// Cheaply cast a value of this type from a `nsISVCParamIPv4Hint`.
    fn coerce_from(v: &nsISVCParamIPv4Hint) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISVCParamIPv4HintCoerce for nsISVCParamIPv4Hint {
    #[inline]
    fn coerce_from(v: &nsISVCParamIPv4Hint) -> &Self {
        v
    }
}

impl nsISVCParamIPv4Hint {
    /// Cast this `nsISVCParamIPv4Hint` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISVCParamIPv4HintCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISVCParamIPv4Hint {
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
impl<T: nsISupportsCoerce> nsISVCParamIPv4HintCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISVCParamIPv4Hint) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISVCParamIPv4Hint
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISVCParamIPv4HintVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute Array<nsINetAddr> ipv4Hint; */
    pub GetIpv4Hint: unsafe extern "system" fn (this: *const nsISVCParamIPv4Hint, aIpv4Hint: *mut thin_vec::ThinVec<RefPtr<nsINetAddr>>) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISVCParamIPv4Hint {


    /// `readonly attribute Array<nsINetAddr> ipv4Hint;`
    #[inline]
    pub unsafe fn GetIpv4Hint(&self, aIpv4Hint: *mut thin_vec::ThinVec<RefPtr<nsINetAddr>>) -> ::nserror::nsresult {
        ((*self.vtable).GetIpv4Hint)(self, aIpv4Hint)
    }


}


/// `interface nsISVCParamEchConfig : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISVCParamEchConfig {
    vtable: *const nsISVCParamEchConfigVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISVCParamEchConfig.
unsafe impl XpCom for nsISVCParamEchConfig {
    const IID: nsIID = nsID(0x1f31e41d, 0xb6d8, 0x4796,
        [0xb1, 0x2a, 0x82, 0xef, 0x8d, 0x2b, 0x0e, 0x43]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISVCParamEchConfig {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISVCParamEchConfig.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISVCParamEchConfigCoerce {
    /// Cheaply cast a value of this type from a `nsISVCParamEchConfig`.
    fn coerce_from(v: &nsISVCParamEchConfig) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISVCParamEchConfigCoerce for nsISVCParamEchConfig {
    #[inline]
    fn coerce_from(v: &nsISVCParamEchConfig) -> &Self {
        v
    }
}

impl nsISVCParamEchConfig {
    /// Cast this `nsISVCParamEchConfig` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISVCParamEchConfigCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISVCParamEchConfig {
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
impl<T: nsISupportsCoerce> nsISVCParamEchConfigCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISVCParamEchConfig) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISVCParamEchConfig
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISVCParamEchConfigVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString echconfig; */
    pub GetEchconfig: unsafe extern "system" fn (this: *const nsISVCParamEchConfig, aEchconfig: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISVCParamEchConfig {


    /// `readonly attribute ACString echconfig;`
    #[inline]
    pub unsafe fn GetEchconfig(&self, aEchconfig: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetEchconfig)(self, aEchconfig)
    }


}


/// `interface nsISVCParamIPv6Hint : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISVCParamIPv6Hint {
    vtable: *const nsISVCParamIPv6HintVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISVCParamIPv6Hint.
unsafe impl XpCom for nsISVCParamIPv6Hint {
    const IID: nsIID = nsID(0x5100bce4, 0x9d3b, 0x42e1,
        [0xa3, 0xc9, 0x0f, 0x38, 0x6b, 0xbc, 0x9d, 0xad]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISVCParamIPv6Hint {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISVCParamIPv6Hint.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISVCParamIPv6HintCoerce {
    /// Cheaply cast a value of this type from a `nsISVCParamIPv6Hint`.
    fn coerce_from(v: &nsISVCParamIPv6Hint) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISVCParamIPv6HintCoerce for nsISVCParamIPv6Hint {
    #[inline]
    fn coerce_from(v: &nsISVCParamIPv6Hint) -> &Self {
        v
    }
}

impl nsISVCParamIPv6Hint {
    /// Cast this `nsISVCParamIPv6Hint` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISVCParamIPv6HintCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISVCParamIPv6Hint {
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
impl<T: nsISupportsCoerce> nsISVCParamIPv6HintCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISVCParamIPv6Hint) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISVCParamIPv6Hint
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISVCParamIPv6HintVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute Array<nsINetAddr> ipv6Hint; */
    pub GetIpv6Hint: unsafe extern "system" fn (this: *const nsISVCParamIPv6Hint, aIpv6Hint: *mut thin_vec::ThinVec<RefPtr<nsINetAddr>>) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISVCParamIPv6Hint {


    /// `readonly attribute Array<nsINetAddr> ipv6Hint;`
    #[inline]
    pub unsafe fn GetIpv6Hint(&self, aIpv6Hint: *mut thin_vec::ThinVec<RefPtr<nsINetAddr>>) -> ::nserror::nsresult {
        ((*self.vtable).GetIpv6Hint)(self, aIpv6Hint)
    }


}


/// `interface nsISVCParamODoHConfig : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISVCParamODoHConfig {
    vtable: *const nsISVCParamODoHConfigVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISVCParamODoHConfig.
unsafe impl XpCom for nsISVCParamODoHConfig {
    const IID: nsIID = nsID(0xbdcef040, 0x452e, 0x11eb,
        [0xb3, 0x78, 0x02, 0x42, 0xac, 0x13, 0x00, 0x02]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISVCParamODoHConfig {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISVCParamODoHConfig.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISVCParamODoHConfigCoerce {
    /// Cheaply cast a value of this type from a `nsISVCParamODoHConfig`.
    fn coerce_from(v: &nsISVCParamODoHConfig) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISVCParamODoHConfigCoerce for nsISVCParamODoHConfig {
    #[inline]
    fn coerce_from(v: &nsISVCParamODoHConfig) -> &Self {
        v
    }
}

impl nsISVCParamODoHConfig {
    /// Cast this `nsISVCParamODoHConfig` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISVCParamODoHConfigCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISVCParamODoHConfig {
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
impl<T: nsISupportsCoerce> nsISVCParamODoHConfigCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISVCParamODoHConfig) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISVCParamODoHConfig
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISVCParamODoHConfigVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString ODoHConfig; */
    pub GetODoHConfig: unsafe extern "system" fn (this: *const nsISVCParamODoHConfig, aODoHConfig: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISVCParamODoHConfig {


    /// `readonly attribute ACString ODoHConfig;`
    #[inline]
    pub unsafe fn GetODoHConfig(&self, aODoHConfig: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetODoHConfig)(self, aODoHConfig)
    }


}


/// `interface nsISVCBRecord : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISVCBRecord {
    vtable: *const nsISVCBRecordVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISVCBRecord.
unsafe impl XpCom for nsISVCBRecord {
    const IID: nsIID = nsID(0xa4da5645, 0x2160, 0x4439,
        [0xbd, 0x11, 0x54, 0x0a, 0x2d, 0x26, 0xc9, 0x89]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISVCBRecord {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISVCBRecord.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISVCBRecordCoerce {
    /// Cheaply cast a value of this type from a `nsISVCBRecord`.
    fn coerce_from(v: &nsISVCBRecord) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISVCBRecordCoerce for nsISVCBRecord {
    #[inline]
    fn coerce_from(v: &nsISVCBRecord) -> &Self {
        v
    }
}

impl nsISVCBRecord {
    /// Cast this `nsISVCBRecord` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISVCBRecordCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISVCBRecord {
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
impl<T: nsISupportsCoerce> nsISVCBRecordCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISVCBRecord) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISVCBRecord
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISVCBRecordVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute uint16_t priority; */
    pub GetPriority: unsafe extern "system" fn (this: *const nsISVCBRecord, aPriority: *mut uint16_t) -> ::nserror::nsresult,

    /* readonly attribute ACString name; */
    pub GetName: unsafe extern "system" fn (this: *const nsISVCBRecord, aName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [noscript,nostdcall,notxpcom] readonly attribute MaybePort port; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetPort: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] readonly attribute MaybeAlpnTuple alpn; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetAlpn: *const ::libc::c_void,

    /* readonly attribute ACString echConfig; */
    pub GetEchConfig: unsafe extern "system" fn (this: *const nsISVCBRecord, aEchConfig: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString ODoHConfig; */
    pub GetODoHConfig: unsafe extern "system" fn (this: *const nsISVCBRecord, aODoHConfig: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute bool hasIPHintAddress; */
    pub GetHasIPHintAddress: unsafe extern "system" fn (this: *const nsISVCBRecord, aHasIPHintAddress: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute Array<nsISVCParam> values; */
    pub GetValues: unsafe extern "system" fn (this: *const nsISVCBRecord, aValues: *mut thin_vec::ThinVec<RefPtr<nsISVCParam>>) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISVCBRecord {


    /// `readonly attribute uint16_t priority;`
    #[inline]
    pub unsafe fn GetPriority(&self, aPriority: *mut uint16_t) -> ::nserror::nsresult {
        ((*self.vtable).GetPriority)(self, aPriority)
    }



    /// `readonly attribute ACString name;`
    #[inline]
    pub unsafe fn GetName(&self, aName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetName)(self, aName)
    }



    /// `[noscript,nostdcall,notxpcom] readonly attribute MaybePort port;`
    const _GetPort: () = ();


    /// `[noscript,nostdcall,notxpcom] readonly attribute MaybeAlpnTuple alpn;`
    const _GetAlpn: () = ();


    /// `readonly attribute ACString echConfig;`
    #[inline]
    pub unsafe fn GetEchConfig(&self, aEchConfig: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetEchConfig)(self, aEchConfig)
    }



    /// `readonly attribute ACString ODoHConfig;`
    #[inline]
    pub unsafe fn GetODoHConfig(&self, aODoHConfig: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetODoHConfig)(self, aODoHConfig)
    }



    /// `readonly attribute bool hasIPHintAddress;`
    #[inline]
    pub unsafe fn GetHasIPHintAddress(&self, aHasIPHintAddress: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetHasIPHintAddress)(self, aHasIPHintAddress)
    }



    /// `readonly attribute Array<nsISVCParam> values;`
    #[inline]
    pub unsafe fn GetValues(&self, aValues: *mut thin_vec::ThinVec<RefPtr<nsISVCParam>>) -> ::nserror::nsresult {
        ((*self.vtable).GetValues)(self, aValues)
    }


}


/// `interface nsIDNSHTTPSSVCRecord : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDNSHTTPSSVCRecord {
    vtable: *const nsIDNSHTTPSSVCRecordVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDNSHTTPSSVCRecord.
unsafe impl XpCom for nsIDNSHTTPSSVCRecord {
    const IID: nsIID = nsID(0x5b649e95, 0xe0d3, 0x422b,
        [0x99, 0xa6, 0x79, 0xd7, 0x0a, 0x04, 0x13, 0x87]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDNSHTTPSSVCRecord {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDNSHTTPSSVCRecord.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDNSHTTPSSVCRecordCoerce {
    /// Cheaply cast a value of this type from a `nsIDNSHTTPSSVCRecord`.
    fn coerce_from(v: &nsIDNSHTTPSSVCRecord) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDNSHTTPSSVCRecordCoerce for nsIDNSHTTPSSVCRecord {
    #[inline]
    fn coerce_from(v: &nsIDNSHTTPSSVCRecord) -> &Self {
        v
    }
}

impl nsIDNSHTTPSSVCRecord {
    /// Cast this `nsIDNSHTTPSSVCRecord` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDNSHTTPSSVCRecordCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDNSHTTPSSVCRecord {
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
impl<T: nsISupportsCoerce> nsIDNSHTTPSSVCRecordCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDNSHTTPSSVCRecord) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDNSHTTPSSVCRecord
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDNSHTTPSSVCRecordVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute Array<nsISVCBRecord> records; */
    pub GetRecords: unsafe extern "system" fn (this: *const nsIDNSHTTPSSVCRecord, aRecords: *mut thin_vec::ThinVec<RefPtr<nsISVCBRecord>>) -> ::nserror::nsresult,

    /* nsISVCBRecord GetServiceModeRecord (in boolean aNoHttp2, in boolean aNoHttp3); */
    pub GetServiceModeRecord: unsafe extern "system" fn (this: *const nsIDNSHTTPSSVCRecord, aNoHttp2: bool, aNoHttp3: bool, _retval: *mut *const nsISVCBRecord) -> ::nserror::nsresult,

    /* readonly attribute boolean hasIPAddresses; */
    pub GetHasIPAddresses: unsafe extern "system" fn (this: *const nsIDNSHTTPSSVCRecord, aHasIPAddresses: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean allRecordsExcluded; */
    pub GetAllRecordsExcluded: unsafe extern "system" fn (this: *const nsIDNSHTTPSSVCRecord, aAllRecordsExcluded: *mut bool) -> ::nserror::nsresult,

    /* Array<nsISVCBRecord> GetAllRecordsWithEchConfig (in boolean aNoHttp2, in boolean aNoHttp3, out boolean aAllRecordsHaveEchConfig, out boolean aAllRecordsInH3ExcludedList); */
    pub GetAllRecordsWithEchConfig: unsafe extern "system" fn (this: *const nsIDNSHTTPSSVCRecord, aNoHttp2: bool, aNoHttp3: bool, aAllRecordsHaveEchConfig: *mut bool, aAllRecordsInH3ExcludedList: *mut bool, _retval: *mut thin_vec::ThinVec<RefPtr<nsISVCBRecord>>) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDNSHTTPSSVCRecord {


    /// `readonly attribute Array<nsISVCBRecord> records;`
    #[inline]
    pub unsafe fn GetRecords(&self, aRecords: *mut thin_vec::ThinVec<RefPtr<nsISVCBRecord>>) -> ::nserror::nsresult {
        ((*self.vtable).GetRecords)(self, aRecords)
    }



    /// `nsISVCBRecord GetServiceModeRecord (in boolean aNoHttp2, in boolean aNoHttp3);`
    #[inline]
    pub unsafe fn GetServiceModeRecord(&self, aNoHttp2: bool, aNoHttp3: bool, _retval: *mut *const nsISVCBRecord) -> ::nserror::nsresult {
        ((*self.vtable).GetServiceModeRecord)(self, aNoHttp2, aNoHttp3, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns true if one of SVCB records has IPv4 or IPv6 hint addresses.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean hasIPAddresses;`
    #[inline]
    pub unsafe fn GetHasIPAddresses(&self, aHasIPAddresses: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetHasIPAddresses)(self, aHasIPAddresses)
    }


    /// ```text
    /// /**
    ///    * Returns true when all names of SVCB records are in exclusion list.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean allRecordsExcluded;`
    #[inline]
    pub unsafe fn GetAllRecordsExcluded(&self, aAllRecordsExcluded: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAllRecordsExcluded)(self, aAllRecordsExcluded)
    }



    /// `Array<nsISVCBRecord> GetAllRecordsWithEchConfig (in boolean aNoHttp2, in boolean aNoHttp3, out boolean aAllRecordsHaveEchConfig, out boolean aAllRecordsInH3ExcludedList);`
    #[inline]
    pub unsafe fn GetAllRecordsWithEchConfig(&self, aNoHttp2: bool, aNoHttp3: bool, aAllRecordsHaveEchConfig: *mut bool, aAllRecordsInH3ExcludedList: *mut bool, _retval: *mut thin_vec::ThinVec<RefPtr<nsISVCBRecord>>) -> ::nserror::nsresult {
        ((*self.vtable).GetAllRecordsWithEchConfig)(self, aNoHttp2, aNoHttp3, aAllRecordsHaveEchConfig, aAllRecordsInH3ExcludedList, _retval)
    }


}


