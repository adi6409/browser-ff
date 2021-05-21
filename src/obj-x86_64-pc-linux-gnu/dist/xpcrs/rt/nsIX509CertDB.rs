//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIX509CertDB.idl
//


/// `typedef uint32_t  AppTrustedRoot;`
///


pub type AppTrustedRoot = uint32_t;


/// `interface nsIOpenSignedAppFileCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIOpenSignedAppFileCallback {
    vtable: *const nsIOpenSignedAppFileCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIOpenSignedAppFileCallback.
unsafe impl XpCom for nsIOpenSignedAppFileCallback {
    const IID: nsIID = nsID(0xfc2b60e5, 0x9a07, 0x47c2,
        [0xa2, 0xcd, 0xb8, 0x3b, 0x68, 0xa6, 0x60, 0xac]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIOpenSignedAppFileCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIOpenSignedAppFileCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIOpenSignedAppFileCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIOpenSignedAppFileCallback`.
    fn coerce_from(v: &nsIOpenSignedAppFileCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIOpenSignedAppFileCallbackCoerce for nsIOpenSignedAppFileCallback {
    #[inline]
    fn coerce_from(v: &nsIOpenSignedAppFileCallback) -> &Self {
        v
    }
}

impl nsIOpenSignedAppFileCallback {
    /// Cast this `nsIOpenSignedAppFileCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIOpenSignedAppFileCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIOpenSignedAppFileCallback {
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
impl<T: nsISupportsCoerce> nsIOpenSignedAppFileCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIOpenSignedAppFileCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIOpenSignedAppFileCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIOpenSignedAppFileCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void openSignedAppFileFinished (in nsresult rv, in nsIZipReader aZipReader, in nsIX509Cert aSignerCert); */
    pub OpenSignedAppFileFinished: unsafe extern "system" fn (this: *const nsIOpenSignedAppFileCallback, rv: ::nserror::nsresult, aZipReader: *const nsIZipReader, aSignerCert: *const nsIX509Cert) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIOpenSignedAppFileCallback {


    /// `void openSignedAppFileFinished (in nsresult rv, in nsIZipReader aZipReader, in nsIX509Cert aSignerCert);`
    #[inline]
    pub unsafe fn OpenSignedAppFileFinished(&self, rv: ::nserror::nsresult, aZipReader: *const nsIZipReader, aSignerCert: *const nsIX509Cert) -> ::nserror::nsresult {
        ((*self.vtable).OpenSignedAppFileFinished)(self, rv, aZipReader, aSignerCert)
    }


}


/// `interface nsIAsyncBoolCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAsyncBoolCallback {
    vtable: *const nsIAsyncBoolCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAsyncBoolCallback.
unsafe impl XpCom for nsIAsyncBoolCallback {
    const IID: nsIID = nsID(0x07c08655, 0x8b11, 0x4650,
        [0xb6, 0xc4, 0x0c, 0x14, 0x55, 0x95, 0xce, 0xb5]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAsyncBoolCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAsyncBoolCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAsyncBoolCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIAsyncBoolCallback`.
    fn coerce_from(v: &nsIAsyncBoolCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAsyncBoolCallbackCoerce for nsIAsyncBoolCallback {
    #[inline]
    fn coerce_from(v: &nsIAsyncBoolCallback) -> &Self {
        v
    }
}

impl nsIAsyncBoolCallback {
    /// Cast this `nsIAsyncBoolCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAsyncBoolCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAsyncBoolCallback {
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
impl<T: nsISupportsCoerce> nsIAsyncBoolCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAsyncBoolCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAsyncBoolCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAsyncBoolCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onResult (in bool result); */
    pub OnResult: unsafe extern "system" fn (this: *const nsIAsyncBoolCallback, result: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAsyncBoolCallback {


    /// `void onResult (in bool result);`
    #[inline]
    pub unsafe fn OnResult(&self, result: bool) -> ::nserror::nsresult {
        ((*self.vtable).OnResult)(self, result)
    }


}


/// `interface nsICertVerificationCallback : nsISupports`
///

/// ```text
/// /**
///  * Callback type for use with asyncVerifyCertAtTime.
///  * If aPRErrorCode is PRErrorCodeSuccess (i.e. 0), aVerifiedChain represents the
///  * verified certificate chain determined by asyncVerifyCertAtTime. aHasEVPolicy
///  * represents whether or not the end-entity certificate verified as EV.
///  * If aPRErrorCode is non-zero, it represents the error encountered during
///  * verification. aVerifiedChain is null in that case and aHasEVPolicy has no
///  * meaning.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICertVerificationCallback {
    vtable: *const nsICertVerificationCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICertVerificationCallback.
unsafe impl XpCom for nsICertVerificationCallback {
    const IID: nsIID = nsID(0x49e16fc8, 0xefac, 0x4f57,
        [0x83, 0x61, 0x95, 0x6e, 0xf6, 0xb9, 0x60, 0xa4]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICertVerificationCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICertVerificationCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICertVerificationCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsICertVerificationCallback`.
    fn coerce_from(v: &nsICertVerificationCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICertVerificationCallbackCoerce for nsICertVerificationCallback {
    #[inline]
    fn coerce_from(v: &nsICertVerificationCallback) -> &Self {
        v
    }
}

impl nsICertVerificationCallback {
    /// Cast this `nsICertVerificationCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICertVerificationCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICertVerificationCallback {
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
impl<T: nsISupportsCoerce> nsICertVerificationCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICertVerificationCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICertVerificationCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICertVerificationCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void verifyCertFinished (in int32_t aPRErrorCode, in Array<nsIX509Cert> aVerifiedChain, in bool aHasEVPolicy); */
    pub VerifyCertFinished: unsafe extern "system" fn (this: *const nsICertVerificationCallback, aPRErrorCode: int32_t, aVerifiedChain: *const thin_vec::ThinVec<RefPtr<nsIX509Cert>>, aHasEVPolicy: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICertVerificationCallback {


    /// `void verifyCertFinished (in int32_t aPRErrorCode, in Array<nsIX509Cert> aVerifiedChain, in bool aHasEVPolicy);`
    #[inline]
    pub unsafe fn VerifyCertFinished(&self, aPRErrorCode: int32_t, aVerifiedChain: *const thin_vec::ThinVec<RefPtr<nsIX509Cert>>, aHasEVPolicy: bool) -> ::nserror::nsresult {
        ((*self.vtable).VerifyCertFinished)(self, aPRErrorCode, aVerifiedChain, aHasEVPolicy)
    }


}


/// `interface nsIX509CertDB : nsISupports`
///

/// ```text
/// /**
///  * This represents a service to access and manipulate
///  * X.509 certificates stored in a database.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIX509CertDB {
    vtable: *const nsIX509CertDBVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIX509CertDB.
unsafe impl XpCom for nsIX509CertDB {
    const IID: nsIID = nsID(0x5c16cd9b, 0x5a73, 0x47f1,
        [0xab, 0x0f, 0x11, 0xed, 0xe7, 0x49, 0x5c, 0xce]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIX509CertDB {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIX509CertDB.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIX509CertDBCoerce {
    /// Cheaply cast a value of this type from a `nsIX509CertDB`.
    fn coerce_from(v: &nsIX509CertDB) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIX509CertDBCoerce for nsIX509CertDB {
    #[inline]
    fn coerce_from(v: &nsIX509CertDB) -> &Self {
        v
    }
}

impl nsIX509CertDB {
    /// Cast this `nsIX509CertDB` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIX509CertDBCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIX509CertDB {
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
impl<T: nsISupportsCoerce> nsIX509CertDBCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIX509CertDB) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIX509CertDB
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIX509CertDBVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] nsIX509Cert findCertByDBKey (in ACString aDBkey); */
    pub FindCertByDBKey: unsafe extern "system" fn (this: *const nsIX509CertDB, aDBkey: *const ::nsstring::nsACString, _retval: *mut*const nsIX509Cert) -> ::nserror::nsresult,

    /* void importCertificates ([array, size_is (length)] in octet data, in unsigned long length, in unsigned long type, in nsIInterfaceRequestor ctx); */
    pub ImportCertificates: unsafe extern "system" fn (this: *const nsIX509CertDB, data: *mut u8, length: u32, type_: u32, ctx: *const nsIInterfaceRequestor) -> ::nserror::nsresult,

    /* void importEmailCertificate ([array, size_is (length)] in octet data, in unsigned long length, in nsIInterfaceRequestor ctx); */
    pub ImportEmailCertificate: unsafe extern "system" fn (this: *const nsIX509CertDB, data: *mut u8, length: u32, ctx: *const nsIInterfaceRequestor) -> ::nserror::nsresult,

    /* void importUserCertificate ([array, size_is (length)] in octet data, in unsigned long length, in nsIInterfaceRequestor ctx); */
    pub ImportUserCertificate: unsafe extern "system" fn (this: *const nsIX509CertDB, data: *mut u8, length: u32, ctx: *const nsIInterfaceRequestor) -> ::nserror::nsresult,

    /* void deleteCertificate (in nsIX509Cert aCert); */
    pub DeleteCertificate: unsafe extern "system" fn (this: *const nsIX509CertDB, aCert: *const nsIX509Cert) -> ::nserror::nsresult,

    /* [must_use] void setCertTrust (in nsIX509Cert cert, in unsigned long type, in unsigned long trust); */
    pub SetCertTrust: unsafe extern "system" fn (this: *const nsIX509CertDB, cert: *const nsIX509Cert, type_: u32, trust: u32) -> ::nserror::nsresult,

    /* [must_use] void setCertTrustFromString (in nsIX509Cert cert, in ACString trustString); */
    pub SetCertTrustFromString: unsafe extern "system" fn (this: *const nsIX509CertDB, cert: *const nsIX509Cert, trustString: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] boolean isCertTrusted (in nsIX509Cert cert, in unsigned long certType, in unsigned long trustType); */
    pub IsCertTrusted: unsafe extern "system" fn (this: *const nsIX509CertDB, cert: *const nsIX509Cert, certType: u32, trustType: u32, _retval: *mut bool) -> ::nserror::nsresult,

    /* [must_use] void importCertsFromFile (in nsIFile aFile, in unsigned long aType); */
    pub ImportCertsFromFile: unsafe extern "system" fn (this: *const nsIX509CertDB, aFile: *const nsIFile, aType: u32) -> ::nserror::nsresult,

    /* [must_use] uint32_t importPKCS12File (in nsIFile aFile, in AString aPassword); */
    pub ImportPKCS12File: unsafe extern "system" fn (this: *const nsIX509CertDB, aFile: *const nsIFile, aPassword: *const ::nsstring::nsAString, _retval: *mut uint32_t) -> ::nserror::nsresult,

    /* [must_use] uint32_t exportPKCS12File (in nsIFile aFile, in Array<nsIX509Cert> aCerts, in AString aPassword); */
    pub ExportPKCS12File: unsafe extern "system" fn (this: *const nsIX509CertDB, aFile: *const nsIFile, aCerts: *const thin_vec::ThinVec<RefPtr<nsIX509Cert>>, aPassword: *const ::nsstring::nsAString, _retval: *mut uint32_t) -> ::nserror::nsresult,

    /* [must_use] nsIX509Cert constructX509FromBase64 (in ACString base64); */
    pub ConstructX509FromBase64: unsafe extern "system" fn (this: *const nsIX509CertDB, base64: *const ::nsstring::nsACString, _retval: *mut*const nsIX509Cert) -> ::nserror::nsresult,

    /* [must_use] nsIX509Cert constructX509 (in Array<uint8_t> certDER); */
    pub ConstructX509: unsafe extern "system" fn (this: *const nsIX509CertDB, certDER: *const thin_vec::ThinVec<uint8_t>, _retval: *mut*const nsIX509Cert) -> ::nserror::nsresult,

    /* [must_use] void openSignedAppFileAsync (in AppTrustedRoot trustedRoot, in nsIFile aJarFile, in nsIOpenSignedAppFileCallback callback); */
    pub OpenSignedAppFileAsync: unsafe extern "system" fn (this: *const nsIX509CertDB, trustedRoot: AppTrustedRoot, aJarFile: *const nsIFile, callback: *const nsIOpenSignedAppFileCallback) -> ::nserror::nsresult,

    /* [must_use] nsIX509Cert addCert (in ACString certDER, in ACString trust); */
    pub AddCert: unsafe extern "system" fn (this: *const nsIX509CertDB, certDER: *const ::nsstring::nsACString, trust: *const ::nsstring::nsACString, _retval: *mut*const nsIX509Cert) -> ::nserror::nsresult,

    /* [must_use] void asyncVerifyCertAtTime (in nsIX509Cert aCert, in int64_t aUsage, in uint32_t aFlags, in ACString aHostname, in uint64_t aTime, in nsICertVerificationCallback aCallback); */
    pub AsyncVerifyCertAtTime: unsafe extern "system" fn (this: *const nsIX509CertDB, aCert: *const nsIX509Cert, aUsage: int64_t, aFlags: uint32_t, aHostname: *const ::nsstring::nsACString, aTime: uint64_t, aCallback: *const nsICertVerificationCallback) -> ::nserror::nsresult,

    /* [must_use] void clearOCSPCache (); */
    pub ClearOCSPCache: unsafe extern "system" fn (this: *const nsIX509CertDB) -> ::nserror::nsresult,

    /* [must_use] nsIX509Cert addCertFromBase64 (in ACString base64, in ACString trust); */
    pub AddCertFromBase64: unsafe extern "system" fn (this: *const nsIX509CertDB, base64: *const ::nsstring::nsACString, trust: *const ::nsstring::nsACString, _retval: *mut*const nsIX509Cert) -> ::nserror::nsresult,

    /* [must_use] Array<nsIX509Cert> getCerts (); */
    pub GetCerts: unsafe extern "system" fn (this: *const nsIX509CertDB, _retval: *mut thin_vec::ThinVec<RefPtr<nsIX509Cert>>) -> ::nserror::nsresult,

    /* [must_use] ACString asPKCS7Blob (in Array<nsIX509Cert> certList); */
    pub AsPKCS7Blob: unsafe extern "system" fn (this: *const nsIX509CertDB, certList: *const thin_vec::ThinVec<RefPtr<nsIX509Cert>>, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] void asyncHasThirdPartyRoots (in nsIAsyncBoolCallback callback); */
    pub AsyncHasThirdPartyRoots: unsafe extern "system" fn (this: *const nsIX509CertDB, callback: *const nsIAsyncBoolCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIX509CertDB {
    /// ```text
    /// /**
    ///    *  Constants that define which usages a certificate
    ///    *  is trusted for.
    ///    */
    /// ```
    ///

    pub const UNTRUSTED: i64 = 0;


    pub const TRUSTED_SSL: i64 = 1;


    pub const TRUSTED_EMAIL: i64 = 2;


    pub const Success: i64 = 0;


    pub const ERROR_UNKNOWN: i64 = 1;


    pub const ERROR_PKCS12_NOSMARTCARD_EXPORT: i64 = 2;


    pub const ERROR_PKCS12_RESTORE_FAILED: i64 = 3;


    pub const ERROR_PKCS12_BACKUP_FAILED: i64 = 4;


    pub const ERROR_PKCS12_CERT_COLLISION: i64 = 5;


    pub const ERROR_BAD_PASSWORD: i64 = 6;


    pub const ERROR_DECODE_ERROR: i64 = 7;


    pub const ERROR_PKCS12_DUPLICATE_DATA: i64 = 8;

    /// ```text
    /// /**
    ///    *  Verifies the signature on the given JAR file to verify that it has a
    ///    *  valid signature.  To be considered valid, there must be exactly one
    ///    *  signature on the JAR file and that signature must have signed every
    ///    *  entry. Further, the signature must come from a certificate that
    ///    *  is trusted for code signing.
    ///    *
    ///    *  On success, NS_OK, a nsIZipReader, and the trusted certificate that
    ///    *  signed the JAR are returned.
    ///    *
    ///    *  On failure, an error code is returned.
    ///    *
    ///    *  This method returns a nsIZipReader, instead of taking an nsIZipReader
    ///    *  as input, to encourage users of the API to verify the signature as the
    ///    *  first step in opening the JAR.
    ///    */
    /// ```
    ///

    pub const AppXPCShellRoot: i64 = 6;


    pub const AddonsPublicRoot: i64 = 7;


    pub const AddonsStageRoot: i64 = 8;


    pub const FLAG_LOCAL_ONLY: i64 = 1;


    pub const FLAG_MUST_BE_EV: i64 = 2;

    /// ```text
    /// /**
    ///    *  Will find a certificate based on its dbkey
    ///    *  retrieved by getting the dbKey attribute of
    ///    *  the certificate.
    ///    *
    ///    *  @param aDBkey Database internal key, as obtained using
    ///    *                attribute dbkey in nsIX509Cert.
    ///    */
    /// ```
    ///

    /// `[must_use] nsIX509Cert findCertByDBKey (in ACString aDBkey);`
    #[inline]
    pub unsafe fn FindCertByDBKey(&self, aDBkey: *const ::nsstring::nsACString, _retval: *mut*const nsIX509Cert) -> ::nserror::nsresult {
        ((*self.vtable).FindCertByDBKey)(self, aDBkey, _retval)
    }


    /// ```text
    /// /**
    ///    *  Use this to import a stream sent down as a mime type into
    ///    *  the certificate database on the default token.
    ///    *  The stream may consist of one or more certificates.
    ///    *
    ///    *  @param data The raw data to be imported
    ///    *  @param length The length of the data to be imported
    ///    *  @param type The type of the certificate, see constants in nsIX509Cert
    ///    *  @param ctx A UI context.
    ///    */
    /// ```
    ///

    /// `void importCertificates ([array, size_is (length)] in octet data, in unsigned long length, in unsigned long type, in nsIInterfaceRequestor ctx);`
    #[inline]
    pub unsafe fn ImportCertificates(&self, data: *mut u8, length: u32, type_: u32, ctx: *const nsIInterfaceRequestor) -> ::nserror::nsresult {
        ((*self.vtable).ImportCertificates)(self, data, length, type_, ctx)
    }


    /// ```text
    /// /**
    ///    *  Import another person's email certificate into the database.
    ///    *
    ///    *  @param data The raw data to be imported
    ///    *  @param length The length of the data to be imported
    ///    *  @param ctx A UI context.
    ///    */
    /// ```
    ///

    /// `void importEmailCertificate ([array, size_is (length)] in octet data, in unsigned long length, in nsIInterfaceRequestor ctx);`
    #[inline]
    pub unsafe fn ImportEmailCertificate(&self, data: *mut u8, length: u32, ctx: *const nsIInterfaceRequestor) -> ::nserror::nsresult {
        ((*self.vtable).ImportEmailCertificate)(self, data, length, ctx)
    }


    /// ```text
    /// /**
    ///    *  Import a personal certificate into the database, assuming
    ///    *  the database already contains the private key for this certificate.
    ///    *
    ///    *  @param data The raw data to be imported
    ///    *  @param length The length of the data to be imported
    ///    *  @param ctx A UI context.
    ///    */
    /// ```
    ///

    /// `void importUserCertificate ([array, size_is (length)] in octet data, in unsigned long length, in nsIInterfaceRequestor ctx);`
    #[inline]
    pub unsafe fn ImportUserCertificate(&self, data: *mut u8, length: u32, ctx: *const nsIInterfaceRequestor) -> ::nserror::nsresult {
        ((*self.vtable).ImportUserCertificate)(self, data, length, ctx)
    }


    /// ```text
    /// /**
    ///    *  Delete a certificate stored in the database.
    ///    *
    ///    *  @param aCert Delete this certificate.
    ///    */
    /// ```
    ///

    /// `void deleteCertificate (in nsIX509Cert aCert);`
    #[inline]
    pub unsafe fn DeleteCertificate(&self, aCert: *const nsIX509Cert) -> ::nserror::nsresult {
        ((*self.vtable).DeleteCertificate)(self, aCert)
    }


    /// ```text
    /// /**
    ///    *  Modify the trust that is stored and associated to a certificate within
    ///    *  a database. Separate trust is stored for
    ///    *  One call manipulates the trust for one trust type only.
    ///    *  See the trust type constants defined within this interface.
    ///    *
    ///    *  @param cert Change the stored trust of this certificate.
    ///    *  @param type The type of the certificate. See nsIX509Cert.
    ///    *  @param trust A bitmask. The new trust for the possible usages.
    ///    *               See the trust constants defined within this interface.
    ///    */
    /// ```
    ///

    /// `[must_use] void setCertTrust (in nsIX509Cert cert, in unsigned long type, in unsigned long trust);`
    #[inline]
    pub unsafe fn SetCertTrust(&self, cert: *const nsIX509Cert, type_: u32, trust: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetCertTrust)(self, cert, type_, trust)
    }


    /// ```text
    /// /**
    ///    * @param cert        The certificate for which to modify trust.
    ///    * @param trustString decoded by CERT_DecodeTrustString. 3 comma separated
    ///    *                    characters, indicating SSL, Email, and Object signing
    ///    *                    trust. The object signing trust flags are effectively
    ///    *                    ignored by gecko, but they still must be specified (at
        ///    *                    least by a final trailing comma) because this argument
    ///    *                    is passed to CERT_DecodeTrustString.
    ///    */
    /// ```
    ///

    /// `[must_use] void setCertTrustFromString (in nsIX509Cert cert, in ACString trustString);`
    #[inline]
    pub unsafe fn SetCertTrustFromString(&self, cert: *const nsIX509Cert, trustString: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetCertTrustFromString)(self, cert, trustString)
    }


    /// ```text
    /// /**
    ///    *  Query whether a certificate is trusted for a particular use.
    ///    *
    ///    *  @param cert Obtain the stored trust of this certificate.
    ///    *  @param certType The type of the certificate. See nsIX509Cert.
    ///    *  @param trustType A single bit from the usages constants defined
    ///    *                   within this interface.
    ///    *
    ///    *  @return Returns true if the certificate is trusted for the given use.
    ///    */
    /// ```
    ///

    /// `[must_use] boolean isCertTrusted (in nsIX509Cert cert, in unsigned long certType, in unsigned long trustType);`
    #[inline]
    pub unsafe fn IsCertTrusted(&self, cert: *const nsIX509Cert, certType: u32, trustType: u32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsCertTrusted)(self, cert, certType, trustType, _retval)
    }


    /// ```text
    /// /**
    ///    *  Import certificate(s) from file
    ///    *
    ///    *  @param aFile Identifies a file that contains the certificate
    ///    *               to be imported.
    ///    *  @param aType Describes the type of certificate that is going to
    ///    *               be imported. See type constants in nsIX509Cert.
    ///    */
    /// ```
    ///

    /// `[must_use] void importCertsFromFile (in nsIFile aFile, in unsigned long aType);`
    #[inline]
    pub unsafe fn ImportCertsFromFile(&self, aFile: *const nsIFile, aType: u32) -> ::nserror::nsresult {
        ((*self.vtable).ImportCertsFromFile)(self, aFile, aType)
    }


    /// ```text
    /// /**
    ///    *  Import a PKCS#12 file containing cert(s) and key(s) into the database.
    ///    *
    ///    *  @param aFile Identifies a file that contains the data to be imported.
    ///    *  @param password The password used to protect the file.
    ///    *  @return Success or the specific error code on failure.  The return
    ///    *          values are defined in this file.
    ///    */
    /// ```
    ///

    /// `[must_use] uint32_t importPKCS12File (in nsIFile aFile, in AString aPassword);`
    #[inline]
    pub unsafe fn ImportPKCS12File(&self, aFile: *const nsIFile, aPassword: *const ::nsstring::nsAString, _retval: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).ImportPKCS12File)(self, aFile, aPassword, _retval)
    }


    /// ```text
    /// /**
    ///    *  Export a set of certs and keys from the database to a PKCS#12 file.
    ///    *
    ///    *  @param aFile Identifies a file that will be filled with the data to be
    ///    *               exported.
    ///    *  @param count The number of certificates to be exported.
    ///    *  @param aCerts The array of all certificates to be exported.
    ///    *  @param password The password used to protect the file.
    ///    *  @return Success or the specific error code on failure
    ///    */
    /// ```
    ///

    /// `[must_use] uint32_t exportPKCS12File (in nsIFile aFile, in Array<nsIX509Cert> aCerts, in AString aPassword);`
    #[inline]
    pub unsafe fn ExportPKCS12File(&self, aFile: *const nsIFile, aCerts: *const thin_vec::ThinVec<RefPtr<nsIX509Cert>>, aPassword: *const ::nsstring::nsAString, _retval: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).ExportPKCS12File)(self, aFile, aCerts, aPassword, _retval)
    }



    /// `[must_use] nsIX509Cert constructX509FromBase64 (in ACString base64);`
    #[inline]
    pub unsafe fn ConstructX509FromBase64(&self, base64: *const ::nsstring::nsACString, _retval: *mut*const nsIX509Cert) -> ::nserror::nsresult {
        ((*self.vtable).ConstructX509FromBase64)(self, base64, _retval)
    }



    /// `[must_use] nsIX509Cert constructX509 (in Array<uint8_t> certDER);`
    #[inline]
    pub unsafe fn ConstructX509(&self, certDER: *const thin_vec::ThinVec<uint8_t>, _retval: *mut*const nsIX509Cert) -> ::nserror::nsresult {
        ((*self.vtable).ConstructX509)(self, certDER, _retval)
    }



    /// `[must_use] void openSignedAppFileAsync (in AppTrustedRoot trustedRoot, in nsIFile aJarFile, in nsIOpenSignedAppFileCallback callback);`
    #[inline]
    pub unsafe fn OpenSignedAppFileAsync(&self, trustedRoot: AppTrustedRoot, aJarFile: *const nsIFile, callback: *const nsIOpenSignedAppFileCallback) -> ::nserror::nsresult {
        ((*self.vtable).OpenSignedAppFileAsync)(self, trustedRoot, aJarFile, callback)
    }



    /// `[must_use] nsIX509Cert addCert (in ACString certDER, in ACString trust);`
    #[inline]
    pub unsafe fn AddCert(&self, certDER: *const ::nsstring::nsACString, trust: *const ::nsstring::nsACString, _retval: *mut*const nsIX509Cert) -> ::nserror::nsresult {
        ((*self.vtable).AddCert)(self, certDER, trust, _retval)
    }



    /// `[must_use] void asyncVerifyCertAtTime (in nsIX509Cert aCert, in int64_t aUsage, in uint32_t aFlags, in ACString aHostname, in uint64_t aTime, in nsICertVerificationCallback aCallback);`
    #[inline]
    pub unsafe fn AsyncVerifyCertAtTime(&self, aCert: *const nsIX509Cert, aUsage: int64_t, aFlags: uint32_t, aHostname: *const ::nsstring::nsACString, aTime: uint64_t, aCallback: *const nsICertVerificationCallback) -> ::nserror::nsresult {
        ((*self.vtable).AsyncVerifyCertAtTime)(self, aCert, aUsage, aFlags, aHostname, aTime, aCallback)
    }



    /// `[must_use] void clearOCSPCache ();`
    #[inline]
    pub unsafe fn ClearOCSPCache(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ClearOCSPCache)(self, )
    }



    /// `[must_use] nsIX509Cert addCertFromBase64 (in ACString base64, in ACString trust);`
    #[inline]
    pub unsafe fn AddCertFromBase64(&self, base64: *const ::nsstring::nsACString, trust: *const ::nsstring::nsACString, _retval: *mut*const nsIX509Cert) -> ::nserror::nsresult {
        ((*self.vtable).AddCertFromBase64)(self, base64, trust, _retval)
    }



    /// `[must_use] Array<nsIX509Cert> getCerts ();`
    #[inline]
    pub unsafe fn GetCerts(&self, _retval: *mut thin_vec::ThinVec<RefPtr<nsIX509Cert>>) -> ::nserror::nsresult {
        ((*self.vtable).GetCerts)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Encode the list of certificates as a PKCS#7 SignedData structure. No data
    ///    * is actually signed - this is merely a way of exporting a collection of
    ///    * certificates.
    ///    */
    /// ```
    ///

    /// `[must_use] ACString asPKCS7Blob (in Array<nsIX509Cert> certList);`
    #[inline]
    pub unsafe fn AsPKCS7Blob(&self, certList: *const thin_vec::ThinVec<RefPtr<nsIX509Cert>>, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).AsPKCS7Blob)(self, certList, _retval)
    }


    /// ```text
    /// /**
    ///    * Iterates through all the certs and returns false if any of the trusted
    ///    * CA certs are not built-in roots; and true otherwise.
    ///    */
    /// ```
    ///

    /// `[must_use] void asyncHasThirdPartyRoots (in nsIAsyncBoolCallback callback);`
    #[inline]
    pub unsafe fn AsyncHasThirdPartyRoots(&self, callback: *const nsIAsyncBoolCallback) -> ::nserror::nsresult {
        ((*self.vtable).AsyncHasThirdPartyRoots)(self, callback)
    }


}


