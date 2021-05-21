//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIProtocolProxyService.idl
//


/// `interface nsIProtocolProxyService : nsISupports`
///

/// ```text
/// /**
///  * nsIProtocolProxyService provides methods to access information about
///  * various network proxies.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIProtocolProxyService {
    vtable: *const nsIProtocolProxyServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIProtocolProxyService.
unsafe impl XpCom for nsIProtocolProxyService {
    const IID: nsIID = nsID(0xef57c8b6, 0xe09d, 0x4cd4,
        [0x92, 0x22, 0x2a, 0x5d, 0x24, 0x02, 0xe1, 0x5d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIProtocolProxyService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIProtocolProxyService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIProtocolProxyServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIProtocolProxyService`.
    fn coerce_from(v: &nsIProtocolProxyService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIProtocolProxyServiceCoerce for nsIProtocolProxyService {
    #[inline]
    fn coerce_from(v: &nsIProtocolProxyService) -> &Self {
        v
    }
}

impl nsIProtocolProxyService {
    /// Cast this `nsIProtocolProxyService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIProtocolProxyServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIProtocolProxyService {
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
impl<T: nsISupportsCoerce> nsIProtocolProxyServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProtocolProxyService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIProtocolProxyService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIProtocolProxyServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsICancelable asyncResolve (in nsISupports aChannelOrURI, in unsigned long aFlags, in nsIProtocolProxyCallback aCallback, [optional] in nsISerialEventTarget aMainThreadTarget); */
    pub AsyncResolve: unsafe extern "system" fn (this: *const nsIProtocolProxyService, aChannelOrURI: *const nsISupports, aFlags: u32, aCallback: *const nsIProtocolProxyCallback, aMainThreadTarget: *const nsISerialEventTarget, _retval: *mut*const nsICancelable) -> ::nserror::nsresult,

    /* nsIProxyInfo newProxyInfo (in ACString aType, in AUTF8String aHost, in long aPort, in ACString aProxyAuthorizationHeader, in ACString aConnectionIsolationKey, in unsigned long aFlags, in unsigned long aFailoverTimeout, in nsIProxyInfo aFailoverProxy); */
    pub NewProxyInfo: unsafe extern "system" fn (this: *const nsIProtocolProxyService, aType: *const ::nsstring::nsACString, aHost: *const ::nsstring::nsACString, aPort: i32, aProxyAuthorizationHeader: *const ::nsstring::nsACString, aConnectionIsolationKey: *const ::nsstring::nsACString, aFlags: u32, aFailoverTimeout: u32, aFailoverProxy: *const nsIProxyInfo, _retval: *mut*const nsIProxyInfo) -> ::nserror::nsresult,

    /* nsIProxyInfo newProxyInfoWithAuth (in ACString aType, in AUTF8String aHost, in long aPort, in ACString aUsername, in ACString aPassword, in ACString aProxyAuthorizationHeader, in ACString aConnectionIsolationKey, in unsigned long aFlags, in unsigned long aFailoverTimeout, in nsIProxyInfo aFailoverProxy); */
    pub NewProxyInfoWithAuth: unsafe extern "system" fn (this: *const nsIProtocolProxyService, aType: *const ::nsstring::nsACString, aHost: *const ::nsstring::nsACString, aPort: i32, aUsername: *const ::nsstring::nsACString, aPassword: *const ::nsstring::nsACString, aProxyAuthorizationHeader: *const ::nsstring::nsACString, aConnectionIsolationKey: *const ::nsstring::nsACString, aFlags: u32, aFailoverTimeout: u32, aFailoverProxy: *const nsIProxyInfo, _retval: *mut*const nsIProxyInfo) -> ::nserror::nsresult,

    /* nsIProxyInfo getFailoverForProxy (in nsIProxyInfo aProxyInfo, in nsIURI aURI, in nsresult aReason); */
    pub GetFailoverForProxy: unsafe extern "system" fn (this: *const nsIProtocolProxyService, aProxyInfo: *const nsIProxyInfo, aURI: *const nsIURI, aReason: ::nserror::nsresult, _retval: *mut*const nsIProxyInfo) -> ::nserror::nsresult,

    /* void registerFilter (in nsIProtocolProxyFilter aFilter, in unsigned long aPosition); */
    pub RegisterFilter: unsafe extern "system" fn (this: *const nsIProtocolProxyService, aFilter: *const nsIProtocolProxyFilter, aPosition: u32) -> ::nserror::nsresult,

    /* void registerChannelFilter (in nsIProtocolProxyChannelFilter aFilter, in unsigned long aPosition); */
    pub RegisterChannelFilter: unsafe extern "system" fn (this: *const nsIProtocolProxyService, aFilter: *const nsIProtocolProxyChannelFilter, aPosition: u32) -> ::nserror::nsresult,

    /* void unregisterFilter (in nsIProtocolProxyFilter aFilter); */
    pub UnregisterFilter: unsafe extern "system" fn (this: *const nsIProtocolProxyService, aFilter: *const nsIProtocolProxyFilter) -> ::nserror::nsresult,

    /* void unregisterChannelFilter (in nsIProtocolProxyChannelFilter aFilter); */
    pub UnregisterChannelFilter: unsafe extern "system" fn (this: *const nsIProtocolProxyService, aFilter: *const nsIProtocolProxyChannelFilter) -> ::nserror::nsresult,

    /* readonly attribute unsigned long proxyConfigType; */
    pub GetProxyConfigType: unsafe extern "system" fn (this: *const nsIProtocolProxyService, aProxyConfigType: *mut u32) -> ::nserror::nsresult,

    /* [nostdcall,notxpcom] readonly attribute boolean isPACLoading; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetIsPACLoading: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIProtocolProxyService {
    /// ```text
    /// /** Flag 1 << 0 is unused **/
    /// /**
    ///      * When the proxy configuration is manual this flag may be passed to the
    ///      * resolve and asyncResolve methods to request to prefer the SOCKS proxy
    ///      * to HTTP ones.
    ///      */
    /// ```
    ///

    pub const RESOLVE_PREFER_SOCKS_PROXY: i64 = 2;

    /// ```text
    /// /**
    ///      * When the proxy configuration is manual this flag may be passed to the
    ///      * resolve and asyncResolve methods to request to not analyze the uri's
    ///      * scheme specific proxy. When this flag is set the main HTTP proxy is the
    ///      * preferred one.
    ///      *
    ///      * NOTE: if RESOLVE_PREFER_SOCKS_PROXY is set then the SOCKS proxy is
    ///      *       the preferred one.
    ///      *
    ///      * NOTE: if RESOLVE_PREFER_HTTPS_PROXY is set then the HTTPS proxy
    ///      *       is the preferred one.
    ///      */
    /// ```
    ///

    pub const RESOLVE_IGNORE_URI_SCHEME: i64 = 4;

    /// ```text
    /// /**
    ///      * When the proxy configuration is manual this flag may be passed to the
    ///      * resolve and asyncResolve methods to request to prefer the HTTPS proxy
    ///      * to the others HTTP ones.
    ///      *
    ///      * NOTE: RESOLVE_PREFER_SOCKS_PROXY takes precedence over this flag.
    ///      *
    ///      * NOTE: This flag implies RESOLVE_IGNORE_URI_SCHEME.
    ///      */
    /// ```
    ///

    pub const RESOLVE_PREFER_HTTPS_PROXY: i64 = 12;

    /// ```text
    /// /**
    ///      * When the proxy configuration is manual this flag may be passed to the
    ///      * resolve and asyncResolve methods to that all methods will be tunneled via
    ///      * CONNECT through the http proxy.
    ///      */
    /// ```
    ///

    pub const RESOLVE_ALWAYS_TUNNEL: i64 = 16;

    /// ```text
    /// /**
    ///       * These values correspond to the possible integer values for the
    ///       * network.proxy.type preference.
    ///       */
    /// ```
    ///

    pub const PROXYCONFIG_DIRECT: i64 = 0;


    pub const PROXYCONFIG_MANUAL: i64 = 1;


    pub const PROXYCONFIG_PAC: i64 = 2;


    pub const PROXYCONFIG_WPAD: i64 = 4;


    pub const PROXYCONFIG_SYSTEM: i64 = 5;

    /// ```text
    /// /**
    ///      * This method returns via callback a nsIProxyInfo instance that identifies
    ///      * a proxy to be used for the given channel.  Otherwise, this method returns
    ///      * null indicating that a direct connection should be used.
    ///      *
    ///      * @param aChannelOrURI
    ///      *        The channel for which a proxy is to be found, or, if no channel is
    ///      *        available, a URI indicating the same. This method will return
    ///      *        NS_ERROR_NOINTERFACE if this argument isn't either an nsIURI or an
    ///      *        nsIChannel.
    ///      * @param aFlags
    ///      *        A bit-wise combination of the RESOLVE_ flags defined above.  Pass
    ///      *        0 to specify the default behavior.  Any additional bits that do
    ///      *        not correspond to a RESOLVE_ flag are reserved for future use.
    ///      * @param aCallback
    ///      *        The object to be notified when the result is available.
    ///      * @param aMainThreadTarget
    ///      *        A labelled event target for dispatching runnables to main thread.
    ///      *
    ///      * @return An object that can be used to cancel the asychronous operation.
    ///      *         If canceled, the cancelation status (aReason) will be forwarded
    ///      *         to the callback's onProxyAvailable method via the aStatus param.
    ///      *
    ///      * NOTE: If this proxy is unavailable, getFailoverForProxy may be called
    ///      * to determine the correct secondary proxy to be used.
    ///      *
    ///      * NOTE: If the protocol handler for the given URI supports
    ///      * nsIProxiedProtocolHandler, then the nsIProxyInfo instance returned from
    ///      * resolve may be passed to the newProxiedChannel method to create a
    ///      * nsIChannel to the given URI that uses the specified proxy.
    ///      *
    ///      * NOTE: However, if the nsIProxyInfo type is "http", then it means that
    ///      * the given URI should be loaded using the HTTP protocol handler, which
    ///      * also supports nsIProxiedProtocolHandler.
    ///      *
    ///      * @see nsIProxiedProtocolHandler::newProxiedChannel
    ///      */
    /// ```
    ///

    /// `nsICancelable asyncResolve (in nsISupports aChannelOrURI, in unsigned long aFlags, in nsIProtocolProxyCallback aCallback, [optional] in nsISerialEventTarget aMainThreadTarget);`
    #[inline]
    pub unsafe fn AsyncResolve(&self, aChannelOrURI: *const nsISupports, aFlags: u32, aCallback: *const nsIProtocolProxyCallback, aMainThreadTarget: *const nsISerialEventTarget, _retval: *mut*const nsICancelable) -> ::nserror::nsresult {
        ((*self.vtable).AsyncResolve)(self, aChannelOrURI, aFlags, aCallback, aMainThreadTarget, _retval)
    }


    /// ```text
    /// /**
    ///      * This method may be called to construct a nsIProxyInfo instance from
    ///      * the given parameters.  This method may be useful in conjunction with
    ///      * nsISocketTransportService::createTransport for creating, for example,
    ///      * a SOCKS connection.
    ///      *
    ///      * @param aType
    ///      *        The proxy type.  This is a string value that identifies the proxy
    ///      *        type.  Standard values include:
    ///      *          "http"    - specifies a HTTP proxy
    ///      *          "https"   - specifies HTTP proxying over TLS connection to proxy
    ///      *          "socks"   - specifies a SOCKS version 5 proxy
    ///      *          "socks4"  - specifies a SOCKS version 4 proxy
    ///      *          "direct"  - specifies a direct connection (useful for failover)
    ///      *        The type name is case-insensitive.  Other string values may be
    ///      *        possible, and new types may be defined by a future version of
    ///      *        this interface.
    ///      * @param aHost
    ///      *        The proxy hostname or IP address.
    ///      * @param aPort
    ///      *        The proxy port.
    ///      * @param aFlags
    ///      *        Flags associated with this connection.  See nsIProxyInfo.idl
    ///      *        for currently defined flags.
    ///      * @param aFailoverTimeout
    ///      *        Specifies the length of time (in seconds) to ignore this proxy if
    ///      *        this proxy fails.  Pass UINT32_MAX to specify the default
    ///      *        timeout value, causing nsIProxyInfo::failoverTimeout to be
    ///      *        assigned the default value.
    ///      * @param aFailoverProxy
    ///      *        Specifies the next proxy to try if this proxy fails.  This
    ///      *        parameter may be null.
    ///      */
    /// ```
    ///

    /// `nsIProxyInfo newProxyInfo (in ACString aType, in AUTF8String aHost, in long aPort, in ACString aProxyAuthorizationHeader, in ACString aConnectionIsolationKey, in unsigned long aFlags, in unsigned long aFailoverTimeout, in nsIProxyInfo aFailoverProxy);`
    #[inline]
    pub unsafe fn NewProxyInfo(&self, aType: *const ::nsstring::nsACString, aHost: *const ::nsstring::nsACString, aPort: i32, aProxyAuthorizationHeader: *const ::nsstring::nsACString, aConnectionIsolationKey: *const ::nsstring::nsACString, aFlags: u32, aFailoverTimeout: u32, aFailoverProxy: *const nsIProxyInfo, _retval: *mut*const nsIProxyInfo) -> ::nserror::nsresult {
        ((*self.vtable).NewProxyInfo)(self, aType, aHost, aPort, aProxyAuthorizationHeader, aConnectionIsolationKey, aFlags, aFailoverTimeout, aFailoverProxy, _retval)
    }


    /// ```text
    /// /**
    ///      * This method may be called to construct a nsIProxyInfo instance for
    ///      * with the specified username and password.
    ///      * Currently implemented for SOCKS proxies only.
    ///      * @param aType
    ///      *        The proxy type.  This is a string value that identifies the proxy
    ///      *        type.  Standard values include:
    ///      *          "socks"   - specifies a SOCKS version 5 proxy
    ///      *          "socks4"  - specifies a SOCKS version 4 proxy
    ///      *        The type name is case-insensitive.  Other string values may be
    ///      *        possible, and new types may be defined by a future version of
    ///      *        this interface.
    ///      * @param aHost
    ///      *        The proxy hostname or IP address.
    ///      * @param aPort
    ///      *        The proxy port.
    ///      * @param aUsername
    ///      *        The proxy username
    ///      * @param aPassword
    ///      *        The proxy password
    ///      * @param aFlags
    ///      *        Flags associated with this connection.  See nsIProxyInfo.idl
    ///      *        for currently defined flags.
    ///      * @param aFailoverTimeout
    ///      *        Specifies the length of time (in seconds) to ignore this proxy if
    ///      *        this proxy fails.  Pass UINT32_MAX to specify the default
    ///      *        timeout value, causing nsIProxyInfo::failoverTimeout to be
    ///      *        assigned the default value.
    ///      * @param aFailoverProxy
    ///      *        Specifies the next proxy to try if this proxy fails.  This
    ///      *        parameter may be null.
    ///      */
    /// ```
    ///

    /// `nsIProxyInfo newProxyInfoWithAuth (in ACString aType, in AUTF8String aHost, in long aPort, in ACString aUsername, in ACString aPassword, in ACString aProxyAuthorizationHeader, in ACString aConnectionIsolationKey, in unsigned long aFlags, in unsigned long aFailoverTimeout, in nsIProxyInfo aFailoverProxy);`
    #[inline]
    pub unsafe fn NewProxyInfoWithAuth(&self, aType: *const ::nsstring::nsACString, aHost: *const ::nsstring::nsACString, aPort: i32, aUsername: *const ::nsstring::nsACString, aPassword: *const ::nsstring::nsACString, aProxyAuthorizationHeader: *const ::nsstring::nsACString, aConnectionIsolationKey: *const ::nsstring::nsACString, aFlags: u32, aFailoverTimeout: u32, aFailoverProxy: *const nsIProxyInfo, _retval: *mut*const nsIProxyInfo) -> ::nserror::nsresult {
        ((*self.vtable).NewProxyInfoWithAuth)(self, aType, aHost, aPort, aUsername, aPassword, aProxyAuthorizationHeader, aConnectionIsolationKey, aFlags, aFailoverTimeout, aFailoverProxy, _retval)
    }


    /// ```text
    /// /**
    ///      * If the proxy identified by aProxyInfo is unavailable for some reason,
    ///      * this method may be called to access an alternate proxy that may be used
    ///      * instead.  As a side-effect, this method may affect future result values
    ///      * from resolve/asyncResolve as well as from getFailoverForProxy.
    ///      *
    ///      * @param aProxyInfo
    ///      *        The proxy that was unavailable.
    ///      * @param aURI
    ///      *        The URI that was originally passed to resolve/asyncResolve.
    ///      * @param aReason
    ///      *        The error code corresponding to the proxy failure.  This value
    ///      *        may be used to tune the delay before this proxy is used again.
    ///      *
    ///      * @throw NS_ERROR_NOT_AVAILABLE if there is no alternate proxy available.
    ///      */
    /// ```
    ///

    /// `nsIProxyInfo getFailoverForProxy (in nsIProxyInfo aProxyInfo, in nsIURI aURI, in nsresult aReason);`
    #[inline]
    pub unsafe fn GetFailoverForProxy(&self, aProxyInfo: *const nsIProxyInfo, aURI: *const nsIURI, aReason: ::nserror::nsresult, _retval: *mut*const nsIProxyInfo) -> ::nserror::nsresult {
        ((*self.vtable).GetFailoverForProxy)(self, aProxyInfo, aURI, aReason, _retval)
    }


    /// ```text
    /// /**
    ///      * This method may be used to register a proxy filter instance.  Each proxy
    ///      * filter is registered with an associated position that determines the
    ///      * order in which the filters are applied (starting from position 0).  When
    ///      * resolve/asyncResolve is called, it generates a list of proxies for the
    ///      * given URI, and then it applies the proxy filters.  The filters have the
    ///      * opportunity to modify the list of proxies.
    ///      *
    ///      * If two filters register for the same position, then the filters will be
    ///      * visited in the order in which they were registered.
    ///      *
    ///      * If the filter is already registered, then its position will be updated.
    ///      *
    ///      * After filters have been run, any disabled or disallowed proxies will be
    ///      * removed from the list.  A proxy is disabled if it had previously failed-
    ///      * over to another proxy (see getFailoverForProxy).  A proxy is disallowed,
    ///      * for example, if it is a HTTP proxy and the nsIProtocolHandler for the
    ///      * queried URI does not permit proxying via HTTP.
    ///      *
    ///      * If a nsIProtocolHandler disallows all proxying, then filters will never
    ///      * have a chance to intercept proxy requests for such URLs.
    ///      *
    ///      * @param aFilter
    ///      *        The nsIProtocolProxyFilter instance to be registered.
    ///      * @param aPosition
    ///      *        The position of the filter.
    ///      *
    ///      * NOTE: It is possible to construct filters that compete with one another
    ///      * in undesirable ways.  This API does not attempt to protect against such
    ///      * problems.  It is recommended that any extensions that choose to call
    ///      * this method make their position value configurable at runtime (perhaps
        ///      * via the preferences service).
    ///      */
    /// ```
    ///

    /// `void registerFilter (in nsIProtocolProxyFilter aFilter, in unsigned long aPosition);`
    #[inline]
    pub unsafe fn RegisterFilter(&self, aFilter: *const nsIProtocolProxyFilter, aPosition: u32) -> ::nserror::nsresult {
        ((*self.vtable).RegisterFilter)(self, aFilter, aPosition)
    }


    /// ```text
    /// /**
    ///      * Similar to registerFilter, but accepts an nsIProtocolProxyChannelFilter,
    ///      * which selects proxies according to channel rather than URI.
    ///      *
    ///      * @param aFilter
    ///      *        The nsIProtocolProxyChannelFilter instance to be registered.
    ///      * @param aPosition
    ///      *        The position of the filter.
    ///      */
    /// ```
    ///

    /// `void registerChannelFilter (in nsIProtocolProxyChannelFilter aFilter, in unsigned long aPosition);`
    #[inline]
    pub unsafe fn RegisterChannelFilter(&self, aFilter: *const nsIProtocolProxyChannelFilter, aPosition: u32) -> ::nserror::nsresult {
        ((*self.vtable).RegisterChannelFilter)(self, aFilter, aPosition)
    }


    /// ```text
    /// /**
    ///      * This method may be used to unregister a proxy filter instance.  All
    ///      * filters will be automatically unregistered at XPCOM shutdown.
    ///      *
    ///      * @param aFilter
    ///      *        The nsIProtocolProxyFilter instance to be unregistered.
    ///      */
    /// ```
    ///

    /// `void unregisterFilter (in nsIProtocolProxyFilter aFilter);`
    #[inline]
    pub unsafe fn UnregisterFilter(&self, aFilter: *const nsIProtocolProxyFilter) -> ::nserror::nsresult {
        ((*self.vtable).UnregisterFilter)(self, aFilter)
    }


    /// ```text
    /// /**
    ///      * This method may be used to unregister a proxy channel filter instance.  All
    ///      * filters will be automatically unregistered at XPCOM shutdown.
    ///      *
    ///      * @param aFilter
    ///      *        The nsIProtocolProxyChannelFilter instance to be unregistered.
    ///      */
    /// ```
    ///

    /// `void unregisterChannelFilter (in nsIProtocolProxyChannelFilter aFilter);`
    #[inline]
    pub unsafe fn UnregisterChannelFilter(&self, aFilter: *const nsIProtocolProxyChannelFilter) -> ::nserror::nsresult {
        ((*self.vtable).UnregisterChannelFilter)(self, aFilter)
    }


    /// ```text
    /// /**
    ///       * This attribute specifies the current type of proxy configuration.
    ///       */
    /// ```
    ///

    /// `readonly attribute unsigned long proxyConfigType;`
    #[inline]
    pub unsafe fn GetProxyConfigType(&self, aProxyConfigType: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetProxyConfigType)(self, aProxyConfigType)
    }


    /// ```text
    /// /**
    ///       * True if there is a PAC download in progress.
    ///       */
    /// ```
    ///

    /// `[nostdcall,notxpcom] readonly attribute boolean isPACLoading;`
    const _GetIsPACLoading: () = ();

}


