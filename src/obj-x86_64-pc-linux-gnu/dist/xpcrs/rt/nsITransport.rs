//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsITransport.idl
//


/// `interface nsITransport : nsISupports`
///

/// ```text
/// /**
///  * nsITransport
///  *
///  * This interface provides a common way of accessing i/o streams connected
///  * to some resource.  This interface does not in any way specify the resource.
///  * It provides methods to open blocking or non-blocking, buffered or unbuffered
///  * streams to the resource.  The name "transport" is meant to connote the
///  * inherent data transfer implied by this interface (i.e., data is being
    ///  * transfered in some fashion via the streams exposed by this interface).
///  *
///  * A transport can have an event sink associated with it.  The event sink
///  * receives transport-specific events as the transfer is occuring.  For a
///  * socket transport, these events can include status about the connection.
///  * See nsISocketTransport for more info about socket transport specifics.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITransport {
    vtable: *const nsITransportVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITransport.
unsafe impl XpCom for nsITransport {
    const IID: nsIID = nsID(0x2a8c6334, 0xa5e6, 0x4ec3,
        [0x98, 0x65, 0x12, 0x56, 0x54, 0x14, 0x46, 0xfb]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITransport {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITransport.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITransportCoerce {
    /// Cheaply cast a value of this type from a `nsITransport`.
    fn coerce_from(v: &nsITransport) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITransportCoerce for nsITransport {
    #[inline]
    fn coerce_from(v: &nsITransport) -> &Self {
        v
    }
}

impl nsITransport {
    /// Cast this `nsITransport` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITransportCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITransport {
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
impl<T: nsISupportsCoerce> nsITransportCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITransport) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITransport
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITransportVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIInputStream openInputStream (in unsigned long aFlags, in unsigned long aSegmentSize, in unsigned long aSegmentCount); */
    pub OpenInputStream: unsafe extern "system" fn (this: *const nsITransport, aFlags: u32, aSegmentSize: u32, aSegmentCount: u32, _retval: *mut*const nsIInputStream) -> ::nserror::nsresult,

    /* nsIOutputStream openOutputStream (in unsigned long aFlags, in unsigned long aSegmentSize, in unsigned long aSegmentCount); */
    pub OpenOutputStream: unsafe extern "system" fn (this: *const nsITransport, aFlags: u32, aSegmentSize: u32, aSegmentCount: u32, _retval: *mut*const nsIOutputStream) -> ::nserror::nsresult,

    /* void close (in nsresult aReason); */
    pub Close: unsafe extern "system" fn (this: *const nsITransport, aReason: ::nserror::nsresult) -> ::nserror::nsresult,

    /* void setEventSink (in nsITransportEventSink aSink, in nsIEventTarget aEventTarget); */
    pub SetEventSink: unsafe extern "system" fn (this: *const nsITransport, aSink: *const nsITransportEventSink, aEventTarget: *const nsIEventTarget) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITransport {
    /// ```text
    /// /**
    ///      * Open flags.
    ///      */
    /// ```
    ///

    pub const OPEN_BLOCKING: i64 = 1;


    pub const OPEN_UNBUFFERED: i64 = 2;

    /// ```text
    /// /**
    ///      * Generic nsITransportEventSink status codes.  nsITransport
    ///      * implementations may override these status codes with their own more
    ///      * specific status codes (e.g., see nsISocketTransport).
    ///      *
    ///      * In C++, these constants have a type of uint32_t, so C++ callers must use
    ///      * the NS_NET_STATUS_* constants defined below, which have a type of
    ///      * nsresult.
    ///      */
    /// ```
    ///

    pub const STATUS_READING: i64 = 2152398856;


    pub const STATUS_WRITING: i64 = 2152398857;

    /// ```text
    /// /**
    ///      * Open an input stream on this transport.
    ///      *
    ///      * Flags have the following meaning:
    ///      *
    ///      * OPEN_BLOCKING
    ///      *   If specified, then the resulting stream will have blocking stream
    ///      *   semantics.  This means that if the stream has no data and is not
    ///      *   closed, then reading from it will block the calling thread until
    ///      *   at least one byte is available or until the stream is closed.
    ///      *   If this flag is NOT specified, then the stream has non-blocking
    ///      *   stream semantics.  This means that if the stream has no data and is
    ///      *   not closed, then reading from it returns NS_BASE_STREAM_WOULD_BLOCK.
    ///      *   In addition, in non-blocking mode, the stream is guaranteed to
    ///      *   support nsIAsyncInputStream.  This interface allows the consumer of
    ///      *   the stream to be notified when the stream can again be read.
    ///      *
    ///      * OPEN_UNBUFFERED
    ///      *   If specified, the resulting stream may not support ReadSegments.
    ///      *   ReadSegments is only gauranteed to be implemented when this flag is
    ///      *   NOT specified.
    ///      *
    ///      * @param aFlags
    ///      *        optional transport specific flags.
    ///      * @param aSegmentSize
    ///      *        if OPEN_UNBUFFERED is not set, then this parameter specifies the
    ///      *        size of each buffer segment (pass 0 to use default value).
    ///      * @param aSegmentCount
    ///      *        if OPEN_UNBUFFERED is not set, then this parameter specifies the
    ///      *        maximum number of buffer segments (pass 0 to use default value).
    ///      */
    /// ```
    ///

    /// `nsIInputStream openInputStream (in unsigned long aFlags, in unsigned long aSegmentSize, in unsigned long aSegmentCount);`
    #[inline]
    pub unsafe fn OpenInputStream(&self, aFlags: u32, aSegmentSize: u32, aSegmentCount: u32, _retval: *mut*const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).OpenInputStream)(self, aFlags, aSegmentSize, aSegmentCount, _retval)
    }


    /// ```text
    /// /**
    ///      * Open an output stream on this transport.
    ///      *
    ///      * Flags have the following meaning:
    ///      *
    ///      * OPEN_BLOCKING
    ///      *   If specified, then the resulting stream will have blocking stream
    ///      *   semantics.  This means that if the stream is full and is not closed,
    ///      *   then writing to it will block the calling thread until ALL of the
    ///      *   data can be written or until the stream is closed.  If this flag is
    ///      *   NOT specified, then the stream has non-blocking stream semantics.
    ///      *   This means that if the stream is full and is not closed, then writing
    ///      *   to it returns NS_BASE_STREAM_WOULD_BLOCK.  In addition, in non-
    ///      *   blocking mode, the stream is guaranteed to support
    ///      *   nsIAsyncOutputStream.  This interface allows the consumer of the
    ///      *   stream to be notified when the stream can again accept more data.
    ///      *
    ///      * OPEN_UNBUFFERED
    ///      *   If specified, the resulting stream may not support WriteSegments and
    ///      *   WriteFrom.  WriteSegments and WriteFrom are only guaranteed to be
    ///      *   implemented when this flag is NOT specified.
    ///      *
    ///      * @param aFlags
    ///      *        optional transport specific flags.
    ///      * @param aSegmentSize
    ///      *        if OPEN_UNBUFFERED is not set, then this parameter specifies the
    ///      *        size of each buffer segment (pass 0 to use default value).
    ///      * @param aSegmentCount
    ///      *        if OPEN_UNBUFFERED is not set, then this parameter specifies the
    ///      *        maximum number of buffer segments (pass 0 to use default value).
    ///      */
    /// ```
    ///

    /// `nsIOutputStream openOutputStream (in unsigned long aFlags, in unsigned long aSegmentSize, in unsigned long aSegmentCount);`
    #[inline]
    pub unsafe fn OpenOutputStream(&self, aFlags: u32, aSegmentSize: u32, aSegmentCount: u32, _retval: *mut*const nsIOutputStream) -> ::nserror::nsresult {
        ((*self.vtable).OpenOutputStream)(self, aFlags, aSegmentSize, aSegmentCount, _retval)
    }


    /// ```text
    /// /**
    ///      * Close the transport and any open streams.
    ///      *
    ///      * @param aReason
    ///      *        the reason for closing the stream.
    ///      */
    /// ```
    ///

    /// `void close (in nsresult aReason);`
    #[inline]
    pub unsafe fn Close(&self, aReason: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).Close)(self, aReason)
    }


    /// ```text
    /// /**
    ///      * Set the transport event sink.
    ///      *
    ///      * @param aSink
    ///      *        receives transport layer notifications
    ///      * @param aEventTarget
    ///      *        indicates the event target to which the notifications should
    ///      *        be delivered.  if NULL, then the notifications may occur on
    ///      *        any thread.
    ///      */
    /// ```
    ///

    /// `void setEventSink (in nsITransportEventSink aSink, in nsIEventTarget aEventTarget);`
    #[inline]
    pub unsafe fn SetEventSink(&self, aSink: *const nsITransportEventSink, aEventTarget: *const nsIEventTarget) -> ::nserror::nsresult {
        ((*self.vtable).SetEventSink)(self, aSink, aEventTarget)
    }


}


/// `interface nsITransportEventSink : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITransportEventSink {
    vtable: *const nsITransportEventSinkVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITransportEventSink.
unsafe impl XpCom for nsITransportEventSink {
    const IID: nsIID = nsID(0xeda4f520, 0x67f7, 0x484b,
        [0xa6, 0x91, 0x8c, 0x32, 0x26, 0xa5, 0xb0, 0xa6]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITransportEventSink {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITransportEventSink.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITransportEventSinkCoerce {
    /// Cheaply cast a value of this type from a `nsITransportEventSink`.
    fn coerce_from(v: &nsITransportEventSink) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITransportEventSinkCoerce for nsITransportEventSink {
    #[inline]
    fn coerce_from(v: &nsITransportEventSink) -> &Self {
        v
    }
}

impl nsITransportEventSink {
    /// Cast this `nsITransportEventSink` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITransportEventSinkCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITransportEventSink {
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
impl<T: nsISupportsCoerce> nsITransportEventSinkCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITransportEventSink) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITransportEventSink
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITransportEventSinkVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onTransportStatus (in nsITransport aTransport, in nsresult aStatus, in long long aProgress, in long long aProgressMax); */
    pub OnTransportStatus: unsafe extern "system" fn (this: *const nsITransportEventSink, aTransport: *const nsITransport, aStatus: ::nserror::nsresult, aProgress: i64, aProgressMax: i64) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITransportEventSink {

    /// ```text
    /// /**
    ///      * Transport status notification.
    ///      *
    ///      * @param aTransport
    ///      *        the transport sending this status notification.
    ///      * @param aStatus
    ///      *        the transport status (resolvable to a string using
        ///      *        nsIErrorService). See nsISocketTransport for socket specific
    ///      *        status codes and more comments.
    ///      * @param aProgress
    ///      *        the amount of data either read or written depending on the value
    ///      *        of the status code.  this value is relative to aProgressMax.
    ///      * @param aProgressMax
    ///      *        the maximum amount of data that will be read or written.  if
    ///      *        unknown, -1 will be passed.
    ///      */
    /// ```
    ///

    /// `void onTransportStatus (in nsITransport aTransport, in nsresult aStatus, in long long aProgress, in long long aProgressMax);`
    #[inline]
    pub unsafe fn OnTransportStatus(&self, aTransport: *const nsITransport, aStatus: ::nserror::nsresult, aProgress: i64, aProgressMax: i64) -> ::nserror::nsresult {
        ((*self.vtable).OnTransportStatus)(self, aTransport, aStatus, aProgress, aProgressMax)
    }


}


