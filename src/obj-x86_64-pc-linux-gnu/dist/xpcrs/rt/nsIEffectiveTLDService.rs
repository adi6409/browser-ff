//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/dns/nsIEffectiveTLDService.idl
//


/// `interface nsIEffectiveTLDService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIEffectiveTLDService {
    vtable: *const nsIEffectiveTLDServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIEffectiveTLDService.
unsafe impl XpCom for nsIEffectiveTLDService {
    const IID: nsIID = nsID(0x68067eb5, 0xad8d, 0x43cb,
        [0xa0, 0x43, 0x1c, 0xc8, 0x5e, 0xbe, 0x06, 0xe7]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIEffectiveTLDService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIEffectiveTLDService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIEffectiveTLDServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIEffectiveTLDService`.
    fn coerce_from(v: &nsIEffectiveTLDService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIEffectiveTLDServiceCoerce for nsIEffectiveTLDService {
    #[inline]
    fn coerce_from(v: &nsIEffectiveTLDService) -> &Self {
        v
    }
}

impl nsIEffectiveTLDService {
    /// Cast this `nsIEffectiveTLDService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIEffectiveTLDServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIEffectiveTLDService {
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
impl<T: nsISupportsCoerce> nsIEffectiveTLDServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEffectiveTLDService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIEffectiveTLDService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIEffectiveTLDServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* ACString getPublicSuffix (in nsIURI aURI); */
    pub GetPublicSuffix: unsafe extern "system" fn (this: *const nsIEffectiveTLDService, aURI: *const nsIURI, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* ACString getKnownPublicSuffix (in nsIURI aURI); */
    pub GetKnownPublicSuffix: unsafe extern "system" fn (this: *const nsIEffectiveTLDService, aURI: *const nsIURI, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* ACString getBaseDomain (in nsIURI aURI, [optional] in uint32_t aAdditionalParts); */
    pub GetBaseDomain: unsafe extern "system" fn (this: *const nsIEffectiveTLDService, aURI: *const nsIURI, aAdditionalParts: uint32_t, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* ACString getPublicSuffixFromHost (in AUTF8String aHost); */
    pub GetPublicSuffixFromHost: unsafe extern "system" fn (this: *const nsIEffectiveTLDService, aHost: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* ACString getKnownPublicSuffixFromHost (in AUTF8String aHost); */
    pub GetKnownPublicSuffixFromHost: unsafe extern "system" fn (this: *const nsIEffectiveTLDService, aHost: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* ACString getBaseDomainFromHost (in AUTF8String aHost, [optional] in uint32_t aAdditionalParts); */
    pub GetBaseDomainFromHost: unsafe extern "system" fn (this: *const nsIEffectiveTLDService, aHost: *const ::nsstring::nsACString, aAdditionalParts: uint32_t, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* ACString getNextSubDomain (in AUTF8String aHost); */
    pub GetNextSubDomain: unsafe extern "system" fn (this: *const nsIEffectiveTLDService, aHost: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* bool hasRootDomain (in AUTF8String aInput, in AUTF8String aHost); */
    pub HasRootDomain: unsafe extern "system" fn (this: *const nsIEffectiveTLDService, aInput: *const ::nsstring::nsACString, aHost: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIEffectiveTLDService {

    /// ```text
    /// /**
    ///      * Returns the public suffix of a URI. A public suffix is the highest-level domain
    ///      * under which individual domains may be registered; it may therefore contain one
    ///      * or more dots. For example, the public suffix for "www.bbc.co.uk" is "co.uk",
    ///      * because the .uk TLD does not allow the registration of domains at the
    ///      * second level ("bbc.uk" is forbidden).
    ///      *
    ///      * The public suffix will be returned encoded in ASCII/ACE and will be normalized
    ///      * according to RFC 3454, i.e. the same encoding returned by nsIURI::GetAsciiHost().
    ///      * If consumers wish to compare the result of this method against the host from
    ///      * another nsIURI, the host should be obtained using nsIURI::GetAsciiHost().
    ///      * In the case of nested URIs, the innermost URI will be used.
    ///      *
    ///      * @param   aURI   The URI to be analyzed
    ///      *
    ///      * @returns the public suffix
    ///      *
    ///      * @throws NS_ERROR_UNEXPECTED
    ///      *         or other error returned by nsIIDNService::normalize when
    ///      *         the hostname contains characters disallowed in URIs
    ///      * @throws NS_ERROR_HOST_IS_IP_ADDRESS
    ///      *         if the host is a numeric IPv4 or IPv6 address (as determined by
        ///      *         the success of a call to PR_StringToNetAddr()).
    ///      */
    /// ```
    ///

    /// `ACString getPublicSuffix (in nsIURI aURI);`
    #[inline]
    pub unsafe fn GetPublicSuffix(&self, aURI: *const nsIURI, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetPublicSuffix)(self, aURI, _retval)
    }


    /// ```text
    /// /**
    ///      * Similar to getPublicSuffix, but the suffix is validated against
    ///      * the Public Suffix List. If the suffix is unknown this will return
    ///      * an empty string.
    ///      *
    ///      * @param   aURI   The URI to be analyzed
    ///      * @returns the public suffix if known, an empty string otherwise
    ///      * @see     getPublicSuffixFromHost()
    ///      */
    /// ```
    ///

    /// `ACString getKnownPublicSuffix (in nsIURI aURI);`
    #[inline]
    pub unsafe fn GetKnownPublicSuffix(&self, aURI: *const nsIURI, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetKnownPublicSuffix)(self, aURI, _retval)
    }


    /// ```text
    /// /**
    ///      * Returns the base domain of a URI; that is, the public suffix with a given
    ///      * number of additional domain name parts. For example, the result of this method
    ///      * for "www.bbc.co.uk", depending on the value of aAdditionalParts parameter, will
    ///      * be:
    ///      *
    ///      *    0 (default) -> bbc.co.uk
    ///      *    1           -> www.bbc.co.uk
    ///      *
    ///      * Similarly, the public suffix for "www.developer.mozilla.org" is "org", and the base
    ///      * domain will be:
    ///      *
    ///      *    0 (default) -> mozilla.org
    ///      *    1           -> developer.mozilla.org
    ///      *    2           -> www.developer.mozilla.org
    ///      *
    ///      * The base domain will be returned encoded in ASCII/ACE and will be normalized
    ///      * according to RFC 3454, i.e. the same encoding returned by nsIURI::GetAsciiHost().
    ///      * If consumers wish to compare the result of this method against the host from
    ///      * another nsIURI, the host should be obtained using nsIURI::GetAsciiHost().
    ///      * In the case of nested URIs, the innermost URI will be used.
    ///      *
    ///      * @param   aURI               The URI to be analyzed
    ///      * @param   aAdditionalParts   Number of domain name parts to be
    ///      *                             returned in addition to the public suffix
    ///      *
    ///      * @returns the base domain (public suffix plus the requested number of additional parts)
    ///      *
    ///      * @throws NS_ERROR_UNEXPECTED
    ///      *         or other error returned by nsIIDNService::normalize when
    ///      *         the hostname contains characters disallowed in URIs
    ///      * @throws NS_ERROR_INSUFFICIENT_DOMAIN_LEVELS
    ///      *         when there are insufficient subdomain levels in the hostname to satisfy the
    ///      *         requested aAdditionalParts value.
    ///      * @throws NS_ERROR_HOST_IS_IP_ADDRESS
    ///      *         if aHost is a numeric IPv4 or IPv6 address (as determined by
        ///      *         the success of a call to PR_StringToNetAddr()).
    ///      *
    ///      * @see    getPublicSuffix()
    ///      */
    /// ```
    ///

    /// `ACString getBaseDomain (in nsIURI aURI, [optional] in uint32_t aAdditionalParts);`
    #[inline]
    pub unsafe fn GetBaseDomain(&self, aURI: *const nsIURI, aAdditionalParts: uint32_t, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetBaseDomain)(self, aURI, aAdditionalParts, _retval)
    }


    /// ```text
    /// /**
    ///      * NOTE: It is strongly recommended to use getPublicSuffix() above if a suitable
    ///      * nsIURI is available. Only use this method if this is not the case.
    ///      *
    ///      * Returns the public suffix of a host string. Otherwise identical to getPublicSuffix().
    ///      *
    ///      * @param   aHost   The host to be analyzed. Any additional parts (e.g. scheme,
        ///      *                  port, or path) will cause this method to throw. ASCII/ACE and
    ///      *                  UTF8 encodings are acceptable as input; normalization will
    ///      *                  be performed as specified in getBaseDomain().
    ///      *
    ///      * @see     getPublicSuffix()
    ///      */
    /// ```
    ///

    /// `ACString getPublicSuffixFromHost (in AUTF8String aHost);`
    #[inline]
    pub unsafe fn GetPublicSuffixFromHost(&self, aHost: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetPublicSuffixFromHost)(self, aHost, _retval)
    }


    /// ```text
    /// /**
    ///      * Similar to getPublicSuffixFromHost, but the suffix is validated against
    ///      * the Public Suffix List. If the suffix is unknown this will return
    ///      * an empty string.
    ///      *
    ///      * @param   aHost   The host to be analyzed.
    ///      * @returns the public suffix if known, an empty string otherwise
    ///      * @see     getPublicSuffixFromHost()
    ///      */
    /// ```
    ///

    /// `ACString getKnownPublicSuffixFromHost (in AUTF8String aHost);`
    #[inline]
    pub unsafe fn GetKnownPublicSuffixFromHost(&self, aHost: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetKnownPublicSuffixFromHost)(self, aHost, _retval)
    }


    /// ```text
    /// /**
    ///      * NOTE: It is strongly recommended to use getBaseDomain() above if a suitable
    ///      * nsIURI is available. Only use this method if this is not the case.
    ///      *
    ///      * Returns the base domain of a host string. Otherwise identical to getBaseDomain().
    ///      *
    ///      * @param   aHost   The host to be analyzed. Any additional parts (e.g. scheme,
        ///      *                  port, or path) will cause this method to throw. ASCII/ACE and
    ///      *                  UTF8 encodings are acceptable as input; normalization will
    ///      *                  be performed as specified in getBaseDomain().
    ///      *
    ///      * @see     getBaseDomain()
    ///      */
    /// ```
    ///

    /// `ACString getBaseDomainFromHost (in AUTF8String aHost, [optional] in uint32_t aAdditionalParts);`
    #[inline]
    pub unsafe fn GetBaseDomainFromHost(&self, aHost: *const ::nsstring::nsACString, aAdditionalParts: uint32_t, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetBaseDomainFromHost)(self, aHost, aAdditionalParts, _retval)
    }


    /// ```text
    /// /**
    ///      * Returns the parent sub-domain of a host string. If the host is a base
    ///      * domain, it will throw NS_ERROR_INSUFFICIENT_DOMAIN_LEVELS.
    ///      *
    ///      * For example: "player.bbc.co.uk" would return "bbc.co.uk" and
    ///      *              "bbc.co.uk" would throw NS_ERROR_INSUFFICIENT_DOMAIN_LEVELS.
    ///      *
    ///      * @param   aHost   The host to be analyzed. Any additional parts (e.g. scheme,
        ///      *                  port, or path) will cause this method to throw. ASCII/ACE and
    ///      *                  UTF8 encodings are acceptable as input; normalization will
    ///      *                  be performed as specified in getBaseDomain().
    ///      */
    /// ```
    ///

    /// `ACString getNextSubDomain (in AUTF8String aHost);`
    #[inline]
    pub unsafe fn GetNextSubDomain(&self, aHost: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetNextSubDomain)(self, aHost, _retval)
    }


    /// ```text
    /// /**
    ///      * Returns true if the |aInput| in is part of the root domain of |aHost|.
    ///      * For example, if |aInput| is "www.mozilla.org", and we pass in
    ///      * "mozilla.org" as |aHost|, this will return true.  It would return false
    ///      * the other way around.
    ///      *
    ///      * @param aInput The host to be analyzed.
    ///      * @param aHost  The host to compare to.
    ///      */
    /// ```
    ///

    /// `bool hasRootDomain (in AUTF8String aInput, in AUTF8String aHost);`
    #[inline]
    pub unsafe fn HasRootDomain(&self, aInput: *const ::nsstring::nsACString, aHost: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasRootDomain)(self, aInput, aHost, _retval)
    }


}


