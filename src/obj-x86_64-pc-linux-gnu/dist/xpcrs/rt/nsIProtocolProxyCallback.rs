//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIProtocolProxyCallback.idl
//


/// `interface nsIProtocolProxyCallback : nsISupports`
///

/// ```text
/// /**
///  * This interface serves as a closure for nsIProtocolProxyService's
///  * asyncResolve method.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIProtocolProxyCallback {
    vtable: *const nsIProtocolProxyCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIProtocolProxyCallback.
unsafe impl XpCom for nsIProtocolProxyCallback {
    const IID: nsIID = nsID(0xfbb6eff6, 0x0cc2, 0x4d99,
        [0x8d, 0x6f, 0x0a, 0x12, 0xb4, 0x62, 0xbd, 0xeb]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIProtocolProxyCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIProtocolProxyCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIProtocolProxyCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIProtocolProxyCallback`.
    fn coerce_from(v: &nsIProtocolProxyCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIProtocolProxyCallbackCoerce for nsIProtocolProxyCallback {
    #[inline]
    fn coerce_from(v: &nsIProtocolProxyCallback) -> &Self {
        v
    }
}

impl nsIProtocolProxyCallback {
    /// Cast this `nsIProtocolProxyCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIProtocolProxyCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIProtocolProxyCallback {
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
impl<T: nsISupportsCoerce> nsIProtocolProxyCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProtocolProxyCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIProtocolProxyCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIProtocolProxyCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onProxyAvailable (in nsICancelable aRequest, in nsIChannel aChannel, in nsIProxyInfo aProxyInfo, in nsresult aStatus); */
    pub OnProxyAvailable: unsafe extern "system" fn (this: *const nsIProtocolProxyCallback, aRequest: *const nsICancelable, aChannel: *const nsIChannel, aProxyInfo: *const nsIProxyInfo, aStatus: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIProtocolProxyCallback {

    /// ```text
    /// /**
    ///    * This method is called when proxy info is available or when an error
    ///    * in the proxy resolution occurs.
    ///    *
    ///    * @param aRequest
    ///    *        The value returned from asyncResolve.
    ///    * @param aChannel
    ///    *        The channel passed to asyncResolve.
    ///    * @param aProxyInfo
    ///    *        The resulting proxy info or null if there is no associated proxy
    ///    *        info for aURI.  As with the result of nsIProtocolProxyService's
    ///    *        resolve method, a null result implies that a direct connection
    ///    *        should be used.
    ///    * @param aStatus
    ///    *        The status of the callback.  This is a failure code if the request
    ///    *        could not be satisfied, in which case the value of aStatus
    ///    *        indicates the reason for the failure and aProxyInfo will be null.
    ///    */
    /// ```
    ///

    /// `void onProxyAvailable (in nsICancelable aRequest, in nsIChannel aChannel, in nsIProxyInfo aProxyInfo, in nsresult aStatus);`
    #[inline]
    pub unsafe fn OnProxyAvailable(&self, aRequest: *const nsICancelable, aChannel: *const nsIChannel, aProxyInfo: *const nsIProxyInfo, aStatus: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).OnProxyAvailable)(self, aRequest, aChannel, aProxyInfo, aStatus)
    }


}


