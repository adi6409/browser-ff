//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIParentRedirectingChannel.idl
//


/// `interface nsIAsyncVerifyRedirectReadyCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAsyncVerifyRedirectReadyCallback {
    vtable: *const nsIAsyncVerifyRedirectReadyCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAsyncVerifyRedirectReadyCallback.
unsafe impl XpCom for nsIAsyncVerifyRedirectReadyCallback {
    const IID: nsIID = nsID(0x01987690, 0x48cf, 0x45de,
        [0xba, 0xe3, 0xe1, 0x43, 0xc2, 0xad, 0xc2, 0xa8]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAsyncVerifyRedirectReadyCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAsyncVerifyRedirectReadyCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAsyncVerifyRedirectReadyCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIAsyncVerifyRedirectReadyCallback`.
    fn coerce_from(v: &nsIAsyncVerifyRedirectReadyCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAsyncVerifyRedirectReadyCallbackCoerce for nsIAsyncVerifyRedirectReadyCallback {
    #[inline]
    fn coerce_from(v: &nsIAsyncVerifyRedirectReadyCallback) -> &Self {
        v
    }
}

impl nsIAsyncVerifyRedirectReadyCallback {
    /// Cast this `nsIAsyncVerifyRedirectReadyCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAsyncVerifyRedirectReadyCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAsyncVerifyRedirectReadyCallback {
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
impl<T: nsISupportsCoerce> nsIAsyncVerifyRedirectReadyCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAsyncVerifyRedirectReadyCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAsyncVerifyRedirectReadyCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAsyncVerifyRedirectReadyCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void readyToVerify (in nsresult result); */
    pub ReadyToVerify: unsafe extern "system" fn (this: *const nsIAsyncVerifyRedirectReadyCallback, result: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAsyncVerifyRedirectReadyCallback {

    /// ```text
    /// /**
    ///    * Asynchronous callback when redirected channel finishes the preparation for
    ///    * completing the verification procedure.
    ///    *
    ///    * @param result
    ///    *    SUCCEEDED if preparation for redirection verification succceed.
    ///    *    If FAILED the redirection must be aborted.
    ///    */
    /// ```
    ///

    /// `void readyToVerify (in nsresult result);`
    #[inline]
    pub unsafe fn ReadyToVerify(&self, result: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).ReadyToVerify)(self, result)
    }


}


/// `interface nsIParentRedirectingChannel : nsIParentChannel`
///

/// ```text
/// /**
///  * Implemented by chrome side of IPC protocols that support redirect responses.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIParentRedirectingChannel {
    vtable: *const nsIParentRedirectingChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIParentRedirectingChannel.
unsafe impl XpCom for nsIParentRedirectingChannel {
    const IID: nsIID = nsID(0x3ed1d288, 0x5324, 0x46ee,
        [0x8a, 0x98, 0x33, 0xac, 0x37, 0xd1, 0x08, 0x0b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIParentRedirectingChannel {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIParentRedirectingChannel.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIParentRedirectingChannelCoerce {
    /// Cheaply cast a value of this type from a `nsIParentRedirectingChannel`.
    fn coerce_from(v: &nsIParentRedirectingChannel) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIParentRedirectingChannelCoerce for nsIParentRedirectingChannel {
    #[inline]
    fn coerce_from(v: &nsIParentRedirectingChannel) -> &Self {
        v
    }
}

impl nsIParentRedirectingChannel {
    /// Cast this `nsIParentRedirectingChannel` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIParentRedirectingChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIParentRedirectingChannel {
    type Target = nsIParentChannel;
    #[inline]
    fn deref(&self) -> &nsIParentChannel {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIParentChannelCoerce> nsIParentRedirectingChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIParentRedirectingChannel) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIParentRedirectingChannel
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIParentRedirectingChannelVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIParentChannelVTable,

    /* void startRedirect (in nsIChannel newChannel, in uint32_t redirectFlags, in nsIAsyncVerifyRedirectCallback callback); */
    pub StartRedirect: unsafe extern "system" fn (this: *const nsIParentRedirectingChannel, newChannel: *const nsIChannel, redirectFlags: uint32_t, callback: *const nsIAsyncVerifyRedirectCallback) -> ::nserror::nsresult,

    /* void continueVerification (in nsIAsyncVerifyRedirectReadyCallback callback); */
    pub ContinueVerification: unsafe extern "system" fn (this: *const nsIParentRedirectingChannel, callback: *const nsIAsyncVerifyRedirectReadyCallback) -> ::nserror::nsresult,

    /* void completeRedirect (in boolean succeeded); */
    pub CompleteRedirect: unsafe extern "system" fn (this: *const nsIParentRedirectingChannel, succeeded: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIParentRedirectingChannel {

    /// ```text
    /// /**
    ///    * Called when the channel got a response that redirects it to a different
    ///    * URI.  The implementation is responsible for calling the redirect observers
    ///    * on the child process and provide the decision result to the callback.
    ///    *
    ///    * @param newURI
    ///    *    the URI we redirect to
    ///    * @param callback
    ///    *    redirect result callback, usage is compatible with how
    ///    *    nsIChannelEventSink defines it
    ///    */
    /// ```
    ///

    /// `void startRedirect (in nsIChannel newChannel, in uint32_t redirectFlags, in nsIAsyncVerifyRedirectCallback callback);`
    #[inline]
    pub unsafe fn StartRedirect(&self, newChannel: *const nsIChannel, redirectFlags: uint32_t, callback: *const nsIAsyncVerifyRedirectCallback) -> ::nserror::nsresult {
        ((*self.vtable).StartRedirect)(self, newChannel, redirectFlags, callback)
    }


    /// ```text
    /// /**
    ///    * Called to new channel when the original channel got Redirect2Verify
    ///    * response from child. Callback will be invoked when the new channel
    ///    * finishes the preparation for Redirect2Verify and can be called immediately.
    ///    *
    ///    * @param callback
    ///    *    redirect ready callback, will be called when redirect verification
    ///    *    procedure can proceed.
    ///    */
    /// ```
    ///

    /// `void continueVerification (in nsIAsyncVerifyRedirectReadyCallback callback);`
    #[inline]
    pub unsafe fn ContinueVerification(&self, callback: *const nsIAsyncVerifyRedirectReadyCallback) -> ::nserror::nsresult {
        ((*self.vtable).ContinueVerification)(self, callback)
    }


    /// ```text
    /// /**
    ///    * Called after we are done with redirecting process and we know if to
    ///    * redirect or not.  Forward the redirect result to the child process.  From
    ///    * that moment the nsIParentChannel implementation expects it will be
    ///    * forwarded all notifications from the 'real' channel.
    ///    *
    ///    * Primarilly used by ParentChannelListener::OnRedirectResult and kept as
    ///    * mActiveChannel and mRedirectChannel in that class.
    ///    */
    /// ```
    ///

    /// `void completeRedirect (in boolean succeeded);`
    #[inline]
    pub unsafe fn CompleteRedirect(&self, succeeded: bool) -> ::nserror::nsresult {
        ((*self.vtable).CompleteRedirect)(self, succeeded)
    }


}


