//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/browser-element/nsIBrowserElementAPI.idl
//


/// `interface nsIBrowserElementAPI : nsISupports`
///

/// ```text
/// /**
///  * Interface to the BrowserElementParent implementation. All methods
///  * but setFrameLoader throw when the remote process is dead.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIBrowserElementAPI {
    vtable: *const nsIBrowserElementAPIVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIBrowserElementAPI.
unsafe impl XpCom for nsIBrowserElementAPI {
    const IID: nsIID = nsID(0x57758c10, 0x6036, 0x11e5,
        [0xa8, 0x37, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIBrowserElementAPI {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIBrowserElementAPI.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIBrowserElementAPICoerce {
    /// Cheaply cast a value of this type from a `nsIBrowserElementAPI`.
    fn coerce_from(v: &nsIBrowserElementAPI) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIBrowserElementAPICoerce for nsIBrowserElementAPI {
    #[inline]
    fn coerce_from(v: &nsIBrowserElementAPI) -> &Self {
        v
    }
}

impl nsIBrowserElementAPI {
    /// Cast this `nsIBrowserElementAPI` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIBrowserElementAPICoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIBrowserElementAPI {
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
impl<T: nsISupportsCoerce> nsIBrowserElementAPICoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBrowserElementAPI) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIBrowserElementAPI
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIBrowserElementAPIVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void destroyFrameScripts (); */
    pub DestroyFrameScripts: unsafe extern "system" fn (this: *const nsIBrowserElementAPI) -> ::nserror::nsresult,

    /* void setFrameLoader (in FrameLoader frameLoader); */
    pub SetFrameLoader: unsafe extern "system" fn (this: *const nsIBrowserElementAPI, frameLoader: *const libc::c_void) -> ::nserror::nsresult,

    /* void sendMouseEvent (in AString type, in uint32_t x, in uint32_t y, in uint32_t button, in uint32_t clickCount, in uint32_t mifiers); */
    pub SendMouseEvent: unsafe extern "system" fn (this: *const nsIBrowserElementAPI, type_: *const ::nsstring::nsAString, x: uint32_t, y: uint32_t, button: uint32_t, clickCount: uint32_t, mifiers: uint32_t) -> ::nserror::nsresult,

    /* void goBack (); */
    pub GoBack: unsafe extern "system" fn (this: *const nsIBrowserElementAPI) -> ::nserror::nsresult,

    /* void goForward (); */
    pub GoForward: unsafe extern "system" fn (this: *const nsIBrowserElementAPI) -> ::nserror::nsresult,

    /* void reload (in boolean hardReload); */
    pub Reload: unsafe extern "system" fn (this: *const nsIBrowserElementAPI, hardReload: bool) -> ::nserror::nsresult,

    /* void stop (); */
    pub Stop: unsafe extern "system" fn (this: *const nsIBrowserElementAPI) -> ::nserror::nsresult,

    /* Promise getCanGoBack (); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub GetCanGoBack: *const ::libc::c_void,

    /* Promise getCanGoForward (); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub GetCanGoForward: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIBrowserElementAPI {

    /// ```text
    /// /**
    ///    * Notify frame scripts that support the API to destroy.
    ///    */
    /// ```
    ///

    /// `void destroyFrameScripts ();`
    #[inline]
    pub unsafe fn DestroyFrameScripts(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).DestroyFrameScripts)(self, )
    }



    /// `void setFrameLoader (in FrameLoader frameLoader);`
    #[inline]
    pub unsafe fn SetFrameLoader(&self, frameLoader: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).SetFrameLoader)(self, frameLoader)
    }



    /// `void sendMouseEvent (in AString type, in uint32_t x, in uint32_t y, in uint32_t button, in uint32_t clickCount, in uint32_t mifiers);`
    #[inline]
    pub unsafe fn SendMouseEvent(&self, type_: *const ::nsstring::nsAString, x: uint32_t, y: uint32_t, button: uint32_t, clickCount: uint32_t, mifiers: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).SendMouseEvent)(self, type_, x, y, button, clickCount, mifiers)
    }



    /// `void goBack ();`
    #[inline]
    pub unsafe fn GoBack(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).GoBack)(self, )
    }



    /// `void goForward ();`
    #[inline]
    pub unsafe fn GoForward(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).GoForward)(self, )
    }



    /// `void reload (in boolean hardReload);`
    #[inline]
    pub unsafe fn Reload(&self, hardReload: bool) -> ::nserror::nsresult {
        ((*self.vtable).Reload)(self, hardReload)
    }



    /// `void stop ();`
    #[inline]
    pub unsafe fn Stop(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Stop)(self, )
    }



    /// `Promise getCanGoBack ();`
    const _GetCanGoBack: () = ();


    /// `Promise getCanGoForward ();`
    const _GetCanGoForward: () = ();

}


