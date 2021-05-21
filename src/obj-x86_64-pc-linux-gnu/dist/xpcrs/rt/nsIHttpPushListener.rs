//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIHttpPushListener.idl
//


/// `interface nsIHttpPushListener : nsISupports`
///

/// ```text
/// /**
///  * nsIHttpPushListener
///  *
///  * Used for triggering when a HTTP/2 push is received.
///  *
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHttpPushListener {
    vtable: *const nsIHttpPushListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHttpPushListener.
unsafe impl XpCom for nsIHttpPushListener {
    const IID: nsIID = nsID(0x0d6ce59c, 0xad5d, 0x4520,
        [0xb4, 0xd3, 0x09, 0x66, 0x48, 0x68, 0xf2, 0x79]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHttpPushListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHttpPushListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHttpPushListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIHttpPushListener`.
    fn coerce_from(v: &nsIHttpPushListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHttpPushListenerCoerce for nsIHttpPushListener {
    #[inline]
    fn coerce_from(v: &nsIHttpPushListener) -> &Self {
        v
    }
}

impl nsIHttpPushListener {
    /// Cast this `nsIHttpPushListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHttpPushListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHttpPushListener {
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
impl<T: nsISupportsCoerce> nsIHttpPushListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpPushListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHttpPushListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHttpPushListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onPush (in nsIHttpChannel associatedChannel, in nsIHttpChannel pushChannel); */
    pub OnPush: unsafe extern "system" fn (this: *const nsIHttpPushListener, associatedChannel: *const nsIHttpChannel, pushChannel: *const nsIHttpChannel) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHttpPushListener {

    /// ```text
    /// /**
    ///    * When provided as a notificationCallback to an httpChannel, this.onPush()
    ///    * will be invoked when there is a >= Http2 push to that
    ///    * channel. The push may be in progress.
    ///    *
    ///    * The consumer must start the new channel in the usual way by calling
    ///    * pushChannel.AsyncOpen with a nsIStreamListener object that
    ///    * will receive the normal sequence of OnStartRequest(),
    ///    * 0 to N OnDataAvailable(), and onStopRequest().
    ///    *
    ///    * The new channel can be canceled after the AsyncOpen if it is not wanted.
    ///    *
    ///    * @param associatedChannel
    ///    *        the monitor channel that was recieved on
    ///    * @param pushChannel
    ///    *        a channel to the resource which is being pushed
    ///    */
    /// ```
    ///

    /// `void onPush (in nsIHttpChannel associatedChannel, in nsIHttpChannel pushChannel);`
    #[inline]
    pub unsafe fn OnPush(&self, associatedChannel: *const nsIHttpChannel, pushChannel: *const nsIHttpChannel) -> ::nserror::nsresult {
        ((*self.vtable).OnPush)(self, associatedChannel, pushChannel)
    }


}


