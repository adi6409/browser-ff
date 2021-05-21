//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIHttpChannelInternal.idl
//


/// `interface nsIHttpUpgradeListener : nsISupports`
///

/// ```text
/// /**
///  * The callback interface for nsIHttpChannelInternal::HTTPUpgrade()
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHttpUpgradeListener {
    vtable: *const nsIHttpUpgradeListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHttpUpgradeListener.
unsafe impl XpCom for nsIHttpUpgradeListener {
    const IID: nsIID = nsID(0x5b515449, 0xab64, 0x4dba,
        [0xb3, 0xcd, 0xda, 0x8f, 0xc2, 0xf8, 0x30, 0x64]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHttpUpgradeListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHttpUpgradeListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHttpUpgradeListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIHttpUpgradeListener`.
    fn coerce_from(v: &nsIHttpUpgradeListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHttpUpgradeListenerCoerce for nsIHttpUpgradeListener {
    #[inline]
    fn coerce_from(v: &nsIHttpUpgradeListener) -> &Self {
        v
    }
}

impl nsIHttpUpgradeListener {
    /// Cast this `nsIHttpUpgradeListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHttpUpgradeListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHttpUpgradeListener {
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
impl<T: nsISupportsCoerce> nsIHttpUpgradeListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpUpgradeListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHttpUpgradeListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHttpUpgradeListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] void onTransportAvailable (in nsISocketTransport aTransport, in nsIAsyncInputStream aSocketIn, in nsIAsyncOutputStream aSocketOut); */
    pub OnTransportAvailable: unsafe extern "system" fn (this: *const nsIHttpUpgradeListener, aTransport: *const nsISocketTransport, aSocketIn: *const nsIAsyncInputStream, aSocketOut: *const nsIAsyncOutputStream) -> ::nserror::nsresult,

    /* [must_use] void onUpgradeFailed (in nsresult aErrorCode); */
    pub OnUpgradeFailed: unsafe extern "system" fn (this: *const nsIHttpUpgradeListener, aErrorCode: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHttpUpgradeListener {


    /// `[must_use] void onTransportAvailable (in nsISocketTransport aTransport, in nsIAsyncInputStream aSocketIn, in nsIAsyncOutputStream aSocketOut);`
    #[inline]
    pub unsafe fn OnTransportAvailable(&self, aTransport: *const nsISocketTransport, aSocketIn: *const nsIAsyncInputStream, aSocketOut: *const nsIAsyncOutputStream) -> ::nserror::nsresult {
        ((*self.vtable).OnTransportAvailable)(self, aTransport, aSocketIn, aSocketOut)
    }



    /// `[must_use] void onUpgradeFailed (in nsresult aErrorCode);`
    #[inline]
    pub unsafe fn OnUpgradeFailed(&self, aErrorCode: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).OnUpgradeFailed)(self, aErrorCode)
    }


}


/// `interface nsIHttpChannelInternal : nsISupports`
///

/// ```text
/// /**
///  * Dumping ground for http.  This interface will never be frozen.  If you are
///  * using any feature exposed by this interface, be aware that this interface
///  * will change and you will be broken.  You have been warned.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHttpChannelInternal {
    vtable: *const nsIHttpChannelInternalVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHttpChannelInternal.
unsafe impl XpCom for nsIHttpChannelInternal {
    const IID: nsIID = nsID(0x4e28263d, 0x1e03, 0x46f4,
        [0xaa, 0x5c, 0x95, 0x12, 0xf9, 0x19, 0x57, 0xf9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHttpChannelInternal {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHttpChannelInternal.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHttpChannelInternalCoerce {
    /// Cheaply cast a value of this type from a `nsIHttpChannelInternal`.
    fn coerce_from(v: &nsIHttpChannelInternal) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHttpChannelInternalCoerce for nsIHttpChannelInternal {
    #[inline]
    fn coerce_from(v: &nsIHttpChannelInternal) -> &Self {
        v
    }
}

impl nsIHttpChannelInternal {
    /// Cast this `nsIHttpChannelInternal` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHttpChannelInternalCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHttpChannelInternal {
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
impl<T: nsISupportsCoerce> nsIHttpChannelInternalCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpChannelInternal) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHttpChannelInternal
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHttpChannelInternalVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] attribute nsIURI documentURI; */
    pub GetDocumentURI: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aDocumentURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* [must_use] attribute nsIURI documentURI; */
    pub SetDocumentURI: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aDocumentURI: *const nsIURI) -> ::nserror::nsresult,

    /* [must_use] void getRequestVersion (out unsigned long major, out unsigned long minor); */
    pub GetRequestVersion: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, major: *mut u32, minor: *mut u32) -> ::nserror::nsresult,

    /* [must_use] void getResponseVersion (out unsigned long major, out unsigned long minor); */
    pub GetResponseVersion: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, major: *mut u32, minor: *mut u32) -> ::nserror::nsresult,

    /* [must_use,noscript] void takeAllSecurityMessages (in securityMessagesArray aMessages); */
    /// Unable to generate binding because `native type nsCOMArray<nsISecurityConsoleMessage> unsupported`
    pub TakeAllSecurityMessages: *const ::libc::c_void,

    /* [must_use] void setCookie (in ACString aCookieHeader); */
    pub SetCookie: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aCookieHeader: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] void setupFallbackChannel (in string aFallbackKey); */
    pub SetupFallbackChannel: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aFallbackKey: *const libc::c_char) -> ::nserror::nsresult,

    /* [must_use,noscript] readonly attribute bool isAuthChannel; */
    pub GetIsAuthChannel: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aIsAuthChannel: *mut bool) -> ::nserror::nsresult,

    /* [must_use] attribute unsigned long thirdPartyFlags; */
    pub GetThirdPartyFlags: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aThirdPartyFlags: *mut u32) -> ::nserror::nsresult,

    /* [must_use] attribute unsigned long thirdPartyFlags; */
    pub SetThirdPartyFlags: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aThirdPartyFlags: u32) -> ::nserror::nsresult,

    /* [must_use] attribute boolean forceAllowThirdPartyCookie; */
    pub GetForceAllowThirdPartyCookie: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aForceAllowThirdPartyCookie: *mut bool) -> ::nserror::nsresult,

    /* [must_use] attribute boolean forceAllowThirdPartyCookie; */
    pub SetForceAllowThirdPartyCookie: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aForceAllowThirdPartyCookie: bool) -> ::nserror::nsresult,

    /* [must_use] attribute boolean channelIsForDownload; */
    pub GetChannelIsForDownload: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aChannelIsForDownload: *mut bool) -> ::nserror::nsresult,

    /* [must_use] attribute boolean channelIsForDownload; */
    pub SetChannelIsForDownload: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aChannelIsForDownload: bool) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AUTF8String localAddress; */
    pub GetLocalAddress: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aLocalAddress: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute int32_t localPort; */
    pub GetLocalPort: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aLocalPort: *mut int32_t) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AUTF8String remoteAddress; */
    pub GetRemoteAddress: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aRemoteAddress: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute int32_t remotePort; */
    pub GetRemotePort: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aRemotePort: *mut int32_t) -> ::nserror::nsresult,

    /* [must_use,noscript] void setCacheKeysRedirectChain (in StringArray cacheKeys); */
    /// Unable to generate binding because `native type nsTArray<nsCString> unsupported`
    pub SetCacheKeysRedirectChain: *const ::libc::c_void,

    /* [must_use] void HTTPUpgrade (in ACString aProtocolName, in nsIHttpUpgradeListener aListener); */
    pub HTTPUpgrade: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aProtocolName: *const ::nsstring::nsACString, aListener: *const nsIHttpUpgradeListener) -> ::nserror::nsresult,

    /* [must_use] void setConnectOnly (); */
    pub SetConnectOnly: unsafe extern "system" fn (this: *const nsIHttpChannelInternal) -> ::nserror::nsresult,

    /* [must_use] readonly attribute boolean onlyConnect; */
    pub GetOnlyConnect: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aOnlyConnect: *mut bool) -> ::nserror::nsresult,

    /* [must_use] attribute boolean allowSpdy; */
    pub GetAllowSpdy: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aAllowSpdy: *mut bool) -> ::nserror::nsresult,

    /* [must_use] attribute boolean allowSpdy; */
    pub SetAllowSpdy: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aAllowSpdy: bool) -> ::nserror::nsresult,

    /* [must_use] attribute boolean allowHttp3; */
    pub GetAllowHttp3: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aAllowHttp3: *mut bool) -> ::nserror::nsresult,

    /* [must_use] attribute boolean allowHttp3; */
    pub SetAllowHttp3: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aAllowHttp3: bool) -> ::nserror::nsresult,

    /* [must_use] attribute boolean responseTimeoutEnabled; */
    pub GetResponseTimeoutEnabled: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aResponseTimeoutEnabled: *mut bool) -> ::nserror::nsresult,

    /* [must_use] attribute boolean responseTimeoutEnabled; */
    pub SetResponseTimeoutEnabled: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aResponseTimeoutEnabled: bool) -> ::nserror::nsresult,

    /* [must_use] attribute unsigned long initialRwin; */
    pub GetInitialRwin: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aInitialRwin: *mut u32) -> ::nserror::nsresult,

    /* [must_use] attribute unsigned long initialRwin; */
    pub SetInitialRwin: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aInitialRwin: u32) -> ::nserror::nsresult,

    /* [must_use] readonly attribute nsIURI apiRedirectToURI; */
    pub GetApiRedirectToURI: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aApiRedirectToURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* [must_use] attribute boolean allowAltSvc; */
    pub GetAllowAltSvc: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aAllowAltSvc: *mut bool) -> ::nserror::nsresult,

    /* [must_use] attribute boolean allowAltSvc; */
    pub SetAllowAltSvc: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aAllowAltSvc: bool) -> ::nserror::nsresult,

    /* [must_use] attribute boolean beConservative; */
    pub GetBeConservative: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aBeConservative: *mut bool) -> ::nserror::nsresult,

    /* [must_use] attribute boolean beConservative; */
    pub SetBeConservative: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aBeConservative: bool) -> ::nserror::nsresult,

    /* [must_use,noscript] attribute boolean isTRRServiceChannel; */
    pub GetIsTRRServiceChannel: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aIsTRRServiceChannel: *mut bool) -> ::nserror::nsresult,

    /* [must_use,noscript] attribute boolean isTRRServiceChannel; */
    pub SetIsTRRServiceChannel: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aIsTRRServiceChannel: bool) -> ::nserror::nsresult,

    /* [must_use,noscript] readonly attribute boolean isResolvedByTRR; */
    pub GetIsResolvedByTRR: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aIsResolvedByTRR: *mut bool) -> ::nserror::nsresult,

    /* [must_use] attribute unsigned long tlsFlags; */
    pub GetTlsFlags: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aTlsFlags: *mut u32) -> ::nserror::nsresult,

    /* [must_use] attribute unsigned long tlsFlags; */
    pub SetTlsFlags: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aTlsFlags: u32) -> ::nserror::nsresult,

    /* [must_use] readonly attribute PRTime lastModifiedTime; */
    pub GetLastModifiedTime: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aLastModifiedTime: *mut PRTime) -> ::nserror::nsresult,

    /* [must_use] attribute boolean corsIncludeCredentials; */
    pub GetCorsIncludeCredentials: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aCorsIncludeCredentials: *mut bool) -> ::nserror::nsresult,

    /* [must_use] attribute boolean corsIncludeCredentials; */
    pub SetCorsIncludeCredentials: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aCorsIncludeCredentials: bool) -> ::nserror::nsresult,

    /* [must_use] attribute unsigned long corsMode; */
    pub GetCorsMode: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aCorsMode: *mut u32) -> ::nserror::nsresult,

    /* [must_use] attribute unsigned long corsMode; */
    pub SetCorsMode: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aCorsMode: u32) -> ::nserror::nsresult,

    /* [must_use] attribute unsigned long redirectMode; */
    pub GetRedirectMode: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aRedirectMode: *mut u32) -> ::nserror::nsresult,

    /* [must_use] attribute unsigned long redirectMode; */
    pub SetRedirectMode: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aRedirectMode: u32) -> ::nserror::nsresult,

    /* [must_use] attribute unsigned long fetchCacheMode; */
    pub GetFetchCacheMode: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aFetchCacheMode: *mut u32) -> ::nserror::nsresult,

    /* [must_use] attribute unsigned long fetchCacheMode; */
    pub SetFetchCacheMode: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aFetchCacheMode: u32) -> ::nserror::nsresult,

    /* [must_use] readonly attribute nsIURI topWindowURI; */
    pub GetTopWindowURI: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aTopWindowURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* [must_use] void setTopWindowURIIfUnknown (in nsIURI topWindowURI); */
    pub SetTopWindowURIIfUnknown: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, topWindowURI: *const nsIURI) -> ::nserror::nsresult,

    /* [must_use] readonly attribute nsIURI proxyURI; */
    pub GetProxyURI: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aProxyURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* [noscript,nostdcall,notxpcom] void setCorsPreflightParameters (in CStringArrayRef unsafeHeaders, in boolean shouldStripRequestBodyHeader); */
    /// Unable to generate binding because `native type const nsTArray<nsCString> unsupported`
    pub SetCorsPreflightParameters: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] void setAltDataForChild (in boolean aIsForChild); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub SetAltDataForChild: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] void disableAltDataCache (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub DisableAltDataCache: *const ::libc::c_void,

    /* [infallible] attribute boolean blockAuthPrompt; */
    pub GetBlockAuthPrompt: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aBlockAuthPrompt: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean blockAuthPrompt; */
    pub SetBlockAuthPrompt: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aBlockAuthPrompt: bool) -> ::nserror::nsresult,

    /* [must_use] attribute AString integrityMetadata; */
    pub GetIntegrityMetadata: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aIntegrityMetadata: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [must_use] attribute AString integrityMetadata; */
    pub SetIntegrityMetadata: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aIntegrityMetadata: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute ACString connectionInfoHashKey; */
    pub GetConnectionInfoHashKey: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aConnectionInfoHashKey: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [infallible,noscript] attribute unsigned long lastRedirectFlags; */
    pub GetLastRedirectFlags: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aLastRedirectFlags: *mut u32) -> ::nserror::nsresult,

    /* [infallible,noscript] attribute unsigned long lastRedirectFlags; */
    pub SetLastRedirectFlags: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aLastRedirectFlags: u32) -> ::nserror::nsresult,

    /* [noscript] attribute TimeStamp navigationStartTimeStamp; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetNavigationStartTimeStamp: *const ::libc::c_void,

    /* [noscript] attribute TimeStamp navigationStartTimeStamp; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub SetNavigationStartTimeStamp: *const ::libc::c_void,

    /* [noscript] void cancelByURLClassifier (in nsresult aErrorCode); */
    pub CancelByURLClassifier: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aErrorCode: ::nserror::nsresult) -> ::nserror::nsresult,

    /* [noscript,nostdcall,notxpcom] void setIPv4Disabled (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub SetIPv4Disabled: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] void setIPv6Disabled (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub SetIPv6Disabled: *const ::libc::c_void,

    /* readonly attribute nsILoadInfo_CrossOriginOpenerPolicy crossOriginOpenerPolicy; */
    pub GetCrossOriginOpenerPolicy: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aCrossOriginOpenerPolicy: *mut u8) -> ::nserror::nsresult,

    /* [noscript] nsILoadInfo_CrossOriginOpenerPolicy computeCrossOriginOpenerPolicy (in nsILoadInfo_CrossOriginOpenerPolicy aInitiatorPolicy); */
    pub ComputeCrossOriginOpenerPolicy: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aInitiatorPolicy:  u8, _retval: *mut u8) -> ::nserror::nsresult,

    /* [noscript] bool hasCrossOriginOpenerPolicyMismatch (); */
    pub HasCrossOriginOpenerPolicyMismatch: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, _retval: *mut bool) -> ::nserror::nsresult,

    /* [noscript] nsILoadInfo_CrossOriginEmbedderPolicy getResponseEmbedderPolicy (); */
    pub GetResponseEmbedderPolicy: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, _retval: *mut u8) -> ::nserror::nsresult,

    /* [nostdcall,notxpcom] attribute boolean hasNonEmptySandboxingFlag; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetHasNonEmptySandboxingFlag: *const ::libc::c_void,

    /* [nostdcall,notxpcom] attribute boolean hasNonEmptySandboxingFlag; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub SetHasNonEmptySandboxingFlag: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] void DoDiagnosticAssertWhenOnStopNotCalledOnDestroy (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub DoDiagnosticAssertWhenOnStopNotCalledOnDestroy: *const ::libc::c_void,

    /* [must_use] void setWaitForHTTPSSVCRecord (); */
    pub SetWaitForHTTPSSVCRecord: unsafe extern "system" fn (this: *const nsIHttpChannelInternal) -> ::nserror::nsresult,

    /* [must_use] readonly attribute boolean supportsHTTP3; */
    pub GetSupportsHTTP3: unsafe extern "system" fn (this: *const nsIHttpChannelInternal, aSupportsHTTP3: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHttpChannelInternal {
    /// ```text
    /// /**
    ///      * This flag is set to force relevant cookies to be sent with this load
    ///      * even if normally they wouldn't be.
    ///      */
    /// ```
    ///

    pub const THIRD_PARTY_FORCE_ALLOW: i64 = 1;


    pub const CORS_MODE_SAME_ORIGIN: i64 = 0;


    pub const CORS_MODE_NO_CORS: i64 = 1;


    pub const CORS_MODE_CORS: i64 = 2;


    pub const CORS_MODE_NAVIGATE: i64 = 3;


    pub const REDIRECT_MODE_FOLLOW: i64 = 0;


    pub const REDIRECT_MODE_ERROR: i64 = 1;


    pub const REDIRECT_MODE_MANUAL: i64 = 2;


    pub const FETCH_CACHE_MODE_DEFAULT: i64 = 0;


    pub const FETCH_CACHE_MODE_NO_STORE: i64 = 1;


    pub const FETCH_CACHE_MODE_RELOAD: i64 = 2;


    pub const FETCH_CACHE_MODE_NO_CACHE: i64 = 3;


    pub const FETCH_CACHE_MODE_FORCE_CACHE: i64 = 4;


    pub const FETCH_CACHE_MODE_ONLY_IF_CACHED: i64 = 5;

    /// ```text
    /// /**
    ///      * An http channel can own a reference to the document URI
    ///      */
    /// ```
    ///

    /// `[must_use] attribute nsIURI documentURI;`
    #[inline]
    pub unsafe fn GetDocumentURI(&self, aDocumentURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetDocumentURI)(self, aDocumentURI)
    }


    /// ```text
    /// /**
    ///      * An http channel can own a reference to the document URI
    ///      */
    /// ```
    ///

    /// `[must_use] attribute nsIURI documentURI;`
    #[inline]
    pub unsafe fn SetDocumentURI(&self, aDocumentURI: *const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).SetDocumentURI)(self, aDocumentURI)
    }


    /// ```text
    /// /**
    ///      * Get the major/minor version numbers for the request
    ///      */
    /// ```
    ///

    /// `[must_use] void getRequestVersion (out unsigned long major, out unsigned long minor);`
    #[inline]
    pub unsafe fn GetRequestVersion(&self, major: *mut u32, minor: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetRequestVersion)(self, major, minor)
    }


    /// ```text
    /// /**
    ///      * Get the major/minor version numbers for the response
    ///      */
    /// ```
    ///

    /// `[must_use] void getResponseVersion (out unsigned long major, out unsigned long minor);`
    #[inline]
    pub unsafe fn GetResponseVersion(&self, major: *mut u32, minor: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetResponseVersion)(self, major, minor)
    }



    /// `[must_use,noscript] void takeAllSecurityMessages (in securityMessagesArray aMessages);`
    const _TakeAllSecurityMessages: () = ();

    /// ```text
    /// /**
    ///      * Helper method to set a cookie with a consumer-provided
    ///      * cookie header, _but_ using the channel's other information
    ///      * (URI's, prompters, date headers etc).
    ///      *
    ///      * @param aCookieHeader
    ///      *        The cookie header to be parsed.
    ///      */
    /// ```
    ///

    /// `[must_use] void setCookie (in ACString aCookieHeader);`
    #[inline]
    pub unsafe fn SetCookie(&self, aCookieHeader: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetCookie)(self, aCookieHeader)
    }


    /// ```text
    /// /**
    ///      * Setup this channel as an application cache fallback channel.
    ///      */
    /// ```
    ///

    /// `[must_use] void setupFallbackChannel (in string aFallbackKey);`
    #[inline]
    pub unsafe fn SetupFallbackChannel(&self, aFallbackKey: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).SetupFallbackChannel)(self, aFallbackKey)
    }


    /// ```text
    /// /**
    ///      * Returns true in case this channel is used for auth;
    ///      * (the response header includes 'www-authenticate').
    ///      */
    /// ```
    ///

    /// `[must_use,noscript] readonly attribute bool isAuthChannel;`
    #[inline]
    pub unsafe fn GetIsAuthChannel(&self, aIsAuthChannel: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsAuthChannel)(self, aIsAuthChannel)
    }


    /// ```text
    /// /**
    ///      * When set, these flags modify the algorithm used to decide whether to
    ///      * send 3rd party cookies for a given channel.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute unsigned long thirdPartyFlags;`
    #[inline]
    pub unsafe fn GetThirdPartyFlags(&self, aThirdPartyFlags: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetThirdPartyFlags)(self, aThirdPartyFlags)
    }


    /// ```text
    /// /**
    ///      * When set, these flags modify the algorithm used to decide whether to
    ///      * send 3rd party cookies for a given channel.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute unsigned long thirdPartyFlags;`
    #[inline]
    pub unsafe fn SetThirdPartyFlags(&self, aThirdPartyFlags: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetThirdPartyFlags)(self, aThirdPartyFlags)
    }


    /// ```text
    /// /**
    ///      * This attribute was added before the "flags" above and is retained here
    ///      * for compatibility. When set to true, has the same effect as
    ///      * THIRD_PARTY_FORCE_ALLOW, described above.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute boolean forceAllowThirdPartyCookie;`
    #[inline]
    pub unsafe fn GetForceAllowThirdPartyCookie(&self, aForceAllowThirdPartyCookie: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetForceAllowThirdPartyCookie)(self, aForceAllowThirdPartyCookie)
    }


    /// ```text
    /// /**
    ///      * This attribute was added before the "flags" above and is retained here
    ///      * for compatibility. When set to true, has the same effect as
    ///      * THIRD_PARTY_FORCE_ALLOW, described above.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute boolean forceAllowThirdPartyCookie;`
    #[inline]
    pub unsafe fn SetForceAllowThirdPartyCookie(&self, aForceAllowThirdPartyCookie: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetForceAllowThirdPartyCookie)(self, aForceAllowThirdPartyCookie)
    }


    /// ```text
    /// /**
    ///      * External handlers may set this to true to notify the channel
    ///      * that it is open on behalf of a download.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute boolean channelIsForDownload;`
    #[inline]
    pub unsafe fn GetChannelIsForDownload(&self, aChannelIsForDownload: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetChannelIsForDownload)(self, aChannelIsForDownload)
    }


    /// ```text
    /// /**
    ///      * External handlers may set this to true to notify the channel
    ///      * that it is open on behalf of a download.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute boolean channelIsForDownload;`
    #[inline]
    pub unsafe fn SetChannelIsForDownload(&self, aChannelIsForDownload: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetChannelIsForDownload)(self, aChannelIsForDownload)
    }


    /// ```text
    /// /**
    ///      * The local IP address to which this channel is bound, in the
    ///      * format produced by PR_NetAddrToString. May be IPv4 or IPv6.
    ///      * Note: in the presence of NAT, this may not be the same as the
    ///      * address that the remote host thinks it's talking to.
    ///      *
    ///      * May throw NS_ERROR_NOT_AVAILABLE if accessed when the channel's
    ///      * endpoints are not yet determined, or in any case when
    ///      * nsIHttpActivityObserver.isActive is false. See bugs 534698 and 526207.
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute AUTF8String localAddress;`
    #[inline]
    pub unsafe fn GetLocalAddress(&self, aLocalAddress: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetLocalAddress)(self, aLocalAddress)
    }


    /// ```text
    /// /**
    ///      * The local port number to which this channel is bound.
    ///      *
    ///      * May throw NS_ERROR_NOT_AVAILABLE if accessed when the channel's
    ///      * endpoints are not yet determined, or in any case when
    ///      * nsIHttpActivityObserver.isActive is false. See bugs 534698 and 526207.
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute int32_t localPort;`
    #[inline]
    pub unsafe fn GetLocalPort(&self, aLocalPort: *mut int32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetLocalPort)(self, aLocalPort)
    }


    /// ```text
    /// /**
    ///      * The IP address of the remote host that this channel is
    ///      * connected to, in the format produced by PR_NetAddrToString.
    ///      *
    ///      * May throw NS_ERROR_NOT_AVAILABLE if accessed when the channel's
    ///      * endpoints are not yet determined, or in any case when
    ///      * nsIHttpActivityObserver.isActive is false. See bugs 534698 and 526207.
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute AUTF8String remoteAddress;`
    #[inline]
    pub unsafe fn GetRemoteAddress(&self, aRemoteAddress: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetRemoteAddress)(self, aRemoteAddress)
    }


    /// ```text
    /// /**
    ///      * The remote port number that this channel is connected to.
    ///      *
    ///      * May throw NS_ERROR_NOT_AVAILABLE if accessed when the channel's
    ///      * endpoints are not yet determined, or in any case when
    ///      * nsIHttpActivityObserver.isActive is false. See bugs 534698 and 526207.
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute int32_t remotePort;`
    #[inline]
    pub unsafe fn GetRemotePort(&self, aRemotePort: *mut int32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetRemotePort)(self, aRemotePort)
    }


    /// ```text
    /// /**
    ///      * Transfer chain of redirected cache-keys.
    ///      */
    /// ```
    ///

    /// `[must_use,noscript] void setCacheKeysRedirectChain (in StringArray cacheKeys);`
    const _SetCacheKeysRedirectChain: () = ();

    /// ```text
    /// /**
    ///      * HTTPUpgrade allows for the use of HTTP to bootstrap another protocol
    ///      * via the RFC 2616 Upgrade request header in conjunction with a 101 level
    ///      * response. The nsIHttpUpgradeListener will have its
    ///      * onTransportAvailable() method invoked if a matching 101 is processed.
    ///      * The arguments to onTransportAvailable provide the new protocol the low
    ///      * level tranport streams that are no longer used by HTTP. If any errors
    ///      * occur during the upgrade but the original request has (potentially)
    ///      * already received onStopRequest, the nsIHttpUpgradeListener will have its
    ///      * onUpgradeFailed() method invoked instead of onTransportAvailable().
    ///      *
    ///      * The onStartRequest and onStopRequest events are still delivered and the
    ///      * listener gets full control over the socket if and when onTransportAvailable
    ///      * is delivered. Note that if onStopRequest is called with an error, no
    ///      * methods on the nsIHttpUpgradeListener might be invoked at all.
    ///      *
    ///      * @param aProtocolName
    ///      *        The value of the HTTP Upgrade request header
    ///      * @param aListener
    ///      *        The callback object used to handle a successful upgrade
    ///      */
    /// ```
    ///

    /// `[must_use] void HTTPUpgrade (in ACString aProtocolName, in nsIHttpUpgradeListener aListener);`
    #[inline]
    pub unsafe fn HTTPUpgrade(&self, aProtocolName: *const ::nsstring::nsACString, aListener: *const nsIHttpUpgradeListener) -> ::nserror::nsresult {
        ((*self.vtable).HTTPUpgrade)(self, aProtocolName, aListener)
    }


    /// ```text
    /// /**
    ///      * Enable only CONNECT to a proxy. Fails if no HTTPUpgrade listener
    ///      * has been defined. An ALPN header is set using the upgrade protocol.
    ///      *
    ///      * Load flags are set with INHIBIT_CACHING, LOAD_ANONYMOUS,
    ///      * LOAD_BYPASS_CACHE, and LOAD_BYPASS_SERVICE_WORKER.
    ///      *
    ///      * Proxy resolve flags are set with RESOLVE_PREFER_HTTPS_PROXY and
    ///      * RESOLVE_ALWAYS_TUNNEL.
    ///      */
    /// ```
    ///

    /// `[must_use] void setConnectOnly ();`
    #[inline]
    pub unsafe fn SetConnectOnly(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SetConnectOnly)(self, )
    }


    /// ```text
    /// /**
    ///      * True iff the channel is CONNECT only.
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute boolean onlyConnect;`
    #[inline]
    pub unsafe fn GetOnlyConnect(&self, aOnlyConnect: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetOnlyConnect)(self, aOnlyConnect)
    }


    /// ```text
    /// /**
    ///      * Enable/Disable Spdy negotiation on per channel basis.
    ///      * The network.http.spdy.enabled preference is still a pre-requisite
    ///      * for starting spdy.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute boolean allowSpdy;`
    #[inline]
    pub unsafe fn GetAllowSpdy(&self, aAllowSpdy: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAllowSpdy)(self, aAllowSpdy)
    }


    /// ```text
    /// /**
    ///      * Enable/Disable Spdy negotiation on per channel basis.
    ///      * The network.http.spdy.enabled preference is still a pre-requisite
    ///      * for starting spdy.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute boolean allowSpdy;`
    #[inline]
    pub unsafe fn SetAllowSpdy(&self, aAllowSpdy: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetAllowSpdy)(self, aAllowSpdy)
    }


    /// ```text
    /// /**
    ///      * Enable/Disable HTTP3 negotiation on per channel basis.
    ///      * The network.http.http3.enabled preference is still a pre-requisite
    ///      * for starting HTTP3.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute boolean allowHttp3;`
    #[inline]
    pub unsafe fn GetAllowHttp3(&self, aAllowHttp3: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAllowHttp3)(self, aAllowHttp3)
    }


    /// ```text
    /// /**
    ///      * Enable/Disable HTTP3 negotiation on per channel basis.
    ///      * The network.http.http3.enabled preference is still a pre-requisite
    ///      * for starting HTTP3.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute boolean allowHttp3;`
    #[inline]
    pub unsafe fn SetAllowHttp3(&self, aAllowHttp3: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetAllowHttp3)(self, aAllowHttp3)
    }


    /// ```text
    /// /**
    ///      * This attribute en/disables the timeout for the first byte of an HTTP
    ///      * response. Enabled by default.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute boolean responseTimeoutEnabled;`
    #[inline]
    pub unsafe fn GetResponseTimeoutEnabled(&self, aResponseTimeoutEnabled: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetResponseTimeoutEnabled)(self, aResponseTimeoutEnabled)
    }


    /// ```text
    /// /**
    ///      * This attribute en/disables the timeout for the first byte of an HTTP
    ///      * response. Enabled by default.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute boolean responseTimeoutEnabled;`
    #[inline]
    pub unsafe fn SetResponseTimeoutEnabled(&self, aResponseTimeoutEnabled: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetResponseTimeoutEnabled)(self, aResponseTimeoutEnabled)
    }


    /// ```text
    /// /**
    ///      * If the underlying transport supports RWIN manipulation, this is the
    ///      * intiial window value for the channel. HTTP/2 implements this.
    ///      * 0 means no override from system default. Set before opening channel.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute unsigned long initialRwin;`
    #[inline]
    pub unsafe fn GetInitialRwin(&self, aInitialRwin: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetInitialRwin)(self, aInitialRwin)
    }


    /// ```text
    /// /**
    ///      * If the underlying transport supports RWIN manipulation, this is the
    ///      * intiial window value for the channel. HTTP/2 implements this.
    ///      * 0 means no override from system default. Set before opening channel.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute unsigned long initialRwin;`
    #[inline]
    pub unsafe fn SetInitialRwin(&self, aInitialRwin: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetInitialRwin)(self, aInitialRwin)
    }


    /// ```text
    /// /**
    ///      * Get value of the URI passed to nsIHttpChannel.redirectTo() if any.
    ///      * May return null when redirectTo() has not been called.
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute nsIURI apiRedirectToURI;`
    #[inline]
    pub unsafe fn GetApiRedirectToURI(&self, aApiRedirectToURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetApiRedirectToURI)(self, aApiRedirectToURI)
    }


    /// ```text
    /// /**
    ///      * Enable/Disable use of Alternate Services with this channel.
    ///      * The network.http.altsvc.enabled preference is still a pre-requisite.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute boolean allowAltSvc;`
    #[inline]
    pub unsafe fn GetAllowAltSvc(&self, aAllowAltSvc: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAllowAltSvc)(self, aAllowAltSvc)
    }


    /// ```text
    /// /**
    ///      * Enable/Disable use of Alternate Services with this channel.
    ///      * The network.http.altsvc.enabled preference is still a pre-requisite.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute boolean allowAltSvc;`
    #[inline]
    pub unsafe fn SetAllowAltSvc(&self, aAllowAltSvc: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetAllowAltSvc)(self, aAllowAltSvc)
    }


    /// ```text
    /// /**
    ///      * If true, do not use newer protocol features that might have interop problems
    ///      * on the Internet. Intended only for use with critical infra like the updater.
    ///      * default is false.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute boolean beConservative;`
    #[inline]
    pub unsafe fn GetBeConservative(&self, aBeConservative: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetBeConservative)(self, aBeConservative)
    }


    /// ```text
    /// /**
    ///      * If true, do not use newer protocol features that might have interop problems
    ///      * on the Internet. Intended only for use with critical infra like the updater.
    ///      * default is false.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute boolean beConservative;`
    #[inline]
    pub unsafe fn SetBeConservative(&self, aBeConservative: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetBeConservative)(self, aBeConservative)
    }


    /// ```text
    /// /**
    ///      * True if channel is used by the internal trusted recursive resolver
    ///      * This flag places data for the request in a cache segment specific to TRR
    ///      */
    /// ```
    ///

    /// `[must_use,noscript] attribute boolean isTRRServiceChannel;`
    #[inline]
    pub unsafe fn GetIsTRRServiceChannel(&self, aIsTRRServiceChannel: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsTRRServiceChannel)(self, aIsTRRServiceChannel)
    }


    /// ```text
    /// /**
    ///      * True if channel is used by the internal trusted recursive resolver
    ///      * This flag places data for the request in a cache segment specific to TRR
    ///      */
    /// ```
    ///

    /// `[must_use,noscript] attribute boolean isTRRServiceChannel;`
    #[inline]
    pub unsafe fn SetIsTRRServiceChannel(&self, aIsTRRServiceChannel: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetIsTRRServiceChannel)(self, aIsTRRServiceChannel)
    }


    /// ```text
    /// /**
    ///      * If the channel's remote IP was resolved using TRR.
    ///      * Is false for resources loaded from the cache or resources that have an
    ///      * IP literal host.
    ///      */
    /// ```
    ///

    /// `[must_use,noscript] readonly attribute boolean isResolvedByTRR;`
    #[inline]
    pub unsafe fn GetIsResolvedByTRR(&self, aIsResolvedByTRR: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsResolvedByTRR)(self, aIsResolvedByTRR)
    }


    /// ```text
    /// /**
    ///      * An opaque flags for non-standard behavior of the TLS system.
    ///      * It is unlikely this will need to be set outside of telemetry studies
    ///      * relating to the TLS implementation.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute unsigned long tlsFlags;`
    #[inline]
    pub unsafe fn GetTlsFlags(&self, aTlsFlags: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetTlsFlags)(self, aTlsFlags)
    }


    /// ```text
    /// /**
    ///      * An opaque flags for non-standard behavior of the TLS system.
    ///      * It is unlikely this will need to be set outside of telemetry studies
    ///      * relating to the TLS implementation.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute unsigned long tlsFlags;`
    #[inline]
    pub unsafe fn SetTlsFlags(&self, aTlsFlags: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetTlsFlags)(self, aTlsFlags)
    }



    /// `[must_use] readonly attribute PRTime lastModifiedTime;`
    #[inline]
    pub unsafe fn GetLastModifiedTime(&self, aLastModifiedTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetLastModifiedTime)(self, aLastModifiedTime)
    }


    /// ```text
    /// /**
    ///      * Set by nsCORSListenerProxy if credentials should be included in
    ///      * cross-origin requests. false indicates "same-origin", users should still
    ///      * check flag LOAD_ANONYMOUS!
    ///      */
    /// ```
    ///

    /// `[must_use] attribute boolean corsIncludeCredentials;`
    #[inline]
    pub unsafe fn GetCorsIncludeCredentials(&self, aCorsIncludeCredentials: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCorsIncludeCredentials)(self, aCorsIncludeCredentials)
    }


    /// ```text
    /// /**
    ///      * Set by nsCORSListenerProxy if credentials should be included in
    ///      * cross-origin requests. false indicates "same-origin", users should still
    ///      * check flag LOAD_ANONYMOUS!
    ///      */
    /// ```
    ///

    /// `[must_use] attribute boolean corsIncludeCredentials;`
    #[inline]
    pub unsafe fn SetCorsIncludeCredentials(&self, aCorsIncludeCredentials: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetCorsIncludeCredentials)(self, aCorsIncludeCredentials)
    }


    /// ```text
    /// /**
    ///      * Set by nsCORSListenerProxy to indicate CORS load type. Defaults to CORS_MODE_NO_CORS.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute unsigned long corsMode;`
    #[inline]
    pub unsafe fn GetCorsMode(&self, aCorsMode: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetCorsMode)(self, aCorsMode)
    }


    /// ```text
    /// /**
    ///      * Set by nsCORSListenerProxy to indicate CORS load type. Defaults to CORS_MODE_NO_CORS.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute unsigned long corsMode;`
    #[inline]
    pub unsafe fn SetCorsMode(&self, aCorsMode: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetCorsMode)(self, aCorsMode)
    }


    /// ```text
    /// /**
    ///      * Set to indicate Request.redirect mode exposed during ServiceWorker
    ///      * interception. No policy enforcement is performed by the channel for this
    ///      * value.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute unsigned long redirectMode;`
    #[inline]
    pub unsafe fn GetRedirectMode(&self, aRedirectMode: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetRedirectMode)(self, aRedirectMode)
    }


    /// ```text
    /// /**
    ///      * Set to indicate Request.redirect mode exposed during ServiceWorker
    ///      * interception. No policy enforcement is performed by the channel for this
    ///      * value.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute unsigned long redirectMode;`
    #[inline]
    pub unsafe fn SetRedirectMode(&self, aRedirectMode: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetRedirectMode)(self, aRedirectMode)
    }


    /// ```text
    /// /**
    ///      * Set to indicate Request.cache mode, which simulates the fetch API
    ///      * semantics, and is also used for exposing this value to the Web page
    ///      * during service worker interception.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute unsigned long fetchCacheMode;`
    #[inline]
    pub unsafe fn GetFetchCacheMode(&self, aFetchCacheMode: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetFetchCacheMode)(self, aFetchCacheMode)
    }


    /// ```text
    /// /**
    ///      * Set to indicate Request.cache mode, which simulates the fetch API
    ///      * semantics, and is also used for exposing this value to the Web page
    ///      * during service worker interception.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute unsigned long fetchCacheMode;`
    #[inline]
    pub unsafe fn SetFetchCacheMode(&self, aFetchCacheMode: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetFetchCacheMode)(self, aFetchCacheMode)
    }


    /// ```text
    /// /**
    ///      * The URI of the top-level window that's associated with this channel.
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute nsIURI topWindowURI;`
    #[inline]
    pub unsafe fn GetTopWindowURI(&self, aTopWindowURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetTopWindowURI)(self, aTopWindowURI)
    }


    /// ```text
    /// /**
    ///      * Set top-level window URI to this channel only when the topWindowURI
    ///      * is null and there is no window associated to this channel.
    ///      * Note that the current usage of this method is only for xpcshell test.
    ///      */
    /// ```
    ///

    /// `[must_use] void setTopWindowURIIfUnknown (in nsIURI topWindowURI);`
    #[inline]
    pub unsafe fn SetTopWindowURIIfUnknown(&self, topWindowURI: *const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).SetTopWindowURIIfUnknown)(self, topWindowURI)
    }


    /// ```text
    /// /**
    ///      * Read the proxy URI, which, if non-null, will be used to resolve
    ///      * proxies for this channel.
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute nsIURI proxyURI;`
    #[inline]
    pub unsafe fn GetProxyURI(&self, aProxyURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetProxyURI)(self, aProxyURI)
    }


    /// ```text
    /// /**
    ///      * Make cross-origin CORS loads happen with a CORS preflight, and specify
    ///      * the CORS preflight parameters.
    ///      */
    /// ```
    ///

    /// `[noscript,nostdcall,notxpcom] void setCorsPreflightParameters (in CStringArrayRef unsafeHeaders, in boolean shouldStripRequestBodyHeader);`
    const _SetCorsPreflightParameters: () = ();


    /// `[noscript,nostdcall,notxpcom] void setAltDataForChild (in boolean aIsForChild);`
    const _SetAltDataForChild: () = ();

    /// ```text
    /// /**
    ///      * Prevent the use of alt-data cache for this request.  Use by the
    ///      * extension StreamFilter class to force use of the regular cache.
    ///      */
    /// ```
    ///

    /// `[noscript,nostdcall,notxpcom] void disableAltDataCache ();`
    const _DisableAltDataCache: () = ();

    /// ```text
    /// /**
    ///      * When set to true, the channel will not pop any authentication prompts up
    ///      * to the user.  When provided or cached credentials lead to an
    ///      * authentication failure, that failure will be propagated to the channel
    ///      * listener.  Must be called before opening the channel, otherwise throws.
    ///      */
    /// ```
    ///

    /// `[infallible] attribute boolean blockAuthPrompt;`
    #[inline]
    pub unsafe fn GetBlockAuthPrompt(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetBlockAuthPrompt)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///      * When set to true, the channel will not pop any authentication prompts up
    ///      * to the user.  When provided or cached credentials lead to an
    ///      * authentication failure, that failure will be propagated to the channel
    ///      * listener.  Must be called before opening the channel, otherwise throws.
    ///      */
    /// ```
    ///

    /// `[infallible] attribute boolean blockAuthPrompt;`
    #[inline]
    pub unsafe fn SetBlockAuthPrompt(&self, aBlockAuthPrompt: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetBlockAuthPrompt)(self, aBlockAuthPrompt)
    }


    /// ```text
    /// /**
    ///      * Set to indicate Request.integrity.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute AString integrityMetadata;`
    #[inline]
    pub unsafe fn GetIntegrityMetadata(&self, aIntegrityMetadata: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetIntegrityMetadata)(self, aIntegrityMetadata)
    }


    /// ```text
    /// /**
    ///      * Set to indicate Request.integrity.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute AString integrityMetadata;`
    #[inline]
    pub unsafe fn SetIntegrityMetadata(&self, aIntegrityMetadata: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetIntegrityMetadata)(self, aIntegrityMetadata)
    }


    /// ```text
    /// /**
    ///      * The connection info's hash key. We use it to test connection separation.
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute ACString connectionInfoHashKey;`
    #[inline]
    pub unsafe fn GetConnectionInfoHashKey(&self, aConnectionInfoHashKey: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetConnectionInfoHashKey)(self, aConnectionInfoHashKey)
    }


    /// ```text
    /// /**
    ///      * If this channel was created as the result of a redirect, then this
    ///      * value will reflect the redirect flags passed to the
    ///      * SetupReplacementChannel() method.
    ///      */
    /// ```
    ///

    /// `[infallible,noscript] attribute unsigned long lastRedirectFlags;`
    #[inline]
    pub unsafe fn GetLastRedirectFlags(&self) -> u32 {
        let mut result = <u32 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetLastRedirectFlags)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///      * If this channel was created as the result of a redirect, then this
    ///      * value will reflect the redirect flags passed to the
    ///      * SetupReplacementChannel() method.
    ///      */
    /// ```
    ///

    /// `[infallible,noscript] attribute unsigned long lastRedirectFlags;`
    #[inline]
    pub unsafe fn SetLastRedirectFlags(&self, aLastRedirectFlags: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetLastRedirectFlags)(self, aLastRedirectFlags)
    }



    /// `[noscript] attribute TimeStamp navigationStartTimeStamp;`
    const _GetNavigationStartTimeStamp: () = ();


    /// `[noscript] attribute TimeStamp navigationStartTimeStamp;`
    const _SetNavigationStartTimeStamp: () = ();

    /// ```text
    /// /**
    ///      * Cancel a channel because we have determined that it needs to be blocked
    ///      * for safe-browsing protection.  This is an internal API that is meant to
    ///      * be called by the channel classifier.  Please DO NOT use this API if you
    ///      * don't know whether you should be using it.
    ///      */
    /// ```
    ///

    /// `[noscript] void cancelByURLClassifier (in nsresult aErrorCode);`
    #[inline]
    pub unsafe fn CancelByURLClassifier(&self, aErrorCode: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).CancelByURLClassifier)(self, aErrorCode)
    }


    /// ```text
    /// /**
    ///      * The channel will be loaded over IPv6, disabling IPv4.
    ///      */
    /// ```
    ///

    /// `[noscript,nostdcall,notxpcom] void setIPv4Disabled ();`
    const _SetIPv4Disabled: () = ();

    /// ```text
    /// /**
    ///      * The channel will be loaded over IPv4, disabling IPv6.
    ///      */
    /// ```
    ///

    /// `[noscript,nostdcall,notxpcom] void setIPv6Disabled ();`
    const _SetIPv6Disabled: () = ();

    /// ```text
    /// /**
    ///      * Returns a cached CrossOriginOpenerPolicy that is computed just before we
    ///      * determine if there is a policy mismatch.
    ///      * @throws NS_ERROR_NOT_AVAILABLE if it has not been computed yet
    ///      */
    /// ```
    ///

    /// `readonly attribute nsILoadInfo_CrossOriginOpenerPolicy crossOriginOpenerPolicy;`
    #[inline]
    pub unsafe fn GetCrossOriginOpenerPolicy(&self, aCrossOriginOpenerPolicy: *mut u8) -> ::nserror::nsresult {
        ((*self.vtable).GetCrossOriginOpenerPolicy)(self, aCrossOriginOpenerPolicy)
    }


    /// ```text
    /// /**
    ///      * Called during onStartRequest to compute the cross-origin-opener-policy
    ///      * for a given channel.
    ///      */
    /// ```
    ///

    /// `[noscript] nsILoadInfo_CrossOriginOpenerPolicy computeCrossOriginOpenerPolicy (in nsILoadInfo_CrossOriginOpenerPolicy aInitiatorPolicy);`
    #[inline]
    pub unsafe fn ComputeCrossOriginOpenerPolicy(&self, aInitiatorPolicy:  u8, _retval: *mut u8) -> ::nserror::nsresult {
        ((*self.vtable).ComputeCrossOriginOpenerPolicy)(self, aInitiatorPolicy, _retval)
    }



    /// `[noscript] bool hasCrossOriginOpenerPolicyMismatch ();`
    #[inline]
    pub unsafe fn HasCrossOriginOpenerPolicyMismatch(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasCrossOriginOpenerPolicyMismatch)(self, _retval)
    }



    /// `[noscript] nsILoadInfo_CrossOriginEmbedderPolicy getResponseEmbedderPolicy ();`
    #[inline]
    pub unsafe fn GetResponseEmbedderPolicy(&self, _retval: *mut u8) -> ::nserror::nsresult {
        ((*self.vtable).GetResponseEmbedderPolicy)(self, _retval)
    }



    /// `[nostdcall,notxpcom] attribute boolean hasNonEmptySandboxingFlag;`
    const _GetHasNonEmptySandboxingFlag: () = ();


    /// `[nostdcall,notxpcom] attribute boolean hasNonEmptySandboxingFlag;`
    const _SetHasNonEmptySandboxingFlag: () = ();


    /// `[noscript,nostdcall,notxpcom] void DoDiagnosticAssertWhenOnStopNotCalledOnDestroy ();`
    const _DoDiagnosticAssertWhenOnStopNotCalledOnDestroy: () = ();

    /// ```text
    /// /**
    ///      * If this is called, this channel's transaction will not be dispatched
    ///      * until the HTTPSSVC record is available.
    ///      */
    /// ```
    ///

    /// `[must_use] void setWaitForHTTPSSVCRecord ();`
    #[inline]
    pub unsafe fn SetWaitForHTTPSSVCRecord(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SetWaitForHTTPSSVCRecord)(self, )
    }


    /// ```text
    /// /**
    ///      * This attribute indicates if the channel has support for HTTP3
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute boolean supportsHTTP3;`
    #[inline]
    pub unsafe fn GetSupportsHTTP3(&self, aSupportsHTTP3: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetSupportsHTTP3)(self, aSupportsHTTP3)
    }


}


