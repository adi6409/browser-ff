//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsILoadInfo.idl
//


/// `typedef uint32_t  nsSecurityFlags;`
///


pub type nsSecurityFlags = u32;


/// `interface nsILoadInfo : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsILoadInfo {
    vtable: *const nsILoadInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsILoadInfo.
unsafe impl XpCom for nsILoadInfo {
    const IID: nsIID = nsID(0xddc65bf9, 0x2f60, 0x41ab,
        [0xb2, 0x2a, 0x4f, 0x1a, 0xe9, 0xef, 0xcd, 0x36]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsILoadInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsILoadInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsILoadInfoCoerce {
    /// Cheaply cast a value of this type from a `nsILoadInfo`.
    fn coerce_from(v: &nsILoadInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsILoadInfoCoerce for nsILoadInfo {
    #[inline]
    fn coerce_from(v: &nsILoadInfo) -> &Self {
        v
    }
}

impl nsILoadInfo {
    /// Cast this `nsILoadInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsILoadInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsILoadInfo {
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
impl<T: nsISupportsCoerce> nsILoadInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILoadInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsILoadInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsILoadInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIPrincipal loadingPrincipal; */
    pub GetLoadingPrincipal: unsafe extern "system" fn (this: *const nsILoadInfo, aLoadingPrincipal: *mut *const nsIPrincipal) -> ::nserror::nsresult,

    /* [noscript,nostdcall,notxpcom] nsIPrincipal virtualGetLoadingPrincipal (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub VirtualGetLoadingPrincipal: *const ::libc::c_void,

    /* readonly attribute nsIPrincipal triggeringPrincipal; */
    pub GetTriggeringPrincipal: unsafe extern "system" fn (this: *const nsILoadInfo, aTriggeringPrincipal: *mut *const nsIPrincipal) -> ::nserror::nsresult,

    /* [binaryname(TriggeringPrincipal),noscript,nostdcall,notxpcom] nsIPrincipal binaryTriggeringPrincipal (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub TriggeringPrincipal: *const ::libc::c_void,

    /* attribute nsIPrincipal principalToInherit; */
    pub GetPrincipalToInherit: unsafe extern "system" fn (this: *const nsILoadInfo, aPrincipalToInherit: *mut *const nsIPrincipal) -> ::nserror::nsresult,

    /* attribute nsIPrincipal principalToInherit; */
    pub SetPrincipalToInherit: unsafe extern "system" fn (this: *const nsILoadInfo, aPrincipalToInherit: *const nsIPrincipal) -> ::nserror::nsresult,

    /* [binaryname(PrincipalToInherit),noscript,nostdcall,notxpcom] nsIPrincipal binaryPrincipalToInherit (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub PrincipalToInherit: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] nsIPrincipal FindPrincipalToInherit (in nsIChannel aChannel); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub FindPrincipalToInherit: *const ::libc::c_void,

    /* readonly attribute Document loadingDocument; */
    pub GetLoadingDocument: unsafe extern "system" fn (this: *const nsILoadInfo, aLoadingDocument: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* [binaryname(LoadingNode),noscript,nostdcall,notxpcom] nsINode binaryLoadingNode (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub LoadingNode: *const ::libc::c_void,

    /* [binaryname(ContextForTopLevelLoad),noscript,nostdcall,notxpcom] LoadContextRef binaryContextForTopLevelLoad (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub ContextForTopLevelLoad: *const ::libc::c_void,

    /* [binaryname(LoadingContextXPCOM)] readonly attribute nsISupports loadingContext; */
    pub GetLoadingContextXPCOM: unsafe extern "system" fn (this: *const nsILoadInfo, aLoadingContext: *mut *const nsISupports) -> ::nserror::nsresult,

    /* [binaryname(GetLoadingContext),noscript,nostdcall,notxpcom] LoadContextRef binaryGetLoadingContext (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetLoadingContext: *const ::libc::c_void,

    /* readonly attribute nsSecurityFlags securityFlags; */
    pub GetSecurityFlags: unsafe extern "system" fn (this: *const nsILoadInfo, aSecurityFlags: *mut nsSecurityFlags) -> ::nserror::nsresult,

    /* [infallible] readonly attribute unsigned long sandboxFlags; */
    pub GetSandboxFlags: unsafe extern "system" fn (this: *const nsILoadInfo, aSandboxFlags: *mut u32) -> ::nserror::nsresult,

    /* [infallible] attribute unsigned long triggeringSandboxFlags; */
    pub GetTriggeringSandboxFlags: unsafe extern "system" fn (this: *const nsILoadInfo, aTriggeringSandboxFlags: *mut u32) -> ::nserror::nsresult,

    /* [infallible] attribute unsigned long triggeringSandboxFlags; */
    pub SetTriggeringSandboxFlags: unsafe extern "system" fn (this: *const nsILoadInfo, aTriggeringSandboxFlags: u32) -> ::nserror::nsresult,

    /* [infallible] readonly attribute unsigned long securityMode; */
    pub GetSecurityMode: unsafe extern "system" fn (this: *const nsILoadInfo, aSecurityMode: *mut u32) -> ::nserror::nsresult,

    /* [infallible] attribute boolean skipContentSniffing; */
    pub GetSkipContentSniffing: unsafe extern "system" fn (this: *const nsILoadInfo, aSkipContentSniffing: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean skipContentSniffing; */
    pub SetSkipContentSniffing: unsafe extern "system" fn (this: *const nsILoadInfo, aSkipContentSniffing: bool) -> ::nserror::nsresult,

    /* [infallible] attribute unsigned long httpsOnlyStatus; */
    pub GetHttpsOnlyStatus: unsafe extern "system" fn (this: *const nsILoadInfo, aHttpsOnlyStatus: *mut u32) -> ::nserror::nsresult,

    /* [infallible] attribute unsigned long httpsOnlyStatus; */
    pub SetHttpsOnlyStatus: unsafe extern "system" fn (this: *const nsILoadInfo, aHttpsOnlyStatus: u32) -> ::nserror::nsresult,

    /* [infallible] attribute boolean hasValidUserGestureActivation; */
    pub GetHasValidUserGestureActivation: unsafe extern "system" fn (this: *const nsILoadInfo, aHasValidUserGestureActivation: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean hasValidUserGestureActivation; */
    pub SetHasValidUserGestureActivation: unsafe extern "system" fn (this: *const nsILoadInfo, aHasValidUserGestureActivation: bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean allowDeprecatedSystemRequests; */
    pub GetAllowDeprecatedSystemRequests: unsafe extern "system" fn (this: *const nsILoadInfo, aAllowDeprecatedSystemRequests: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean allowDeprecatedSystemRequests; */
    pub SetAllowDeprecatedSystemRequests: unsafe extern "system" fn (this: *const nsILoadInfo, aAllowDeprecatedSystemRequests: bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean parserCreatedScript; */
    pub GetParserCreatedScript: unsafe extern "system" fn (this: *const nsILoadInfo, aParserCreatedScript: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean parserCreatedScript; */
    pub SetParserCreatedScript: unsafe extern "system" fn (this: *const nsILoadInfo, aParserCreatedScript: bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean isInDevToolsContext; */
    pub GetIsInDevToolsContext: unsafe extern "system" fn (this: *const nsILoadInfo, aIsInDevToolsContext: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean isInDevToolsContext; */
    pub SetIsInDevToolsContext: unsafe extern "system" fn (this: *const nsILoadInfo, aIsInDevToolsContext: bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean isInThirdPartyContext; */
    pub GetIsInThirdPartyContext: unsafe extern "system" fn (this: *const nsILoadInfo, aIsInThirdPartyContext: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean isInThirdPartyContext; */
    pub SetIsInThirdPartyContext: unsafe extern "system" fn (this: *const nsILoadInfo, aIsInThirdPartyContext: bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean isThirdPartyContextToTopWindow; */
    pub GetIsThirdPartyContextToTopWindow: unsafe extern "system" fn (this: *const nsILoadInfo, aIsThirdPartyContextToTopWindow: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean isThirdPartyContextToTopWindow; */
    pub SetIsThirdPartyContextToTopWindow: unsafe extern "system" fn (this: *const nsILoadInfo, aIsThirdPartyContextToTopWindow: bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute unsigned long cookiePolicy; */
    pub GetCookiePolicy: unsafe extern "system" fn (this: *const nsILoadInfo, aCookiePolicy: *mut u32) -> ::nserror::nsresult,

    /* attribute nsICookieJarSettings cookieJarSettings; */
    pub GetCookieJarSettings: unsafe extern "system" fn (this: *const nsILoadInfo, aCookieJarSettings: *mut*const nsICookieJarSettings) -> ::nserror::nsresult,

    /* attribute nsICookieJarSettings cookieJarSettings; */
    pub SetCookieJarSettings: unsafe extern "system" fn (this: *const nsILoadInfo, aCookieJarSettings: *const nsICookieJarSettings) -> ::nserror::nsresult,

    /* [infallible] attribute boolean hasStoragePermission; */
    pub GetHasStoragePermission: unsafe extern "system" fn (this: *const nsILoadInfo, aHasStoragePermission: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean hasStoragePermission; */
    pub SetHasStoragePermission: unsafe extern "system" fn (this: *const nsILoadInfo, aHasStoragePermission: bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean forceInheritPrincipal; */
    pub GetForceInheritPrincipal: unsafe extern "system" fn (this: *const nsILoadInfo, aForceInheritPrincipal: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean forceInheritPrincipalOverruleOwner; */
    pub GetForceInheritPrincipalOverruleOwner: unsafe extern "system" fn (this: *const nsILoadInfo, aForceInheritPrincipalOverruleOwner: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean loadingSandboxed; */
    pub GetLoadingSandboxed: unsafe extern "system" fn (this: *const nsILoadInfo, aLoadingSandboxed: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean aboutBlankInherits; */
    pub GetAboutBlankInherits: unsafe extern "system" fn (this: *const nsILoadInfo, aAboutBlankInherits: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean allowChrome; */
    pub GetAllowChrome: unsafe extern "system" fn (this: *const nsILoadInfo, aAllowChrome: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean disallowScript; */
    pub GetDisallowScript: unsafe extern "system" fn (this: *const nsILoadInfo, aDisallowScript: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean dontFollowRedirects; */
    pub GetDontFollowRedirects: unsafe extern "system" fn (this: *const nsILoadInfo, aDontFollowRedirects: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean loadErrorPage; */
    pub GetLoadErrorPage: unsafe extern "system" fn (this: *const nsILoadInfo, aLoadErrorPage: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean isFormSubmission; */
    pub GetIsFormSubmission: unsafe extern "system" fn (this: *const nsILoadInfo, aIsFormSubmission: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean isFormSubmission; */
    pub SetIsFormSubmission: unsafe extern "system" fn (this: *const nsILoadInfo, aIsFormSubmission: bool) -> ::nserror::nsresult,

    /* readonly attribute nsContentPolicyType externalContentPolicyType; */
    pub GetExternalContentPolicyType: unsafe extern "system" fn (this: *const nsILoadInfo, aExternalContentPolicyType: *mut nsContentPolicyType) -> ::nserror::nsresult,

    /* [infallible] attribute boolean sendCSPViolationEvents; */
    pub GetSendCSPViolationEvents: unsafe extern "system" fn (this: *const nsILoadInfo, aSendCSPViolationEvents: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean sendCSPViolationEvents; */
    pub SetSendCSPViolationEvents: unsafe extern "system" fn (this: *const nsILoadInfo, aSendCSPViolationEvents: bool) -> ::nserror::nsresult,

    /* [binaryname(InternalContentPolicyType),noscript,nostdcall,notxpcom] nsContentPolicyType binaryInternalContentPolicyType (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub InternalContentPolicyType: *const ::libc::c_void,

    /* readonly attribute nsContentPolicyType internalContentPolicyType; */
    pub GetInternalContentPolicyType: unsafe extern "system" fn (this: *const nsILoadInfo, aInternalContentPolicyType: *mut nsContentPolicyType) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean blockAllMixedContent; */
    pub GetBlockAllMixedContent: unsafe extern "system" fn (this: *const nsILoadInfo, aBlockAllMixedContent: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean upgradeInsecureRequests; */
    pub GetUpgradeInsecureRequests: unsafe extern "system" fn (this: *const nsILoadInfo, aUpgradeInsecureRequests: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean browserUpgradeInsecureRequests; */
    pub GetBrowserUpgradeInsecureRequests: unsafe extern "system" fn (this: *const nsILoadInfo, aBrowserUpgradeInsecureRequests: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean browserDidUpgradeInsecureRequests; */
    pub GetBrowserDidUpgradeInsecureRequests: unsafe extern "system" fn (this: *const nsILoadInfo, aBrowserDidUpgradeInsecureRequests: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean browserDidUpgradeInsecureRequests; */
    pub SetBrowserDidUpgradeInsecureRequests: unsafe extern "system" fn (this: *const nsILoadInfo, aBrowserDidUpgradeInsecureRequests: bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean browserWouldUpgradeInsecureRequests; */
    pub GetBrowserWouldUpgradeInsecureRequests: unsafe extern "system" fn (this: *const nsILoadInfo, aBrowserWouldUpgradeInsecureRequests: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean forceAllowDataURI; */
    pub GetForceAllowDataURI: unsafe extern "system" fn (this: *const nsILoadInfo, aForceAllowDataURI: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean forceAllowDataURI; */
    pub SetForceAllowDataURI: unsafe extern "system" fn (this: *const nsILoadInfo, aForceAllowDataURI: bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean allowInsecureRedirectToDataURI; */
    pub GetAllowInsecureRedirectToDataURI: unsafe extern "system" fn (this: *const nsILoadInfo, aAllowInsecureRedirectToDataURI: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean allowInsecureRedirectToDataURI; */
    pub SetAllowInsecureRedirectToDataURI: unsafe extern "system" fn (this: *const nsILoadInfo, aAllowInsecureRedirectToDataURI: bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean bypassCORSChecks; */
    pub GetBypassCORSChecks: unsafe extern "system" fn (this: *const nsILoadInfo, aBypassCORSChecks: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean bypassCORSChecks; */
    pub SetBypassCORSChecks: unsafe extern "system" fn (this: *const nsILoadInfo, aBypassCORSChecks: bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean skipContentPolicyCheckForWebRequest; */
    pub GetSkipContentPolicyCheckForWebRequest: unsafe extern "system" fn (this: *const nsILoadInfo, aSkipContentPolicyCheckForWebRequest: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean skipContentPolicyCheckForWebRequest; */
    pub SetSkipContentPolicyCheckForWebRequest: unsafe extern "system" fn (this: *const nsILoadInfo, aSkipContentPolicyCheckForWebRequest: bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean originalFrameSrcLoad; */
    pub GetOriginalFrameSrcLoad: unsafe extern "system" fn (this: *const nsILoadInfo, aOriginalFrameSrcLoad: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean originalFrameSrcLoad; */
    pub SetOriginalFrameSrcLoad: unsafe extern "system" fn (this: *const nsILoadInfo, aOriginalFrameSrcLoad: bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean forceInheritPrincipalDropped; */
    pub GetForceInheritPrincipalDropped: unsafe extern "system" fn (this: *const nsILoadInfo, aForceInheritPrincipalDropped: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute unsigned long long innerWindowID; */
    pub GetInnerWindowID: unsafe extern "system" fn (this: *const nsILoadInfo, aInnerWindowID: *mut u64) -> ::nserror::nsresult,

    /* [infallible] readonly attribute unsigned long long browsingContextID; */
    pub GetBrowsingContextID: unsafe extern "system" fn (this: *const nsILoadInfo, aBrowsingContextID: *mut u64) -> ::nserror::nsresult,

    /* [infallible] readonly attribute BrowsingContext browsingContext; */
    pub GetBrowsingContext: unsafe extern "system" fn (this: *const nsILoadInfo, aBrowsingContext: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* [infallible] readonly attribute unsigned long long frameBrowsingContextID; */
    pub GetFrameBrowsingContextID: unsafe extern "system" fn (this: *const nsILoadInfo, aFrameBrowsingContextID: *mut u64) -> ::nserror::nsresult,

    /* [infallible] readonly attribute BrowsingContext frameBrowsingContext; */
    pub GetFrameBrowsingContext: unsafe extern "system" fn (this: *const nsILoadInfo, aFrameBrowsingContext: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* [infallible] readonly attribute unsigned long long targetBrowsingContextID; */
    pub GetTargetBrowsingContextID: unsafe extern "system" fn (this: *const nsILoadInfo, aTargetBrowsingContextID: *mut u64) -> ::nserror::nsresult,

    /* [infallible] readonly attribute BrowsingContext targetBrowsingContext; */
    pub GetTargetBrowsingContext: unsafe extern "system" fn (this: *const nsILoadInfo, aTargetBrowsingContext: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* void resetPrincipalToInheritToNullPrincipal (); */
    pub ResetPrincipalToInheritToNullPrincipal: unsafe extern "system" fn (this: *const nsILoadInfo) -> ::nserror::nsresult,

    /* [binaryname(ScriptableOriginAttributes),implicit_jscontext] attribute jsval originAttributes; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetScriptableOriginAttributes: *const ::libc::c_void,

    /* [binaryname(ScriptableOriginAttributes),implicit_jscontext] attribute jsval originAttributes; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub SetScriptableOriginAttributes: *const ::libc::c_void,

    /* [binaryname(GetOriginAttributes),noscript,nostdcall] OriginAttributes binaryGetOriginAttributes (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetOriginAttributes: *const ::libc::c_void,

    /* [binaryname(SetOriginAttributes),noscript,nostdcall] void binarySetOriginAttributes (in const_OriginAttributesRef aOriginAttrs); */
    /// Unable to generate binding because `native type const mozilla::OriginAttributes unsupported`
    pub SetOriginAttributes: *const ::libc::c_void,

    /* [infallible] attribute boolean initialSecurityCheckDone; */
    pub GetInitialSecurityCheckDone: unsafe extern "system" fn (this: *const nsILoadInfo, aInitialSecurityCheckDone: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean initialSecurityCheckDone; */
    pub SetInitialSecurityCheckDone: unsafe extern "system" fn (this: *const nsILoadInfo, aInitialSecurityCheckDone: bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean loadTriggeredFromExternal; */
    pub GetLoadTriggeredFromExternal: unsafe extern "system" fn (this: *const nsILoadInfo, aLoadTriggeredFromExternal: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean loadTriggeredFromExternal; */
    pub SetLoadTriggeredFromExternal: unsafe extern "system" fn (this: *const nsILoadInfo, aLoadTriggeredFromExternal: bool) -> ::nserror::nsresult,

    /* [infallible,noscript] readonly attribute boolean serviceWorkerTaintingSynthesized; */
    pub GetServiceWorkerTaintingSynthesized: unsafe extern "system" fn (this: *const nsILoadInfo, aServiceWorkerTaintingSynthesized: *mut bool) -> ::nserror::nsresult,

    /* void appendRedirectHistoryEntry (in nsIRedirectHistoryEntry entry, in boolean isInternalRedirect); */
    pub AppendRedirectHistoryEntry: unsafe extern "system" fn (this: *const nsILoadInfo, entry: *const nsIRedirectHistoryEntry, isInternalRedirect: bool) -> ::nserror::nsresult,

    /* [implicit_jscontext] readonly attribute jsval redirectChainIncludingInternalRedirects; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetRedirectChainIncludingInternalRedirects: *const ::libc::c_void,

    /* [binaryname(RedirectChainIncludingInternalRedirects),noscript,nostdcall,notxpcom] nsIRedirectHistoryEntryArray binaryRedirectChainIncludingInternalRedirects (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub RedirectChainIncludingInternalRedirects: *const ::libc::c_void,

    /* [implicit_jscontext] readonly attribute jsval redirectChain; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetRedirectChain: *const ::libc::c_void,

    /* [binaryname(RedirectChain),noscript,nostdcall,notxpcom] nsIRedirectHistoryEntryArray binaryRedirectChain (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub RedirectChain: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] PrincipalArrayRef AncestorPrincipals (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub AncestorPrincipals: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] Uint64ArrayRef AncestorBrowsingContextIDs (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub AncestorBrowsingContextIDs: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] void setCorsPreflightInfo (in CStringArrayRef unsafeHeaders, in boolean forcePreflight); */
    /// Unable to generate binding because `native type const nsTArray<nsCString> unsupported`
    pub SetCorsPreflightInfo: *const ::libc::c_void,

    /* [binaryname(CorsUnsafeHeaders),noscript,nostdcall,notxpcom] CStringArrayRef corsUnsafeHeaders (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub CorsUnsafeHeaders: *const ::libc::c_void,

    /* [infallible] readonly attribute boolean forcePreflight; */
    pub GetForcePreflight: unsafe extern "system" fn (this: *const nsILoadInfo, aForcePreflight: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean isPreflight; */
    pub GetIsPreflight: unsafe extern "system" fn (this: *const nsILoadInfo, aIsPreflight: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute unsigned long tainting; */
    pub GetTainting: unsafe extern "system" fn (this: *const nsILoadInfo, aTainting: *mut u32) -> ::nserror::nsresult,

    /* void maybeIncreaseTainting (in unsigned long aTainting); */
    pub MaybeIncreaseTainting: unsafe extern "system" fn (this: *const nsILoadInfo, aTainting: u32) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean isTopLevelLoad; */
    pub GetIsTopLevelLoad: unsafe extern "system" fn (this: *const nsILoadInfo, aIsTopLevelLoad: *mut bool) -> ::nserror::nsresult,

    /* attribute nsIURI resultPrincipalURI; */
    pub GetResultPrincipalURI: unsafe extern "system" fn (this: *const nsILoadInfo, aResultPrincipalURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* attribute nsIURI resultPrincipalURI; */
    pub SetResultPrincipalURI: unsafe extern "system" fn (this: *const nsILoadInfo, aResultPrincipalURI: *const nsIURI) -> ::nserror::nsresult,

    /* [nostdcall,notxpcom] readonly attribute nsIPrincipal sandboxedLoadingPrincipal; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetSandboxedLoadingPrincipal: *const ::libc::c_void,

    /* [nostdcall,notxpcom] readonly attribute nsIPrincipal topLevelPrincipal; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetTopLevelPrincipal: *const ::libc::c_void,

    /* [nostdcall,notxpcom] readonly attribute nsIPrincipal topLevelStorageAreaPrincipal; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetTopLevelStorageAreaPrincipal: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] void SetClientInfo (in const_ClientInfoRef aClientInfo); */
    /// Unable to generate binding because `native type const mozilla::dom::ClientInfo unsupported`
    pub SetClientInfo: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] const_MaybeClientInfoRef GetClientInfo (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetClientInfo: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] void GiveReservedClientSource (in UniqueClientSourceMove aClientSource); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GiveReservedClientSource: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] UniqueClientSource TakeReservedClientSource (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub TakeReservedClientSource: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] void SetReservedClientInfo (in const_ClientInfoRef aClientInfo); */
    /// Unable to generate binding because `native type const mozilla::dom::ClientInfo unsupported`
    pub SetReservedClientInfo: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] void OverrideReservedClientInfoInParent (in const_ClientInfoRef aClientInfo); */
    /// Unable to generate binding because `native type const mozilla::dom::ClientInfo unsupported`
    pub OverrideReservedClientInfoInParent: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] const_MaybeClientInfoRef GetReservedClientInfo (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetReservedClientInfo: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] void SetInitialClientInfo (in const_ClientInfoRef aClientInfo); */
    /// Unable to generate binding because `native type const mozilla::dom::ClientInfo unsupported`
    pub SetInitialClientInfo: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] const_MaybeClientInfoRef GetInitialClientInfo (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetInitialClientInfo: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] void SetController (in const_ServiceWorkerDescriptorRef aServiceWorker); */
    /// Unable to generate binding because `native type const mozilla::dom::ServiceWorkerDescriptor unsupported`
    pub SetController: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] void ClearController (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub ClearController: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] const_MaybeServiceWorkerDescriptorRef GetController (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetController: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] void SetPerformanceStorage (in PerformanceStoragePtr aPerformanceStorage); */
    /// Unable to generate binding because `native type mozilla::dom::PerformanceStorage unsupported`
    pub SetPerformanceStorage: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] PerformanceStoragePtr GetPerformanceStorage (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetPerformanceStorage: *const ::libc::c_void,

    /* [nostdcall,notxpcom] CSPRef GetCsp (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetCsp: *const ::libc::c_void,

    /* [nostdcall,notxpcom] CSPRef GetPreloadCsp (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetPreloadCsp: *const ::libc::c_void,

    /* [nostdcall,notxpcom] CSPRef GetCspToInherit (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetCspToInherit: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] void SynthesizeServiceWorkerTainting (in LoadTainting aTainting); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub SynthesizeServiceWorkerTainting: *const ::libc::c_void,

    /* [infallible] attribute boolean documentHasUserInteracted; */
    pub GetDocumentHasUserInteracted: unsafe extern "system" fn (this: *const nsILoadInfo, aDocumentHasUserInteracted: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean documentHasUserInteracted; */
    pub SetDocumentHasUserInteracted: unsafe extern "system" fn (this: *const nsILoadInfo, aDocumentHasUserInteracted: bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean allowListFutureDocumentsCreatedFromThisRedirectChain; */
    pub GetAllowListFutureDocumentsCreatedFromThisRedirectChain: unsafe extern "system" fn (this: *const nsILoadInfo, aAllowListFutureDocumentsCreatedFromThisRedirectChain: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean allowListFutureDocumentsCreatedFromThisRedirectChain; */
    pub SetAllowListFutureDocumentsCreatedFromThisRedirectChain: unsafe extern "system" fn (this: *const nsILoadInfo, aAllowListFutureDocumentsCreatedFromThisRedirectChain: bool) -> ::nserror::nsresult,

    /* attribute AString cspNonce; */
    pub GetCspNonce: unsafe extern "system" fn (this: *const nsILoadInfo, aCspNonce: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString cspNonce; */
    pub SetCspNonce: unsafe extern "system" fn (this: *const nsILoadInfo, aCspNonce: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [infallible] attribute unsigned long requestBlockingReason; */
    pub GetRequestBlockingReason: unsafe extern "system" fn (this: *const nsILoadInfo, aRequestBlockingReason: *mut u32) -> ::nserror::nsresult,

    /* [infallible] attribute unsigned long requestBlockingReason; */
    pub SetRequestBlockingReason: unsafe extern "system" fn (this: *const nsILoadInfo, aRequestBlockingReason: u32) -> ::nserror::nsresult,

    /* attribute nsICSPEventListener cspEventListener; */
    pub GetCspEventListener: unsafe extern "system" fn (this: *const nsILoadInfo, aCspEventListener: *mut *const nsICSPEventListener) -> ::nserror::nsresult,

    /* attribute nsICSPEventListener cspEventListener; */
    pub SetCspEventListener: unsafe extern "system" fn (this: *const nsILoadInfo, aCspEventListener: *const nsICSPEventListener) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean isFromProcessingFrameAttributes; */
    pub GetIsFromProcessingFrameAttributes: unsafe extern "system" fn (this: *const nsILoadInfo, aIsFromProcessingFrameAttributes: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute nsILoadInfo_CrossOriginEmbedderPolicy loadingEmbedderPolicy; */
    pub GetLoadingEmbedderPolicy: unsafe extern "system" fn (this: *const nsILoadInfo, aLoadingEmbedderPolicy: *mut u8) -> ::nserror::nsresult,

    /* [infallible] attribute nsILoadInfo_CrossOriginEmbedderPolicy loadingEmbedderPolicy; */
    pub SetLoadingEmbedderPolicy: unsafe extern "system" fn (this: *const nsILoadInfo, aLoadingEmbedderPolicy:  u8) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsILoadInfo {
    /// ```text
    /// /**
    ///  * The LoadInfo object contains information about a network load, why it
    ///  * was started, and how we plan on using the resulting response.
    ///  * If a network request is redirected, the new channel will receive a new
    ///  * LoadInfo object. The new object will contain mostly the same
    ///  * information as the pre-redirect one, but updated as appropriate.
    ///  * For detailed information about what parts of LoadInfo are updated on
    ///  * redirect, see documentation on individual properties.
    ///  */
    /// /**
    ///    * The following five flags determine the security mode and hence what kind of
    ///    * security checks should be performed throughout the lifetime of the channel.
    ///    *
    ///    *    * SEC_REQUIRE_SAME_ORIGIN_INHERITS_SEC_CONTEXT
    ///    *    * SEC_REQUIRE_SAME_ORIGIN_DATA_IS_BLOCKED
    ///    *    * SEC_ALLOW_CROSS_ORIGIN_INHERITS_SEC_CONTEXT
    ///    *    * SEC_ALLOW_CROSS_ORIGIN_SEC_CONTEXT_IS_NULL
    ///    *    * SEC_REQUIRE_CORS_INHERITS_SEC_CONTEXT
    ///    *
    ///    * Exactly one of these flags are required to be set in order to allow
    ///    * the channel to perform the correct security checks (SOP, CORS, ...) and
    ///    * return the correct result principal. If none or more than one of these
    ///    * flags are set AsyncOpen will fail.
    ///    */
    /// /**
    ///     * Warning: Never use this flag when creating a new channel!
    ///     * Only use this flag if you have to create a temporary LoadInfo
    ///     * for performing an explicit nsIContentPolicy check, like e.g.
    ///     * when loading something from the cache that needs an explicit
    ///     * nsIContentPolicy check. In all other cases pick one of the
    ///     * security flags underneath.
    ///     */
    /// ```
    ///

    pub const SEC_ONLY_FOR_EXPLICIT_CONTENTSEC_CHECK: i64 = 0;


    pub const SEC_REQUIRE_SAME_ORIGIN_INHERITS_SEC_CONTEXT: i64 = 1;


    pub const SEC_REQUIRE_SAME_ORIGIN_DATA_IS_BLOCKED: i64 = 2;

    /// ```text
    /// /**
    ///    * Allow loads from other origins. Loads which inherit the principal should
    ///    * see the documentation for principalToInherit, which describes exactly what
    ///    * principal is inherited.
    ///    *
    ///    * Commonly used by plain <img>, <video>, <link rel=stylesheet> etc.
    ///    */
    /// ```
    ///

    pub const SEC_ALLOW_CROSS_ORIGIN_INHERITS_SEC_CONTEXT: i64 = 4;

    /// ```text
    /// /**
    ///    * Allow loads from other origins. Loads from data: will be allowed,
    ///    * but the resulting resource will get a null principal.
    ///    * Used in blink/webkit for <iframe>s. Likely also the mode
    ///    * that should be used by most Chrome code.
    ///    */
    /// ```
    ///

    pub const SEC_ALLOW_CROSS_ORIGIN_SEC_CONTEXT_IS_NULL: i64 = 8;

    /// ```text
    /// /**
    ///    * Allow loads from any origin, but require CORS for cross-origin loads.
    ///    * See the documentation for principalToInherit, which describes exactly what
    ///    * principal is inherited.
    ///    *
    ///    * Commonly used by <img crossorigin>, <video crossorigin>,
    ///    * XHR, fetch(), etc.
    ///    */
    /// ```
    ///

    pub const SEC_REQUIRE_CORS_INHERITS_SEC_CONTEXT: i64 = 16;

    /// ```text
    /// /**
    ///    * Choose cookie policy. The default policy is equivalent to "INCLUDE" for
    ///    * SEC_REQUIRE_SAME_ORIGIN_* and SEC_ALLOW_CROSS_ORIGIN_* modes, and
    ///    * equivalent to "SAME_ORIGIN" for SEC_REQUIRE_CORS_INHERITS_SEC_CONTEXT mode.
    ///    *
    ///    * This means that if you want to perform a CORS load with credentials, pass
    ///    * SEC_COOKIES_INCLUDE.
    ///    *
    ///    * Note that these flags are still subject to the user's cookie policies.
    ///    * For example, if the user is blocking 3rd party cookies, those cookies
    ///    * will be blocked no matter which of these flags are set.
    ///    */
    /// ```
    ///

    pub const SEC_COOKIES_DEFAULT: i64 = 0;


    pub const SEC_COOKIES_INCLUDE: i64 = 32;


    pub const SEC_COOKIES_SAME_ORIGIN: i64 = 64;


    pub const SEC_COOKIES_OMIT: i64 = 96;

    /// ```text
    /// /**
    ///    * Force inheriting of the principal.  See the documentation for
    ///    * principalToInherit, which describes exactly what principal is inherited.
    ///    *
    ///    * Setting this flag will cause GetChannelResultPrincipal to return the
    ///    * principal to be inherited as the channel principal.
    ///    *
    ///    * This will happen independently of the scheme of the URI that the
    ///    * channel is loading.
    ///    *
    ///    * So if the principal that gets inherited is "http://a.com/", and the channel
    ///    * is loading the URI "http://b.com/whatever", GetChannelResultPrincipal
    ///    * will return a principal from "http://a.com/".
    ///    *
    ///    * This flag can not be used together with SANDBOXED_ORIGIN sandbox flag.  If
    ///    * both are passed to the LoadInfo constructor then this flag will be dropped.
    ///    * If you need to know whether this flag would have been present but was dropped
    ///    * due to sandboxing, check for the forceInheritPrincipalDropped flag.
    ///    */
    /// ```
    ///

    pub const SEC_FORCE_INHERIT_PRINCIPAL: i64 = 128;

    /// ```text
    /// /**
    ///    * Inherit the Principal for about:blank.
    ///    */
    /// ```
    ///

    pub const SEC_ABOUT_BLANK_INHERITS: i64 = 512;

    /// ```text
    /// /**
    ///    * Allow access to chrome: packages that are content accessible.
    ///    */
    /// ```
    ///

    pub const SEC_ALLOW_CHROME: i64 = 1024;

    /// ```text
    /// /**
    ///    * Disallow access to javascript: uris.
    ///    */
    /// ```
    ///

    pub const SEC_DISALLOW_SCRIPT: i64 = 2048;

    /// ```text
    /// /**
    ///    * Don't follow redirects. Instead the redirect response is returned
    ///    * as a successful response for the channel.
    ///    *
    ///    * Redirects not initiated by a server response, i.e. REDIRECT_INTERNAL and
    ///    * REDIRECT_STS_UPGRADE, are still followed.
    ///    *
    ///    * Note: If this flag is set and the channel response is a redirect, then
    ///    * the response body might not be available.
    ///    * This can happen if the redirect was cached.
    ///    */
    /// ```
    ///

    pub const SEC_DONT_FOLLOW_REDIRECTS: i64 = 4096;

    /// ```text
    /// /**
    ///    * Load an error page, it should be one of following : about:neterror,
    ///    * about:certerror, about:blocked, about:tabcrashed or about:restartrequired.
    ///    */
    /// ```
    ///

    pub const SEC_LOAD_ERROR_PAGE: i64 = 8192;

    /// ```text
    /// /**
    ///    * Force inheriting of the principal, overruling any owner that might be set
    ///    * on the channel. (Please note that channel.owner is deprecated and will be
        ///    * removed within Bug 1286838).  See the documentation for principalToInherit,
    ///    * which describes exactly what principal is inherited.
    ///    *
    ///    * Setting this flag will cause GetChannelResultPrincipal to return the
    ///    * principal to be inherited as the channel principal.
    ///    *
    ///    * This will happen independently of the scheme of the URI that the
    ///    * channel is loading.
    ///    */
    /// ```
    ///

    pub const SEC_FORCE_INHERIT_PRINCIPAL_OVERRULE_OWNER: i64 = 16384;

    /// ```text
    /// /**
    ///    * (default) If this flag is set, it has not yet been determined if the
    ///    * HTTPS-Only mode will upgrade the request.
    ///    */
    /// ```
    ///

    pub const HTTPS_ONLY_UNINITIALIZED: i64 = 1;

    /// ```text
    /// /**
    ///    * Indicates that the request will get upgraded, and the HTTPS-Only
    ///    * StreamListener got registered.
    ///    */
    /// ```
    ///

    pub const HTTPS_ONLY_UPGRADED_LISTENER_NOT_REGISTERED: i64 = 2;

    /// ```text
    /// /**
    ///    * Indicates that this is the first time the request gets upgraded, and thus
    ///    * the HTTPS-Only StreamListener hasn't been registered yet. Even though there
    ///    * might be multiple channels per request that have to be upgraded (e.g.,
        ///    * because of redirects), the StreamListener only has to be attached to one
    ///    * channel.
    ///    */
    /// ```
    ///

    pub const HTTPS_ONLY_UPGRADED_LISTENER_REGISTERED: i64 = 4;

    /// ```text
    /// /**
    ///    * This flag can be manually set if the HTTPS-Only mode should exempt the
    ///    * request and not upgrade it. (e.g in the case of OCSP.
        ///    */
        /// ```
        ///

        pub const HTTPS_ONLY_EXEMPT: i64 = 8;

        /// ```text
        /// /**
        ///    * This flag can only ever be set on top-level loads. It indicates
        ///    * that the top-level https connection succeeded. This flag is mostly
        ///    * used to counter time-outs which allows to cancel the channel
        ///    * if the https load has not started.
        ///    */
        /// ```
        ///

        pub const HTTPS_ONLY_TOP_LEVEL_LOAD_IN_PROGRESS: i64 = 16;

        /// ```text
        /// /**
        ///    * This flag indicates that the request should not be logged to the
        ///    * console.
        ///    */
        /// ```
        ///

        pub const HTTPS_ONLY_DO_NOT_LOG_TO_CONSOLE: i64 = 32;

        /// ```text
        /// /**
        ///   * Constants reflecting the channel tainting.  These are mainly defined here
        ///   * for script.  Internal C++ code should use the enum defined in LoadTainting.h.
        ///   * See LoadTainting.h for documentation.
        ///   */
        /// ```
        ///

        pub const TAINTING_BASIC: i64 = 0;


        pub const TAINTING_CORS: i64 = 1;


        pub const TAINTING_OPAQUE: i64 = 2;

        /// ```text
        /// /**
        ///    * List of possible reasons the request associated with this load info
        ///    * may have been blocked, set by various content blocking checkers.
        ///    */
        /// ```
        ///

        pub const BLOCKING_REASON_NONE: i64 = 0;


        pub const BLOCKING_REASON_CORSDISABLED: i64 = 1001;


        pub const BLOCKING_REASON_CORSDIDNOTSUCCEED: i64 = 1002;


        pub const BLOCKING_REASON_CORSREQUESTNOTHTTP: i64 = 1003;


        pub const BLOCKING_REASON_CORSMULTIPLEALLOWORIGINNOTALLOWED: i64 = 1004;


        pub const BLOCKING_REASON_CORSMISSINGALLOWORIGIN: i64 = 1005;


        pub const BLOCKING_REASON_CORSNOTSUPPORTINGCREDENTIALS: i64 = 1006;


        pub const BLOCKING_REASON_CORSALLOWORIGINNOTMATCHINGORIGIN: i64 = 1007;


        pub const BLOCKING_REASON_CORSMISSINGALLOWCREDENTIALS: i64 = 1008;


        pub const BLOCKING_REASON_CORSORIGINHEADERNOTADDED: i64 = 1009;


        pub const BLOCKING_REASON_CORSEXTERNALREDIRECTNOTALLOWED: i64 = 1010;


        pub const BLOCKING_REASON_CORSPREFLIGHTDIDNOTSUCCEED: i64 = 1011;


        pub const BLOCKING_REASON_CORSINVALIDALLOWMETHOD: i64 = 1012;


        pub const BLOCKING_REASON_CORSMETHODNOTFOUND: i64 = 1013;


        pub const BLOCKING_REASON_CORSINVALIDALLOWHEADER: i64 = 1014;


        pub const BLOCKING_REASON_CORSMISSINGALLOWHEADERFROMPREFLIGHT: i64 = 1015;


        pub const BLOCKING_REASON_CLASSIFY_MALWARE_URI: i64 = 2001;


        pub const BLOCKING_REASON_CLASSIFY_PHISHING_URI: i64 = 2002;


        pub const BLOCKING_REASON_CLASSIFY_UNWANTED_URI: i64 = 2003;


        pub const BLOCKING_REASON_CLASSIFY_TRACKING_URI: i64 = 2004;


        pub const BLOCKING_REASON_CLASSIFY_BLOCKED_URI: i64 = 2005;


        pub const BLOCKING_REASON_CLASSIFY_HARMFUL_URI: i64 = 2006;


        pub const BLOCKING_REASON_CLASSIFY_CRYPTOMINING_URI: i64 = 2007;


        pub const BLOCKING_REASON_CLASSIFY_FINGERPRINTING_URI: i64 = 2008;


        pub const BLOCKING_REASON_CLASSIFY_SOCIALTRACKING_URI: i64 = 2009;


        pub const BLOCKING_REASON_MIXED_BLOCKED: i64 = 3001;


        pub const BLOCKING_REASON_CONTENT_POLICY_GENERAL: i64 = 4000;


        pub const BLOCKING_REASON_CONTENT_POLICY_NO_DATA_PROTOCOL: i64 = 4001;


        pub const BLOCKING_REASON_CONTENT_POLICY_WEBEXT: i64 = 4002;


        pub const BLOCKING_REASON_CONTENT_POLICY_CONTENT_BLOCKED: i64 = 4003;


        pub const BLOCKING_REASON_CONTENT_POLICY_DATA_DOCUMENT: i64 = 4004;


        pub const BLOCKING_REASON_CONTENT_POLICY_WEB_BROWSER: i64 = 4005;


        pub const BLOCKING_REASON_CONTENT_POLICY_PRELOAD: i64 = 4006;


        pub const BLOCKING_REASON_NOT_SAME_ORIGIN: i64 = 5000;


        pub const BLOCKING_REASON_EXTENSION_WEBREQUEST: i64 = 6000;

        /// ```text
        /// /**
        ///    * This is the principal of the network request's caller/requester where
        ///    * the resulting resource will be used. I.e. it is the principal which
        ///    * will get access to the result of the request. (Where "get access to"
            ///    * might simply mean "embed" depending on the type of resource that is
            ///    * loaded).
        ///    *
        ///    * For example for an image, it is the principal of the document where
        ///    * the image is rendered. For a stylesheet it is the principal of the
        ///    * document where the stylesheet will be applied.
        ///    *
        ///    * So if document at http://a.com/page.html loads an image from
        ///    * http://b.com/pic.jpg, then loadingPrincipal will be
        ///    * http://a.com/page.html.
        ///    *
        ///    * For <iframe> and <frame> loads, the LoadingPrincipal is the
        ///    * principal of the parent document. For top-level loads, the
        ///    * LoadingPrincipal is null. For all loads except top-level loads
        ///    * the LoadingPrincipal is never null.
        ///    *
        ///    * If the loadingPrincipal is the system principal, no security checks
        ///    * will be done at all. There will be no security checks on the initial
        ///    * load or any subsequent redirects. This means there will be no
        ///    * nsIContentPolicy checks or any CheckLoadURI checks. Because of
        ///    * this, never set the loadingPrincipal to the system principal when
        ///    * the URI to be loaded is controlled by a webpage.
        ///    * If the loadingPrincipal and triggeringPrincipal are both
        ///    * content principals, then we will always call into
        ///    * nsIContentPolicies and CheckLoadURI. The call to nsIContentPolicies
        ///    * and CheckLoadURI happen even if the URI to be loaded is same-origin
        ///    * with the loadingPrincipal or triggeringPrincipal.
        ///    */
        /// ```
        ///

        /// `readonly attribute nsIPrincipal loadingPrincipal;`
        #[inline]
        pub unsafe fn GetLoadingPrincipal(&self, aLoadingPrincipal: *mut *const nsIPrincipal) -> ::nserror::nsresult {
            ((*self.vtable).GetLoadingPrincipal)(self, aLoadingPrincipal)
        }


        /// ```text
        /// /**
        ///    * A C++-friendly version of triggeringPrincipal.
        ///    *
        ///    * This is a bit awkward because we can't use
        ///    * binaryname(GetLoadingPrincipal).
        ///    */
        /// ```
        ///

        /// `[noscript,nostdcall,notxpcom] nsIPrincipal virtualGetLoadingPrincipal ();`
        const _VirtualGetLoadingPrincipal: () = ();

        /// ```text
        /// /**
        ///    * This is the principal which caused the network load to start. I.e.
        ///    * this is the principal which provided the URL to be loaded. This is
        ///    * often the same as the LoadingPrincipal, but there are a few cases
        ///    * where that's not true.
        ///    *
        ///    * For example for loads into an <iframe>, the LoadingPrincipal is always
        ///    * the principal of the parent document. However the triggeringPrincipal
        ///    * is the principal of the document which provided the URL that the
        ///    * <iframe> is navigating to. This could be the previous document inside
        ///    * the <iframe> which set document.location. Or a document elsewhere in
        ///    * the frame tree which contained a <a target="..."> which targetted the
        ///    * <iframe>.
        ///    *
        ///    * If a stylesheet links to a sub-resource, like an @imported stylesheet,
        ///    * or a background image, then the triggeringPrincipal is the principal
        ///    * of the stylesheet, while the LoadingPrincipal is the principal of the
        ///    * document being styled.
        ///    *
        ///    * The triggeringPrincipal is never null.
        ///    *
        ///    * If the triggeringPrincipal is the system principal, no security checks
        ///    * will be done at all. There will be no security checks on the initial
        ///    * load or any subsequent redirects. This means there will be no
        ///    * nsIContentPolicy checks or any CheckLoadURI checks. Because of
        ///    * this, never set the triggeringPrincipal to the system principal when
        ///    * the URI to be loaded is controlled by a webpage.
        ///    * If the loadingPrincipal and triggeringPrincipal are both
        ///    * content principals, then we will always call into
        ///    * nsIContentPolicies and CheckLoadURI. The call to nsIContentPolicies
        ///    * and CheckLoadURI happen even if the URI to be loaded is same-origin
        ///    * with the loadingPrincipal or triggeringPrincipal.
        ///    */
        /// ```
        ///

        /// `readonly attribute nsIPrincipal triggeringPrincipal;`
        #[inline]
        pub unsafe fn GetTriggeringPrincipal(&self, aTriggeringPrincipal: *mut *const nsIPrincipal) -> ::nserror::nsresult {
            ((*self.vtable).GetTriggeringPrincipal)(self, aTriggeringPrincipal)
        }


        /// ```text
        /// /**
        ///    * A C++-friendly version of triggeringPrincipal.
        ///    */
        /// ```
        ///

        /// `[binaryname(TriggeringPrincipal),noscript,nostdcall,notxpcom] nsIPrincipal binaryTriggeringPrincipal ();`
        const _TriggeringPrincipal: () = ();

        /// ```text
        /// /**
        ///    * For non-document loads the principalToInherit is always null. For
        ///    * loads of type TYPE_DOCUMENT or TYPE_SUBDOCUMENT the principalToInherit
        ///    * might be null. If it's non null, then this is the principal that is
        ///    * inherited if a principal needs to be inherited. If the principalToInherit
        ///    * is null but the inherit flag is set, then the triggeringPrincipal is
        ///    * the principal that is inherited.
        ///    */
        /// ```
        ///

        /// `attribute nsIPrincipal principalToInherit;`
        #[inline]
        pub unsafe fn GetPrincipalToInherit(&self, aPrincipalToInherit: *mut *const nsIPrincipal) -> ::nserror::nsresult {
            ((*self.vtable).GetPrincipalToInherit)(self, aPrincipalToInherit)
        }


        /// ```text
        /// /**
        ///    * For non-document loads the principalToInherit is always null. For
        ///    * loads of type TYPE_DOCUMENT or TYPE_SUBDOCUMENT the principalToInherit
        ///    * might be null. If it's non null, then this is the principal that is
        ///    * inherited if a principal needs to be inherited. If the principalToInherit
        ///    * is null but the inherit flag is set, then the triggeringPrincipal is
        ///    * the principal that is inherited.
        ///    */
        /// ```
        ///

        /// `attribute nsIPrincipal principalToInherit;`
        #[inline]
        pub unsafe fn SetPrincipalToInherit(&self, aPrincipalToInherit: *const nsIPrincipal) -> ::nserror::nsresult {
            ((*self.vtable).SetPrincipalToInherit)(self, aPrincipalToInherit)
        }


        /// ```text
        /// /**
        ///    * A C++-friendly version of principalToInherit.
        ///    */
        /// ```
        ///

        /// `[binaryname(PrincipalToInherit),noscript,nostdcall,notxpcom] nsIPrincipal binaryPrincipalToInherit ();`
        const _PrincipalToInherit: () = ();

        /// ```text
        /// /**
        ///    * Finds the correct principal to inherit for the given channel, based on
        ///    * the values of PrincipalToInherit and TriggeringPrincipal.
        ///    */
        /// ```
        ///

        /// `[noscript,nostdcall,notxpcom] nsIPrincipal FindPrincipalToInherit (in nsIChannel aChannel);`
        const _FindPrincipalToInherit: () = ();

        /// ```text
        /// /**
        ///    * This is the ownerDocument of the LoadingNode. Unless the LoadingNode
        ///    * is a Document, in which case the LoadingDocument is the same as the
        ///    * LoadingNode.
        ///    *
        ///    * For top-level loads, and for loads originating from workers, the
        ///    * LoadingDocument is null. When the LoadingDocument is not null, the
        ///    * LoadingPrincipal is set to the principal of the LoadingDocument.
        ///    */
        /// ```
        ///

        /// `readonly attribute Document loadingDocument;`
        #[inline]
        pub unsafe fn GetLoadingDocument(&self, aLoadingDocument: *mut *const libc::c_void) -> ::nserror::nsresult {
            ((*self.vtable).GetLoadingDocument)(self, aLoadingDocument)
        }


        /// ```text
        /// /**
        ///    * A C++-friendly version of loadingDocument (loadingNode).
        ///    * This is the Node where the resulting resource will be used. I.e. it is
        ///    * the Node which will get access to the result of the request. (Where
            ///    * "get access to" might simply mean "embed" depending on the type of
            ///    * resource that is loaded).
        ///    *
        ///    * For example for an <img>/<video> it is the image/video element. For
        ///    * document loads inside <iframe> and <frame>s, the LoadingNode is the
        ///    * <iframe>/<frame> element. For an XMLHttpRequest, it is the Document
        ///    * which contained the JS which initiated the XHR. For a stylesheet, it
        ///    * is the Document that contains <link rel=stylesheet>.
        ///    *
        ///    * For loads triggered by the HTML pre-parser, the LoadingNode is the
        ///    * Document which is currently being parsed.
        ///    *
        ///    * For top-level loads, and for loads originating from workers, the
        ///    * LoadingNode is null. If the LoadingNode is non-null, then the
        ///    * LoadingPrincipal is the principal of the LoadingNode.
        ///    */
        /// ```
        ///

        /// `[binaryname(LoadingNode),noscript,nostdcall,notxpcom] nsINode binaryLoadingNode ();`
        const _LoadingNode: () = ();

        /// ```text
        /// /**
        ///    * A C++ friendly version of the loadingContext for toplevel loads.
        ///    * Most likely you want to query the ownerDocument or LoadingNode
        ///    * and not this context only available for TYPE_DOCUMENT loads.
        ///    * Please note that except for loads of TYPE_DOCUMENT, this
        ///    * ContextForTopLevelLoad will always return null.
        ///    */
        /// ```
        ///

        /// `[binaryname(ContextForTopLevelLoad),noscript,nostdcall,notxpcom] LoadContextRef binaryContextForTopLevelLoad ();`
        const _ContextForTopLevelLoad: () = ();

        /// ```text
        /// /**
        ///    * For all loads except loads of TYPE_DOCUMENT, the loadingContext
        ///    * simply returns the loadingNode. For loads of TYPE_DOCUMENT this
        ///    * will return the context available for top-level loads which
        ///    * do not have a loadingNode.
        ///    */
        /// ```
        ///

        /// `[binaryname(LoadingContextXPCOM)] readonly attribute nsISupports loadingContext;`
        #[inline]
        pub unsafe fn GetLoadingContextXPCOM(&self, aLoadingContext: *mut *const nsISupports) -> ::nserror::nsresult {
            ((*self.vtable).GetLoadingContextXPCOM)(self, aLoadingContext)
        }


        /// ```text
        /// /**
        ///    * A C++ friendly version of the loadingContext.
        ///    */
        /// ```
        ///

        /// `[binaryname(GetLoadingContext),noscript,nostdcall,notxpcom] LoadContextRef binaryGetLoadingContext ();`
        const _GetLoadingContext: () = ();

        /// ```text
        /// /**
        ///    * The securityFlags of that channel.
        ///    */
        /// ```
        ///

        /// `readonly attribute nsSecurityFlags securityFlags;`
        #[inline]
        pub unsafe fn GetSecurityFlags(&self, aSecurityFlags: *mut nsSecurityFlags) -> ::nserror::nsresult {
            ((*self.vtable).GetSecurityFlags)(self, aSecurityFlags)
        }


        /// ```text
        /// /**
        ///    * The sandboxFlags of that channel.
        ///    */
        /// ```
        ///

        /// `[infallible] readonly attribute unsigned long sandboxFlags;`
        #[inline]
        pub unsafe fn GetSandboxFlags(&self) -> u32 {
            let mut result = <u32 as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetSandboxFlags)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///   *   The TriggingSandboxFlags are the SandboxFlags of the entity
        ///   *   responsible for causing the load to occur.
        ///   */
        /// ```
        ///

        /// `[infallible] attribute unsigned long triggeringSandboxFlags;`
        #[inline]
        pub unsafe fn GetTriggeringSandboxFlags(&self) -> u32 {
            let mut result = <u32 as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetTriggeringSandboxFlags)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///   *   The TriggingSandboxFlags are the SandboxFlags of the entity
        ///   *   responsible for causing the load to occur.
        ///   */
        /// ```
        ///

        /// `[infallible] attribute unsigned long triggeringSandboxFlags;`
        #[inline]
        pub unsafe fn SetTriggeringSandboxFlags(&self, aTriggeringSandboxFlags: u32) -> ::nserror::nsresult {
            ((*self.vtable).SetTriggeringSandboxFlags)(self, aTriggeringSandboxFlags)
        }


        /// ```text
        /// /**
        ///    * Allows to query only the security mode bits from above.
        ///    */
        /// ```
        ///

        /// `[infallible] readonly attribute unsigned long securityMode;`
        #[inline]
        pub unsafe fn GetSecurityMode(&self) -> u32 {
            let mut result = <u32 as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetSecurityMode)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * This flag is used for any browsing context where we should not sniff
        ///    * the content type. E.g if an iframe has the XCTO nosniff header, then
        ///    * that flag is set to true so we skip content sniffing for that browsing
        ///    * context.
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean skipContentSniffing;`
        #[inline]
        pub unsafe fn GetSkipContentSniffing(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetSkipContentSniffing)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * This flag is used for any browsing context where we should not sniff
        ///    * the content type. E.g if an iframe has the XCTO nosniff header, then
        ///    * that flag is set to true so we skip content sniffing for that browsing
        ///    * context.
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean skipContentSniffing;`
        #[inline]
        pub unsafe fn SetSkipContentSniffing(&self, aSkipContentSniffing: bool) -> ::nserror::nsresult {
            ((*self.vtable).SetSkipContentSniffing)(self, aSkipContentSniffing)
        }


        /// ```text
        /// /**
        ///    * Upgrade state of HTTPS-Only Mode. The flag HTTPS_ONLY_EXEMPT can get
        ///    * set on requests that should be excempt from an upgrade.
        ///    */
        /// ```
        ///

        /// `[infallible] attribute unsigned long httpsOnlyStatus;`
        #[inline]
        pub unsafe fn GetHttpsOnlyStatus(&self) -> u32 {
            let mut result = <u32 as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetHttpsOnlyStatus)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * Upgrade state of HTTPS-Only Mode. The flag HTTPS_ONLY_EXEMPT can get
        ///    * set on requests that should be excempt from an upgrade.
        ///    */
        /// ```
        ///

        /// `[infallible] attribute unsigned long httpsOnlyStatus;`
        #[inline]
        pub unsafe fn SetHttpsOnlyStatus(&self, aHttpsOnlyStatus: u32) -> ::nserror::nsresult {
            ((*self.vtable).SetHttpsOnlyStatus)(self, aHttpsOnlyStatus)
        }


        /// ```text
        /// /**
        ///    * Returns true if at the time of the loadinfo construction the document
        ///    * that triggered this load has the bit hasValidTransientUserGestureActivation
        ///    * set or the load was triggered from External. (Mostly this bool is used
            ///    * in the context of Sec-Fetch-User.)
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean hasValidUserGestureActivation;`
        #[inline]
        pub unsafe fn GetHasValidUserGestureActivation(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetHasValidUserGestureActivation)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * Returns true if at the time of the loadinfo construction the document
        ///    * that triggered this load has the bit hasValidTransientUserGestureActivation
        ///    * set or the load was triggered from External. (Mostly this bool is used
            ///    * in the context of Sec-Fetch-User.)
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean hasValidUserGestureActivation;`
        #[inline]
        pub unsafe fn SetHasValidUserGestureActivation(&self, aHasValidUserGestureActivation: bool) -> ::nserror::nsresult {
            ((*self.vtable).SetHasValidUserGestureActivation)(self, aHasValidUserGestureActivation)
        }


        /// ```text
        /// /**
        ///    * We disallow the SystemPrincipal to initiate requests to
        ///    * the public web. This flag is to allow exceptions.
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean allowDeprecatedSystemRequests;`
        #[inline]
        pub unsafe fn GetAllowDeprecatedSystemRequests(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetAllowDeprecatedSystemRequests)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * We disallow the SystemPrincipal to initiate requests to
        ///    * the public web. This flag is to allow exceptions.
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean allowDeprecatedSystemRequests;`
        #[inline]
        pub unsafe fn SetAllowDeprecatedSystemRequests(&self, aAllowDeprecatedSystemRequests: bool) -> ::nserror::nsresult {
            ((*self.vtable).SetAllowDeprecatedSystemRequests)(self, aAllowDeprecatedSystemRequests)
        }


        /// ```text
        /// /**
        ///    * Only ever returns true if the loadinfo is of TYPE_SCRIPT and
        ///    * the script was created by the HTML parser.
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean parserCreatedScript;`
        #[inline]
        pub unsafe fn GetParserCreatedScript(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetParserCreatedScript)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * Only ever returns true if the loadinfo is of TYPE_SCRIPT and
        ///    * the script was created by the HTML parser.
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean parserCreatedScript;`
        #[inline]
        pub unsafe fn SetParserCreatedScript(&self, aParserCreatedScript: bool) -> ::nserror::nsresult {
            ((*self.vtable).SetParserCreatedScript)(self, aParserCreatedScript)
        }


        /// ```text
        /// /**
        ///    * True if this request is from DevTools.
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean isInDevToolsContext;`
        #[inline]
        pub unsafe fn GetIsInDevToolsContext(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetIsInDevToolsContext)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * True if this request is from DevTools.
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean isInDevToolsContext;`
        #[inline]
        pub unsafe fn SetIsInDevToolsContext(&self, aIsInDevToolsContext: bool) -> ::nserror::nsresult {
            ((*self.vtable).SetIsInDevToolsContext)(self, aIsInDevToolsContext)
        }


        /// ```text
        /// /**
        ///    * True if this request is embedded in a context that can't be third-party
        ///    * (i.e. an iframe embedded in a cross-origin parent window). If this is
        ///    * false, then this request may be third-party if it's a third-party to
        ///    * loadingPrincipal.
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean isInThirdPartyContext;`
        #[inline]
        pub unsafe fn GetIsInThirdPartyContext(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetIsInThirdPartyContext)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * True if this request is embedded in a context that can't be third-party
        ///    * (i.e. an iframe embedded in a cross-origin parent window). If this is
        ///    * false, then this request may be third-party if it's a third-party to
        ///    * loadingPrincipal.
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean isInThirdPartyContext;`
        #[inline]
        pub unsafe fn SetIsInThirdPartyContext(&self, aIsInThirdPartyContext: bool) -> ::nserror::nsresult {
            ((*self.vtable).SetIsInThirdPartyContext)(self, aIsInThirdPartyContext)
        }


        /// ```text
        /// /**
        ///    * True if this request is a third party in respect to the top-level window.
        ///    *
        ///    * Note that this doesn't consider the parent window. I.e. It will still
        ///    * return false even in the case that the parent is cross-origin but the
        ///    * top-level is same-origin.
        ///    *
        ///    * This value would be set during opening the channel in parent and propagate
        ///    * to the channel in the content.
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean isThirdPartyContextToTopWindow;`
        #[inline]
        pub unsafe fn GetIsThirdPartyContextToTopWindow(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetIsThirdPartyContextToTopWindow)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * True if this request is a third party in respect to the top-level window.
        ///    *
        ///    * Note that this doesn't consider the parent window. I.e. It will still
        ///    * return false even in the case that the parent is cross-origin but the
        ///    * top-level is same-origin.
        ///    *
        ///    * This value would be set during opening the channel in parent and propagate
        ///    * to the channel in the content.
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean isThirdPartyContextToTopWindow;`
        #[inline]
        pub unsafe fn SetIsThirdPartyContextToTopWindow(&self, aIsThirdPartyContextToTopWindow: bool) -> ::nserror::nsresult {
            ((*self.vtable).SetIsThirdPartyContextToTopWindow)(self, aIsThirdPartyContextToTopWindow)
        }


        /// ```text
        /// /**
        ///    * See the SEC_COOKIES_* flags above. This attribute will never return
        ///    * SEC_COOKIES_DEFAULT, but will instead return what the policy resolves to.
        ///    * I.e. SEC_COOKIES_SAME_ORIGIN for CORS mode, and SEC_COOKIES_INCLUDE
        ///    * otherwise.
        ///    */
        /// ```
        ///

        /// `[infallible] readonly attribute unsigned long cookiePolicy;`
        #[inline]
        pub unsafe fn GetCookiePolicy(&self) -> u32 {
            let mut result = <u32 as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetCookiePolicy)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * The cookie jar settings inherited from the top-level document's loadInfo.
        ///    * It cannot be null.
        ///    */
        /// ```
        ///

        /// `attribute nsICookieJarSettings cookieJarSettings;`
        #[inline]
        pub unsafe fn GetCookieJarSettings(&self, aCookieJarSettings: *mut*const nsICookieJarSettings) -> ::nserror::nsresult {
            ((*self.vtable).GetCookieJarSettings)(self, aCookieJarSettings)
        }


        /// ```text
        /// /**
        ///    * The cookie jar settings inherited from the top-level document's loadInfo.
        ///    * It cannot be null.
        ///    */
        /// ```
        ///

        /// `attribute nsICookieJarSettings cookieJarSettings;`
        #[inline]
        pub unsafe fn SetCookieJarSettings(&self, aCookieJarSettings: *const nsICookieJarSettings) -> ::nserror::nsresult {
            ((*self.vtable).SetCookieJarSettings)(self, aCookieJarSettings)
        }


        /// ```text
        /// /**
        ///    * True if the loading document has the storage permission. This value would
        ///    * be set during opening the channel.
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean hasStoragePermission;`
        #[inline]
        pub unsafe fn GetHasStoragePermission(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetHasStoragePermission)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * True if the loading document has the storage permission. This value would
        ///    * be set during opening the channel.
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean hasStoragePermission;`
        #[inline]
        pub unsafe fn SetHasStoragePermission(&self, aHasStoragePermission: bool) -> ::nserror::nsresult {
            ((*self.vtable).SetHasStoragePermission)(self, aHasStoragePermission)
        }


        /// ```text
        /// /**
        ///    * If forceInheritPrincipal is true, the data coming from the channel should
        ///    * inherit its principal, even when the data is loaded over http:// or another
        ///    * protocol that would normally use a URI-based principal.
        ///    *
        ///    * See the documentation for principalToInherit, which describes exactly what
        ///    * principal is inherited.
        ///    *
        ///    * This attribute will never be true when loadingSandboxed is true.
        ///    */
        /// ```
        ///

        /// `[infallible] readonly attribute boolean forceInheritPrincipal;`
        #[inline]
        pub unsafe fn GetForceInheritPrincipal(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetForceInheritPrincipal)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * If forceInheritPrincipalOverruleOwner is true, the data coming from the
        ///    * channel should inherit the principal, even when the data is loaded over
        ///    * http:// or another protocol that would normally use a URI-based principal
        ///    * and even if the channel's .owner is not null.  This last is the difference
        ///    * between forceInheritPrincipalOverruleOwner and forceInheritPrincipal: the
        ///    * latter does _not_ overrule the .owner setting.
        ///    *
        ///    * See the documentation for principalToInherit, which describes exactly what
        ///    * principal is inherited.
        ///    */
        /// ```
        ///

        /// `[infallible] readonly attribute boolean forceInheritPrincipalOverruleOwner;`
        #[inline]
        pub unsafe fn GetForceInheritPrincipalOverruleOwner(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetForceInheritPrincipalOverruleOwner)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * If loadingSandboxed is true, the data coming from the channel is
        ///    * being loaded sandboxed, so it should have a nonce origin and
        ///    * hence should use a NullPrincipal.
        ///    */
        /// ```
        ///

        /// `[infallible] readonly attribute boolean loadingSandboxed;`
        #[inline]
        pub unsafe fn GetLoadingSandboxed(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetLoadingSandboxed)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * If aboutBlankInherits is true, then about:blank should inherit
        ///    * the principal.
        ///    */
        /// ```
        ///

        /// `[infallible] readonly attribute boolean aboutBlankInherits;`
        #[inline]
        pub unsafe fn GetAboutBlankInherits(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetAboutBlankInherits)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * If allowChrome is true, then use nsIScriptSecurityManager::ALLOW_CHROME
        ///    * when calling CheckLoadURIWithPrincipal().
        ///    */
        /// ```
        ///

        /// `[infallible] readonly attribute boolean allowChrome;`
        #[inline]
        pub unsafe fn GetAllowChrome(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetAllowChrome)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * If disallowScript is true, then use nsIScriptSecurityManager::DISALLOW_SCRIPT
        ///    * when calling CheckLoadURIWithPrincipal().
        ///    */
        /// ```
        ///

        /// `[infallible] readonly attribute boolean disallowScript;`
        #[inline]
        pub unsafe fn GetDisallowScript(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetDisallowScript)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * Returns true if SEC_DONT_FOLLOW_REDIRECTS is set.
        ///    */
        /// ```
        ///

        /// `[infallible] readonly attribute boolean dontFollowRedirects;`
        #[inline]
        pub unsafe fn GetDontFollowRedirects(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetDontFollowRedirects)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * Returns true if SEC_LOAD_ERROR_PAGE is set.
        ///    */
        /// ```
        ///

        /// `[infallible] readonly attribute boolean loadErrorPage;`
        #[inline]
        pub unsafe fn GetLoadErrorPage(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetLoadErrorPage)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * True if the load was initiated by a form request.
        ///    * This is important to know to handle the CSP directive navigate-to.
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean isFormSubmission;`
        #[inline]
        pub unsafe fn GetIsFormSubmission(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetIsFormSubmission)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * True if the load was initiated by a form request.
        ///    * This is important to know to handle the CSP directive navigate-to.
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean isFormSubmission;`
        #[inline]
        pub unsafe fn SetIsFormSubmission(&self, aIsFormSubmission: bool) -> ::nserror::nsresult {
            ((*self.vtable).SetIsFormSubmission)(self, aIsFormSubmission)
        }


        /// ```text
        /// /**
        ///    * The external contentPolicyType of the channel, used for security checks
        ///    * like Mixed Content Blocking and Content Security Policy.
        ///    *
        ///    * Specifically, content policy types with _INTERNAL_ in their name will
        ///    * never get returned from this attribute.
        ///    */
        /// ```
        ///

        /// `readonly attribute nsContentPolicyType externalContentPolicyType;`
        #[inline]
        pub unsafe fn GetExternalContentPolicyType(&self, aExternalContentPolicyType: *mut nsContentPolicyType) -> ::nserror::nsresult {
            ((*self.vtable).GetExternalContentPolicyType)(self, aExternalContentPolicyType)
        }


        /// ```text
        /// /**
        ///    * CSP uses this parameter to send or not CSP violation events.
        ///    * Default value: true.
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean sendCSPViolationEvents;`
        #[inline]
        pub unsafe fn GetSendCSPViolationEvents(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetSendCSPViolationEvents)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * CSP uses this parameter to send or not CSP violation events.
        ///    * Default value: true.
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean sendCSPViolationEvents;`
        #[inline]
        pub unsafe fn SetSendCSPViolationEvents(&self, aSendCSPViolationEvents: bool) -> ::nserror::nsresult {
            ((*self.vtable).SetSendCSPViolationEvents)(self, aSendCSPViolationEvents)
        }


        /// ```text
        /// /**
        ///    * The internal contentPolicyType of the channel, used for constructing
        ///    * RequestContext values when creating a fetch event for an intercepted
        ///    * channel.
        ///    *
        ///    * This should not be used for the purposes of security checks, since
        ///    * the content policy implementations cannot be expected to deal with
        ///    * _INTERNAL_ values.  Please use the contentPolicyType attribute above
        ///    * for that purpose.
        ///    */
        /// ```
        ///

        /// `[binaryname(InternalContentPolicyType),noscript,nostdcall,notxpcom] nsContentPolicyType binaryInternalContentPolicyType ();`
        const _InternalContentPolicyType: () = ();


        /// `readonly attribute nsContentPolicyType internalContentPolicyType;`
        #[inline]
        pub unsafe fn GetInternalContentPolicyType(&self, aInternalContentPolicyType: *mut nsContentPolicyType) -> ::nserror::nsresult {
            ((*self.vtable).GetInternalContentPolicyType)(self, aInternalContentPolicyType)
        }


        /// ```text
        /// /**
        ///    * Returns true if document or any of the documents ancestors
        ///    * up to the toplevel document make use of the CSP directive
        ///    * 'block-all-mixed-content'.
        ///    *
        ///    * Warning: If the loadingDocument is null, then the
        ///    * blockAllMixedContent is false.
        ///    */
        /// ```
        ///

        /// `[infallible] readonly attribute boolean blockAllMixedContent;`
        #[inline]
        pub unsafe fn GetBlockAllMixedContent(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetBlockAllMixedContent)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * Returns true if document or any of the documents ancestors
        ///    * up to the toplevel document make use of the CSP directive
        ///    * 'upgrade-insecure-requests'.
        ///    *
        ///    * Warning: If the loadingDocument is null, then the
        ///    * upgradeInsecureRequests is false.
        ///    */
        /// ```
        ///

        /// `[infallible] readonly attribute boolean upgradeInsecureRequests;`
        #[inline]
        pub unsafe fn GetUpgradeInsecureRequests(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetUpgradeInsecureRequests)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * Returns true if the the page is https and the content is upgradable from http
        ///    * requires 'security.mixed_content.upgrade_display_content' pref to be true.
        ///    * Currently this only upgrades display content but might be expanded to other loads.
        ///    * This is very similar in implementation to upgradeInsecureRequests but browser set.
        ///    */
        /// ```
        ///

        /// `[infallible] readonly attribute boolean browserUpgradeInsecureRequests;`
        #[inline]
        pub unsafe fn GetBrowserUpgradeInsecureRequests(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetBrowserUpgradeInsecureRequests)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * Returns true if the display content was or will get upgraded from http to https.
        ///    * Requires 'security.mixed_content.upgrade_display_content' pref to be true.
        ///    * Flag is set purely to collect telemetry.
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean browserDidUpgradeInsecureRequests;`
        #[inline]
        pub unsafe fn GetBrowserDidUpgradeInsecureRequests(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetBrowserDidUpgradeInsecureRequests)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * Returns true if the display content was or will get upgraded from http to https.
        ///    * Requires 'security.mixed_content.upgrade_display_content' pref to be true.
        ///    * Flag is set purely to collect telemetry.
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean browserDidUpgradeInsecureRequests;`
        #[inline]
        pub unsafe fn SetBrowserDidUpgradeInsecureRequests(&self, aBrowserDidUpgradeInsecureRequests: bool) -> ::nserror::nsresult {
            ((*self.vtable).SetBrowserDidUpgradeInsecureRequests)(self, aBrowserDidUpgradeInsecureRequests)
        }


        /// ```text
        /// /**
        ///    * Returns true if the the page is https and the content is upgradable from http
        ///    * requires 'security.mixed_content.upgrade_display_content' pref to be false.
        ///    * See browserUpgradeInsecureRequests for more details, this only happens
        ///    * when *not* upgrading purely for telemetry.
        ///    */
        /// ```
        ///

        /// `[infallible] readonly attribute boolean browserWouldUpgradeInsecureRequests;`
        #[inline]
        pub unsafe fn GetBrowserWouldUpgradeInsecureRequests(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetBrowserWouldUpgradeInsecureRequests)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * If true, toplevel data: URI navigation is allowed
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean forceAllowDataURI;`
        #[inline]
        pub unsafe fn GetForceAllowDataURI(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetForceAllowDataURI)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * If true, toplevel data: URI navigation is allowed
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean forceAllowDataURI;`
        #[inline]
        pub unsafe fn SetForceAllowDataURI(&self, aForceAllowDataURI: bool) -> ::nserror::nsresult {
            ((*self.vtable).SetForceAllowDataURI)(self, aForceAllowDataURI)
        }


        /// ```text
        /// /**
        ///    * If true, insecure redirects to a data: URI are allowed.
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean allowInsecureRedirectToDataURI;`
        #[inline]
        pub unsafe fn GetAllowInsecureRedirectToDataURI(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetAllowInsecureRedirectToDataURI)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * If true, insecure redirects to a data: URI are allowed.
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean allowInsecureRedirectToDataURI;`
        #[inline]
        pub unsafe fn SetAllowInsecureRedirectToDataURI(&self, aAllowInsecureRedirectToDataURI: bool) -> ::nserror::nsresult {
            ((*self.vtable).SetAllowInsecureRedirectToDataURI)(self, aAllowInsecureRedirectToDataURI)
        }


        /// ```text
        /// /**
        ///    * If true, CORS checks will be skipped.
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean bypassCORSChecks;`
        #[inline]
        pub unsafe fn GetBypassCORSChecks(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetBypassCORSChecks)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * If true, CORS checks will be skipped.
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean bypassCORSChecks;`
        #[inline]
        pub unsafe fn SetBypassCORSChecks(&self, aBypassCORSChecks: bool) -> ::nserror::nsresult {
            ((*self.vtable).SetBypassCORSChecks)(self, aBypassCORSChecks)
        }


        /// ```text
        /// /**
        ///    * If true, the content policy security check is excluded from web requests.
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean skipContentPolicyCheckForWebRequest;`
        #[inline]
        pub unsafe fn GetSkipContentPolicyCheckForWebRequest(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetSkipContentPolicyCheckForWebRequest)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * If true, the content policy security check is excluded from web requests.
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean skipContentPolicyCheckForWebRequest;`
        #[inline]
        pub unsafe fn SetSkipContentPolicyCheckForWebRequest(&self, aSkipContentPolicyCheckForWebRequest: bool) -> ::nserror::nsresult {
            ((*self.vtable).SetSkipContentPolicyCheckForWebRequest)(self, aSkipContentPolicyCheckForWebRequest)
        }


        /// ```text
        /// /**
        ///    * If true, this is the load of a frame's original src attribute
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean originalFrameSrcLoad;`
        #[inline]
        pub unsafe fn GetOriginalFrameSrcLoad(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetOriginalFrameSrcLoad)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * If true, this is the load of a frame's original src attribute
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean originalFrameSrcLoad;`
        #[inline]
        pub unsafe fn SetOriginalFrameSrcLoad(&self, aOriginalFrameSrcLoad: bool) -> ::nserror::nsresult {
            ((*self.vtable).SetOriginalFrameSrcLoad)(self, aOriginalFrameSrcLoad)
        }


        /// ```text
        /// /**
        ///    * The SEC_FORCE_INHERIT_PRINCIPAL flag may be dropped when a load info
        ///    * object is created.  Specifically, it will be dropped if the SANDBOXED_ORIGIN
        ///    * sandbox flag is also present.  This flag is set if SEC_FORCE_INHERIT_PRINCIPAL
        ///    * was dropped.
        ///    */
        /// ```
        ///

        /// `[infallible] readonly attribute boolean forceInheritPrincipalDropped;`
        #[inline]
        pub unsafe fn GetForceInheritPrincipalDropped(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetForceInheritPrincipalDropped)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * This is the inner window ID of the window in which the element being
        ///    * loaded lives.
        ///    *
        ///    * Note that this window ID can be 0 if the window is not
        ///    * available.
        ///    */
        /// ```
        ///

        /// `[infallible] readonly attribute unsigned long long innerWindowID;`
        #[inline]
        pub unsafe fn GetInnerWindowID(&self) -> u64 {
            let mut result = <u64 as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetInnerWindowID)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * The BrowsingContext performing the load for this nsILoadInfo object.
        ///    */
        /// ```
        ///

        /// `[infallible] readonly attribute unsigned long long browsingContextID;`
        #[inline]
        pub unsafe fn GetBrowsingContextID(&self) -> u64 {
            let mut result = <u64 as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetBrowsingContextID)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }



        /// `[infallible] readonly attribute BrowsingContext browsingContext;`
        #[inline]
        pub unsafe fn GetBrowsingContext(&self, aBrowsingContext: *mut *const libc::c_void) -> ::nserror::nsresult {
            ((*self.vtable).GetBrowsingContext)(self, aBrowsingContext)
        }


        /// ```text
        /// /**
        ///    * Only when the element being loaded is <frame src="foo.html">
        ///    * (or, more generally, if the element QIs to nsFrameLoaderOwner),
        ///    * the frameBrowsingContext is the browsing context containing the
        ///    * foo.html document.
        ///    *
        ///    * Note: For other cases, frameBrowsingContextID is 0.
        ///    */
        /// ```
        ///

        /// `[infallible] readonly attribute unsigned long long frameBrowsingContextID;`
        #[inline]
        pub unsafe fn GetFrameBrowsingContextID(&self) -> u64 {
            let mut result = <u64 as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetFrameBrowsingContextID)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }



        /// `[infallible] readonly attribute BrowsingContext frameBrowsingContext;`
        #[inline]
        pub unsafe fn GetFrameBrowsingContext(&self, aFrameBrowsingContext: *mut *const libc::c_void) -> ::nserror::nsresult {
            ((*self.vtable).GetFrameBrowsingContext)(self, aFrameBrowsingContext)
        }


        /// ```text
        /// /**
        ///    * If the element being loaded is a nsFrameLoaderOwner,
        ///    * `targetBrowsingContext` is the Browsing Context which will contain the
        ///    * loading document (see `frameBrowsingContext`). Otherwise, it is the
        ///    * Browsing Context performing the load (see `browsingContext`).
        ///    */
        /// ```
        ///

        /// `[infallible] readonly attribute unsigned long long targetBrowsingContextID;`
        #[inline]
        pub unsafe fn GetTargetBrowsingContextID(&self) -> u64 {
            let mut result = <u64 as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetTargetBrowsingContextID)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }



        /// `[infallible] readonly attribute BrowsingContext targetBrowsingContext;`
        #[inline]
        pub unsafe fn GetTargetBrowsingContext(&self, aTargetBrowsingContext: *mut *const libc::c_void) -> ::nserror::nsresult {
            ((*self.vtable).GetTargetBrowsingContext)(self, aTargetBrowsingContext)
        }


        /// ```text
        /// /**
        ///    * Resets the PrincipalToInherit to a freshly created NullPrincipal
        ///    * which inherits the origin attributes from the loadInfo.
        ///    *
        ///    * WARNING: Please only use that function if you know exactly what
        ///    * you are doing!!!
        ///    */
        /// ```
        ///

        /// `void resetPrincipalToInheritToNullPrincipal ();`
        #[inline]
        pub unsafe fn ResetPrincipalToInheritToNullPrincipal(&self, ) -> ::nserror::nsresult {
            ((*self.vtable).ResetPrincipalToInheritToNullPrincipal)(self, )
        }


        /// ```text
        /// /**
        ///    * Customized OriginAttributes within LoadInfo to allow overwriting of the
        ///    * default originAttributes from the loadingPrincipal.
        ///    *
        ///    * In chrome side, originAttributes.privateBrowsingId will always be 0 even if
        ///    * the usePrivateBrowsing is true, because chrome docshell won't set
        ///    * privateBrowsingId on origin attributes (See bug 1278664). This is to make
        ///    * sure nsILoadInfo and nsILoadContext have the same origin attributes.
        ///    */
        /// ```
        ///

        /// `[binaryname(ScriptableOriginAttributes),implicit_jscontext] attribute jsval originAttributes;`
        const _GetScriptableOriginAttributes: () = ();

        /// ```text
        /// /**
        ///    * Customized OriginAttributes within LoadInfo to allow overwriting of the
        ///    * default originAttributes from the loadingPrincipal.
        ///    *
        ///    * In chrome side, originAttributes.privateBrowsingId will always be 0 even if
        ///    * the usePrivateBrowsing is true, because chrome docshell won't set
        ///    * privateBrowsingId on origin attributes (See bug 1278664). This is to make
        ///    * sure nsILoadInfo and nsILoadContext have the same origin attributes.
        ///    */
        /// ```
        ///

        /// `[binaryname(ScriptableOriginAttributes),implicit_jscontext] attribute jsval originAttributes;`
        const _SetScriptableOriginAttributes: () = ();


        /// `[binaryname(GetOriginAttributes),noscript,nostdcall] OriginAttributes binaryGetOriginAttributes ();`
        const _GetOriginAttributes: () = ();


        /// `[binaryname(SetOriginAttributes),noscript,nostdcall] void binarySetOriginAttributes (in const_OriginAttributesRef aOriginAttrs);`
        const _SetOriginAttributes: () = ();

        /// ```text
        /// /**
        ///    * Whenever a channel is evaluated by the ContentSecurityManager
        ///    * the first time, we set this flag to true to indicate that
        ///    * subsequent calls of AsyncOpen() do not have to enforce all
        ///    * security checks again. E.g., after a redirect there is no
        ///    * need to set up CORS again. We need this separate flag
        ///    * because the redirectChain might also contain internal
        ///    * redirects which might pollute the redirectChain so we can't
        ///    * rely on the size of the redirectChain-array to query whether
        ///    * a channel got redirected or not.
        ///    *
        ///    * Please note, once the flag is set to true it must remain true
        ///    * throughout the lifetime of the channel. Trying to set it
        ///    * to anything else than true will be discarded.
        ///    *
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean initialSecurityCheckDone;`
        #[inline]
        pub unsafe fn GetInitialSecurityCheckDone(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetInitialSecurityCheckDone)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * Whenever a channel is evaluated by the ContentSecurityManager
        ///    * the first time, we set this flag to true to indicate that
        ///    * subsequent calls of AsyncOpen() do not have to enforce all
        ///    * security checks again. E.g., after a redirect there is no
        ///    * need to set up CORS again. We need this separate flag
        ///    * because the redirectChain might also contain internal
        ///    * redirects which might pollute the redirectChain so we can't
        ///    * rely on the size of the redirectChain-array to query whether
        ///    * a channel got redirected or not.
        ///    *
        ///    * Please note, once the flag is set to true it must remain true
        ///    * throughout the lifetime of the channel. Trying to set it
        ///    * to anything else than true will be discarded.
        ///    *
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean initialSecurityCheckDone;`
        #[inline]
        pub unsafe fn SetInitialSecurityCheckDone(&self, aInitialSecurityCheckDone: bool) -> ::nserror::nsresult {
            ((*self.vtable).SetInitialSecurityCheckDone)(self, aInitialSecurityCheckDone)
        }


        /// ```text
        /// /**
        ///    * Returns true if the load was triggered from an external application
        ///    * (e.g. Thunderbird). Please note that this flag will only ever be true
        ///    * if the load is of TYPE_DOCUMENT.
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean loadTriggeredFromExternal;`
        #[inline]
        pub unsafe fn GetLoadTriggeredFromExternal(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetLoadTriggeredFromExternal)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * Returns true if the load was triggered from an external application
        ///    * (e.g. Thunderbird). Please note that this flag will only ever be true
        ///    * if the load is of TYPE_DOCUMENT.
        ///    */
        /// ```
        ///

        /// `[infallible] attribute boolean loadTriggeredFromExternal;`
        #[inline]
        pub unsafe fn SetLoadTriggeredFromExternal(&self, aLoadTriggeredFromExternal: bool) -> ::nserror::nsresult {
            ((*self.vtable).SetLoadTriggeredFromExternal)(self, aLoadTriggeredFromExternal)
        }


        /// ```text
        /// /**
        ///    * True if the tainting has been set by the service worker.
        ///    */
        /// ```
        ///

        /// `[infallible,noscript] readonly attribute boolean serviceWorkerTaintingSynthesized;`
        #[inline]
        pub unsafe fn GetServiceWorkerTaintingSynthesized(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetServiceWorkerTaintingSynthesized)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * Whenever a channel gets redirected, append the redirect history entry of
        ///    * the channel which contains principal referrer and remote address [before
            ///    * the channels got redirected] to the loadinfo, so that at every point this
        ///    * array provides us information about all the redirects this channel went
        ///    * through.
        ///    * @param entry, the nsIRedirectHistoryEntry before the channel
        ///    *         got redirected.
        ///    * @param aIsInternalRedirect should be true if the channel is going
        ///    *        through an internal redirect, otherwise false.
        ///    */
        /// ```
        ///

        /// `void appendRedirectHistoryEntry (in nsIRedirectHistoryEntry entry, in boolean isInternalRedirect);`
        #[inline]
        pub unsafe fn AppendRedirectHistoryEntry(&self, entry: *const nsIRedirectHistoryEntry, isInternalRedirect: bool) -> ::nserror::nsresult {
            ((*self.vtable).AppendRedirectHistoryEntry)(self, entry, isInternalRedirect)
        }


        /// ```text
        /// /**
        ///    * An array of nsIRedirectHistoryEntry which stores redirects associated
        ///    * with this channel. This array is filled whether or not the channel has
        ///    * ever been opened. The last element of the array is associated with the
        ///    * most recent redirect. Please note, that this array *includes* internal
        ///    * redirects.
        ///    */
        /// ```
        ///

        /// `[implicit_jscontext] readonly attribute jsval redirectChainIncludingInternalRedirects;`
        const _GetRedirectChainIncludingInternalRedirects: () = ();

        /// ```text
        /// /**
        ///    * A C++-friendly version of redirectChain.
        ///    * Please note that this array has the same lifetime as the
        ///    * loadInfo object - use with caution!
        ///    */
        /// ```
        ///

        /// `[binaryname(RedirectChainIncludingInternalRedirects),noscript,nostdcall,notxpcom] nsIRedirectHistoryEntryArray binaryRedirectChainIncludingInternalRedirects ();`
        const _RedirectChainIncludingInternalRedirects: () = ();

        /// ```text
        /// /**
        ///    * Same as RedirectChain but does *not* include internal redirects.
        ///    */
        /// ```
        ///

        /// `[implicit_jscontext] readonly attribute jsval redirectChain;`
        const _GetRedirectChain: () = ();

        /// ```text
        /// /**
        ///    * A C++-friendly version of redirectChain.
        ///    * Please note that this array has the same lifetime as the
        ///    * loadInfo object - use with caution!
        ///    */
        /// ```
        ///

        /// `[binaryname(RedirectChain),noscript,nostdcall,notxpcom] nsIRedirectHistoryEntryArray binaryRedirectChain ();`
        const _RedirectChain: () = ();

        /// ```text
        /// /**
        ///    * This array is only filled out when we are in the parent process and we are
        ///    * creating a loadInfo object or deserializing LoadInfoArgs into LoadInfo,
        ///    * as we ever only need in the parent process.
        ///    *
        ///    * The array is meant to be a list of principals of the documents that the
        ///    * browsing context, corresponding to this loadInfo object, is "nested through" in
        ///    * the sense of
        ///    * <https://html.spec.whatwg.org/multipage/browsers.html#browsing-context-nested-through>.
        ///    * Note that the array does not include the principal corresponding to the frame
        ///    * loading this request. The closest ancestor is at index zero and the top level
        ///    * ancestor is at the last index.
        ///    *
        ///    * If this is a toplevel content browsing context (i.e. toplevel document in spec
            ///    * terms), the list is empty.
        ///    *
        ///    * Otherwise the array is a list for the document we're nested through (again in
            ///    * the spec sense), with the principal of that document prepended. The
        ///    * ancestorPrincipals[0] entry for an iframe load will be the principal of the
        ///    * iframe element's owner document. The ancestorPrincipals[0] entry for an image
        ///    * loaded in an iframe will be the principal of the iframe element's owner
        ///    * document. This matches the ordering specified for Location.ancestorOrigins.
        ///    *
        ///    * Please note that this array has the same lifetime as the loadInfo object - use
        ///    * with caution!
        ///    */
        /// ```
        ///

        /// `[noscript,nostdcall,notxpcom] PrincipalArrayRef AncestorPrincipals ();`
        const _AncestorPrincipals: () = ();

        /// ```text
        /// /**
        ///    * An array of BrowsingContext IDs which correspond to nsILoadInfo::AncestorPrincipals
        ///    * above.  AncestorBrowsingContextIDs[0] is the BrowsingContext ID of the frame
        ///    * associated with the principal at ancestorPrincipals[0], and so forth.
        ///    *
        ///    * Please note that this array has the same lifetime as the
        ///    * loadInfo object - use with caution!
        ///    */
        /// ```
        ///

        /// `[noscript,nostdcall,notxpcom] Uint64ArrayRef AncestorBrowsingContextIDs ();`
        const _AncestorBrowsingContextIDs: () = ();

        /// ```text
        /// /**
        ///    * Sets the list of unsafe headers according to CORS spec, as well as
        ///    * potentially forces a preflight.
        ///    * Note that you do not need to set the Content-Type header. That will be
        ///    * automatically detected as needed.
        ///    *
        ///    * Only call this function when using the SEC_REQUIRE_CORS_INHERITS_SEC_CONTEXT mode.
        ///    */
        /// ```
        ///

        /// `[noscript,nostdcall,notxpcom] void setCorsPreflightInfo (in CStringArrayRef unsafeHeaders, in boolean forcePreflight);`
        const _SetCorsPreflightInfo: () = ();

        /// ```text
        /// /**
        ///    * A C++-friendly getter for the list of cors-unsafe headers.
        ///    * Please note that this array has the same lifetime as the
        ///    * loadInfo object - use with caution!
        ///    */
        /// ```
        ///

        /// `[binaryname(CorsUnsafeHeaders),noscript,nostdcall,notxpcom] CStringArrayRef corsUnsafeHeaders ();`
        const _CorsUnsafeHeaders: () = ();

        /// ```text
        /// /**
        ///    * Returns value set through setCorsPreflightInfo.
        ///    */
        /// ```
        ///

        /// `[infallible] readonly attribute boolean forcePreflight;`
        #[inline]
        pub unsafe fn GetForcePreflight(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetForcePreflight)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * A C++ friendly getter for the forcePreflight flag.
        ///    */
        /// ```
        ///

        /// `[infallible] readonly attribute boolean isPreflight;`
        #[inline]
        pub unsafe fn GetIsPreflight(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetIsPreflight)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * Determine the associated channel's current tainting.  Note, this can
        ///    * change due to a service worker intercept, so it should be checked after
        ///    * OnStartRequest() fires.
        ///    */
        /// ```
        ///

        /// `readonly attribute unsigned long tainting;`
        #[inline]
        pub unsafe fn GetTainting(&self, aTainting: *mut u32) -> ::nserror::nsresult {
            ((*self.vtable).GetTainting)(self, aTainting)
        }


        /// ```text
        /// /**
        ///    * Note a new tainting level and possibly increase the current tainting
        ///    * to match.  If the tainting level is already greater than the given
        ///    * value, then there is no effect.  It is not possible to reduce the tainting
        ///    * level on an existing channel/loadinfo.
        ///    */
        /// ```
        ///

        /// `void maybeIncreaseTainting (in unsigned long aTainting);`
        #[inline]
        pub unsafe fn MaybeIncreaseTainting(&self, aTainting: u32) -> ::nserror::nsresult {
            ((*self.vtable).MaybeIncreaseTainting)(self, aTainting)
        }


        /// ```text
        /// /**
        ///    * Various helper code to provide more convenient C++ access to the tainting
        ///    * attribute and maybeIncreaseTainting().
        ///    */
        /// /**
        ///    * Returns true if this load is for top level document.
        ///    * Note that the load for a sub-frame's document will return false here.
        ///    */
        /// ```
        ///

        /// `[infallible] readonly attribute boolean isTopLevelLoad;`
        #[inline]
        pub unsafe fn GetIsTopLevelLoad(&self) -> bool {
            let mut result = <bool as ::std::default::Default>::default();
            let _rv = ((*self.vtable).GetIsTopLevelLoad)(self, &mut result);
            debug_assert!(_rv.succeeded());
            result
        }


        /// ```text
        /// /**
        ///    * If this is non-null, this property represents two things: (1) the
        ///    * URI to be used for the principal if the channel with this loadinfo
        ///    * gets a principal based on URI and (2) the URI to use for a document
        ///    * created from the channel with this loadinfo.
        ///    */
        /// ```
        ///

        /// `attribute nsIURI resultPrincipalURI;`
        #[inline]
        pub unsafe fn GetResultPrincipalURI(&self, aResultPrincipalURI: *mut*const nsIURI) -> ::nserror::nsresult {
            ((*self.vtable).GetResultPrincipalURI)(self, aResultPrincipalURI)
        }


        /// ```text
        /// /**
        ///    * If this is non-null, this property represents two things: (1) the
        ///    * URI to be used for the principal if the channel with this loadinfo
        ///    * gets a principal based on URI and (2) the URI to use for a document
        ///    * created from the channel with this loadinfo.
        ///    */
        /// ```
        ///

        /// `attribute nsIURI resultPrincipalURI;`
        #[inline]
        pub unsafe fn SetResultPrincipalURI(&self, aResultPrincipalURI: *const nsIURI) -> ::nserror::nsresult {
            ((*self.vtable).SetResultPrincipalURI)(self, aResultPrincipalURI)
        }


        /// ```text
        /// /**
        ///    * Returns the null principal of the resulting resource if the SANDBOXED_ORIGIN
        ///    * flag is set.  Otherwise returns null.  This is used by
        ///    * GetChannelResultPrincipal() to ensure that the same null principal object
        ///    * is returned every time.
        ///    */
        /// ```
        ///

        /// `[nostdcall,notxpcom] readonly attribute nsIPrincipal sandboxedLoadingPrincipal;`
        const _GetSandboxedLoadingPrincipal: () = ();

        /// ```text
        /// /**
        ///    * Return the top-level principal, which is the principal of the top-level
        ///    * window.
        ///    */
        /// ```
        ///

        /// `[nostdcall,notxpcom] readonly attribute nsIPrincipal topLevelPrincipal;`
        const _GetTopLevelPrincipal: () = ();

        /// ```text
        /// /**
        ///    * Return the top-level storage area principal, which is the principal of
        ///    * the top-level window if it's not a 3rd party context, non tracking
        ///    * resource.
        ///    */
        /// ```
        ///

        /// `[nostdcall,notxpcom] readonly attribute nsIPrincipal topLevelStorageAreaPrincipal;`
        const _GetTopLevelStorageAreaPrincipal: () = ();

        /// ```text
        /// /**
        ///    * Note which client (i.e. global) initiated this network request.  All
        ///    * nsGlobalWindow and WorkerPrivate can be converted to a ClientInfo to
        ///    * be set here.  While this is being added to support service worker
        ///    * FetchEvent, it can also be used to communicate other information about
        ///    * the source global context in the future.
        ///    */
        /// ```
        ///

        /// `[noscript,nostdcall,notxpcom] void SetClientInfo (in const_ClientInfoRef aClientInfo);`
        const _SetClientInfo: () = ();

        /// ```text
        /// /**
        ///    * Get the ClientInfo for the global that initiated the network request,
        ///    * if it has been set.
        ///    */
        /// ```
        ///

        /// `[noscript,nostdcall,notxpcom] const_MaybeClientInfoRef GetClientInfo ();`
        const _GetClientInfo: () = ();

        /// ```text
        /// /**
        ///    * Give a pre-allocated ClientSource to the channel LoadInfo.  This is
        ///    * intended to be used by docshell when loading windows without an
        ///    * initial about:blank document.  The docshell will allocate the ClientSource
        ///    * to represent the client that will be created as a result of the navigation
        ///    * network request.  If the channel succeeds and remains same-origin, then
        ///    * the result nsGlobalWindow will take ownership of the reserved ClientSource.
        ///    *
        ///    * This method is also called when a cross-origin redirect occurs.  A new
        ///    * ClientSource with a different UUID must be created in this case.
        ///    *
        ///    * This method automatically calls SetReservedClientInfo() with the
        ///    * ClientSource::Info().
        ///    */
        /// ```
        ///

        /// `[noscript,nostdcall,notxpcom] void GiveReservedClientSource (in UniqueClientSourceMove aClientSource);`
        const _GiveReservedClientSource: () = ();

        /// ```text
        /// /**
        ///    * This method takes ownership of the reserved ClientSource previously
        ///    * provided in GiveReservedClientSource().  It may return nullptr if the
        ///    * nsILoadInfo does not own a ClientSource object.
        ///    */
        /// ```
        ///

        /// `[noscript,nostdcall,notxpcom] UniqueClientSource TakeReservedClientSource ();`
        const _TakeReservedClientSource: () = ();

        /// ```text
        /// /**
        ///    * Note the reserved client that be created if this non-subresource
        ///    * network request succeeds.  Depending on the type of client this
        ///    * may be called directly or indirectly via GiveReservedClientSource().
        ///    * For example, web workers do not call give their ClientSource to
        ///    * the nsILoadInfo, but must still call this method to indicate the
        ///    * reserved client for their main script load.
        ///    */
        /// ```
        ///

        /// `[noscript,nostdcall,notxpcom] void SetReservedClientInfo (in const_ClientInfoRef aClientInfo);`
        const _SetReservedClientInfo: () = ();

        /// ```text
        /// /**
        ///    * This will clear any existing reserved or initial client and override
        ///    * it with the given reserved client.  This is similar to calling
        ///    * TakeReservedClientSource() and then GiveReservedClientSource() with
        ///    * a new client as ClientChannelHelper does.  This method is needed,
        ///    * though, to perform this operation in the parent process where
        ///    * the LoadInfo does not have direct access to a ClientSource.
        ///    *
        ///    * If in doubt, do not call this method.  Its really only needed for
        ///    * a specific redirect case where the child has created a new client on
        ///    * redirect and we need to override the parent side's reserved client
        ///    * to match.
        ///    */
        /// ```
        ///

        /// `[noscript,nostdcall,notxpcom] void OverrideReservedClientInfoInParent (in const_ClientInfoRef aClientInfo);`
        const _OverrideReservedClientInfoInParent: () = ();

        /// ```text
        /// /**
        ///    * Return the reserved ClientInfo for this load, if one has been set.
        ///    */
        /// ```
        ///

        /// `[noscript,nostdcall,notxpcom] const_MaybeClientInfoRef GetReservedClientInfo ();`
        const _GetReservedClientInfo: () = ();

        /// ```text
        /// /**
        ///    * Note that this non-subresource network request will result in
        ///    * re-using an existing "initial" active client.  This mainly only
        ///    * happens when an initial about:blank document is replaced with
        ///    * a real load in a window.  In these cases we need to track this
        ///    * initial client so that we may report its existence in a FetchEvent.
        ///    *
        ///    * Note, an nsILoadInfo may only have a reserved client or an
        ///    * initial client.  It should never have both.
        ///    */
        /// ```
        ///

        /// `[noscript,nostdcall,notxpcom] void SetInitialClientInfo (in const_ClientInfoRef aClientInfo);`
        const _SetInitialClientInfo: () = ();

        /// ```text
        /// /**
        ///    * Return the initial ClientInfo for this load, if one has been set.
        ///    */
        /// ```
        ///

        /// `[noscript,nostdcall,notxpcom] const_MaybeClientInfoRef GetInitialClientInfo ();`
        const _GetInitialClientInfo: () = ();

        /// ```text
        /// /**
        ///    * Note that this network request should be controlled by a service worker.
        ///    * For non-subresource requests this may be set during the load when
        ///    * the first service worker interception occurs.  For subresource requests
        ///    * it may be set by the source client if its already controlled by a
        ///    * service worker.
        ///    */
        /// ```
        ///

        /// `[noscript,nostdcall,notxpcom] void SetController (in const_ServiceWorkerDescriptorRef aServiceWorker);`
        const _SetController: () = ();

        /// ```text
        /// /**
        ///    * Clear the service worker controller for this channel.  This should only
        ///    * be used for window navigation redirects.  By default we want to keep
        ///    * the controller in all other cases.
        ///    */
        /// ```
        ///

        /// `[noscript,nostdcall,notxpcom] void ClearController ();`
        const _ClearController: () = ();

        /// ```text
        /// /**
        ///    * Get the service worker controlling this network request, if one has
        ///    * been set.
        ///    */
        /// ```
        ///

        /// `[noscript,nostdcall,notxpcom] const_MaybeServiceWorkerDescriptorRef GetController ();`
        const _GetController: () = ();

        /// ```text
        /// /**
        ///    * Set a custom performance storage. This is meant to be executed only for
        ///    * workers. If a PerformanceStorage is not set, the loadingDocument->Window
        ///    * Performance object will be returned instead.
        ///    */
        /// ```
        ///

        /// `[noscript,nostdcall,notxpcom] void SetPerformanceStorage (in PerformanceStoragePtr aPerformanceStorage);`
        const _SetPerformanceStorage: () = ();

        /// ```text
        /// /**
        ///    * Get the custom PerformanceStorage if set by SetPerformanceStorage.
        ///    * Otherwise the loadingDocument->Window Performance object will be returned
        ///    * instead if all the following conditions are met:
        ///    * - the triggeringPrincipal is the same as the loadingDocument's principal.
        ///    * - if the external content policy type is TYPE_SUBDOCUMENT then loading
        ///    *   wasn't caused by processing the attributes of the browsing context
        ///    *   container.
        ///    */
        /// ```
        ///

        /// `[noscript,nostdcall,notxpcom] PerformanceStoragePtr GetPerformanceStorage ();`
        const _GetPerformanceStorage: () = ();

        /// ```text
        /// /**
        ///    * Returns the CSP (or Preload CSP for preloads) which should be enforced
        ///    * when fetching the resource this loadinfo belongs to.
        ///    *
        ///    * a) Non-navigations:
    ///    * For non-navigation loads, GetCSP() returns what the spec refers to as the
    ///    * "request's client's global object's CSP list". In practice, if this is the
    ///    * loadinfo of a subresource load (e.g an image load), then GetCSP() or
    ///    * GetPreloadCSP() returns the CSP of the document which embeds the image.
    ///    * The returned CSP includes any policy delivered through the HTTP header or
    ///    * also through the meta tag (modulo the difference for preloads, e.g. image
        ///    * preloads have to query GetPreloadCsp() because at the time of preloading
        ///    * we are not entirely sure if the Meta CSP will be applied to the document
        ///    * in the end or not). Please note that GetCSPToInherit() called on a
    ///    * loadinfo for any non-navigation always returns null.
    ///    *
    ///    * b) Navigations:
///    *   * Top-level loads:
///    *     For top-level loads (navigations) GetCSP() will return null, unless
///    *     the navigation is started by a WebExtension, in which case it will
///    *     return the CSP of the webextension, if any.
///    *     If you need to query the CSP that potentially should apply to the
///    *     new top-level load, you have to query GetCspToInherit(), which is
///    *     the CSP of the request's client's global object, just like GetCsp()
///    *     is for non-navigation requests.
///    *
///    *   * Iframe-loads:
///    *     For iframe-loads (navigations) GetCSP() will return the CSP of the
///    *     parent document, unless the navigation is started by a WebExtension,
///    *     in which case it will return the CSP of the webextension, if any.
///    *
///    * If you need to query the CSP that should potentially be inherited
///    * into the new document, you have to query GetCSPToInherit().
///    *
///    * TODO Bug 1557114:
///    * After evaluating what CSP to use for frame navigations we should
///    * update the above documentation to match the outcome of Bug 1557114.
///    */
/// ```
///

/// `[nostdcall,notxpcom] CSPRef GetCsp ();`
const _GetCsp: () = ();


/// `[nostdcall,notxpcom] CSPRef GetPreloadCsp ();`
const _GetPreloadCsp: () = ();


/// `[nostdcall,notxpcom] CSPRef GetCspToInherit ();`
const _GetCspToInherit: () = ();

/// ```text
/// /**
///     * The service worker and fetch specifications require returning the
///     * exact tainting level of the Response passed to FetchEvent.respondWith().
///     * This method allows us to override the tainting level in that case.
///     *
///     * NOTE: This should not be used outside of service worker code! Use
///     *       nsILoadInfo::MaybeIncreaseTainting() instead.
///    */
/// ```
///

/// `[noscript,nostdcall,notxpcom] void SynthesizeServiceWorkerTainting (in LoadTainting aTainting);`
const _SynthesizeServiceWorkerTainting: () = ();

/// ```text
/// /**
///     * The top-level document has been user-interacted.
///     */
/// ```
///

/// `[infallible] attribute boolean documentHasUserInteracted;`
#[inline]
pub unsafe fn GetDocumentHasUserInteracted(&self) -> bool {
    let mut result = <bool as ::std::default::Default>::default();
    let _rv = ((*self.vtable).GetDocumentHasUserInteracted)(self, &mut result);
    debug_assert!(_rv.succeeded());
    result
}


/// ```text
/// /**
///     * The top-level document has been user-interacted.
///     */
/// ```
///

/// `[infallible] attribute boolean documentHasUserInteracted;`
#[inline]
pub unsafe fn SetDocumentHasUserInteracted(&self, aDocumentHasUserInteracted: bool) -> ::nserror::nsresult {
    ((*self.vtable).SetDocumentHasUserInteracted)(self, aDocumentHasUserInteracted)
}


/// ```text
/// /**
///     * During a top-level document channel redirect from tracking to
///     * non-tracking resources, our anti-tracking heuristic, grants the storage
///     * access permission for a short amount of seconds (See
    ///     * privacy.restrict3rdpartystorage.expiration_redirect pref).
///     * We use this flag to remember this decision even if this channel is part
///     * of a chain of redirects.
///     */
/// ```
///

/// `[infallible] attribute boolean allowListFutureDocumentsCreatedFromThisRedirectChain;`
#[inline]
pub unsafe fn GetAllowListFutureDocumentsCreatedFromThisRedirectChain(&self) -> bool {
    let mut result = <bool as ::std::default::Default>::default();
    let _rv = ((*self.vtable).GetAllowListFutureDocumentsCreatedFromThisRedirectChain)(self, &mut result);
    debug_assert!(_rv.succeeded());
    result
}


/// ```text
/// /**
///     * During a top-level document channel redirect from tracking to
///     * non-tracking resources, our anti-tracking heuristic, grants the storage
///     * access permission for a short amount of seconds (See
    ///     * privacy.restrict3rdpartystorage.expiration_redirect pref).
///     * We use this flag to remember this decision even if this channel is part
///     * of a chain of redirects.
///     */
/// ```
///

/// `[infallible] attribute boolean allowListFutureDocumentsCreatedFromThisRedirectChain;`
#[inline]
pub unsafe fn SetAllowListFutureDocumentsCreatedFromThisRedirectChain(&self, aAllowListFutureDocumentsCreatedFromThisRedirectChain: bool) -> ::nserror::nsresult {
    ((*self.vtable).SetAllowListFutureDocumentsCreatedFromThisRedirectChain)(self, aAllowListFutureDocumentsCreatedFromThisRedirectChain)
}


/// ```text
/// /**
///    * A snapshot of the nonce at load start time which is used for CSP
///    * checks and only set for:
///    *  * TYPE_SCRIPT and
///    *  * TYPE_STYLESHEET
///    */
/// ```
///

/// `attribute AString cspNonce;`
#[inline]
pub unsafe fn GetCspNonce(&self, aCspNonce: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
    ((*self.vtable).GetCspNonce)(self, aCspNonce)
}


/// ```text
/// /**
///    * A snapshot of the nonce at load start time which is used for CSP
///    * checks and only set for:
///    *  * TYPE_SCRIPT and
///    *  * TYPE_STYLESHEET
///    */
/// ```
///

/// `attribute AString cspNonce;`
#[inline]
pub unsafe fn SetCspNonce(&self, aCspNonce: *const ::nsstring::nsAString) -> ::nserror::nsresult {
    ((*self.vtable).SetCspNonce)(self, aCspNonce)
}


/// ```text
/// /**
///    * If the request associated with this load info was blocked by some of
///    * our content or load blockers, the reason can be found here.
///    * Note that setting this attribute has NO EFFECT on blocking the request.
///    * This attribute is only informative!
///    *
///    * By default the value is '0' - NONE.
///    * Each write rewrites the last value.
///    * Can be accessed only on a single thread.
///    */
/// ```
///

/// `[infallible] attribute unsigned long requestBlockingReason;`
#[inline]
pub unsafe fn GetRequestBlockingReason(&self) -> u32 {
    let mut result = <u32 as ::std::default::Default>::default();
    let _rv = ((*self.vtable).GetRequestBlockingReason)(self, &mut result);
    debug_assert!(_rv.succeeded());
    result
}


/// ```text
/// /**
///    * If the request associated with this load info was blocked by some of
///    * our content or load blockers, the reason can be found here.
///    * Note that setting this attribute has NO EFFECT on blocking the request.
///    * This attribute is only informative!
///    *
///    * By default the value is '0' - NONE.
///    * Each write rewrites the last value.
///    * Can be accessed only on a single thread.
///    */
/// ```
///

/// `[infallible] attribute unsigned long requestBlockingReason;`
#[inline]
pub unsafe fn SetRequestBlockingReason(&self, aRequestBlockingReason: u32) -> ::nserror::nsresult {
    ((*self.vtable).SetRequestBlockingReason)(self, aRequestBlockingReason)
}


/// ```text
/// /**
///     * The object in charged to receive CSP violation events. It can be null.
///     * This attribute will be merged into the CSP object eventually.
///     * See bug 1500908.
///     */
/// ```
///

/// `attribute nsICSPEventListener cspEventListener;`
#[inline]
pub unsafe fn GetCspEventListener(&self, aCspEventListener: *mut *const nsICSPEventListener) -> ::nserror::nsresult {
    ((*self.vtable).GetCspEventListener)(self, aCspEventListener)
}


/// ```text
/// /**
///     * The object in charged to receive CSP violation events. It can be null.
///     * This attribute will be merged into the CSP object eventually.
///     * See bug 1500908.
///     */
/// ```
///

/// `attribute nsICSPEventListener cspEventListener;`
#[inline]
pub unsafe fn SetCspEventListener(&self, aCspEventListener: *const nsICSPEventListener) -> ::nserror::nsresult {
    ((*self.vtable).SetCspEventListener)(self, aCspEventListener)
}


/// ```text
/// /**
///    * This attribute will be true if this is a load triggered by
///    * https://html.spec.whatwg.org/multipage/iframe-embed-object.html#process-the-iframe-attributes
///    * or https://html.spec.whatwg.org/multipage/obsolete.html#process-the-frame-attributes
///    */
/// ```
///

/// `[infallible] readonly attribute boolean isFromProcessingFrameAttributes;`
#[inline]
pub unsafe fn GetIsFromProcessingFrameAttributes(&self) -> bool {
    let mut result = <bool as ::std::default::Default>::default();
    let _rv = ((*self.vtable).GetIsFromProcessingFrameAttributes)(self, &mut result);
    debug_assert!(_rv.succeeded());
    result
}


/// ```text
/// /**
///    * This attribute is the loading context's cross origin embedder policy.
///    * The value is initialized with corresponding WindowContext which get by
///    * innerWindowIID in the nsILoadInfo.
///    * It also could be set by workers when fetch is called under
///    * the workers' scope.
///    */
/// ```
///

/// `[infallible] attribute nsILoadInfo_CrossOriginEmbedderPolicy loadingEmbedderPolicy;`
#[inline]
pub unsafe fn GetLoadingEmbedderPolicy(&self, aLoadingEmbedderPolicy: *mut u8) -> ::nserror::nsresult {
    ((*self.vtable).GetLoadingEmbedderPolicy)(self, aLoadingEmbedderPolicy)
}


/// ```text
/// /**
///    * This attribute is the loading context's cross origin embedder policy.
///    * The value is initialized with corresponding WindowContext which get by
///    * innerWindowIID in the nsILoadInfo.
///    * It also could be set by workers when fetch is called under
///    * the workers' scope.
///    */
/// ```
///

/// `[infallible] attribute nsILoadInfo_CrossOriginEmbedderPolicy loadingEmbedderPolicy;`
#[inline]
pub unsafe fn SetLoadingEmbedderPolicy(&self, aLoadingEmbedderPolicy:  u8) -> ::nserror::nsresult {
    ((*self.vtable).SetLoadingEmbedderPolicy)(self, aLoadingEmbedderPolicy)
}


}


