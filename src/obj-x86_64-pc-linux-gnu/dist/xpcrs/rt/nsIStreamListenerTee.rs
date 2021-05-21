//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIStreamListenerTee.idl
//


/// `interface nsIStreamListenerTee : nsIStreamListener`
///

/// ```text
/// /**
///  * As data "flows" into a stream listener tee, it is copied to the output stream
///  * and then forwarded to the real listener.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIStreamListenerTee {
    vtable: *const nsIStreamListenerTeeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIStreamListenerTee.
unsafe impl XpCom for nsIStreamListenerTee {
    const IID: nsIID = nsID(0x62b27fc1, 0x6e8c, 0x4225,
        [0x8a, 0xd0, 0xb9, 0xd4, 0x42, 0x52, 0x97, 0x3a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIStreamListenerTee {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIStreamListenerTee.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIStreamListenerTeeCoerce {
    /// Cheaply cast a value of this type from a `nsIStreamListenerTee`.
    fn coerce_from(v: &nsIStreamListenerTee) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIStreamListenerTeeCoerce for nsIStreamListenerTee {
    #[inline]
    fn coerce_from(v: &nsIStreamListenerTee) -> &Self {
        v
    }
}

impl nsIStreamListenerTee {
    /// Cast this `nsIStreamListenerTee` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIStreamListenerTeeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIStreamListenerTee {
    type Target = nsIStreamListener;
    #[inline]
    fn deref(&self) -> &nsIStreamListener {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIStreamListenerCoerce> nsIStreamListenerTeeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStreamListenerTee) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIStreamListenerTee
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIStreamListenerTeeVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIStreamListenerVTable,

    /* void init (in nsIStreamListener listener, in nsIOutputStream sink, [optional] in nsIRequestObserver requestObserver); */
    pub Init: unsafe extern "system" fn (this: *const nsIStreamListenerTee, listener: *const nsIStreamListener, sink: *const nsIOutputStream, requestObserver: *const nsIRequestObserver) -> ::nserror::nsresult,

    /* void initAsync (in nsIStreamListener listener, in nsIEventTarget eventTarget, in nsIOutputStream sink, [optional] in nsIRequestObserver requestObserver); */
    pub InitAsync: unsafe extern "system" fn (this: *const nsIStreamListenerTee, listener: *const nsIStreamListener, eventTarget: *const nsIEventTarget, sink: *const nsIOutputStream, requestObserver: *const nsIRequestObserver) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIStreamListenerTee {

    /// ```text
    /// /**
    ///      * Initalize the tee.
    ///      *
    ///      * @param listener
    ///      *    the original listener the tee will propagate onStartRequest,
    ///      *    onDataAvailable and onStopRequest notifications to, exceptions from
    ///      *    the listener will be propagated back to the channel
    ///      * @param sink
    ///      *    the stream the data coming from the channel will be written to,
    ///      *    should be blocking
    ///      * @param requestObserver
    ///      *    optional parameter, listener that gets only onStartRequest and
    ///      *    onStopRequest notifications; exceptions threw within this optional
    ///      *    observer are also propagated to the channel, but exceptions from
    ///      *    the original listener (listener parameter) are privileged
    ///      */
    /// ```
    ///

    /// `void init (in nsIStreamListener listener, in nsIOutputStream sink, [optional] in nsIRequestObserver requestObserver);`
    #[inline]
    pub unsafe fn Init(&self, listener: *const nsIStreamListener, sink: *const nsIOutputStream, requestObserver: *const nsIRequestObserver) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, listener, sink, requestObserver)
    }


    /// ```text
    /// /**
    ///      * Initalize the tee like above, but with the extra parameter to make it
    ///      * possible to copy the output asynchronously
    ///      * @param anEventTarget
    ///      *    if set, this event-target is used to copy data to the output stream,
    ///      *    giving an asynchronous tee
    ///     */
    /// ```
    ///

    /// `void initAsync (in nsIStreamListener listener, in nsIEventTarget eventTarget, in nsIOutputStream sink, [optional] in nsIRequestObserver requestObserver);`
    #[inline]
    pub unsafe fn InitAsync(&self, listener: *const nsIStreamListener, eventTarget: *const nsIEventTarget, sink: *const nsIOutputStream, requestObserver: *const nsIRequestObserver) -> ::nserror::nsresult {
        ((*self.vtable).InitAsync)(self, listener, eventTarget, sink, requestObserver)
    }


}


