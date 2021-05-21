//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsICertOverrideService.idl
//


/// `interface nsICertOverride : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICertOverride {
    vtable: *const nsICertOverrideVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICertOverride.
unsafe impl XpCom for nsICertOverride {
    const IID: nsIID = nsID(0xed735e24, 0xfa55, 0x4163,
        [0x90, 0x6d, 0x17, 0xfb, 0x78, 0x85, 0x1f, 0xe1]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICertOverride {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICertOverride.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICertOverrideCoerce {
    /// Cheaply cast a value of this type from a `nsICertOverride`.
    fn coerce_from(v: &nsICertOverride) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICertOverrideCoerce for nsICertOverride {
    #[inline]
    fn coerce_from(v: &nsICertOverride) -> &Self {
        v
    }
}

impl nsICertOverride {
    /// Cast this `nsICertOverride` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICertOverrideCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICertOverride {
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
impl<T: nsISupportsCoerce> nsICertOverrideCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICertOverride) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICertOverride
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICertOverrideVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString asciiHost; */
    pub GetAsciiHost: unsafe extern "system" fn (this: *const nsICertOverride, aAsciiHost: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute int32_t port; */
    pub GetPort: unsafe extern "system" fn (this: *const nsICertOverride, aPort: *mut int32_t) -> ::nserror::nsresult,

    /* readonly attribute boolean isTemporary; */
    pub GetIsTemporary: unsafe extern "system" fn (this: *const nsICertOverride, aIsTemporary: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute ACString dbKey; */
    pub GetDbKey: unsafe extern "system" fn (this: *const nsICertOverride, aDbKey: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString hostPort; */
    pub GetHostPort: unsafe extern "system" fn (this: *const nsICertOverride, aHostPort: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICertOverride {

    /// ```text
    /// /**
    ///   *   The hostname of the server the override is used for.
    ///   */
    /// ```
    ///

    /// `readonly attribute ACString asciiHost;`
    #[inline]
    pub unsafe fn GetAsciiHost(&self, aAsciiHost: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetAsciiHost)(self, aAsciiHost)
    }


    /// ```text
    /// /**
    ///   *   The port of the server the override is used for.
    ///   */
    /// ```
    ///

    /// `readonly attribute int32_t port;`
    #[inline]
    pub unsafe fn GetPort(&self, aPort: *mut int32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetPort)(self, aPort)
    }


    /// ```text
    /// /**
    ///   *   Whether or not the override is only used for this
    ///   *   session (true) or stored persistently (false)
    ///   */
    /// ```
    ///

    /// `readonly attribute boolean isTemporary;`
    #[inline]
    pub unsafe fn GetIsTemporary(&self, aIsTemporary: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsTemporary)(self, aIsTemporary)
    }


    /// ```text
    /// /**
    ///   *   The database key for the associated certificate.
    ///   */
    /// ```
    ///

    /// `readonly attribute ACString dbKey;`
    #[inline]
    pub unsafe fn GetDbKey(&self, aDbKey: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetDbKey)(self, aDbKey)
    }


    /// ```text
    /// /**
    ///   *   A combination of hostname and port in the form host:port.
    ///   *   Since the port can be -1 which is equivalent to port 433 we use an
    ///   *   existing function of nsCertOverrideService to create this property.
    ///   */
    /// ```
    ///

    /// `readonly attribute ACString hostPort;`
    #[inline]
    pub unsafe fn GetHostPort(&self, aHostPort: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetHostPort)(self, aHostPort)
    }


}


/// `interface nsICertOverrideService : nsISupports`
///

/// ```text
/// /**
///  * This represents the global list of triples
///  *   {host:port, cert-fingerprint, allowed-overrides}
///  * that the user wants to accept without further warnings.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICertOverrideService {
    vtable: *const nsICertOverrideServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICertOverrideService.
unsafe impl XpCom for nsICertOverrideService {
    const IID: nsIID = nsID(0xbe019e47, 0x22fc, 0x4355,
        [0x9f, 0x16, 0x9a, 0xb0, 0x47, 0xd6, 0x74, 0x2d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICertOverrideService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICertOverrideService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICertOverrideServiceCoerce {
    /// Cheaply cast a value of this type from a `nsICertOverrideService`.
    fn coerce_from(v: &nsICertOverrideService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICertOverrideServiceCoerce for nsICertOverrideService {
    #[inline]
    fn coerce_from(v: &nsICertOverrideService) -> &Self {
        v
    }
}

impl nsICertOverrideService {
    /// Cast this `nsICertOverrideService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICertOverrideServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICertOverrideService {
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
impl<T: nsISupportsCoerce> nsICertOverrideServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICertOverrideService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICertOverrideService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICertOverrideServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] void rememberValidityOverride (in AUTF8String aHostName, in int32_t aPort, in nsIX509Cert aCert, in uint32_t aOverrideBits, in boolean aTemporary); */
    pub RememberValidityOverride: unsafe extern "system" fn (this: *const nsICertOverrideService, aHostName: *const ::nsstring::nsACString, aPort: int32_t, aCert: *const nsIX509Cert, aOverrideBits: uint32_t, aTemporary: bool) -> ::nserror::nsresult,

    /* [must_use] void rememberTemporaryValidityOverrideUsingFingerprint (in AUTF8String aHostName, in int32_t aPort, in AUTF8String aCertFingerprint, in uint32_t aOverrideBits); */
    pub RememberTemporaryValidityOverrideUsingFingerprint: unsafe extern "system" fn (this: *const nsICertOverrideService, aHostName: *const ::nsstring::nsACString, aPort: int32_t, aCertFingerprint: *const ::nsstring::nsACString, aOverrideBits: uint32_t) -> ::nserror::nsresult,

    /* [must_use] boolean hasMatchingOverride (in AUTF8String aHostName, in int32_t aPort, in nsIX509Cert aCert, out uint32_t aOverrideBits, out boolean aIsTemporary); */
    pub HasMatchingOverride: unsafe extern "system" fn (this: *const nsICertOverrideService, aHostName: *const ::nsstring::nsACString, aPort: int32_t, aCert: *const nsIX509Cert, aOverrideBits: *mut uint32_t, aIsTemporary: *mut bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* void clearValidityOverride (in AUTF8String aHostName, in int32_t aPort); */
    pub ClearValidityOverride: unsafe extern "system" fn (this: *const nsICertOverrideService, aHostName: *const ::nsstring::nsACString, aPort: int32_t) -> ::nserror::nsresult,

    /* void clearAllOverrides (); */
    pub ClearAllOverrides: unsafe extern "system" fn (this: *const nsICertOverrideService) -> ::nserror::nsresult,

    /* [must_use] uint32_t isCertUsedForOverrides (in nsIX509Cert aCert, in boolean aCheckTemporaries, in boolean aCheckPermanents); */
    pub IsCertUsedForOverrides: unsafe extern "system" fn (this: *const nsICertOverrideService, aCert: *const nsIX509Cert, aCheckTemporaries: bool, aCheckPermanents: bool, _retval: *mut uint32_t) -> ::nserror::nsresult,

    /* Array<nsICertOverride> getOverrides (); */
    pub GetOverrides: unsafe extern "system" fn (this: *const nsICertOverrideService, _retval: *mut thin_vec::ThinVec<RefPtr<nsICertOverride>>) -> ::nserror::nsresult,

    /* void setDisableAllSecurityChecksAndLetAttackersInterceptMyData (in boolean aDisable); */
    pub SetDisableAllSecurityChecksAndLetAttackersInterceptMyData: unsafe extern "system" fn (this: *const nsICertOverrideService, aDisable: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICertOverrideService {
    /// ```text
    /// /**
    ///    *  Override Untrusted
    ///    */
    /// ```
    ///

    pub const ERROR_UNTRUSTED: i64 = 1;

    /// ```text
    /// /**
    ///    *  Override hostname Mismatch
    ///    */
    /// ```
    ///

    pub const ERROR_MISMATCH: i64 = 2;

    /// ```text
    /// /**
    ///    *  Override Time error
    ///    */
    /// ```
    ///

    pub const ERROR_TIME: i64 = 4;

    /// ```text
    /// /**
    ///    *  The given cert should always be accepted for the given hostname:port,
    ///    *  regardless of errors verifying the cert.
    ///    *  Host:Port is a primary key, only one entry per host:port can exist.
    ///    *  The implementation will store a fingerprint of the cert.
    ///    *  The implementation will decide which fingerprint alg is used.
    ///    *
    ///    *  Each override is specific to exactly the errors overridden, so
    ///    *  overriding everything won't match certs at the given host:port
    ///    *  which only exhibit some subset of errors.
    ///    *
    ///    *  @param aHostName The host (punycode) this mapping belongs to
    ///    *  @param aPort The port this mapping belongs to, if it is -1 then it
    ///    *          is internaly treated as 443
    ///    *  @param aCert The cert that should always be accepted
    ///    *  @param aOverrideBits The precise set of errors we want to be overriden
    ///    */
    /// ```
    ///

    /// `[must_use] void rememberValidityOverride (in AUTF8String aHostName, in int32_t aPort, in nsIX509Cert aCert, in uint32_t aOverrideBits, in boolean aTemporary);`
    #[inline]
    pub unsafe fn RememberValidityOverride(&self, aHostName: *const ::nsstring::nsACString, aPort: int32_t, aCert: *const nsIX509Cert, aOverrideBits: uint32_t, aTemporary: bool) -> ::nserror::nsresult {
        ((*self.vtable).RememberValidityOverride)(self, aHostName, aPort, aCert, aOverrideBits, aTemporary)
    }


    /// ```text
    /// /**
    ///    *  Certs with the given fingerprint should always be accepted for the
    ///    *  given hostname:port, regardless of errors verifying the cert.
    ///    *  Host:Port is a primary key, only one entry per host:port can exist.
    ///    *  The fingerprint should be an SHA-256 hash of the certificate.
    ///    *
    ///    *  @param aHostName The host (punycode) this mapping belongs to
    ///    *  @param aPort The port this mapping belongs to, if it is -1 then it
    ///    *          is internaly treated as 443
    ///    *  @param aCertFingerprint The cert fingerprint that should be accepted, in
    ///    *          the format 'AA:BB:...' (colon-separated upper-case hex bytes).
    ///    *  @param aOverrideBits The errors we want to be overriden
    ///    */
    /// ```
    ///

    /// `[must_use] void rememberTemporaryValidityOverrideUsingFingerprint (in AUTF8String aHostName, in int32_t aPort, in AUTF8String aCertFingerprint, in uint32_t aOverrideBits);`
    #[inline]
    pub unsafe fn RememberTemporaryValidityOverrideUsingFingerprint(&self, aHostName: *const ::nsstring::nsACString, aPort: int32_t, aCertFingerprint: *const ::nsstring::nsACString, aOverrideBits: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).RememberTemporaryValidityOverrideUsingFingerprint)(self, aHostName, aPort, aCertFingerprint, aOverrideBits)
    }


    /// ```text
    /// /**
    ///    *  Return whether this host, port, cert triple has a stored override.
    ///    *  If so, the outparams will contain the specific errors that were
    ///    *  overridden, and whether the override is permanent, or only for the current
    ///    *  session.
    ///    *
    ///    *  @param aHostName The host (punycode) this mapping belongs to
    ///    *  @param aPort The port this mapping belongs to, if it is -1 then it
    ///    *         is internally treated as 443
    ///    *  @param aCert The certificate this mapping belongs to
    ///    *  @param aOverrideBits The errors that are currently overridden
    ///    *  @param aIsTemporary Whether the stored override is session-only,
    ///    *         or permanent
    ///    *  @return Whether an override has been stored for this host+port+cert
    ///    */
    /// ```
    ///

    /// `[must_use] boolean hasMatchingOverride (in AUTF8String aHostName, in int32_t aPort, in nsIX509Cert aCert, out uint32_t aOverrideBits, out boolean aIsTemporary);`
    #[inline]
    pub unsafe fn HasMatchingOverride(&self, aHostName: *const ::nsstring::nsACString, aPort: int32_t, aCert: *const nsIX509Cert, aOverrideBits: *mut uint32_t, aIsTemporary: *mut bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasMatchingOverride)(self, aHostName, aPort, aCert, aOverrideBits, aIsTemporary, _retval)
    }


    /// ```text
    /// /**
    ///    *  Remove a override for the given hostname:port.
    ///    *
    ///    *  @param aHostName The host (punycode) whose entry should be cleared.
    ///    *  @param aPort The port whose entry should be cleared.
    ///    *               If it is -1, then it is internaly treated as 443.
    ///    *               If it is 0 and aHostName is "all:temporary-certificates",
    ///    *               then all temporary certificates should be cleared.
    ///    */
    /// ```
    ///

    /// `void clearValidityOverride (in AUTF8String aHostName, in int32_t aPort);`
    #[inline]
    pub unsafe fn ClearValidityOverride(&self, aHostName: *const ::nsstring::nsACString, aPort: int32_t) -> ::nserror::nsresult {
        ((*self.vtable).ClearValidityOverride)(self, aHostName, aPort)
    }


    /// ```text
    /// /**
    ///    *  Remove all overrides.
    ///    */
    /// ```
    ///

    /// `void clearAllOverrides ();`
    #[inline]
    pub unsafe fn ClearAllOverrides(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ClearAllOverrides)(self, )
    }


    /// ```text
    /// /**
    ///    *  Is the given cert used in rules?
    ///    *
    ///    *  @param aCert The cert we're looking for
    ///    *  @return how many override entries are currently on file
    ///    *          for the given certificate
    ///    */
    /// ```
    ///

    /// `[must_use] uint32_t isCertUsedForOverrides (in nsIX509Cert aCert, in boolean aCheckTemporaries, in boolean aCheckPermanents);`
    #[inline]
    pub unsafe fn IsCertUsedForOverrides(&self, aCert: *const nsIX509Cert, aCheckTemporaries: bool, aCheckPermanents: bool, _retval: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).IsCertUsedForOverrides)(self, aCert, aCheckTemporaries, aCheckPermanents, _retval)
    }



    /// `Array<nsICertOverride> getOverrides ();`
    #[inline]
    pub unsafe fn GetOverrides(&self, _retval: *mut thin_vec::ThinVec<RefPtr<nsICertOverride>>) -> ::nserror::nsresult {
        ((*self.vtable).GetOverrides)(self, _retval)
    }


    /// ```text
    /// /**
    ///    *  NOTE: This function is used only for testing!
    ///    *
    ///    *  @param aDisable If true, disable all security check and make
    ///    *                  hasMatchingOverride always return true.
    ///    */
    /// ```
    ///

    /// `void setDisableAllSecurityChecksAndLetAttackersInterceptMyData (in boolean aDisable);`
    #[inline]
    pub unsafe fn SetDisableAllSecurityChecksAndLetAttackersInterceptMyData(&self, aDisable: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetDisableAllSecurityChecksAndLetAttackersInterceptMyData)(self, aDisable)
    }


}


