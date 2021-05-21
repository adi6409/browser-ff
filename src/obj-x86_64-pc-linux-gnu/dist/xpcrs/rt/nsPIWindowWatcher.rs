//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/windowwatcher/nsPIWindowWatcher.idl
//


/// `interface nsPIWindowWatcher : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsPIWindowWatcher {
    vtable: *const nsPIWindowWatcherVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsPIWindowWatcher.
unsafe impl XpCom for nsPIWindowWatcher {
    const IID: nsIID = nsID(0xd162f9c4, 0x19d5, 0x4723,
        [0x93, 0x1f, 0xf1, 0xe5, 0x1b, 0xfa, 0x9f, 0x68]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsPIWindowWatcher {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsPIWindowWatcher.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsPIWindowWatcherCoerce {
    /// Cheaply cast a value of this type from a `nsPIWindowWatcher`.
    fn coerce_from(v: &nsPIWindowWatcher) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsPIWindowWatcherCoerce for nsPIWindowWatcher {
    #[inline]
    fn coerce_from(v: &nsPIWindowWatcher) -> &Self {
        v
    }
}

impl nsPIWindowWatcher {
    /// Cast this `nsPIWindowWatcher` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsPIWindowWatcherCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsPIWindowWatcher {
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
impl<T: nsISupportsCoerce> nsPIWindowWatcherCoerce for T {
    #[inline]
    fn coerce_from(v: &nsPIWindowWatcher) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsPIWindowWatcher
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsPIWindowWatcherVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void addWindow (in mozIDOMWindowProxy aWindow, in nsIWebBrowserChrome aChrome); */
    pub AddWindow: unsafe extern "system" fn (this: *const nsPIWindowWatcher, aWindow: *const mozIDOMWindowProxy, aChrome: *const nsIWebBrowserChrome) -> ::nserror::nsresult,

    /* void removeWindow (in mozIDOMWindowProxy aWindow); */
    pub RemoveWindow: unsafe extern "system" fn (this: *const nsPIWindowWatcher, aWindow: *const mozIDOMWindowProxy) -> ::nserror::nsresult,

    /* [noscript] BrowsingContext openWindow2 (in mozIDOMWindowProxy aParent, in ACString aUrl, in ACString aName, in ACString aFeatures, in boolean aCalledFromScript, in boolean aDialog, in boolean aNavigate, in nsISupports aArgs, in boolean aIsPopupSpam, in boolean aForceNoOpener, in boolean aForceNoReferrer, in nsPIWindowWatcher_PrintKind aPrintKind, in nsDocShellLoadStatePtr aLoadState); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub OpenWindow2: *const ::libc::c_void,

    /* nsIRemoteTab openWindowWithRemoteTab (in nsIRemoteTab aOpeningTab, in ACString aFeatures, in boolean aCalledFromJS, in float aOpenerFullZoom, in nsIOpenWindowInfo aOpenWindowInfo); */
    pub OpenWindowWithRemoteTab: unsafe extern "system" fn (this: *const nsPIWindowWatcher, aOpeningTab: *const nsIRemoteTab, aFeatures: *const ::nsstring::nsACString, aCalledFromJS: bool, aOpenerFullZoom: libc::c_float, aOpenWindowInfo: *const nsIOpenWindowInfo, _retval: *mut*const nsIRemoteTab) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsPIWindowWatcher {

    /// ```text
    /// /** A window has been created. Add it to our list.
    ///       @param aWindow the window to add
    ///       @param aChrome the corresponding chrome window. The DOM window
    ///                      and chrome will be mapped together, and the corresponding
    ///                      chrome can be retrieved using the (not private)
    ///                      method getChromeForWindow. If null, any extant mapping
    ///                      will be cleared.
    ///   */
    /// ```
    ///

    /// `void addWindow (in mozIDOMWindowProxy aWindow, in nsIWebBrowserChrome aChrome);`
    #[inline]
    pub unsafe fn AddWindow(&self, aWindow: *const mozIDOMWindowProxy, aChrome: *const nsIWebBrowserChrome) -> ::nserror::nsresult {
        ((*self.vtable).AddWindow)(self, aWindow, aChrome)
    }


    /// ```text
    /// /** A window has been closed. Remove it from our list.
    ///       @param aWindow the window to remove
    ///   */
    /// ```
    ///

    /// `void removeWindow (in mozIDOMWindowProxy aWindow);`
    #[inline]
    pub unsafe fn RemoveWindow(&self, aWindow: *const mozIDOMWindowProxy) -> ::nserror::nsresult {
        ((*self.vtable).RemoveWindow)(self, aWindow)
    }


    /// ```text
    /// /** Like the public interface's open(), but can handle openDialog-style
    ///       arguments and calls which shouldn't result in us navigating the window.
    ///
    ///       @param aParent parent window, if any. Null if no parent.  If it is
    ///              impossible to get to an nsIWebBrowserChrome from aParent, this
    ///              method will effectively act as if aParent were null.
    ///       @param aURL url to which to open the new window. Must already be
    ///              escaped, if applicable. can be null.
    ///       @param aName window name from JS window.open. can be null.  If a window
    ///              with this name already exists, the openWindow call may just load
    ///              aUrl in it (if aUrl is not null) and return it.
    ///       @param aFeatures window features from JS window.open. can be null.
    ///       @param aCalledFromScript true if we were called from script.
    ///       @param aDialog use dialog defaults (see nsGlobalWindowOuter::OpenInternal)
    ///       @param aNavigate true if we should navigate the new window to the
    ///              specified URL.
    ///       @param aArgs Window argument
    ///       @param aIsPopupSpam true if the window is a popup spam window; used for
    ///                           popup blocker internals.
    ///       @param aForceNoOpener If true, force noopener behavior.  This means not
    ///                             looking for existing windows with the given name,
    ///                             not setting an opener on the newly opened window,
    ///                             and returning null from this method.
    ///       @param aLoadState if aNavigate is true, this allows the caller to pass in
    ///                         an nsIDocShellLoadState to use for the navigation.
    ///                        Callers can pass in null if they want the windowwatcher
    ///                        to just construct a loadinfo itself.  If aNavigate is
    ///                        false, this argument is ignored.
    ///
    ///       @return the new window
    ///
    ///       @note This method may examine the JS context stack for purposes of
    ///             determining the security context to use for the search for a given
    ///             window named aName.
    ///       @note This method should try to set the default charset for the new
    ///             window to the default charset of the document in the calling window
    ///             (which is determined based on the JS stack and the value of
        ///             aParent).  This is not guaranteed, however.
    ///   */
    /// ```
    ///

    /// `[noscript] BrowsingContext openWindow2 (in mozIDOMWindowProxy aParent, in ACString aUrl, in ACString aName, in ACString aFeatures, in boolean aCalledFromScript, in boolean aDialog, in boolean aNavigate, in nsISupports aArgs, in boolean aIsPopupSpam, in boolean aForceNoOpener, in boolean aForceNoReferrer, in nsPIWindowWatcher_PrintKind aPrintKind, in nsDocShellLoadStatePtr aLoadState);`
    const _OpenWindow2: () = ();

    /// ```text
    /// /**
    ///    * Opens a new window so that the window that aOpeningTab belongs to
    ///    * is set as the parent window. The newly opened window will also
    ///    * inherit load context information from aOpeningTab.
    ///    *
    ///    * @param aOpeningTab
    ///    *        The nsIRemoteTab that is requesting the new window be opened.
    ///    * @param aFeatures
    ///    *        Window features if called with window.open or similar.
    ///    * @param aCalledFromJS
    ///    *        True if called via window.open or similar.
    ///    * @param aOpenerFullZoom
    ///    *        The current zoom multiplier for the opener tab. This is then
    ///    *        applied to the newly opened window.
    ///    * @param aOpenWindowInfo
    ///    *        Information used to create the initial content browser in the new
    ///    *        window.
    ///    *
    ///    * @return the nsIRemoteTab of the initial browser for the newly opened
    ///    *         window.
    ///    */
    /// ```
    ///

    /// `nsIRemoteTab openWindowWithRemoteTab (in nsIRemoteTab aOpeningTab, in ACString aFeatures, in boolean aCalledFromJS, in float aOpenerFullZoom, in nsIOpenWindowInfo aOpenWindowInfo);`
    #[inline]
    pub unsafe fn OpenWindowWithRemoteTab(&self, aOpeningTab: *const nsIRemoteTab, aFeatures: *const ::nsstring::nsACString, aCalledFromJS: bool, aOpenerFullZoom: libc::c_float, aOpenWindowInfo: *const nsIOpenWindowInfo, _retval: *mut*const nsIRemoteTab) -> ::nserror::nsresult {
        ((*self.vtable).OpenWindowWithRemoteTab)(self, aOpeningTab, aFeatures, aCalledFromJS, aOpenerFullZoom, aOpenWindowInfo, _retval)
    }


}


