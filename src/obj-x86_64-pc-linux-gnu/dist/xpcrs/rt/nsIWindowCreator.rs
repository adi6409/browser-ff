//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/windowcreator/nsIWindowCreator.idl
//


/// `interface nsIWindowCreator : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWindowCreator {
    vtable: *const nsIWindowCreatorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWindowCreator.
unsafe impl XpCom for nsIWindowCreator {
    const IID: nsIID = nsID(0x30465632, 0xa777, 0x44cc,
        [0x90, 0xf9, 0x81, 0x45, 0x47, 0x5e, 0xf9, 0x99]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWindowCreator {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWindowCreator.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWindowCreatorCoerce {
    /// Cheaply cast a value of this type from a `nsIWindowCreator`.
    fn coerce_from(v: &nsIWindowCreator) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWindowCreatorCoerce for nsIWindowCreator {
    #[inline]
    fn coerce_from(v: &nsIWindowCreator) -> &Self {
        v
    }
}

impl nsIWindowCreator {
    /// Cast this `nsIWindowCreator` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWindowCreatorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWindowCreator {
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
impl<T: nsISupportsCoerce> nsIWindowCreatorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWindowCreator) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWindowCreator
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWindowCreatorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIWebBrowserChrome createChromeWindow (in nsIWebBrowserChrome parent, in uint32_t chromeFlags, in nsIOpenWindowInfo aOpenWindowInfo, out boolean cancel); */
    pub CreateChromeWindow: unsafe extern "system" fn (this: *const nsIWindowCreator, parent: *const nsIWebBrowserChrome, chromeFlags: uint32_t, aOpenWindowInfo: *const nsIOpenWindowInfo, cancel: *mut bool, _retval: *mut*const nsIWebBrowserChrome) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWindowCreator {

    /// ```text
    /// /** Create a new window. Gecko will/may call this method, if made
    ///       available to it, to create new windows.
    ///       @param parent Parent window, if any. Null if not. The newly created
    ///                     window should be made a child/dependent window of
    ///                     the parent, if any (and if the concept applies
        ///                     to the underlying OS).
    ///       @param chromeFlags Chrome features from nsIWebBrowserChrome
    ///       @param aOpenWindowInfo Information used to open initial content in
    ///                              the new chrome window. Can be nullptr.
    ///       @param cancel Return |true| to reject window creation. If true the
    ///                     implementation has determined the window should not
    ///                     be created at all. The caller should not default
    ///                     to any possible backup scheme for creating the window.
    ///       @return the new window. Will be null if canceled or an error occurred.
    ///   */
    /// ```
    ///

    /// `nsIWebBrowserChrome createChromeWindow (in nsIWebBrowserChrome parent, in uint32_t chromeFlags, in nsIOpenWindowInfo aOpenWindowInfo, out boolean cancel);`
    #[inline]
    pub unsafe fn CreateChromeWindow(&self, parent: *const nsIWebBrowserChrome, chromeFlags: uint32_t, aOpenWindowInfo: *const nsIOpenWindowInfo, cancel: *mut bool, _retval: *mut*const nsIWebBrowserChrome) -> ::nserror::nsresult {
        ((*self.vtable).CreateChromeWindow)(self, parent, chromeFlags, aOpenWindowInfo, cancel, _retval)
    }


}


