//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/dns/nsIDNSService.idl
//


/// `interface nsIDNSService : nsISupports`
///

/// ```text
/// /**
///  * nsIDNSService
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDNSService {
    vtable: *const nsIDNSServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDNSService.
unsafe impl XpCom for nsIDNSService {
    const IID: nsIID = nsID(0xde5642c6, 0x61fc, 0x4fcf,
        [0x9a, 0x47, 0x03, 0x22, 0x6b, 0x0d, 0x4e, 0x21]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDNSService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDNSService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDNSServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIDNSService`.
    fn coerce_from(v: &nsIDNSService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDNSServiceCoerce for nsIDNSService {
    #[inline]
    fn coerce_from(v: &nsIDNSService) -> &Self {
        v
    }
}

impl nsIDNSService {
    /// Cast this `nsIDNSService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDNSServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDNSService {
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
impl<T: nsISupportsCoerce> nsIDNSServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDNSService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDNSService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDNSServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext,optional_argc] nsICancelable asyncResolve (in AUTF8String aHostName, in nsIDNSService_ResolveType aType, in unsigned long aFlags, in nsIDNSResolverInfo aResolver, in nsIDNSListener aListener, in nsIEventTarget aListenerTarget, [optional] in jsval aOriginAttributes); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub AsyncResolve: *const ::libc::c_void,

    /* [notxpcom] nsresult asyncResolveNative (in AUTF8String aHostName, in nsIDNSService_ResolveType aType, in unsigned long aFlags, in nsIDNSResolverInfo aResolver, in nsIDNSListener aListener, in nsIEventTarget aListenerTarget, in OriginAttributes aOriginAttributes, out nsICancelable aResult); */
    /// Unable to generate binding because `native type const mozilla::OriginAttributes unsupported`
    pub AsyncResolveNative: *const ::libc::c_void,

    /* nsIDNSResolverInfo newTRRResolverInfo (in AUTF8String aTrrURL); */
    pub NewTRRResolverInfo: unsafe extern "system" fn (this: *const nsIDNSService, aTrrURL: *const ::nsstring::nsACString, _retval: *mut*const nsIDNSResolverInfo) -> ::nserror::nsresult,

    /* [implicit_jscontext,optional_argc] void cancelAsyncResolve (in AUTF8String aHostName, in nsIDNSService_ResolveType aType, in unsigned long aFlags, in nsIDNSResolverInfo aResolver, in nsIDNSListener aListener, in nsresult aReason, [optional] in jsval aOriginAttributes); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub CancelAsyncResolve: *const ::libc::c_void,

    /* [notxpcom] nsresult cancelAsyncResolveNative (in AUTF8String aHostName, in nsIDNSService_ResolveType aType, in unsigned long aFlags, in nsIDNSResolverInfo aResolver, in nsIDNSListener aListener, in nsresult aReason, in OriginAttributes aOriginAttributes); */
    /// Unable to generate binding because `native type const mozilla::OriginAttributes unsupported`
    pub CancelAsyncResolveNative: *const ::libc::c_void,

    /* [implicit_jscontext,optional_argc] nsIDNSRecord resolve (in AUTF8String aHostName, in unsigned long aFlags, [optional] in jsval aOriginAttributes); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub Resolve: *const ::libc::c_void,

    /* [notxpcom] nsresult resolveNative (in AUTF8String aHostName, in unsigned long aFlags, in OriginAttributes aOriginAttributes, out nsIDNSRecord aResult); */
    /// Unable to generate binding because `native type const mozilla::OriginAttributes unsupported`
    pub ResolveNative: *const ::libc::c_void,

    /* [noscript] void getDNSCacheEntries (in EntriesArray args); */
    /// Unable to generate binding because `native type nsTArray<mozilla::net::DNSCacheEntries> unsupported`
    pub GetDNSCacheEntries: *const ::libc::c_void,

    /* void clearCache (in boolean aTrrToo); */
    pub ClearCache: unsafe extern "system" fn (this: *const nsIDNSService, aTrrToo: bool) -> ::nserror::nsresult,

    /* void reloadParentalControlEnabled (); */
    pub ReloadParentalControlEnabled: unsafe extern "system" fn (this: *const nsIDNSService) -> ::nserror::nsresult,

    /* void setDetectedTrrURI (in AUTF8String aURI); */
    pub SetDetectedTrrURI: unsafe extern "system" fn (this: *const nsIDNSService, aURI: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [noscript] void ReportFailedSVCDomainName (in ACString aOwnerName, in ACString aSVCDomainName); */
    pub ReportFailedSVCDomainName: unsafe extern "system" fn (this: *const nsIDNSService, aOwnerName: *const ::nsstring::nsACString, aSVCDomainName: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [noscript] boolean IsSVCDomainNameFailed (in ACString aOwnerName, in ACString aSVCDomainName); */
    pub IsSVCDomainNameFailed: unsafe extern "system" fn (this: *const nsIDNSService, aOwnerName: *const ::nsstring::nsACString, aSVCDomainName: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,

    /* [noscript] void ResetExcludedSVCDomainName (in ACString aOwnerName); */
    pub ResetExcludedSVCDomainName: unsafe extern "system" fn (this: *const nsIDNSService, aOwnerName: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String currentTrrURI; */
    pub GetCurrentTrrURI: unsafe extern "system" fn (this: *const nsIDNSService, aCurrentTrrURI: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute nsIDNSService_ResolverMode currentTrrMode; */
    pub GetCurrentTrrMode: unsafe extern "system" fn (this: *const nsIDNSService, aCurrentTrrMode: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String myHostName; */
    pub GetMyHostName: unsafe extern "system" fn (this: *const nsIDNSService, aMyHostName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute boolean ODoHActivated; */
    pub GetODoHActivated: unsafe extern "system" fn (this: *const nsIDNSService, aODoHActivated: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDNSService {
    /// ```text
    /// /*************************************************************************
    ///      * Listed below are the various flags that may be OR'd together to form
    ///      * the aFlags parameter passed to asyncResolve() and resolve().
    ///      */
    /// /**
    ///      * if set, this flag suppresses the internal DNS lookup cache.
    ///      */
    /// ```
    ///

    pub const RESOLVE_BYPASS_CACHE: i64 = 1;

    /// ```text
    /// /**
    ///      * if set, the canonical name of the specified host will be queried.
    ///      */
    /// ```
    ///

    pub const RESOLVE_CANONICAL_NAME: i64 = 2;

    /// ```text
    /// /**
    ///      * if set, the query is given lower priority. Medium takes precedence
    ///      * if both are used.
    ///      */
    /// ```
    ///

    pub const RESOLVE_PRIORITY_MEDIUM: i64 = 4;


    pub const RESOLVE_PRIORITY_LOW: i64 = 8;

    /// ```text
    /// /**
    ///      * if set, indicates request is speculative. Speculative requests
    ///      * return errors if prefetching is disabled by configuration.
    ///      */
    /// ```
    ///

    pub const RESOLVE_SPECULATE: i64 = 16;

    /// ```text
    /// /**
    ///      * If set, only IPv4 addresses will be returned from resolve/asyncResolve.
    ///      */
    /// ```
    ///

    pub const RESOLVE_DISABLE_IPV6: i64 = 32;

    /// ```text
    /// /**
    ///      * If set, only literals and cached entries will be returned from resolve/
    ///      * asyncResolve.
    ///      */
    /// ```
    ///

    pub const RESOLVE_OFFLINE: i64 = 64;

    /// ```text
    /// /**
    ///      * If set, only IPv6 addresses will be returned from resolve/asyncResolve.
    ///      */
    /// ```
    ///

    pub const RESOLVE_DISABLE_IPV4: i64 = 128;

    /// ```text
    /// /**
    ///      * If set, allow name collision results (127.0.53.53) which are normally filtered.
    ///      */
    /// ```
    ///

    pub const RESOLVE_ALLOW_NAME_COLLISION: i64 = 256;

    /// ```text
    /// /**
    ///      * If set, do not use TRR for resolving the host name.
    ///      */
    /// ```
    ///

    pub const RESOLVE_DISABLE_TRR: i64 = 512;

    /// ```text
    /// /**
    ///      * if set (together with RESOLVE_BYPASS_CACHE), invalidate the DNS
    ///      * existing cache entry first (if existing) then make a new resolve.
    ///      */
    /// ```
    ///

    pub const RESOLVE_REFRESH_CACHE: i64 = 1024;

    /// ```text
    /// /**
    ///      * These two bits encode the TRR mode of the request.
    ///      * Use the static helper methods convert between the TRR mode and flags.
    ///      */
    /// ```
    ///

    pub const RESOLVE_TRR_MODE_MASK: i64 = 6144;


    pub const RESOLVE_TRR_DISABLED_MODE: i64 = 2048;

    /// ```text
    /// /**
    ///      * Force resolution even when SOCKS proxy with DNS forwarding is configured.
    ///      * Only to be used for the proxy host resolution.
    ///      */
    /// ```
    ///

    pub const RESOLVE_IGNORE_SOCKS_DNS: i64 = 8192;

    /// ```text
    /// /**
    ///      * If set, only cached IP hint addresses will be returned from
    ///      * resolve/asyncResolve.
    ///      */
    /// ```
    ///

    pub const RESOLVE_IP_HINT: i64 = 16384;

    /// ```text
    /// /**
    ///      * If set, do not use ODoH for resolving the host name.
    ///      */
    /// ```
    ///

    pub const RESOLVE_DISABLE_ODOH: i64 = 32768;

    /// ```text
    /// /**
    ///      * kicks off an asynchronous host lookup.
    ///      *
    ///      * @param aHostName
    ///      *        the hostname or IP-address-literal to resolve.
    ///      * @param aType
    ///      *        one of RESOLVE_TYPE_*.
    ///      * @param aFlags
    ///      *        a bitwise OR of the RESOLVE_ prefixed constants defined below.
    ///      * @param aResolver
    ///      *        a resolverInfo object that holds information about the resolver
    ///      *        to be used such as TRR URL. If null we use the default configuration.
    ///      * @param aListener
    ///      *        the listener to be notified when the result is available.
    ///      * @param aListenerTarget
    ///      *        optional parameter (may be null).  if non-null, this parameter
    ///      *        specifies the nsIEventTarget of the thread on which the
    ///      *        listener's onLookupComplete should be called.  however, if this
    ///      *        parameter is null, then onLookupComplete will be called on an
    ///      *        unspecified thread (possibly recursively).
    ///      * @param aOriginAttributes
    ///      *        the originAttribute for this resolving, the DNS cache will be
    ///      *        separated according to this originAttributes. This attribute is
    ///      *        optional to avoid breaking add-ons.
    ///      *
    ///      * @return An object that can be used to cancel the host lookup.
    ///      */
    /// ```
    ///

    /// `[implicit_jscontext,optional_argc] nsICancelable asyncResolve (in AUTF8String aHostName, in nsIDNSService_ResolveType aType, in unsigned long aFlags, in nsIDNSResolverInfo aResolver, in nsIDNSListener aListener, in nsIEventTarget aListenerTarget, [optional] in jsval aOriginAttributes);`
    const _AsyncResolve: () = ();


    /// `[notxpcom] nsresult asyncResolveNative (in AUTF8String aHostName, in nsIDNSService_ResolveType aType, in unsigned long aFlags, in nsIDNSResolverInfo aResolver, in nsIDNSListener aListener, in nsIEventTarget aListenerTarget, in OriginAttributes aOriginAttributes, out nsICancelable aResult);`
    const _AsyncResolveNative: () = ();

    /// ```text
    /// /**
    ///      * Returns a new resolverInfo object containing the URL we pass to it.
    ///      */
    /// ```
    ///

    /// `nsIDNSResolverInfo newTRRResolverInfo (in AUTF8String aTrrURL);`
    #[inline]
    pub unsafe fn NewTRRResolverInfo(&self, aTrrURL: *const ::nsstring::nsACString, _retval: *mut*const nsIDNSResolverInfo) -> ::nserror::nsresult {
        ((*self.vtable).NewTRRResolverInfo)(self, aTrrURL, _retval)
    }


    /// ```text
    /// /**
    ///      * Attempts to cancel a previously requested async DNS lookup
    ///      *
    ///      * @param aHostName
    ///      *        the hostname or IP-address-literal to resolve.
    ///      * @param aType
    ///      *        one of RESOLVE_TYPE_*.
    ///      * @param aFlags
    ///      *        a bitwise OR of the RESOLVE_ prefixed constants defined below.
    ///      * @param aResolver
    ///      *        a resolverInfo object that holds information about the resolver
    ///      *        to be used such as TRR URL. If null we use the default configuration.
    ///      * @param aListener
    ///      *        the original listener which was to be notified about the host lookup
    ///      *        result - used to match request information to requestor.
    ///      * @param aReason
    ///      *        nsresult reason for the cancellation
    ///      * @param aOriginAttributes
    ///      *        the originAttribute for this resolving. This attribute is optional
    ///      *        to avoid breaking add-ons.
    ///      */
    /// ```
    ///

    /// `[implicit_jscontext,optional_argc] void cancelAsyncResolve (in AUTF8String aHostName, in nsIDNSService_ResolveType aType, in unsigned long aFlags, in nsIDNSResolverInfo aResolver, in nsIDNSListener aListener, in nsresult aReason, [optional] in jsval aOriginAttributes);`
    const _CancelAsyncResolve: () = ();


    /// `[notxpcom] nsresult cancelAsyncResolveNative (in AUTF8String aHostName, in nsIDNSService_ResolveType aType, in unsigned long aFlags, in nsIDNSResolverInfo aResolver, in nsIDNSListener aListener, in nsresult aReason, in OriginAttributes aOriginAttributes);`
    const _CancelAsyncResolveNative: () = ();

    /// ```text
    /// /**
    ///      * called to synchronously resolve a hostname.
    ///      *
    ///      * Since this method may block the calling thread for a long period of
    ///      * time, it may not be accessed from the main thread.
    ///      *
    ///      * @param aHostName
    ///      *        the hostname or IP-address-literal to resolve.
    ///      * @param aFlags
    ///      *        a bitwise OR of the RESOLVE_ prefixed constants defined below.
    ///      * @param aOriginAttributes
    ///      *        the originAttribute for this resolving, the DNS cache will be
    ///      *        separated according to this originAttributes. This attribute is
    ///      *        optional to avoid breaking add-ons.
    ///      *
    ///      * @return DNS record corresponding to the given hostname.
    ///      * @throws NS_ERROR_UNKNOWN_HOST if host could not be resolved.
    ///      * @throws NS_ERROR_NOT_AVAILABLE if accessed from the main thread.
    ///      */
    /// ```
    ///

    /// `[implicit_jscontext,optional_argc] nsIDNSRecord resolve (in AUTF8String aHostName, in unsigned long aFlags, [optional] in jsval aOriginAttributes);`
    const _Resolve: () = ();


    /// `[notxpcom] nsresult resolveNative (in AUTF8String aHostName, in unsigned long aFlags, in OriginAttributes aOriginAttributes, out nsIDNSRecord aResult);`
    const _ResolveNative: () = ();

    /// ```text
    /// /**
    ///      * The method takes a pointer to an nsTArray
    ///      * and fills it with cache entry data
    ///      * Called by the networking dashboard
    ///      */
    /// ```
    ///

    /// `[noscript] void getDNSCacheEntries (in EntriesArray args);`
    const _GetDNSCacheEntries: () = ();

    /// ```text
    /// /**
    ///      * Clears the DNS cache.
    ///      * @param aTrrToo
    ///      *        If true we will clear TRR cached entries too. Since these
    ///      *        are resolved remotely it's not necessary to clear them when
    ///      *        the network status changes, but it's sometimes useful to do so
    ///      *        for tests or other situations.
    ///      */
    /// ```
    ///

    /// `void clearCache (in boolean aTrrToo);`
    #[inline]
    pub unsafe fn ClearCache(&self, aTrrToo: bool) -> ::nserror::nsresult {
        ((*self.vtable).ClearCache)(self, aTrrToo)
    }


    /// ```text
    /// /**
    ///      * The method is used only for test purpose. We use this to recheck if
    ///      * parental control is enabled or not.
    ///      */
    /// ```
    ///

    /// `void reloadParentalControlEnabled ();`
    #[inline]
    pub unsafe fn ReloadParentalControlEnabled(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ReloadParentalControlEnabled)(self, )
    }


    /// ```text
    /// /**
    ///      * Notifies the TRR service of a TRR that was automatically detected based
    ///      * on network preferences.
    ///      */
    /// ```
    ///

    /// `void setDetectedTrrURI (in AUTF8String aURI);`
    #[inline]
    pub unsafe fn SetDetectedTrrURI(&self, aURI: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetDetectedTrrURI)(self, aURI)
    }


    /// ```text
    /// /**
    ///      * Notifies the DNS service that we failed to connect to this alternative
    ///      * endpoint.
    ///      * @param aOwnerName
    ///      *        The owner name of this HTTPS RRs.
    ///      * @param aSVCDomainName
    ///      *        The domain name of this alternative endpoint.
    ///      */
    /// ```
    ///

    /// `[noscript] void ReportFailedSVCDomainName (in ACString aOwnerName, in ACString aSVCDomainName);`
    #[inline]
    pub unsafe fn ReportFailedSVCDomainName(&self, aOwnerName: *const ::nsstring::nsACString, aSVCDomainName: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).ReportFailedSVCDomainName)(self, aOwnerName, aSVCDomainName)
    }


    /// ```text
    /// /**
    ///      * Check if the given domain name was failed to connect to before.
    ///      * @param aOwnerName
    ///      *        The owner name of this HTTPS RRs.
    ///      * @param aSVCDomainName
    ///      *        The domain name of this alternative endpoint.
    ///      */
    /// ```
    ///

    /// `[noscript] boolean IsSVCDomainNameFailed (in ACString aOwnerName, in ACString aSVCDomainName);`
    #[inline]
    pub unsafe fn IsSVCDomainNameFailed(&self, aOwnerName: *const ::nsstring::nsACString, aSVCDomainName: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsSVCDomainNameFailed)(self, aOwnerName, aSVCDomainName, _retval)
    }


    /// ```text
    /// /**
    ///      * Reset the exclusion list.
    ///      * @param aOwnerName
    ///      *        The owner name of this HTTPS RRs.
    ///      */
    /// ```
    ///

    /// `[noscript] void ResetExcludedSVCDomainName (in ACString aOwnerName);`
    #[inline]
    pub unsafe fn ResetExcludedSVCDomainName(&self, aOwnerName: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).ResetExcludedSVCDomainName)(self, aOwnerName)
    }


    /// ```text
    /// /**
    ///      * Returns a string containing the URI currently used by the TRR service.
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String currentTrrURI;`
    #[inline]
    pub unsafe fn GetCurrentTrrURI(&self, aCurrentTrrURI: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetCurrentTrrURI)(self, aCurrentTrrURI)
    }


    /// ```text
    /// /**
    ///      * Returns the value of the TRR Service's current default mode.
    ///      */
    /// ```
    ///

    /// `readonly attribute nsIDNSService_ResolverMode currentTrrMode;`
    #[inline]
    pub unsafe fn GetCurrentTrrMode(&self, aCurrentTrrMode: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetCurrentTrrMode)(self, aCurrentTrrMode)
    }


    /// ```text
    /// /**
    ///      * @return the hostname of the operating system.
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String myHostName;`
    #[inline]
    pub unsafe fn GetMyHostName(&self, aMyHostName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetMyHostName)(self, aMyHostName)
    }


    /// ```text
    /// /**
    ///      * Returns true when we have valid ODoHConfigs to encrypt/decrypt oblivious
    ///      * DNS packets.
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean ODoHActivated;`
    #[inline]
    pub unsafe fn GetODoHActivated(&self, aODoHActivated: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetODoHActivated)(self, aODoHActivated)
    }


}


