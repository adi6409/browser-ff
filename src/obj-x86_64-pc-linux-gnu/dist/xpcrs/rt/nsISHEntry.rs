//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/shistory/nsISHEntry.idl
//


/// `interface nsISHEntry : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISHEntry {
    vtable: *const nsISHEntryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISHEntry.
unsafe impl XpCom for nsISHEntry {
    const IID: nsIID = nsID(0x0dad26b8, 0xa259, 0x42c7,
        [0x93, 0xf1, 0x2f, 0xa7, 0xfc, 0x07, 0x6e, 0x45]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISHEntry {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISHEntry.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISHEntryCoerce {
    /// Cheaply cast a value of this type from a `nsISHEntry`.
    fn coerce_from(v: &nsISHEntry) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISHEntryCoerce for nsISHEntry {
    #[inline]
    fn coerce_from(v: &nsISHEntry) -> &Self {
        v
    }
}

impl nsISHEntry {
    /// Cast this `nsISHEntry` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISHEntryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISHEntry {
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
impl<T: nsISupportsCoerce> nsISHEntryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISHEntry) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISHEntry
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISHEntryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [infallible] attribute nsIURI URI; */
    pub GetURI: unsafe extern "system" fn (this: *const nsISHEntry, aURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* [infallible] attribute nsIURI URI; */
    pub SetURI: unsafe extern "system" fn (this: *const nsISHEntry, aURI: *const nsIURI) -> ::nserror::nsresult,

    /* [infallible] attribute nsIURI originalURI; */
    pub GetOriginalURI: unsafe extern "system" fn (this: *const nsISHEntry, aOriginalURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* [infallible] attribute nsIURI originalURI; */
    pub SetOriginalURI: unsafe extern "system" fn (this: *const nsISHEntry, aOriginalURI: *const nsIURI) -> ::nserror::nsresult,

    /* [infallible] attribute nsIURI resultPrincipalURI; */
    pub GetResultPrincipalURI: unsafe extern "system" fn (this: *const nsISHEntry, aResultPrincipalURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* [infallible] attribute nsIURI resultPrincipalURI; */
    pub SetResultPrincipalURI: unsafe extern "system" fn (this: *const nsISHEntry, aResultPrincipalURI: *const nsIURI) -> ::nserror::nsresult,

    /* [infallible] attribute boolean loadReplace; */
    pub GetLoadReplace: unsafe extern "system" fn (this: *const nsISHEntry, aLoadReplace: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean loadReplace; */
    pub SetLoadReplace: unsafe extern "system" fn (this: *const nsISHEntry, aLoadReplace: bool) -> ::nserror::nsresult,

    /* attribute AString title; */
    pub GetTitle: unsafe extern "system" fn (this: *const nsISHEntry, aTitle: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString title; */
    pub SetTitle: unsafe extern "system" fn (this: *const nsISHEntry, aTitle: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString name; */
    pub GetName: unsafe extern "system" fn (this: *const nsISHEntry, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString name; */
    pub SetName: unsafe extern "system" fn (this: *const nsISHEntry, aName: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [infallible] attribute boolean isSubFrame; */
    pub GetIsSubFrame: unsafe extern "system" fn (this: *const nsISHEntry, aIsSubFrame: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean isSubFrame; */
    pub SetIsSubFrame: unsafe extern "system" fn (this: *const nsISHEntry, aIsSubFrame: bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean hasUserInteraction; */
    pub GetHasUserInteraction: unsafe extern "system" fn (this: *const nsISHEntry, aHasUserInteraction: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean hasUserInteraction; */
    pub SetHasUserInteraction: unsafe extern "system" fn (this: *const nsISHEntry, aHasUserInteraction: bool) -> ::nserror::nsresult,

    /* [infallible] attribute nsIReferrerInfo referrerInfo; */
    pub GetReferrerInfo: unsafe extern "system" fn (this: *const nsISHEntry, aReferrerInfo: *mut*const nsIReferrerInfo) -> ::nserror::nsresult,

    /* [infallible] attribute nsIReferrerInfo referrerInfo; */
    pub SetReferrerInfo: unsafe extern "system" fn (this: *const nsISHEntry, aReferrerInfo: *const nsIReferrerInfo) -> ::nserror::nsresult,

    /* [infallible] attribute nsIContentViewer contentViewer; */
    pub GetContentViewer: unsafe extern "system" fn (this: *const nsISHEntry, aContentViewer: *mut*const nsIContentViewer) -> ::nserror::nsresult,

    /* [infallible] attribute nsIContentViewer contentViewer; */
    pub SetContentViewer: unsafe extern "system" fn (this: *const nsISHEntry, aContentViewer: *const nsIContentViewer) -> ::nserror::nsresult,

    /* [infallible] attribute boolean sticky; */
    pub GetSticky: unsafe extern "system" fn (this: *const nsISHEntry, aSticky: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean sticky; */
    pub SetSticky: unsafe extern "system" fn (this: *const nsISHEntry, aSticky: bool) -> ::nserror::nsresult,

    /* [infallible] attribute nsISupports windowState; */
    pub GetWindowState: unsafe extern "system" fn (this: *const nsISHEntry, aWindowState: *mut *const nsISupports) -> ::nserror::nsresult,

    /* [infallible] attribute nsISupports windowState; */
    pub SetWindowState: unsafe extern "system" fn (this: *const nsISHEntry, aWindowState: *const nsISupports) -> ::nserror::nsresult,

    /* [infallible] attribute nsIMutableArray refreshURIList; */
    pub GetRefreshURIList: unsafe extern "system" fn (this: *const nsISHEntry, aRefreshURIList: *mut*const nsIMutableArray) -> ::nserror::nsresult,

    /* [infallible] attribute nsIMutableArray refreshURIList; */
    pub SetRefreshURIList: unsafe extern "system" fn (this: *const nsISHEntry, aRefreshURIList: *const nsIMutableArray) -> ::nserror::nsresult,

    /* [infallible] attribute nsIInputStream postData; */
    pub GetPostData: unsafe extern "system" fn (this: *const nsISHEntry, aPostData: *mut*const nsIInputStream) -> ::nserror::nsresult,

    /* [infallible] attribute nsIInputStream postData; */
    pub SetPostData: unsafe extern "system" fn (this: *const nsISHEntry, aPostData: *const nsIInputStream) -> ::nserror::nsresult,

    /* [infallible] attribute nsILayoutHistoryState layoutHistoryState; */
    pub GetLayoutHistoryState: unsafe extern "system" fn (this: *const nsISHEntry, aLayoutHistoryState: *mut*const nsILayoutHistoryState) -> ::nserror::nsresult,

    /* [infallible] attribute nsILayoutHistoryState layoutHistoryState; */
    pub SetLayoutHistoryState: unsafe extern "system" fn (this: *const nsISHEntry, aLayoutHistoryState: *const nsILayoutHistoryState) -> ::nserror::nsresult,

    /* [infallible] attribute nsISHEntry parent; */
    pub GetParent: unsafe extern "system" fn (this: *const nsISHEntry, aParent: *mut *const nsISHEntry) -> ::nserror::nsresult,

    /* [infallible] attribute nsISHEntry parent; */
    pub SetParent: unsafe extern "system" fn (this: *const nsISHEntry, aParent: *const nsISHEntry) -> ::nserror::nsresult,

    /* [infallible] attribute unsigned long loadType; */
    pub GetLoadType: unsafe extern "system" fn (this: *const nsISHEntry, aLoadType: *mut u32) -> ::nserror::nsresult,

    /* [infallible] attribute unsigned long loadType; */
    pub SetLoadType: unsafe extern "system" fn (this: *const nsISHEntry, aLoadType: u32) -> ::nserror::nsresult,

    /* [infallible] attribute unsigned long ID; */
    pub GetID: unsafe extern "system" fn (this: *const nsISHEntry, aID: *mut u32) -> ::nserror::nsresult,

    /* [infallible] attribute unsigned long ID; */
    pub SetID: unsafe extern "system" fn (this: *const nsISHEntry, aID: u32) -> ::nserror::nsresult,

    /* [infallible] attribute unsigned long cacheKey; */
    pub GetCacheKey: unsafe extern "system" fn (this: *const nsISHEntry, aCacheKey: *mut u32) -> ::nserror::nsresult,

    /* [infallible] attribute unsigned long cacheKey; */
    pub SetCacheKey: unsafe extern "system" fn (this: *const nsISHEntry, aCacheKey: u32) -> ::nserror::nsresult,

    /* [infallible] attribute boolean saveLayoutStateFlag; */
    pub GetSaveLayoutStateFlag: unsafe extern "system" fn (this: *const nsISHEntry, aSaveLayoutStateFlag: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean saveLayoutStateFlag; */
    pub SetSaveLayoutStateFlag: unsafe extern "system" fn (this: *const nsISHEntry, aSaveLayoutStateFlag: bool) -> ::nserror::nsresult,

    /* attribute ACString contentType; */
    pub GetContentType: unsafe extern "system" fn (this: *const nsISHEntry, aContentType: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute ACString contentType; */
    pub SetContentType: unsafe extern "system" fn (this: *const nsISHEntry, aContentType: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [infallible] attribute boolean URIWasModified; */
    pub GetURIWasModified: unsafe extern "system" fn (this: *const nsISHEntry, aURIWasModified: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean URIWasModified; */
    pub SetURIWasModified: unsafe extern "system" fn (this: *const nsISHEntry, aURIWasModified: bool) -> ::nserror::nsresult,

    /* [infallible] attribute nsIPrincipal triggeringPrincipal; */
    pub GetTriggeringPrincipal: unsafe extern "system" fn (this: *const nsISHEntry, aTriggeringPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult,

    /* [infallible] attribute nsIPrincipal triggeringPrincipal; */
    pub SetTriggeringPrincipal: unsafe extern "system" fn (this: *const nsISHEntry, aTriggeringPrincipal: *const nsIPrincipal) -> ::nserror::nsresult,

    /* [infallible] attribute nsIPrincipal principalToInherit; */
    pub GetPrincipalToInherit: unsafe extern "system" fn (this: *const nsISHEntry, aPrincipalToInherit: *mut*const nsIPrincipal) -> ::nserror::nsresult,

    /* [infallible] attribute nsIPrincipal principalToInherit; */
    pub SetPrincipalToInherit: unsafe extern "system" fn (this: *const nsISHEntry, aPrincipalToInherit: *const nsIPrincipal) -> ::nserror::nsresult,

    /* [infallible] attribute nsIPrincipal partitionedPrincipalToInherit; */
    pub GetPartitionedPrincipalToInherit: unsafe extern "system" fn (this: *const nsISHEntry, aPartitionedPrincipalToInherit: *mut*const nsIPrincipal) -> ::nserror::nsresult,

    /* [infallible] attribute nsIPrincipal partitionedPrincipalToInherit; */
    pub SetPartitionedPrincipalToInherit: unsafe extern "system" fn (this: *const nsISHEntry, aPartitionedPrincipalToInherit: *const nsIPrincipal) -> ::nserror::nsresult,

    /* [infallible] attribute nsIContentSecurityPolicy csp; */
    pub GetCsp: unsafe extern "system" fn (this: *const nsISHEntry, aCsp: *mut*const nsIContentSecurityPolicy) -> ::nserror::nsresult,

    /* [infallible] attribute nsIContentSecurityPolicy csp; */
    pub SetCsp: unsafe extern "system" fn (this: *const nsISHEntry, aCsp: *const nsIContentSecurityPolicy) -> ::nserror::nsresult,

    /* [infallible] attribute nsIStructuredCloneContainer stateData; */
    pub GetStateData: unsafe extern "system" fn (this: *const nsISHEntry, aStateData: *mut*const nsIStructuredCloneContainer) -> ::nserror::nsresult,

    /* [infallible] attribute nsIStructuredCloneContainer stateData; */
    pub SetStateData: unsafe extern "system" fn (this: *const nsISHEntry, aStateData: *const nsIStructuredCloneContainer) -> ::nserror::nsresult,

    /* attribute nsIDRef docshellID; */
    pub GetDocshellID: unsafe extern "system" fn (this: *const nsISHEntry, aDocshellID: *mut nsID) -> ::nserror::nsresult,

    /* attribute nsIDRef docshellID; */
    pub SetDocshellID: unsafe extern "system" fn (this: *const nsISHEntry, aDocshellID: *const nsID) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean isSrcdocEntry; */
    pub GetIsSrcdocEntry: unsafe extern "system" fn (this: *const nsISHEntry, aIsSrcdocEntry: *mut bool) -> ::nserror::nsresult,

    /* attribute AString srcdocData; */
    pub GetSrcdocData: unsafe extern "system" fn (this: *const nsISHEntry, aSrcdocData: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString srcdocData; */
    pub SetSrcdocData: unsafe extern "system" fn (this: *const nsISHEntry, aSrcdocData: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [infallible] attribute nsIURI baseURI; */
    pub GetBaseURI: unsafe extern "system" fn (this: *const nsISHEntry, aBaseURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* [infallible] attribute nsIURI baseURI; */
    pub SetBaseURI: unsafe extern "system" fn (this: *const nsISHEntry, aBaseURI: *const nsIURI) -> ::nserror::nsresult,

    /* [infallible] attribute boolean scrollRestorationIsManual; */
    pub GetScrollRestorationIsManual: unsafe extern "system" fn (this: *const nsISHEntry, aScrollRestorationIsManual: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean scrollRestorationIsManual; */
    pub SetScrollRestorationIsManual: unsafe extern "system" fn (this: *const nsISHEntry, aScrollRestorationIsManual: bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean loadedInThisProcess; */
    pub GetLoadedInThisProcess: unsafe extern "system" fn (this: *const nsISHEntry, aLoadedInThisProcess: *mut bool) -> ::nserror::nsresult,

    /* [infallible,noscript] attribute nsISHistory shistory; */
    pub GetShistory: unsafe extern "system" fn (this: *const nsISHEntry, aShistory: *mut*const nsISHistory) -> ::nserror::nsresult,

    /* [infallible,noscript] attribute nsISHistory shistory; */
    pub SetShistory: unsafe extern "system" fn (this: *const nsISHEntry, aShistory: *const nsISHistory) -> ::nserror::nsresult,

    /* [infallible,noscript] attribute unsigned long lastTouched; */
    pub GetLastTouched: unsafe extern "system" fn (this: *const nsISHEntry, aLastTouched: *mut u32) -> ::nserror::nsresult,

    /* [infallible,noscript] attribute unsigned long lastTouched; */
    pub SetLastTouched: unsafe extern "system" fn (this: *const nsISHEntry, aLastTouched: u32) -> ::nserror::nsresult,

    /* [infallible] readonly attribute long childCount; */
    pub GetChildCount: unsafe extern "system" fn (this: *const nsISHEntry, aChildCount: *mut i32) -> ::nserror::nsresult,

    /* [infallible] attribute boolean persist; */
    pub GetPersist: unsafe extern "system" fn (this: *const nsISHEntry, aPersist: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean persist; */
    pub SetPersist: unsafe extern "system" fn (this: *const nsISHEntry, aPersist: bool) -> ::nserror::nsresult,

    /* void setScrollPosition (in long x, in long y); */
    pub SetScrollPosition: unsafe extern "system" fn (this: *const nsISHEntry, x: i32, y: i32) -> ::nserror::nsresult,

    /* void getScrollPosition (out long x, out long y); */
    pub GetScrollPosition: unsafe extern "system" fn (this: *const nsISHEntry, x: *mut i32, y: *mut i32) -> ::nserror::nsresult,

    /* [noscript,notxpcom] void getViewerBounds (in nsIntRect bounds); */
    /// Unable to generate binding because `native type nsIntRect unsupported`
    pub GetViewerBounds: *const ::libc::c_void,

    /* [noscript,notxpcom] void setViewerBounds ([const] in nsIntRect bounds); */
    /// Unable to generate binding because `native type nsIntRect unsupported`
    pub SetViewerBounds: *const ::libc::c_void,

    /* [noscript,notxpcom] void addChildShell (in nsIDocShellTreeItem shell); */
    pub AddChildShell: unsafe extern "system" fn (this: *const nsISHEntry, shell: *const nsIDocShellTreeItem) -> libc::c_void,

    /* [noscript] nsIDocShellTreeItem childShellAt (in long index); */
    pub ChildShellAt: unsafe extern "system" fn (this: *const nsISHEntry, index: i32, _retval: *mut*const nsIDocShellTreeItem) -> ::nserror::nsresult,

    /* [noscript,notxpcom] void clearChildShells (); */
    pub ClearChildShells: unsafe extern "system" fn (this: *const nsISHEntry) -> libc::c_void,

    /* [noscript,notxpcom] void syncPresentationState (); */
    pub SyncPresentationState: unsafe extern "system" fn (this: *const nsISHEntry) -> libc::c_void,

    /* nsILayoutHistoryState initLayoutHistoryState (); */
    pub InitLayoutHistoryState: unsafe extern "system" fn (this: *const nsISHEntry, _retval: *mut*const nsILayoutHistoryState) -> ::nserror::nsresult,

    /* [noscript] void create (in nsIURI URI, in AString title, in nsIInputStream inputStream, in unsigned long cacheKey, in ACString contentType, in nsIPrincipal triggeringPrincipal, in nsIPrincipal principalToInherit, in nsIPrincipal partitionedPrincipalToInherit, in nsIContentSecurityPolicy aCsp, in nsIDRef docshellID, in boolean dynamicCreation, in nsIURI originalURI, in nsIURI resultPrincipalURI, in bool loadReplace, in nsIReferrerInfo referrerInfo, in AString srcdoc, in bool srcdocEntry, in nsIURI baseURI, in bool saveLayoutState, in bool expired); */
    pub Create: unsafe extern "system" fn (this: *const nsISHEntry, URI: *const nsIURI, title: *const ::nsstring::nsAString, inputStream: *const nsIInputStream, cacheKey: u32, contentType: *const ::nsstring::nsACString, triggeringPrincipal: *const nsIPrincipal, principalToInherit: *const nsIPrincipal, partitionedPrincipalToInherit: *const nsIPrincipal, aCsp: *const nsIContentSecurityPolicy, docshellID: *const nsID, dynamicCreation: bool, originalURI: *const nsIURI, resultPrincipalURI: *const nsIURI, loadReplace: bool, referrerInfo: *const nsIReferrerInfo, srcdoc: *const ::nsstring::nsAString, srcdocEntry: bool, baseURI: *const nsIURI, saveLayoutState: bool, expired: bool) -> ::nserror::nsresult,

    /* nsISHEntry clone (); */
    pub Clone: unsafe extern "system" fn (this: *const nsISHEntry, _retval: *mut *const nsISHEntry) -> ::nserror::nsresult,

    /* [noscript,notxpcom] nsDocShellEditorDataPtr forgetEditorData (); */
    /// Unable to generate binding because `native type nsDocShellEditorData unsupported`
    pub ForgetEditorData: *const ::libc::c_void,

    /* [noscript,notxpcom] void setEditorData (in nsDocShellEditorDataPtr aData); */
    /// Unable to generate binding because `native type nsDocShellEditorData unsupported`
    pub SetEditorData: *const ::libc::c_void,

    /* [noscript,notxpcom] boolean hasDetachedEditor (); */
    pub HasDetachedEditor: unsafe extern "system" fn (this: *const nsISHEntry) -> bool,

    /* [noscript,notxpcom] boolean isDynamicallyAdded (); */
    pub IsDynamicallyAdded: unsafe extern "system" fn (this: *const nsISHEntry) -> bool,

    /* boolean hasDynamicallyAddedChild (); */
    pub HasDynamicallyAddedChild: unsafe extern "system" fn (this: *const nsISHEntry, _retval: *mut bool) -> ::nserror::nsresult,

    /* [noscript,notxpcom] boolean hasBFCacheEntry (in nsIBFCacheEntry aEntry); */
    pub HasBFCacheEntry: unsafe extern "system" fn (this: *const nsISHEntry, aEntry: *const nsIBFCacheEntry) -> bool,

    /* void adoptBFCacheEntry (in nsISHEntry aEntry); */
    pub AdoptBFCacheEntry: unsafe extern "system" fn (this: *const nsISHEntry, aEntry: *const nsISHEntry) -> ::nserror::nsresult,

    /* void abandonBFCacheEntry (); */
    pub AbandonBFCacheEntry: unsafe extern "system" fn (this: *const nsISHEntry) -> ::nserror::nsresult,

    /* boolean sharesDocumentWith (in nsISHEntry aEntry); */
    pub SharesDocumentWith: unsafe extern "system" fn (this: *const nsISHEntry, aEntry: *const nsISHEntry, _retval: *mut bool) -> ::nserror::nsresult,

    /* void setLoadTypeAsHistory (); */
    pub SetLoadTypeAsHistory: unsafe extern "system" fn (this: *const nsISHEntry) -> ::nserror::nsresult,

    /* void AddChild (in nsISHEntry aChild, in long aOffset, [optional, default (false)] in bool aUseRemoteSubframes); */
    pub AddChild: unsafe extern "system" fn (this: *const nsISHEntry, aChild: *const nsISHEntry, aOffset: i32, aUseRemoteSubframes: bool) -> ::nserror::nsresult,

    /* [noscript] void RemoveChild (in nsISHEntry aChild); */
    pub RemoveChild: unsafe extern "system" fn (this: *const nsISHEntry, aChild: *const nsISHEntry) -> ::nserror::nsresult,

    /* nsISHEntry GetChildAt (in long aIndex); */
    pub GetChildAt: unsafe extern "system" fn (this: *const nsISHEntry, aIndex: i32, _retval: *mut *const nsISHEntry) -> ::nserror::nsresult,

    /* [notxpcom] void GetChildSHEntryIfHasNoDynamicallyAddedChild (in long aChildOffset, out nsISHEntry aChild); */
    pub GetChildSHEntryIfHasNoDynamicallyAddedChild: unsafe extern "system" fn (this: *const nsISHEntry, aChildOffset: i32, aChild: *mut *const nsISHEntry) -> libc::c_void,

    /* [noscript] void ReplaceChild (in nsISHEntry aNewChild); */
    pub ReplaceChild: unsafe extern "system" fn (this: *const nsISHEntry, aNewChild: *const nsISHEntry) -> ::nserror::nsresult,

    /* [notxpcom] void ClearEntry (); */
    pub ClearEntry: unsafe extern "system" fn (this: *const nsISHEntry) -> libc::c_void,

    /* [noscript] nsDocShellLoadStatePtr CreateLoadInfo (); */
    /// Unable to generate binding because `native type nsDocShellLoadState unsupported`
    pub CreateLoadInfo: *const ::libc::c_void,

    /* [infallible] readonly attribute unsigned long long bfcacheID; */
    pub GetBfcacheID: unsafe extern "system" fn (this: *const nsISHEntry, aBfcacheID: *mut u64) -> ::nserror::nsresult,

    /* [notxpcom] void SyncTreesForSubframeNavigation (in nsISHEntry aEntry, in BrowsingContext aTopBC, in BrowsingContext aIgnoreBC); */
    pub SyncTreesForSubframeNavigation: unsafe extern "system" fn (this: *const nsISHEntry, aEntry: *const nsISHEntry, aTopBC: *const libc::c_void, aIgnoreBC: *const libc::c_void) -> libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISHEntry {

    /// ```text
    /// /**
    ///      * The URI of the current entry.
    ///      */
    /// ```
    ///

    /// `[infallible] attribute nsIURI URI;`
    #[inline]
    pub unsafe fn GetURI(&self, aURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetURI)(self, aURI)
    }


    /// ```text
    /// /**
    ///      * The URI of the current entry.
    ///      */
    /// ```
    ///

    /// `[infallible] attribute nsIURI URI;`
    #[inline]
    pub unsafe fn SetURI(&self, aURI: *const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).SetURI)(self, aURI)
    }


    /// ```text
    /// /**
    ///      * The original URI of the current entry. If an entry is the result of a
    ///      * redirect this attribute holds the original URI.
    ///      */
    /// ```
    ///

    /// `[infallible] attribute nsIURI originalURI;`
    #[inline]
    pub unsafe fn GetOriginalURI(&self, aOriginalURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetOriginalURI)(self, aOriginalURI)
    }


    /// ```text
    /// /**
    ///      * The original URI of the current entry. If an entry is the result of a
    ///      * redirect this attribute holds the original URI.
    ///      */
    /// ```
    ///

    /// `[infallible] attribute nsIURI originalURI;`
    #[inline]
    pub unsafe fn SetOriginalURI(&self, aOriginalURI: *const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).SetOriginalURI)(self, aOriginalURI)
    }


    /// ```text
    /// /**
    ///      * URL as stored from nsILoadInfo.resultPrincipalURI.  See nsILoadInfo
    ///      * for more details.
    ///      */
    /// ```
    ///

    /// `[infallible] attribute nsIURI resultPrincipalURI;`
    #[inline]
    pub unsafe fn GetResultPrincipalURI(&self, aResultPrincipalURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetResultPrincipalURI)(self, aResultPrincipalURI)
    }


    /// ```text
    /// /**
    ///      * URL as stored from nsILoadInfo.resultPrincipalURI.  See nsILoadInfo
    ///      * for more details.
    ///      */
    /// ```
    ///

    /// `[infallible] attribute nsIURI resultPrincipalURI;`
    #[inline]
    pub unsafe fn SetResultPrincipalURI(&self, aResultPrincipalURI: *const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).SetResultPrincipalURI)(self, aResultPrincipalURI)
    }


    /// ```text
    /// /**
    ///      *  This flag remembers whether channel has LOAD_REPLACE set.
    ///      */
    /// ```
    ///

    /// `[infallible] attribute boolean loadReplace;`
    #[inline]
    pub unsafe fn GetLoadReplace(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetLoadReplace)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///      *  This flag remembers whether channel has LOAD_REPLACE set.
    ///      */
    /// ```
    ///

    /// `[infallible] attribute boolean loadReplace;`
    #[inline]
    pub unsafe fn SetLoadReplace(&self, aLoadReplace: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetLoadReplace)(self, aLoadReplace)
    }


    /// ```text
    /// /**
    ///      * The title of the current entry.
    ///      */
    /// ```
    ///

    /// `attribute AString title;`
    #[inline]
    pub unsafe fn GetTitle(&self, aTitle: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetTitle)(self, aTitle)
    }


    /// ```text
    /// /**
    ///      * The title of the current entry.
    ///      */
    /// ```
    ///

    /// `attribute AString title;`
    #[inline]
    pub unsafe fn SetTitle(&self, aTitle: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetTitle)(self, aTitle)
    }


    /// ```text
    /// /**
    ///      * The name of the browsing context.
    ///      */
    /// ```
    ///

    /// `attribute AString name;`
    #[inline]
    pub unsafe fn GetName(&self, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetName)(self, aName)
    }


    /// ```text
    /// /**
    ///      * The name of the browsing context.
    ///      */
    /// ```
    ///

    /// `attribute AString name;`
    #[inline]
    pub unsafe fn SetName(&self, aName: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetName)(self, aName)
    }


    /// ```text
    /// /**
    ///      * Was the entry created as a result of a subframe navigation?
    ///      * - Will be 'false' when a frameset page is visited for the first time.
    ///      * - Will be 'true' for all history entries created as a result of a
    ///      *   subframe navigation.
    ///      */
    /// ```
    ///

    /// `[infallible] attribute boolean isSubFrame;`
    #[inline]
    pub unsafe fn GetIsSubFrame(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetIsSubFrame)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///      * Was the entry created as a result of a subframe navigation?
    ///      * - Will be 'false' when a frameset page is visited for the first time.
    ///      * - Will be 'true' for all history entries created as a result of a
    ///      *   subframe navigation.
    ///      */
    /// ```
    ///

    /// `[infallible] attribute boolean isSubFrame;`
    #[inline]
    pub unsafe fn SetIsSubFrame(&self, aIsSubFrame: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetIsSubFrame)(self, aIsSubFrame)
    }


    /// ```text
    /// /**
    ///      * Whether the user interacted with the page while this entry was active.
    ///      * This includes interactions with subframe documents associated with
    ///      * child entries that are rooted at this entry.
    ///      * This field will only be set on top-level entries.
    ///      */
    /// ```
    ///

    /// `[infallible] attribute boolean hasUserInteraction;`
    #[inline]
    pub unsafe fn GetHasUserInteraction(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetHasUserInteraction)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///      * Whether the user interacted with the page while this entry was active.
    ///      * This includes interactions with subframe documents associated with
    ///      * child entries that are rooted at this entry.
    ///      * This field will only be set on top-level entries.
    ///      */
    /// ```
    ///

    /// `[infallible] attribute boolean hasUserInteraction;`
    #[inline]
    pub unsafe fn SetHasUserInteraction(&self, aHasUserInteraction: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetHasUserInteraction)(self, aHasUserInteraction)
    }


    /// ```text
    /// /** Referrer Info*/
    /// ```
    ///

    /// `[infallible] attribute nsIReferrerInfo referrerInfo;`
    #[inline]
    pub unsafe fn GetReferrerInfo(&self, aReferrerInfo: *mut*const nsIReferrerInfo) -> ::nserror::nsresult {
        ((*self.vtable).GetReferrerInfo)(self, aReferrerInfo)
    }


    /// ```text
    /// /** Referrer Info*/
    /// ```
    ///

    /// `[infallible] attribute nsIReferrerInfo referrerInfo;`
    #[inline]
    pub unsafe fn SetReferrerInfo(&self, aReferrerInfo: *const nsIReferrerInfo) -> ::nserror::nsresult {
        ((*self.vtable).SetReferrerInfo)(self, aReferrerInfo)
    }


    /// ```text
    /// /** Content viewer, for fast restoration of presentation */
    /// ```
    ///

    /// `[infallible] attribute nsIContentViewer contentViewer;`
    #[inline]
    pub unsafe fn GetContentViewer(&self, aContentViewer: *mut*const nsIContentViewer) -> ::nserror::nsresult {
        ((*self.vtable).GetContentViewer)(self, aContentViewer)
    }


    /// ```text
    /// /** Content viewer, for fast restoration of presentation */
    /// ```
    ///

    /// `[infallible] attribute nsIContentViewer contentViewer;`
    #[inline]
    pub unsafe fn SetContentViewer(&self, aContentViewer: *const nsIContentViewer) -> ::nserror::nsresult {
        ((*self.vtable).SetContentViewer)(self, aContentViewer)
    }


    /// ```text
    /// /** Whether the content viewer is marked "sticky" */
    /// ```
    ///

    /// `[infallible] attribute boolean sticky;`
    #[inline]
    pub unsafe fn GetSticky(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetSticky)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /** Whether the content viewer is marked "sticky" */
    /// ```
    ///

    /// `[infallible] attribute boolean sticky;`
    #[inline]
    pub unsafe fn SetSticky(&self, aSticky: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetSticky)(self, aSticky)
    }


    /// ```text
    /// /** Saved state of the global window object */
    /// ```
    ///

    /// `[infallible] attribute nsISupports windowState;`
    #[inline]
    pub unsafe fn GetWindowState(&self, aWindowState: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).GetWindowState)(self, aWindowState)
    }


    /// ```text
    /// /** Saved state of the global window object */
    /// ```
    ///

    /// `[infallible] attribute nsISupports windowState;`
    #[inline]
    pub unsafe fn SetWindowState(&self, aWindowState: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).SetWindowState)(self, aWindowState)
    }


    /// ```text
    /// /** Saved refresh URI list for the content viewer */
    /// ```
    ///

    /// `[infallible] attribute nsIMutableArray refreshURIList;`
    #[inline]
    pub unsafe fn GetRefreshURIList(&self, aRefreshURIList: *mut*const nsIMutableArray) -> ::nserror::nsresult {
        ((*self.vtable).GetRefreshURIList)(self, aRefreshURIList)
    }


    /// ```text
    /// /** Saved refresh URI list for the content viewer */
    /// ```
    ///

    /// `[infallible] attribute nsIMutableArray refreshURIList;`
    #[inline]
    pub unsafe fn SetRefreshURIList(&self, aRefreshURIList: *const nsIMutableArray) -> ::nserror::nsresult {
        ((*self.vtable).SetRefreshURIList)(self, aRefreshURIList)
    }


    /// ```text
    /// /** Post Data for the document */
    /// ```
    ///

    /// `[infallible] attribute nsIInputStream postData;`
    #[inline]
    pub unsafe fn GetPostData(&self, aPostData: *mut*const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).GetPostData)(self, aPostData)
    }


    /// ```text
    /// /** Post Data for the document */
    /// ```
    ///

    /// `[infallible] attribute nsIInputStream postData;`
    #[inline]
    pub unsafe fn SetPostData(&self, aPostData: *const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).SetPostData)(self, aPostData)
    }


    /// ```text
    /// /** LayoutHistoryState for scroll position and form values */
    /// ```
    ///

    /// `[infallible] attribute nsILayoutHistoryState layoutHistoryState;`
    #[inline]
    pub unsafe fn GetLayoutHistoryState(&self, aLayoutHistoryState: *mut*const nsILayoutHistoryState) -> ::nserror::nsresult {
        ((*self.vtable).GetLayoutHistoryState)(self, aLayoutHistoryState)
    }


    /// ```text
    /// /** LayoutHistoryState for scroll position and form values */
    /// ```
    ///

    /// `[infallible] attribute nsILayoutHistoryState layoutHistoryState;`
    #[inline]
    pub unsafe fn SetLayoutHistoryState(&self, aLayoutHistoryState: *const nsILayoutHistoryState) -> ::nserror::nsresult {
        ((*self.vtable).SetLayoutHistoryState)(self, aLayoutHistoryState)
    }


    /// ```text
    /// /** parent of this entry */
    /// ```
    ///

    /// `[infallible] attribute nsISHEntry parent;`
    #[inline]
    pub unsafe fn GetParent(&self, aParent: *mut *const nsISHEntry) -> ::nserror::nsresult {
        ((*self.vtable).GetParent)(self, aParent)
    }


    /// ```text
    /// /** parent of this entry */
    /// ```
    ///

    /// `[infallible] attribute nsISHEntry parent;`
    #[inline]
    pub unsafe fn SetParent(&self, aParent: *const nsISHEntry) -> ::nserror::nsresult {
        ((*self.vtable).SetParent)(self, aParent)
    }


    /// ```text
    /// /**
    ///      * The loadType for this entry. This is typically loadHistory except
    ///      * when reload is pressed, it has the appropriate reload flag
    ///      */
    /// ```
    ///

    /// `[infallible] attribute unsigned long loadType;`
    #[inline]
    pub unsafe fn GetLoadType(&self) -> u32 {
        let mut result = <u32 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetLoadType)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///      * The loadType for this entry. This is typically loadHistory except
    ///      * when reload is pressed, it has the appropriate reload flag
    ///      */
    /// ```
    ///

    /// `[infallible] attribute unsigned long loadType;`
    #[inline]
    pub unsafe fn SetLoadType(&self, aLoadType: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetLoadType)(self, aLoadType)
    }


    /// ```text
    /// /**
    ///      * An ID to help identify this entry from others during
    ///      * subframe navigation
    ///      */
    /// ```
    ///

    /// `[infallible] attribute unsigned long ID;`
    #[inline]
    pub unsafe fn GetID(&self) -> u32 {
        let mut result = <u32 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetID)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///      * An ID to help identify this entry from others during
    ///      * subframe navigation
    ///      */
    /// ```
    ///

    /// `[infallible] attribute unsigned long ID;`
    #[inline]
    pub unsafe fn SetID(&self, aID: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetID)(self, aID)
    }


    /// ```text
    /// /** The cache key for the entry */
    /// ```
    ///

    /// `[infallible] attribute unsigned long cacheKey;`
    #[inline]
    pub unsafe fn GetCacheKey(&self) -> u32 {
        let mut result = <u32 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetCacheKey)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /** The cache key for the entry */
    /// ```
    ///

    /// `[infallible] attribute unsigned long cacheKey;`
    #[inline]
    pub unsafe fn SetCacheKey(&self, aCacheKey: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetCacheKey)(self, aCacheKey)
    }


    /// ```text
    /// /** Should the layoutHistoryState be saved? */
    /// ```
    ///

    /// `[infallible] attribute boolean saveLayoutStateFlag;`
    #[inline]
    pub unsafe fn GetSaveLayoutStateFlag(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetSaveLayoutStateFlag)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /** Should the layoutHistoryState be saved? */
    /// ```
    ///

    /// `[infallible] attribute boolean saveLayoutStateFlag;`
    #[inline]
    pub unsafe fn SetSaveLayoutStateFlag(&self, aSaveLayoutStateFlag: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetSaveLayoutStateFlag)(self, aSaveLayoutStateFlag)
    }


    /// ```text
    /// /**
    ///      * attribute to indicate the content-type of the document that this
    ///      * is a session history entry for
    ///      */
    /// ```
    ///

    /// `attribute ACString contentType;`
    #[inline]
    pub unsafe fn GetContentType(&self, aContentType: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetContentType)(self, aContentType)
    }


    /// ```text
    /// /**
    ///      * attribute to indicate the content-type of the document that this
    ///      * is a session history entry for
    ///      */
    /// ```
    ///

    /// `attribute ACString contentType;`
    #[inline]
    pub unsafe fn SetContentType(&self, aContentType: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetContentType)(self, aContentType)
    }


    /// ```text
    /// /**
    ///      * If we created this SHEntry via history.pushState or modified it via
    ///      * history.replaceState, and if we changed the SHEntry's URI via the
    ///      * push/replaceState call, and if the SHEntry's new URI differs from its
    ///      * old URI by more than just the hash, then we set this field to true.
    ///      *
    ///      * Additionally, if this SHEntry was created by calling pushState from a
    ///      * SHEntry whose URI was modified, this SHEntry's URIWasModified field is
    ///      * true.
    ///      */
    /// ```
    ///

    /// `[infallible] attribute boolean URIWasModified;`
    #[inline]
    pub unsafe fn GetURIWasModified(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetURIWasModified)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///      * If we created this SHEntry via history.pushState or modified it via
    ///      * history.replaceState, and if we changed the SHEntry's URI via the
    ///      * push/replaceState call, and if the SHEntry's new URI differs from its
    ///      * old URI by more than just the hash, then we set this field to true.
    ///      *
    ///      * Additionally, if this SHEntry was created by calling pushState from a
    ///      * SHEntry whose URI was modified, this SHEntry's URIWasModified field is
    ///      * true.
    ///      */
    /// ```
    ///

    /// `[infallible] attribute boolean URIWasModified;`
    #[inline]
    pub unsafe fn SetURIWasModified(&self, aURIWasModified: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetURIWasModified)(self, aURIWasModified)
    }


    /// ```text
    /// /**
    ///      * Get the principal, if any, that was associated with the channel
    ///      * that the document that was loaded to create this history entry
    ///      * came from.
    ///      */
    /// ```
    ///

    /// `[infallible] attribute nsIPrincipal triggeringPrincipal;`
    #[inline]
    pub unsafe fn GetTriggeringPrincipal(&self, aTriggeringPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).GetTriggeringPrincipal)(self, aTriggeringPrincipal)
    }


    /// ```text
    /// /**
    ///      * Get the principal, if any, that was associated with the channel
    ///      * that the document that was loaded to create this history entry
    ///      * came from.
    ///      */
    /// ```
    ///

    /// `[infallible] attribute nsIPrincipal triggeringPrincipal;`
    #[inline]
    pub unsafe fn SetTriggeringPrincipal(&self, aTriggeringPrincipal: *const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).SetTriggeringPrincipal)(self, aTriggeringPrincipal)
    }


    /// ```text
    /// /**
    ///      * Get the principal, if any, that is used when the inherit flag
    ///      * is set.
    ///      */
    /// ```
    ///

    /// `[infallible] attribute nsIPrincipal principalToInherit;`
    #[inline]
    pub unsafe fn GetPrincipalToInherit(&self, aPrincipalToInherit: *mut*const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).GetPrincipalToInherit)(self, aPrincipalToInherit)
    }


    /// ```text
    /// /**
    ///      * Get the principal, if any, that is used when the inherit flag
    ///      * is set.
    ///      */
    /// ```
    ///

    /// `[infallible] attribute nsIPrincipal principalToInherit;`
    #[inline]
    pub unsafe fn SetPrincipalToInherit(&self, aPrincipalToInherit: *const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).SetPrincipalToInherit)(self, aPrincipalToInherit)
    }


    /// ```text
    /// /**
    ///      * Get the storage principal, if any, that is used when the inherit flag is
    ///      * set.
    ///      */
    /// ```
    ///

    /// `[infallible] attribute nsIPrincipal partitionedPrincipalToInherit;`
    #[inline]
    pub unsafe fn GetPartitionedPrincipalToInherit(&self, aPartitionedPrincipalToInherit: *mut*const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).GetPartitionedPrincipalToInherit)(self, aPartitionedPrincipalToInherit)
    }


    /// ```text
    /// /**
    ///      * Get the storage principal, if any, that is used when the inherit flag is
    ///      * set.
    ///      */
    /// ```
    ///

    /// `[infallible] attribute nsIPrincipal partitionedPrincipalToInherit;`
    #[inline]
    pub unsafe fn SetPartitionedPrincipalToInherit(&self, aPartitionedPrincipalToInherit: *const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).SetPartitionedPrincipalToInherit)(self, aPartitionedPrincipalToInherit)
    }


    /// ```text
    /// /**
    ///      * Get the csp, if any, that was used for this document load. That
    ///      * is not the CSP that was applied to subresource loads within the
    ///      * document, but the CSP that was applied to this document load.
    ///      */
    /// ```
    ///

    /// `[infallible] attribute nsIContentSecurityPolicy csp;`
    #[inline]
    pub unsafe fn GetCsp(&self, aCsp: *mut*const nsIContentSecurityPolicy) -> ::nserror::nsresult {
        ((*self.vtable).GetCsp)(self, aCsp)
    }


    /// ```text
    /// /**
    ///      * Get the csp, if any, that was used for this document load. That
    ///      * is not the CSP that was applied to subresource loads within the
    ///      * document, but the CSP that was applied to this document load.
    ///      */
    /// ```
    ///

    /// `[infallible] attribute nsIContentSecurityPolicy csp;`
    #[inline]
    pub unsafe fn SetCsp(&self, aCsp: *const nsIContentSecurityPolicy) -> ::nserror::nsresult {
        ((*self.vtable).SetCsp)(self, aCsp)
    }


    /// ```text
    /// /**
    ///      * Get/set data associated with this history state via a pushState() call,
    ///      * serialized using structured clone.
    ///      **/
    /// ```
    ///

    /// `[infallible] attribute nsIStructuredCloneContainer stateData;`
    #[inline]
    pub unsafe fn GetStateData(&self, aStateData: *mut*const nsIStructuredCloneContainer) -> ::nserror::nsresult {
        ((*self.vtable).GetStateData)(self, aStateData)
    }


    /// ```text
    /// /**
    ///      * Get/set data associated with this history state via a pushState() call,
    ///      * serialized using structured clone.
    ///      **/
    /// ```
    ///

    /// `[infallible] attribute nsIStructuredCloneContainer stateData;`
    #[inline]
    pub unsafe fn SetStateData(&self, aStateData: *const nsIStructuredCloneContainer) -> ::nserror::nsresult {
        ((*self.vtable).SetStateData)(self, aStateData)
    }


    /// ```text
    /// /**
    ///      * The history ID of the docshell.
    ///      */
    /// ```
    ///

    /// `attribute nsIDRef docshellID;`
    #[inline]
    pub unsafe fn GetDocshellID(&self, aDocshellID: *mut nsID) -> ::nserror::nsresult {
        ((*self.vtable).GetDocshellID)(self, aDocshellID)
    }


    /// ```text
    /// /**
    ///      * The history ID of the docshell.
    ///      */
    /// ```
    ///

    /// `attribute nsIDRef docshellID;`
    #[inline]
    pub unsafe fn SetDocshellID(&self, aDocshellID: *const nsID) -> ::nserror::nsresult {
        ((*self.vtable).SetDocshellID)(self, aDocshellID)
    }


    /// ```text
    /// /**
    ///      * True if this SHEntry corresponds to a document created by a srcdoc
    ///      * iframe. Set when a value is assigned to srcdocData.
    ///      */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean isSrcdocEntry;`
    #[inline]
    pub unsafe fn GetIsSrcdocEntry(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetIsSrcdocEntry)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///      * Contents of the srcdoc attribute in a srcdoc iframe to be loaded instead
    ///      * of the URI.  Similar to a Data URI, this information is needed to
    ///      * recreate the document at a later stage.
    ///      * Setting this sets isSrcdocEntry to true
    ///      */
    /// ```
    ///

    /// `attribute AString srcdocData;`
    #[inline]
    pub unsafe fn GetSrcdocData(&self, aSrcdocData: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSrcdocData)(self, aSrcdocData)
    }


    /// ```text
    /// /**
    ///      * Contents of the srcdoc attribute in a srcdoc iframe to be loaded instead
    ///      * of the URI.  Similar to a Data URI, this information is needed to
    ///      * recreate the document at a later stage.
    ///      * Setting this sets isSrcdocEntry to true
    ///      */
    /// ```
    ///

    /// `attribute AString srcdocData;`
    #[inline]
    pub unsafe fn SetSrcdocData(&self, aSrcdocData: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetSrcdocData)(self, aSrcdocData)
    }


    /// ```text
    /// /**
    ///      * When isSrcdocEntry is true, this contains the baseURI of the srcdoc
    ///      * document for use in situations where it cannot otherwise be determined,
    ///      * for example with view-source.
    ///      */
    /// ```
    ///

    /// `[infallible] attribute nsIURI baseURI;`
    #[inline]
    pub unsafe fn GetBaseURI(&self, aBaseURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetBaseURI)(self, aBaseURI)
    }


    /// ```text
    /// /**
    ///      * When isSrcdocEntry is true, this contains the baseURI of the srcdoc
    ///      * document for use in situations where it cannot otherwise be determined,
    ///      * for example with view-source.
    ///      */
    /// ```
    ///

    /// `[infallible] attribute nsIURI baseURI;`
    #[inline]
    pub unsafe fn SetBaseURI(&self, aBaseURI: *const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).SetBaseURI)(self, aBaseURI)
    }


    /// ```text
    /// /**
    ///      * Sets/gets the current scroll restoration state,
    ///      * if true == "manual", false == "auto".
    ///      */
    /// ```
    ///

    /// `[infallible] attribute boolean scrollRestorationIsManual;`
    #[inline]
    pub unsafe fn GetScrollRestorationIsManual(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetScrollRestorationIsManual)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///      * Sets/gets the current scroll restoration state,
    ///      * if true == "manual", false == "auto".
    ///      */
    /// ```
    ///

    /// `[infallible] attribute boolean scrollRestorationIsManual;`
    #[inline]
    pub unsafe fn SetScrollRestorationIsManual(&self, aScrollRestorationIsManual: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetScrollRestorationIsManual)(self, aScrollRestorationIsManual)
    }


    /// ```text
    /// /**
    ///      * Flag to indicate that the history entry was originally loaded in the
    ///      * current process. This flag does not survive a browser process switch.
    ///      */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean loadedInThisProcess;`
    #[inline]
    pub unsafe fn GetLoadedInThisProcess(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetLoadedInThisProcess)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///      * The session history it belongs to. This is set only on the root entries.
    ///      */
    /// ```
    ///

    /// `[infallible,noscript] attribute nsISHistory shistory;`
    #[inline]
    pub unsafe fn GetShistory(&self, aShistory: *mut*const nsISHistory) -> ::nserror::nsresult {
        ((*self.vtable).GetShistory)(self, aShistory)
    }


    /// ```text
    /// /**
    ///      * The session history it belongs to. This is set only on the root entries.
    ///      */
    /// ```
    ///

    /// `[infallible,noscript] attribute nsISHistory shistory;`
    #[inline]
    pub unsafe fn SetShistory(&self, aShistory: *const nsISHistory) -> ::nserror::nsresult {
        ((*self.vtable).SetShistory)(self, aShistory)
    }


    /// ```text
    /// /**
    ///      * A number that is assigned by the sHistory when the entry is activated
    ///      */
    /// ```
    ///

    /// `[infallible,noscript] attribute unsigned long lastTouched;`
    #[inline]
    pub unsafe fn GetLastTouched(&self) -> u32 {
        let mut result = <u32 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetLastTouched)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///      * A number that is assigned by the sHistory when the entry is activated
    ///      */
    /// ```
    ///

    /// `[infallible,noscript] attribute unsigned long lastTouched;`
    #[inline]
    pub unsafe fn SetLastTouched(&self, aLastTouched: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetLastTouched)(self, aLastTouched)
    }


    /// ```text
    /// /**
    ///      * The current number of nsISHEntries which are immediate children of this
    ///      * SHEntry.
    ///      */
    /// ```
    ///

    /// `[infallible] readonly attribute long childCount;`
    #[inline]
    pub unsafe fn GetChildCount(&self) -> i32 {
        let mut result = <i32 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetChildCount)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///      * When an entry is serving is within nsISHistory's array of entries, this
    ///      * property specifies if it should persist. If not it will be replaced by
    ///      * new additions to the list.
    ///      */
    /// ```
    ///

    /// `[infallible] attribute boolean persist;`
    #[inline]
    pub unsafe fn GetPersist(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetPersist)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///      * When an entry is serving is within nsISHistory's array of entries, this
    ///      * property specifies if it should persist. If not it will be replaced by
    ///      * new additions to the list.
    ///      */
    /// ```
    ///

    /// `[infallible] attribute boolean persist;`
    #[inline]
    pub unsafe fn SetPersist(&self, aPersist: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetPersist)(self, aPersist)
    }


    /// ```text
    /// /**
    ///      * Set/Get the visual viewport scroll position if session history is
    ///      * changed through anchor navigation or pushState.
    ///      */
    /// ```
    ///

    /// `void setScrollPosition (in long x, in long y);`
    #[inline]
    pub unsafe fn SetScrollPosition(&self, x: i32, y: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetScrollPosition)(self, x, y)
    }



    /// `void getScrollPosition (out long x, out long y);`
    #[inline]
    pub unsafe fn GetScrollPosition(&self, x: *mut i32, y: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetScrollPosition)(self, x, y)
    }


    /// ```text
    /// /**
    ///      * Saved position and dimensions of the content viewer; we must adjust the
    ///      * root view's widget accordingly if this has changed when the presentation
    ///      * is restored.
    ///      */
    /// ```
    ///

    /// `[noscript,notxpcom] void getViewerBounds (in nsIntRect bounds);`
    const _GetViewerBounds: () = ();


    /// `[noscript,notxpcom] void setViewerBounds ([const] in nsIntRect bounds);`
    const _SetViewerBounds: () = ();

    /// ```text
    /// /**
    ///      * Saved child docshells corresponding to contentViewer.  The child shells
    ///      * are restored as children of the parent docshell, in this order, when the
    ///      * parent docshell restores a saved presentation.
    ///      */
    /// /** Append a child shell to the end of our list. */
    /// ```
    ///

    /// `[noscript,notxpcom] void addChildShell (in nsIDocShellTreeItem shell);`
    #[inline]
    pub unsafe fn AddChildShell(&self, shell: *const nsIDocShellTreeItem) -> libc::c_void {
        ((*self.vtable).AddChildShell)(self, shell)
    }


    /// ```text
    /// /**
    ///      * Get the child shell at |index|; returns null if |index| is out of bounds.
    ///      */
    /// ```
    ///

    /// `[noscript] nsIDocShellTreeItem childShellAt (in long index);`
    #[inline]
    pub unsafe fn ChildShellAt(&self, index: i32, _retval: *mut*const nsIDocShellTreeItem) -> ::nserror::nsresult {
        ((*self.vtable).ChildShellAt)(self, index, _retval)
    }


    /// ```text
    /// /**
    ///      * Clear the child shell list.
    ///      */
    /// ```
    ///

    /// `[noscript,notxpcom] void clearChildShells ();`
    #[inline]
    pub unsafe fn ClearChildShells(&self, ) -> libc::c_void {
        ((*self.vtable).ClearChildShells)(self, )
    }


    /// ```text
    /// /**
    ///      * Ensure that the cached presentation members are self-consistent.
    ///      * If either |contentViewer| or |windowState| are null, then all of the
    ///      * following members are cleared/reset:
    ///      *  contentViewer, sticky, windowState, viewerBounds, childShells,
    ///      *  refreshURIList.
    ///      */
    /// ```
    ///

    /// `[noscript,notxpcom] void syncPresentationState ();`
    #[inline]
    pub unsafe fn SyncPresentationState(&self, ) -> libc::c_void {
        ((*self.vtable).SyncPresentationState)(self, )
    }


    /// ```text
    /// /**
    ///      * Initialises `layoutHistoryState` if it doesn't already exist
    ///      * and returns a reference to it.
    ///      */
    /// ```
    ///

    /// `nsILayoutHistoryState initLayoutHistoryState ();`
    #[inline]
    pub unsafe fn InitLayoutHistoryState(&self, _retval: *mut*const nsILayoutHistoryState) -> ::nserror::nsresult {
        ((*self.vtable).InitLayoutHistoryState)(self, _retval)
    }


    /// ```text
    /// /** Additional ways to create an entry */
    /// ```
    ///

    /// `[noscript] void create (in nsIURI URI, in AString title, in nsIInputStream inputStream, in unsigned long cacheKey, in ACString contentType, in nsIPrincipal triggeringPrincipal, in nsIPrincipal principalToInherit, in nsIPrincipal partitionedPrincipalToInherit, in nsIContentSecurityPolicy aCsp, in nsIDRef docshellID, in boolean dynamicCreation, in nsIURI originalURI, in nsIURI resultPrincipalURI, in bool loadReplace, in nsIReferrerInfo referrerInfo, in AString srcdoc, in bool srcdocEntry, in nsIURI baseURI, in bool saveLayoutState, in bool expired);`
    #[inline]
    pub unsafe fn Create(&self, URI: *const nsIURI, title: *const ::nsstring::nsAString, inputStream: *const nsIInputStream, cacheKey: u32, contentType: *const ::nsstring::nsACString, triggeringPrincipal: *const nsIPrincipal, principalToInherit: *const nsIPrincipal, partitionedPrincipalToInherit: *const nsIPrincipal, aCsp: *const nsIContentSecurityPolicy, docshellID: *const nsID, dynamicCreation: bool, originalURI: *const nsIURI, resultPrincipalURI: *const nsIURI, loadReplace: bool, referrerInfo: *const nsIReferrerInfo, srcdoc: *const ::nsstring::nsAString, srcdocEntry: bool, baseURI: *const nsIURI, saveLayoutState: bool, expired: bool) -> ::nserror::nsresult {
        ((*self.vtable).Create)(self, URI, title, inputStream, cacheKey, contentType, triggeringPrincipal, principalToInherit, partitionedPrincipalToInherit, aCsp, docshellID, dynamicCreation, originalURI, resultPrincipalURI, loadReplace, referrerInfo, srcdoc, srcdocEntry, baseURI, saveLayoutState, expired)
    }



    /// `nsISHEntry clone ();`
    #[inline]
    pub unsafe fn Clone(&self, _retval: *mut *const nsISHEntry) -> ::nserror::nsresult {
        ((*self.vtable).Clone)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * Gets the owning pointer to the editor data assosicated with
    ///      * this shistory entry. This forgets its pointer, so free it when
    ///      * you're done.
    ///      */
    /// ```
    ///

    /// `[noscript,notxpcom] nsDocShellEditorDataPtr forgetEditorData ();`
    const _ForgetEditorData: () = ();

    /// ```text
    /// /**
    ///      * Sets the owning pointer to the editor data assosicated with
    ///      * this shistory entry. Unless forgetEditorData() is called, this
    ///      * shentry will destroy the editor data when it's destroyed.
    ///      */
    /// ```
    ///

    /// `[noscript,notxpcom] void setEditorData (in nsDocShellEditorDataPtr aData);`
    const _SetEditorData: () = ();

    /// ```text
    /// /** Returns true if this shistory entry is storing a detached editor. */
    /// ```
    ///

    /// `[noscript,notxpcom] boolean hasDetachedEditor ();`
    #[inline]
    pub unsafe fn HasDetachedEditor(&self, ) -> bool {
        ((*self.vtable).HasDetachedEditor)(self, )
    }


    /// ```text
    /// /**
    ///      * Returns true if the related docshell was added because of
    ///      * dynamic addition of an iframe/frame.
    ///      */
    /// ```
    ///

    /// `[noscript,notxpcom] boolean isDynamicallyAdded ();`
    #[inline]
    pub unsafe fn IsDynamicallyAdded(&self, ) -> bool {
        ((*self.vtable).IsDynamicallyAdded)(self, )
    }


    /// ```text
    /// /**
    ///      * Returns true if any of the child entries returns true
    ///      * when isDynamicallyAdded is called on it.
    ///      */
    /// ```
    ///

    /// `boolean hasDynamicallyAddedChild ();`
    #[inline]
    pub unsafe fn HasDynamicallyAddedChild(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasDynamicallyAddedChild)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * Does this SHEntry point to the given BFCache entry? If so, evicting
    ///      * the BFCache entry will evict the SHEntry, since the two entries
    ///      * correspond to the same document.
    ///      */
    /// ```
    ///

    /// `[noscript,notxpcom] boolean hasBFCacheEntry (in nsIBFCacheEntry aEntry);`
    #[inline]
    pub unsafe fn HasBFCacheEntry(&self, aEntry: *const nsIBFCacheEntry) -> bool {
        ((*self.vtable).HasBFCacheEntry)(self, aEntry)
    }


    /// ```text
    /// /**
    ///      * Adopt aEntry's BFCacheEntry, so now both this and aEntry point to
    ///      * aEntry's BFCacheEntry.
    ///      */
    /// ```
    ///

    /// `void adoptBFCacheEntry (in nsISHEntry aEntry);`
    #[inline]
    pub unsafe fn AdoptBFCacheEntry(&self, aEntry: *const nsISHEntry) -> ::nserror::nsresult {
        ((*self.vtable).AdoptBFCacheEntry)(self, aEntry)
    }


    /// ```text
    /// /**
    ///      * Create a new BFCache entry and drop our reference to our old one. This
    ///      * call unlinks this SHEntry from any other SHEntries for its document.
    ///      */
    /// ```
    ///

    /// `void abandonBFCacheEntry ();`
    #[inline]
    pub unsafe fn AbandonBFCacheEntry(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).AbandonBFCacheEntry)(self, )
    }


    /// ```text
    /// /**
    ///      * Does this SHEntry correspond to the same document as aEntry? This is
    ///      * true iff the two SHEntries have the same BFCacheEntry. So in particular,
    ///      * sharesDocumentWith(aEntry) is guaranteed to return true if it's
    ///      * preceded by a call to adoptBFCacheEntry(aEntry).
    ///      */
    /// ```
    ///

    /// `boolean sharesDocumentWith (in nsISHEntry aEntry);`
    #[inline]
    pub unsafe fn SharesDocumentWith(&self, aEntry: *const nsISHEntry, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).SharesDocumentWith)(self, aEntry, _retval)
    }


    /// ```text
    /// /**
    ///      * Sets an SHEntry to reflect that it is a history type load. As
    ///      * nsIDocShellLoadInfo and its LoadType enum were removed, this is the
    ///      * equivalent to doing
    ///      *
    ///      * shEntry.loadType = 4;
    ///      *
    ///      * in js, but easier to maintain and less opaque.
    ///      */
    /// ```
    ///

    /// `void setLoadTypeAsHistory ();`
    #[inline]
    pub unsafe fn SetLoadTypeAsHistory(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SetLoadTypeAsHistory)(self, )
    }


    /// ```text
    /// /**
    ///      * Add a new child SHEntry. If offset is -1 adds to the end of the list.
    ///      */
    /// ```
    ///

    /// `void AddChild (in nsISHEntry aChild, in long aOffset, [optional, default (false)] in bool aUseRemoteSubframes);`
    #[inline]
    pub unsafe fn AddChild(&self, aChild: *const nsISHEntry, aOffset: i32, aUseRemoteSubframes: bool) -> ::nserror::nsresult {
        ((*self.vtable).AddChild)(self, aChild, aOffset, aUseRemoteSubframes)
    }


    /// ```text
    /// /**
    ///      * Remove a child SHEntry.
    ///      */
    /// ```
    ///

    /// `[noscript] void RemoveChild (in nsISHEntry aChild);`
    #[inline]
    pub unsafe fn RemoveChild(&self, aChild: *const nsISHEntry) -> ::nserror::nsresult {
        ((*self.vtable).RemoveChild)(self, aChild)
    }


    /// ```text
    /// /**
    ///      * Get child at an index.
    ///      */
    /// ```
    ///

    /// `nsISHEntry GetChildAt (in long aIndex);`
    #[inline]
    pub unsafe fn GetChildAt(&self, aIndex: i32, _retval: *mut *const nsISHEntry) -> ::nserror::nsresult {
        ((*self.vtable).GetChildAt)(self, aIndex, _retval)
    }


    /// ```text
    /// /**
    ///      * If this entry has no dynamically added child, get the child SHEntry
    ///      * at the given offset. The loadtype of the returned entry is set
    ///      * to its parent's loadtype.
    ///      */
    /// ```
    ///

    /// `[notxpcom] void GetChildSHEntryIfHasNoDynamicallyAddedChild (in long aChildOffset, out nsISHEntry aChild);`
    #[inline]
    pub unsafe fn GetChildSHEntryIfHasNoDynamicallyAddedChild(&self, aChildOffset: i32, aChild: *mut *const nsISHEntry) -> libc::c_void {
        ((*self.vtable).GetChildSHEntryIfHasNoDynamicallyAddedChild)(self, aChildOffset, aChild)
    }


    /// ```text
    /// /**
    ///      * Replaces a child which is for the same docshell as aNewChild
    ///      * with aNewChild.
    ///      * @throw if nothing was replaced.
    ///      */
    /// ```
    ///

    /// `[noscript] void ReplaceChild (in nsISHEntry aNewChild);`
    #[inline]
    pub unsafe fn ReplaceChild(&self, aNewChild: *const nsISHEntry) -> ::nserror::nsresult {
        ((*self.vtable).ReplaceChild)(self, aNewChild)
    }


    /// ```text
    /// /**
    ///     * Remove all children of this entry and call abandonBFCacheEntry.
    ///     */
    /// ```
    ///

    /// `[notxpcom] void ClearEntry ();`
    #[inline]
    pub unsafe fn ClearEntry(&self, ) -> libc::c_void {
        ((*self.vtable).ClearEntry)(self, )
    }


    /// ```text
    /// /**
    ///      * Create nsDocShellLoadState and fill it with information.
    ///      * Don't set nsSHEntry here to avoid serializing it.
    ///      */
    /// ```
    ///

    /// `[noscript] nsDocShellLoadStatePtr CreateLoadInfo ();`
    const _CreateLoadInfo: () = ();


    /// `[infallible] readonly attribute unsigned long long bfcacheID;`
    #[inline]
    pub unsafe fn GetBfcacheID(&self) -> u64 {
        let mut result = <u64 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetBfcacheID)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///      * Sync up the docshell and session history trees for subframe navigation.
    ///      *
    ///      * @param aEntry                    new entry
    ///      * @param aTopBC                    top BC corresponding to the root ancestor
    ///                                         of the docshell that called this method
    ///      * @param aIgnoreBC                 current BC
    ///      */
    /// ```
    ///

    /// `[notxpcom] void SyncTreesForSubframeNavigation (in nsISHEntry aEntry, in BrowsingContext aTopBC, in BrowsingContext aIgnoreBC);`
    #[inline]
    pub unsafe fn SyncTreesForSubframeNavigation(&self, aEntry: *const nsISHEntry, aTopBC: *const libc::c_void, aIgnoreBC: *const libc::c_void) -> libc::c_void {
        ((*self.vtable).SyncTreesForSubframeNavigation)(self, aEntry, aTopBC, aIgnoreBC)
    }


}


