//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIBrowserDOMWindow.idl
//


/// `interface nsIOpenURIInFrameParams : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIOpenURIInFrameParams {
    vtable: *const nsIOpenURIInFrameParamsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIOpenURIInFrameParams.
unsafe impl XpCom for nsIOpenURIInFrameParams {
    const IID: nsIID = nsID(0xe774db14, 0x79ac, 0x4156,
        [0xa7, 0xa3, 0xaa, 0x3f, 0xd0, 0xa2, 0x2c, 0x10]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIOpenURIInFrameParams {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIOpenURIInFrameParams.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIOpenURIInFrameParamsCoerce {
    /// Cheaply cast a value of this type from a `nsIOpenURIInFrameParams`.
    fn coerce_from(v: &nsIOpenURIInFrameParams) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIOpenURIInFrameParamsCoerce for nsIOpenURIInFrameParams {
    #[inline]
    fn coerce_from(v: &nsIOpenURIInFrameParams) -> &Self {
        v
    }
}

impl nsIOpenURIInFrameParams {
    /// Cast this `nsIOpenURIInFrameParams` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIOpenURIInFrameParamsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIOpenURIInFrameParams {
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
impl<T: nsISupportsCoerce> nsIOpenURIInFrameParamsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIOpenURIInFrameParams) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIOpenURIInFrameParams
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIOpenURIInFrameParamsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIOpenWindowInfo openWindowInfo; */
    pub GetOpenWindowInfo: unsafe extern "system" fn (this: *const nsIOpenURIInFrameParams, aOpenWindowInfo: *mut*const nsIOpenWindowInfo) -> ::nserror::nsresult,

    /* attribute nsIReferrerInfo referrerInfo; */
    pub GetReferrerInfo: unsafe extern "system" fn (this: *const nsIOpenURIInFrameParams, aReferrerInfo: *mut*const nsIReferrerInfo) -> ::nserror::nsresult,

    /* attribute nsIReferrerInfo referrerInfo; */
    pub SetReferrerInfo: unsafe extern "system" fn (this: *const nsIOpenURIInFrameParams, aReferrerInfo: *const nsIReferrerInfo) -> ::nserror::nsresult,

    /* readonly attribute boolean isPrivate; */
    pub GetIsPrivate: unsafe extern "system" fn (this: *const nsIOpenURIInFrameParams, aIsPrivate: *mut bool) -> ::nserror::nsresult,

    /* attribute nsIPrincipal triggeringPrincipal; */
    pub GetTriggeringPrincipal: unsafe extern "system" fn (this: *const nsIOpenURIInFrameParams, aTriggeringPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult,

    /* attribute nsIPrincipal triggeringPrincipal; */
    pub SetTriggeringPrincipal: unsafe extern "system" fn (this: *const nsIOpenURIInFrameParams, aTriggeringPrincipal: *const nsIPrincipal) -> ::nserror::nsresult,

    /* attribute nsIContentSecurityPolicy csp; */
    pub GetCsp: unsafe extern "system" fn (this: *const nsIOpenURIInFrameParams, aCsp: *mut*const nsIContentSecurityPolicy) -> ::nserror::nsresult,

    /* attribute nsIContentSecurityPolicy csp; */
    pub SetCsp: unsafe extern "system" fn (this: *const nsIOpenURIInFrameParams, aCsp: *const nsIContentSecurityPolicy) -> ::nserror::nsresult,

    /* readonly attribute Element openerBrowser; */
    pub GetOpenerBrowser: unsafe extern "system" fn (this: *const nsIOpenURIInFrameParams, aOpenerBrowser: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* [implicit_jscontext] readonly attribute jsval openerOriginAttributes; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetOpenerOriginAttributes: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIOpenURIInFrameParams {


    /// `readonly attribute nsIOpenWindowInfo openWindowInfo;`
    #[inline]
    pub unsafe fn GetOpenWindowInfo(&self, aOpenWindowInfo: *mut*const nsIOpenWindowInfo) -> ::nserror::nsresult {
        ((*self.vtable).GetOpenWindowInfo)(self, aOpenWindowInfo)
    }



    /// `attribute nsIReferrerInfo referrerInfo;`
    #[inline]
    pub unsafe fn GetReferrerInfo(&self, aReferrerInfo: *mut*const nsIReferrerInfo) -> ::nserror::nsresult {
        ((*self.vtable).GetReferrerInfo)(self, aReferrerInfo)
    }



    /// `attribute nsIReferrerInfo referrerInfo;`
    #[inline]
    pub unsafe fn SetReferrerInfo(&self, aReferrerInfo: *const nsIReferrerInfo) -> ::nserror::nsresult {
        ((*self.vtable).SetReferrerInfo)(self, aReferrerInfo)
    }



    /// `readonly attribute boolean isPrivate;`
    #[inline]
    pub unsafe fn GetIsPrivate(&self, aIsPrivate: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsPrivate)(self, aIsPrivate)
    }



    /// `attribute nsIPrincipal triggeringPrincipal;`
    #[inline]
    pub unsafe fn GetTriggeringPrincipal(&self, aTriggeringPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).GetTriggeringPrincipal)(self, aTriggeringPrincipal)
    }



    /// `attribute nsIPrincipal triggeringPrincipal;`
    #[inline]
    pub unsafe fn SetTriggeringPrincipal(&self, aTriggeringPrincipal: *const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).SetTriggeringPrincipal)(self, aTriggeringPrincipal)
    }



    /// `attribute nsIContentSecurityPolicy csp;`
    #[inline]
    pub unsafe fn GetCsp(&self, aCsp: *mut*const nsIContentSecurityPolicy) -> ::nserror::nsresult {
        ((*self.vtable).GetCsp)(self, aCsp)
    }



    /// `attribute nsIContentSecurityPolicy csp;`
    #[inline]
    pub unsafe fn SetCsp(&self, aCsp: *const nsIContentSecurityPolicy) -> ::nserror::nsresult {
        ((*self.vtable).SetCsp)(self, aCsp)
    }



    /// `readonly attribute Element openerBrowser;`
    #[inline]
    pub unsafe fn GetOpenerBrowser(&self, aOpenerBrowser: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetOpenerBrowser)(self, aOpenerBrowser)
    }



    /// `[implicit_jscontext] readonly attribute jsval openerOriginAttributes;`
    const _GetOpenerOriginAttributes: () = ();

}


/// `interface nsIBrowserDOMWindow : nsISupports`
///

/// ```text
/// /**
///  * The C++ source has access to the browser script source through
///  * nsIBrowserDOMWindow. It is intended to be attached to the chrome DOMWindow
///  * of a toplevel browser window (a XUL window). A DOMWindow that does not
///  * happen to be a browser chrome window will simply have no access to any such
///  * interface.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIBrowserDOMWindow {
    vtable: *const nsIBrowserDOMWindowVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIBrowserDOMWindow.
unsafe impl XpCom for nsIBrowserDOMWindow {
    const IID: nsIID = nsID(0x2a9bb880, 0x5d73, 0x40f3,
        [0x81, 0x52, 0xc6, 0x0c, 0x8d, 0x13, 0x7a, 0x14]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIBrowserDOMWindow {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIBrowserDOMWindow.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIBrowserDOMWindowCoerce {
    /// Cheaply cast a value of this type from a `nsIBrowserDOMWindow`.
    fn coerce_from(v: &nsIBrowserDOMWindow) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIBrowserDOMWindowCoerce for nsIBrowserDOMWindow {
    #[inline]
    fn coerce_from(v: &nsIBrowserDOMWindow) -> &Self {
        v
    }
}

impl nsIBrowserDOMWindow {
    /// Cast this `nsIBrowserDOMWindow` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIBrowserDOMWindowCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIBrowserDOMWindow {
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
impl<T: nsISupportsCoerce> nsIBrowserDOMWindowCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBrowserDOMWindow) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIBrowserDOMWindow
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIBrowserDOMWindowVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* BrowsingContext createContentWindow (in nsIURI aURI, in nsIOpenWindowInfo aOpenWindowInfo, in short aWhere, in long aFlags, in nsIPrincipal aTriggeringPrincipal, [optional] in nsIContentSecurityPolicy aCsp); */
    pub CreateContentWindow: unsafe extern "system" fn (this: *const nsIBrowserDOMWindow, aURI: *const nsIURI, aOpenWindowInfo: *const nsIOpenWindowInfo, aWhere: i16, aFlags: i32, aTriggeringPrincipal: *const nsIPrincipal, aCsp: *const nsIContentSecurityPolicy, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* Element createContentWindowInFrame (in nsIURI aURI, in nsIOpenURIInFrameParams params, in short aWhere, in long aFlags, in AString aName); */
    pub CreateContentWindowInFrame: unsafe extern "system" fn (this: *const nsIBrowserDOMWindow, aURI: *const nsIURI, params: *const nsIOpenURIInFrameParams, aWhere: i16, aFlags: i32, aName: *const ::nsstring::nsAString, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* BrowsingContext openURI (in nsIURI aURI, in nsIOpenWindowInfo aOpenWindowInfo, in short aWhere, in long aFlags, in nsIPrincipal aTriggeringPrincipal, [optional] in nsIContentSecurityPolicy aCsp); */
    pub OpenURI: unsafe extern "system" fn (this: *const nsIBrowserDOMWindow, aURI: *const nsIURI, aOpenWindowInfo: *const nsIOpenWindowInfo, aWhere: i16, aFlags: i32, aTriggeringPrincipal: *const nsIPrincipal, aCsp: *const nsIContentSecurityPolicy, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* Element openURIInFrame (in nsIURI aURI, in nsIOpenURIInFrameParams params, in short aWhere, in long aFlags, in AString aName); */
    pub OpenURIInFrame: unsafe extern "system" fn (this: *const nsIBrowserDOMWindow, aURI: *const nsIURI, params: *const nsIOpenURIInFrameParams, aWhere: i16, aFlags: i32, aName: *const ::nsstring::nsAString, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* boolean canClose (); */
    pub CanClose: unsafe extern "system" fn (this: *const nsIBrowserDOMWindow, _retval: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute unsigned long tabCount; */
    pub GetTabCount: unsafe extern "system" fn (this: *const nsIBrowserDOMWindow, aTabCount: *mut u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIBrowserDOMWindow {
    /// ```text
    /// /**
    ///    * Values for createContentWindow's and openURI's aWhere parameter.
    ///    */
    /// /**
    ///    * Do whatever the default is based on application state, user preferences,
    ///    * and the value of the aContext parameter to openURI.
    ///    */
    /// ```
    ///

    pub const OPEN_DEFAULTWINDOW: i64 = 0;

    /// ```text
    /// /**
    ///    * Open in the "current window".  If aOpener is provided, this should be the
    ///    * top window in aOpener's window hierarchy, but exact behavior is
    ///    * application-dependent.  If aOpener is not provided, it's up to the
    ///    * application to decide what constitutes a "current window".
    ///    */
    /// ```
    ///

    pub const OPEN_CURRENTWINDOW: i64 = 1;

    /// ```text
    /// /**
    ///    * Open in a new window.
    ///    */
    /// ```
    ///

    pub const OPEN_NEWWINDOW: i64 = 2;

    /// ```text
    /// /**
    ///    * Open in a new content tab in the toplevel browser window corresponding to
    ///    * this nsIBrowserDOMWindow.
    ///    */
    /// ```
    ///

    pub const OPEN_NEWTAB: i64 = 3;

    /// ```text
    /// /**
    ///    * Open in a hidden browser. Used for printing.
    ///    */
    /// ```
    ///

    pub const OPEN_PRINT_BROWSER: i64 = 4;

    /// ```text
    /// /**
    ///    * Values for createContentWindow's and openURI's aFlags parameter.
    ///    * This is a bitflags field.
    ///    *
    ///    * The 0x1 bit decides the behavior of OPEN_DEFAULTWINDOW, and the 0x4 bit
    ///    * controls whether or not to set the window.opener property on the newly
    ///    * opened window.
    ///    *
    ///    * NOTE: The 0x2 bit is ignored for backwards compatibility with addons, as
    ///    * OPEN_NEW used to have the value 2. The values 0 and 2 are treated
    ///    * the same way internally.
    ///    */
    /// /**
    ///    * Internal open new window.
    ///    */
    /// ```
    ///

    pub const OPEN_NEW: i64 = 0;

    /// ```text
    /// /**
    ///    * External link (load request from another application, xremote, etc).
    ///    */
    /// ```
    ///

    pub const OPEN_EXTERNAL: i64 = 1;

    /// ```text
    /// /**
    ///    * Don't set the window.opener property on the window which is being opened.
    ///    */
    /// ```
    ///

    pub const OPEN_NO_OPENER: i64 = 4;

    /// ```text
    /// /**
    ///    * Don't set the referrer on the navigation inside the window which is
    ///    * being opened.
    ///    */
    /// ```
    ///

    pub const OPEN_NO_REFERRER: i64 = 8;

    /// ```text
    /// /**
    ///    * Create the content window for the given URI.
    ///
    ///    * @param aURI the URI to be opened in the window (can be null).
    ///    * @param aWhere see possible values described above.
    ///    * @param aOpenWindowInfo info about the creation (can be null).
    ///    * @param aFlags flags which control the behavior of the load. The
    ///    *               OPEN_EXTERNAL/OPEN_NEW flag is only used when
    ///    *               aWhere == OPEN_DEFAULTWINDOW.
    ///    * @param aTriggeringPrincipal the principal that would trigger the potential
    ///    *        load of aURI.
    ///    * @param aCsp the CSP to use (if any) for the new window.
    ///    * @return the window into which the URI would have been opened.
    ///   */
    /// ```
    ///

    /// `BrowsingContext createContentWindow (in nsIURI aURI, in nsIOpenWindowInfo aOpenWindowInfo, in short aWhere, in long aFlags, in nsIPrincipal aTriggeringPrincipal, [optional] in nsIContentSecurityPolicy aCsp);`
    #[inline]
    pub unsafe fn CreateContentWindow(&self, aURI: *const nsIURI, aOpenWindowInfo: *const nsIOpenWindowInfo, aWhere: i16, aFlags: i32, aTriggeringPrincipal: *const nsIPrincipal, aCsp: *const nsIContentSecurityPolicy, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).CreateContentWindow)(self, aURI, aOpenWindowInfo, aWhere, aFlags, aTriggeringPrincipal, aCsp, _retval)
    }


    /// ```text
    /// /**
    ///    * As above, but return the nsFrameLoaderOwner for the new window. Value is
    ///    * returned as Element, QI'd back to nsFrameLoaderOwner as needed.
    ///    *
    ///    * Additional Parameters:
    ///    * @param aName The name to give the window opened in the new tab.
    ///    * @return The frame element for the newly opened window.
    ///    */
    /// ```
    ///

    /// `Element createContentWindowInFrame (in nsIURI aURI, in nsIOpenURIInFrameParams params, in short aWhere, in long aFlags, in AString aName);`
    #[inline]
    pub unsafe fn CreateContentWindowInFrame(&self, aURI: *const nsIURI, params: *const nsIOpenURIInFrameParams, aWhere: i16, aFlags: i32, aName: *const ::nsstring::nsAString, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).CreateContentWindowInFrame)(self, aURI, params, aWhere, aFlags, aName, _retval)
    }


    /// ```text
    /// /**
    ///    * Load a URI.
    ///    * @param aURI the URI to open. null is not allowed. To create the window
    ///    *        without loading the URI, use createContentWindow instead.
    ///    * @param aWhere see possible values described above.
    ///    * @param aOpenWindowInfo info about the open (can be null).
    ///    * @param aFlags flags which control the behavior of the load. The
    ///    *               OPEN_EXTERNAL/OPEN_NEW flag is only used when
    ///    *               aWhere == OPEN_DEFAULTWINDOW.
    ///    * @param aTriggeringPrincipal the principal that triggered the load of aURI.
    ///    * @param aCsp the CSP to be applied to the new load.
    ///    * @return the window into which the URI was opened.
    ///   */
    /// ```
    ///

    /// `BrowsingContext openURI (in nsIURI aURI, in nsIOpenWindowInfo aOpenWindowInfo, in short aWhere, in long aFlags, in nsIPrincipal aTriggeringPrincipal, [optional] in nsIContentSecurityPolicy aCsp);`
    #[inline]
    pub unsafe fn OpenURI(&self, aURI: *const nsIURI, aOpenWindowInfo: *const nsIOpenWindowInfo, aWhere: i16, aFlags: i32, aTriggeringPrincipal: *const nsIPrincipal, aCsp: *const nsIContentSecurityPolicy, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).OpenURI)(self, aURI, aOpenWindowInfo, aWhere, aFlags, aTriggeringPrincipal, aCsp, _retval)
    }


    /// ```text
    /// /**
    ///    * As above, but return the nsFrameLoaderOwner for the new window. Value is
    ///    * returned as Element, QI'd back to nsFrameLoaderOwner as needed.
    ///    *
    ///    * Additional Parameters:
    ///    * @param aName The name to give the window opened in the new tab.
    ///    * @return The frame element for the newly opened window.
    ///    // XXXbz is this the right API?
    ///    // See bug 537428
    ///    */
    /// ```
    ///

    /// `Element openURIInFrame (in nsIURI aURI, in nsIOpenURIInFrameParams params, in short aWhere, in long aFlags, in AString aName);`
    #[inline]
    pub unsafe fn OpenURIInFrame(&self, aURI: *const nsIURI, params: *const nsIOpenURIInFrameParams, aWhere: i16, aFlags: i32, aName: *const ::nsstring::nsAString, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).OpenURIInFrame)(self, aURI, params, aWhere, aFlags, aName, _retval)
    }


    /// ```text
    /// /**
    ///    * This function is responsible for calling
    ///    * nsIContentViewer::PermitUnload on each frame in the window. It
    ///    * returns true if closing the window is allowed. See canClose() in
    ///    * BrowserUtils.jsm for a simple implementation of this method.
    ///    */
    /// ```
    ///

    /// `boolean canClose ();`
    #[inline]
    pub unsafe fn CanClose(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).CanClose)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * The number browser tabs in the window. This number currently includes
    ///    * lazy tabs, though for most uses it probably should not.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long tabCount;`
    #[inline]
    pub unsafe fn GetTabCount(&self, aTabCount: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetTabCount)(self, aTabCount)
    }


}


