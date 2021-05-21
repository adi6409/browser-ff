//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIThrottledInputChannel.idl
//


/// `interface nsIInputChannelThrottleQueue : nsISupports`
///

/// ```text
/// /**
///  * An instance of this interface can be used to throttle the uploads
///  * of a group of associated channels.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIInputChannelThrottleQueue {
    vtable: *const nsIInputChannelThrottleQueueVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIInputChannelThrottleQueue.
unsafe impl XpCom for nsIInputChannelThrottleQueue {
    const IID: nsIID = nsID(0x6b4b96fe, 0x3c67, 0x4587,
        [0xaf, 0x7b, 0x58, 0xb6, 0xb1, 0x7d, 0xa4, 0x11]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIInputChannelThrottleQueue {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIInputChannelThrottleQueue.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIInputChannelThrottleQueueCoerce {
    /// Cheaply cast a value of this type from a `nsIInputChannelThrottleQueue`.
    fn coerce_from(v: &nsIInputChannelThrottleQueue) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIInputChannelThrottleQueueCoerce for nsIInputChannelThrottleQueue {
    #[inline]
    fn coerce_from(v: &nsIInputChannelThrottleQueue) -> &Self {
        v
    }
}

impl nsIInputChannelThrottleQueue {
    /// Cast this `nsIInputChannelThrottleQueue` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIInputChannelThrottleQueueCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIInputChannelThrottleQueue {
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
impl<T: nsISupportsCoerce> nsIInputChannelThrottleQueueCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIInputChannelThrottleQueue) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIInputChannelThrottleQueue
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIInputChannelThrottleQueueVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void init (in unsigned long aMeanBytesPerSecond, in unsigned long aMaxBytesPerSecond); */
    pub Init: unsafe extern "system" fn (this: *const nsIInputChannelThrottleQueue, aMeanBytesPerSecond: u32, aMaxBytesPerSecond: u32) -> ::nserror::nsresult,

    /* [noscript] readonly attribute unsigned long meanBytesPerSecond; */
    pub GetMeanBytesPerSecond: unsafe extern "system" fn (this: *const nsIInputChannelThrottleQueue, aMeanBytesPerSecond: *mut u32) -> ::nserror::nsresult,

    /* [noscript] readonly attribute unsigned long maxBytesPerSecond; */
    pub GetMaxBytesPerSecond: unsafe extern "system" fn (this: *const nsIInputChannelThrottleQueue, aMaxBytesPerSecond: *mut u32) -> ::nserror::nsresult,

    /* unsigned long available (in unsigned long aRemaining); */
    pub Available: unsafe extern "system" fn (this: *const nsIInputChannelThrottleQueue, aRemaining: u32, _retval: *mut u32) -> ::nserror::nsresult,

    /* void recordRead (in unsigned long aBytesRead); */
    pub RecordRead: unsafe extern "system" fn (this: *const nsIInputChannelThrottleQueue, aBytesRead: u32) -> ::nserror::nsresult,

    /* unsigned long long bytesProcessed (); */
    pub BytesProcessed: unsafe extern "system" fn (this: *const nsIInputChannelThrottleQueue, _retval: *mut u64) -> ::nserror::nsresult,

    /* nsIAsyncInputStream wrapStream (in nsIInputStream aInputStream); */
    pub WrapStream: unsafe extern "system" fn (this: *const nsIInputChannelThrottleQueue, aInputStream: *const nsIInputStream, _retval: *mut*const nsIAsyncInputStream) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIInputChannelThrottleQueue {

    /// ```text
    /// /**
    ///      * Initialize this object with the mean and maximum bytes per
    ///      * second that will be allowed.  Neither value may be zero, and
    ///      * the maximum must not be less than the mean.
    ///      *
    ///      * @param aMeanBytesPerSecond
    ///      *        Mean number of bytes per second.
    ///      * @param aMaxBytesPerSecond
    ///      *        Maximum number of bytes per second.
    ///      */
    /// ```
    ///

    /// `void init (in unsigned long aMeanBytesPerSecond, in unsigned long aMaxBytesPerSecond);`
    #[inline]
    pub unsafe fn Init(&self, aMeanBytesPerSecond: u32, aMaxBytesPerSecond: u32) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aMeanBytesPerSecond, aMaxBytesPerSecond)
    }


    /// ```text
    /// /**
    ///      * Internal use only. Get the values set by init method.
    ///      */
    /// ```
    ///

    /// `[noscript] readonly attribute unsigned long meanBytesPerSecond;`
    #[inline]
    pub unsafe fn GetMeanBytesPerSecond(&self, aMeanBytesPerSecond: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetMeanBytesPerSecond)(self, aMeanBytesPerSecond)
    }



    /// `[noscript] readonly attribute unsigned long maxBytesPerSecond;`
    #[inline]
    pub unsafe fn GetMaxBytesPerSecond(&self, aMaxBytesPerSecond: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetMaxBytesPerSecond)(self, aMaxBytesPerSecond)
    }


    /// ```text
    /// /**
    ///      * Return the number of bytes that are available to the caller in
    ///      * this time slice.
    ///      *
    ///      * @param aRemaining
    ///      *        The number of bytes available to be processed
    ///      * @return the number of bytes allowed to be processed during this
    ///      *        time slice; this will never be greater than aRemaining.
    ///      */
    /// ```
    ///

    /// `unsigned long available (in unsigned long aRemaining);`
    #[inline]
    pub unsafe fn Available(&self, aRemaining: u32, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).Available)(self, aRemaining, _retval)
    }


    /// ```text
    /// /**
    ///      * Record a successful read.
    ///      *
    ///      * @param aBytesRead
    ///      *        The number of bytes actually read.
    ///      */
    /// ```
    ///

    /// `void recordRead (in unsigned long aBytesRead);`
    #[inline]
    pub unsafe fn RecordRead(&self, aBytesRead: u32) -> ::nserror::nsresult {
        ((*self.vtable).RecordRead)(self, aBytesRead)
    }


    /// ```text
    /// /**
    ///      * Return the number of bytes allowed through this queue.  This is
    ///      * the sum of all the values passed to recordRead.  This method is
    ///      * primarily useful for testing.
    ///      */
    /// ```
    ///

    /// `unsigned long long bytesProcessed ();`
    #[inline]
    pub unsafe fn BytesProcessed(&self, _retval: *mut u64) -> ::nserror::nsresult {
        ((*self.vtable).BytesProcessed)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * Wrap the given input stream in a new input stream which
    ///      * throttles the incoming data.
    ///      *
    ///      * @param aInputStream the input stream to wrap
    ///      * @return a new input stream that throttles the data.
    ///      */
    /// ```
    ///

    /// `nsIAsyncInputStream wrapStream (in nsIInputStream aInputStream);`
    #[inline]
    pub unsafe fn WrapStream(&self, aInputStream: *const nsIInputStream, _retval: *mut*const nsIAsyncInputStream) -> ::nserror::nsresult {
        ((*self.vtable).WrapStream)(self, aInputStream, _retval)
    }


}


/// `interface nsIThrottledInputChannel : nsISupports`
///

/// ```text
/// /**
///  * A throttled input channel can be managed by an
///  * nsIInputChannelThrottleQueue to limit how much data is sent during
///  * a given time slice.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIThrottledInputChannel {
    vtable: *const nsIThrottledInputChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIThrottledInputChannel.
unsafe impl XpCom for nsIThrottledInputChannel {
    const IID: nsIID = nsID(0x0a32a100, 0xc031, 0x45b6,
        [0x9e, 0x8b, 0x04, 0x44, 0xc7, 0xd4, 0xa1, 0x43]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIThrottledInputChannel {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIThrottledInputChannel.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIThrottledInputChannelCoerce {
    /// Cheaply cast a value of this type from a `nsIThrottledInputChannel`.
    fn coerce_from(v: &nsIThrottledInputChannel) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIThrottledInputChannelCoerce for nsIThrottledInputChannel {
    #[inline]
    fn coerce_from(v: &nsIThrottledInputChannel) -> &Self {
        v
    }
}

impl nsIThrottledInputChannel {
    /// Cast this `nsIThrottledInputChannel` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIThrottledInputChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIThrottledInputChannel {
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
impl<T: nsISupportsCoerce> nsIThrottledInputChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIThrottledInputChannel) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIThrottledInputChannel
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIThrottledInputChannelVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute nsIInputChannelThrottleQueue throttleQueue; */
    pub GetThrottleQueue: unsafe extern "system" fn (this: *const nsIThrottledInputChannel, aThrottleQueue: *mut *const nsIInputChannelThrottleQueue) -> ::nserror::nsresult,

    /* attribute nsIInputChannelThrottleQueue throttleQueue; */
    pub SetThrottleQueue: unsafe extern "system" fn (this: *const nsIThrottledInputChannel, aThrottleQueue: *const nsIInputChannelThrottleQueue) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIThrottledInputChannel {

    /// ```text
    /// /**
    ///      * The queue that manages this channel.  Multiple channels can
    ///      * share a single queue.  A null value means that no throttling
    ///      * will be done.
    ///      */
    /// ```
    ///

    /// `attribute nsIInputChannelThrottleQueue throttleQueue;`
    #[inline]
    pub unsafe fn GetThrottleQueue(&self, aThrottleQueue: *mut *const nsIInputChannelThrottleQueue) -> ::nserror::nsresult {
        ((*self.vtable).GetThrottleQueue)(self, aThrottleQueue)
    }


    /// ```text
    /// /**
    ///      * The queue that manages this channel.  Multiple channels can
    ///      * share a single queue.  A null value means that no throttling
    ///      * will be done.
    ///      */
    /// ```
    ///

    /// `attribute nsIInputChannelThrottleQueue throttleQueue;`
    #[inline]
    pub unsafe fn SetThrottleQueue(&self, aThrottleQueue: *const nsIInputChannelThrottleQueue) -> ::nserror::nsresult {
        ((*self.vtable).SetThrottleQueue)(self, aThrottleQueue)
    }


}


