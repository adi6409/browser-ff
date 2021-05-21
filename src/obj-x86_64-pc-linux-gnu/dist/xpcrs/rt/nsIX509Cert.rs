//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIX509Cert.idl
//


/// `interface nsIX509Cert : nsISupports`
///

/// ```text
/// /**
///  * This represents a X.509 certificate.
///  *
///  * NOTE: Service workers persist x.509 certs in object form on disk.  If you
///  *       change this uuid you probably need a hack in nsBinaryInputStream to
///  *       read the old uuid.  If you change the format of the object
///  *       serialization then more complex changes will be needed.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIX509Cert {
    vtable: *const nsIX509CertVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIX509Cert.
unsafe impl XpCom for nsIX509Cert {
    const IID: nsIID = nsID(0xbdc3979a, 0x5422, 0x4cd5,
        [0x85, 0x89, 0x69, 0x6b, 0x6e, 0x96, 0xea, 0x83]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIX509Cert {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIX509Cert.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIX509CertCoerce {
    /// Cheaply cast a value of this type from a `nsIX509Cert`.
    fn coerce_from(v: &nsIX509Cert) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIX509CertCoerce for nsIX509Cert {
    #[inline]
    fn coerce_from(v: &nsIX509Cert) -> &Self {
        v
    }
}

impl nsIX509Cert {
    /// Cast this `nsIX509Cert` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIX509CertCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIX509Cert {
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
impl<T: nsISupportsCoerce> nsIX509CertCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIX509Cert) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIX509Cert
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIX509CertVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString emailAddress; */
    pub GetEmailAddress: unsafe extern "system" fn (this: *const nsIX509Cert, aEmailAddress: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute bool isBuiltInRoot; */
    pub GetIsBuiltInRoot: unsafe extern "system" fn (this: *const nsIX509Cert, aIsBuiltInRoot: *mut bool) -> ::nserror::nsresult,

    /* [must_use] Array<AString> getEmailAddresses (); */
    pub GetEmailAddresses: unsafe extern "system" fn (this: *const nsIX509Cert, _retval: *mut thin_vec::ThinVec<::nsstring::nsString>) -> ::nserror::nsresult,

    /* [must_use] boolean containsEmailAddress (in AString aEmailAddress); */
    pub ContainsEmailAddress: unsafe extern "system" fn (this: *const nsIX509Cert, aEmailAddress: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute AString subjectName; */
    pub GetSubjectName: unsafe extern "system" fn (this: *const nsIX509Cert, aSubjectName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString commonName; */
    pub GetCommonName: unsafe extern "system" fn (this: *const nsIX509Cert, aCommonName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString organization; */
    pub GetOrganization: unsafe extern "system" fn (this: *const nsIX509Cert, aOrganization: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AString organizationalUnit; */
    pub GetOrganizationalUnit: unsafe extern "system" fn (this: *const nsIX509Cert, aOrganizationalUnit: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString sha256Fingerprint; */
    pub GetSha256Fingerprint: unsafe extern "system" fn (this: *const nsIX509Cert, aSha256Fingerprint: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AString sha1Fingerprint; */
    pub GetSha1Fingerprint: unsafe extern "system" fn (this: *const nsIX509Cert, aSha1Fingerprint: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString tokenName; */
    pub GetTokenName: unsafe extern "system" fn (this: *const nsIX509Cert, aTokenName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString issuerName; */
    pub GetIssuerName: unsafe extern "system" fn (this: *const nsIX509Cert, aIssuerName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AString serialNumber; */
    pub GetSerialNumber: unsafe extern "system" fn (this: *const nsIX509Cert, aSerialNumber: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AString issuerCommonName; */
    pub GetIssuerCommonName: unsafe extern "system" fn (this: *const nsIX509Cert, aIssuerCommonName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString issuerOrganization; */
    pub GetIssuerOrganization: unsafe extern "system" fn (this: *const nsIX509Cert, aIssuerOrganization: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AString issuerOrganizationUnit; */
    pub GetIssuerOrganizationUnit: unsafe extern "system" fn (this: *const nsIX509Cert, aIssuerOrganizationUnit: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute nsIX509CertValidity validity; */
    pub GetValidity: unsafe extern "system" fn (this: *const nsIX509Cert, aValidity: *mut*const nsIX509CertValidity) -> ::nserror::nsresult,

    /* [must_use] readonly attribute ACString dbKey; */
    pub GetDbKey: unsafe extern "system" fn (this: *const nsIX509Cert, aDbKey: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AString displayName; */
    pub GetDisplayName: unsafe extern "system" fn (this: *const nsIX509Cert, aDisplayName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute unsigned long certType; */
    pub GetCertType: unsafe extern "system" fn (this: *const nsIX509Cert, aCertType: *mut u32) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AString keyUsages; */
    pub GetKeyUsages: unsafe extern "system" fn (this: *const nsIX509Cert, aKeyUsages: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [must_use] Array<octet> getRawDER (); */
    pub GetRawDER: unsafe extern "system" fn (this: *const nsIX509Cert, _retval: *mut thin_vec::ThinVec<u8>) -> ::nserror::nsresult,

    /* [must_use] ACString getBase64DERString (); */
    pub GetBase64DERString: unsafe extern "system" fn (this: *const nsIX509Cert, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] boolean equals (in nsIX509Cert other); */
    pub Equals: unsafe extern "system" fn (this: *const nsIX509Cert, other: *const nsIX509Cert, _retval: *mut bool) -> ::nserror::nsresult,

    /* [must_use] readonly attribute ACString sha256SubjectPublicKeyInfoDigest; */
    pub GetSha256SubjectPublicKeyInfoDigest: unsafe extern "system" fn (this: *const nsIX509Cert, aSha256SubjectPublicKeyInfoDigest: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use,noscript,notxpcom] CERTCertificatePtr getCert (); */
    /// Unable to generate binding because `native type CERTCertificate unsupported`
    pub GetCert: *const ::libc::c_void,

    /* [noscript,notxpcom] void SerializeToIPC (in IpcMessagePtr aMsg); */
    /// Unable to generate binding because `native type IPC::Message unsupported`
    pub SerializeToIPC: *const ::libc::c_void,

    /* [noscript,notxpcom] bool DeserializeFromIPC ([const] in IpcMessagePtr aMsg, in PickleIteratorPtr aIter); */
    /// Unable to generate binding because `native type IPC::Message unsupported`
    pub DeserializeFromIPC: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIX509Cert {
    /// ```text
    /// /**
    ///    *  Constants to classify the type of a certificate.
    ///    */
    /// ```
    ///

    pub const UNKNOWN_CERT: i64 = 0;


    pub const CA_CERT: i64 = 1;


    pub const USER_CERT: i64 = 2;


    pub const EMAIL_CERT: i64 = 4;


    pub const SERVER_CERT: i64 = 8;


    pub const ANY_CERT: i64 = 65535;

    /// ```text
    /// /**
    ///    *  The primary email address of the certificate, if present.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString emailAddress;`
    #[inline]
    pub unsafe fn GetEmailAddress(&self, aEmailAddress: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetEmailAddress)(self, aEmailAddress)
    }


    /// ```text
    /// /**
    ///    * Did this certificate ship with the platform as a built-in root?
    ///    */
    /// ```
    ///

    /// `[must_use] readonly attribute bool isBuiltInRoot;`
    #[inline]
    pub unsafe fn GetIsBuiltInRoot(&self, aIsBuiltInRoot: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsBuiltInRoot)(self, aIsBuiltInRoot)
    }


    /// ```text
    /// /**
    ///    *  Obtain a list of all email addresses
    ///    *  contained in the certificate.
    ///    *
    ///    *  @return An array of email addresses.
    ///    */
    /// ```
    ///

    /// `[must_use] Array<AString> getEmailAddresses ();`
    #[inline]
    pub unsafe fn GetEmailAddresses(&self, _retval: *mut thin_vec::ThinVec<::nsstring::nsString>) -> ::nserror::nsresult {
        ((*self.vtable).GetEmailAddresses)(self, _retval)
    }


    /// ```text
    /// /**
    ///    *  Check whether a given address is contained in the certificate.
    ///    *  The comparison will convert the email address to lowercase.
    ///    *  The behaviour for non ASCII characters is undefined.
    ///    *
    ///    *  @param aEmailAddress The address to search for.
    ///    *
    ///    *  @return True if the address is contained in the certificate.
    ///    */
    /// ```
    ///

    /// `[must_use] boolean containsEmailAddress (in AString aEmailAddress);`
    #[inline]
    pub unsafe fn ContainsEmailAddress(&self, aEmailAddress: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ContainsEmailAddress)(self, aEmailAddress, _retval)
    }


    /// ```text
    /// /**
    ///    *  The subject owning the certificate.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString subjectName;`
    #[inline]
    pub unsafe fn GetSubjectName(&self, aSubjectName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSubjectName)(self, aSubjectName)
    }


    /// ```text
    /// /**
    ///    *  The subject's common name.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString commonName;`
    #[inline]
    pub unsafe fn GetCommonName(&self, aCommonName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetCommonName)(self, aCommonName)
    }


    /// ```text
    /// /**
    ///    *  The subject's organization.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString organization;`
    #[inline]
    pub unsafe fn GetOrganization(&self, aOrganization: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetOrganization)(self, aOrganization)
    }


    /// ```text
    /// /**
    ///    *  The subject's organizational unit.
    ///    */
    /// ```
    ///

    /// `[must_use] readonly attribute AString organizationalUnit;`
    #[inline]
    pub unsafe fn GetOrganizationalUnit(&self, aOrganizationalUnit: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetOrganizationalUnit)(self, aOrganizationalUnit)
    }


    /// ```text
    /// /**
    ///    *  The fingerprint of the certificate's DER encoding,
    ///    *  calculated using the SHA-256 algorithm.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString sha256Fingerprint;`
    #[inline]
    pub unsafe fn GetSha256Fingerprint(&self, aSha256Fingerprint: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSha256Fingerprint)(self, aSha256Fingerprint)
    }


    /// ```text
    /// /**
    ///    *  The fingerprint of the certificate's DER encoding,
    ///    *  calculated using the SHA1 algorithm.
    ///    */
    /// ```
    ///

    /// `[must_use] readonly attribute AString sha1Fingerprint;`
    #[inline]
    pub unsafe fn GetSha1Fingerprint(&self, aSha1Fingerprint: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSha1Fingerprint)(self, aSha1Fingerprint)
    }


    /// ```text
    /// /**
    ///    *  A human readable name identifying the hardware or
    ///    *  software token the certificate is stored on.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString tokenName;`
    #[inline]
    pub unsafe fn GetTokenName(&self, aTokenName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetTokenName)(self, aTokenName)
    }


    /// ```text
    /// /**
    ///    *  The subject identifying the issuer certificate.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString issuerName;`
    #[inline]
    pub unsafe fn GetIssuerName(&self, aIssuerName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetIssuerName)(self, aIssuerName)
    }


    /// ```text
    /// /**
    ///    *  The serial number the issuer assigned to this certificate.
    ///    */
    /// ```
    ///

    /// `[must_use] readonly attribute AString serialNumber;`
    #[inline]
    pub unsafe fn GetSerialNumber(&self, aSerialNumber: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSerialNumber)(self, aSerialNumber)
    }


    /// ```text
    /// /**
    ///    *  The issuer subject's common name.
    ///    */
    /// ```
    ///

    /// `[must_use] readonly attribute AString issuerCommonName;`
    #[inline]
    pub unsafe fn GetIssuerCommonName(&self, aIssuerCommonName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetIssuerCommonName)(self, aIssuerCommonName)
    }


    /// ```text
    /// /**
    ///    *  The issuer subject's organization.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString issuerOrganization;`
    #[inline]
    pub unsafe fn GetIssuerOrganization(&self, aIssuerOrganization: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetIssuerOrganization)(self, aIssuerOrganization)
    }


    /// ```text
    /// /**
    ///    *  The issuer subject's organizational unit.
    ///    */
    /// ```
    ///

    /// `[must_use] readonly attribute AString issuerOrganizationUnit;`
    #[inline]
    pub unsafe fn GetIssuerOrganizationUnit(&self, aIssuerOrganizationUnit: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetIssuerOrganizationUnit)(self, aIssuerOrganizationUnit)
    }


    /// ```text
    /// /**
    ///    *  This certificate's validity period.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIX509CertValidity validity;`
    #[inline]
    pub unsafe fn GetValidity(&self, aValidity: *mut*const nsIX509CertValidity) -> ::nserror::nsresult {
        ((*self.vtable).GetValidity)(self, aValidity)
    }


    /// ```text
    /// /**
    ///    *  A unique identifier of this certificate within the local storage.
    ///    */
    /// ```
    ///

    /// `[must_use] readonly attribute ACString dbKey;`
    #[inline]
    pub unsafe fn GetDbKey(&self, aDbKey: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetDbKey)(self, aDbKey)
    }


    /// ```text
    /// /**
    ///    *  A human readable identifier to label this certificate.
    ///    */
    /// ```
    ///

    /// `[must_use] readonly attribute AString displayName;`
    #[inline]
    pub unsafe fn GetDisplayName(&self, aDisplayName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetDisplayName)(self, aDisplayName)
    }


    /// ```text
    /// /**
    ///    * Type of this certificate
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long certType;`
    #[inline]
    pub unsafe fn GetCertType(&self, aCertType: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetCertType)(self, aCertType)
    }


    /// ```text
    /// /**
    ///    * A comma separated list of localized strings representing the contents of
    ///    * the certificate's key usage extension, if present. The empty string if the
    ///    * certificate doesn't have the key usage extension, or has an empty extension.
    ///    */
    /// ```
    ///

    /// `[must_use] readonly attribute AString keyUsages;`
    #[inline]
    pub unsafe fn GetKeyUsages(&self, aKeyUsages: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetKeyUsages)(self, aKeyUsages)
    }


    /// ```text
    /// /**
    ///    *  Obtain a raw binary encoding of this certificate
    ///    *  in DER format.
    ///    *
    ///    *  @return The bytes representing the DER encoded certificate.
    ///    */
    /// ```
    ///

    /// `[must_use] Array<octet> getRawDER ();`
    #[inline]
    pub unsafe fn GetRawDER(&self, _retval: *mut thin_vec::ThinVec<u8>) -> ::nserror::nsresult {
        ((*self.vtable).GetRawDER)(self, _retval)
    }


    /// ```text
    /// /**
    ///    *  Obtain a base 64 string representation of this certificate
    ///    *  in DER format.
    ///    *
    ///    *  @return The DER encoded certificate as a string.
    ///    */
    /// ```
    ///

    /// `[must_use] ACString getBase64DERString ();`
    #[inline]
    pub unsafe fn GetBase64DERString(&self, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetBase64DERString)(self, _retval)
    }


    /// ```text
    /// /**
    ///    *  Test whether two certificate instances represent the
    ///    *  same certificate.
    ///    *
    ///    *  @return Whether the certificates are equal
    ///    */
    /// ```
    ///

    /// `[must_use] boolean equals (in nsIX509Cert other);`
    #[inline]
    pub unsafe fn Equals(&self, other: *const nsIX509Cert, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Equals)(self, other, _retval)
    }


    /// ```text
    /// /**
    ///    * The base64 encoding of the DER encoded public key info using the specified
    ///    * digest.
    ///    */
    /// ```
    ///

    /// `[must_use] readonly attribute ACString sha256SubjectPublicKeyInfoDigest;`
    #[inline]
    pub unsafe fn GetSha256SubjectPublicKeyInfoDigest(&self, aSha256SubjectPublicKeyInfoDigest: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetSha256SubjectPublicKeyInfoDigest)(self, aSha256SubjectPublicKeyInfoDigest)
    }


    /// ```text
    /// /**
    ///    * Retrieves the NSS certificate object wrapped by this interface
    ///    */
    /// ```
    ///

    /// `[must_use,noscript,notxpcom] CERTCertificatePtr getCert ();`
    const _GetCert: () = ();


    /// `[noscript,notxpcom] void SerializeToIPC (in IpcMessagePtr aMsg);`
    const _SerializeToIPC: () = ();


    /// `[noscript,notxpcom] bool DeserializeFromIPC ([const] in IpcMessagePtr aMsg, in PickleIteratorPtr aIter);`
    const _DeserializeFromIPC: () = ();

}


