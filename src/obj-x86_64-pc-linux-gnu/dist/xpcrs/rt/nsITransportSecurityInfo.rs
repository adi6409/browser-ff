//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/socket/nsITransportSecurityInfo.idl
//


/// `interface nsITransportSecurityInfo : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITransportSecurityInfo {
    vtable: *const nsITransportSecurityInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITransportSecurityInfo.
unsafe impl XpCom for nsITransportSecurityInfo {
    const IID: nsIID = nsID(0x216112d3, 0x28bc, 0x4671,
        [0xb0, 0x57, 0xf9, 0x8c, 0xc0, 0x9b, 0xa1, 0xea]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITransportSecurityInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITransportSecurityInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITransportSecurityInfoCoerce {
    /// Cheaply cast a value of this type from a `nsITransportSecurityInfo`.
    fn coerce_from(v: &nsITransportSecurityInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITransportSecurityInfoCoerce for nsITransportSecurityInfo {
    #[inline]
    fn coerce_from(v: &nsITransportSecurityInfo) -> &Self {
        v
    }
}

impl nsITransportSecurityInfo {
    /// Cast this `nsITransportSecurityInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITransportSecurityInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITransportSecurityInfo {
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
impl<T: nsISupportsCoerce> nsITransportSecurityInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITransportSecurityInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITransportSecurityInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITransportSecurityInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long securityState; */
    pub GetSecurityState: unsafe extern "system" fn (this: *const nsITransportSecurityInfo, aSecurityState: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute long errorCode; */
    pub GetErrorCode: unsafe extern "system" fn (this: *const nsITransportSecurityInfo, aErrorCode: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute AString errorCodeString; */
    pub GetErrorCodeString: unsafe extern "system" fn (this: *const nsITransportSecurityInfo, aErrorCodeString: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute Array<nsIX509Cert> failedCertChain; */
    pub GetFailedCertChain: unsafe extern "system" fn (this: *const nsITransportSecurityInfo, aFailedCertChain: *mut thin_vec::ThinVec<RefPtr<nsIX509Cert>>) -> ::nserror::nsresult,

    /* readonly attribute nsIX509Cert serverCert; */
    pub GetServerCert: unsafe extern "system" fn (this: *const nsITransportSecurityInfo, aServerCert: *mut*const nsIX509Cert) -> ::nserror::nsresult,

    /* readonly attribute Array<nsIX509Cert> succeededCertChain; */
    pub GetSucceededCertChain: unsafe extern "system" fn (this: *const nsITransportSecurityInfo, aSucceededCertChain: *mut thin_vec::ThinVec<RefPtr<nsIX509Cert>>) -> ::nserror::nsresult,

    /* [must_use] readonly attribute ACString cipherName; */
    pub GetCipherName: unsafe extern "system" fn (this: *const nsITransportSecurityInfo, aCipherName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute unsigned long keyLength; */
    pub GetKeyLength: unsafe extern "system" fn (this: *const nsITransportSecurityInfo, aKeyLength: *mut u32) -> ::nserror::nsresult,

    /* [must_use] readonly attribute unsigned long secretKeyLength; */
    pub GetSecretKeyLength: unsafe extern "system" fn (this: *const nsITransportSecurityInfo, aSecretKeyLength: *mut u32) -> ::nserror::nsresult,

    /* [must_use] readonly attribute ACString keaGroupName; */
    pub GetKeaGroupName: unsafe extern "system" fn (this: *const nsITransportSecurityInfo, aKeaGroupName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute ACString signatureSchemeName; */
    pub GetSignatureSchemeName: unsafe extern "system" fn (this: *const nsITransportSecurityInfo, aSignatureSchemeName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute unsigned short protocolVersion; */
    pub GetProtocolVersion: unsafe extern "system" fn (this: *const nsITransportSecurityInfo, aProtocolVersion: *mut u16) -> ::nserror::nsresult,

    /* [must_use] readonly attribute unsigned short certificateTransparencyStatus; */
    pub GetCertificateTransparencyStatus: unsafe extern "system" fn (this: *const nsITransportSecurityInfo, aCertificateTransparencyStatus: *mut u16) -> ::nserror::nsresult,

    /* [must_use] readonly attribute boolean isAcceptedEch; */
    pub GetIsAcceptedEch: unsafe extern "system" fn (this: *const nsITransportSecurityInfo, aIsAcceptedEch: *mut bool) -> ::nserror::nsresult,

    /* [must_use] readonly attribute boolean isDelegatedCredential; */
    pub GetIsDelegatedCredential: unsafe extern "system" fn (this: *const nsITransportSecurityInfo, aIsDelegatedCredential: *mut bool) -> ::nserror::nsresult,

    /* [must_use] readonly attribute boolean isDomainMismatch; */
    pub GetIsDomainMismatch: unsafe extern "system" fn (this: *const nsITransportSecurityInfo, aIsDomainMismatch: *mut bool) -> ::nserror::nsresult,

    /* [must_use] readonly attribute boolean isNotValidAtThisTime; */
    pub GetIsNotValidAtThisTime: unsafe extern "system" fn (this: *const nsITransportSecurityInfo, aIsNotValidAtThisTime: *mut bool) -> ::nserror::nsresult,

    /* [must_use] readonly attribute boolean isUntrusted; */
    pub GetIsUntrusted: unsafe extern "system" fn (this: *const nsITransportSecurityInfo, aIsUntrusted: *mut bool) -> ::nserror::nsresult,

    /* [must_use] readonly attribute boolean isExtendedValidation; */
    pub GetIsExtendedValidation: unsafe extern "system" fn (this: *const nsITransportSecurityInfo, aIsExtendedValidation: *mut bool) -> ::nserror::nsresult,

    /* [noscript,notxpcom] void SerializeToIPC (in IpcMessagePtr aMsg); */
    /// Unable to generate binding because `native type IPC::Message unsupported`
    pub SerializeToIPC: *const ::libc::c_void,

    /* [noscript,notxpcom] bool DeserializeFromIPC ([const] in IpcMessagePtr aMsg, in PickleIteratorPtr aIter); */
    /// Unable to generate binding because `native type IPC::Message unsupported`
    pub DeserializeFromIPC: *const ::libc::c_void,

    /* readonly attribute ACString negotiatedNPN; */
    pub GetNegotiatedNPN: unsafe extern "system" fn (this: *const nsITransportSecurityInfo, aNegotiatedNPN: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute boolean resumed; */
    pub GetResumed: unsafe extern "system" fn (this: *const nsITransportSecurityInfo, aResumed: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean isBuiltCertChainRootBuiltInRoot; */
    pub GetIsBuiltCertChainRootBuiltInRoot: unsafe extern "system" fn (this: *const nsITransportSecurityInfo, aIsBuiltCertChainRootBuiltInRoot: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean isBuiltCertChainRootBuiltInRoot; */
    pub SetIsBuiltCertChainRootBuiltInRoot: unsafe extern "system" fn (this: *const nsITransportSecurityInfo, aIsBuiltCertChainRootBuiltInRoot: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITransportSecurityInfo {

    pub const SSL_VERSION_3: i64 = 0;


    pub const TLS_VERSION_1: i64 = 1;


    pub const TLS_VERSION_1_1: i64 = 2;


    pub const TLS_VERSION_1_2: i64 = 3;


    pub const TLS_VERSION_1_3: i64 = 4;


    pub const CERTIFICATE_TRANSPARENCY_NOT_APPLICABLE: i64 = 0;


    pub const CERTIFICATE_TRANSPARENCY_POLICY_COMPLIANT: i64 = 5;


    pub const CERTIFICATE_TRANSPARENCY_POLICY_NOT_ENOUGH_SCTS: i64 = 6;


    pub const CERTIFICATE_TRANSPARENCY_POLICY_NOT_DIVERSE_SCTS: i64 = 7;


    /// `readonly attribute unsigned long securityState;`
    #[inline]
    pub unsafe fn GetSecurityState(&self, aSecurityState: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetSecurityState)(self, aSecurityState)
    }



    /// `readonly attribute long errorCode;`
    #[inline]
    pub unsafe fn GetErrorCode(&self, aErrorCode: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetErrorCode)(self, aErrorCode)
    }



    /// `readonly attribute AString errorCodeString;`
    #[inline]
    pub unsafe fn GetErrorCodeString(&self, aErrorCodeString: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetErrorCodeString)(self, aErrorCodeString)
    }


    /// ```text
    /// /**
    ///      * The following parameters are only valid after the TLS handshake
    ///      * has completed.  Check securityState first.
    ///      */
    /// /**
    ///      * If certificate verification failed, this will be the peer certificate
    ///      * chain provided in the handshake, so it can be used for error reporting.
    ///      * If verification succeeded, this will be empty.
    ///      */
    /// ```
    ///

    /// `readonly attribute Array<nsIX509Cert> failedCertChain;`
    #[inline]
    pub unsafe fn GetFailedCertChain(&self, aFailedCertChain: *mut thin_vec::ThinVec<RefPtr<nsIX509Cert>>) -> ::nserror::nsresult {
        ((*self.vtable).GetFailedCertChain)(self, aFailedCertChain)
    }



    /// `readonly attribute nsIX509Cert serverCert;`
    #[inline]
    pub unsafe fn GetServerCert(&self, aServerCert: *mut*const nsIX509Cert) -> ::nserror::nsresult {
        ((*self.vtable).GetServerCert)(self, aServerCert)
    }



    /// `readonly attribute Array<nsIX509Cert> succeededCertChain;`
    #[inline]
    pub unsafe fn GetSucceededCertChain(&self, aSucceededCertChain: *mut thin_vec::ThinVec<RefPtr<nsIX509Cert>>) -> ::nserror::nsresult {
        ((*self.vtable).GetSucceededCertChain)(self, aSucceededCertChain)
    }



    /// `[must_use] readonly attribute ACString cipherName;`
    #[inline]
    pub unsafe fn GetCipherName(&self, aCipherName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetCipherName)(self, aCipherName)
    }



    /// `[must_use] readonly attribute unsigned long keyLength;`
    #[inline]
    pub unsafe fn GetKeyLength(&self, aKeyLength: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetKeyLength)(self, aKeyLength)
    }



    /// `[must_use] readonly attribute unsigned long secretKeyLength;`
    #[inline]
    pub unsafe fn GetSecretKeyLength(&self, aSecretKeyLength: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetSecretKeyLength)(self, aSecretKeyLength)
    }



    /// `[must_use] readonly attribute ACString keaGroupName;`
    #[inline]
    pub unsafe fn GetKeaGroupName(&self, aKeaGroupName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetKeaGroupName)(self, aKeaGroupName)
    }



    /// `[must_use] readonly attribute ACString signatureSchemeName;`
    #[inline]
    pub unsafe fn GetSignatureSchemeName(&self, aSignatureSchemeName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetSignatureSchemeName)(self, aSignatureSchemeName)
    }



    /// `[must_use] readonly attribute unsigned short protocolVersion;`
    #[inline]
    pub unsafe fn GetProtocolVersion(&self, aProtocolVersion: *mut u16) -> ::nserror::nsresult {
        ((*self.vtable).GetProtocolVersion)(self, aProtocolVersion)
    }



    /// `[must_use] readonly attribute unsigned short certificateTransparencyStatus;`
    #[inline]
    pub unsafe fn GetCertificateTransparencyStatus(&self, aCertificateTransparencyStatus: *mut u16) -> ::nserror::nsresult {
        ((*self.vtable).GetCertificateTransparencyStatus)(self, aCertificateTransparencyStatus)
    }



    /// `[must_use] readonly attribute boolean isAcceptedEch;`
    #[inline]
    pub unsafe fn GetIsAcceptedEch(&self, aIsAcceptedEch: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsAcceptedEch)(self, aIsAcceptedEch)
    }



    /// `[must_use] readonly attribute boolean isDelegatedCredential;`
    #[inline]
    pub unsafe fn GetIsDelegatedCredential(&self, aIsDelegatedCredential: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsDelegatedCredential)(self, aIsDelegatedCredential)
    }



    /// `[must_use] readonly attribute boolean isDomainMismatch;`
    #[inline]
    pub unsafe fn GetIsDomainMismatch(&self, aIsDomainMismatch: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsDomainMismatch)(self, aIsDomainMismatch)
    }



    /// `[must_use] readonly attribute boolean isNotValidAtThisTime;`
    #[inline]
    pub unsafe fn GetIsNotValidAtThisTime(&self, aIsNotValidAtThisTime: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsNotValidAtThisTime)(self, aIsNotValidAtThisTime)
    }



    /// `[must_use] readonly attribute boolean isUntrusted;`
    #[inline]
    pub unsafe fn GetIsUntrusted(&self, aIsUntrusted: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsUntrusted)(self, aIsUntrusted)
    }


    /// ```text
    /// /**
    ///      * True only if (and after) serverCert was successfully validated as
    ///      * Extended Validation (EV).
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute boolean isExtendedValidation;`
    #[inline]
    pub unsafe fn GetIsExtendedValidation(&self, aIsExtendedValidation: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsExtendedValidation)(self, aIsExtendedValidation)
    }



    /// `[noscript,notxpcom] void SerializeToIPC (in IpcMessagePtr aMsg);`
    const _SerializeToIPC: () = ();


    /// `[noscript,notxpcom] bool DeserializeFromIPC ([const] in IpcMessagePtr aMsg, in PickleIteratorPtr aIter);`
    const _DeserializeFromIPC: () = ();


    /// `readonly attribute ACString negotiatedNPN;`
    #[inline]
    pub unsafe fn GetNegotiatedNPN(&self, aNegotiatedNPN: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetNegotiatedNPN)(self, aNegotiatedNPN)
    }


    /// ```text
    /// /**
    ///      * True iff the connection was resumed using the resumption token.
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean resumed;`
    #[inline]
    pub unsafe fn GetResumed(&self, aResumed: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetResumed)(self, aResumed)
    }


    /// ```text
    /// /**
    ///      * True iff the succeededCertChain is built in root.
    ///      */
    /// ```
    ///

    /// `attribute boolean isBuiltCertChainRootBuiltInRoot;`
    #[inline]
    pub unsafe fn GetIsBuiltCertChainRootBuiltInRoot(&self, aIsBuiltCertChainRootBuiltInRoot: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsBuiltCertChainRootBuiltInRoot)(self, aIsBuiltCertChainRootBuiltInRoot)
    }


    /// ```text
    /// /**
    ///      * True iff the succeededCertChain is built in root.
    ///      */
    /// ```
    ///

    /// `attribute boolean isBuiltCertChainRootBuiltInRoot;`
    #[inline]
    pub unsafe fn SetIsBuiltCertChainRootBuiltInRoot(&self, aIsBuiltCertChainRootBuiltInRoot: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetIsBuiltCertChainRootBuiltInRoot)(self, aIsBuiltCertChainRootBuiltInRoot)
    }


}


