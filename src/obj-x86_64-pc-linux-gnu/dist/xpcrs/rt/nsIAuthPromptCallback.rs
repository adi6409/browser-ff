//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIAuthPromptCallback.idl
//


/// `interface nsIAuthPromptCallback : nsISupports`
///

/// ```text
/// /**
///  * Interface for callback methods for the asynchronous nsIAuthPrompt2 method.
///  * Callers MUST call exactly one method if nsIAuthPrompt2::promptPasswordAsync
///  * returns successfully. They MUST NOT call any method on this interface before
///  * promptPasswordAsync returns.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAuthPromptCallback {
    vtable: *const nsIAuthPromptCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAuthPromptCallback.
unsafe impl XpCom for nsIAuthPromptCallback {
    const IID: nsIID = nsID(0xbdc387d7, 0x2d29, 0x4cac,
        [0x92, 0xf1, 0xdd, 0x75, 0xd7, 0x86, 0x63, 0x1d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAuthPromptCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAuthPromptCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAuthPromptCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIAuthPromptCallback`.
    fn coerce_from(v: &nsIAuthPromptCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAuthPromptCallbackCoerce for nsIAuthPromptCallback {
    #[inline]
    fn coerce_from(v: &nsIAuthPromptCallback) -> &Self {
        v
    }
}

impl nsIAuthPromptCallback {
    /// Cast this `nsIAuthPromptCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAuthPromptCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAuthPromptCallback {
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
impl<T: nsISupportsCoerce> nsIAuthPromptCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAuthPromptCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAuthPromptCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAuthPromptCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onAuthAvailable (in nsISupports aContext, in nsIAuthInformation aAuthInfo); */
    pub OnAuthAvailable: unsafe extern "system" fn (this: *const nsIAuthPromptCallback, aContext: *const nsISupports, aAuthInfo: *const nsIAuthInformation) -> ::nserror::nsresult,

    /* void onAuthCancelled (in nsISupports aContext, in boolean userCancel); */
    pub OnAuthCancelled: unsafe extern "system" fn (this: *const nsIAuthPromptCallback, aContext: *const nsISupports, userCancel: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAuthPromptCallback {

    /// ```text
    /// /**
    ///    * Authentication information is available.
    ///    *
    ///    * @param aContext
    ///    *        The context as passed to promptPasswordAsync
    ///    * @param aAuthInfo
    ///    *        Authentication information. Must be the same object that was passed
    ///    *        to promptPasswordAsync.
    ///    *
    ///    * @note  Any exceptions thrown from this method should be ignored.
    ///    */
    /// ```
    ///

    /// `void onAuthAvailable (in nsISupports aContext, in nsIAuthInformation aAuthInfo);`
    #[inline]
    pub unsafe fn OnAuthAvailable(&self, aContext: *const nsISupports, aAuthInfo: *const nsIAuthInformation) -> ::nserror::nsresult {
        ((*self.vtable).OnAuthAvailable)(self, aContext, aAuthInfo)
    }


    /// ```text
    /// /**
    ///    * Notification that the prompt was cancelled.
    ///    *
    ///    * @param aContext
    ///    *        The context that was passed to promptPasswordAsync.
    ///    * @param userCancel
    ///    *        If false, this prompt was cancelled by calling the
    ///    *        the cancel method on the nsICancelable; otherwise,
    ///    *        it was cancelled by the user.
    ///    */
    /// ```
    ///

    /// `void onAuthCancelled (in nsISupports aContext, in boolean userCancel);`
    #[inline]
    pub unsafe fn OnAuthCancelled(&self, aContext: *const nsISupports, userCancel: bool) -> ::nserror::nsresult {
        ((*self.vtable).OnAuthCancelled)(self, aContext, userCancel)
    }


}


