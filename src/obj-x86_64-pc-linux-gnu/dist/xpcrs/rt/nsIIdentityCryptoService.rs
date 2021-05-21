//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/services/crypto/component/nsIIdentityCryptoService.idl
//


/// `interface nsIIdentityCryptoService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIIdentityCryptoService {
    vtable: *const nsIIdentityCryptoServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIIdentityCryptoService.
unsafe impl XpCom for nsIIdentityCryptoService {
    const IID: nsIID = nsID(0xf087e6bc, 0xdd33, 0x4f6c,
        [0xa1, 0x06, 0xdd, 0x78, 0x6e, 0x05, 0x2e, 0xe9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIIdentityCryptoService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIIdentityCryptoService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIIdentityCryptoServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIIdentityCryptoService`.
    fn coerce_from(v: &nsIIdentityCryptoService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIIdentityCryptoServiceCoerce for nsIIdentityCryptoService {
    #[inline]
    fn coerce_from(v: &nsIIdentityCryptoService) -> &Self {
        v
    }
}

impl nsIIdentityCryptoService {
    /// Cast this `nsIIdentityCryptoService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIIdentityCryptoServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIIdentityCryptoService {
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
impl<T: nsISupportsCoerce> nsIIdentityCryptoServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIIdentityCryptoService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIIdentityCryptoService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIIdentityCryptoServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void generateKeyPair (in AUTF8String algorithm, in nsIIdentityKeyGenCallback callback); */
    pub GenerateKeyPair: unsafe extern "system" fn (this: *const nsIIdentityCryptoService, algorithm: *const ::nsstring::nsACString, callback: *const nsIIdentityKeyGenCallback) -> ::nserror::nsresult,

    /* ACString base64UrlEncode (in AUTF8String toEncode); */
    pub Base64UrlEncode: unsafe extern "system" fn (this: *const nsIIdentityCryptoService, toEncode: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIIdentityCryptoService {


    /// `void generateKeyPair (in AUTF8String algorithm, in nsIIdentityKeyGenCallback callback);`
    #[inline]
    pub unsafe fn GenerateKeyPair(&self, algorithm: *const ::nsstring::nsACString, callback: *const nsIIdentityKeyGenCallback) -> ::nserror::nsresult {
        ((*self.vtable).GenerateKeyPair)(self, algorithm, callback)
    }



    /// `ACString base64UrlEncode (in AUTF8String toEncode);`
    #[inline]
    pub unsafe fn Base64UrlEncode(&self, toEncode: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Base64UrlEncode)(self, toEncode, _retval)
    }


}


/// `interface nsIIdentityKeyPair : nsISupports`
///

/// ```text
/// /**
///  * This interface provides a keypair and signing interface for Identity functionality
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIIdentityKeyPair {
    vtable: *const nsIIdentityKeyPairVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIIdentityKeyPair.
unsafe impl XpCom for nsIIdentityKeyPair {
    const IID: nsIID = nsID(0x73962dc7, 0x8ee7, 0x4346,
        [0xa1, 0x2b, 0xb0, 0x39, 0xe1, 0xd9, 0xb5, 0x4d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIIdentityKeyPair {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIIdentityKeyPair.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIIdentityKeyPairCoerce {
    /// Cheaply cast a value of this type from a `nsIIdentityKeyPair`.
    fn coerce_from(v: &nsIIdentityKeyPair) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIIdentityKeyPairCoerce for nsIIdentityKeyPair {
    #[inline]
    fn coerce_from(v: &nsIIdentityKeyPair) -> &Self {
        v
    }
}

impl nsIIdentityKeyPair {
    /// Cast this `nsIIdentityKeyPair` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIIdentityKeyPairCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIIdentityKeyPair {
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
impl<T: nsISupportsCoerce> nsIIdentityKeyPairCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIIdentityKeyPair) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIIdentityKeyPair
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIIdentityKeyPairVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AUTF8String keyType; */
    pub GetKeyType: unsafe extern "system" fn (this: *const nsIIdentityKeyPair, aKeyType: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String hexRSAPublicKeyExponent; */
    pub GetHexRSAPublicKeyExponent: unsafe extern "system" fn (this: *const nsIIdentityKeyPair, aHexRSAPublicKeyExponent: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String hexRSAPublicKeyModulus; */
    pub GetHexRSAPublicKeyModulus: unsafe extern "system" fn (this: *const nsIIdentityKeyPair, aHexRSAPublicKeyModulus: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String hexDSAPrime; */
    pub GetHexDSAPrime: unsafe extern "system" fn (this: *const nsIIdentityKeyPair, aHexDSAPrime: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String hexDSASubPrime; */
    pub GetHexDSASubPrime: unsafe extern "system" fn (this: *const nsIIdentityKeyPair, aHexDSASubPrime: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String hexDSAGenerator; */
    pub GetHexDSAGenerator: unsafe extern "system" fn (this: *const nsIIdentityKeyPair, aHexDSAGenerator: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String hexDSAPublicValue; */
    pub GetHexDSAPublicValue: unsafe extern "system" fn (this: *const nsIIdentityKeyPair, aHexDSAPublicValue: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void sign (in AUTF8String aText, in nsIIdentitySignCallback callback); */
    pub Sign: unsafe extern "system" fn (this: *const nsIIdentityKeyPair, aText: *const ::nsstring::nsACString, callback: *const nsIIdentitySignCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIIdentityKeyPair {


    /// `readonly attribute AUTF8String keyType;`
    #[inline]
    pub unsafe fn GetKeyType(&self, aKeyType: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetKeyType)(self, aKeyType)
    }



    /// `readonly attribute AUTF8String hexRSAPublicKeyExponent;`
    #[inline]
    pub unsafe fn GetHexRSAPublicKeyExponent(&self, aHexRSAPublicKeyExponent: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetHexRSAPublicKeyExponent)(self, aHexRSAPublicKeyExponent)
    }



    /// `readonly attribute AUTF8String hexRSAPublicKeyModulus;`
    #[inline]
    pub unsafe fn GetHexRSAPublicKeyModulus(&self, aHexRSAPublicKeyModulus: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetHexRSAPublicKeyModulus)(self, aHexRSAPublicKeyModulus)
    }



    /// `readonly attribute AUTF8String hexDSAPrime;`
    #[inline]
    pub unsafe fn GetHexDSAPrime(&self, aHexDSAPrime: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetHexDSAPrime)(self, aHexDSAPrime)
    }



    /// `readonly attribute AUTF8String hexDSASubPrime;`
    #[inline]
    pub unsafe fn GetHexDSASubPrime(&self, aHexDSASubPrime: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetHexDSASubPrime)(self, aHexDSASubPrime)
    }



    /// `readonly attribute AUTF8String hexDSAGenerator;`
    #[inline]
    pub unsafe fn GetHexDSAGenerator(&self, aHexDSAGenerator: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetHexDSAGenerator)(self, aHexDSAGenerator)
    }



    /// `readonly attribute AUTF8String hexDSAPublicValue;`
    #[inline]
    pub unsafe fn GetHexDSAPublicValue(&self, aHexDSAPublicValue: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetHexDSAPublicValue)(self, aHexDSAPublicValue)
    }



    /// `void sign (in AUTF8String aText, in nsIIdentitySignCallback callback);`
    #[inline]
    pub unsafe fn Sign(&self, aText: *const ::nsstring::nsACString, callback: *const nsIIdentitySignCallback) -> ::nserror::nsresult {
        ((*self.vtable).Sign)(self, aText, callback)
    }


}


/// `interface nsIIdentityKeyGenCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIIdentityKeyGenCallback {
    vtable: *const nsIIdentityKeyGenCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIIdentityKeyGenCallback.
unsafe impl XpCom for nsIIdentityKeyGenCallback {
    const IID: nsIID = nsID(0x90f24ca2, 0x2b05, 0x4ca9,
        [0x8a, 0xec, 0x89, 0xd3, 0x8e, 0x2f, 0x90, 0x5a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIIdentityKeyGenCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIIdentityKeyGenCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIIdentityKeyGenCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIIdentityKeyGenCallback`.
    fn coerce_from(v: &nsIIdentityKeyGenCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIIdentityKeyGenCallbackCoerce for nsIIdentityKeyGenCallback {
    #[inline]
    fn coerce_from(v: &nsIIdentityKeyGenCallback) -> &Self {
        v
    }
}

impl nsIIdentityKeyGenCallback {
    /// Cast this `nsIIdentityKeyGenCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIIdentityKeyGenCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIIdentityKeyGenCallback {
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
impl<T: nsISupportsCoerce> nsIIdentityKeyGenCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIIdentityKeyGenCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIIdentityKeyGenCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIIdentityKeyGenCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void generateKeyPairFinished (in nsresult rv, in nsIIdentityKeyPair keyPair); */
    pub GenerateKeyPairFinished: unsafe extern "system" fn (this: *const nsIIdentityKeyGenCallback, rv: ::nserror::nsresult, keyPair: *const nsIIdentityKeyPair) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIIdentityKeyGenCallback {

    /// ```text
    /// /**
    ///  * This interface provides a JavaScript callback object used to collect the
    ///  * nsIIdentityServeKeyPair when the keygen operation is complete
    ///  *
    ///  * though there is discussion as to whether we need the nsresult,
    ///  * we keep it so we can track deeper crypto errors.
    ///  */
    /// ```
    ///

    /// `void generateKeyPairFinished (in nsresult rv, in nsIIdentityKeyPair keyPair);`
    #[inline]
    pub unsafe fn GenerateKeyPairFinished(&self, rv: ::nserror::nsresult, keyPair: *const nsIIdentityKeyPair) -> ::nserror::nsresult {
        ((*self.vtable).GenerateKeyPairFinished)(self, rv, keyPair)
    }


}


/// `interface nsIIdentitySignCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIIdentitySignCallback {
    vtable: *const nsIIdentitySignCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIIdentitySignCallback.
unsafe impl XpCom for nsIIdentitySignCallback {
    const IID: nsIID = nsID(0x2d3e5036, 0x374b, 0x4b47,
        [0xa4, 0x30, 0x11, 0x96, 0xb6, 0x7b, 0x89, 0x0f]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIIdentitySignCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIIdentitySignCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIIdentitySignCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIIdentitySignCallback`.
    fn coerce_from(v: &nsIIdentitySignCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIIdentitySignCallbackCoerce for nsIIdentitySignCallback {
    #[inline]
    fn coerce_from(v: &nsIIdentitySignCallback) -> &Self {
        v
    }
}

impl nsIIdentitySignCallback {
    /// Cast this `nsIIdentitySignCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIIdentitySignCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIIdentitySignCallback {
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
impl<T: nsISupportsCoerce> nsIIdentitySignCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIIdentitySignCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIIdentitySignCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIIdentitySignCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void signFinished (in nsresult rv, in ACString base64urlSignature); */
    pub SignFinished: unsafe extern "system" fn (this: *const nsIIdentitySignCallback, rv: ::nserror::nsresult, base64urlSignature: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIIdentitySignCallback {

    /// ```text
    /// /**
    ///  * This interface provides a JavaScript callback object used to collect the
    ///  * AUTF8String signature
    ///  */
    /// /** On success, base64urlSignature is the base-64-URL-encoded signature
    ///    *
    ///    * For RS256 signatures, XXX bug 769858
    ///    *
    ///    * For DSA128 signatures, the signature is the r value concatenated with the
    ///    * s value, each component padded with leading zeroes as necessary.
    ///    */
    /// ```
    ///

    /// `void signFinished (in nsresult rv, in ACString base64urlSignature);`
    #[inline]
    pub unsafe fn SignFinished(&self, rv: ::nserror::nsresult, base64urlSignature: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SignFinished)(self, rv, base64urlSignature)
    }


}


