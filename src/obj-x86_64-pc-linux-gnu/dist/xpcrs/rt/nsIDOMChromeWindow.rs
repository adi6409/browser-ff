//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIDOMChromeWindow.idl
//


/// `interface nsIDOMChromeWindow : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDOMChromeWindow {
    vtable: *const nsIDOMChromeWindowVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDOMChromeWindow.
unsafe impl XpCom for nsIDOMChromeWindow {
    const IID: nsIID = nsID(0x78bdcb41, 0x1efa, 0x409f,
        [0xaa, 0xba, 0x70, 0x84, 0x22, 0x13, 0xf8, 0x0f]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDOMChromeWindow {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDOMChromeWindow.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDOMChromeWindowCoerce {
    /// Cheaply cast a value of this type from a `nsIDOMChromeWindow`.
    fn coerce_from(v: &nsIDOMChromeWindow) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDOMChromeWindowCoerce for nsIDOMChromeWindow {
    #[inline]
    fn coerce_from(v: &nsIDOMChromeWindow) -> &Self {
        v
    }
}

impl nsIDOMChromeWindow {
    /// Cast this `nsIDOMChromeWindow` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDOMChromeWindowCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDOMChromeWindow {
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
impl<T: nsISupportsCoerce> nsIDOMChromeWindowCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMChromeWindow) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDOMChromeWindow
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDOMChromeWindowVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [noscript] readonly attribute nsIBrowserDOMWindow browserDOMWindow; */
    pub GetBrowserDOMWindow: unsafe extern "system" fn (this: *const nsIDOMChromeWindow, aBrowserDOMWindow: *mut*const nsIBrowserDOMWindow) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDOMChromeWindow {

    /// ```text
    /// /**
    ///    * browserDOMWindow provides access to yet another layer of
    ///    * utility functions implemented by chrome script. It will be null
    ///    * for DOMWindows not corresponding to browsers.
    ///    */
    /// ```
    ///

    /// `[noscript] readonly attribute nsIBrowserDOMWindow browserDOMWindow;`
    #[inline]
    pub unsafe fn GetBrowserDOMWindow(&self, aBrowserDOMWindow: *mut*const nsIBrowserDOMWindow) -> ::nserror::nsresult {
        ((*self.vtable).GetBrowserDOMWindow)(self, aBrowserDOMWindow)
    }


}


