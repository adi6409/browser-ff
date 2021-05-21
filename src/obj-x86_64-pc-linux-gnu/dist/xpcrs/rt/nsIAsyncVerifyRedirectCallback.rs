//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIAsyncVerifyRedirectCallback.idl
//


/// `interface nsIAsyncVerifyRedirectCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAsyncVerifyRedirectCallback {
    vtable: *const nsIAsyncVerifyRedirectCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAsyncVerifyRedirectCallback.
unsafe impl XpCom for nsIAsyncVerifyRedirectCallback {
    const IID: nsIID = nsID(0x8d171460, 0xa716, 0x41f1,
        [0x92, 0xbe, 0x8c, 0x65, 0x9d, 0xb3, 0x9b, 0x45]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAsyncVerifyRedirectCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAsyncVerifyRedirectCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAsyncVerifyRedirectCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIAsyncVerifyRedirectCallback`.
    fn coerce_from(v: &nsIAsyncVerifyRedirectCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAsyncVerifyRedirectCallbackCoerce for nsIAsyncVerifyRedirectCallback {
    #[inline]
    fn coerce_from(v: &nsIAsyncVerifyRedirectCallback) -> &Self {
        v
    }
}

impl nsIAsyncVerifyRedirectCallback {
    /// Cast this `nsIAsyncVerifyRedirectCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAsyncVerifyRedirectCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAsyncVerifyRedirectCallback {
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
impl<T: nsISupportsCoerce> nsIAsyncVerifyRedirectCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAsyncVerifyRedirectCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAsyncVerifyRedirectCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAsyncVerifyRedirectCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onRedirectVerifyCallback (in nsresult result); */
    pub OnRedirectVerifyCallback: unsafe extern "system" fn (this: *const nsIAsyncVerifyRedirectCallback, result: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAsyncVerifyRedirectCallback {

    /// ```text
    /// /**
    ///      * Complement to nsIChannelEventSink asynchronous callback. The result of
    ///      * the redirect decision is passed through this callback.
    ///      *
    ///      * @param result
    ///      *    Result of the redirect veto decision. If FAILED the redirect has been
    ///      *    vetoed. If SUCCEEDED the redirect has been allowed by all consumers.
    ///      */
    /// ```
    ///

    /// `void onRedirectVerifyCallback (in nsresult result);`
    #[inline]
    pub unsafe fn OnRedirectVerifyCallback(&self, result: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).OnRedirectVerifyCallback)(self, result)
    }


}


