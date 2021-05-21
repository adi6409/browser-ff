//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/caps/nsIScriptSecurityManager.idl
//


/// `interface nsIScriptSecurityManager : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIScriptSecurityManager {
    vtable: *const nsIScriptSecurityManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIScriptSecurityManager.
unsafe impl XpCom for nsIScriptSecurityManager {
    const IID: nsIID = nsID(0x51daad87, 0x3a0c, 0x44cc,
        [0xb6, 0x20, 0x73, 0x56, 0x80, 0x1c, 0x90, 0x22]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIScriptSecurityManager {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIScriptSecurityManager.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIScriptSecurityManagerCoerce {
    /// Cheaply cast a value of this type from a `nsIScriptSecurityManager`.
    fn coerce_from(v: &nsIScriptSecurityManager) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIScriptSecurityManagerCoerce for nsIScriptSecurityManager {
    #[inline]
    fn coerce_from(v: &nsIScriptSecurityManager) -> &Self {
        v
    }
}

impl nsIScriptSecurityManager {
    /// Cast this `nsIScriptSecurityManager` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIScriptSecurityManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIScriptSecurityManager {
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
impl<T: nsISupportsCoerce> nsIScriptSecurityManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIScriptSecurityManager) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIScriptSecurityManager
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIScriptSecurityManagerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [noscript] void canCreateWrapper (in JSContextPtr aJSContext, in nsIIDRef aIID, in nsISupports aObj, in nsIClassInfo aClassInfo); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub CanCreateWrapper: *const ::libc::c_void,

    /* [noscript] void canCreateInstance (in JSContextPtr aJSContext, in nsCIDRef aCID); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub CanCreateInstance: *const ::libc::c_void,

    /* [noscript] void canGetService (in JSContextPtr aJSContext, in nsCIDRef aCID); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub CanGetService: *const ::libc::c_void,

    /* [noscript] void checkLoadURIFromScript (in JSContextPtr cx, in nsIURI uri); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub CheckLoadURIFromScript: *const ::libc::c_void,

    /* [binaryname(CheckLoadURIWithPrincipal)] void checkLoadURIWithPrincipalXPCOM (in nsIPrincipal aPrincipal, in nsIURI uri, in unsigned long flags, [optional] in unsigned long long innerWindowID); */
    pub CheckLoadURIWithPrincipal: unsafe extern "system" fn (this: *const nsIScriptSecurityManager, aPrincipal: *const nsIPrincipal, uri: *const nsIURI, flags: u32, innerWindowID: u64) -> ::nserror::nsresult,

    /* [binaryname(CheckLoadURIWithPrincipalFromJS),implicit_jscontext] void checkLoadURIWithPrincipal (in nsIPrincipal aPrincipal, in nsIURI uri, [optional] in unsigned long flags, [optional] in unsigned long long innerWindowID); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub CheckLoadURIWithPrincipalFromJS: *const ::libc::c_void,

    /* [binaryname(CheckLoadURIStrWithPrincipal)] void checkLoadURIStrWithPrincipalXPCOM (in nsIPrincipal aPrincipal, in AUTF8String uri, in unsigned long flags); */
    pub CheckLoadURIStrWithPrincipal: unsafe extern "system" fn (this: *const nsIScriptSecurityManager, aPrincipal: *const nsIPrincipal, uri: *const ::nsstring::nsACString, flags: u32) -> ::nserror::nsresult,

    /* [binaryname(CheckLoadURIStrWithPrincipalFromJS),implicit_jscontext] void checkLoadURIStrWithPrincipal (in nsIPrincipal aPrincipal, in AUTF8String uri, [optional] in unsigned long flags); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub CheckLoadURIStrWithPrincipalFromJS: *const ::libc::c_void,

    /* bool inFileURIAllowlist (in nsIURI aUri); */
    pub InFileURIAllowlist: unsafe extern "system" fn (this: *const nsIScriptSecurityManager, aUri: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult,

    /* nsIPrincipal getSystemPrincipal (); */
    pub GetSystemPrincipal: unsafe extern "system" fn (this: *const nsIScriptSecurityManager, _retval: *mut *const nsIPrincipal) -> ::nserror::nsresult,

    /* nsIPrincipal getLoadContextContentPrincipal (in nsIURI uri, in nsILoadContext loadContext); */
    pub GetLoadContextContentPrincipal: unsafe extern "system" fn (this: *const nsIScriptSecurityManager, uri: *const nsIURI, loadContext: *const nsILoadContext, _retval: *mut *const nsIPrincipal) -> ::nserror::nsresult,

    /* nsIPrincipal getDocShellContentPrincipal (in nsIURI uri, in nsIDocShell docShell); */
    pub GetDocShellContentPrincipal: unsafe extern "system" fn (this: *const nsIScriptSecurityManager, uri: *const nsIURI, docShell: *const nsIDocShell, _retval: *mut *const nsIPrincipal) -> ::nserror::nsresult,

    /* [implicit_jscontext] nsIPrincipal principalWithOA (in nsIPrincipal principal, in jsval originAttributes); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub PrincipalWithOA: *const ::libc::c_void,

    /* [implicit_jscontext] nsIPrincipal createContentPrincipal (in nsIURI uri, in jsval originAttributes); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub CreateContentPrincipal: *const ::libc::c_void,

    /* nsIPrincipal createContentPrincipalFromOrigin (in ACString origin); */
    pub CreateContentPrincipalFromOrigin: unsafe extern "system" fn (this: *const nsIScriptSecurityManager, origin: *const ::nsstring::nsACString, _retval: *mut *const nsIPrincipal) -> ::nserror::nsresult,

    /* ACString principalToJSON (in nsIPrincipal principal); */
    pub PrincipalToJSON: unsafe extern "system" fn (this: *const nsIScriptSecurityManager, principal: *const nsIPrincipal, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* nsIPrincipal JSONToPrincipal (in ACString json); */
    pub JSONToPrincipal: unsafe extern "system" fn (this: *const nsIScriptSecurityManager, json: *const ::nsstring::nsACString, _retval: *mut *const nsIPrincipal) -> ::nserror::nsresult,

    /* [implicit_jscontext] nsIPrincipal createNullPrincipal (in jsval originAttributes); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub CreateNullPrincipal: *const ::libc::c_void,

    /* void checkSameOriginURI (in nsIURI aSourceURI, in nsIURI aTargetURI, in boolean reportError, in boolean fromPrivateWindow); */
    pub CheckSameOriginURI: unsafe extern "system" fn (this: *const nsIScriptSecurityManager, aSourceURI: *const nsIURI, aTargetURI: *const nsIURI, reportError: bool, fromPrivateWindow: bool) -> ::nserror::nsresult,

    /* nsIPrincipal getChannelResultPrincipal (in nsIChannel aChannel); */
    pub GetChannelResultPrincipal: unsafe extern "system" fn (this: *const nsIScriptSecurityManager, aChannel: *const nsIChannel, _retval: *mut *const nsIPrincipal) -> ::nserror::nsresult,

    /* nsIPrincipal getChannelResultStoragePrincipal (in nsIChannel aChannel); */
    pub GetChannelResultStoragePrincipal: unsafe extern "system" fn (this: *const nsIScriptSecurityManager, aChannel: *const nsIChannel, _retval: *mut *const nsIPrincipal) -> ::nserror::nsresult,

    /* void getChannelResultPrincipals (in nsIChannel aChannel, out nsIPrincipal aPrincipal, out nsIPrincipal aPartitionedPrincipal); */
    pub GetChannelResultPrincipals: unsafe extern "system" fn (this: *const nsIScriptSecurityManager, aChannel: *const nsIChannel, aPrincipal: *mut *const nsIPrincipal, aPartitionedPrincipal: *mut *const nsIPrincipal) -> ::nserror::nsresult,

    /* [noscript,nostdcall] nsIPrincipal getChannelResultPrincipalIfNotSandboxed (in nsIChannel aChannel); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetChannelResultPrincipalIfNotSandboxed: *const ::libc::c_void,

    /* nsIPrincipal getChannelURIPrincipal (in nsIChannel aChannel); */
    pub GetChannelURIPrincipal: unsafe extern "system" fn (this: *const nsIScriptSecurityManager, aChannel: *const nsIChannel, _retval: *mut *const nsIPrincipal) -> ::nserror::nsresult,

    /* nsIDomainPolicy activateDomainPolicy (); */
    pub ActivateDomainPolicy: unsafe extern "system" fn (this: *const nsIScriptSecurityManager, _retval: *mut*const nsIDomainPolicy) -> ::nserror::nsresult,

    /* readonly attribute boolean domainPolicyActive; */
    pub GetDomainPolicyActive: unsafe extern "system" fn (this: *const nsIScriptSecurityManager, aDomainPolicyActive: *mut bool) -> ::nserror::nsresult,

    /* [noscript] nsIDomainPolicy activateDomainPolicyInternal (); */
    pub ActivateDomainPolicyInternal: unsafe extern "system" fn (this: *const nsIScriptSecurityManager, _retval: *mut*const nsIDomainPolicy) -> ::nserror::nsresult,

    /* [noscript,notxpcom] void cloneDomainPolicy (in DomainPolicyClonePtr aClone); */
    /// Unable to generate binding because `native type mozilla::dom::DomainPolicyClone unsupported`
    pub CloneDomainPolicy: *const ::libc::c_void,

    /* bool policyAllowsScript (in nsIURI aDomain); */
    pub PolicyAllowsScript: unsafe extern "system" fn (this: *const nsIScriptSecurityManager, aDomain: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIScriptSecurityManager {
    /// ```text
    /// /**
    ///      * Default CheckLoadURI permissions
    ///      */
    /// ```
    ///

    pub const STANDARD: i64 = 0;


    pub const LOAD_IS_AUTOMATIC_DOCUMENT_REPLACEMENT: i64 = 1;


    pub const ALLOW_CHROME: i64 = 2;


    pub const DISALLOW_INHERIT_PRINCIPAL: i64 = 4;


    pub const DISALLOW_SCRIPT_OR_DATA: i64 = 4;


    pub const DISALLOW_SCRIPT: i64 = 8;


    pub const DONT_REPORT_ERRORS: i64 = 16;


    pub const DEFAULT_USER_CONTEXT_ID: i64 = 0;


    pub const DEFAULT_PRIVATE_BROWSING_ID: i64 = 0;

    /// ```text
    /// /**
    ///      * For each of these hooks returning NS_OK means 'let the action continue'.
    ///      * Returning an error code means 'veto the action'. XPConnect will return
    ///      * false to the js engine if the action is vetoed. The implementor of this
    ///      * interface is responsible for setting a JS exception into the JSContext
    ///      * if that is appropriate.
    ///      */
    /// ```
    ///

    /// `[noscript] void canCreateWrapper (in JSContextPtr aJSContext, in nsIIDRef aIID, in nsISupports aObj, in nsIClassInfo aClassInfo);`
    const _CanCreateWrapper: () = ();


    /// `[noscript] void canCreateInstance (in JSContextPtr aJSContext, in nsCIDRef aCID);`
    const _CanCreateInstance: () = ();


    /// `[noscript] void canGetService (in JSContextPtr aJSContext, in nsCIDRef aCID);`
    const _CanGetService: () = ();

    /// ```text
    /// /**
    ///      * Check that the script currently running in context "cx" can load "uri".
    ///      *
    ///      * Will return error code NS_ERROR_DOM_BAD_URI if the load request
    ///      * should be denied.
    ///      *
    ///      * @param cx the JSContext of the script causing the load
    ///      * @param uri the URI that is being loaded
    ///      */
    /// ```
    ///

    /// `[noscript] void checkLoadURIFromScript (in JSContextPtr cx, in nsIURI uri);`
    const _CheckLoadURIFromScript: () = ();

    /// ```text
    /// /**
    ///      * Check that content with principal aPrincipal can load "uri".
    ///      *
    ///      * Will return error code NS_ERROR_DOM_BAD_URI if the load request
    ///      * should be denied.
    ///      *
    ///      * @param aPrincipal the principal identifying the actor causing the load
    ///      * @param uri the URI that is being loaded
    ///      * @param flags the permission set, see above
    ///      * @param innerWindowID the window ID for error reporting.  If this is 0
    ///      *        (which happens automatically if it's not passed from JS), errors
    ///      *        will only appear in the browser console, not window-associated
    ///      *        consoles like the web console.
    ///      */
    /// ```
    ///

    /// `[binaryname(CheckLoadURIWithPrincipal)] void checkLoadURIWithPrincipalXPCOM (in nsIPrincipal aPrincipal, in nsIURI uri, in unsigned long flags, [optional] in unsigned long long innerWindowID);`
    #[inline]
    pub unsafe fn CheckLoadURIWithPrincipal(&self, aPrincipal: *const nsIPrincipal, uri: *const nsIURI, flags: u32, innerWindowID: u64) -> ::nserror::nsresult {
        ((*self.vtable).CheckLoadURIWithPrincipal)(self, aPrincipal, uri, flags, innerWindowID)
    }


    /// ```text
    /// /**
    ///      * Same as the above, but when called from JS, raises exceptions with more
    ///      * useful messages, including both the tested URI and the principal string.
    ///      */
    /// ```
    ///

    /// `[binaryname(CheckLoadURIWithPrincipalFromJS),implicit_jscontext] void checkLoadURIWithPrincipal (in nsIPrincipal aPrincipal, in nsIURI uri, [optional] in unsigned long flags, [optional] in unsigned long long innerWindowID);`
    const _CheckLoadURIWithPrincipalFromJS: () = ();

    /// ```text
    /// /**
    ///      * Similar to checkLoadURIWithPrincipal but there are two differences:
    ///      *
    ///      * 1) The URI is a string, not a URI object.
///      * 2) This function assumes that the URI may still be subject to fixup (and
///      * hence will check whether fixed-up versions of the URI are allowed to
///      * load as well); if any of the versions of this URI is not allowed, this
///      * function will return error code NS_ERROR_DOM_BAD_URI.
///      */
/// ```
///

/// `[binaryname(CheckLoadURIStrWithPrincipal)] void checkLoadURIStrWithPrincipalXPCOM (in nsIPrincipal aPrincipal, in AUTF8String uri, in unsigned long flags);`
#[inline]
pub unsafe fn CheckLoadURIStrWithPrincipal(&self, aPrincipal: *const nsIPrincipal, uri: *const ::nsstring::nsACString, flags: u32) -> ::nserror::nsresult {
((*self.vtable).CheckLoadURIStrWithPrincipal)(self, aPrincipal, uri, flags)
}


/// ```text
/// /**
///      * Same as the above, but when called from JS, raises exceptions with more
///      * useful messages, including both the tested URI and the principal string.
///      */
/// ```
///

/// `[binaryname(CheckLoadURIStrWithPrincipalFromJS),implicit_jscontext] void checkLoadURIStrWithPrincipal (in nsIPrincipal aPrincipal, in AUTF8String uri, [optional] in unsigned long flags);`
const _CheckLoadURIStrWithPrincipalFromJS: () = ();

/// ```text
/// /**
///      * Returns true if the URI is from a domain that is allow-listed through
///      * prefs to be allowed to use file:// URIs.
///      * @param aUri the URI to be tested
///      */
/// ```
///

/// `bool inFileURIAllowlist (in nsIURI aUri);`
#[inline]
pub unsafe fn InFileURIAllowlist(&self, aUri: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult {
((*self.vtable).InFileURIAllowlist)(self, aUri, _retval)
}


/// ```text
/// /**
///      * Return the all-powerful system principal.
///      */
/// ```
///

/// `nsIPrincipal getSystemPrincipal ();`
#[inline]
pub unsafe fn GetSystemPrincipal(&self, _retval: *mut *const nsIPrincipal) -> ::nserror::nsresult {
((*self.vtable).GetSystemPrincipal)(self, _retval)
}


/// ```text
/// /**
///      * Returns a principal that has the OriginAttributes of the load context.
///      * @param loadContext to get the OriginAttributes from.
///      */
/// ```
///

/// `nsIPrincipal getLoadContextContentPrincipal (in nsIURI uri, in nsILoadContext loadContext);`
#[inline]
pub unsafe fn GetLoadContextContentPrincipal(&self, uri: *const nsIURI, loadContext: *const nsILoadContext, _retval: *mut *const nsIPrincipal) -> ::nserror::nsresult {
((*self.vtable).GetLoadContextContentPrincipal)(self, uri, loadContext, _retval)
}


/// ```text
/// /**
///      * Returns a principal that has the OriginAttributes of the docshell.
///      * @param docShell to get the OriginAttributes from.
///      */
/// ```
///

/// `nsIPrincipal getDocShellContentPrincipal (in nsIURI uri, in nsIDocShell docShell);`
#[inline]
pub unsafe fn GetDocShellContentPrincipal(&self, uri: *const nsIURI, docShell: *const nsIDocShell, _retval: *mut *const nsIPrincipal) -> ::nserror::nsresult {
((*self.vtable).GetDocShellContentPrincipal)(self, uri, docShell, _retval)
}


/// ```text
/// /**
///      * If this is a content principal, return a copy with different
///      * origin attributes.
///      */
/// ```
///

/// `[implicit_jscontext] nsIPrincipal principalWithOA (in nsIPrincipal principal, in jsval originAttributes);`
const _PrincipalWithOA: () = ();

/// ```text
/// /**
///      * Returns a principal whose origin is composed of |uri| and |originAttributes|.
///      * See nsIPrincipal.idl for a description of origin attributes, and
///      * ChromeUtils.webidl for a list of origin attributes and their defaults.
///      */
/// ```
///

/// `[implicit_jscontext] nsIPrincipal createContentPrincipal (in nsIURI uri, in jsval originAttributes);`
const _CreateContentPrincipal: () = ();

/// ```text
/// /**
///      * Returns a principal whose origin is the one we pass in.
///      * See nsIPrincipal.idl for a description of origin attributes, and
///      * ChromeUtils.webidl for a list of origin attributes and their defaults.
///      */
/// ```
///

/// `nsIPrincipal createContentPrincipalFromOrigin (in ACString origin);`
#[inline]
pub unsafe fn CreateContentPrincipalFromOrigin(&self, origin: *const ::nsstring::nsACString, _retval: *mut *const nsIPrincipal) -> ::nserror::nsresult {
((*self.vtable).CreateContentPrincipalFromOrigin)(self, origin, _retval)
}


/// ```text
/// /**
///      * Takes a principal and returns a string representation of it or a nullptr if it can't be serialized.
///      * Example output: `{"1": {"0": "https://mozilla.com", "2": "^privateBrowsingId=1"}}`
///      */
/// ```
///

/// `ACString principalToJSON (in nsIPrincipal principal);`
#[inline]
pub unsafe fn PrincipalToJSON(&self, principal: *const nsIPrincipal, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
((*self.vtable).PrincipalToJSON)(self, principal, _retval)
}


/// ```text
/// /**
///      * Takes a string of the following format:
///      * `{"1": {"0": "https://mozilla.com", "2": "^privateBrowsingId=1"}}`
///      * and turns it into a principal or a nullptr on error.
///      */
/// ```
///

/// `nsIPrincipal JSONToPrincipal (in ACString json);`
#[inline]
pub unsafe fn JSONToPrincipal(&self, json: *const ::nsstring::nsACString, _retval: *mut *const nsIPrincipal) -> ::nserror::nsresult {
((*self.vtable).JSONToPrincipal)(self, json, _retval)
}


/// ```text
/// /**
///      * Returns a unique nonce principal with |originAttributes|.
///      * See nsIPrincipal.idl for a description of origin attributes, and
///      * ChromeUtils.webidl for a list of origin attributes and their defaults.
///      */
/// ```
///

/// `[implicit_jscontext] nsIPrincipal createNullPrincipal (in jsval originAttributes);`
const _CreateNullPrincipal: () = ();

/// ```text
/// /**
///      * Returns OK if aSourceURI and target have the same "origin"
///      * (scheme, host, and port).
///      * ReportError flag suppresses error reports for functions that
///      * don't need reporting.
///      * FromPrivateWindow indicates whether the error occurs in a private
///      * window or not.
///      */
/// ```
///

/// `void checkSameOriginURI (in nsIURI aSourceURI, in nsIURI aTargetURI, in boolean reportError, in boolean fromPrivateWindow);`
#[inline]
pub unsafe fn CheckSameOriginURI(&self, aSourceURI: *const nsIURI, aTargetURI: *const nsIURI, reportError: bool, fromPrivateWindow: bool) -> ::nserror::nsresult {
((*self.vtable).CheckSameOriginURI)(self, aSourceURI, aTargetURI, reportError, fromPrivateWindow)
}


/// ```text
/// /**
///      * Get the principal for the given channel.  This will typically be the
///      * channel owner if there is one, and the content principal for the
///      * channel's URI otherwise.  aChannel must not be null.
///      */
/// ```
///

/// `nsIPrincipal getChannelResultPrincipal (in nsIChannel aChannel);`
#[inline]
pub unsafe fn GetChannelResultPrincipal(&self, aChannel: *const nsIChannel, _retval: *mut *const nsIPrincipal) -> ::nserror::nsresult {
((*self.vtable).GetChannelResultPrincipal)(self, aChannel, _retval)
}


/// ```text
/// /**
///      * Get the storage principal for the given channel.  This is basically the
///      * same of getChannelResultPrincipal() execept for trackers, where we
///      * return a principal with a different OriginAttributes.
///      */
/// ```
///

/// `nsIPrincipal getChannelResultStoragePrincipal (in nsIChannel aChannel);`
#[inline]
pub unsafe fn GetChannelResultStoragePrincipal(&self, aChannel: *const nsIChannel, _retval: *mut *const nsIPrincipal) -> ::nserror::nsresult {
((*self.vtable).GetChannelResultStoragePrincipal)(self, aChannel, _retval)
}


/// ```text
/// /**
///      * This method returns 2 principals from a nsIChannel:
///      * - aPrincipal is the regular principal.
///      * - aPartitionedPrincipal is aPrincipal plus an isolation key in its
///      *   originAttributes.
///      * See more in StoragePrincipalHelper.h
///      */
/// ```
///

/// `void getChannelResultPrincipals (in nsIChannel aChannel, out nsIPrincipal aPrincipal, out nsIPrincipal aPartitionedPrincipal);`
#[inline]
pub unsafe fn GetChannelResultPrincipals(&self, aChannel: *const nsIChannel, aPrincipal: *mut *const nsIPrincipal, aPartitionedPrincipal: *mut *const nsIPrincipal) -> ::nserror::nsresult {
((*self.vtable).GetChannelResultPrincipals)(self, aChannel, aPrincipal, aPartitionedPrincipal)
}


/// ```text
/// /**
///      * Temporary API until bug 1220687 is fixed.
///      *
///      * Returns the same value as getChannelResultPrincipal, but ignoring
///      * sandboxing.  Specifically, if sandboxing would have prevented the
///      * channel's triggering principal from being returned by
///      * getChannelResultPrincipal, the triggering principal will be returned
///      * by this method.
///      *
///      * Note that this method only ignores sandboxing of the channel in
///      * question, it does not ignore sandboxing of any channels further up a
///      * document chain.  The triggering principal itself may still be the null
///      * principal due to sandboxing further up a document chain.  In that regard
///      * the ignoring of sandboxing is limited.
///      */
/// ```
///

/// `[noscript,nostdcall] nsIPrincipal getChannelResultPrincipalIfNotSandboxed (in nsIChannel aChannel);`
const _GetChannelResultPrincipalIfNotSandboxed: () = ();

/// ```text
/// /**
///      * Get the content principal for the channel's URI.
///      * aChannel must not be null.
///      */
/// ```
///

/// `nsIPrincipal getChannelURIPrincipal (in nsIChannel aChannel);`
#[inline]
pub unsafe fn GetChannelURIPrincipal(&self, aChannel: *const nsIChannel, _retval: *mut *const nsIPrincipal) -> ::nserror::nsresult {
((*self.vtable).GetChannelURIPrincipal)(self, aChannel, _retval)
}


/// ```text
/// /**
///      * Per-domain controls to enable and disable script. This system is designed
///      * to be used by at most one consumer, and enforces this with its semantics.
///      *
///      * Initially, domainPolicyActive is false. When activateDomainPolicy() is
///      * invoked, domainPolicyActive becomes true, and subsequent calls to
///      * activateDomainPolicy() will fail until deactivate() is invoked on the
///      * nsIDomainPolicy returned from activateDomainPolicy(). At this point,
///      * domainPolicyActive becomes false again, and a new consumer may acquire
///      * control of the system by invoking activateDomainPolicy().
///      */
/// ```
///

/// `nsIDomainPolicy activateDomainPolicy ();`
#[inline]
pub unsafe fn ActivateDomainPolicy(&self, _retval: *mut*const nsIDomainPolicy) -> ::nserror::nsresult {
((*self.vtable).ActivateDomainPolicy)(self, _retval)
}



/// `readonly attribute boolean domainPolicyActive;`
#[inline]
pub unsafe fn GetDomainPolicyActive(&self, aDomainPolicyActive: *mut bool) -> ::nserror::nsresult {
((*self.vtable).GetDomainPolicyActive)(self, aDomainPolicyActive)
}


/// ```text
/// /**
///      * Only the parent process can directly access domain policies, child
///      * processes only have a read-only mirror to the one in the parent.
///      * For child processes the mirror is updated via messages
///      * and ContentChild will hold the DomainPolicy by calling
///      * ActivateDomainPolicyInternal directly. New consumer to this
///      * function should not be addded.
///      */
/// ```
///

/// `[noscript] nsIDomainPolicy activateDomainPolicyInternal ();`
#[inline]
pub unsafe fn ActivateDomainPolicyInternal(&self, _retval: *mut*const nsIDomainPolicy) -> ::nserror::nsresult {
((*self.vtable).ActivateDomainPolicyInternal)(self, _retval)
}


/// ```text
/// /**
///      * This function is for internal use only. Every time a child process is spawned, we
///      * must clone any active domain policies in the parent to the new child.
///      */
/// ```
///

/// `[noscript,notxpcom] void cloneDomainPolicy (in DomainPolicyClonePtr aClone);`
const _CloneDomainPolicy: () = ();

/// ```text
/// /**
///      * Query mechanism for the above policy.
///      *
///      * If domainPolicyEnabled is false, this simply returns the current value
///      * of javascript.enabled. Otherwise, it returns the same value, but taking
///      * the various blocklist/allowlist exceptions into account.
///      */
/// ```
///

/// `bool policyAllowsScript (in nsIURI aDomain);`
#[inline]
pub unsafe fn PolicyAllowsScript(&self, aDomain: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult {
((*self.vtable).PolicyAllowsScript)(self, aDomain, _retval)
}


}


