//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsILoadURIDelegate.idl
//


/// `interface nsILoadURIDelegate : nsISupports`
///

/// ```text
/// /**
///  * The nsILoadURIDelegate interface.
///  * Used for delegating URI loads to GeckoView's application, e.g., Custom Tabs
///  * or Progressive Web Apps.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsILoadURIDelegate {
    vtable: *const nsILoadURIDelegateVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsILoadURIDelegate.
unsafe impl XpCom for nsILoadURIDelegate {
    const IID: nsIID = nsID(0x78e42d37, 0xa34c, 0x4d96,
        [0xb9, 0x01, 0x25, 0x38, 0x56, 0x69, 0xab, 0xa4]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsILoadURIDelegate {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsILoadURIDelegate.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsILoadURIDelegateCoerce {
    /// Cheaply cast a value of this type from a `nsILoadURIDelegate`.
    fn coerce_from(v: &nsILoadURIDelegate) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsILoadURIDelegateCoerce for nsILoadURIDelegate {
    #[inline]
    fn coerce_from(v: &nsILoadURIDelegate) -> &Self {
        v
    }
}

impl nsILoadURIDelegate {
    /// Cast this `nsILoadURIDelegate` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsILoadURIDelegateCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsILoadURIDelegate {
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
impl<T: nsISupportsCoerce> nsILoadURIDelegateCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILoadURIDelegate) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsILoadURIDelegate
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsILoadURIDelegateVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* boolean loadURI (in nsIURI aURI, in short aWhere, in long aFlags, in nsIPrincipal aTriggeringPrincipal); */
    pub LoadURI: unsafe extern "system" fn (this: *const nsILoadURIDelegate, aURI: *const nsIURI, aWhere: i16, aFlags: i32, aTriggeringPrincipal: *const nsIPrincipal, _retval: *mut bool) -> ::nserror::nsresult,

    /* nsIURI handleLoadError (in nsIURI aURI, in nsresult aError, in short aErrorModule); */
    pub HandleLoadError: unsafe extern "system" fn (this: *const nsILoadURIDelegate, aURI: *const nsIURI, aError: ::nserror::nsresult, aErrorModule: i16, _retval: *mut*const nsIURI) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsILoadURIDelegate {

    /// ```text
    /// /**
    ///    * Delegates the URI load. This should only be called for top-level loads.
    ///    *
    ///    * @param aURI The URI to load.
    ///    * @param aWhere See possible values described in nsIBrowserDOMWindow.
    ///    * @param aFlags Flags which control the behavior of the load.
    ///    * @param aTriggeringPrincipal The principal that triggered the load of aURI.
    ///    *
    ///    * Returns whether the load has been successfully handled.
    ///   */
    /// ```
    ///

    /// `boolean loadURI (in nsIURI aURI, in short aWhere, in long aFlags, in nsIPrincipal aTriggeringPrincipal);`
    #[inline]
    pub unsafe fn LoadURI(&self, aURI: *const nsIURI, aWhere: i16, aFlags: i32, aTriggeringPrincipal: *const nsIPrincipal, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).LoadURI)(self, aURI, aWhere, aFlags, aTriggeringPrincipal, _retval)
    }


    /// ```text
    /// /**
    ///    * Delegates page load error handling. This may be called for either top-level
    ///    * loads or subframes.
    ///    *
    ///    * @param aURI The URI that failed to load.
    ///    * @param aError The error code.
    ///    * @param aErrorModule The error module code.
    ///
    ///    * Returns an error page URL to load, or null to show the default error page.
    ///    * No error page is shown at all if an error is thrown.
    ///    */
    /// ```
    ///

    /// `nsIURI handleLoadError (in nsIURI aURI, in nsresult aError, in short aErrorModule);`
    #[inline]
    pub unsafe fn HandleLoadError(&self, aURI: *const nsIURI, aError: ::nserror::nsresult, aErrorModule: i16, _retval: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).HandleLoadError)(self, aURI, aError, aErrorModule, _retval)
    }


}


