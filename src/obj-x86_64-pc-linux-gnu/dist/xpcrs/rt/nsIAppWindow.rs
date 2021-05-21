//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpfe/appshell/nsIAppWindow.idl
//


/// `interface nsIAppWindow : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAppWindow {
    vtable: *const nsIAppWindowVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAppWindow.
unsafe impl XpCom for nsIAppWindow {
    const IID: nsIID = nsID(0xd6d7a014, 0xe28d, 0x4c9d,
        [0x87, 0x27, 0x1c, 0xf6, 0xd8, 0x70, 0x61, 0x9b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAppWindow {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAppWindow.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAppWindowCoerce {
    /// Cheaply cast a value of this type from a `nsIAppWindow`.
    fn coerce_from(v: &nsIAppWindow) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAppWindowCoerce for nsIAppWindow {
    #[inline]
    fn coerce_from(v: &nsIAppWindow) -> &Self {
        v
    }
}

impl nsIAppWindow {
    /// Cast this `nsIAppWindow` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAppWindowCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAppWindow {
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
impl<T: nsISupportsCoerce> nsIAppWindowCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAppWindow) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAppWindow
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAppWindowVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIDocShell docShell; */
    pub GetDocShell: unsafe extern "system" fn (this: *const nsIAppWindow, aDocShell: *mut*const nsIDocShell) -> ::nserror::nsresult,

    /* attribute boolean intrinsicallySized; */
    pub GetIntrinsicallySized: unsafe extern "system" fn (this: *const nsIAppWindow, aIntrinsicallySized: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean intrinsicallySized; */
    pub SetIntrinsicallySized: unsafe extern "system" fn (this: *const nsIAppWindow, aIntrinsicallySized: bool) -> ::nserror::nsresult,

    /* readonly attribute nsIDocShellTreeItem primaryContentShell; */
    pub GetPrimaryContentShell: unsafe extern "system" fn (this: *const nsIAppWindow, aPrimaryContentShell: *mut*const nsIDocShellTreeItem) -> ::nserror::nsresult,

    /* readonly attribute nsIRemoteTab primaryRemoteTab; */
    pub GetPrimaryRemoteTab: unsafe extern "system" fn (this: *const nsIAppWindow, aPrimaryRemoteTab: *mut*const nsIRemoteTab) -> ::nserror::nsresult,

    /* void remoteTabAdded (in nsIRemoteTab aTab, in boolean aPrimary); */
    pub RemoteTabAdded: unsafe extern "system" fn (this: *const nsIAppWindow, aTab: *const nsIRemoteTab, aPrimary: bool) -> ::nserror::nsresult,

    /* void remoteTabRemoved (in nsIRemoteTab aTab); */
    pub RemoteTabRemoved: unsafe extern "system" fn (this: *const nsIAppWindow, aTab: *const nsIRemoteTab) -> ::nserror::nsresult,

    /* [noscript,notxpcom] LiveResizeListenerArray getLiveResizeListeners (); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetLiveResizeListeners: *const ::libc::c_void,

    /* void addChildWindow (in nsIAppWindow aChild); */
    pub AddChildWindow: unsafe extern "system" fn (this: *const nsIAppWindow, aChild: *const nsIAppWindow) -> ::nserror::nsresult,

    /* [infallible] readonly attribute unsigned long outerToInnerHeightDifferenceInCSSPixels; */
    pub GetOuterToInnerHeightDifferenceInCSSPixels: unsafe extern "system" fn (this: *const nsIAppWindow, aOuterToInnerHeightDifferenceInCSSPixels: *mut u32) -> ::nserror::nsresult,

    /* [infallible] readonly attribute unsigned long outerToInnerWidthDifferenceInCSSPixels; */
    pub GetOuterToInnerWidthDifferenceInCSSPixels: unsafe extern "system" fn (this: *const nsIAppWindow, aOuterToInnerWidthDifferenceInCSSPixels: *mut u32) -> ::nserror::nsresult,

    /* void removeChildWindow (in nsIAppWindow aChild); */
    pub RemoveChildWindow: unsafe extern "system" fn (this: *const nsIAppWindow, aChild: *const nsIAppWindow) -> ::nserror::nsresult,

    /* void center (in nsIAppWindow aRelative, in boolean aScreen, in boolean aAlert); */
    pub Center: unsafe extern "system" fn (this: *const nsIAppWindow, aRelative: *const nsIAppWindow, aScreen: bool, aAlert: bool) -> ::nserror::nsresult,

    /* void showModal (); */
    pub ShowModal: unsafe extern "system" fn (this: *const nsIAppWindow) -> ::nserror::nsresult,

    /* void lockAspectRatio (in bool aShouldLock); */
    pub LockAspectRatio: unsafe extern "system" fn (this: *const nsIAppWindow, aShouldLock: bool) -> ::nserror::nsresult,

    /* attribute unsigned long zLevel; */
    pub GetZLevel: unsafe extern "system" fn (this: *const nsIAppWindow, aZLevel: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long zLevel; */
    pub SetZLevel: unsafe extern "system" fn (this: *const nsIAppWindow, aZLevel: u32) -> ::nserror::nsresult,

    /* attribute uint32_t chromeFlags; */
    pub GetChromeFlags: unsafe extern "system" fn (this: *const nsIAppWindow, aChromeFlags: *mut uint32_t) -> ::nserror::nsresult,

    /* attribute uint32_t chromeFlags; */
    pub SetChromeFlags: unsafe extern "system" fn (this: *const nsIAppWindow, aChromeFlags: uint32_t) -> ::nserror::nsresult,

    /* void assumeChromeFlagsAreFrozen (); */
    pub AssumeChromeFlagsAreFrozen: unsafe extern "system" fn (this: *const nsIAppWindow) -> ::nserror::nsresult,

    /* nsIAppWindow createNewWindow (in int32_t aChromeFlags, in nsIOpenWindowInfo aOpenWindowInfo); */
    pub CreateNewWindow: unsafe extern "system" fn (this: *const nsIAppWindow, aChromeFlags: int32_t, aOpenWindowInfo: *const nsIOpenWindowInfo, _retval: *mut *const nsIAppWindow) -> ::nserror::nsresult,

    /* attribute nsIXULBrowserWindow XULBrowserWindow; */
    pub GetXULBrowserWindow: unsafe extern "system" fn (this: *const nsIAppWindow, aXULBrowserWindow: *mut*const nsIXULBrowserWindow) -> ::nserror::nsresult,

    /* attribute nsIXULBrowserWindow XULBrowserWindow; */
    pub SetXULBrowserWindow: unsafe extern "system" fn (this: *const nsIAppWindow, aXULBrowserWindow: *const nsIXULBrowserWindow) -> ::nserror::nsresult,

    /* [noscript] void beforeStartLayout (); */
    pub BeforeStartLayout: unsafe extern "system" fn (this: *const nsIAppWindow) -> ::nserror::nsresult,

    /* [noscript,notxpcom] void sizeShellToWithLimit (in int32_t aDesiredWidth, in int32_t aDesiredHeight, in int32_t shellItemWidth, in int32_t shellItemHeight); */
    pub SizeShellToWithLimit: unsafe extern "system" fn (this: *const nsIAppWindow, aDesiredWidth: int32_t, aDesiredHeight: int32_t, shellItemWidth: int32_t, shellItemHeight: int32_t) -> libc::c_void,

    /* readonly attribute nsIOpenWindowInfo initialOpenWindowInfo; */
    pub GetInitialOpenWindowInfo: unsafe extern "system" fn (this: *const nsIAppWindow, aInitialOpenWindowInfo: *mut*const nsIOpenWindowInfo) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAppWindow {

    pub const lowestZ: i64 = 0;


    pub const loweredZ: i64 = 4;


    pub const normalZ: i64 = 5;


    pub const raisedZ: i64 = 6;


    pub const highestZ: i64 = 9;

    /// ```text
    /// /**
    ///    * The docshell owning the XUL for this window.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIDocShell docShell;`
    #[inline]
    pub unsafe fn GetDocShell(&self, aDocShell: *mut*const nsIDocShell) -> ::nserror::nsresult {
        ((*self.vtable).GetDocShell)(self, aDocShell)
    }


    /// ```text
    /// /**
    ///    * Indicates if this window is instrinsically sized.
    ///    */
    /// ```
    ///

    /// `attribute boolean intrinsicallySized;`
    #[inline]
    pub unsafe fn GetIntrinsicallySized(&self, aIntrinsicallySized: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIntrinsicallySized)(self, aIntrinsicallySized)
    }


    /// ```text
    /// /**
    ///    * Indicates if this window is instrinsically sized.
    ///    */
    /// ```
    ///

    /// `attribute boolean intrinsicallySized;`
    #[inline]
    pub unsafe fn SetIntrinsicallySized(&self, aIntrinsicallySized: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetIntrinsicallySized)(self, aIntrinsicallySized)
    }


    /// ```text
    /// /**
    ///    * The primary content shell.
    ///    *
    ///    * Note that this is a docshell tree item and therefore can not be assured of
    ///    * what object it is. It could be an editor, a docshell, or a browser object.
    ///    * Or down the road any other object that supports being a DocShellTreeItem
    ///    * Query accordingly to determine the capabilities.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIDocShellTreeItem primaryContentShell;`
    #[inline]
    pub unsafe fn GetPrimaryContentShell(&self, aPrimaryContentShell: *mut*const nsIDocShellTreeItem) -> ::nserror::nsresult {
        ((*self.vtable).GetPrimaryContentShell)(self, aPrimaryContentShell)
    }


    /// ```text
    /// /**
    ///    * In multiprocess case we may not have primaryContentShell but
    ///    * primaryRemoteTab.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIRemoteTab primaryRemoteTab;`
    #[inline]
    pub unsafe fn GetPrimaryRemoteTab(&self, aPrimaryRemoteTab: *mut*const nsIRemoteTab) -> ::nserror::nsresult {
        ((*self.vtable).GetPrimaryRemoteTab)(self, aPrimaryRemoteTab)
    }



    /// `void remoteTabAdded (in nsIRemoteTab aTab, in boolean aPrimary);`
    #[inline]
    pub unsafe fn RemoteTabAdded(&self, aTab: *const nsIRemoteTab, aPrimary: bool) -> ::nserror::nsresult {
        ((*self.vtable).RemoteTabAdded)(self, aTab, aPrimary)
    }



    /// `void remoteTabRemoved (in nsIRemoteTab aTab);`
    #[inline]
    pub unsafe fn RemoteTabRemoved(&self, aTab: *const nsIRemoteTab) -> ::nserror::nsresult {
        ((*self.vtable).RemoteTabRemoved)(self, aTab)
    }



    /// `[noscript,notxpcom] LiveResizeListenerArray getLiveResizeListeners ();`
    const _GetLiveResizeListeners: () = ();

    /// ```text
    /// /**
    ///    * Tell this window that it has picked up a child XUL window
    ///    * @param aChild the child window being added
    ///    */
    /// ```
    ///

    /// `void addChildWindow (in nsIAppWindow aChild);`
    #[inline]
    pub unsafe fn AddChildWindow(&self, aChild: *const nsIAppWindow) -> ::nserror::nsresult {
        ((*self.vtable).AddChildWindow)(self, aChild)
    }


    /// ```text
    /// /**
    ///    * Returns the difference between the inner window size (client size) and the
    ///    * outer window size, in CSS pixels.
    ///    */
    /// ```
    ///

    /// `[infallible] readonly attribute unsigned long outerToInnerHeightDifferenceInCSSPixels;`
    #[inline]
    pub unsafe fn GetOuterToInnerHeightDifferenceInCSSPixels(&self) -> u32 {
        let mut result = <u32 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetOuterToInnerHeightDifferenceInCSSPixels)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `[infallible] readonly attribute unsigned long outerToInnerWidthDifferenceInCSSPixels;`
    #[inline]
    pub unsafe fn GetOuterToInnerWidthDifferenceInCSSPixels(&self) -> u32 {
        let mut result = <u32 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetOuterToInnerWidthDifferenceInCSSPixels)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///    * Tell this window that it has lost a child XUL window
    ///    * @param aChild the child window being removed
    ///    */
    /// ```
    ///

    /// `void removeChildWindow (in nsIAppWindow aChild);`
    #[inline]
    pub unsafe fn RemoveChildWindow(&self, aChild: *const nsIAppWindow) -> ::nserror::nsresult {
        ((*self.vtable).RemoveChildWindow)(self, aChild)
    }


    /// ```text
    /// /**
    ///    * Move the window to a centered position.
    ///    * @param aRelative If not null, the window relative to which the window is
    ///    *                  moved. See aScreen parameter for details.
    ///    * @param aScreen   PR_TRUE to center the window relative to the screen
    ///    *                  containing aRelative if aRelative is not null. If
    ///    *                  aRelative is null then relative to the screen of the
    ///    *                  opener window if it was initialized by passing it to
    ///    *                  nsWebShellWindow::Initialize. Failing that relative to
    ///    *                  the main screen.
    ///    *                  PR_FALSE to center it relative to aRelative itself.
    ///    * @param aAlert    PR_TRUE to move the window to an alert position,
    ///    *                  generally centered horizontally and 1/3 down from the top.
    ///    */
    /// ```
    ///

    /// `void center (in nsIAppWindow aRelative, in boolean aScreen, in boolean aAlert);`
    #[inline]
    pub unsafe fn Center(&self, aRelative: *const nsIAppWindow, aScreen: bool, aAlert: bool) -> ::nserror::nsresult {
        ((*self.vtable).Center)(self, aRelative, aScreen, aAlert)
    }


    /// ```text
    /// /**
    ///    * Shows the window as a modal window. That is, ensures that it is visible
    ///    * and runs a local event loop, exiting only once the window has been closed.
    ///    */
    /// ```
    ///

    /// `void showModal ();`
    #[inline]
    pub unsafe fn ShowModal(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ShowModal)(self, )
    }


    /// ```text
    /// /**
    ///    * Locks the aspect ratio for a window.
    ///    * @param aShouldLock boolean
    ///    */
    /// ```
    ///

    /// `void lockAspectRatio (in bool aShouldLock);`
    #[inline]
    pub unsafe fn LockAspectRatio(&self, aShouldLock: bool) -> ::nserror::nsresult {
        ((*self.vtable).LockAspectRatio)(self, aShouldLock)
    }



    /// `attribute unsigned long zLevel;`
    #[inline]
    pub unsafe fn GetZLevel(&self, aZLevel: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetZLevel)(self, aZLevel)
    }



    /// `attribute unsigned long zLevel;`
    #[inline]
    pub unsafe fn SetZLevel(&self, aZLevel: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetZLevel)(self, aZLevel)
    }



    /// `attribute uint32_t chromeFlags;`
    #[inline]
    pub unsafe fn GetChromeFlags(&self, aChromeFlags: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetChromeFlags)(self, aChromeFlags)
    }



    /// `attribute uint32_t chromeFlags;`
    #[inline]
    pub unsafe fn SetChromeFlags(&self, aChromeFlags: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).SetChromeFlags)(self, aChromeFlags)
    }


    /// ```text
    /// /**
    ///    * Begin assuming |chromeFlags| don't change hereafter, and assert
    ///    * if they do change.  The state change is one-way and idempotent.
    ///    */
    /// ```
    ///

    /// `void assumeChromeFlagsAreFrozen ();`
    #[inline]
    pub unsafe fn AssumeChromeFlagsAreFrozen(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).AssumeChromeFlagsAreFrozen)(self, )
    }


    /// ```text
    /// /**
    ///    * Create a new window.
    ///    * @param aChromeFlags see nsIWebBrowserChrome
    ///    * @param aOpenWindowInfo information about the request for a content window
    ///    *                        to be opened. Will be null for non-content loads.
    ///    * @return the newly minted window
    ///    */
    /// ```
    ///

    /// `nsIAppWindow createNewWindow (in int32_t aChromeFlags, in nsIOpenWindowInfo aOpenWindowInfo);`
    #[inline]
    pub unsafe fn CreateNewWindow(&self, aChromeFlags: int32_t, aOpenWindowInfo: *const nsIOpenWindowInfo, _retval: *mut *const nsIAppWindow) -> ::nserror::nsresult {
        ((*self.vtable).CreateNewWindow)(self, aChromeFlags, aOpenWindowInfo, _retval)
    }



    /// `attribute nsIXULBrowserWindow XULBrowserWindow;`
    #[inline]
    pub unsafe fn GetXULBrowserWindow(&self, aXULBrowserWindow: *mut*const nsIXULBrowserWindow) -> ::nserror::nsresult {
        ((*self.vtable).GetXULBrowserWindow)(self, aXULBrowserWindow)
    }



    /// `attribute nsIXULBrowserWindow XULBrowserWindow;`
    #[inline]
    pub unsafe fn SetXULBrowserWindow(&self, aXULBrowserWindow: *const nsIXULBrowserWindow) -> ::nserror::nsresult {
        ((*self.vtable).SetXULBrowserWindow)(self, aXULBrowserWindow)
    }


    /// ```text
    /// /**
    ///    * Back-door method to make sure some stuff is done when the document is
    ///    * ready for layout, that would cause expensive computation otherwise later.
    ///    *
    ///    * Do NOT call this unless you know what you're doing!  In particular,
    ///    * calling this when this XUL window doesn't yet have a document in its
    ///    * docshell could cause problems.
    ///    */
    /// ```
    ///

    /// `[noscript] void beforeStartLayout ();`
    #[inline]
    pub unsafe fn BeforeStartLayout(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).BeforeStartLayout)(self, )
    }


    /// ```text
    /// /**
    ///    * Given the dimensions of some content area held within this
    ///    * XUL window, and assuming that that content area will change
    ///    * its dimensions in linear proportion to the dimensions of this
    ///    * XUL window, changes the size of the XUL window so that the
    ///    * content area reaches a particular size.
    ///    *
    ///    * We need to supply the content area dimensions because sometimes
    ///    * the child's nsDocShellTreeOwner needs to propagate a SizeShellTo
    ///    * call to the parent. But the shellItem argument of the call will
    ///    * not be available on the parent side.
    ///    *
    ///    * Note: this is an internal method, other consumers should never call this.
    ///    *
    ///    * @param aDesiredWidth
    ///    *        The desired width of the content area in device pixels.
    ///    * @param aDesiredHeight
    ///    *        The desired height of the content area in device pixels.
    ///    * @param shellItemWidth
    ///    *        The current width of the content area.
    ///    * @param shellItemHeight
    ///    *        The current height of the content area.
    ///    */
    /// ```
    ///

    /// `[noscript,notxpcom] void sizeShellToWithLimit (in int32_t aDesiredWidth, in int32_t aDesiredHeight, in int32_t shellItemWidth, in int32_t shellItemHeight);`
    #[inline]
    pub unsafe fn SizeShellToWithLimit(&self, aDesiredWidth: int32_t, aDesiredHeight: int32_t, shellItemWidth: int32_t, shellItemHeight: int32_t) -> libc::c_void {
        ((*self.vtable).SizeShellToWithLimit)(self, aDesiredWidth, aDesiredHeight, shellItemWidth, shellItemHeight)
    }


    /// ```text
    /// /**
    ///    * If the window was opened as a content window, this will return the initial
    ///    * nsIOpenWindowInfo to use.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIOpenWindowInfo initialOpenWindowInfo;`
    #[inline]
    pub unsafe fn GetInitialOpenWindowInfo(&self, aInitialOpenWindowInfo: *mut*const nsIOpenWindowInfo) -> ::nserror::nsresult {
        ((*self.vtable).GetInitialOpenWindowInfo)(self, aInitialOpenWindowInfo)
    }


}


