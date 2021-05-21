//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIProgressEventSink.idl
//


/// `interface nsIProgressEventSink : nsISupports`
///

/// ```text
/// /**
///  * nsIProgressEventSink
///  *
///  * This interface is used to asynchronously convey channel status and progress
///  * information that is generally not critical to the processing of the channel.
///  * The information is intended to be displayed to the user in some meaningful
///  * way.
///  *
///  * An implementation of this interface can be passed to a channel via the
///  * channel's notificationCallbacks attribute.  See nsIChannel for more info.
///  *
///  * The channel will begin passing notifications to the progress event sink
///  * after its asyncOpen method has been called.  Notifications will cease once
///  * the channel calls its listener's onStopRequest method or once the channel
///  * is canceled (via nsIRequest::cancel).
///  *
///  * NOTE: This interface is actually not specific to channels and may be used
///  * with other implementations of nsIRequest.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIProgressEventSink {
    vtable: *const nsIProgressEventSinkVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIProgressEventSink.
unsafe impl XpCom for nsIProgressEventSink {
    const IID: nsIID = nsID(0x87d55fba, 0xcb7e, 0x4f38,
        [0x84, 0xc1, 0x5c, 0x6c, 0x2b, 0x2a, 0x55, 0xe9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIProgressEventSink {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIProgressEventSink.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIProgressEventSinkCoerce {
    /// Cheaply cast a value of this type from a `nsIProgressEventSink`.
    fn coerce_from(v: &nsIProgressEventSink) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIProgressEventSinkCoerce for nsIProgressEventSink {
    #[inline]
    fn coerce_from(v: &nsIProgressEventSink) -> &Self {
        v
    }
}

impl nsIProgressEventSink {
    /// Cast this `nsIProgressEventSink` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIProgressEventSinkCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIProgressEventSink {
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
impl<T: nsISupportsCoerce> nsIProgressEventSinkCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProgressEventSink) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIProgressEventSink
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIProgressEventSinkVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onProgress (in nsIRequest aRequest, in long long aProgress, in long long aProgressMax); */
    pub OnProgress: unsafe extern "system" fn (this: *const nsIProgressEventSink, aRequest: *const nsIRequest, aProgress: i64, aProgressMax: i64) -> ::nserror::nsresult,

    /* void onStatus (in nsIRequest aRequest, in nsresult aStatus, in wstring aStatusArg); */
    pub OnStatus: unsafe extern "system" fn (this: *const nsIProgressEventSink, aRequest: *const nsIRequest, aStatus: ::nserror::nsresult, aStatusArg: *const i16) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIProgressEventSink {

    /// ```text
    /// /**
    ///      * Called to notify the event sink that progress has occurred for the
    ///      * given request.
    ///      *
    ///      * @param aRequest
    ///      *        the request being observed (may QI to nsIChannel).
    ///      * @param aProgress
    ///      *        numeric value in the range 0 to aProgressMax indicating the
    ///      *        number of bytes transfered thus far.
    ///      * @param aProgressMax
    ///      *        numeric value indicating maximum number of bytes that will be
    ///      *        transfered (or -1 if total is unknown).
    ///      */
    /// ```
    ///

    /// `void onProgress (in nsIRequest aRequest, in long long aProgress, in long long aProgressMax);`
    #[inline]
    pub unsafe fn OnProgress(&self, aRequest: *const nsIRequest, aProgress: i64, aProgressMax: i64) -> ::nserror::nsresult {
        ((*self.vtable).OnProgress)(self, aRequest, aProgress, aProgressMax)
    }


    /// ```text
    /// /**
    ///      * Called to notify the event sink with a status message for the given
    ///      * request.
    ///      *
    ///      * @param aRequest
    ///      *        the request being observed (may QI to nsIChannel).
    ///      * @param aStatus
    ///      *        status code (not necessarily an error code) indicating the
    ///      *        state of the channel (usually the state of the underlying
        ///      *        transport).  see nsISocketTransport for socket specific status
    ///      *        codes.
    ///      * @param aStatusArg
    ///      *        status code argument to be used with the string bundle service
    ///      *        to convert the status message into localized, human readable
    ///      *        text.  the meaning of this parameter is specific to the value
    ///      *        of the status code.  for socket status codes, this parameter
    ///      *        indicates the host:port associated with the status code.
    ///      */
    /// ```
    ///

    /// `void onStatus (in nsIRequest aRequest, in nsresult aStatus, in wstring aStatusArg);`
    #[inline]
    pub unsafe fn OnStatus(&self, aRequest: *const nsIRequest, aStatus: ::nserror::nsresult, aStatusArg: *const i16) -> ::nserror::nsresult {
        ((*self.vtable).OnStatus)(self, aRequest, aStatus, aStatusArg)
    }


}


