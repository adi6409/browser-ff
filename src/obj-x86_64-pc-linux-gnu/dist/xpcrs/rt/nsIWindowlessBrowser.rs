//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpfe/appshell/nsIWindowlessBrowser.idl
//


/// `interface nsIWindowlessBrowser : nsIWebNavigation`
///

/// ```text
/// /**
///  * This interface represents a nsIWebBrowser instance with no associated OS
///  * window. Its main function is to manage the lifetimes of those windows.
///  * A strong reference to this object must be held until the window is
///  * ready to be destroyed.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWindowlessBrowser {
    vtable: *const nsIWindowlessBrowserVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWindowlessBrowser.
unsafe impl XpCom for nsIWindowlessBrowser {
    const IID: nsIID = nsID(0xabb46f48, 0xabfc, 0x41bf,
        [0xaa, 0x9a, 0x7f, 0xec, 0xce, 0xfc, 0xf9, 0x77]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWindowlessBrowser {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWindowlessBrowser.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWindowlessBrowserCoerce {
    /// Cheaply cast a value of this type from a `nsIWindowlessBrowser`.
    fn coerce_from(v: &nsIWindowlessBrowser) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWindowlessBrowserCoerce for nsIWindowlessBrowser {
    #[inline]
    fn coerce_from(v: &nsIWindowlessBrowser) -> &Self {
        v
    }
}

impl nsIWindowlessBrowser {
    /// Cast this `nsIWindowlessBrowser` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWindowlessBrowserCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWindowlessBrowser {
    type Target = nsIWebNavigation;
    #[inline]
    fn deref(&self) -> &nsIWebNavigation {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIWebNavigationCoerce> nsIWindowlessBrowserCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWindowlessBrowser) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWindowlessBrowser
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWindowlessBrowserVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIWebNavigationVTable,

    /* void close (); */
    pub Close: unsafe extern "system" fn (this: *const nsIWindowlessBrowser) -> ::nserror::nsresult,

    /* readonly attribute nsIDocShell docShell; */
    pub GetDocShell: unsafe extern "system" fn (this: *const nsIWindowlessBrowser, aDocShell: *mut*const nsIDocShell) -> ::nserror::nsresult,

    /* readonly attribute BrowsingContext browsingContext; */
    pub GetBrowsingContext: unsafe extern "system" fn (this: *const nsIWindowlessBrowser, aBrowsingContext: *mut *const libc::c_void) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWindowlessBrowser {

    /// ```text
    /// /**
    ///    * "Closes" the windowless browser and destroys its associated nsIWebBrowser
    ///    * and docshell.
    ///    *
    ///    * This method *must* be called for every windowless browser before its last
    ///    * reference is released.
    ///    */
    /// ```
    ///

    /// `void close ();`
    #[inline]
    pub unsafe fn Close(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Close)(self, )
    }


    /// ```text
    /// /**
    ///    * Get the docshell for this browser.  This is the docshell that gets
    ///    * navigated when the browser's nsIWebNavigation interface is used.
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
    ///    * Get the Browsing Context for this browser.  This is the Browsing Context
    ///    * that owns the docshell used for navigation.
    ///    */
    /// ```
    ///

    /// `readonly attribute BrowsingContext browsingContext;`
    #[inline]
    pub unsafe fn GetBrowsingContext(&self, aBrowsingContext: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetBrowsingContext)(self, aBrowsingContext)
    }


}


