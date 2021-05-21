//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIInputStreamPump.idl
//


/// `interface nsIInputStreamPump : nsIRequest`
///

/// ```text
/// /**
///  * nsIInputStreamPump
///  *
///  * This interface provides a means to configure and use a input stream pump
///  * instance.  The input stream pump will asynchronously read from an input
///  * stream, and push data to an nsIStreamListener instance.  It utilizes the
///  * current thread's nsIEventTarget in order to make reading from the stream
///  * asynchronous. A different thread can be used if the pump also implements
///  * nsIThreadRetargetableRequest.
///  *
///  * If the given stream supports nsIAsyncInputStream, then the stream pump will
///  * call the stream's AsyncWait method to drive the stream listener.  Otherwise,
///  * the stream will be read on a background thread utilizing the stream
///  * transport service.  More details are provided below.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIInputStreamPump {
    vtable: *const nsIInputStreamPumpVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIInputStreamPump.
unsafe impl XpCom for nsIInputStreamPump {
    const IID: nsIID = nsID(0x400f5468, 0x97e7, 0x4d2b,
        [0x9c, 0x65, 0xa8, 0x2a, 0xec, 0xc7, 0xae, 0x82]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIInputStreamPump {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIInputStreamPump.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIInputStreamPumpCoerce {
    /// Cheaply cast a value of this type from a `nsIInputStreamPump`.
    fn coerce_from(v: &nsIInputStreamPump) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIInputStreamPumpCoerce for nsIInputStreamPump {
    #[inline]
    fn coerce_from(v: &nsIInputStreamPump) -> &Self {
        v
    }
}

impl nsIInputStreamPump {
    /// Cast this `nsIInputStreamPump` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIInputStreamPumpCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIInputStreamPump {
    type Target = nsIRequest;
    #[inline]
    fn deref(&self) -> &nsIRequest {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIRequestCoerce> nsIInputStreamPumpCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIInputStreamPump) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIInputStreamPump
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIInputStreamPumpVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIRequestVTable,

    /* void init (in nsIInputStream aStream, in unsigned long aSegmentSize, in unsigned long aSegmentCount, in boolean aCloseWhenDone, [optional] in nsIEventTarget aMainThreadTarget); */
    pub Init: unsafe extern "system" fn (this: *const nsIInputStreamPump, aStream: *const nsIInputStream, aSegmentSize: u32, aSegmentCount: u32, aCloseWhenDone: bool, aMainThreadTarget: *const nsIEventTarget) -> ::nserror::nsresult,

    /* void asyncRead (in nsIStreamListener aListener); */
    pub AsyncRead: unsafe extern "system" fn (this: *const nsIInputStreamPump, aListener: *const nsIStreamListener) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIInputStreamPump {

    /// ```text
    /// /**
    ///      * Initialize the input stream pump.
    ///      *
    ///      * @param aStream
    ///      *        contains the data to be read.  if the input stream is non-blocking,
    ///      *        then it will be QI'd to nsIAsyncInputStream.  if the QI succeeds
    ///      *        then the stream will be read directly.  otherwise, it will be read
    ///      *        on a background thread using the stream transport service.
    ///      * @param aSegmentSize
    ///      *        if the stream transport service is used, then this parameter
    ///      *        specifies the segment size for the stream transport's buffer.
    ///      *        pass 0 to specify the default value.
    ///      * @param aSegmentCount
    ///      *        if the stream transport service is used, then this parameter
    ///      *        specifies the segment count for the stream transport's buffer.
    ///      *        pass 0 to specify the default value.
    ///      * @param aCloseWhenDone
    ///      *        if true, the input stream will be closed after it has been read.
    ///      * @param aMainThreadTarget
    ///      *        a labeled main therad event target.
    ///      */
    /// ```
    ///

    /// `void init (in nsIInputStream aStream, in unsigned long aSegmentSize, in unsigned long aSegmentCount, in boolean aCloseWhenDone, [optional] in nsIEventTarget aMainThreadTarget);`
    #[inline]
    pub unsafe fn Init(&self, aStream: *const nsIInputStream, aSegmentSize: u32, aSegmentCount: u32, aCloseWhenDone: bool, aMainThreadTarget: *const nsIEventTarget) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aStream, aSegmentSize, aSegmentCount, aCloseWhenDone, aMainThreadTarget)
    }


    /// ```text
    /// /**
    ///      * asyncRead causes the input stream to be read in chunks and delivered
    ///      * asynchronously to the listener via OnDataAvailable.
    ///      *
    ///      * @param aListener
    ///      *        receives notifications.
    ///      * @param aListenerContext
    ///      *        passed to listener methods.
    ///      */
    /// ```
    ///

    /// `void asyncRead (in nsIStreamListener aListener);`
    #[inline]
    pub unsafe fn AsyncRead(&self, aListener: *const nsIStreamListener) -> ::nserror::nsresult {
        ((*self.vtable).AsyncRead)(self, aListener)
    }


}


