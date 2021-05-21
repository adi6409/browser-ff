//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIDocShell.idl
//


/// `interface nsIDocShell : nsIDocShellTreeItem`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDocShell {
    vtable: *const nsIDocShellVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDocShell.
unsafe impl XpCom for nsIDocShell {
    const IID: nsIID = nsID(0x049234fe, 0xda10, 0x478b,
        [0xbc, 0x5d, 0xbc, 0x6f, 0x9a, 0x1b, 0xa6, 0x3d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDocShell {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDocShell.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDocShellCoerce {
    /// Cheaply cast a value of this type from a `nsIDocShell`.
    fn coerce_from(v: &nsIDocShell) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDocShellCoerce for nsIDocShell {
    #[inline]
    fn coerce_from(v: &nsIDocShell) -> &Self {
        v
    }
}

impl nsIDocShell {
    /// Cast this `nsIDocShell` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDocShellCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDocShell {
    type Target = nsIDocShellTreeItem;
    #[inline]
    fn deref(&self) -> &nsIDocShellTreeItem {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIDocShellTreeItemCoerce> nsIDocShellCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDocShell) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDocShell
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDocShellVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIDocShellTreeItemVTable,

    /* void setCancelContentJSEpoch (in long aEpoch); */
    pub SetCancelContentJSEpoch: unsafe extern "system" fn (this: *const nsIDocShell, aEpoch: i32) -> ::nserror::nsresult,

    /* [noscript] void loadURI (in nsDocShellLoadStatePtr aLoadState, in boolean aSetNavigating); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub LoadURI: *const ::libc::c_void,

    /* [implicit_jscontext] void addState (in jsval aData, in AString aTitle, in AString aURL, in boolean aReplace); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub AddState: *const ::libc::c_void,

    /* [nostdcall] void updateURLAndHistory (in Document aDocument, in nsIURI aNewURI, in nsIStructuredCloneContainer aData, in AString aTitle, in boolean aReplace, in nsIURI aCurrentURI, in boolean aEqualURIs); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub UpdateURLAndHistory: *const ::libc::c_void,

    /* void prepareForNewContentModel (); */
    pub PrepareForNewContentModel: unsafe extern "system" fn (this: *const nsIDocShell) -> ::nserror::nsresult,

    /* void setCurrentURI (in nsIURI aURI); */
    pub SetCurrentURI: unsafe extern "system" fn (this: *const nsIDocShell, aURI: *const nsIURI) -> ::nserror::nsresult,

    /* [noscript] void firePageHideNotification (in boolean isUnload); */
    pub FirePageHideNotification: unsafe extern "system" fn (this: *const nsIDocShell, isUnload: bool) -> ::nserror::nsresult,

    /* [nostdcall,notxpcom] readonly attribute nsPresContext presContext; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetPresContext: *const ::libc::c_void,

    /* [nostdcall,notxpcom] readonly attribute PresShell presShell; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetPresShell: *const ::libc::c_void,

    /* [nostdcall,notxpcom] readonly attribute PresShell eldestPresShell; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetEldestPresShell: *const ::libc::c_void,

    /* [infallible] readonly attribute nsIContentViewer contentViewer; */
    pub GetContentViewer: unsafe extern "system" fn (this: *const nsIDocShell, aContentViewer: *mut*const nsIContentViewer) -> ::nserror::nsresult,

    /* [infallible] readonly attribute unsigned long long outerWindowID; */
    pub GetOuterWindowID: unsafe extern "system" fn (this: *const nsIDocShell, aOuterWindowID: *mut u64) -> ::nserror::nsresult,

    /* attribute EventTarget chromeEventHandler; */
    pub GetChromeEventHandler: unsafe extern "system" fn (this: *const nsIDocShell, aChromeEventHandler: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* attribute EventTarget chromeEventHandler; */
    pub SetChromeEventHandler: unsafe extern "system" fn (this: *const nsIDocShell, aChromeEventHandler: *const libc::c_void) -> ::nserror::nsresult,

    /* attribute AString customUserAgent; */
    pub GetCustomUserAgent: unsafe extern "system" fn (this: *const nsIDocShell, aCustomUserAgent: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString customUserAgent; */
    pub SetCustomUserAgent: unsafe extern "system" fn (this: *const nsIDocShell, aCustomUserAgent: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute boolean cssErrorReportingEnabled; */
    pub GetCssErrorReportingEnabled: unsafe extern "system" fn (this: *const nsIDocShell, aCssErrorReportingEnabled: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean cssErrorReportingEnabled; */
    pub SetCssErrorReportingEnabled: unsafe extern "system" fn (this: *const nsIDocShell, aCssErrorReportingEnabled: bool) -> ::nserror::nsresult,

    /* attribute boolean allowPlugins; */
    pub GetAllowPlugins: unsafe extern "system" fn (this: *const nsIDocShell, aAllowPlugins: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean allowPlugins; */
    pub SetAllowPlugins: unsafe extern "system" fn (this: *const nsIDocShell, aAllowPlugins: bool) -> ::nserror::nsresult,

    /* attribute boolean allowJavascript; */
    pub GetAllowJavascript: unsafe extern "system" fn (this: *const nsIDocShell, aAllowJavascript: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean allowJavascript; */
    pub SetAllowJavascript: unsafe extern "system" fn (this: *const nsIDocShell, aAllowJavascript: bool) -> ::nserror::nsresult,

    /* attribute boolean allowMetaRedirects; */
    pub GetAllowMetaRedirects: unsafe extern "system" fn (this: *const nsIDocShell, aAllowMetaRedirects: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean allowMetaRedirects; */
    pub SetAllowMetaRedirects: unsafe extern "system" fn (this: *const nsIDocShell, aAllowMetaRedirects: bool) -> ::nserror::nsresult,

    /* attribute boolean allowSubframes; */
    pub GetAllowSubframes: unsafe extern "system" fn (this: *const nsIDocShell, aAllowSubframes: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean allowSubframes; */
    pub SetAllowSubframes: unsafe extern "system" fn (this: *const nsIDocShell, aAllowSubframes: bool) -> ::nserror::nsresult,

    /* attribute boolean allowImages; */
    pub GetAllowImages: unsafe extern "system" fn (this: *const nsIDocShell, aAllowImages: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean allowImages; */
    pub SetAllowImages: unsafe extern "system" fn (this: *const nsIDocShell, aAllowImages: bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean allowMedia; */
    pub GetAllowMedia: unsafe extern "system" fn (this: *const nsIDocShell, aAllowMedia: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean allowMedia; */
    pub SetAllowMedia: unsafe extern "system" fn (this: *const nsIDocShell, aAllowMedia: bool) -> ::nserror::nsresult,

    /* attribute boolean allowDNSPrefetch; */
    pub GetAllowDNSPrefetch: unsafe extern "system" fn (this: *const nsIDocShell, aAllowDNSPrefetch: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean allowDNSPrefetch; */
    pub SetAllowDNSPrefetch: unsafe extern "system" fn (this: *const nsIDocShell, aAllowDNSPrefetch: bool) -> ::nserror::nsresult,

    /* attribute boolean allowWindowControl; */
    pub GetAllowWindowControl: unsafe extern "system" fn (this: *const nsIDocShell, aAllowWindowControl: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean allowWindowControl; */
    pub SetAllowWindowControl: unsafe extern "system" fn (this: *const nsIDocShell, aAllowWindowControl: bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean allowContentRetargeting; */
    pub GetAllowContentRetargeting: unsafe extern "system" fn (this: *const nsIDocShell, aAllowContentRetargeting: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean allowContentRetargeting; */
    pub SetAllowContentRetargeting: unsafe extern "system" fn (this: *const nsIDocShell, aAllowContentRetargeting: bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean allowContentRetargetingOnChildren; */
    pub GetAllowContentRetargetingOnChildren: unsafe extern "system" fn (this: *const nsIDocShell, aAllowContentRetargetingOnChildren: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean allowContentRetargetingOnChildren; */
    pub SetAllowContentRetargetingOnChildren: unsafe extern "system" fn (this: *const nsIDocShell, aAllowContentRetargetingOnChildren: bool) -> ::nserror::nsresult,

    /* Array<nsIDocShell> getAllDocShellsInSubtree (in long aItemType, in nsIDocShell_DocShellEnumeratorDirection aDirection); */
    pub GetAllDocShellsInSubtree: unsafe extern "system" fn (this: *const nsIDocShell, aItemType: i32, aDirection:  u8, _retval: *mut thin_vec::ThinVec<RefPtr<nsIDocShell>>) -> ::nserror::nsresult,

    /* [infallible] attribute nsIDocShell_AppType appType; */
    pub GetAppType: unsafe extern "system" fn (this: *const nsIDocShell, aAppType: *mut u8) -> ::nserror::nsresult,

    /* [infallible] attribute nsIDocShell_AppType appType; */
    pub SetAppType: unsafe extern "system" fn (this: *const nsIDocShell, aAppType:  u8) -> ::nserror::nsresult,

    /* attribute boolean allowAuth; */
    pub GetAllowAuth: unsafe extern "system" fn (this: *const nsIDocShell, aAllowAuth: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean allowAuth; */
    pub SetAllowAuth: unsafe extern "system" fn (this: *const nsIDocShell, aAllowAuth: bool) -> ::nserror::nsresult,

    /* attribute float zoom; */
    pub GetZoom: unsafe extern "system" fn (this: *const nsIDocShell, aZoom: *mut libc::c_float) -> ::nserror::nsresult,

    /* attribute float zoom; */
    pub SetZoom: unsafe extern "system" fn (this: *const nsIDocShell, aZoom: libc::c_float) -> ::nserror::nsresult,

    /* bool tabToTreeOwner (in boolean forward, in boolean forDocumentNavigation); */
    pub TabToTreeOwner: unsafe extern "system" fn (this: *const nsIDocShell, forward: bool, forDocumentNavigation: bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute nsIDocShell_BusyFlags busyFlags; */
    pub GetBusyFlags: unsafe extern "system" fn (this: *const nsIDocShell, aBusyFlags: *mut u8) -> ::nserror::nsresult,

    /* [infallible] attribute unsigned long loadType; */
    pub GetLoadType: unsafe extern "system" fn (this: *const nsIDocShell, aLoadType: *mut u32) -> ::nserror::nsresult,

    /* [infallible] attribute unsigned long loadType; */
    pub SetLoadType: unsafe extern "system" fn (this: *const nsIDocShell, aLoadType: u32) -> ::nserror::nsresult,

    /* attribute nsLoadFlags defaultLoadFlags; */
    pub GetDefaultLoadFlags: unsafe extern "system" fn (this: *const nsIDocShell, aDefaultLoadFlags: *mut nsLoadFlags) -> ::nserror::nsresult,

    /* attribute nsLoadFlags defaultLoadFlags; */
    pub SetDefaultLoadFlags: unsafe extern "system" fn (this: *const nsIDocShell, aDefaultLoadFlags: nsLoadFlags) -> ::nserror::nsresult,

    /* boolean isBeingDestroyed (); */
    pub IsBeingDestroyed: unsafe extern "system" fn (this: *const nsIDocShell, _retval: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean isExecutingOnLoadHandler; */
    pub GetIsExecutingOnLoadHandler: unsafe extern "system" fn (this: *const nsIDocShell, aIsExecutingOnLoadHandler: *mut bool) -> ::nserror::nsresult,

    /* attribute nsILayoutHistoryState layoutHistoryState; */
    pub GetLayoutHistoryState: unsafe extern "system" fn (this: *const nsIDocShell, aLayoutHistoryState: *mut*const nsILayoutHistoryState) -> ::nserror::nsresult,

    /* attribute nsILayoutHistoryState layoutHistoryState; */
    pub SetLayoutHistoryState: unsafe extern "system" fn (this: *const nsIDocShell, aLayoutHistoryState: *const nsILayoutHistoryState) -> ::nserror::nsresult,

    /* readonly attribute nsILoadURIDelegate loadURIDelegate; */
    pub GetLoadURIDelegate: unsafe extern "system" fn (this: *const nsIDocShell, aLoadURIDelegate: *mut*const nsILoadURIDelegate) -> ::nserror::nsresult,

    /* void suspendRefreshURIs (); */
    pub SuspendRefreshURIs: unsafe extern "system" fn (this: *const nsIDocShell) -> ::nserror::nsresult,

    /* void resumeRefreshURIs (); */
    pub ResumeRefreshURIs: unsafe extern "system" fn (this: *const nsIDocShell) -> ::nserror::nsresult,

    /* void beginRestore (in nsIContentViewer viewer, in boolean top); */
    pub BeginRestore: unsafe extern "system" fn (this: *const nsIDocShell, viewer: *const nsIContentViewer, top: bool) -> ::nserror::nsresult,

    /* void finishRestore (); */
    pub FinishRestore: unsafe extern "system" fn (this: *const nsIDocShell) -> ::nserror::nsresult,

    /* void clearCachedUserAgent (); */
    pub ClearCachedUserAgent: unsafe extern "system" fn (this: *const nsIDocShell) -> ::nserror::nsresult,

    /* void clearCachedPlatform (); */
    pub ClearCachedPlatform: unsafe extern "system" fn (this: *const nsIDocShell) -> ::nserror::nsresult,

    /* readonly attribute boolean restoringDocument; */
    pub GetRestoringDocument: unsafe extern "system" fn (this: *const nsIDocShell, aRestoringDocument: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean useErrorPages; */
    pub GetUseErrorPages: unsafe extern "system" fn (this: *const nsIDocShell, aUseErrorPages: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean useErrorPages; */
    pub SetUseErrorPages: unsafe extern "system" fn (this: *const nsIDocShell, aUseErrorPages: bool) -> ::nserror::nsresult,

    /* boolean displayLoadError (in nsresult aError, in nsIURI aURI, in wstring aURL, [optional] in nsIChannel aFailedChannel); */
    pub DisplayLoadError: unsafe extern "system" fn (this: *const nsIDocShell, aError: ::nserror::nsresult, aURI: *const nsIURI, aURL: *const i16, aFailedChannel: *const nsIChannel, _retval: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute nsIChannel failedChannel; */
    pub GetFailedChannel: unsafe extern "system" fn (this: *const nsIDocShell, aFailedChannel: *mut*const nsIChannel) -> ::nserror::nsresult,

    /* readonly attribute long previousEntryIndex; */
    pub GetPreviousEntryIndex: unsafe extern "system" fn (this: *const nsIDocShell, aPreviousEntryIndex: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute long loadedEntryIndex; */
    pub GetLoadedEntryIndex: unsafe extern "system" fn (this: *const nsIDocShell, aLoadedEntryIndex: *mut i32) -> ::nserror::nsresult,

    /* void historyPurged (in long numEntries); */
    pub HistoryPurged: unsafe extern "system" fn (this: *const nsIDocShell, numEntries: i32) -> ::nserror::nsresult,

    /* readonly attribute nsIChannel currentDocumentChannel; */
    pub GetCurrentDocumentChannel: unsafe extern "system" fn (this: *const nsIDocShell, aCurrentDocumentChannel: *mut*const nsIChannel) -> ::nserror::nsresult,

    /* [nostdcall,notxpcom] attribute long childOffset; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetChildOffset: *const ::libc::c_void,

    /* [nostdcall,notxpcom] attribute long childOffset; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub SetChildOffset: *const ::libc::c_void,

    /* [infallible] readonly attribute boolean isInUnload; */
    pub GetIsInUnload: unsafe extern "system" fn (this: *const nsIDocShell, aIsInUnload: *mut bool) -> ::nserror::nsresult,

    /* [noscript,notxpcom] void DetachEditorFromWindow (); */
    pub DetachEditorFromWindow: unsafe extern "system" fn (this: *const nsIDocShell) -> libc::c_void,

    /* attribute boolean isOffScreenBrowser; */
    pub GetIsOffScreenBrowser: unsafe extern "system" fn (this: *const nsIDocShell, aIsOffScreenBrowser: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean isOffScreenBrowser; */
    pub SetIsOffScreenBrowser: unsafe extern "system" fn (this: *const nsIDocShell, aIsOffScreenBrowser: bool) -> ::nserror::nsresult,

    /* void exitPrintPreview (); */
    pub ExitPrintPreview: unsafe extern "system" fn (this: *const nsIDocShell) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean canExecuteScripts; */
    pub GetCanExecuteScripts: unsafe extern "system" fn (this: *const nsIDocShell, aCanExecuteScripts: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute nsIDRef historyID; */
    pub GetHistoryID: unsafe extern "system" fn (this: *const nsIDocShell, aHistoryID: *mut nsID) -> ::nserror::nsresult,

    /* [noscript,notxpcom] nsIDRef HistoryID (); */
    pub HistoryID: unsafe extern "system" fn (this: *const nsIDocShell) -> *const nsID,

    /* attribute boolean isAppTab; */
    pub GetIsAppTab: unsafe extern "system" fn (this: *const nsIDocShell, aIsAppTab: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean isAppTab; */
    pub SetIsAppTab: unsafe extern "system" fn (this: *const nsIDocShell, aIsAppTab: bool) -> ::nserror::nsresult,

    /* void createAboutBlankContentViewer (in nsIPrincipal aPrincipal, in nsIPrincipal aPartitionedPrincipal, [optional] in nsIContentSecurityPolicy aCSP); */
    pub CreateAboutBlankContentViewer: unsafe extern "system" fn (this: *const nsIDocShell, aPrincipal: *const nsIPrincipal, aPartitionedPrincipal: *const nsIPrincipal, aCSP: *const nsIContentSecurityPolicy) -> ::nserror::nsresult,

    /* attribute ACString charset; */
    pub GetCharset: unsafe extern "system" fn (this: *const nsIDocShell, aCharset: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute ACString charset; */
    pub SetCharset: unsafe extern "system" fn (this: *const nsIDocShell, aCharset: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void gatherCharsetMenuTelemetry (); */
    pub GatherCharsetMenuTelemetry: unsafe extern "system" fn (this: *const nsIDocShell) -> ::nserror::nsresult,

    /* [noscript,nostdcall,notxpcom] void setParentCharset (in Encoding parentCharset, in int32_t parentCharsetSource, in nsIPrincipal parentCharsetPrincipal); */
    /// Unable to generate binding because `native type const mozilla::Encoding* unsupported`
    pub SetParentCharset: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] void getParentCharset (out Encoding parentCharset, out int32_t parentCharsetSource, out nsIPrincipal parentCharsetPrincipal); */
    /// Unable to generate binding because `native type const mozilla::Encoding* unsupported`
    pub GetParentCharset: *const ::libc::c_void,

    /* [infallible] attribute boolean recordProfileTimelineMarkers; */
    pub GetRecordProfileTimelineMarkers: unsafe extern "system" fn (this: *const nsIDocShell, aRecordProfileTimelineMarkers: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean recordProfileTimelineMarkers; */
    pub SetRecordProfileTimelineMarkers: unsafe extern "system" fn (this: *const nsIDocShell, aRecordProfileTimelineMarkers: bool) -> ::nserror::nsresult,

    /* DOMHighResTimeStamp now (); */
    pub Now: unsafe extern "system" fn (this: *const nsIDocShell, _retval: *mut DOMHighResTimeStamp) -> ::nserror::nsresult,

    /* [implicit_jscontext] jsval popProfileTimelineMarkers (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub PopProfileTimelineMarkers: *const ::libc::c_void,

    /* void addWeakPrivacyTransitionObserver (in nsIPrivacyTransitionObserver obs); */
    pub AddWeakPrivacyTransitionObserver: unsafe extern "system" fn (this: *const nsIDocShell, obs: *const nsIPrivacyTransitionObserver) -> ::nserror::nsresult,

    /* void addWeakReflowObserver (in nsIReflowObserver obs); */
    pub AddWeakReflowObserver: unsafe extern "system" fn (this: *const nsIDocShell, obs: *const nsIReflowObserver) -> ::nserror::nsresult,

    /* void removeWeakReflowObserver (in nsIReflowObserver obs); */
    pub RemoveWeakReflowObserver: unsafe extern "system" fn (this: *const nsIDocShell, obs: *const nsIReflowObserver) -> ::nserror::nsresult,

    /* [noscript] void notifyReflowObservers (in bool interruptible, in DOMHighResTimeStamp start, in DOMHighResTimeStamp end); */
    pub NotifyReflowObservers: unsafe extern "system" fn (this: *const nsIDocShell, interruptible: bool, start: DOMHighResTimeStamp, end: DOMHighResTimeStamp) -> ::nserror::nsresult,

    /* [noscript] void addWeakScrollObserver (in nsIScrollObserver obs); */
    pub AddWeakScrollObserver: unsafe extern "system" fn (this: *const nsIDocShell, obs: *const nsIScrollObserver) -> ::nserror::nsresult,

    /* [noscript] void removeWeakScrollObserver (in nsIScrollObserver obs); */
    pub RemoveWeakScrollObserver: unsafe extern "system" fn (this: *const nsIDocShell, obs: *const nsIScrollObserver) -> ::nserror::nsresult,

    /* [noscript] void notifyScrollObservers (); */
    pub NotifyScrollObservers: unsafe extern "system" fn (this: *const nsIDocShell) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean isTopLevelContentDocShell; */
    pub GetIsTopLevelContentDocShell: unsafe extern "system" fn (this: *const nsIDocShell, aIsTopLevelContentDocShell: *mut bool) -> ::nserror::nsresult,

    /* nsIDocShell getSameTypeInProcessParentIgnoreBrowserBoundaries (); */
    pub GetSameTypeInProcessParentIgnoreBrowserBoundaries: unsafe extern "system" fn (this: *const nsIDocShell, _retval: *mut *const nsIDocShell) -> ::nserror::nsresult,

    /* readonly attribute bool asyncPanZoomEnabled; */
    pub GetAsyncPanZoomEnabled: unsafe extern "system" fn (this: *const nsIDocShell, aAsyncPanZoomEnabled: *mut bool) -> ::nserror::nsresult,

    /* attribute nsIChannel mixedContentChannel; */
    pub GetMixedContentChannel: unsafe extern "system" fn (this: *const nsIDocShell, aMixedContentChannel: *mut*const nsIChannel) -> ::nserror::nsresult,

    /* attribute nsIChannel mixedContentChannel; */
    pub SetMixedContentChannel: unsafe extern "system" fn (this: *const nsIDocShell, aMixedContentChannel: *const nsIChannel) -> ::nserror::nsresult,

    /* [noscript,notxpcom] bool pluginsAllowedInCurrentDoc (); */
    pub PluginsAllowedInCurrentDoc: unsafe extern "system" fn (this: *const nsIDocShell) -> bool,

    /* [infallible,noscript] attribute boolean affectPrivateSessionLifetime; */
    pub GetAffectPrivateSessionLifetime: unsafe extern "system" fn (this: *const nsIDocShell, aAffectPrivateSessionLifetime: *mut bool) -> ::nserror::nsresult,

    /* [infallible,noscript] attribute boolean affectPrivateSessionLifetime; */
    pub SetAffectPrivateSessionLifetime: unsafe extern "system" fn (this: *const nsIDocShell, aAffectPrivateSessionLifetime: bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean mayEnableCharacterEncodingMenu; */
    pub GetMayEnableCharacterEncodingMenu: unsafe extern "system" fn (this: *const nsIDocShell, aMayEnableCharacterEncodingMenu: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean charsetAutodetected; */
    pub GetCharsetAutodetected: unsafe extern "system" fn (this: *const nsIDocShell, aCharsetAutodetected: *mut bool) -> ::nserror::nsresult,

    /* attribute nsIEditor editor; */
    pub GetEditor: unsafe extern "system" fn (this: *const nsIDocShell, aEditor: *mut*const nsIEditor) -> ::nserror::nsresult,

    /* attribute nsIEditor editor; */
    pub SetEditor: unsafe extern "system" fn (this: *const nsIDocShell, aEditor: *const nsIEditor) -> ::nserror::nsresult,

    /* readonly attribute boolean editable; */
    pub GetEditable: unsafe extern "system" fn (this: *const nsIDocShell, aEditable: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean hasEditingSession; */
    pub GetHasEditingSession: unsafe extern "system" fn (this: *const nsIDocShell, aHasEditingSession: *mut bool) -> ::nserror::nsresult,

    /* void makeEditable (in boolean inWaitForUriLoad); */
    pub MakeEditable: unsafe extern "system" fn (this: *const nsIDocShell, inWaitForUriLoad: bool) -> ::nserror::nsresult,

    /* boolean getCurrentSHEntry (out nsISHEntry aEntry); */
    pub GetCurrentSHEntry: unsafe extern "system" fn (this: *const nsIDocShell, aEntry: *mut*const nsISHEntry, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean isCommandEnabled (in string command); */
    pub IsCommandEnabled: unsafe extern "system" fn (this: *const nsIDocShell, command: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult,

    /* [can_run_script] void doCommand (in string command); */
    pub DoCommand: unsafe extern "system" fn (this: *const nsIDocShell, command: *const libc::c_char) -> ::nserror::nsresult,

    /* [can_run_script] void doCommandWithParams (in string command, in nsICommandParams aParams); */
    pub DoCommandWithParams: unsafe extern "system" fn (this: *const nsIDocShell, command: *const libc::c_char, aParams: *const nsICommandParams) -> ::nserror::nsresult,

    /* [noscript,notxpcom] bool IsInvisible (); */
    pub IsInvisible: unsafe extern "system" fn (this: *const nsIDocShell) -> bool,

    /* [noscript,notxpcom] void SetInvisible (in bool aIsInvisibleDocshell); */
    pub SetInvisible: unsafe extern "system" fn (this: *const nsIDocShell, aIsInvisibleDocshell: bool) -> libc::c_void,

    /* [noscript,nostdcall,notxpcom] nsIScriptGlobalObject GetScriptGlobalObject (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetScriptGlobalObject: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] Document getExtantDocument (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetExtantDocument: *const ::libc::c_void,

    /* [infallible] attribute boolean deviceSizeIsPageSize; */
    pub GetDeviceSizeIsPageSize: unsafe extern "system" fn (this: *const nsIDocShell, aDeviceSizeIsPageSize: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean deviceSizeIsPageSize; */
    pub SetDeviceSizeIsPageSize: unsafe extern "system" fn (this: *const nsIDocShell, aDeviceSizeIsPageSize: bool) -> ::nserror::nsresult,

    /* [noscript,nostdcall,notxpcom] void notifyJSRunToCompletionStart (in string aReason, in AString functionName, in AString fileName, in unsigned long lineNumber, in jsval asyncStack, in string asyncCause); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub NotifyJSRunToCompletionStart: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] void notifyJSRunToCompletionStop (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub NotifyJSRunToCompletionStop: *const ::libc::c_void,

    /* [infallible] readonly attribute boolean hasLoadedNonBlankURI; */
    pub GetHasLoadedNonBlankURI: unsafe extern "system" fn (this: *const nsIDocShell, aHasLoadedNonBlankURI: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean windowDraggingAllowed; */
    pub GetWindowDraggingAllowed: unsafe extern "system" fn (this: *const nsIDocShell, aWindowDraggingAllowed: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean windowDraggingAllowed; */
    pub SetWindowDraggingAllowed: unsafe extern "system" fn (this: *const nsIDocShell, aWindowDraggingAllowed: bool) -> ::nserror::nsresult,

    /* attribute boolean currentScrollRestorationIsManual; */
    pub GetCurrentScrollRestorationIsManual: unsafe extern "system" fn (this: *const nsIDocShell, aCurrentScrollRestorationIsManual: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean currentScrollRestorationIsManual; */
    pub SetCurrentScrollRestorationIsManual: unsafe extern "system" fn (this: *const nsIDocShell, aCurrentScrollRestorationIsManual: bool) -> ::nserror::nsresult,

    /* [implicit_jscontext] jsval getOriginAttributes (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetOriginAttributes: *const ::libc::c_void,

    /* [implicit_jscontext] void setOriginAttributes (in jsval aAttrs); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub SetOriginAttributes: *const ::libc::c_void,

    /* readonly attribute nsIEditingSession editingSession; */
    pub GetEditingSession: unsafe extern "system" fn (this: *const nsIDocShell, aEditingSession: *mut*const nsIEditingSession) -> ::nserror::nsresult,

    /* [binaryname(ScriptableBrowserChild)] readonly attribute nsIBrowserChild browserChild; */
    pub GetScriptableBrowserChild: unsafe extern "system" fn (this: *const nsIDocShell, aBrowserChild: *mut*const nsIBrowserChild) -> ::nserror::nsresult,

    /* [noscript,nostdcall,notxpcom] BrowserChildRef GetBrowserChild (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetBrowserChild: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] nsCommandManager GetCommandManager (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetCommandManager: *const ::libc::c_void,

    /* [infallible] attribute nsIDocShell_MetaViewportOverride metaViewportOverride; */
    pub GetMetaViewportOverride: unsafe extern "system" fn (this: *const nsIDocShell, aMetaViewportOverride: *mut u8) -> ::nserror::nsresult,

    /* [infallible] attribute nsIDocShell_MetaViewportOverride metaViewportOverride; */
    pub SetMetaViewportOverride: unsafe extern "system" fn (this: *const nsIDocShell, aMetaViewportOverride:  u8) -> ::nserror::nsresult,

    /* attribute boolean useTrackingProtection; */
    pub GetUseTrackingProtection: unsafe extern "system" fn (this: *const nsIDocShell, aUseTrackingProtection: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean useTrackingProtection; */
    pub SetUseTrackingProtection: unsafe extern "system" fn (this: *const nsIDocShell, aUseTrackingProtection: bool) -> ::nserror::nsresult,

    /* [noscript] void dispatchLocationChangeEvent (); */
    pub DispatchLocationChangeEvent: unsafe extern "system" fn (this: *const nsIDocShell) -> ::nserror::nsresult,

    /* [noscript] void startDelayedAutoplayMediaComponents (); */
    pub StartDelayedAutoplayMediaComponents: unsafe extern "system" fn (this: *const nsIDocShell) -> ::nserror::nsresult,

    /* [noscript,nostdcall,notxpcom] UniqueClientSource TakeInitialClientSource (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub TakeInitialClientSource: *const ::libc::c_void,

    /* void setColorMatrix (in Array<float> aMatrix); */
    pub SetColorMatrix: unsafe extern "system" fn (this: *const nsIDocShell, aMatrix: *const thin_vec::ThinVec<libc::c_float>) -> ::nserror::nsresult,

    /* readonly attribute bool isForceReloading; */
    pub GetIsForceReloading: unsafe extern "system" fn (this: *const nsIDocShell, aIsForceReloading: *mut bool) -> ::nserror::nsresult,

    /* Array<float> getColorMatrix (); */
    pub GetColorMatrix: unsafe extern "system" fn (this: *const nsIDocShell, _retval: *mut thin_vec::ThinVec<libc::c_float>) -> ::nserror::nsresult,

    /* [infallible] readonly attribute ContentFrameMessageManager messageManager; */
    pub GetMessageManager: unsafe extern "system" fn (this: *const nsIDocShell, aMessageManager: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* Promise getHasTrackingContentBlocked (); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub GetHasTrackingContentBlocked: *const ::libc::c_void,

    /* [nostdcall,notxpcom] readonly attribute boolean isAttemptingToNavigate; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetIsAttemptingToNavigate: *const ::libc::c_void,

    /* [infallible] readonly attribute boolean isNavigating; */
    pub GetIsNavigating: unsafe extern "system" fn (this: *const nsIDocShell, aIsNavigating: *mut bool) -> ::nserror::nsresult,

    /* void synchronizeLayoutHistoryState (); */
    pub SynchronizeLayoutHistoryState: unsafe extern "system" fn (this: *const nsIDocShell) -> ::nserror::nsresult,

    /* void persistLayoutHistoryState (); */
    pub PersistLayoutHistoryState: unsafe extern "system" fn (this: *const nsIDocShell) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDocShell {


    /// `void setCancelContentJSEpoch (in long aEpoch);`
    #[inline]
    pub unsafe fn SetCancelContentJSEpoch(&self, aEpoch: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetCancelContentJSEpoch)(self, aEpoch)
    }


    /// ```text
    /// /**
    ///    * Loads a given URI.  This will give priority to loading the requested URI
    ///    * in the object implementing this interface.  If it can't be loaded here
    ///    * however, the URL dispatcher will go through its normal process of content
    ///    * loading.
    ///    *
    ///    * @param aLoadState This is the extended load info for this load.
    ///    * @param aSetNavigating If we should set isNavigating to true while initiating
    ///    *                       the load.
    ///    */
    /// ```
    ///

    /// `[noscript] void loadURI (in nsDocShellLoadStatePtr aLoadState, in boolean aSetNavigating);`
    const _LoadURI: () = ();

    /// ```text
    /// /**
    ///    * Do either a history.pushState() or history.replaceState() operation,
    ///    * depending on the value of aReplace.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] void addState (in jsval aData, in AString aTitle, in AString aURL, in boolean aReplace);`
    const _AddState: () = ();

    /// ```text
    /// /**
    ///    * Helper for addState and document.open that does just the
    ///    * history-manipulation guts.
    ///    *
    ///    * Arguments the spec defines:
    ///    *
    ///    * @param aDocument the document we're manipulating.  This will get the new URI.
    ///    * @param aNewURI the new URI.
    ///    * @param aData The serialized state data.  May be null.
    ///    * @param aTitle The new title.  May be empty.
    ///    * @param aReplace whether this should replace the exising SHEntry.
    ///    *
    ///    * Arguments we need internally because deriving them from the
    ///    * others is a bit complicated:
    ///    *
    ///    * @param aCurrentURI the current URI we're working with.  Might be null.
    ///    * @param aEqualURIs whether the two URIs involved are equal.
    ///    */
    /// ```
    ///

    /// `[nostdcall] void updateURLAndHistory (in Document aDocument, in nsIURI aNewURI, in nsIStructuredCloneContainer aData, in AString aTitle, in boolean aReplace, in nsIURI aCurrentURI, in boolean aEqualURIs);`
    const _UpdateURLAndHistory: () = ();

    /// ```text
    /// /**
    ///    * Reset state to a new content model within the current document and the document
    ///    * viewer.  Called by the document before initiating an out of band document.write().
    ///    */
    /// ```
    ///

    /// `void prepareForNewContentModel ();`
    #[inline]
    pub unsafe fn PrepareForNewContentModel(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).PrepareForNewContentModel)(self, )
    }


    /// ```text
    /// /**
    ///    * For editors and suchlike who wish to change the URI associated with the
    ///    * document. Note if you want to get the current URI, use the read-only
    ///    * property on nsIWebNavigation.
    ///    */
    /// ```
    ///

    /// `void setCurrentURI (in nsIURI aURI);`
    #[inline]
    pub unsafe fn SetCurrentURI(&self, aURI: *const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).SetCurrentURI)(self, aURI)
    }


    /// ```text
    /// /**
    ///    * Notify the associated content viewer and all child docshells that they are
    ///    * about to be hidden.  If |isUnload| is true, then the document is being
    ///    * unloaded and all dynamic subframe history entries are removed as well.
    ///    *
    ///    * @param isUnload
    ///    *        True to fire the unload event in addition to the pagehide event,
    ///    *        and remove all dynamic subframe history entries.
    ///    */
    /// ```
    ///

    /// `[noscript] void firePageHideNotification (in boolean isUnload);`
    #[inline]
    pub unsafe fn FirePageHideNotification(&self, isUnload: bool) -> ::nserror::nsresult {
        ((*self.vtable).FirePageHideNotification)(self, isUnload)
    }


    /// ```text
    /// /**
    ///    * Presentation context for the currently loaded document.  This may be null.
    ///    */
    /// ```
    ///

    /// `[nostdcall,notxpcom] readonly attribute nsPresContext presContext;`
    const _GetPresContext: () = ();

    /// ```text
    /// /**
    ///    * Presentation shell for the currently loaded document.  This may be null.
    ///    */
    /// ```
    ///

    /// `[nostdcall,notxpcom] readonly attribute PresShell presShell;`
    const _GetPresShell: () = ();

    /// ```text
    /// /**
    ///    * Presentation shell for the oldest document, if this docshell is
    ///    * currently transitioning between documents.
    ///    */
    /// ```
    ///

    /// `[nostdcall,notxpcom] readonly attribute PresShell eldestPresShell;`
    const _GetEldestPresShell: () = ();

    /// ```text
    /// /**
    ///    * Content Viewer that is currently loaded for this DocShell.  This may
    ///    * change as the underlying content changes.
    ///    */
    /// ```
    ///

    /// `[infallible] readonly attribute nsIContentViewer contentViewer;`
    #[inline]
    pub unsafe fn GetContentViewer(&self, aContentViewer: *mut*const nsIContentViewer) -> ::nserror::nsresult {
        ((*self.vtable).GetContentViewer)(self, aContentViewer)
    }


    /// ```text
    /// /**
    ///    * Get the id of the outer window that is or will be in this docshell.
    ///    */
    /// ```
    ///

    /// `[infallible] readonly attribute unsigned long long outerWindowID;`
    #[inline]
    pub unsafe fn GetOuterWindowID(&self) -> u64 {
        let mut result = <u64 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetOuterWindowID)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///    * This attribute allows chrome to tie in to handle DOM events that may
    ///    * be of interest to chrome.
    ///    */
    /// ```
    ///

    /// `attribute EventTarget chromeEventHandler;`
    #[inline]
    pub unsafe fn GetChromeEventHandler(&self, aChromeEventHandler: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetChromeEventHandler)(self, aChromeEventHandler)
    }


    /// ```text
    /// /**
    ///    * This attribute allows chrome to tie in to handle DOM events that may
    ///    * be of interest to chrome.
    ///    */
    /// ```
    ///

    /// `attribute EventTarget chromeEventHandler;`
    #[inline]
    pub unsafe fn SetChromeEventHandler(&self, aChromeEventHandler: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).SetChromeEventHandler)(self, aChromeEventHandler)
    }


    /// ```text
    /// /**
    ///    * This allows chrome to set a custom User agent on a specific docshell
    ///    */
    /// ```
    ///

    /// `attribute AString customUserAgent;`
    #[inline]
    pub unsafe fn GetCustomUserAgent(&self, aCustomUserAgent: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetCustomUserAgent)(self, aCustomUserAgent)
    }


    /// ```text
    /// /**
    ///    * This allows chrome to set a custom User agent on a specific docshell
    ///    */
    /// ```
    ///

    /// `attribute AString customUserAgent;`
    #[inline]
    pub unsafe fn SetCustomUserAgent(&self, aCustomUserAgent: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetCustomUserAgent)(self, aCustomUserAgent)
    }


    /// ```text
    /// /**
    ///    * Whether CSS error reporting is enabled.
    ///    */
    /// ```
    ///

    /// `attribute boolean cssErrorReportingEnabled;`
    #[inline]
    pub unsafe fn GetCssErrorReportingEnabled(&self, aCssErrorReportingEnabled: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCssErrorReportingEnabled)(self, aCssErrorReportingEnabled)
    }


    /// ```text
    /// /**
    ///    * Whether CSS error reporting is enabled.
    ///    */
    /// ```
    ///

    /// `attribute boolean cssErrorReportingEnabled;`
    #[inline]
    pub unsafe fn SetCssErrorReportingEnabled(&self, aCssErrorReportingEnabled: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetCssErrorReportingEnabled)(self, aCssErrorReportingEnabled)
    }


    /// ```text
    /// /**
    ///    * Whether to allow plugin execution
    ///    */
    /// ```
    ///

    /// `attribute boolean allowPlugins;`
    #[inline]
    pub unsafe fn GetAllowPlugins(&self, aAllowPlugins: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAllowPlugins)(self, aAllowPlugins)
    }


    /// ```text
    /// /**
    ///    * Whether to allow plugin execution
    ///    */
    /// ```
    ///

    /// `attribute boolean allowPlugins;`
    #[inline]
    pub unsafe fn SetAllowPlugins(&self, aAllowPlugins: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetAllowPlugins)(self, aAllowPlugins)
    }


    /// ```text
    /// /**
    ///    * Whether to allow Javascript execution
    ///    */
    /// ```
    ///

    /// `attribute boolean allowJavascript;`
    #[inline]
    pub unsafe fn GetAllowJavascript(&self, aAllowJavascript: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAllowJavascript)(self, aAllowJavascript)
    }


    /// ```text
    /// /**
    ///    * Whether to allow Javascript execution
    ///    */
    /// ```
    ///

    /// `attribute boolean allowJavascript;`
    #[inline]
    pub unsafe fn SetAllowJavascript(&self, aAllowJavascript: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetAllowJavascript)(self, aAllowJavascript)
    }


    /// ```text
    /// /**
    ///    * Attribute stating if refresh based redirects can be allowed
    ///    */
    /// ```
    ///

    /// `attribute boolean allowMetaRedirects;`
    #[inline]
    pub unsafe fn GetAllowMetaRedirects(&self, aAllowMetaRedirects: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAllowMetaRedirects)(self, aAllowMetaRedirects)
    }


    /// ```text
    /// /**
    ///    * Attribute stating if refresh based redirects can be allowed
    ///    */
    /// ```
    ///

    /// `attribute boolean allowMetaRedirects;`
    #[inline]
    pub unsafe fn SetAllowMetaRedirects(&self, aAllowMetaRedirects: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetAllowMetaRedirects)(self, aAllowMetaRedirects)
    }


    /// ```text
    /// /**
    ///    * Attribute stating if it should allow subframes (framesets/iframes) or not
    ///    */
    /// ```
    ///

    /// `attribute boolean allowSubframes;`
    #[inline]
    pub unsafe fn GetAllowSubframes(&self, aAllowSubframes: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAllowSubframes)(self, aAllowSubframes)
    }


    /// ```text
    /// /**
    ///    * Attribute stating if it should allow subframes (framesets/iframes) or not
    ///    */
    /// ```
    ///

    /// `attribute boolean allowSubframes;`
    #[inline]
    pub unsafe fn SetAllowSubframes(&self, aAllowSubframes: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetAllowSubframes)(self, aAllowSubframes)
    }


    /// ```text
    /// /**
    ///    * Attribute stating whether or not images should be loaded.
    ///    */
    /// ```
    ///

    /// `attribute boolean allowImages;`
    #[inline]
    pub unsafe fn GetAllowImages(&self, aAllowImages: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAllowImages)(self, aAllowImages)
    }


    /// ```text
    /// /**
    ///    * Attribute stating whether or not images should be loaded.
    ///    */
    /// ```
    ///

    /// `attribute boolean allowImages;`
    #[inline]
    pub unsafe fn SetAllowImages(&self, aAllowImages: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetAllowImages)(self, aAllowImages)
    }


    /// ```text
    /// /**
    ///    * Attribute stating whether or not media (audio/video) should be loaded.
    ///    */
    /// ```
    ///

    /// `[infallible] attribute boolean allowMedia;`
    #[inline]
    pub unsafe fn GetAllowMedia(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetAllowMedia)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///    * Attribute stating whether or not media (audio/video) should be loaded.
    ///    */
    /// ```
    ///

    /// `[infallible] attribute boolean allowMedia;`
    #[inline]
    pub unsafe fn SetAllowMedia(&self, aAllowMedia: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetAllowMedia)(self, aAllowMedia)
    }


    /// ```text
    /// /**
    ///    * Attribute that determines whether DNS prefetch is allowed for this subtree
    ///    * of the docshell tree.  Defaults to true.  Setting this will make it take
    ///    * effect starting with the next document loaded in the docshell.
    ///    */
    /// ```
    ///

    /// `attribute boolean allowDNSPrefetch;`
    #[inline]
    pub unsafe fn GetAllowDNSPrefetch(&self, aAllowDNSPrefetch: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAllowDNSPrefetch)(self, aAllowDNSPrefetch)
    }


    /// ```text
    /// /**
    ///    * Attribute that determines whether DNS prefetch is allowed for this subtree
    ///    * of the docshell tree.  Defaults to true.  Setting this will make it take
    ///    * effect starting with the next document loaded in the docshell.
    ///    */
    /// ```
    ///

    /// `attribute boolean allowDNSPrefetch;`
    #[inline]
    pub unsafe fn SetAllowDNSPrefetch(&self, aAllowDNSPrefetch: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetAllowDNSPrefetch)(self, aAllowDNSPrefetch)
    }


    /// ```text
    /// /**
    ///    * Attribute that determines whether window control (move/resize) is allowed.
    ///    */
    /// ```
    ///

    /// `attribute boolean allowWindowControl;`
    #[inline]
    pub unsafe fn GetAllowWindowControl(&self, aAllowWindowControl: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAllowWindowControl)(self, aAllowWindowControl)
    }


    /// ```text
    /// /**
    ///    * Attribute that determines whether window control (move/resize) is allowed.
    ///    */
    /// ```
    ///

    /// `attribute boolean allowWindowControl;`
    #[inline]
    pub unsafe fn SetAllowWindowControl(&self, aAllowWindowControl: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetAllowWindowControl)(self, aAllowWindowControl)
    }


    /// ```text
    /// /**
    ///    * True if the docshell allows its content to be handled by a content listener
    ///    * other than the docshell itself, including the external helper app service,
    ///    * and false otherwise.  Defaults to true.
    ///    */
    /// ```
    ///

    /// `[infallible] attribute boolean allowContentRetargeting;`
    #[inline]
    pub unsafe fn GetAllowContentRetargeting(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetAllowContentRetargeting)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///    * True if the docshell allows its content to be handled by a content listener
    ///    * other than the docshell itself, including the external helper app service,
    ///    * and false otherwise.  Defaults to true.
    ///    */
    /// ```
    ///

    /// `[infallible] attribute boolean allowContentRetargeting;`
    #[inline]
    pub unsafe fn SetAllowContentRetargeting(&self, aAllowContentRetargeting: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetAllowContentRetargeting)(self, aAllowContentRetargeting)
    }


    /// ```text
    /// /**
    ///    * True if new child docshells should allow content retargeting.
    ///    * Setting allowContentRetargeting also overwrites this value.
    ///    */
    /// ```
    ///

    /// `[infallible] attribute boolean allowContentRetargetingOnChildren;`
    #[inline]
    pub unsafe fn GetAllowContentRetargetingOnChildren(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetAllowContentRetargetingOnChildren)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///    * True if new child docshells should allow content retargeting.
    ///    * Setting allowContentRetargeting also overwrites this value.
    ///    */
    /// ```
    ///

    /// `[infallible] attribute boolean allowContentRetargetingOnChildren;`
    #[inline]
    pub unsafe fn SetAllowContentRetargetingOnChildren(&self, aAllowContentRetargetingOnChildren: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetAllowContentRetargetingOnChildren)(self, aAllowContentRetargetingOnChildren)
    }



    /// `Array<nsIDocShell> getAllDocShellsInSubtree (in long aItemType, in nsIDocShell_DocShellEnumeratorDirection aDirection);`
    #[inline]
    pub unsafe fn GetAllDocShellsInSubtree(&self, aItemType: i32, aDirection:  u8, _retval: *mut thin_vec::ThinVec<RefPtr<nsIDocShell>>) -> ::nserror::nsresult {
        ((*self.vtable).GetAllDocShellsInSubtree)(self, aItemType, aDirection, _retval)
    }



    /// `[infallible] attribute nsIDocShell_AppType appType;`
    #[inline]
    pub unsafe fn GetAppType(&self, aAppType: *mut u8) -> ::nserror::nsresult {
        ((*self.vtable).GetAppType)(self, aAppType)
    }



    /// `[infallible] attribute nsIDocShell_AppType appType;`
    #[inline]
    pub unsafe fn SetAppType(&self, aAppType:  u8) -> ::nserror::nsresult {
        ((*self.vtable).SetAppType)(self, aAppType)
    }


    /// ```text
    /// /**
    ///    * certain docshells (like the message pane)
    ///    * should not throw up auth dialogs
    ///    * because it can act as a password trojan
    ///    */
    /// ```
    ///

    /// `attribute boolean allowAuth;`
    #[inline]
    pub unsafe fn GetAllowAuth(&self, aAllowAuth: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAllowAuth)(self, aAllowAuth)
    }


    /// ```text
    /// /**
    ///    * certain docshells (like the message pane)
    ///    * should not throw up auth dialogs
    ///    * because it can act as a password trojan
    ///    */
    /// ```
    ///

    /// `attribute boolean allowAuth;`
    #[inline]
    pub unsafe fn SetAllowAuth(&self, aAllowAuth: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetAllowAuth)(self, aAllowAuth)
    }


    /// ```text
    /// /**
    ///    * Set/Get the document scale factor.  When setting this attribute, a
    ///    * NS_ERROR_NOT_IMPLEMENTED error may be returned by implementations
    ///    * not supporting zoom.  Implementations not supporting zoom should return
    ///    * 1.0 all the time for the Get operation.  1.0 by the way is the default
    ///    * of zoom.  This means 100% of normal scaling or in other words normal size
    ///    * no zoom.
    ///    */
    /// ```
    ///

    /// `attribute float zoom;`
    #[inline]
    pub unsafe fn GetZoom(&self, aZoom: *mut libc::c_float) -> ::nserror::nsresult {
        ((*self.vtable).GetZoom)(self, aZoom)
    }


    /// ```text
    /// /**
    ///    * Set/Get the document scale factor.  When setting this attribute, a
    ///    * NS_ERROR_NOT_IMPLEMENTED error may be returned by implementations
    ///    * not supporting zoom.  Implementations not supporting zoom should return
    ///    * 1.0 all the time for the Get operation.  1.0 by the way is the default
    ///    * of zoom.  This means 100% of normal scaling or in other words normal size
    ///    * no zoom.
    ///    */
    /// ```
    ///

    /// `attribute float zoom;`
    #[inline]
    pub unsafe fn SetZoom(&self, aZoom: libc::c_float) -> ::nserror::nsresult {
        ((*self.vtable).SetZoom)(self, aZoom)
    }



    /// `bool tabToTreeOwner (in boolean forward, in boolean forDocumentNavigation);`
    #[inline]
    pub unsafe fn TabToTreeOwner(&self, forward: bool, forDocumentNavigation: bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).TabToTreeOwner)(self, forward, forDocumentNavigation, _retval)
    }



    /// `[infallible] readonly attribute nsIDocShell_BusyFlags busyFlags;`
    #[inline]
    pub unsafe fn GetBusyFlags(&self, aBusyFlags: *mut u8) -> ::nserror::nsresult {
        ((*self.vtable).GetBusyFlags)(self, aBusyFlags)
    }



    /// `[infallible] attribute unsigned long loadType;`
    #[inline]
    pub unsafe fn GetLoadType(&self) -> u32 {
        let mut result = <u32 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetLoadType)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `[infallible] attribute unsigned long loadType;`
    #[inline]
    pub unsafe fn SetLoadType(&self, aLoadType: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetLoadType)(self, aLoadType)
    }



    /// `attribute nsLoadFlags defaultLoadFlags;`
    #[inline]
    pub unsafe fn GetDefaultLoadFlags(&self, aDefaultLoadFlags: *mut nsLoadFlags) -> ::nserror::nsresult {
        ((*self.vtable).GetDefaultLoadFlags)(self, aDefaultLoadFlags)
    }



    /// `attribute nsLoadFlags defaultLoadFlags;`
    #[inline]
    pub unsafe fn SetDefaultLoadFlags(&self, aDefaultLoadFlags: nsLoadFlags) -> ::nserror::nsresult {
        ((*self.vtable).SetDefaultLoadFlags)(self, aDefaultLoadFlags)
    }



    /// `boolean isBeingDestroyed ();`
    #[inline]
    pub unsafe fn IsBeingDestroyed(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsBeingDestroyed)(self, _retval)
    }



    /// `readonly attribute boolean isExecutingOnLoadHandler;`
    #[inline]
    pub unsafe fn GetIsExecutingOnLoadHandler(&self, aIsExecutingOnLoadHandler: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsExecutingOnLoadHandler)(self, aIsExecutingOnLoadHandler)
    }



    /// `attribute nsILayoutHistoryState layoutHistoryState;`
    #[inline]
    pub unsafe fn GetLayoutHistoryState(&self, aLayoutHistoryState: *mut*const nsILayoutHistoryState) -> ::nserror::nsresult {
        ((*self.vtable).GetLayoutHistoryState)(self, aLayoutHistoryState)
    }



    /// `attribute nsILayoutHistoryState layoutHistoryState;`
    #[inline]
    pub unsafe fn SetLayoutHistoryState(&self, aLayoutHistoryState: *const nsILayoutHistoryState) -> ::nserror::nsresult {
        ((*self.vtable).SetLayoutHistoryState)(self, aLayoutHistoryState)
    }


    /// ```text
    /// /**
    ///    * Object used to delegate URI loading to an upper context.
    ///    * Currently only set for GeckoView to allow handling of load requests
    ///    * at the application level.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsILoadURIDelegate loadURIDelegate;`
    #[inline]
    pub unsafe fn GetLoadURIDelegate(&self, aLoadURIDelegate: *mut*const nsILoadURIDelegate) -> ::nserror::nsresult {
        ((*self.vtable).GetLoadURIDelegate)(self, aLoadURIDelegate)
    }


    /// ```text
    /// /**
    ///    * Cancel the XPCOM timers for each meta-refresh URI in this docshell,
    ///    * and this docshell's children, recursively. The meta-refresh timers can be
    ///    * restarted using resumeRefreshURIs().  If the timers are already suspended,
    ///    * this has no effect.
    ///    */
    /// ```
    ///

    /// `void suspendRefreshURIs ();`
    #[inline]
    pub unsafe fn SuspendRefreshURIs(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SuspendRefreshURIs)(self, )
    }


    /// ```text
    /// /**
    ///    * Restart the XPCOM timers for each meta-refresh URI in this docshell,
    ///    * and this docshell's children, recursively.  If the timers are already
    ///    * running, this has no effect.
    ///    */
    /// ```
    ///

    /// `void resumeRefreshURIs ();`
    #[inline]
    pub unsafe fn ResumeRefreshURIs(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ResumeRefreshURIs)(self, )
    }


    /// ```text
    /// /**
    ///    * Begin firing WebProgressListener notifications for restoring a page
    ///    * presentation. |viewer| is the content viewer whose document we are
    ///    * starting to load.  If null, it defaults to the docshell's current content
    ///    * viewer, creating one if necessary.  |top| should be true for the toplevel
    ///    * docshell that is being restored; it will be set to false when this method
    ///    * is called for child docshells.  This method will post an event to
    ///    * complete the simulated load after returning to the event loop.
    ///    */
    /// ```
    ///

    /// `void beginRestore (in nsIContentViewer viewer, in boolean top);`
    #[inline]
    pub unsafe fn BeginRestore(&self, viewer: *const nsIContentViewer, top: bool) -> ::nserror::nsresult {
        ((*self.vtable).BeginRestore)(self, viewer, top)
    }


    /// ```text
    /// /**
    ///    * Finish firing WebProgressListener notifications and DOM events for
    ///    * restoring a page presentation.  This should only be called via
    ///    * beginRestore().
    ///    */
    /// ```
    ///

    /// `void finishRestore ();`
    #[inline]
    pub unsafe fn FinishRestore(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).FinishRestore)(self, )
    }



    /// `void clearCachedUserAgent ();`
    #[inline]
    pub unsafe fn ClearCachedUserAgent(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ClearCachedUserAgent)(self, )
    }



    /// `void clearCachedPlatform ();`
    #[inline]
    pub unsafe fn ClearCachedPlatform(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ClearCachedPlatform)(self, )
    }



    /// `readonly attribute boolean restoringDocument;`
    #[inline]
    pub unsafe fn GetRestoringDocument(&self, aRestoringDocument: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetRestoringDocument)(self, aRestoringDocument)
    }



    /// `attribute boolean useErrorPages;`
    #[inline]
    pub unsafe fn GetUseErrorPages(&self, aUseErrorPages: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetUseErrorPages)(self, aUseErrorPages)
    }



    /// `attribute boolean useErrorPages;`
    #[inline]
    pub unsafe fn SetUseErrorPages(&self, aUseErrorPages: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetUseErrorPages)(self, aUseErrorPages)
    }


    /// ```text
    /// /**
    ///    * Display a load error in a frame while keeping that frame's currentURI
    ///    * pointing correctly to the page where the error ocurred, rather than to
    ///    * the error document page. You must provide either the aURI or aURL parameter.
    ///    *
    ///    * @param  aError         The error code to be displayed
    ///    * @param  aURI           nsIURI of the page where the error happened
    ///    * @param  aURL           wstring of the page where the error happened
    ///    * @param  aFailedChannel The channel related to this error
    ///    *
    ///    * Returns whether or not we displayed an error page (note: will always
        ///    * return false if in-content error pages are disabled!)
    ///    */
    /// ```
    ///

    /// `boolean displayLoadError (in nsresult aError, in nsIURI aURI, in wstring aURL, [optional] in nsIChannel aFailedChannel);`
    #[inline]
    pub unsafe fn DisplayLoadError(&self, aError: ::nserror::nsresult, aURI: *const nsIURI, aURL: *const i16, aFailedChannel: *const nsIChannel, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).DisplayLoadError)(self, aError, aURI, aURL, aFailedChannel, _retval)
    }


    /// ```text
    /// /**
    ///    * The channel that failed to load and resulted in an error page.
    ///    * May be null. Relevant only to error pages.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIChannel failedChannel;`
    #[inline]
    pub unsafe fn GetFailedChannel(&self, aFailedChannel: *mut*const nsIChannel) -> ::nserror::nsresult {
        ((*self.vtable).GetFailedChannel)(self, aFailedChannel)
    }


    /// ```text
    /// /**
    ///    * Keeps track of the previous nsISHEntry index and the current
    ///    * nsISHEntry index at the time that the doc shell begins to load.
    ///    * Used for ContentViewer eviction.
    ///    */
    /// ```
    ///

    /// `readonly attribute long previousEntryIndex;`
    #[inline]
    pub unsafe fn GetPreviousEntryIndex(&self, aPreviousEntryIndex: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetPreviousEntryIndex)(self, aPreviousEntryIndex)
    }



    /// `readonly attribute long loadedEntryIndex;`
    #[inline]
    pub unsafe fn GetLoadedEntryIndex(&self, aLoadedEntryIndex: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetLoadedEntryIndex)(self, aLoadedEntryIndex)
    }


    /// ```text
    /// /**
    ///    * Notification that entries have been removed from the beginning of a
    ///    * nsSHistory which has this as its rootDocShell.
    ///    *
    ///    * @param numEntries - The number of entries removed
    ///    */
    /// ```
    ///

    /// `void historyPurged (in long numEntries);`
    #[inline]
    pub unsafe fn HistoryPurged(&self, numEntries: i32) -> ::nserror::nsresult {
        ((*self.vtable).HistoryPurged)(self, numEntries)
    }


    /// ```text
    /// /**
    ///    * Gets the channel for the currently loaded document, if any.
    ///    * For a new document load, this will be the channel of the previous document
    ///    * until after OnLocationChange fires.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIChannel currentDocumentChannel;`
    #[inline]
    pub unsafe fn GetCurrentDocumentChannel(&self, aCurrentDocumentChannel: *mut*const nsIChannel) -> ::nserror::nsresult {
        ((*self.vtable).GetCurrentDocumentChannel)(self, aCurrentDocumentChannel)
    }


    /// ```text
    /// /**
    ///    * The original offset of this child in its container. This property is -1 for
    ///    * dynamically added docShells.
    ///    */
    /// ```
    ///

    /// `[nostdcall,notxpcom] attribute long childOffset;`
    const _GetChildOffset: () = ();

    /// ```text
    /// /**
    ///    * The original offset of this child in its container. This property is -1 for
    ///    * dynamically added docShells.
    ///    */
    /// ```
    ///

    /// `[nostdcall,notxpcom] attribute long childOffset;`
    const _SetChildOffset: () = ();

    /// ```text
    /// /**
    ///    * Find out whether the docshell is currently in the middle of a page
    ///    * transition. This is set just before the pagehide/unload events fire.
    ///    */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean isInUnload;`
    #[inline]
    pub unsafe fn GetIsInUnload(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetIsInUnload)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///    * Disconnects this docshell's editor from its window, and stores the
    ///    * editor data in the open document's session history entry.  This
    ///    * should be called only during page transitions.
    ///    */
    /// ```
    ///

    /// `[noscript,notxpcom] void DetachEditorFromWindow ();`
    #[inline]
    pub unsafe fn DetachEditorFromWindow(&self, ) -> libc::c_void {
        ((*self.vtable).DetachEditorFromWindow)(self, )
    }


    /// ```text
    /// /**
    ///    * If true, this browser is not visible in the traditional sense, but
    ///    * is actively being rendered to the screen (ex. painted on a canvas)
    ///    * and should be treated accordingly.
    ///    **/
    /// ```
    ///

    /// `attribute boolean isOffScreenBrowser;`
    #[inline]
    pub unsafe fn GetIsOffScreenBrowser(&self, aIsOffScreenBrowser: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsOffScreenBrowser)(self, aIsOffScreenBrowser)
    }


    /// ```text
    /// /**
    ///    * If true, this browser is not visible in the traditional sense, but
    ///    * is actively being rendered to the screen (ex. painted on a canvas)
    ///    * and should be treated accordingly.
    ///    **/
    /// ```
    ///

    /// `attribute boolean isOffScreenBrowser;`
    #[inline]
    pub unsafe fn SetIsOffScreenBrowser(&self, aIsOffScreenBrowser: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetIsOffScreenBrowser)(self, aIsOffScreenBrowser)
    }


    /// ```text
    /// /**
    ///    * Propagated to the print preview document viewer.  Must only be called on
    ///    * a document viewer that has been initialized for print preview.
    ///    */
    /// ```
    ///

    /// `void exitPrintPreview ();`
    #[inline]
    pub unsafe fn ExitPrintPreview(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ExitPrintPreview)(self, )
    }


    /// ```text
    /// /**
    ///    * Whether this docshell can execute scripts based on its hierarchy.
    ///    * The rule of thumb here is that we disable js if this docshell or any
    ///    * of its parents disallow scripting.
    ///    */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean canExecuteScripts;`
    #[inline]
    pub unsafe fn GetCanExecuteScripts(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetCanExecuteScripts)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///    * The ID of the docshell in the session history.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIDRef historyID;`
    #[inline]
    pub unsafe fn GetHistoryID(&self, aHistoryID: *mut nsID) -> ::nserror::nsresult {
        ((*self.vtable).GetHistoryID)(self, aHistoryID)
    }


    /// ```text
    /// /**
    ///    * Helper method for accessing this value from C++
    ///    */
    /// ```
    ///

    /// `[noscript,notxpcom] nsIDRef HistoryID ();`
    #[inline]
    pub unsafe fn HistoryID(&self, ) -> *const nsID {
        ((*self.vtable).HistoryID)(self, )
    }


    /// ```text
    /// /**
    ///    * Sets whether a docshell is an app tab. An app tab docshell may behave
    ///    * differently than a non-app tab docshell in some cases, such as when
    ///    * handling link clicks. Docshells are not app tabs unless told otherwise.
    ///    */
    /// ```
    ///

    /// `attribute boolean isAppTab;`
    #[inline]
    pub unsafe fn GetIsAppTab(&self, aIsAppTab: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsAppTab)(self, aIsAppTab)
    }


    /// ```text
    /// /**
    ///    * Sets whether a docshell is an app tab. An app tab docshell may behave
    ///    * differently than a non-app tab docshell in some cases, such as when
    ///    * handling link clicks. Docshells are not app tabs unless told otherwise.
    ///    */
    /// ```
    ///

    /// `attribute boolean isAppTab;`
    #[inline]
    pub unsafe fn SetIsAppTab(&self, aIsAppTab: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetIsAppTab)(self, aIsAppTab)
    }


    /// ```text
    /// /**
    ///    * Create a new about:blank document and content viewer.
    ///    * @param aPrincipal the principal to use for the new document.
    ///    * @param aPartitionedPrincipal the partitioned principal to use for the new
    ///    *        document.
    ///    * @param aCsp the CSP to use for the new document.
    ///    */
    /// ```
    ///

    /// `void createAboutBlankContentViewer (in nsIPrincipal aPrincipal, in nsIPrincipal aPartitionedPrincipal, [optional] in nsIContentSecurityPolicy aCSP);`
    #[inline]
    pub unsafe fn CreateAboutBlankContentViewer(&self, aPrincipal: *const nsIPrincipal, aPartitionedPrincipal: *const nsIPrincipal, aCSP: *const nsIContentSecurityPolicy) -> ::nserror::nsresult {
        ((*self.vtable).CreateAboutBlankContentViewer)(self, aPrincipal, aPartitionedPrincipal, aCSP)
    }


    /// ```text
    /// /**
    ///    * Upon getting, returns the canonical encoding label of the document
    ///    * currently loaded into this docshell.
    ///    *
    ///    * Upon setting, sets the forced encoding for compatibility with legacy callers.
    ///    */
    /// ```
    ///

    /// `attribute ACString charset;`
    #[inline]
    pub unsafe fn GetCharset(&self, aCharset: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetCharset)(self, aCharset)
    }


    /// ```text
    /// /**
    ///    * Upon getting, returns the canonical encoding label of the document
    ///    * currently loaded into this docshell.
    ///    *
    ///    * Upon setting, sets the forced encoding for compatibility with legacy callers.
    ///    */
    /// ```
    ///

    /// `attribute ACString charset;`
    #[inline]
    pub unsafe fn SetCharset(&self, aCharset: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetCharset)(self, aCharset)
    }


    /// ```text
    /// /**
    ///    * Called when the user chose an encoding override from the character
    ///    * encoding menu. Separate from the setter for the charset property to avoid
    ///    * extensions adding noise to the data.
    ///    */
    /// ```
    ///

    /// `void gatherCharsetMenuTelemetry ();`
    #[inline]
    pub unsafe fn GatherCharsetMenuTelemetry(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).GatherCharsetMenuTelemetry)(self, )
    }


    /// ```text
    /// /**
    ///    * In a child docshell, this is the charset of the parent docshell
    ///    */
    /// ```
    ///

    /// `[noscript,nostdcall,notxpcom] void setParentCharset (in Encoding parentCharset, in int32_t parentCharsetSource, in nsIPrincipal parentCharsetPrincipal);`
    const _SetParentCharset: () = ();


    /// `[noscript,nostdcall,notxpcom] void getParentCharset (out Encoding parentCharset, out int32_t parentCharsetSource, out nsIPrincipal parentCharsetPrincipal);`
    const _GetParentCharset: () = ();

    /// ```text
    /// /**
    ///    * Whether the docShell records profile timeline markers at the moment
    ///    */
    /// ```
    ///

    /// `[infallible] attribute boolean recordProfileTimelineMarkers;`
    #[inline]
    pub unsafe fn GetRecordProfileTimelineMarkers(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetRecordProfileTimelineMarkers)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///    * Whether the docShell records profile timeline markers at the moment
    ///    */
    /// ```
    ///

    /// `[infallible] attribute boolean recordProfileTimelineMarkers;`
    #[inline]
    pub unsafe fn SetRecordProfileTimelineMarkers(&self, aRecordProfileTimelineMarkers: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetRecordProfileTimelineMarkers)(self, aRecordProfileTimelineMarkers)
    }


    /// ```text
    /// /**
    ///    * Return a DOMHighResTimeStamp representing the number of
    ///    * milliseconds from an arbitrary point in time.  The reference
    ///    * point is shared by all DocShells and is also used by timestamps
    ///    * on markers.
    ///    */
    /// ```
    ///

    /// `DOMHighResTimeStamp now ();`
    #[inline]
    pub unsafe fn Now(&self, _retval: *mut DOMHighResTimeStamp) -> ::nserror::nsresult {
        ((*self.vtable).Now)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns and flushes the profile timeline markers gathered by the docShell
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] jsval popProfileTimelineMarkers ();`
    const _PopProfileTimelineMarkers: () = ();

    /// ```text
    /// /**
    ///    * Add an observer to the list of parties to be notified when this docshell's
    ///    * private browsing status is changed. |obs| must support weak references.
    ///    */
    /// ```
    ///

    /// `void addWeakPrivacyTransitionObserver (in nsIPrivacyTransitionObserver obs);`
    #[inline]
    pub unsafe fn AddWeakPrivacyTransitionObserver(&self, obs: *const nsIPrivacyTransitionObserver) -> ::nserror::nsresult {
        ((*self.vtable).AddWeakPrivacyTransitionObserver)(self, obs)
    }


    /// ```text
    /// /**
    ///    * Add an observer to the list of parties to be notified when reflows are
    ///    * occurring. |obs| must support weak references.
    ///    */
    /// ```
    ///

    /// `void addWeakReflowObserver (in nsIReflowObserver obs);`
    #[inline]
    pub unsafe fn AddWeakReflowObserver(&self, obs: *const nsIReflowObserver) -> ::nserror::nsresult {
        ((*self.vtable).AddWeakReflowObserver)(self, obs)
    }


    /// ```text
    /// /**
    ///    * Remove an observer from the list of parties to be notified about reflows.
    ///    */
    /// ```
    ///

    /// `void removeWeakReflowObserver (in nsIReflowObserver obs);`
    #[inline]
    pub unsafe fn RemoveWeakReflowObserver(&self, obs: *const nsIReflowObserver) -> ::nserror::nsresult {
        ((*self.vtable).RemoveWeakReflowObserver)(self, obs)
    }


    /// ```text
    /// /**
    ///    * Notify all attached observers that a reflow has just occurred.
    ///    *
    ///    * @param interruptible if true, the reflow was interruptible.
    ///    * @param start         timestamp when reflow started, in milliseconds since
    ///    *                      navigationStart (accurate to 1/1000 of a ms)
    ///    * @param end           timestamp when reflow ended, in milliseconds since
    ///    *                      navigationStart (accurate to 1/1000 of a ms)
    ///    */
    /// ```
    ///

    /// `[noscript] void notifyReflowObservers (in bool interruptible, in DOMHighResTimeStamp start, in DOMHighResTimeStamp end);`
    #[inline]
    pub unsafe fn NotifyReflowObservers(&self, interruptible: bool, start: DOMHighResTimeStamp, end: DOMHighResTimeStamp) -> ::nserror::nsresult {
        ((*self.vtable).NotifyReflowObservers)(self, interruptible, start, end)
    }


    /// ```text
    /// /**
    ///    * Add an observer to the list of parties to be notified when scroll position
    ///    * of some elements is changed.
    ///    */
    /// ```
    ///

    /// `[noscript] void addWeakScrollObserver (in nsIScrollObserver obs);`
    #[inline]
    pub unsafe fn AddWeakScrollObserver(&self, obs: *const nsIScrollObserver) -> ::nserror::nsresult {
        ((*self.vtable).AddWeakScrollObserver)(self, obs)
    }


    /// ```text
    /// /**
    ///    * Add an observer to the list of parties to be notified when scroll position
    ///    * of some elements is changed.
    ///    */
    /// ```
    ///

    /// `[noscript] void removeWeakScrollObserver (in nsIScrollObserver obs);`
    #[inline]
    pub unsafe fn RemoveWeakScrollObserver(&self, obs: *const nsIScrollObserver) -> ::nserror::nsresult {
        ((*self.vtable).RemoveWeakScrollObserver)(self, obs)
    }


    /// ```text
    /// /**
    ///    * Notify all attached observers that the scroll position of some element
    ///    * has changed.
    ///    */
    /// ```
    ///

    /// `[noscript] void notifyScrollObservers ();`
    #[inline]
    pub unsafe fn NotifyScrollObservers(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).NotifyScrollObservers)(self, )
    }


    /// ```text
    /// /**
    ///    * Returns true if this docshell is the top level content docshell.
    ///    */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean isTopLevelContentDocShell;`
    #[inline]
    pub unsafe fn GetIsTopLevelContentDocShell(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetIsTopLevelContentDocShell)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///    * Like nsIDocShellTreeItem::GetSameTypeParent, except this ignores <iframe
    ///    * mozbrowser> boundaries. Which no longer exist.
    ///    *
    ///    * @deprecated: Use `BrowsingContext::GetParent()` in the future.
    ///    */
    /// ```
    ///

    /// `nsIDocShell getSameTypeInProcessParentIgnoreBrowserBoundaries ();`
    #[inline]
    pub unsafe fn GetSameTypeInProcessParentIgnoreBrowserBoundaries(&self, _retval: *mut *const nsIDocShell) -> ::nserror::nsresult {
        ((*self.vtable).GetSameTypeInProcessParentIgnoreBrowserBoundaries)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * True iff asynchronous panning and zooming is enabled for this
    ///    * docshell.
    ///    */
    /// ```
    ///

    /// `readonly attribute bool asyncPanZoomEnabled;`
    #[inline]
    pub unsafe fn GetAsyncPanZoomEnabled(&self, aAsyncPanZoomEnabled: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAsyncPanZoomEnabled)(self, aAsyncPanZoomEnabled)
    }


    /// ```text
    /// /**
    ///    * This member variable determines whether a document has Mixed Active Content that
    ///    * was initially blocked from loading, but the user has choosen to override the
    ///    * block and allow the content to load. mMixedContentChannel is set to the document's
    ///    * channel when the user allows mixed content. The nsMixedContentBlocker content policy
    ///    * checks if the document's root channel matches the mMixedContentChannel.  If it matches,
    ///    * then Mixed Content is loaded.  If it does match, mixed content is blocked.
    ///    *
    ///    * A match implies that there is definitely mixed active content on a page that was
    ///    * initially blocked by nsMixedContentBlocker and then allowed and loaded by the user.
    ///    * A miss imples that IF there is mixed active content on the page AND it was
    ///    * blocked by nsMixedContentBlocker.cpp, the user has not choosen to override
    ///    * the block. Note that if the about:config setting
    ///    * security.mixed_content.block_active_content is set to false, this boolean
    ///    * will be false, mMixedContentChannel will remain null since blocking active content has
    ///    * been disabled and hence mMixedContentChannel will never be set.
    ///    */
    /// ```
    ///

    /// `attribute nsIChannel mixedContentChannel;`
    #[inline]
    pub unsafe fn GetMixedContentChannel(&self, aMixedContentChannel: *mut*const nsIChannel) -> ::nserror::nsresult {
        ((*self.vtable).GetMixedContentChannel)(self, aMixedContentChannel)
    }


    /// ```text
    /// /**
    ///    * This member variable determines whether a document has Mixed Active Content that
    ///    * was initially blocked from loading, but the user has choosen to override the
    ///    * block and allow the content to load. mMixedContentChannel is set to the document's
    ///    * channel when the user allows mixed content. The nsMixedContentBlocker content policy
    ///    * checks if the document's root channel matches the mMixedContentChannel.  If it matches,
    ///    * then Mixed Content is loaded.  If it does match, mixed content is blocked.
    ///    *
    ///    * A match implies that there is definitely mixed active content on a page that was
    ///    * initially blocked by nsMixedContentBlocker and then allowed and loaded by the user.
    ///    * A miss imples that IF there is mixed active content on the page AND it was
    ///    * blocked by nsMixedContentBlocker.cpp, the user has not choosen to override
    ///    * the block. Note that if the about:config setting
    ///    * security.mixed_content.block_active_content is set to false, this boolean
    ///    * will be false, mMixedContentChannel will remain null since blocking active content has
    ///    * been disabled and hence mMixedContentChannel will never be set.
    ///    */
    /// ```
    ///

    /// `attribute nsIChannel mixedContentChannel;`
    #[inline]
    pub unsafe fn SetMixedContentChannel(&self, aMixedContentChannel: *const nsIChannel) -> ::nserror::nsresult {
        ((*self.vtable).SetMixedContentChannel)(self, aMixedContentChannel)
    }


    /// ```text
    /// /**
    ///    * Are plugins allowed in the current document loaded in this docshell ?
    ///    * (if there is one). This depends on whether plugins are allowed by this
    ///    * docshell itself or if the document is sandboxed and hence plugins should
    ///    * not be allowed.
    ///    */
    /// ```
    ///

    /// `[noscript,notxpcom] bool pluginsAllowedInCurrentDoc ();`
    #[inline]
    pub unsafe fn PluginsAllowedInCurrentDoc(&self, ) -> bool {
        ((*self.vtable).PluginsAllowedInCurrentDoc)(self, )
    }



    /// `[infallible,noscript] attribute boolean affectPrivateSessionLifetime;`
    #[inline]
    pub unsafe fn GetAffectPrivateSessionLifetime(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetAffectPrivateSessionLifetime)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `[infallible,noscript] attribute boolean affectPrivateSessionLifetime;`
    #[inline]
    pub unsafe fn SetAffectPrivateSessionLifetime(&self, aAffectPrivateSessionLifetime: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetAffectPrivateSessionLifetime)(self, aAffectPrivateSessionLifetime)
    }


    /// ```text
    /// /**
    ///    * Indicates whether the UI may enable the character encoding menu. The UI
    ///    * must disable the menu when this property is false.
    ///    */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean mayEnableCharacterEncodingMenu;`
    #[inline]
    pub unsafe fn GetMayEnableCharacterEncodingMenu(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetMayEnableCharacterEncodingMenu)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///    * Indicates that the character encoding was autodetected.
    ///    */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean charsetAutodetected;`
    #[inline]
    pub unsafe fn GetCharsetAutodetected(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetCharsetAutodetected)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `attribute nsIEditor editor;`
    #[inline]
    pub unsafe fn GetEditor(&self, aEditor: *mut*const nsIEditor) -> ::nserror::nsresult {
        ((*self.vtable).GetEditor)(self, aEditor)
    }



    /// `attribute nsIEditor editor;`
    #[inline]
    pub unsafe fn SetEditor(&self, aEditor: *const nsIEditor) -> ::nserror::nsresult {
        ((*self.vtable).SetEditor)(self, aEditor)
    }



    /// `readonly attribute boolean editable;`
    #[inline]
    pub unsafe fn GetEditable(&self, aEditable: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetEditable)(self, aEditable)
    }



    /// `readonly attribute boolean hasEditingSession;`
    #[inline]
    pub unsafe fn GetHasEditingSession(&self, aHasEditingSession: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetHasEditingSession)(self, aHasEditingSession)
    }


    /// ```text
    /// /**
    ///    * Make this docShell editable, setting a flag that causes
    ///    * an editor to get created, either immediately, or after
    ///    * a url has been loaded.
    ///    *      @param  inWaitForUriLoad    true to wait for a URI before
    ///    *                                  creating the editor.
    ///    */
    /// ```
    ///

    /// `void makeEditable (in boolean inWaitForUriLoad);`
    #[inline]
    pub unsafe fn MakeEditable(&self, inWaitForUriLoad: bool) -> ::nserror::nsresult {
        ((*self.vtable).MakeEditable)(self, inWaitForUriLoad)
    }


    /// ```text
    /// /**
    ///    * Returns false for mLSHE, true for mOSHE
    ///    */
    /// ```
    ///

    /// `boolean getCurrentSHEntry (out nsISHEntry aEntry);`
    #[inline]
    pub unsafe fn GetCurrentSHEntry(&self, aEntry: *mut*const nsISHEntry, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCurrentSHEntry)(self, aEntry, _retval)
    }


    /// ```text
    /// /**
    ///     * Cherry picked parts of nsIController.
    ///     * They are here, because we want to call these functions
    ///     * from JS.
    ///     */
    /// ```
    ///

    /// `boolean isCommandEnabled (in string command);`
    #[inline]
    pub unsafe fn IsCommandEnabled(&self, command: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsCommandEnabled)(self, command, _retval)
    }



    /// `[can_run_script] void doCommand (in string command);`
    #[inline]
    pub unsafe fn DoCommand(&self, command: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).DoCommand)(self, command)
    }



    /// `[can_run_script] void doCommandWithParams (in string command, in nsICommandParams aParams);`
    #[inline]
    pub unsafe fn DoCommandWithParams(&self, command: *const libc::c_char, aParams: *const nsICommandParams) -> ::nserror::nsresult {
        ((*self.vtable).DoCommandWithParams)(self, command, aParams)
    }


    /// ```text
    /// /**
    ///    * Invisible DocShell are dummy construct to simulate DOM windows
    ///    * without any actual visual representation. They have to be marked
    ///    * at construction time, to avoid any painting activity.
    ///    */
    /// ```
    ///

    /// `[noscript,notxpcom] bool IsInvisible ();`
    #[inline]
    pub unsafe fn IsInvisible(&self, ) -> bool {
        ((*self.vtable).IsInvisible)(self, )
    }



    /// `[noscript,notxpcom] void SetInvisible (in bool aIsInvisibleDocshell);`
    #[inline]
    pub unsafe fn SetInvisible(&self, aIsInvisibleDocshell: bool) -> libc::c_void {
        ((*self.vtable).SetInvisible)(self, aIsInvisibleDocshell)
    }


    /// ```text
    /// /**
    ///  * Get the script global for the document in this docshell.
    /// */
    /// ```
    ///

    /// `[noscript,nostdcall,notxpcom] nsIScriptGlobalObject GetScriptGlobalObject ();`
    const _GetScriptGlobalObject: () = ();


    /// `[noscript,nostdcall,notxpcom] Document getExtantDocument ();`
    const _GetExtantDocument: () = ();

    /// ```text
    /// /**
    ///    * If deviceSizeIsPageSize is set to true, device-width/height media queries
    ///    * will be calculated from the page size, not the device size.
    ///    *
    ///    * Used by the Responsive Design Mode and B2G Simulator.
    ///    *
    ///    * Default is False.
    ///    * Default value can be overriden with
    ///    * docshell.device_size_is_page_size pref.
    ///    */
    /// ```
    ///

    /// `[infallible] attribute boolean deviceSizeIsPageSize;`
    #[inline]
    pub unsafe fn GetDeviceSizeIsPageSize(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetDeviceSizeIsPageSize)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///    * If deviceSizeIsPageSize is set to true, device-width/height media queries
    ///    * will be calculated from the page size, not the device size.
    ///    *
    ///    * Used by the Responsive Design Mode and B2G Simulator.
    ///    *
    ///    * Default is False.
    ///    * Default value can be overriden with
    ///    * docshell.device_size_is_page_size pref.
    ///    */
    /// ```
    ///

    /// `[infallible] attribute boolean deviceSizeIsPageSize;`
    #[inline]
    pub unsafe fn SetDeviceSizeIsPageSize(&self, aDeviceSizeIsPageSize: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetDeviceSizeIsPageSize)(self, aDeviceSizeIsPageSize)
    }


    /// ```text
    /// /**
    ///    * Notify DocShell when the browser is about to start executing JS, and after
    ///    * that execution has stopped.  This only occurs when the Timeline devtool
    ///    * is collecting information.
    ///    */
    /// ```
    ///

    /// `[noscript,nostdcall,notxpcom] void notifyJSRunToCompletionStart (in string aReason, in AString functionName, in AString fileName, in unsigned long lineNumber, in jsval asyncStack, in string asyncCause);`
    const _NotifyJSRunToCompletionStart: () = ();


    /// `[noscript,nostdcall,notxpcom] void notifyJSRunToCompletionStop ();`
    const _NotifyJSRunToCompletionStop: () = ();

    /// ```text
    /// /**
    ///    * This attribute determines whether a document which is not about:blank has
    ///    * already be loaded by this docShell.
    ///    */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean hasLoadedNonBlankURI;`
    #[inline]
    pub unsafe fn GetHasLoadedNonBlankURI(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetHasLoadedNonBlankURI)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///    * Allow usage of -moz-window-dragging:drag for content docshells.
    ///    * True for top level chrome docshells. Throws if set to false with
    ///    * top level chrome docshell.
    ///    */
    /// ```
    ///

    /// `attribute boolean windowDraggingAllowed;`
    #[inline]
    pub unsafe fn GetWindowDraggingAllowed(&self, aWindowDraggingAllowed: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetWindowDraggingAllowed)(self, aWindowDraggingAllowed)
    }


    /// ```text
    /// /**
    ///    * Allow usage of -moz-window-dragging:drag for content docshells.
    ///    * True for top level chrome docshells. Throws if set to false with
    ///    * top level chrome docshell.
    ///    */
    /// ```
    ///

    /// `attribute boolean windowDraggingAllowed;`
    #[inline]
    pub unsafe fn SetWindowDraggingAllowed(&self, aWindowDraggingAllowed: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetWindowDraggingAllowed)(self, aWindowDraggingAllowed)
    }


    /// ```text
    /// /**
    ///    * Sets/gets the current scroll restoration mode.
    ///    * @see https://html.spec.whatwg.org/#dom-history-scroll-restoration
    ///    */
    /// ```
    ///

    /// `attribute boolean currentScrollRestorationIsManual;`
    #[inline]
    pub unsafe fn GetCurrentScrollRestorationIsManual(&self, aCurrentScrollRestorationIsManual: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCurrentScrollRestorationIsManual)(self, aCurrentScrollRestorationIsManual)
    }


    /// ```text
    /// /**
    ///    * Sets/gets the current scroll restoration mode.
    ///    * @see https://html.spec.whatwg.org/#dom-history-scroll-restoration
    ///    */
    /// ```
    ///

    /// `attribute boolean currentScrollRestorationIsManual;`
    #[inline]
    pub unsafe fn SetCurrentScrollRestorationIsManual(&self, aCurrentScrollRestorationIsManual: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetCurrentScrollRestorationIsManual)(self, aCurrentScrollRestorationIsManual)
    }


    /// ```text
    /// /**
    ///    * Setter and getter for the origin attributes living on this docshell.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] jsval getOriginAttributes ();`
    const _GetOriginAttributes: () = ();


    /// `[implicit_jscontext] void setOriginAttributes (in jsval aAttrs);`
    const _SetOriginAttributes: () = ();

    /// ```text
    /// /**
    ///    * The editing session for this docshell.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIEditingSession editingSession;`
    #[inline]
    pub unsafe fn GetEditingSession(&self, aEditingSession: *mut*const nsIEditingSession) -> ::nserror::nsresult {
        ((*self.vtable).GetEditingSession)(self, aEditingSession)
    }


    /// ```text
    /// /**
    ///    * The browser child for this docshell.
    ///    */
    /// ```
    ///

    /// `[binaryname(ScriptableBrowserChild)] readonly attribute nsIBrowserChild browserChild;`
    #[inline]
    pub unsafe fn GetScriptableBrowserChild(&self, aBrowserChild: *mut*const nsIBrowserChild) -> ::nserror::nsresult {
        ((*self.vtable).GetScriptableBrowserChild)(self, aBrowserChild)
    }



    /// `[noscript,nostdcall,notxpcom] BrowserChildRef GetBrowserChild ();`
    const _GetBrowserChild: () = ();


    /// `[noscript,nostdcall,notxpcom] nsCommandManager GetCommandManager ();`
    const _GetCommandManager: () = ();

    /// ```text
    /// /**
    ///    * This allows chrome to override the default choice of whether the
    ///    * <meta name="viewport"> tag is respected in a specific docshell.
    ///    * Possible values are listed above.
    ///    */
    /// ```
    ///

    /// `[infallible] attribute nsIDocShell_MetaViewportOverride metaViewportOverride;`
    #[inline]
    pub unsafe fn GetMetaViewportOverride(&self, aMetaViewportOverride: *mut u8) -> ::nserror::nsresult {
        ((*self.vtable).GetMetaViewportOverride)(self, aMetaViewportOverride)
    }


    /// ```text
    /// /**
    ///    * This allows chrome to override the default choice of whether the
    ///    * <meta name="viewport"> tag is respected in a specific docshell.
    ///    * Possible values are listed above.
    ///    */
    /// ```
    ///

    /// `[infallible] attribute nsIDocShell_MetaViewportOverride metaViewportOverride;`
    #[inline]
    pub unsafe fn SetMetaViewportOverride(&self, aMetaViewportOverride:  u8) -> ::nserror::nsresult {
        ((*self.vtable).SetMetaViewportOverride)(self, aMetaViewportOverride)
    }


    /// ```text
    /// /**
    ///    * Attribute that determines whether tracking protection is enabled.
    ///    */
    /// ```
    ///

    /// `attribute boolean useTrackingProtection;`
    #[inline]
    pub unsafe fn GetUseTrackingProtection(&self, aUseTrackingProtection: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetUseTrackingProtection)(self, aUseTrackingProtection)
    }


    /// ```text
    /// /**
    ///    * Attribute that determines whether tracking protection is enabled.
    ///    */
    /// ```
    ///

    /// `attribute boolean useTrackingProtection;`
    #[inline]
    pub unsafe fn SetUseTrackingProtection(&self, aUseTrackingProtection: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetUseTrackingProtection)(self, aUseTrackingProtection)
    }


    /// ```text
    /// /**
    ///   * Fire a dummy location change event asynchronously.
    ///   */
    /// ```
    ///

    /// `[noscript] void dispatchLocationChangeEvent ();`
    #[inline]
    pub unsafe fn DispatchLocationChangeEvent(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).DispatchLocationChangeEvent)(self, )
    }


    /// ```text
    /// /**
    ///    * Start delayed autoplay media which are in the current document.
    ///    */
    /// ```
    ///

    /// `[noscript] void startDelayedAutoplayMediaComponents ();`
    #[inline]
    pub unsafe fn StartDelayedAutoplayMediaComponents(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).StartDelayedAutoplayMediaComponents)(self, )
    }


    /// ```text
    /// /**
    ///    * Take ownership of the ClientSource representing an initial about:blank
    ///    * document that was never needed.  As an optimization we avoid creating
    ///    * this document if no code calls GetDocument(), but we still need a
    ///    * ClientSource object to represent the about:blank window.  This may return
    ///    * nullptr; for example if the docshell has created a real window and document
    ///    * already.
    ///    */
    /// ```
    ///

    /// `[noscript,nostdcall,notxpcom] UniqueClientSource TakeInitialClientSource ();`
    const _TakeInitialClientSource: () = ();


    /// `void setColorMatrix (in Array<float> aMatrix);`
    #[inline]
    pub unsafe fn SetColorMatrix(&self, aMatrix: *const thin_vec::ThinVec<libc::c_float>) -> ::nserror::nsresult {
        ((*self.vtable).SetColorMatrix)(self, aMatrix)
    }


    /// ```text
    /// /**
    ///    * Returns true if the current load is a forced reload,
    ///    * e.g. started by holding shift whilst triggering reload.
    ///    */
    /// ```
    ///

    /// `readonly attribute bool isForceReloading;`
    #[inline]
    pub unsafe fn GetIsForceReloading(&self, aIsForceReloading: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsForceReloading)(self, aIsForceReloading)
    }



    /// `Array<float> getColorMatrix ();`
    #[inline]
    pub unsafe fn GetColorMatrix(&self, _retval: *mut thin_vec::ThinVec<libc::c_float>) -> ::nserror::nsresult {
        ((*self.vtable).GetColorMatrix)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * The message manager for this docshell.  This does not throw, but
    ///    * can return null if the docshell has no message manager.
    ///    */
    /// ```
    ///

    /// `[infallible] readonly attribute ContentFrameMessageManager messageManager;`
    #[inline]
    pub unsafe fn GetMessageManager(&self, aMessageManager: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetMessageManager)(self, aMessageManager)
    }


    /// ```text
    /// /**
    ///    * This returns a Promise which resolves to a boolean. True when the
    ///    * document has Tracking Content that has been blocked from loading, false
    ///    * otherwise.
    ///    */
    /// ```
    ///

    /// `Promise getHasTrackingContentBlocked ();`
    const _GetHasTrackingContentBlocked: () = ();

    /// ```text
    /// /**
    ///    * Return whether this docshell is "attempting to navigate" in the
    ///    * sense that's relevant to document.open.
    ///    */
    /// ```
    ///

    /// `[nostdcall,notxpcom] readonly attribute boolean isAttemptingToNavigate;`
    const _GetIsAttemptingToNavigate: () = ();


    /// `[infallible] readonly attribute boolean isNavigating;`
    #[inline]
    pub unsafe fn GetIsNavigating(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetIsNavigating)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///    * @see nsISHEntry synchronizeLayoutHistoryState().
    ///    */
    /// ```
    ///

    /// `void synchronizeLayoutHistoryState ();`
    #[inline]
    pub unsafe fn SynchronizeLayoutHistoryState(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SynchronizeLayoutHistoryState)(self, )
    }


    /// ```text
    /// /**
    ///    * This attempts to save any applicable layout history state (like
        ///    * scroll position) in the nsISHEntry. This is normally done
    ///    * automatically when transitioning from page to page in the
    ///    * same process. We expose this function to support transitioning
    ///    * from page to page across processes as a workaround for bug 1630234
    ///    * until session history state is moved into the parent process.
    ///    */
    /// ```
    ///

    /// `void persistLayoutHistoryState ();`
    #[inline]
    pub unsafe fn PersistLayoutHistoryState(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).PersistLayoutHistoryState)(self, )
    }


}


