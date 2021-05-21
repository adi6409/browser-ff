//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/mozapps/extensions/amIWebInstallPrompt.idl
//


/// `interface amIWebInstallPrompt : nsISupports`
///

/// ```text
/// /**
///  * amIWebInstallPrompt is used, if available, by the default implementation of
///  * amIWebInstallInfo to display a confirmation UI to the user before running
///  * installs.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct amIWebInstallPrompt {
    vtable: *const amIWebInstallPromptVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for amIWebInstallPrompt.
unsafe impl XpCom for amIWebInstallPrompt {
    const IID: nsIID = nsID(0x386906f1, 0x4d18, 0x45bf,
        [0xbc, 0x81, 0x5d, 0xcd, 0x68, 0xe4, 0x2c, 0x3b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for amIWebInstallPrompt {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from amIWebInstallPrompt.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait amIWebInstallPromptCoerce {
    /// Cheaply cast a value of this type from a `amIWebInstallPrompt`.
    fn coerce_from(v: &amIWebInstallPrompt) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl amIWebInstallPromptCoerce for amIWebInstallPrompt {
    #[inline]
    fn coerce_from(v: &amIWebInstallPrompt) -> &Self {
        v
    }
}

impl amIWebInstallPrompt {
    /// Cast this `amIWebInstallPrompt` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: amIWebInstallPromptCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for amIWebInstallPrompt {
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
impl<T: nsISupportsCoerce> amIWebInstallPromptCoerce for T {
    #[inline]
    fn coerce_from(v: &amIWebInstallPrompt) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every amIWebInstallPrompt
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct amIWebInstallPromptVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void confirm (in Element aBrowser, in nsIURI aUri, in Array<nsIVariant> aInstalls); */
    pub Confirm: unsafe extern "system" fn (this: *const amIWebInstallPrompt, aBrowser: *const libc::c_void, aUri: *const nsIURI, aInstalls: *const thin_vec::ThinVec<RefPtr<nsIVariant>>) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl amIWebInstallPrompt {

    /// ```text
    /// /**
    ///    * Get a confirmation that the user wants to start the installs.
    ///    *
    ///    * @param  aBrowser
    ///    *         The browser that triggered the installs
    ///    * @param  aUri
    ///    *         The URI of the site that triggered the installs
    ///    * @param  aInstalls
    ///    *         The AddonInstalls that were requested
    ///    */
    /// ```
    ///

    /// `void confirm (in Element aBrowser, in nsIURI aUri, in Array<nsIVariant> aInstalls);`
    #[inline]
    pub unsafe fn Confirm(&self, aBrowser: *const libc::c_void, aUri: *const nsIURI, aInstalls: *const thin_vec::ThinVec<RefPtr<nsIVariant>>) -> ::nserror::nsresult {
        ((*self.vtable).Confirm)(self, aBrowser, aUri, aInstalls)
    }


}


