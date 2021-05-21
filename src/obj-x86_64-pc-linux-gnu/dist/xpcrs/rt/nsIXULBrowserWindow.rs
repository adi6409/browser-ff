//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpfe/appshell/nsIXULBrowserWindow.idl
//


/// `interface nsIXULBrowserWindow : nsISupports`
///

/// ```text
/// /**
///  * The nsIXULBrowserWindow supplies the methods that may be called from the
///  * internals of the browser area to tell the containing xul window to update
///  * its ui.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIXULBrowserWindow {
    vtable: *const nsIXULBrowserWindowVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIXULBrowserWindow.
unsafe impl XpCom for nsIXULBrowserWindow {
    const IID: nsIID = nsID(0xa8675fa9, 0xc8b4, 0x4350,
        [0x98, 0x03, 0xc3, 0x8f, 0x34, 0x4a, 0x9e, 0x38]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIXULBrowserWindow {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIXULBrowserWindow.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIXULBrowserWindowCoerce {
    /// Cheaply cast a value of this type from a `nsIXULBrowserWindow`.
    fn coerce_from(v: &nsIXULBrowserWindow) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIXULBrowserWindowCoerce for nsIXULBrowserWindow {
    #[inline]
    fn coerce_from(v: &nsIXULBrowserWindow) -> &Self {
        v
    }
}

impl nsIXULBrowserWindow {
    /// Cast this `nsIXULBrowserWindow` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIXULBrowserWindowCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIXULBrowserWindow {
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
impl<T: nsISupportsCoerce> nsIXULBrowserWindowCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXULBrowserWindow) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIXULBrowserWindow
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIXULBrowserWindowVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void setOverLink (in AString link); */
    pub SetOverLink: unsafe extern "system" fn (this: *const nsIXULBrowserWindow, link: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString onBeforeLinkTraversal (in AString originalTarget, in nsIURI linkURI, in Node linkNode, in boolean isAppTab); */
    pub OnBeforeLinkTraversal: unsafe extern "system" fn (this: *const nsIXULBrowserWindow, originalTarget: *const ::nsstring::nsAString, linkURI: *const nsIURI, linkNode: *const libc::c_void, isAppTab: bool, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void showTooltip (in long x, in long y, in AString tooltip, in AString direction, in Element browser); */
    pub ShowTooltip: unsafe extern "system" fn (this: *const nsIXULBrowserWindow, x: i32, y: i32, tooltip: *const ::nsstring::nsAString, direction: *const ::nsstring::nsAString, browser: *const libc::c_void) -> ::nserror::nsresult,

    /* void hideTooltip (); */
    pub HideTooltip: unsafe extern "system" fn (this: *const nsIXULBrowserWindow) -> ::nserror::nsresult,

    /* uint32_t getTabCount (); */
    pub GetTabCount: unsafe extern "system" fn (this: *const nsIXULBrowserWindow, _retval: *mut uint32_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIXULBrowserWindow {

    /// ```text
    /// /**
    ///    * Tells the object implementing this function what link we are currently
    ///    * over.
    ///    */
    /// ```
    ///

    /// `void setOverLink (in AString link);`
    #[inline]
    pub unsafe fn SetOverLink(&self, link: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetOverLink)(self, link)
    }


    /// ```text
    /// /**
    ///    * Determines the appropriate target for a link.
    ///    */
    /// ```
    ///

    /// `AString onBeforeLinkTraversal (in AString originalTarget, in nsIURI linkURI, in Node linkNode, in boolean isAppTab);`
    #[inline]
    pub unsafe fn OnBeforeLinkTraversal(&self, originalTarget: *const ::nsstring::nsAString, linkURI: *const nsIURI, linkNode: *const libc::c_void, isAppTab: bool, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).OnBeforeLinkTraversal)(self, originalTarget, linkURI, linkNode, isAppTab, _retval)
    }


    /// ```text
    /// /**
    ///    * Show/hide a tooltip (when the user mouses over a link, say).
    ///    */
    /// ```
    ///

    /// `void showTooltip (in long x, in long y, in AString tooltip, in AString direction, in Element browser);`
    #[inline]
    pub unsafe fn ShowTooltip(&self, x: i32, y: i32, tooltip: *const ::nsstring::nsAString, direction: *const ::nsstring::nsAString, browser: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).ShowTooltip)(self, x, y, tooltip, direction, browser)
    }



    /// `void hideTooltip ();`
    #[inline]
    pub unsafe fn HideTooltip(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).HideTooltip)(self, )
    }


    /// ```text
    /// /**
    ///    * Return the number of tabs in this window.
    ///    */
    /// ```
    ///

    /// `uint32_t getTabCount ();`
    #[inline]
    pub unsafe fn GetTabCount(&self, _retval: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetTabCount)(self, _retval)
    }


}


