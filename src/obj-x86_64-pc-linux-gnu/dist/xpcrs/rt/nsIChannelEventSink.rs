//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIChannelEventSink.idl
//


/// `interface nsIChannelEventSink : nsISupports`
///

/// ```text
/// /**
///  * Implement this interface to receive control over various channel events.
///  * Channels will try to get this interface from a channel's
///  * notificationCallbacks or, if not available there, from the loadGroup's
///  * notificationCallbacks.
///  *
///  * These methods are called before onStartRequest.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIChannelEventSink {
    vtable: *const nsIChannelEventSinkVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIChannelEventSink.
unsafe impl XpCom for nsIChannelEventSink {
    const IID: nsIID = nsID(0x0197720d, 0x37ed, 0x4e75,
        [0x89, 0x56, 0xd0, 0xd2, 0x96, 0xe4, 0xd8, 0xa6]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIChannelEventSink {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIChannelEventSink.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIChannelEventSinkCoerce {
    /// Cheaply cast a value of this type from a `nsIChannelEventSink`.
    fn coerce_from(v: &nsIChannelEventSink) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIChannelEventSinkCoerce for nsIChannelEventSink {
    #[inline]
    fn coerce_from(v: &nsIChannelEventSink) -> &Self {
        v
    }
}

impl nsIChannelEventSink {
    /// Cast this `nsIChannelEventSink` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIChannelEventSinkCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIChannelEventSink {
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
impl<T: nsISupportsCoerce> nsIChannelEventSinkCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIChannelEventSink) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIChannelEventSink
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIChannelEventSinkVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void asyncOnChannelRedirect (in nsIChannel oldChannel, in nsIChannel newChannel, in unsigned long flags, in nsIAsyncVerifyRedirectCallback callback); */
    pub AsyncOnChannelRedirect: unsafe extern "system" fn (this: *const nsIChannelEventSink, oldChannel: *const nsIChannel, newChannel: *const nsIChannel, flags: u32, callback: *const nsIAsyncVerifyRedirectCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIChannelEventSink {
    /// ```text
    /// /**
    ///      * This is a temporary redirect. New requests for this resource should
    ///      * continue to use the URI of the old channel.
    ///      *
    ///      * The new URI may be identical to the old one.
    ///      */
    /// ```
    ///

    pub const REDIRECT_TEMPORARY: i64 = 1;

    /// ```text
    /// /**
    ///      * This is a permanent redirect. New requests for this resource should use
    ///      * the URI of the new channel (This might be an HTTP 301 reponse).
    ///      * If this flag is not set, this is a temporary redirect.
    ///      *
    ///      * The new URI may be identical to the old one.
    ///      */
    /// ```
    ///

    pub const REDIRECT_PERMANENT: i64 = 2;

    /// ```text
    /// /**
    ///      * This is an internal redirect, i.e. it was not initiated by the remote
    ///      * server, but is specific to the channel implementation.
    ///      *
    ///      * The new URI may be identical to the old one.
    ///      */
    /// ```
    ///

    pub const REDIRECT_INTERNAL: i64 = 4;

    /// ```text
    /// /**
    ///      * This is a special-cased redirect coming from hitting HSTS upgrade
    ///      * redirect from http to https only.  In some cases this type of redirect
    ///      * may be considered as safe despite not being the-same-origin redirect.
    ///      */
    /// ```
    ///

    pub const REDIRECT_STS_UPGRADE: i64 = 8;

    /// ```text
    /// /**
    ///      * Called when a redirect occurs. This may happen due to an HTTP 3xx status
    ///      * code. The purpose of this method is to notify the sink that a redirect
    ///      * is about to happen, but also to give the sink the right to veto the
    ///      * redirect by throwing or passing a failure-code in the callback.
    ///      *
    ///      * Note that vetoing the redirect simply means that |newChannel| will not
    ///      * be opened. It is important to understand that |oldChannel| will continue
    ///      * loading as if it received a HTTP 200, which includes notifying observers
    ///      * and possibly display or process content attached to the HTTP response.
    ///      * If the sink wants to prevent this loading it must explicitly deal with
    ///      * it, e.g. by calling |oldChannel->Cancel()|
    ///      *
    ///      * There is a certain freedom in implementing this method:
    ///      *
    ///      * If the return-value indicates success, a callback on |callback| is
    ///      * required. This callback can be done from within asyncOnChannelRedirect
    ///      * (effectively making the call synchronous) or at some point later
    ///      * (making the call asynchronous). Repeat: A callback must be done
    ///      * if this method returns successfully.
    ///      *
    ///      * If the return value indicates error (method throws an exception)
    ///      * the redirect is vetoed and no callback must be done. Repeat: No
    ///      * callback must be done if this method throws!
    ///      *
    ///      * @see nsIAsyncVerifyRedirectCallback::onRedirectVerifyCallback()
    ///      *
    ///      * @param oldChannel
    ///      *        The channel that's being redirected.
    ///      * @param newChannel
    ///      *        The new channel. This channel is not opened yet.
    ///      * @param flags
    ///      *        Flags indicating the type of redirect. A bitmask consisting
    ///      *        of flags from above.
    ///      *        One of REDIRECT_TEMPORARY and REDIRECT_PERMANENT will always be
    ///      *        set.
    ///      * @param callback
    ///      *        Object to inform about the async result of this method
    ///      *
    ///      * @throw <any> Throwing an exception will cause the redirect to be
    ///      *        cancelled
    ///      */
    /// ```
    ///

    /// `void asyncOnChannelRedirect (in nsIChannel oldChannel, in nsIChannel newChannel, in unsigned long flags, in nsIAsyncVerifyRedirectCallback callback);`
    #[inline]
    pub unsafe fn AsyncOnChannelRedirect(&self, oldChannel: *const nsIChannel, newChannel: *const nsIChannel, flags: u32, callback: *const nsIAsyncVerifyRedirectCallback) -> ::nserror::nsresult {
        ((*self.vtable).AsyncOnChannelRedirect)(self, oldChannel, newChannel, flags, callback)
    }


}


