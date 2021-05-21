//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/caps/nsIPrincipal.idl
//


/// `interface nsIPrincipal : nsISerializable`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPrincipal {
    vtable: *const nsIPrincipalVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPrincipal.
unsafe impl XpCom for nsIPrincipal {
    const IID: nsIID = nsID(0xf75f502d, 0x79fd, 0x48be,
        [0xa0, 0x79, 0xe5, 0xa7, 0xb8, 0xf8, 0x0c, 0x8b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPrincipal {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPrincipal.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPrincipalCoerce {
    /// Cheaply cast a value of this type from a `nsIPrincipal`.
    fn coerce_from(v: &nsIPrincipal) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPrincipalCoerce for nsIPrincipal {
    #[inline]
    fn coerce_from(v: &nsIPrincipal) -> &Self {
        v
    }
}

impl nsIPrincipal {
    /// Cast this `nsIPrincipal` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPrincipalCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPrincipal {
    type Target = nsISerializable;
    #[inline]
    fn deref(&self) -> &nsISerializable {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISerializableCoerce> nsIPrincipalCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrincipal) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPrincipal
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPrincipalVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISerializableVTable,

    /* boolean equals (in nsIPrincipal other); */
    pub Equals: unsafe extern "system" fn (this: *const nsIPrincipal, other: *const nsIPrincipal, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean equalsForPermission (in nsIPrincipal other, in bool aExactHost); */
    pub EqualsForPermission: unsafe extern "system" fn (this: *const nsIPrincipal, other: *const nsIPrincipal, aExactHost: bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean equalsConsideringDomain (in nsIPrincipal other); */
    pub EqualsConsideringDomain: unsafe extern "system" fn (this: *const nsIPrincipal, other: *const nsIPrincipal, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean equalsURI (in nsIURI aOtherURI); */
    pub EqualsURI: unsafe extern "system" fn (this: *const nsIPrincipal, aOtherURI: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult,

    /* [nostdcall,notxpcom] readonly attribute unsigned long hashValue; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetHashValue: *const ::libc::c_void,

    /* [infallible] readonly attribute nsIURI URI; */
    pub GetURI: unsafe extern "system" fn (this: *const nsIPrincipal, aURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* [noscript] attribute nsIURI domain; */
    pub GetDomain: unsafe extern "system" fn (this: *const nsIPrincipal, aDomain: *mut*const nsIURI) -> ::nserror::nsresult,

    /* [noscript] attribute nsIURI domain; */
    pub SetDomain: unsafe extern "system" fn (this: *const nsIPrincipal, aDomain: *const nsIURI) -> ::nserror::nsresult,

    /* boolean subsumes (in nsIPrincipal other); */
    pub Subsumes: unsafe extern "system" fn (this: *const nsIPrincipal, other: *const nsIPrincipal, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean subsumesConsideringDomain (in nsIPrincipal other); */
    pub SubsumesConsideringDomain: unsafe extern "system" fn (this: *const nsIPrincipal, other: *const nsIPrincipal, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean subsumesConsideringDomainIgnoringFPD (in nsIPrincipal other); */
    pub SubsumesConsideringDomainIgnoringFPD: unsafe extern "system" fn (this: *const nsIPrincipal, other: *const nsIPrincipal, _retval: *mut bool) -> ::nserror::nsresult,

    /* void checkMayLoad (in nsIURI uri, in boolean allowIfInheritsPrincipal); */
    pub CheckMayLoad: unsafe extern "system" fn (this: *const nsIPrincipal, uri: *const nsIURI, allowIfInheritsPrincipal: bool) -> ::nserror::nsresult,

    /* void checkMayLoadWithReporting (in nsIURI uri, in boolean allowIfInheritsPrincipal, in unsigned long long innerWindowID); */
    pub CheckMayLoadWithReporting: unsafe extern "system" fn (this: *const nsIPrincipal, uri: *const nsIURI, allowIfInheritsPrincipal: bool, innerWindowID: u64) -> ::nserror::nsresult,

    /* boolean isThirdPartyURI (in nsIURI uri); */
    pub IsThirdPartyURI: unsafe extern "system" fn (this: *const nsIPrincipal, uri: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean isThirdPartyPrincipal (in nsIPrincipal principal); */
    pub IsThirdPartyPrincipal: unsafe extern "system" fn (this: *const nsIPrincipal, principal: *const nsIPrincipal, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean isThirdPartyChannel (in nsIChannel channel); */
    pub IsThirdPartyChannel: unsafe extern "system" fn (this: *const nsIPrincipal, channel: *const nsIChannel, _retval: *mut bool) -> ::nserror::nsresult,

    /* [implicit_jscontext] readonly attribute jsval originAttributes; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetOriginAttributes: *const ::libc::c_void,

    /* [binaryname(OriginAttributesRef),noscript,nostdcall,notxpcom] const_OriginAttributes OriginAttributesRef (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub OriginAttributesRef: *const ::libc::c_void,

    /* readonly attribute ACString origin; */
    pub GetOrigin: unsafe extern "system" fn (this: *const nsIPrincipal, aOrigin: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [noscript] readonly attribute ACString asciiOrigin; */
    pub GetAsciiOrigin: unsafe extern "system" fn (this: *const nsIPrincipal, aAsciiOrigin: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString hostPort; */
    pub GetHostPort: unsafe extern "system" fn (this: *const nsIPrincipal, aHostPort: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString asciiHost; */
    pub GetAsciiHost: unsafe extern "system" fn (this: *const nsIPrincipal, aAsciiHost: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString host; */
    pub GetHost: unsafe extern "system" fn (this: *const nsIPrincipal, aHost: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString prepath; */
    pub GetPrepath: unsafe extern "system" fn (this: *const nsIPrincipal, aPrepath: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString filePath; */
    pub GetFilePath: unsafe extern "system" fn (this: *const nsIPrincipal, aFilePath: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString asciiSpec; */
    pub GetAsciiSpec: unsafe extern "system" fn (this: *const nsIPrincipal, aAsciiSpec: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString spec; */
    pub GetSpec: unsafe extern "system" fn (this: *const nsIPrincipal, aSpec: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString exposablePrePath; */
    pub GetExposablePrePath: unsafe extern "system" fn (this: *const nsIPrincipal, aExposablePrePath: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString exposableSpec; */
    pub GetExposableSpec: unsafe extern "system" fn (this: *const nsIPrincipal, aExposableSpec: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString scheme; */
    pub GetScheme: unsafe extern "system" fn (this: *const nsIPrincipal, aScheme: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* boolean schemeIs (in string scheme); */
    pub SchemeIs: unsafe extern "system" fn (this: *const nsIPrincipal, scheme: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult,

    /* bool isURIInPrefList (in string pref); */
    pub IsURIInPrefList: unsafe extern "system" fn (this: *const nsIPrincipal, pref: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult,

    /* bool isSameOrigin (in nsIURI otherURI, in bool aIsPrivateWin); */
    pub IsSameOrigin: unsafe extern "system" fn (this: *const nsIPrincipal, otherURI: *const nsIURI, aIsPrivateWin: bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* bool allowsRelaxStrictFileOriginPolicy (in nsIURI aURI); */
    pub AllowsRelaxStrictFileOriginPolicy: unsafe extern "system" fn (this: *const nsIPrincipal, aURI: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult,

    /* [noscript] ACString getPrefLightCacheKey (in nsIURI aURI, in bool aWithCredentials, in const_OriginAttributes aOriginAttributes); */
    /// Unable to generate binding because `native type const mozilla::OriginAttributes unsupported`
    pub GetPrefLightCacheKey: *const ::libc::c_void,

    /* bool hasFirstpartyStorageAccess (in mozIDOMWindow aWindow, out uint32_t rejectedReason); */
    pub HasFirstpartyStorageAccess: unsafe extern "system" fn (this: *const nsIPrincipal, aWindow: *const mozIDOMWindow, rejectedReason: *mut uint32_t, _retval: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute ACString localStorageQuotaKey; */
    pub GetLocalStorageQuotaKey: unsafe extern "system" fn (this: *const nsIPrincipal, aLocalStorageQuotaKey: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean isOriginPotentiallyTrustworthy; */
    pub GetIsOriginPotentiallyTrustworthy: unsafe extern "system" fn (this: *const nsIPrincipal, aIsOriginPotentiallyTrustworthy: *mut bool) -> ::nserror::nsresult,

    /* uint32_t getAboutModuleFlags (); */
    pub GetAboutModuleFlags: unsafe extern "system" fn (this: *const nsIPrincipal, _retval: *mut uint32_t) -> ::nserror::nsresult,

    /* readonly attribute ACString storageOriginKey; */
    pub GetStorageOriginKey: unsafe extern "system" fn (this: *const nsIPrincipal, aStorageOriginKey: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* nsIReferrerInfo createReferrerInfo (in ReferrerPolicy aReferrerPolicy); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub CreateReferrerInfo: *const ::libc::c_void,

    /* readonly attribute ACString originNoSuffix; */
    pub GetOriginNoSuffix: unsafe extern "system" fn (this: *const nsIPrincipal, aOriginNoSuffix: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String originSuffix; */
    pub GetOriginSuffix: unsafe extern "system" fn (this: *const nsIPrincipal, aOriginSuffix: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString siteOrigin; */
    pub GetSiteOrigin: unsafe extern "system" fn (this: *const nsIPrincipal, aSiteOrigin: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString siteOriginNoSuffix; */
    pub GetSiteOriginNoSuffix: unsafe extern "system" fn (this: *const nsIPrincipal, aSiteOriginNoSuffix: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString baseDomain; */
    pub GetBaseDomain: unsafe extern "system" fn (this: *const nsIPrincipal, aBaseDomain: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AString addonId; */
    pub GetAddonId: unsafe extern "system" fn (this: *const nsIPrincipal, aAddonId: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute nsISupports addonPolicy; */
    pub GetAddonPolicy: unsafe extern "system" fn (this: *const nsIPrincipal, aAddonPolicy: *mut *const nsISupports) -> ::nserror::nsresult,

    /* [infallible] readonly attribute unsigned long userContextId; */
    pub GetUserContextId: unsafe extern "system" fn (this: *const nsIPrincipal, aUserContextId: *mut u32) -> ::nserror::nsresult,

    /* [infallible] readonly attribute unsigned long privateBrowsingId; */
    pub GetPrivateBrowsingId: unsafe extern "system" fn (this: *const nsIPrincipal, aPrivateBrowsingId: *mut u32) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean isInIsolatedMozBrowserElement; */
    pub GetIsInIsolatedMozBrowserElement: unsafe extern "system" fn (this: *const nsIPrincipal, aIsInIsolatedMozBrowserElement: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean isNullPrincipal; */
    pub GetIsNullPrincipal: unsafe extern "system" fn (this: *const nsIPrincipal, aIsNullPrincipal: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean isContentPrincipal; */
    pub GetIsContentPrincipal: unsafe extern "system" fn (this: *const nsIPrincipal, aIsContentPrincipal: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean isExpandedPrincipal; */
    pub GetIsExpandedPrincipal: unsafe extern "system" fn (this: *const nsIPrincipal, aIsExpandedPrincipal: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean isSystemPrincipal; */
    pub GetIsSystemPrincipal: unsafe extern "system" fn (this: *const nsIPrincipal, aIsSystemPrincipal: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean isAddonOrExpandedAddonPrincipal; */
    pub GetIsAddonOrExpandedAddonPrincipal: unsafe extern "system" fn (this: *const nsIPrincipal, aIsAddonOrExpandedAddonPrincipal: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean isOnion; */
    pub GetIsOnion: unsafe extern "system" fn (this: *const nsIPrincipal, aIsOnion: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean isScriptAllowedByPolicy; */
    pub GetIsScriptAllowedByPolicy: unsafe extern "system" fn (this: *const nsIPrincipal, aIsScriptAllowedByPolicy: *mut bool) -> ::nserror::nsresult,

    /* boolean isL10nAllowed (in nsIURI aDocumentURI); */
    pub IsL10nAllowed: unsafe extern "system" fn (this: *const nsIPrincipal, aDocumentURI: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute nsIPrincipal nextSubDomainPrincipal; */
    pub GetNextSubDomainPrincipal: unsafe extern "system" fn (this: *const nsIPrincipal, aNextSubDomainPrincipal: *mut *const nsIPrincipal) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean isIpAddress; */
    pub GetIsIpAddress: unsafe extern "system" fn (this: *const nsIPrincipal, aIsIpAddress: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean isLocalIpAddress; */
    pub GetIsLocalIpAddress: unsafe extern "system" fn (this: *const nsIPrincipal, aIsLocalIpAddress: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPrincipal {

    /// ```text
    /// /**
    ///      * Returns whether the other principal is equivalent to this principal.
    ///      * Principals are considered equal if they are the same principal, or
    ///      * they have the same origin.
    ///      */
    /// ```
    ///

    /// `boolean equals (in nsIPrincipal other);`
    #[inline]
    pub unsafe fn Equals(&self, other: *const nsIPrincipal, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Equals)(self, other, _retval)
    }


    /// ```text
    /// /**
    ///      * Returns whether the other principal is equivalent to this principal
    ///      * for permission purposes
    ///      * Matches {originAttributes ,equalsURIForPermission}
    ///      */
    /// ```
    ///

    /// `boolean equalsForPermission (in nsIPrincipal other, in bool aExactHost);`
    #[inline]
    pub unsafe fn EqualsForPermission(&self, other: *const nsIPrincipal, aExactHost: bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).EqualsForPermission)(self, other, aExactHost, _retval)
    }


    /// ```text
    /// /**
    ///      * Like equals, but takes document.domain changes into account.
    ///      */
    /// ```
    ///

    /// `boolean equalsConsideringDomain (in nsIPrincipal other);`
    #[inline]
    pub unsafe fn EqualsConsideringDomain(&self, other: *const nsIPrincipal, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).EqualsConsideringDomain)(self, other, _retval)
    }



    /// `boolean equalsURI (in nsIURI aOtherURI);`
    #[inline]
    pub unsafe fn EqualsURI(&self, aOtherURI: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).EqualsURI)(self, aOtherURI, _retval)
    }


    /// ```text
    /// /**
    ///      * Returns a hash value for the principal.
    ///      */
    /// ```
    ///

    /// `[nostdcall,notxpcom] readonly attribute unsigned long hashValue;`
    const _GetHashValue: () = ();

    /// ```text
    /// /**
    ///      * The principal URI to which this principal pertains.  This is
    ///      * generally the document URI.
    ///      */
    /// ```
    ///

    /// `[infallible] readonly attribute nsIURI URI;`
    #[inline]
    pub unsafe fn GetURI(&self, aURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetURI)(self, aURI)
    }


    /// ```text
    /// /**
    ///      * The domain URI to which this principal pertains.
    ///      * This is null unless script successfully sets document.domain to our URI
    ///      * or a superdomain of our URI.
    ///      * Setting this has no effect on the URI.
    ///      * See https://developer.mozilla.org/en-US/docs/Web/Security/Same-origin_policy#Changing_origin
    ///      */
    /// ```
    ///

    /// `[noscript] attribute nsIURI domain;`
    #[inline]
    pub unsafe fn GetDomain(&self, aDomain: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetDomain)(self, aDomain)
    }


    /// ```text
    /// /**
    ///      * The domain URI to which this principal pertains.
    ///      * This is null unless script successfully sets document.domain to our URI
    ///      * or a superdomain of our URI.
    ///      * Setting this has no effect on the URI.
    ///      * See https://developer.mozilla.org/en-US/docs/Web/Security/Same-origin_policy#Changing_origin
    ///      */
    /// ```
    ///

    /// `[noscript] attribute nsIURI domain;`
    #[inline]
    pub unsafe fn SetDomain(&self, aDomain: *const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).SetDomain)(self, aDomain)
    }


    /// ```text
    /// /**
    ///      * Returns whether the other principal is equal to or weaker than this
    ///      * principal. Principals are equal if they are the same object or they
    ///      * have the same origin.
    ///      *
    ///      * Thus a principal always subsumes itself.
    ///      *
    ///      * The system principal subsumes itself and all other principals.
    ///      *
    ///      * A null principal (corresponding to an unknown, hence assumed minimally
        ///      * privileged, security context) is not equal to any other principal
    ///      * (including other null principals), and therefore does not subsume
    ///      * anything but itself.
    ///      */
    /// ```
    ///

    /// `boolean subsumes (in nsIPrincipal other);`
    #[inline]
    pub unsafe fn Subsumes(&self, other: *const nsIPrincipal, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Subsumes)(self, other, _retval)
    }


    /// ```text
    /// /**
    ///      * Same as the previous method, subsumes(), but takes document.domain into
    ///      * account.
    ///      */
    /// ```
    ///

    /// `boolean subsumesConsideringDomain (in nsIPrincipal other);`
    #[inline]
    pub unsafe fn SubsumesConsideringDomain(&self, other: *const nsIPrincipal, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).SubsumesConsideringDomain)(self, other, _retval)
    }


    /// ```text
    /// /**
    ///      * Same as the subsumesConsideringDomain(), but ignores the first party
    ///      * domain in its originAttributes.
    ///      */
    /// ```
    ///

    /// `boolean subsumesConsideringDomainIgnoringFPD (in nsIPrincipal other);`
    #[inline]
    pub unsafe fn SubsumesConsideringDomainIgnoringFPD(&self, other: *const nsIPrincipal, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).SubsumesConsideringDomainIgnoringFPD)(self, other, _retval)
    }


    /// ```text
    /// /**
    ///      * Checks whether this principal is allowed to load the network resource
    ///      * located at the given URI under the same-origin policy. This means that
    ///      * content principals are only allowed to load resources from the same
    ///      * domain, the system principal is allowed to load anything, and null
    ///      * principals can only load URIs where they are the principal. This is
    ///      * changed by the optional flag allowIfInheritsPrincipal (which defaults to
        ///      * false) which allows URIs that inherit their loader's principal.
    ///      *
    ///      * If the load is allowed this function does nothing. If the load is not
    ///      * allowed the function throws NS_ERROR_DOM_BAD_URI.
    ///      *
    ///      * NOTE: Other policies might override this, such as the Access-Control
    ///      *       specification.
    ///      * NOTE: The 'domain' attribute has no effect on the behaviour of this
    ///      *       function.
    ///      *
    ///      *
    ///      * @param uri    The URI about to be loaded.
    ///      * @param allowIfInheritsPrincipal   If true, the load is allowed if the
    ///      *                                   loadee inherits the principal of the
    ///      *                                   loader.
    ///      * @throws NS_ERROR_DOM_BAD_URI if the load is not allowed.
    ///      */
    /// ```
    ///

    /// `void checkMayLoad (in nsIURI uri, in boolean allowIfInheritsPrincipal);`
    #[inline]
    pub unsafe fn CheckMayLoad(&self, uri: *const nsIURI, allowIfInheritsPrincipal: bool) -> ::nserror::nsresult {
        ((*self.vtable).CheckMayLoad)(self, uri, allowIfInheritsPrincipal)
    }


    /// ```text
    /// /**
    ///      * Like checkMayLoad, but if returning an error will also report that error
    ///      * to the console, using the provided window id.  The window id may be 0 to
    ///      * report to just the browser console, not web consoles.
    ///      */
    /// ```
    ///

    /// `void checkMayLoadWithReporting (in nsIURI uri, in boolean allowIfInheritsPrincipal, in unsigned long long innerWindowID);`
    #[inline]
    pub unsafe fn CheckMayLoadWithReporting(&self, uri: *const nsIURI, allowIfInheritsPrincipal: bool, innerWindowID: u64) -> ::nserror::nsresult {
        ((*self.vtable).CheckMayLoadWithReporting)(self, uri, allowIfInheritsPrincipal, innerWindowID)
    }


    /// ```text
    /// /**
    ///     * Checks if the provided URI is considered third-party to the
    ///     * URI of the principal.
    ///     * Returns true if the URI is third-party.
    ///     *
    ///     * @param uri - The URI to check
    ///     */
    /// ```
    ///

    /// `boolean isThirdPartyURI (in nsIURI uri);`
    #[inline]
    pub unsafe fn IsThirdPartyURI(&self, uri: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsThirdPartyURI)(self, uri, _retval)
    }


    /// ```text
    /// /**
    ///     * Checks if the provided principal is considered third-party to the
    ///     * URI of the Principal.
    ///     * Returns true if the principal is third-party.
    ///     *
    ///     * @param principal - The principal to check
    ///     */
    /// ```
    ///

    /// `boolean isThirdPartyPrincipal (in nsIPrincipal principal);`
    #[inline]
    pub unsafe fn IsThirdPartyPrincipal(&self, principal: *const nsIPrincipal, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsThirdPartyPrincipal)(self, principal, _retval)
    }


    /// ```text
    /// /**
    ///     * Checks if the provided channel is considered third-party to the
    ///     * URI of the principal.
    ///     * Returns true if the channel is third-party.
    ///     * Returns false if the Principal is a System Principal
    ///     *
    ///     * @param channel - The Channel to check
    ///     */
    /// ```
    ///

    /// `boolean isThirdPartyChannel (in nsIChannel channel);`
    #[inline]
    pub unsafe fn IsThirdPartyChannel(&self, channel: *const nsIChannel, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsThirdPartyChannel)(self, channel, _retval)
    }


    /// ```text
    /// /**
    ///      * A dictionary of the non-default origin attributes associated with this
    ///      * nsIPrincipal.
    ///      *
    ///      * Attributes are tokens that are taken into account when determining whether
    ///      * two principals are same-origin - if any attributes differ, the principals
    ///      * are cross-origin, even if the scheme, host, and port are the same.
    ///      * Attributes should also be considered for all security and bucketing decisions,
    ///      * even those which make non-standard comparisons (like cookies, which ignore
        ///      * scheme, or quotas, which ignore subdomains).
    ///      *
    ///      * If you're looking for an easy-to-use canonical stringification of the origin
    ///      * attributes, see |originSuffix| below.
    ///      */
    /// ```
    ///

    /// `[implicit_jscontext] readonly attribute jsval originAttributes;`
    const _GetOriginAttributes: () = ();


    /// `[binaryname(OriginAttributesRef),noscript,nostdcall,notxpcom] const_OriginAttributes OriginAttributesRef ();`
    const _OriginAttributesRef: () = ();

    /// ```text
    /// /**
    ///      * A canonical representation of the origin for this principal. This
    ///      * consists of a base string (which, for content principals, is of the
        ///      * format scheme://host:port), concatenated with |originAttributes| (see
        ///      * below).
    ///      *
    ///      * We maintain the invariant that principalA.equals(principalB) if and only
    ///      * if principalA.origin == principalB.origin.
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString origin;`
    #[inline]
    pub unsafe fn GetOrigin(&self, aOrigin: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetOrigin)(self, aOrigin)
    }


    /// ```text
    /// /**
    ///      * Returns an ASCII compatible representation
    ///      * of the principals Origin
    ///      */
    /// ```
    ///

    /// `[noscript] readonly attribute ACString asciiOrigin;`
    #[inline]
    pub unsafe fn GetAsciiOrigin(&self, aAsciiOrigin: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetAsciiOrigin)(self, aAsciiOrigin)
    }


    /// ```text
    /// /**
    ///      * Returns the "host:port" portion of the
    ///      * Principals URI, if any.
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString hostPort;`
    #[inline]
    pub unsafe fn GetHostPort(&self, aHostPort: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetHostPort)(self, aHostPort)
    }


    /// ```text
    /// /**
    ///      * Returns the "host:port" portion of the
    ///      * Principals URI, if any.
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString asciiHost;`
    #[inline]
    pub unsafe fn GetAsciiHost(&self, aAsciiHost: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetAsciiHost)(self, aAsciiHost)
    }


    /// ```text
    /// /**
    ///      * Returns the "host" portion of the
    ///      * Principals URI, if any.
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString host;`
    #[inline]
    pub unsafe fn GetHost(&self, aHost: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetHost)(self, aHost)
    }


    /// ```text
    /// /**
    ///      * Returns the prepath of the principals uri
    ///      * follows the format scheme:
    ///      * "scheme://username:password@hostname:portnumber/"
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString prepath;`
    #[inline]
    pub unsafe fn GetPrepath(&self, aPrepath: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetPrepath)(self, aPrepath)
    }


    /// ```text
    /// /**
    ///      * Returns the filePath of the principals uri. See nsIURI.
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString filePath;`
    #[inline]
    pub unsafe fn GetFilePath(&self, aFilePath: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetFilePath)(self, aFilePath)
    }


    /// ```text
    /// /**
    ///       * Returns the ASCII Spec from the Principals URI.
    ///       * Might return the empty string, e.g. for the case of
    ///       * a SystemPrincipal or an EpxandedPrincipal.
    ///       *
    ///       * WARNING: DO NOT USE FOR SECURITY CHECKS.
    ///       * just for logging purposes!
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString asciiSpec;`
    #[inline]
    pub unsafe fn GetAsciiSpec(&self, aAsciiSpec: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetAsciiSpec)(self, aAsciiSpec)
    }


    /// ```text
    /// /**
    ///       * Returns the Spec from the Principals URI.
    ///       * Might return the empty string, e.g. for the case of
    ///       * a SystemPrincipal or an EpxandedPrincipal.
    ///       *
    ///       * WARNING: Do not land new Code using, as this will be removed soon
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString spec;`
    #[inline]
    pub unsafe fn GetSpec(&self, aSpec: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetSpec)(self, aSpec)
    }



    /// `readonly attribute ACString exposablePrePath;`
    #[inline]
    pub unsafe fn GetExposablePrePath(&self, aExposablePrePath: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetExposablePrePath)(self, aExposablePrePath)
    }



    /// `readonly attribute ACString exposableSpec;`
    #[inline]
    pub unsafe fn GetExposableSpec(&self, aExposableSpec: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetExposableSpec)(self, aExposableSpec)
    }


    /// ```text
    /// /**
    ///      * Return the scheme of the principals URI
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString scheme;`
    #[inline]
    pub unsafe fn GetScheme(&self, aScheme: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetScheme)(self, aScheme)
    }


    /// ```text
    /// /**
    ///     * Checks if the Principal's URI Scheme matches with the parameter
    ///     *
    ///     * @param scheme    The scheme to be checked
    ///     */
    /// ```
    ///

    /// `boolean schemeIs (in string scheme);`
    #[inline]
    pub unsafe fn SchemeIs(&self, scheme: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).SchemeIs)(self, scheme, _retval)
    }



    /// `bool isURIInPrefList (in string pref);`
    #[inline]
    pub unsafe fn IsURIInPrefList(&self, pref: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsURIInPrefList)(self, pref, _retval)
    }



    /// `bool isSameOrigin (in nsIURI otherURI, in bool aIsPrivateWin);`
    #[inline]
    pub unsafe fn IsSameOrigin(&self, otherURI: *const nsIURI, aIsPrivateWin: bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsSameOrigin)(self, otherURI, aIsPrivateWin, _retval)
    }



    /// `bool allowsRelaxStrictFileOriginPolicy (in nsIURI aURI);`
    #[inline]
    pub unsafe fn AllowsRelaxStrictFileOriginPolicy(&self, aURI: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).AllowsRelaxStrictFileOriginPolicy)(self, aURI, _retval)
    }



    /// `[noscript] ACString getPrefLightCacheKey (in nsIURI aURI, in bool aWithCredentials, in const_OriginAttributes aOriginAttributes);`
    const _GetPrefLightCacheKey: () = ();


    /// `bool hasFirstpartyStorageAccess (in mozIDOMWindow aWindow, out uint32_t rejectedReason);`
    #[inline]
    pub unsafe fn HasFirstpartyStorageAccess(&self, aWindow: *const mozIDOMWindow, rejectedReason: *mut uint32_t, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasFirstpartyStorageAccess)(self, aWindow, rejectedReason, _retval)
    }



    /// `readonly attribute ACString localStorageQuotaKey;`
    #[inline]
    pub unsafe fn GetLocalStorageQuotaKey(&self, aLocalStorageQuotaKey: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetLocalStorageQuotaKey)(self, aLocalStorageQuotaKey)
    }


    /// ```text
    /// /**
    ///      * Implementation of
    ///      * https://w3c.github.io/webappsec-secure-contexts/#is-origin-trustworthy
    ///      *
    ///      * The value returned by this method feeds into the the Secure Context
    ///      * algorithm that determins the value of Window.isSecureContext and
    ///      * WorkerGlobalScope.isSecureContext.
    ///      *
    ///      * This method returns false instead of throwing upon errors.
    ///      */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean isOriginPotentiallyTrustworthy;`
    #[inline]
    pub unsafe fn GetIsOriginPotentiallyTrustworthy(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetIsOriginPotentiallyTrustworthy)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///      * Returns the Flags of the Principals
    ///      * associated AboutModule, in case there is one.
    ///      */
    /// ```
    ///

    /// `uint32_t getAboutModuleFlags ();`
    #[inline]
    pub unsafe fn GetAboutModuleFlags(&self, _retval: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetAboutModuleFlags)(self, _retval)
    }



    /// `readonly attribute ACString storageOriginKey;`
    #[inline]
    pub unsafe fn GetStorageOriginKey(&self, aStorageOriginKey: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetStorageOriginKey)(self, aStorageOriginKey)
    }


    /// ```text
    /// /**
    ///      * Creates and Returns a new ReferrerInfo with the
    ///      * Principals URI
    ///      */
    /// ```
    ///

    /// `nsIReferrerInfo createReferrerInfo (in ReferrerPolicy aReferrerPolicy);`
    const _CreateReferrerInfo: () = ();

    /// ```text
    /// /**
    ///      * The base part of |origin| without the concatenation with |originSuffix|.
    ///      * This doesn't have the important invariants described above with |origin|,
    ///      * and as such should only be used for legacy situations.
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString originNoSuffix;`
    #[inline]
    pub unsafe fn GetOriginNoSuffix(&self, aOriginNoSuffix: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetOriginNoSuffix)(self, aOriginNoSuffix)
    }


    /// ```text
    /// /**
    ///      * A string of the form !key1=value1&key2=value2, where each pair represents
    ///      * an attribute with a non-default value. If all attributes have default
    ///      * values, this is the empty string.
    ///      *
    ///      * The value of .originSuffix is automatically serialized into .origin, so any
    ///      * consumers using that are automatically origin-attribute-aware. Consumers with
    ///      * special requirements must inspect and compare .originSuffix manually.
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String originSuffix;`
    #[inline]
    pub unsafe fn GetOriginSuffix(&self, aOriginSuffix: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetOriginSuffix)(self, aOriginSuffix)
    }


    /// ```text
    /// /**
    ///      * A canonical representation of the site-origin for this principal.
    ///      * This string has the same format as |origin| (see above). Two principals
    ///      * with differing |siteOrigin| values will never compare equal, even when
    ///      * considering domain mutations.
    ///      *
    ///      * For most principals, |siteOrigin| matches |origin| precisely. Only
    ///      * principals which allow mutating |domain|, such as ContentPrincipal,
    ///      * override the default implementation in BasePrincipal.
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString siteOrigin;`
    #[inline]
    pub unsafe fn GetSiteOrigin(&self, aSiteOrigin: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetSiteOrigin)(self, aSiteOrigin)
    }


    /// ```text
    /// /**
    ///      * The base part of |siteOrigin| without the concatenation with
    ///      * |originSuffix|.
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString siteOriginNoSuffix;`
    #[inline]
    pub unsafe fn GetSiteOriginNoSuffix(&self, aSiteOriginNoSuffix: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetSiteOriginNoSuffix)(self, aSiteOriginNoSuffix)
    }


    /// ```text
    /// /**
    ///      * The base domain of the principal URI to which this principal pertains
    ///      * (generally the document URI), handling null principals and
    ///      * non-hierarchical schemes correctly.
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString baseDomain;`
    #[inline]
    pub unsafe fn GetBaseDomain(&self, aBaseDomain: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetBaseDomain)(self, aBaseDomain)
    }


    /// ```text
    /// /**
    ///      * Gets the ID of the add-on this principal belongs to.
    ///      */
    /// ```
    ///

    /// `readonly attribute AString addonId;`
    #[inline]
    pub unsafe fn GetAddonId(&self, aAddonId: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetAddonId)(self, aAddonId)
    }



    /// `readonly attribute nsISupports addonPolicy;`
    #[inline]
    pub unsafe fn GetAddonPolicy(&self, aAddonPolicy: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).GetAddonPolicy)(self, aAddonPolicy)
    }


    /// ```text
    /// /**
    ///      * Gets the id of the user context this principal is inside.  If this
    ///      * principal is inside the default userContext, this returns
    ///      * nsIScriptSecurityManager::DEFAULT_USER_CONTEXT_ID.
    ///      */
    /// ```
    ///

    /// `[infallible] readonly attribute unsigned long userContextId;`
    #[inline]
    pub unsafe fn GetUserContextId(&self) -> u32 {
        let mut result = <u32 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetUserContextId)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///      * Gets the id of the private browsing state of the context containing
    ///      * this principal. If the principal has a private browsing value of 0, it
    ///      * is not in private browsing.
    ///      */
    /// ```
    ///

    /// `[infallible] readonly attribute unsigned long privateBrowsingId;`
    #[inline]
    pub unsafe fn GetPrivateBrowsingId(&self) -> u32 {
        let mut result = <u32 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetPrivateBrowsingId)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///      * Returns true iff the principal is inside an isolated mozbrowser element.
    ///      * <xul:browser> is not considered to be a mozbrowser element.
    ///      * <iframe mozbrowser noisolation> does not count as isolated since
    ///      * isolation is disabled.  Isolation can only be disabled if the
    ///      * containing document is chrome.
    ///      */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean isInIsolatedMozBrowserElement;`
    #[inline]
    pub unsafe fn GetIsInIsolatedMozBrowserElement(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetIsInIsolatedMozBrowserElement)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///      * Returns true iff this is a null principal (corresponding to an
        ///      * unknown, hence assumed minimally privileged, security context).
    ///      */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean isNullPrincipal;`
    #[inline]
    pub unsafe fn GetIsNullPrincipal(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetIsNullPrincipal)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///      * Returns true iff this principal corresponds to a principal origin.
    ///      */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean isContentPrincipal;`
    #[inline]
    pub unsafe fn GetIsContentPrincipal(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetIsContentPrincipal)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///      * Returns true iff this is an expanded principal.
    ///      */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean isExpandedPrincipal;`
    #[inline]
    pub unsafe fn GetIsExpandedPrincipal(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetIsExpandedPrincipal)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///      * Returns true iff this is the system principal.  C++ callers should use
    ///      * IsSystemPrincipal() instead of this scriptable accessor.
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean isSystemPrincipal;`
    #[inline]
    pub unsafe fn GetIsSystemPrincipal(&self, aIsSystemPrincipal: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsSystemPrincipal)(self, aIsSystemPrincipal)
    }


    /// ```text
    /// /**
    ///      * Faster and nicer version callable from C++.  Callers must include
    ///      * BasePrincipal.h, where it's implemented.
    ///      */
    /// /**
    ///      * Returns true iff the principal is either an addon principal or
    ///      * an expanded principal, which contains at least one addon principal.
    ///      */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean isAddonOrExpandedAddonPrincipal;`
    #[inline]
    pub unsafe fn GetIsAddonOrExpandedAddonPrincipal(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetIsAddonOrExpandedAddonPrincipal)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `[infallible] readonly attribute boolean isOnion;`
    #[inline]
    pub unsafe fn GetIsOnion(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetIsOnion)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `readonly attribute boolean isScriptAllowedByPolicy;`
    #[inline]
    pub unsafe fn GetIsScriptAllowedByPolicy(&self, aIsScriptAllowedByPolicy: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsScriptAllowedByPolicy)(self, aIsScriptAllowedByPolicy)
    }



    /// `boolean isL10nAllowed (in nsIURI aDocumentURI);`
    #[inline]
    pub unsafe fn IsL10nAllowed(&self, aDocumentURI: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsL10nAllowed)(self, aDocumentURI, _retval)
    }


    /// ```text
    /// /**
    ///     * Returns a nsIPrincipal, with one less Subdomain Segment
    ///     * Returns `nullptr` if there are no more segments to remove.
    ///     */
    /// ```
    ///

    /// `[infallible] readonly attribute nsIPrincipal nextSubDomainPrincipal;`
    #[inline]
    pub unsafe fn GetNextSubDomainPrincipal(&self, aNextSubDomainPrincipal: *mut *const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).GetNextSubDomainPrincipal)(self, aNextSubDomainPrincipal)
    }


    /// ```text
    /// /**
    ///      * Returns if the principal is for an IP address.
    ///      */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean isIpAddress;`
    #[inline]
    pub unsafe fn GetIsIpAddress(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetIsIpAddress)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///      * Returns if the principal is for a local IP address.
    ///      */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean isLocalIpAddress;`
    #[inline]
    pub unsafe fn GetIsLocalIpAddress(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetIsLocalIpAddress)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


}


/// `interface nsIExpandedPrincipal : nsISupports`
///

/// ```text
/// /**
///  * If SystemPrincipal is too risky to use, but we want a principal to access
///  * more than one origin, ExpandedPrincipals letting us define an array of
///  * principals it subsumes. So script with an ExpandedPrincipals will gain
///  * same origin access when at least one of its principals it contains gained
///  * sameorigin acccess. An ExpandedPrincipal will be subsumed by the system
///  * principal, and by another ExpandedPrincipal that has all its principals.
///  * It is added for jetpack content-scripts to let them interact with the
///  * content and a well defined set of other domains, without the risk of
///  * leaking out a system principal to the content. See: Bug 734891
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIExpandedPrincipal {
    vtable: *const nsIExpandedPrincipalVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIExpandedPrincipal.
unsafe impl XpCom for nsIExpandedPrincipal {
    const IID: nsIID = nsID(0xf3e177df, 0x6a5e, 0x489f,
        [0x80, 0xa7, 0x2d, 0xd1, 0x48, 0x14, 0x71, 0xd8]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIExpandedPrincipal {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIExpandedPrincipal.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIExpandedPrincipalCoerce {
    /// Cheaply cast a value of this type from a `nsIExpandedPrincipal`.
    fn coerce_from(v: &nsIExpandedPrincipal) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIExpandedPrincipalCoerce for nsIExpandedPrincipal {
    #[inline]
    fn coerce_from(v: &nsIExpandedPrincipal) -> &Self {
        v
    }
}

impl nsIExpandedPrincipal {
    /// Cast this `nsIExpandedPrincipal` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIExpandedPrincipalCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIExpandedPrincipal {
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
impl<T: nsISupportsCoerce> nsIExpandedPrincipalCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIExpandedPrincipal) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIExpandedPrincipal
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIExpandedPrincipalVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [noscript,nostdcall,notxpcom] PrincipalArray AllowList (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub AllowList: *const ::libc::c_void,

    /* readonly attribute nsIContentSecurityPolicy csp; */
    pub GetCsp: unsafe extern "system" fn (this: *const nsIExpandedPrincipal, aCsp: *mut *const nsIContentSecurityPolicy) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIExpandedPrincipal {

    /// ```text
    /// /**
    ///    * An array of principals that the expanded principal subsumes.
    ///    *
    ///    * When an expanded principal is used as a triggering principal for a
    ///    * request that inherits a security context, one of its constitutent
    ///    * principals is inherited rather than the expanded principal itself. The
    ///    * last principal in the allowlist is the default principal to inherit.
    ///    *
    ///    * Note: this list is not reference counted, it is shared, so
    ///    * should not be changed and should only be used ephemerally.
    ///    */
    /// ```
    ///

    /// `[noscript,nostdcall,notxpcom] PrincipalArray AllowList ();`
    const _AllowList: () = ();

    /// ```text
    /// /**
    ///    * Bug 1548468: Move CSP off ExpandedPrincipal.
    ///    *
    ///    * A Content Security Policy associated with this principal. Use this function
    ///    * to query the associated CSP with this principal.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIContentSecurityPolicy csp;`
    #[inline]
    pub unsafe fn GetCsp(&self, aCsp: *mut *const nsIContentSecurityPolicy) -> ::nserror::nsresult {
        ((*self.vtable).GetCsp)(self, aCsp)
    }


}


