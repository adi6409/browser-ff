//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsICertStorage.idl
//


/// `interface nsICertStorageCallback : nsISupports`
///

/// ```text
/// /**
///  * Callback type used to notify callers that an operation performed by
///  * nsICertStorage has completed. Indicates the result of the requested
///  * operation, as well as any data returned by the operation.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICertStorageCallback {
    vtable: *const nsICertStorageCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICertStorageCallback.
unsafe impl XpCom for nsICertStorageCallback {
    const IID: nsIID = nsID(0x3f8fe26a, 0xa436, 0x4ad4,
        [0x9c, 0x1c, 0xa5, 0x3c, 0x60, 0x97, 0x3c, 0x31]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICertStorageCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICertStorageCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICertStorageCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsICertStorageCallback`.
    fn coerce_from(v: &nsICertStorageCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICertStorageCallbackCoerce for nsICertStorageCallback {
    #[inline]
    fn coerce_from(v: &nsICertStorageCallback) -> &Self {
        v
    }
}

impl nsICertStorageCallback {
    /// Cast this `nsICertStorageCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICertStorageCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICertStorageCallback {
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
impl<T: nsISupportsCoerce> nsICertStorageCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICertStorageCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICertStorageCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICertStorageCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] void done (in nsresult rv, in nsIVariant result); */
    pub Done: unsafe extern "system" fn (this: *const nsICertStorageCallback, rv: ::nserror::nsresult, result: *const nsIVariant) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICertStorageCallback {


    /// `[must_use] void done (in nsresult rv, in nsIVariant result);`
    #[inline]
    pub unsafe fn Done(&self, rv: ::nserror::nsresult, result: *const nsIVariant) -> ::nserror::nsresult {
        ((*self.vtable).Done)(self, rv, result)
    }


}


/// `interface nsIRevocationState : nsISupports`
///

/// ```text
/// /**
///  * A base interface for representing the revocation state of a certificate.
///  * Implementing this interface by itself is insufficient; your type must
///  * implement an inheriting interface that specifies the certificate by issuer
///  * and serial number or by subject and public key hash.
///  * Set state to nsICertStorage.STATE_UNSET to mark the certificate as not revoked.
///  * Set state to nsICertStorage.STATE_ENFORCE to mark the certificate as revoked.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIRevocationState {
    vtable: *const nsIRevocationStateVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIRevocationState.
unsafe impl XpCom for nsIRevocationState {
    const IID: nsIID = nsID(0x96db6fd7, 0x6b64, 0x4a5a,
        [0x95, 0x5d, 0x31, 0x0b, 0xd9, 0xca, 0x42, 0x34]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIRevocationState {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIRevocationState.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIRevocationStateCoerce {
    /// Cheaply cast a value of this type from a `nsIRevocationState`.
    fn coerce_from(v: &nsIRevocationState) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIRevocationStateCoerce for nsIRevocationState {
    #[inline]
    fn coerce_from(v: &nsIRevocationState) -> &Self {
        v
    }
}

impl nsIRevocationState {
    /// Cast this `nsIRevocationState` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIRevocationStateCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIRevocationState {
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
impl<T: nsISupportsCoerce> nsIRevocationStateCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRevocationState) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIRevocationState
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIRevocationStateVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute short state; */
    pub GetState: unsafe extern "system" fn (this: *const nsIRevocationState, aState: *mut i16) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIRevocationState {


    /// `readonly attribute short state;`
    #[inline]
    pub unsafe fn GetState(&self, aState: *mut i16) -> ::nserror::nsresult {
        ((*self.vtable).GetState)(self, aState)
    }


}


/// `interface nsIIssuerAndSerialRevocationState : nsIRevocationState`
///

/// ```text
/// /**
///  * An interface representing the revocation state of a certificate by issuer
///  * and serial number. Both issuer name and serial number are base64-encoded.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIIssuerAndSerialRevocationState {
    vtable: *const nsIIssuerAndSerialRevocationStateVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIIssuerAndSerialRevocationState.
unsafe impl XpCom for nsIIssuerAndSerialRevocationState {
    const IID: nsIID = nsID(0x23ce3546, 0xf1b9, 0x46f6,
        [0x8d, 0xe3, 0x77, 0x70, 0x4d, 0xa5, 0x70, 0x2f]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIIssuerAndSerialRevocationState {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIIssuerAndSerialRevocationState.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIIssuerAndSerialRevocationStateCoerce {
    /// Cheaply cast a value of this type from a `nsIIssuerAndSerialRevocationState`.
    fn coerce_from(v: &nsIIssuerAndSerialRevocationState) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIIssuerAndSerialRevocationStateCoerce for nsIIssuerAndSerialRevocationState {
    #[inline]
    fn coerce_from(v: &nsIIssuerAndSerialRevocationState) -> &Self {
        v
    }
}

impl nsIIssuerAndSerialRevocationState {
    /// Cast this `nsIIssuerAndSerialRevocationState` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIIssuerAndSerialRevocationStateCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIIssuerAndSerialRevocationState {
    type Target = nsIRevocationState;
    #[inline]
    fn deref(&self) -> &nsIRevocationState {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIRevocationStateCoerce> nsIIssuerAndSerialRevocationStateCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIIssuerAndSerialRevocationState) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIIssuerAndSerialRevocationState
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIIssuerAndSerialRevocationStateVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIRevocationStateVTable,

    /* readonly attribute ACString issuer; */
    pub GetIssuer: unsafe extern "system" fn (this: *const nsIIssuerAndSerialRevocationState, aIssuer: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString serial; */
    pub GetSerial: unsafe extern "system" fn (this: *const nsIIssuerAndSerialRevocationState, aSerial: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIIssuerAndSerialRevocationState {


    /// `readonly attribute ACString issuer;`
    #[inline]
    pub unsafe fn GetIssuer(&self, aIssuer: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetIssuer)(self, aIssuer)
    }



    /// `readonly attribute ACString serial;`
    #[inline]
    pub unsafe fn GetSerial(&self, aSerial: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetSerial)(self, aSerial)
    }


}


/// `interface nsISubjectAndPubKeyRevocationState : nsIRevocationState`
///

/// ```text
/// /**
///  * An interface representing the revocation state of a certificate by subject
///  * and pub key hash (the hash algorithm should be SHA-256). Both subject name
///  * and public key hash are base64-encoded.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISubjectAndPubKeyRevocationState {
    vtable: *const nsISubjectAndPubKeyRevocationStateVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISubjectAndPubKeyRevocationState.
unsafe impl XpCom for nsISubjectAndPubKeyRevocationState {
    const IID: nsIID = nsID(0xe78b51b4, 0x6fa4, 0x41e2,
        [0x92, 0xce, 0xe9, 0x40, 0x4f, 0x54, 0x1e, 0x96]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISubjectAndPubKeyRevocationState {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISubjectAndPubKeyRevocationState.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISubjectAndPubKeyRevocationStateCoerce {
    /// Cheaply cast a value of this type from a `nsISubjectAndPubKeyRevocationState`.
    fn coerce_from(v: &nsISubjectAndPubKeyRevocationState) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISubjectAndPubKeyRevocationStateCoerce for nsISubjectAndPubKeyRevocationState {
    #[inline]
    fn coerce_from(v: &nsISubjectAndPubKeyRevocationState) -> &Self {
        v
    }
}

impl nsISubjectAndPubKeyRevocationState {
    /// Cast this `nsISubjectAndPubKeyRevocationState` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISubjectAndPubKeyRevocationStateCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISubjectAndPubKeyRevocationState {
    type Target = nsIRevocationState;
    #[inline]
    fn deref(&self) -> &nsIRevocationState {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIRevocationStateCoerce> nsISubjectAndPubKeyRevocationStateCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISubjectAndPubKeyRevocationState) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISubjectAndPubKeyRevocationState
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISubjectAndPubKeyRevocationStateVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIRevocationStateVTable,

    /* readonly attribute ACString subject; */
    pub GetSubject: unsafe extern "system" fn (this: *const nsISubjectAndPubKeyRevocationState, aSubject: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString pubKey; */
    pub GetPubKey: unsafe extern "system" fn (this: *const nsISubjectAndPubKeyRevocationState, aPubKey: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISubjectAndPubKeyRevocationState {


    /// `readonly attribute ACString subject;`
    #[inline]
    pub unsafe fn GetSubject(&self, aSubject: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetSubject)(self, aSubject)
    }



    /// `readonly attribute ACString pubKey;`
    #[inline]
    pub unsafe fn GetPubKey(&self, aPubKey: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetPubKey)(self, aPubKey)
    }


}


/// `interface nsICRLiteState : nsISupports`
///

/// ```text
/// /**
///  * An interface representing the CRLite enrollment state of a certificate
///  * identified by its subject and subject public key info hash.
///  * subject is a base 64-encoded DER subject distinguished name.
///  * spkiHash is a base 64-encoded SHA-256 hash of a DER subject public key info.
///  * state is nsICertStorage.STATE_ENFORCE or STATE_UNSET, meaning the certificate
///  * is or is not enrolled in CRLite, respectively.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICRLiteState {
    vtable: *const nsICRLiteStateVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICRLiteState.
unsafe impl XpCom for nsICRLiteState {
    const IID: nsIID = nsID(0x5d0d22be, 0x185f, 0x4cf0,
        [0xb7, 0x3b, 0xc5, 0xa9, 0x11, 0x27, 0x3e, 0x77]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICRLiteState {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICRLiteState.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICRLiteStateCoerce {
    /// Cheaply cast a value of this type from a `nsICRLiteState`.
    fn coerce_from(v: &nsICRLiteState) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICRLiteStateCoerce for nsICRLiteState {
    #[inline]
    fn coerce_from(v: &nsICRLiteState) -> &Self {
        v
    }
}

impl nsICRLiteState {
    /// Cast this `nsICRLiteState` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICRLiteStateCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICRLiteState {
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
impl<T: nsISupportsCoerce> nsICRLiteStateCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICRLiteState) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICRLiteState
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICRLiteStateVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString subject; */
    pub GetSubject: unsafe extern "system" fn (this: *const nsICRLiteState, aSubject: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString spkiHash; */
    pub GetSpkiHash: unsafe extern "system" fn (this: *const nsICRLiteState, aSpkiHash: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute short state; */
    pub GetState: unsafe extern "system" fn (this: *const nsICRLiteState, aState: *mut i16) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICRLiteState {


    /// `readonly attribute ACString subject;`
    #[inline]
    pub unsafe fn GetSubject(&self, aSubject: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetSubject)(self, aSubject)
    }



    /// `readonly attribute ACString spkiHash;`
    #[inline]
    pub unsafe fn GetSpkiHash(&self, aSpkiHash: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetSpkiHash)(self, aSpkiHash)
    }



    /// `readonly attribute short state;`
    #[inline]
    pub unsafe fn GetState(&self, aState: *mut i16) -> ::nserror::nsresult {
        ((*self.vtable).GetState)(self, aState)
    }


}


/// `interface nsICertInfo : nsISupports`
///

/// ```text
/// /**
///  * An interface representing a certificate to add to storage. Consists of the
///  * base64-encoded DER bytes of the certificate (cert), the base64-encoded DER
///  * bytes of the subject distinguished name of the certificate (subject), and the
///  * trust of the certificate (one of the nsICertStorage.TRUST_* constants).
///  * (Note that this implementation does not validate that the given subject DN
    ///  * actually matches the subject DN of the certificate, nor that the given cert
    ///  * is a valid DER X.509 certificate.)
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICertInfo {
    vtable: *const nsICertInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICertInfo.
unsafe impl XpCom for nsICertInfo {
    const IID: nsIID = nsID(0x27b66f5e, 0x0faf, 0x403b,
        [0x95, 0xb4, 0xbc, 0x11, 0x69, 0x1a, 0xc5, 0x0d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICertInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICertInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICertInfoCoerce {
    /// Cheaply cast a value of this type from a `nsICertInfo`.
    fn coerce_from(v: &nsICertInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICertInfoCoerce for nsICertInfo {
    #[inline]
    fn coerce_from(v: &nsICertInfo) -> &Self {
        v
    }
}

impl nsICertInfo {
    /// Cast this `nsICertInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICertInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICertInfo {
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
impl<T: nsISupportsCoerce> nsICertInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICertInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICertInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICertInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString cert; */
    pub GetCert: unsafe extern "system" fn (this: *const nsICertInfo, aCert: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString subject; */
    pub GetSubject: unsafe extern "system" fn (this: *const nsICertInfo, aSubject: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute short trust; */
    pub GetTrust: unsafe extern "system" fn (this: *const nsICertInfo, aTrust: *mut i16) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICertInfo {


    /// `readonly attribute ACString cert;`
    #[inline]
    pub unsafe fn GetCert(&self, aCert: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetCert)(self, aCert)
    }



    /// `readonly attribute ACString subject;`
    #[inline]
    pub unsafe fn GetSubject(&self, aSubject: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetSubject)(self, aSubject)
    }



    /// `readonly attribute short trust;`
    #[inline]
    pub unsafe fn GetTrust(&self, aTrust: *mut i16) -> ::nserror::nsresult {
        ((*self.vtable).GetTrust)(self, aTrust)
    }


}


/// `interface nsICertStorage : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICertStorage {
    vtable: *const nsICertStorageVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICertStorage.
unsafe impl XpCom for nsICertStorage {
    const IID: nsIID = nsID(0x327100a7, 0x3401, 0x45ef,
        [0xb1, 0x60, 0xbf, 0x88, 0x0f, 0x10, 0x16, 0xfd]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICertStorage {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICertStorage.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICertStorageCoerce {
    /// Cheaply cast a value of this type from a `nsICertStorage`.
    fn coerce_from(v: &nsICertStorage) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICertStorageCoerce for nsICertStorage {
    #[inline]
    fn coerce_from(v: &nsICertStorage) -> &Self {
        v
    }
}

impl nsICertStorage {
    /// Cast this `nsICertStorage` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICertStorageCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICertStorage {
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
impl<T: nsISupportsCoerce> nsICertStorageCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICertStorage) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICertStorage
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICertStorageVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] void hasPriorData (in octet type, in nsICertStorageCallback callback); */
    pub HasPriorData: unsafe extern "system" fn (this: *const nsICertStorage, type_: u8, callback: *const nsICertStorageCallback) -> ::nserror::nsresult,

    /* [must_use] void setRevocations (in Array<nsIRevocationState> revocations, in nsICertStorageCallback callback); */
    pub SetRevocations: unsafe extern "system" fn (this: *const nsICertStorage, revocations: *const thin_vec::ThinVec<RefPtr<nsIRevocationState>>, callback: *const nsICertStorageCallback) -> ::nserror::nsresult,

    /* [must_use] short getRevocationState (in Array<octet> issuer, in Array<octet> serial, in Array<octet> subject, in Array<octet> pubkey); */
    pub GetRevocationState: unsafe extern "system" fn (this: *const nsICertStorage, issuer: *const thin_vec::ThinVec<u8>, serial: *const thin_vec::ThinVec<u8>, subject: *const thin_vec::ThinVec<u8>, pubkey: *const thin_vec::ThinVec<u8>, _retval: *mut i16) -> ::nserror::nsresult,

    /* [must_use] boolean isBlocklistFresh (); */
    pub IsBlocklistFresh: unsafe extern "system" fn (this: *const nsICertStorage, _retval: *mut bool) -> ::nserror::nsresult,

    /* [must_use] void setCRLiteState (in Array<nsICRLiteState> crliteState, in nsICertStorageCallback callback); */
    pub SetCRLiteState: unsafe extern "system" fn (this: *const nsICertStorage, crliteState: *const thin_vec::ThinVec<RefPtr<nsICRLiteState>>, callback: *const nsICertStorageCallback) -> ::nserror::nsresult,

    /* [must_use] short getCRLiteState (in Array<octet> subject, in Array<octet> spki); */
    pub GetCRLiteState: unsafe extern "system" fn (this: *const nsICertStorage, subject: *const thin_vec::ThinVec<u8>, spki: *const thin_vec::ThinVec<u8>, _retval: *mut i16) -> ::nserror::nsresult,

    /* [must_use] void setFullCRLiteFilter (in Array<octet> filter, in uint64_t timestamp, in nsICertStorageCallback callback); */
    pub SetFullCRLiteFilter: unsafe extern "system" fn (this: *const nsICertStorage, filter: *const thin_vec::ThinVec<u8>, timestamp: uint64_t, callback: *const nsICertStorageCallback) -> ::nserror::nsresult,

    /* [must_use] short getCRLiteRevocationState (in Array<octet> issuer, in Array<octet> issuerSPKI, in Array<octet> serialNumber, out uint64_t validBefore); */
    pub GetCRLiteRevocationState: unsafe extern "system" fn (this: *const nsICertStorage, issuer: *const thin_vec::ThinVec<u8>, issuerSPKI: *const thin_vec::ThinVec<u8>, serialNumber: *const thin_vec::ThinVec<u8>, validBefore: *mut uint64_t, _retval: *mut i16) -> ::nserror::nsresult,

    /* [must_use] void addCRLiteStash (in Array<octet> stash, in nsICertStorageCallback callback); */
    pub AddCRLiteStash: unsafe extern "system" fn (this: *const nsICertStorage, stash: *const thin_vec::ThinVec<u8>, callback: *const nsICertStorageCallback) -> ::nserror::nsresult,

    /* [must_use] bool isCertRevokedByStash (in Array<octet> issuerSPKI, in Array<octet> serialNumber); */
    pub IsCertRevokedByStash: unsafe extern "system" fn (this: *const nsICertStorage, issuerSPKI: *const thin_vec::ThinVec<u8>, serialNumber: *const thin_vec::ThinVec<u8>, _retval: *mut bool) -> ::nserror::nsresult,

    /* [must_use] void addCerts (in Array<nsICertInfo> certs, in nsICertStorageCallback callback); */
    pub AddCerts: unsafe extern "system" fn (this: *const nsICertStorage, certs: *const thin_vec::ThinVec<RefPtr<nsICertInfo>>, callback: *const nsICertStorageCallback) -> ::nserror::nsresult,

    /* [must_use] void removeCertsByHashes (in Array<ACString> hashes, in nsICertStorageCallback callback); */
    pub RemoveCertsByHashes: unsafe extern "system" fn (this: *const nsICertStorage, hashes: *const thin_vec::ThinVec<::nsstring::nsCString>, callback: *const nsICertStorageCallback) -> ::nserror::nsresult,

    /* [must_use] Array<Array<octet>> findCertsBySubject (in Array<octet> subject); */
    pub FindCertsBySubject: unsafe extern "system" fn (this: *const nsICertStorage, subject: *const thin_vec::ThinVec<u8>, _retval: *mut thin_vec::ThinVec<thin_vec::ThinVec<u8>>) -> ::nserror::nsresult,

    /* [must_use] int32_t GetRemainingOperationCount (); */
    pub GetRemainingOperationCount: unsafe extern "system" fn (this: *const nsICertStorage, _retval: *mut int32_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICertStorage {

    pub const DATA_TYPE_REVOCATION: i64 = 1;


    pub const DATA_TYPE_CERTIFICATE: i64 = 2;


    pub const DATA_TYPE_CRLITE: i64 = 3;


    pub const DATA_TYPE_CRLITE_FILTER_FULL: i64 = 4;


    pub const DATA_TYPE_CRLITE_FILTER_INCREMENTAL: i64 = 5;


    pub const STATE_UNSET: i64 = 0;


    pub const STATE_ENFORCE: i64 = 1;


    pub const STATE_NOT_ENROLLED: i64 = 2;

    /// ```text
    /// /**
    ///    * Trust flags to use when adding a adding a certificate.
    ///    * TRUST_INHERIT indicates a certificate inherits trust from another
    ///    * certificate.
    ///    * TRUST_ANCHOR indicates the certificate is a root of trust.
    ///    */
    /// ```
    ///

    pub const TRUST_INHERIT: i64 = 0;


    pub const TRUST_ANCHOR: i64 = 1;

    /// ```text
    /// /**
    ///    * Asynchronously check if the backing storage has stored data of the given
    ///    * type in the past. This is useful if the backing storage may have had to
    ///    * have been deleted and recreated (as in bug 1546361 when we discovered that
        ///    * moving from a 32-bit binary to a 64-bit binary caused the DB to become
        ///    * unreadable, thus necessitating its deletion and recreation).
    ///    */
    /// ```
    ///

    /// `[must_use] void hasPriorData (in octet type, in nsICertStorageCallback callback);`
    #[inline]
    pub unsafe fn HasPriorData(&self, type_: u8, callback: *const nsICertStorageCallback) -> ::nserror::nsresult {
        ((*self.vtable).HasPriorData)(self, type_, callback)
    }


    /// ```text
    /// /**
    ///    * Asynchronously set the revocation states of a set of certificates.
    ///    * The given callback is called with the result of the operation when it
    ///    * completes.
    ///    * Must only be called from the main thread.
    ///    */
    /// ```
    ///

    /// `[must_use] void setRevocations (in Array<nsIRevocationState> revocations, in nsICertStorageCallback callback);`
    #[inline]
    pub unsafe fn SetRevocations(&self, revocations: *const thin_vec::ThinVec<RefPtr<nsIRevocationState>>, callback: *const nsICertStorageCallback) -> ::nserror::nsresult {
        ((*self.vtable).SetRevocations)(self, revocations, callback)
    }


    /// ```text
    /// /**
    ///    * Get the revocation state of a certificate. STATE_UNSET indicates the
    ///    * certificate is not revoked. STATE_ENFORCE indicates the certificate is
    ///    * revoked.
    ///    * issuer - issuer name, DER encoded
    ///    * serial - serial number, DER encoded
    ///    * subject - subject name, DER encoded
    ///    * pubkey - public key, DER encoded
    ///    * Must not be called from the main thread. See bug 1541212.
    ///    */
    /// ```
    ///

    /// `[must_use] short getRevocationState (in Array<octet> issuer, in Array<octet> serial, in Array<octet> subject, in Array<octet> pubkey);`
    #[inline]
    pub unsafe fn GetRevocationState(&self, issuer: *const thin_vec::ThinVec<u8>, serial: *const thin_vec::ThinVec<u8>, subject: *const thin_vec::ThinVec<u8>, pubkey: *const thin_vec::ThinVec<u8>, _retval: *mut i16) -> ::nserror::nsresult {
        ((*self.vtable).GetRevocationState)(self, issuer, serial, subject, pubkey, _retval)
    }


    /// ```text
    /// /**
    ///     * Check that the blocklist data is current. Specifically, that the current
    ///     * time is no more than security.onecrl.maximum_staleness_in_seconds seconds
    ///     * after the last blocklist update (as stored in the
        ///     * services.blocklist.onecrl.checked pref)
    ///     */
    /// ```
    ///

    /// `[must_use] boolean isBlocklistFresh ();`
    #[inline]
    pub unsafe fn IsBlocklistFresh(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsBlocklistFresh)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Asynchronously set a batch of CRLite enrollment state. See the
    ///    * documentation for nsICRLiteState.
    ///    * Must only be called from the main thread.
    ///    */
    /// ```
    ///

    /// `[must_use] void setCRLiteState (in Array<nsICRLiteState> crliteState, in nsICertStorageCallback callback);`
    #[inline]
    pub unsafe fn SetCRLiteState(&self, crliteState: *const thin_vec::ThinVec<RefPtr<nsICRLiteState>>, callback: *const nsICertStorageCallback) -> ::nserror::nsresult {
        ((*self.vtable).SetCRLiteState)(self, crliteState, callback)
    }


    /// ```text
    /// /**
    ///    * Get the CRLite enrollment state of a certificate identified by the given
    ///    * subject distinguished name and subject public key info (both as DER bytes).
    ///    * STATE_ENFORCE indicates the certificate is enrolled, whereas STATE_UNSET
    ///    * indicates it is not.
    ///    */
    /// ```
    ///

    /// `[must_use] short getCRLiteState (in Array<octet> subject, in Array<octet> spki);`
    #[inline]
    pub unsafe fn GetCRLiteState(&self, subject: *const thin_vec::ThinVec<u8>, spki: *const thin_vec::ThinVec<u8>, _retval: *mut i16) -> ::nserror::nsresult {
        ((*self.vtable).GetCRLiteState)(self, subject, spki, _retval)
    }


    /// ```text
    /// /**
    ///    * Given the contents of a CRLite filter and its creation date as seconds since the epoch,
    ///    * replaces any existing filter with the new one. Also clears any previously-set incremental
    ///    * revocation updates ("stashes").
    ///    */
    /// ```
    ///

    /// `[must_use] void setFullCRLiteFilter (in Array<octet> filter, in uint64_t timestamp, in nsICertStorageCallback callback);`
    #[inline]
    pub unsafe fn SetFullCRLiteFilter(&self, filter: *const thin_vec::ThinVec<u8>, timestamp: uint64_t, callback: *const nsICertStorageCallback) -> ::nserror::nsresult {
        ((*self.vtable).SetFullCRLiteFilter)(self, filter, timestamp, callback)
    }


    /// ```text
    /// /**
    ///    * Given the DER-encoded issuer distinguished name, DER-encoded issuer subject public key info,
    ///    * and the bytes of the value of the serial number (so, not including the DER tag and length) of a
    ///    * certificate, returns the result of looking up the corresponding entry in the currently-saved
    ///    * CRLite filter (if any). Returns STATE_ENFORCE if the lookup indicates the corresponding
    ///    * certificate is revoked via CRLite. Returns STATE_UNSET if the lookup indicates the
    ///    * corresponding certificate is not revoked via CRLite. Returns STATE_NOT_ENROLLED if the issuer
    ///    * certificate is not enrolled in CRLite and thus no lookup was made. Also returns the timestamp
    ///    * before which lookups will be valid. That is, if a certificate has a notBefore value after the
    ///    * returned filter timestamp, the lookup is not trustworthy because the certificate may have been
    ///    * created after the filter, and thus it may cause a false negative or positive. If no filter is
    ///    * available (it may not have been downloaded yet), validBefore will be 0. The timestamp is
    ///    * represented as seconds since the epoch (i.e. it returns what was most recently set via
        ///    * setFullCRLiteFilter).
    ///    */
    /// ```
    ///

    /// `[must_use] short getCRLiteRevocationState (in Array<octet> issuer, in Array<octet> issuerSPKI, in Array<octet> serialNumber, out uint64_t validBefore);`
    #[inline]
    pub unsafe fn GetCRLiteRevocationState(&self, issuer: *const thin_vec::ThinVec<u8>, issuerSPKI: *const thin_vec::ThinVec<u8>, serialNumber: *const thin_vec::ThinVec<u8>, validBefore: *mut uint64_t, _retval: *mut i16) -> ::nserror::nsresult {
        ((*self.vtable).GetCRLiteRevocationState)(self, issuer, issuerSPKI, serialNumber, validBefore, _retval)
    }


    /// ```text
    /// /**
    ///    * Given the contents of a CRLite incremental revocation update ("stash"), adds the revocation
    ///    * information to the current set of stashed revocations. The basic unit of the stash file is an
    ///    * issuer subject public key info hash (sha-256) followed by a number of serial numbers
    ///    * corresponding to revoked certificates issued by that issuer. More specifically, each unit
    ///    * consists of:
    ///    *   4 bytes little-endian: the number of serial numbers following the issuer spki hash
    ///    *   1 byte: the length of the issuer spki hash
    ///    *   issuer spki hash length bytes: the issuer spki hash
    ///    *   as many times as the indicated serial numbers:
    ///    *     1 byte: the length of the serial number
    ///    *     serial number length bytes: the serial number
    ///    * The stash file consists of any number of these units concatenated together.
    ///    */
    /// ```
    ///

    /// `[must_use] void addCRLiteStash (in Array<octet> stash, in nsICertStorageCallback callback);`
    #[inline]
    pub unsafe fn AddCRLiteStash(&self, stash: *const thin_vec::ThinVec<u8>, callback: *const nsICertStorageCallback) -> ::nserror::nsresult {
        ((*self.vtable).AddCRLiteStash)(self, stash, callback)
    }


    /// ```text
    /// /**
    ///    * Given a DER-encoded issuer subject public key info and the bytes of the value of the serial
    ///    * number (so, not including the DER tag and length), determines if the certificate identified by
    ///    * this issuer SPKI and serial number is revoked according to the current set of stashed CRLite
    ///    * revocation information.
    ///    */
    /// ```
    ///

    /// `[must_use] bool isCertRevokedByStash (in Array<octet> issuerSPKI, in Array<octet> serialNumber);`
    #[inline]
    pub unsafe fn IsCertRevokedByStash(&self, issuerSPKI: *const thin_vec::ThinVec<u8>, serialNumber: *const thin_vec::ThinVec<u8>, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsCertRevokedByStash)(self, issuerSPKI, serialNumber, _retval)
    }


    /// ```text
    /// /**
    ///    * Asynchronously add a list of certificates to the backing storage.
    ///    * See the documentation for nsICertInfo.
    ///    * The given callback is called with the result of the operation when it
    ///    * completes.
    ///    * Must only be called from the main thread.
    ///    */
    /// ```
    ///

    /// `[must_use] void addCerts (in Array<nsICertInfo> certs, in nsICertStorageCallback callback);`
    #[inline]
    pub unsafe fn AddCerts(&self, certs: *const thin_vec::ThinVec<RefPtr<nsICertInfo>>, callback: *const nsICertStorageCallback) -> ::nserror::nsresult {
        ((*self.vtable).AddCerts)(self, certs, callback)
    }


    /// ```text
    /// /**
    ///    * Asynchronously remove the certificates with the given sha-256 hashes from
    ///    * the backing storage.
    ///    * hashes is an array of base64-encoded bytes of the sha-256 hashes of each
    ///    * certificate's bytes (DER-encoded).
    ///    * The given callback is called with the result of the operation when it
    ///    * completes.
    ///    * Must only be called from the main thread.
    ///    */
    /// ```
    ///

    /// `[must_use] void removeCertsByHashes (in Array<ACString> hashes, in nsICertStorageCallback callback);`
    #[inline]
    pub unsafe fn RemoveCertsByHashes(&self, hashes: *const thin_vec::ThinVec<::nsstring::nsCString>, callback: *const nsICertStorageCallback) -> ::nserror::nsresult {
        ((*self.vtable).RemoveCertsByHashes)(self, hashes, callback)
    }


    /// ```text
    /// /**
    ///    * Find all certificates in the backing storage with the given subject
    ///    * distinguished name.
    ///    * subject is the DER-encoded bytes of the subject distinguished name.
    ///    * Returns an array of arrays of bytes, where each inner array corresponds to
    ///    * the DER-encoded bytes of a certificate that has the given subject (although
        ///    * as these certificates were presumably added via addCertBySubject, this
        ///    * aspect is never actually valided by nsICertStorage).
    ///    * Must not be called from the main thread. See bug 1541212.
    ///    */
    /// ```
    ///

    /// `[must_use] Array<Array<octet>> findCertsBySubject (in Array<octet> subject);`
    #[inline]
    pub unsafe fn FindCertsBySubject(&self, subject: *const thin_vec::ThinVec<u8>, _retval: *mut thin_vec::ThinVec<thin_vec::ThinVec<u8>>) -> ::nserror::nsresult {
        ((*self.vtable).FindCertsBySubject)(self, subject, _retval)
    }


    /// ```text
    /// /**
    ///    * Get the count of remaining async operations. Called to ensure we don't skip
    ///    * or interrupt any operations during fast shutdown.
    ///    * Must only be called from the main thread.
    ///    */
    /// ```
    ///

    /// `[must_use] int32_t GetRemainingOperationCount ();`
    #[inline]
    pub unsafe fn GetRemainingOperationCount(&self, _retval: *mut int32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetRemainingOperationCount)(self, _retval)
    }


}


