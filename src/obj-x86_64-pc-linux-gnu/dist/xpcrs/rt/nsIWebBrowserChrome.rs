//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/browser/nsIWebBrowserChrome.idl
//


/// `interface nsIWebBrowserChrome : nsISupports`
///

/// ```text
/// /**
///  * nsIWebBrowserChrome corresponds to the top-level, outermost window
///  * containing an embedded Gecko web browser.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWebBrowserChrome {
    vtable: *const nsIWebBrowserChromeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWebBrowserChrome.
unsafe impl XpCom for nsIWebBrowserChrome {
    const IID: nsIID = nsID(0xe8c414c4, 0xdc38, 0x4ba3,
        [0xab, 0x4e, 0xec, 0x4c, 0xbb, 0xe2, 0x29, 0x07]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWebBrowserChrome {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWebBrowserChrome.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWebBrowserChromeCoerce {
    /// Cheaply cast a value of this type from a `nsIWebBrowserChrome`.
    fn coerce_from(v: &nsIWebBrowserChrome) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWebBrowserChromeCoerce for nsIWebBrowserChrome {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserChrome) -> &Self {
        v
    }
}

impl nsIWebBrowserChrome {
    /// Cast this `nsIWebBrowserChrome` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWebBrowserChromeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWebBrowserChrome {
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
impl<T: nsISupportsCoerce> nsIWebBrowserChromeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserChrome) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWebBrowserChrome
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWebBrowserChromeVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void setLinkStatus (in AString status); */
    pub SetLinkStatus: unsafe extern "system" fn (this: *const nsIWebBrowserChrome, status: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute unsigned long chromeFlags; */
    pub GetChromeFlags: unsafe extern "system" fn (this: *const nsIWebBrowserChrome, aChromeFlags: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long chromeFlags; */
    pub SetChromeFlags: unsafe extern "system" fn (this: *const nsIWebBrowserChrome, aChromeFlags: u32) -> ::nserror::nsresult,

    /* void showAsModal (); */
    pub ShowAsModal: unsafe extern "system" fn (this: *const nsIWebBrowserChrome) -> ::nserror::nsresult,

    /* boolean isWindowModal (); */
    pub IsWindowModal: unsafe extern "system" fn (this: *const nsIWebBrowserChrome, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWebBrowserChrome {
    /// ```text
    /// /**
    ///      * Definitions for the chrome flags
    ///      */
    /// ```
    ///

    pub const CHROME_DEFAULT: i64 = 1;


    pub const CHROME_WINDOW_BORDERS: i64 = 2;


    pub const CHROME_WINDOW_CLOSE: i64 = 4;


    pub const CHROME_WINDOW_RESIZE: i64 = 8;


    pub const CHROME_MENUBAR: i64 = 16;


    pub const CHROME_TOOLBAR: i64 = 32;


    pub const CHROME_LOCATIONBAR: i64 = 64;


    pub const CHROME_STATUSBAR: i64 = 128;


    pub const CHROME_PERSONAL_TOOLBAR: i64 = 256;


    pub const CHROME_SCROLLBARS: i64 = 512;


    pub const CHROME_TITLEBAR: i64 = 1024;


    pub const CHROME_EXTRA: i64 = 2048;


    pub const CHROME_WITH_SIZE: i64 = 4096;


    pub const CHROME_WITH_POSITION: i64 = 8192;


    pub const CHROME_WINDOW_MIN: i64 = 16384;


    pub const CHROME_WINDOW_POPUP: i64 = 32768;


    pub const CHROME_PRIVATE_WINDOW: i64 = 65536;


    pub const CHROME_NON_PRIVATE_WINDOW: i64 = 131072;


    pub const CHROME_PRIVATE_LIFETIME: i64 = 262144;


    pub const CHROME_ALWAYS_ON_TOP: i64 = 524288;


    pub const CHROME_REMOTE_WINDOW: i64 = 1048576;


    pub const CHROME_FISSION_WINDOW: i64 = 2097152;


    pub const CHROME_SUPPRESS_ANIMATION: i64 = 16777216;


    pub const CHROME_WINDOW_RAISED: i64 = 33554432;


    pub const CHROME_WINDOW_LOWERED: i64 = 67108864;


    pub const CHROME_CENTER_SCREEN: i64 = 134217728;


    pub const CHROME_DEPENDENT: i64 = 268435456;


    pub const CHROME_MODAL: i64 = 536870912;


    pub const CHROME_OPENAS_DIALOG: i64 = 1073741824;


    pub const CHROME_OPENAS_CHROME: i64 = 2147483648;


    pub const CHROME_ALL: i64 = 4094;

    /// ```text
    /// /**
    ///      * Called when the link hover status is being changed.
    ///      * @param status status string. empty string is an acceptable value
    ///      *               meaning no link is hovered.
    ///      */
    /// ```
    ///

    /// `void setLinkStatus (in AString status);`
    #[inline]
    pub unsafe fn SetLinkStatus(&self, status: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetLinkStatus)(self, status)
    }


    /// ```text
    /// /**
    ///      * The chrome flags for this browser chrome. The implementation should
    ///      * reflect the value of this attribute by hiding or showing its chrome
    ///      * appropriately.
    ///      */
    /// ```
    ///

    /// `attribute unsigned long chromeFlags;`
    #[inline]
    pub unsafe fn GetChromeFlags(&self, aChromeFlags: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetChromeFlags)(self, aChromeFlags)
    }


    /// ```text
    /// /**
    ///      * The chrome flags for this browser chrome. The implementation should
    ///      * reflect the value of this attribute by hiding or showing its chrome
    ///      * appropriately.
    ///      */
    /// ```
    ///

    /// `attribute unsigned long chromeFlags;`
    #[inline]
    pub unsafe fn SetChromeFlags(&self, aChromeFlags: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetChromeFlags)(self, aChromeFlags)
    }


    /// ```text
    /// /**
    ///      * Shows the window as a modal window.
    ///      */
    /// ```
    ///

    /// `void showAsModal ();`
    #[inline]
    pub unsafe fn ShowAsModal(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ShowAsModal)(self, )
    }


    /// ```text
    /// /**
    ///      * Is the window modal (that is, currently executing a modal loop)?
    ///      * @return true if it's a modal window
    ///      */
    /// ```
    ///

    /// `boolean isWindowModal ();`
    #[inline]
    pub unsafe fn IsWindowModal(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsWindowModal)(self, _retval)
    }


}


