//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIWebNavigation.idl
//


/// `interface nsIWebNavigation : nsISupports`
///

/// ```text
/// /**
///  * The nsIWebNavigation interface defines an interface for navigating the web.
///  * It provides methods and attributes to direct an object to navigate to a new
///  * location, stop or restart an in process load, or determine where the object
///  * has previously gone.
///  *
///  * Even though this is builtinclass, most of the interface is also implemented
///  * in RemoteWebNavigation, so if this interface changes, the implementation
///  * there may also need to change.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWebNavigation {
    vtable: *const nsIWebNavigationVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWebNavigation.
unsafe impl XpCom for nsIWebNavigation {
    const IID: nsIID = nsID(0x3ade79d4, 0x8cb9, 0x4952,
        [0xb1, 0x8d, 0x4f, 0x9b, 0x63, 0xca, 0x0d, 0x31]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWebNavigation {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWebNavigation.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWebNavigationCoerce {
    /// Cheaply cast a value of this type from a `nsIWebNavigation`.
    fn coerce_from(v: &nsIWebNavigation) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWebNavigationCoerce for nsIWebNavigation {
    #[inline]
    fn coerce_from(v: &nsIWebNavigation) -> &Self {
        v
    }
}

impl nsIWebNavigation {
    /// Cast this `nsIWebNavigation` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWebNavigationCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWebNavigation {
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
impl<T: nsISupportsCoerce> nsIWebNavigationCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebNavigation) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWebNavigation
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWebNavigationVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean canGoBack; */
    pub GetCanGoBack: unsafe extern "system" fn (this: *const nsIWebNavigation, aCanGoBack: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean canGoForward; */
    pub GetCanGoForward: unsafe extern "system" fn (this: *const nsIWebNavigation, aCanGoForward: *mut bool) -> ::nserror::nsresult,

    /* void goBack ([optional] in boolean aRequireUserInteraction); */
    pub GoBack: unsafe extern "system" fn (this: *const nsIWebNavigation, aRequireUserInteraction: bool) -> ::nserror::nsresult,

    /* void goForward ([optional] in boolean aRequireUserInteraction); */
    pub GoForward: unsafe extern "system" fn (this: *const nsIWebNavigation, aRequireUserInteraction: bool) -> ::nserror::nsresult,

    /* void gotoIndex (in long index); */
    pub GotoIndex: unsafe extern "system" fn (this: *const nsIWebNavigation, index: i32) -> ::nserror::nsresult,

    /* [binaryname(LoadURIFromScript),implicit_jscontext] void loadURI (in AString aURI, in jsval aLoadURIOptions); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub LoadURIFromScript: *const ::libc::c_void,

    /* [binaryname(LoadURI),nostdcall] void binaryLoadURI (in AString aURI, in LoadURIOptionsRef aLoadURIOptions); */
    /// Unable to generate binding because `native type const mozilla::dom::LoadURIOptions unsupported`
    pub LoadURI: *const ::libc::c_void,

    /* void reload (in unsigned long aReloadFlags); */
    pub Reload: unsafe extern "system" fn (this: *const nsIWebNavigation, aReloadFlags: u32) -> ::nserror::nsresult,

    /* void stop (in unsigned long aStopFlags); */
    pub Stop: unsafe extern "system" fn (this: *const nsIWebNavigation, aStopFlags: u32) -> ::nserror::nsresult,

    /* readonly attribute Document document; */
    pub GetDocument: unsafe extern "system" fn (this: *const nsIWebNavigation, aDocument: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* readonly attribute nsIURI currentURI; */
    pub GetCurrentURI: unsafe extern "system" fn (this: *const nsIWebNavigation, aCurrentURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* [binaryname(SessionHistoryXPCOM)] readonly attribute nsISupports sessionHistory; */
    pub GetSessionHistoryXPCOM: unsafe extern "system" fn (this: *const nsIWebNavigation, aSessionHistory: *mut *const nsISupports) -> ::nserror::nsresult,

    /* void resumeRedirectedLoad (in unsigned long long aLoadIdentifier, in long aHistoryIndex); */
    pub ResumeRedirectedLoad: unsafe extern "system" fn (this: *const nsIWebNavigation, aLoadIdentifier: u64, aHistoryIndex: i32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWebNavigation {
    /// ```text
    /// /****************************************************************************
    ///    * The following flags may be bitwise combined to form the load flags
    ///    * parameter passed to either the loadURI or reload method.  Some of these
    ///    * flags are only applicable to loadURI.
    ///    */
    /// /**
    ///    * This flags defines the range of bits that may be specified.  Flags
    ///    * outside this range may be used, but may not be passed to Reload().
    ///    */
    /// ```
    ///

    pub const LOAD_FLAGS_MASK: i64 = 65535;

    /// ```text
    /// /**
    ///    * This is the default value for the load flags parameter.
    ///    */
    /// ```
    ///

    pub const LOAD_FLAGS_NONE: i64 = 0;

    /// ```text
    /// /**
    ///    * Flags 0x1, 0x2, 0x4, 0x8 are reserved for internal use by
    ///    * nsIWebNavigation implementations for now.
    ///    */
    /// /**
    ///    * This flag specifies that the load should have the semantics of an HTML
    ///    * Meta-refresh tag (i.e., that the cache should be bypassed).  This flag
    ///    * is only applicable to loadURI.
    ///    * XXX the meaning of this flag is poorly defined.
    ///    * XXX no one uses this, so we should probably deprecate and remove it.
    ///    */
    /// ```
    ///

    pub const LOAD_FLAGS_IS_REFRESH: i64 = 16;

    /// ```text
    /// /**
    ///    * This flag specifies that the load should have the semantics of a link
    ///    * click.  This flag is only applicable to loadURI.
    ///    * XXX the meaning of this flag is poorly defined.
    ///    */
    /// ```
    ///

    pub const LOAD_FLAGS_IS_LINK: i64 = 32;

    /// ```text
    /// /**
    ///    * This flag specifies that history should not be updated.  This flag is only
    ///    * applicable to loadURI.
    ///    */
    /// ```
    ///

    pub const LOAD_FLAGS_BYPASS_HISTORY: i64 = 64;

    /// ```text
    /// /**
    ///    * This flag specifies that any existing history entry should be replaced.
    ///    * This flag is only applicable to loadURI.
    ///    */
    /// ```
    ///

    pub const LOAD_FLAGS_REPLACE_HISTORY: i64 = 128;

    /// ```text
    /// /**
    ///    * This flag specifies that the local web cache should be bypassed, but an
    ///    * intermediate proxy cache could still be used to satisfy the load.
    ///    */
    /// ```
    ///

    pub const LOAD_FLAGS_BYPASS_CACHE: i64 = 256;

    /// ```text
    /// /**
    ///    * This flag specifies that any intermediate proxy caches should be bypassed
    ///    * (i.e., that the content should be loaded from the origin server).
    ///    */
    /// ```
    ///

    pub const LOAD_FLAGS_BYPASS_PROXY: i64 = 512;

    /// ```text
    /// /**
    ///    * This flag specifies that a reload was triggered as a result of detecting
    ///    * an incorrect character encoding while parsing a previously loaded
    ///    * document.
    ///    */
    /// ```
    ///

    pub const LOAD_FLAGS_CHARSET_CHANGE: i64 = 1024;

    /// ```text
    /// /**
    ///    * If this flag is set, Stop() will be called before the load starts
    ///    * and will stop both content and network activity (the default is to
        ///    * only stop network activity).  Effectively, this passes the
    ///    * STOP_CONTENT flag to Stop(), in addition to the STOP_NETWORK flag.
    ///    */
    /// ```
    ///

    pub const LOAD_FLAGS_STOP_CONTENT: i64 = 2048;

    /// ```text
    /// /**
    ///    * A hint this load was prompted by an external program: take care!
    ///    */
    /// ```
    ///

    pub const LOAD_FLAGS_FROM_EXTERNAL: i64 = 4096;

    /// ```text
    /// /**
    ///     This flag is set when a user explicitly disables the Mixed Content
    ///     Blocker, and allows Mixed Content to load on an https page.
    ///   */
    /// ```
    ///

    pub const LOAD_FLAGS_ALLOW_MIXED_CONTENT: i64 = 8192;

    /// ```text
    /// /**
    ///    * This flag specifies that this is the first load in this object.
    ///    * Set with care, since setting incorrectly can cause us to assume that
    ///    * nothing was actually loaded in this object if the load ends up being
    ///    * handled by an external application.  This flag must not be passed to
    ///    * Reload.
    ///    */
    /// ```
    ///

    pub const LOAD_FLAGS_FIRST_LOAD: i64 = 16384;

    /// ```text
    /// /**
    ///    * This flag specifies that the load should not be subject to popup
    ///    * blocking checks.  This flag must not be passed to Reload.
    ///    */
    /// ```
    ///

    pub const LOAD_FLAGS_ALLOW_POPUPS: i64 = 32768;

    /// ```text
    /// /**
    ///    * This flag specifies that the URI classifier should not be checked for
    ///    * this load.  This flag must not be passed to Reload.
    ///    */
    /// ```
    ///

    pub const LOAD_FLAGS_BYPASS_CLASSIFIER: i64 = 65536;

    /// ```text
    /// /**
    ///    * Force relevant cookies to be sent with this load even if normally they
    ///    * wouldn't be.
    ///    */
    /// ```
    ///

    pub const LOAD_FLAGS_FORCE_ALLOW_COOKIES: i64 = 131072;

    /// ```text
    /// /**
    ///    * Prevent the owner principal from being inherited for this load.
    ///    */
    /// ```
    ///

    pub const LOAD_FLAGS_DISALLOW_INHERIT_PRINCIPAL: i64 = 262144;

    /// ```text
    /// /**
    ///    * Overwrite the returned error code with a specific result code
    ///    * when an error page is displayed.
    ///    */
    /// ```
    ///

    pub const LOAD_FLAGS_ERROR_LOAD_CHANGES_RV: i64 = 524288;

    /// ```text
    /// /**
    ///    * This flag specifies that the URI may be submitted to a third-party
    ///    * server for correction. This should only be applied to non-sensitive
    ///    * URIs entered by users.  This flag must not be passed to Reload.
    ///    */
    /// ```
    ///

    pub const LOAD_FLAGS_ALLOW_THIRD_PARTY_FIXUP: i64 = 1048576;

    /// ```text
    /// /**
    ///    * This flag specifies that common scheme typos should be corrected.
    ///    */
    /// ```
    ///

    pub const LOAD_FLAGS_FIXUP_SCHEME_TYPOS: i64 = 2097152;

    /// ```text
    /// /**
    ///    * Allows a top-level data: navigation to occur. E.g. view-image
    ///    * is an explicit user action which should be allowed.
    ///    */
    /// ```
    ///

    pub const LOAD_FLAGS_FORCE_ALLOW_DATA_URI: i64 = 4194304;

    /// ```text
    /// /**
    ///    * This load is the result of an HTTP redirect.
    ///    */
    /// ```
    ///

    pub const LOAD_FLAGS_IS_REDIRECT: i64 = 8388608;

    /// ```text
    /// /**
    ///    * These flags force TRR modes 1 or 3 for the load.
    ///    */
    /// ```
    ///

    pub const LOAD_FLAGS_DISABLE_TRR: i64 = 16777216;


    pub const LOAD_FLAGS_FORCE_TRR: i64 = 33554432;

    /// ```text
    /// /**
    ///    * This load should bypass the LoadURIDelegate.loadUri.
    ///    */
    /// ```
    ///

    pub const LOAD_FLAGS_BYPASS_LOAD_URI_DELEGATE: i64 = 67108864;

    /// ```text
    /// /****************************************************************************
    ///    * The following flags may be passed as the stop flags parameter to the stop
    ///    * method defined on this interface.
    ///    */
    /// /**
    ///    * This flag specifies that all network activity should be stopped.  This
    ///    * includes both active network loads and pending META-refreshes.
    ///    */
    /// ```
    ///

    pub const STOP_NETWORK: i64 = 1;

    /// ```text
    /// /**
    ///    * This flag specifies that all content activity should be stopped.  This
    ///    * includes animated images, plugins and pending Javascript timeouts.
    ///    */
    /// ```
    ///

    pub const STOP_CONTENT: i64 = 2;

    /// ```text
    /// /**
    ///    * This flag specifies that all activity should be stopped.
    ///    */
    /// ```
    ///

    pub const STOP_ALL: i64 = 3;

    /// ```text
    /// /**
    ///    * Indicates if the object can go back.  If true this indicates that
    ///    * there is back session history available for navigation.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean canGoBack;`
    #[inline]
    pub unsafe fn GetCanGoBack(&self, aCanGoBack: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCanGoBack)(self, aCanGoBack)
    }


    /// ```text
    /// /**
    ///    * Indicates if the object can go forward.  If true this indicates that
    ///    * there is forward session history available for navigation
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean canGoForward;`
    #[inline]
    pub unsafe fn GetCanGoForward(&self, aCanGoForward: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCanGoForward)(self, aCanGoForward)
    }


    /// ```text
    /// /**
    ///    * Tells the object to navigate to the previous session history item.  When a
    ///    * page is loaded from session history, all content is loaded from the cache
    ///    * (if available) and page state (such as form values and scroll position) is
    ///    * restored.
    ///    *
    ///    * @param {boolean} aRequireUserInteraction
    ///    *        Tells goBack to skip history items that did not record any user
    ///    *        interaction on their corresponding document while they were active.
    ///    *        This means in case of multiple entries mapping to the same document,
    ///    *        each entry has to have been flagged with user interaction separately.
    ///    *        If no items have user interaction, the function will fall back
    ///    *        to the first session history entry.
    ///    *
    ///    * @throw NS_ERROR_UNEXPECTED
    ///    *        Indicates that the call was unexpected at this time, which implies
    ///    *        that canGoBack is false.
    ///    */
    /// ```
    ///

    /// `void goBack ([optional] in boolean aRequireUserInteraction);`
    #[inline]
    pub unsafe fn GoBack(&self, aRequireUserInteraction: bool) -> ::nserror::nsresult {
        ((*self.vtable).GoBack)(self, aRequireUserInteraction)
    }


    /// ```text
    /// /**
    ///    * Tells the object to navigate to the next session history item.  When a
    ///    * page is loaded from session history, all content is loaded from the cache
    ///    * (if available) and page state (such as form values and scroll position) is
    ///    * restored.
    ///    *
    ///    * @param {boolean} aRequireUserInteraction
    ///    *        Tells goForward to skip history items that did not record any user
    ///    *        interaction on their corresponding document while they were active.
    ///    *        This means in case of multiple entries mapping to the same document,
    ///    *        each entry has to have been flagged with user interaction separately.
    ///    *        If no items have user interaction, the function will fall back
    ///    *        to the latest session history entry.
    ///    *
    ///    * @throw NS_ERROR_UNEXPECTED
    ///    *        Indicates that the call was unexpected at this time, which implies
    ///    *        that canGoForward is false.
    ///    */
    /// ```
    ///

    /// `void goForward ([optional] in boolean aRequireUserInteraction);`
    #[inline]
    pub unsafe fn GoForward(&self, aRequireUserInteraction: bool) -> ::nserror::nsresult {
        ((*self.vtable).GoForward)(self, aRequireUserInteraction)
    }


    /// ```text
    /// /**
    ///    * Tells the object to navigate to the session history item at a given index.
    ///    *
    ///    * @throw NS_ERROR_UNEXPECTED
    ///    *        Indicates that the call was unexpected at this time, which implies
    ///    *        that session history entry at the given index does not exist.
    ///    */
    /// ```
    ///

    /// `void gotoIndex (in long index);`
    #[inline]
    pub unsafe fn GotoIndex(&self, index: i32) -> ::nserror::nsresult {
        ((*self.vtable).GotoIndex)(self, index)
    }


    /// ```text
    /// /**
    ///    * Loads a given URI.  This will give priority to loading the requested URI
    ///    * in the object implementing this interface.  If it can't be loaded here
    ///    * however, the URI dispatcher will go through its normal process of content
    ///    * loading.
    ///    *
    ///    * @param aURI
    ///    *        The URI string to load.  For HTTP and FTP URLs and possibly others,
    ///    *        characters above U+007F will be converted to UTF-8 and then URL-
    ///    *        escaped per the rules of RFC 2396.
    ///    * @param aLoadURIOptions
    ///    *        A JSObject defined in LoadURIOptions.webidl holding info like e.g.
    ///    *        the triggeringPrincipal, the referrer info.
    ///    */
    /// ```
    ///

    /// `[binaryname(LoadURIFromScript),implicit_jscontext] void loadURI (in AString aURI, in jsval aLoadURIOptions);`
    const _LoadURIFromScript: () = ();

    /// ```text
    /// /**
    ///    * A C++ friendly version of loadURI
    ///    */
    /// ```
    ///

    /// `[binaryname(LoadURI),nostdcall] void binaryLoadURI (in AString aURI, in LoadURIOptionsRef aLoadURIOptions);`
    const _LoadURI: () = ();

    /// ```text
    /// /**
    ///    * Tells the Object to reload the current page.  There may be cases where the
    ///    * user will be asked to confirm the reload (for example, when it is
        ///    * determined that the request is non-idempotent).
    ///    *
    ///    * @param aReloadFlags
    ///    *        Flags modifying load behaviour.  This parameter is a bitwise
    ///    *        combination of the Load Flags defined above.  (Undefined bits are
        ///    *        reserved for future use.)  Generally you will pass LOAD_FLAGS_NONE
    ///    *        for this parameter.
    ///    *
    ///    * @throw NS_BINDING_ABORTED
    ///    *        Indicating that the user canceled the reload.
    ///    */
    /// ```
    ///

    /// `void reload (in unsigned long aReloadFlags);`
    #[inline]
    pub unsafe fn Reload(&self, aReloadFlags: u32) -> ::nserror::nsresult {
        ((*self.vtable).Reload)(self, aReloadFlags)
    }


    /// ```text
    /// /**
    ///    * Stops a load of a URI.
    ///    *
    ///    * @param aStopFlags
    ///    *        This parameter is one of the stop flags defined above.
    ///    */
    /// ```
    ///

    /// `void stop (in unsigned long aStopFlags);`
    #[inline]
    pub unsafe fn Stop(&self, aStopFlags: u32) -> ::nserror::nsresult {
        ((*self.vtable).Stop)(self, aStopFlags)
    }


    /// ```text
    /// /**
    ///    * Retrieves the current DOM document for the frame, or lazily creates a
    ///    * blank document if there is none.  This attribute never returns null except
    ///    * for unexpected error situations.
    ///    */
    /// ```
    ///

    /// `readonly attribute Document document;`
    #[inline]
    pub unsafe fn GetDocument(&self, aDocument: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetDocument)(self, aDocument)
    }


    /// ```text
    /// /**
    ///    * The currently loaded URI or null.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIURI currentURI;`
    #[inline]
    pub unsafe fn GetCurrentURI(&self, aCurrentURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetCurrentURI)(self, aCurrentURI)
    }


    /// ```text
    /// /**
    ///    * The session history object used by this web navigation instance. This
    ///    * object will be a mozilla::dom::ChildSHistory object, but is returned as
    ///    * nsISupports so it can be called from JS code.
    ///    */
    /// ```
    ///

    /// `[binaryname(SessionHistoryXPCOM)] readonly attribute nsISupports sessionHistory;`
    #[inline]
    pub unsafe fn GetSessionHistoryXPCOM(&self, aSessionHistory: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).GetSessionHistoryXPCOM)(self, aSessionHistory)
    }


    /// ```text
    /// /**
    ///    * Resume a load which has been redirected from another process.
    ///    *
    ///    * A negative |aHistoryIndex| value corresponds to a non-history load being
    ///    * resumed.
    ///    */
    /// ```
    ///

    /// `void resumeRedirectedLoad (in unsigned long long aLoadIdentifier, in long aHistoryIndex);`
    #[inline]
    pub unsafe fn ResumeRedirectedLoad(&self, aLoadIdentifier: u64, aHistoryIndex: i32) -> ::nserror::nsresult {
        ((*self.vtable).ResumeRedirectedLoad)(self, aLoadIdentifier, aHistoryIndex)
    }


}


