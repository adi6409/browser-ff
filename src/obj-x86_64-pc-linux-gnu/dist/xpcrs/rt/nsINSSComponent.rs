//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsINSSComponent.idl
//


/// `interface nsINSSComponent : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINSSComponent {
    vtable: *const nsINSSComponentVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINSSComponent.
unsafe impl XpCom for nsINSSComponent {
    const IID: nsIID = nsID(0xa0a8f52b, 0xea18, 0x4abc,
        [0xa3, 0xca, 0xec, 0xcf, 0x70, 0x4f, 0xfe, 0x63]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINSSComponent {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINSSComponent.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINSSComponentCoerce {
    /// Cheaply cast a value of this type from a `nsINSSComponent`.
    fn coerce_from(v: &nsINSSComponent) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINSSComponentCoerce for nsINSSComponent {
    #[inline]
    fn coerce_from(v: &nsINSSComponent) -> &Self {
        v
    }
}

impl nsINSSComponent {
    /// Cast this `nsINSSComponent` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINSSComponentCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINSSComponent {
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
impl<T: nsISupportsCoerce> nsINSSComponentCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINSSComponent) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINSSComponent
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINSSComponentVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [noscript] void logoutAuthenticatedPK11 (); */
    pub LogoutAuthenticatedPK11: unsafe extern "system" fn (this: *const nsINSSComponent) -> ::nserror::nsresult,

    /* [noscript] bool isCertTestBuiltInRoot (in CERTCertificatePtr cert); */
    /// Unable to generate binding because `native type CERTCertificate unsupported`
    pub IsCertTestBuiltInRoot: *const ::libc::c_void,

    /* [noscript] bool isCertContentSigningRoot (in Array<octet> cert); */
    pub IsCertContentSigningRoot: unsafe extern "system" fn (this: *const nsINSSComponent, cert: *const thin_vec::ThinVec<u8>, _retval: *mut bool) -> ::nserror::nsresult,

    /* Array<Array<octet>> getEnterpriseRoots (); */
    pub GetEnterpriseRoots: unsafe extern "system" fn (this: *const nsINSSComponent, _retval: *mut thin_vec::ThinVec<thin_vec::ThinVec<u8>>) -> ::nserror::nsresult,

    /* Array<Array<octet>> getEnterpriseIntermediates (); */
    pub GetEnterpriseIntermediates: unsafe extern "system" fn (this: *const nsINSSComponent, _retval: *mut thin_vec::ThinVec<thin_vec::ThinVec<u8>>) -> ::nserror::nsresult,

    /* void addEnterpriseIntermediate (in Array<octet> intermediateBytes); */
    pub AddEnterpriseIntermediate: unsafe extern "system" fn (this: *const nsINSSComponent, intermediateBytes: *const thin_vec::ThinVec<u8>) -> ::nserror::nsresult,

    /* [noscript] void blockUntilLoadableCertsLoaded (); */
    pub BlockUntilLoadableCertsLoaded: unsafe extern "system" fn (this: *const nsINSSComponent) -> ::nserror::nsresult,

    /* [noscript] void checkForSmartCardChanges (); */
    pub CheckForSmartCardChanges: unsafe extern "system" fn (this: *const nsINSSComponent) -> ::nserror::nsresult,

    /* [noscript] void issuerMatchesMitmCanary (in string certIssuer); */
    pub IssuerMatchesMitmCanary: unsafe extern "system" fn (this: *const nsINSSComponent, certIssuer: *const libc::c_char) -> ::nserror::nsresult,

    /* [noscript] bool hasActiveSmartCards (); */
    pub HasActiveSmartCards: unsafe extern "system" fn (this: *const nsINSSComponent, _retval: *mut bool) -> ::nserror::nsresult,

    /* [noscript] bool hasUserCertsInstalled (); */
    pub HasUserCertsInstalled: unsafe extern "system" fn (this: *const nsINSSComponent, _retval: *mut bool) -> ::nserror::nsresult,

    /* [noscript] SharedCertVerifierPtr getDefaultCertVerifier (); */
    /// Unable to generate binding because `native type mozilla::psm::SharedCertVerifier unsupported`
    pub GetDefaultCertVerifier: *const ::libc::c_void,

    /* void clearSSLExternalAndInternalSessionCache (); */
    pub ClearSSLExternalAndInternalSessionCache: unsafe extern "system" fn (this: *const nsINSSComponent) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINSSComponent {

    /// ```text
    /// /**
    ///    * When we log out of a PKCS#11 token, any TLS connections that may have
    ///    * involved a client certificate stored on that token must be closed. Since we
    ///    * don't have a fine-grained way to do this, we basically cancel everything.
    ///    * More speficially, this clears all temporary certificate exception overrides
    ///    * and any remembered client authentication certificate decisions, and then
    ///    * cancels all network connections (strictly speaking, this last part is
        ///    * overzealous - we only need to cancel all https connections (see bug
            ///    * 1446645)).
    ///    */
    /// ```
    ///

    /// `[noscript] void logoutAuthenticatedPK11 ();`
    #[inline]
    pub unsafe fn LogoutAuthenticatedPK11(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).LogoutAuthenticatedPK11)(self, )
    }


    /// ```text
    /// /**
    ///    * Used to determine if the given CERTCertificate is the certificate we use in
    ///    * tests to simulate a built-in root certificate. Returns false in non-debug
    ///    * builds.
    ///    */
    /// ```
    ///

    /// `[noscript] bool isCertTestBuiltInRoot (in CERTCertificatePtr cert);`
    const _IsCertTestBuiltInRoot: () = ();

    /// ```text
    /// /**
    ///    * Used to determine if the given certificate (represented as an array of
        ///    * bytes) is the content signing root certificate.
    ///    */
    /// ```
    ///

    /// `[noscript] bool isCertContentSigningRoot (in Array<octet> cert);`
    #[inline]
    pub unsafe fn IsCertContentSigningRoot(&self, cert: *const thin_vec::ThinVec<u8>, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsCertContentSigningRoot)(self, cert, _retval)
    }


    /// ```text
    /// /**
    ///    * If enabled by the preference "security.enterprise_roots.enabled", returns
    ///    * an array of arrays of bytes representing the imported enterprise root
    ///    * certificates (i.e. root certificates gleaned from the OS certificate
        ///    * store). Returns an empty array otherwise.
    ///    * Currently this is only implemented on Windows and MacOS X, so this
    ///    * function returns an empty array on all other platforms.
    ///    */
    /// ```
    ///

    /// `Array<Array<octet>> getEnterpriseRoots ();`
    #[inline]
    pub unsafe fn GetEnterpriseRoots(&self, _retval: *mut thin_vec::ThinVec<thin_vec::ThinVec<u8>>) -> ::nserror::nsresult {
        ((*self.vtable).GetEnterpriseRoots)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Similarly, but for intermediate certificates.
    ///    */
    /// ```
    ///

    /// `Array<Array<octet>> getEnterpriseIntermediates ();`
    #[inline]
    pub unsafe fn GetEnterpriseIntermediates(&self, _retval: *mut thin_vec::ThinVec<thin_vec::ThinVec<u8>>) -> ::nserror::nsresult {
        ((*self.vtable).GetEnterpriseIntermediates)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Test utility for adding an intermediate certificate to the current set of
    ///    * imported enterprise intermediates, if any. Additions to the set made using
    ///    * this function will be cleared when the value of the preference
    ///    * "security.enterprise_roots.enabled" changes.
    ///    */
    /// ```
    ///

    /// `void addEnterpriseIntermediate (in Array<octet> intermediateBytes);`
    #[inline]
    pub unsafe fn AddEnterpriseIntermediate(&self, intermediateBytes: *const thin_vec::ThinVec<u8>) -> ::nserror::nsresult {
        ((*self.vtable).AddEnterpriseIntermediate)(self, intermediateBytes)
    }


    /// ```text
    /// /**
    ///    * For performance reasons, the builtin roots module is loaded on a background
    ///    * thread. When any code that depends on the builtin roots module runs, it
    ///    * must first wait for the module to be loaded.
    ///    */
    /// ```
    ///

    /// `[noscript] void blockUntilLoadableCertsLoaded ();`
    #[inline]
    pub unsafe fn BlockUntilLoadableCertsLoaded(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).BlockUntilLoadableCertsLoaded)(self, )
    }


    /// ```text
    /// /**
    ///    * In theory a token on a PKCS#11 module can be inserted or removed at any
    ///    * time. Operations that may depend on resources on external tokens should
    ///    * call this to ensure they have a recent view of the token.
    ///    */
    /// ```
    ///

    /// `[noscript] void checkForSmartCardChanges ();`
    #[inline]
    pub unsafe fn CheckForSmartCardChanges(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).CheckForSmartCardChanges)(self, )
    }


    /// ```text
    /// /**
    ///    * Used to potentially detect when a user's internet connection is being
    ///    * intercepted. When doing an update ping, if certificate verification fails,
    ///    * we make a note of the issuer distinguished name of that certificate.
    ///    * If a subsequent certificate verification fails, we compare issuer
    ///    * distinguished names. If they match, something may be intercepting the
    ///    * user's traffic (if they don't match, the server is likely misconfigured).
    ///    * This function succeeds if the given DN matches the noted DN and fails
    ///    * otherwise (e.g. if the update ping never failed).
    ///    */
    /// ```
    ///

    /// `[noscript] void issuerMatchesMitmCanary (in string certIssuer);`
    #[inline]
    pub unsafe fn IssuerMatchesMitmCanary(&self, certIssuer: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).IssuerMatchesMitmCanary)(self, certIssuer)
    }


    /// ```text
    /// /**
    ///    * Returns true if the user has a PKCS#11 module with removable slots.
    ///    */
    /// ```
    ///

    /// `[noscript] bool hasActiveSmartCards ();`
    #[inline]
    pub unsafe fn HasActiveSmartCards(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasActiveSmartCards)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns true if the user has any client authentication certificates.
    ///    */
    /// ```
    ///

    /// `[noscript] bool hasUserCertsInstalled ();`
    #[inline]
    pub unsafe fn HasUserCertsInstalled(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasUserCertsInstalled)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns an already-adrefed handle to the currently configured shared
    ///    * certificate verifier.
    ///    */
    /// ```
    ///

    /// `[noscript] SharedCertVerifierPtr getDefaultCertVerifier ();`
    const _GetDefaultCertVerifier: () = ();

    /// ```text
    /// /**
    ///    * For clearing both SSL internal and external session cache from JS.
    ///    */
    /// ```
    ///

    /// `void clearSSLExternalAndInternalSessionCache ();`
    #[inline]
    pub unsafe fn ClearSSLExternalAndInternalSessionCache(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ClearSSLExternalAndInternalSessionCache)(self, )
    }


}


