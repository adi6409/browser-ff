//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIHttpAuthenticatorCallback.idl
//


/// `interface nsIHttpAuthenticatorCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHttpAuthenticatorCallback {
    vtable: *const nsIHttpAuthenticatorCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHttpAuthenticatorCallback.
unsafe impl XpCom for nsIHttpAuthenticatorCallback {
    const IID: nsIID = nsID(0xd989cb03, 0xe446, 0x4086,
        [0xb9, 0xe6, 0x46, 0x84, 0x2c, 0xb9, 0x7b, 0xd5]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHttpAuthenticatorCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHttpAuthenticatorCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHttpAuthenticatorCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIHttpAuthenticatorCallback`.
    fn coerce_from(v: &nsIHttpAuthenticatorCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHttpAuthenticatorCallbackCoerce for nsIHttpAuthenticatorCallback {
    #[inline]
    fn coerce_from(v: &nsIHttpAuthenticatorCallback) -> &Self {
        v
    }
}

impl nsIHttpAuthenticatorCallback {
    /// Cast this `nsIHttpAuthenticatorCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHttpAuthenticatorCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHttpAuthenticatorCallback {
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
impl<T: nsISupportsCoerce> nsIHttpAuthenticatorCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpAuthenticatorCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHttpAuthenticatorCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHttpAuthenticatorCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onCredsGenerated (in string aCreds, in unsigned long aFlags, in nsresult aResult, in nsISupports aSessionsState, in nsISupports aContinuationState); */
    pub OnCredsGenerated: unsafe extern "system" fn (this: *const nsIHttpAuthenticatorCallback, aCreds: *const libc::c_char, aFlags: u32, aResult: ::nserror::nsresult, aSessionsState: *const nsISupports, aContinuationState: *const nsISupports) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHttpAuthenticatorCallback {

    /// ```text
    /// /**
    ///    * Authentication data for a header is available.
    ///    *
    ///    * @param aCreds
    ///    *        Credentials which were obtained asynchonously.
    ///    * @param aFlags
    ///    *        Flags set by asynchronous call.
    ///    * @param aResult
    ///    *        Result status of credentials generation
    ///    * @param aSessionState
    ///    *        Modified session state to be passed to caller
    ///    * @param aContinuationState
    ///    *        Modified continuation state to be passed to caller
    ///    */
    /// ```
    ///

    /// `void onCredsGenerated (in string aCreds, in unsigned long aFlags, in nsresult aResult, in nsISupports aSessionsState, in nsISupports aContinuationState);`
    #[inline]
    pub unsafe fn OnCredsGenerated(&self, aCreds: *const libc::c_char, aFlags: u32, aResult: ::nserror::nsresult, aSessionsState: *const nsISupports, aContinuationState: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).OnCredsGenerated)(self, aCreds, aFlags, aResult, aSessionsState, aContinuationState)
    }


}


