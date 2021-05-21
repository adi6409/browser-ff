//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIAsyncInputStream.idl
//


/// `interface nsIAsyncInputStream : nsIInputStream`
///

/// ```text
/// /**
///  * If an input stream is non-blocking, it may return NS_BASE_STREAM_WOULD_BLOCK
///  * when read.  The caller must then wait for the stream to have some data to
///  * read.  If the stream implements nsIAsyncInputStream, then the caller can use
///  * this interface to request an asynchronous notification when the stream
///  * becomes readable or closed (via the AsyncWait method).
///  *
///  * While this interface is almost exclusively used with non-blocking streams, it
///  * is not necessary that nsIInputStream::isNonBlocking return true.  Nor is it
///  * necessary that a non-blocking nsIInputStream implementation also implement
///  * nsIAsyncInputStream.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAsyncInputStream {
    vtable: *const nsIAsyncInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAsyncInputStream.
unsafe impl XpCom for nsIAsyncInputStream {
    const IID: nsIID = nsID(0xa5f255ab, 0x4801, 0x4161,
        [0x88, 0x16, 0x27, 0x7a, 0xc9, 0x2f, 0x6a, 0xd1]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAsyncInputStream {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAsyncInputStream.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAsyncInputStreamCoerce {
    /// Cheaply cast a value of this type from a `nsIAsyncInputStream`.
    fn coerce_from(v: &nsIAsyncInputStream) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAsyncInputStreamCoerce for nsIAsyncInputStream {
    #[inline]
    fn coerce_from(v: &nsIAsyncInputStream) -> &Self {
        v
    }
}

impl nsIAsyncInputStream {
    /// Cast this `nsIAsyncInputStream` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAsyncInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAsyncInputStream {
    type Target = nsIInputStream;
    #[inline]
    fn deref(&self) -> &nsIInputStream {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIInputStreamCoerce> nsIAsyncInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAsyncInputStream) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAsyncInputStream
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAsyncInputStreamVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIInputStreamVTable,

    /* void closeWithStatus (in nsresult aStatus); */
    pub CloseWithStatus: unsafe extern "system" fn (this: *const nsIAsyncInputStream, aStatus: ::nserror::nsresult) -> ::nserror::nsresult,

    /* void asyncWait (in nsIInputStreamCallback aCallback, in unsigned long aFlags, in unsigned long aRequestedCount, in nsIEventTarget aEventTarget); */
    pub AsyncWait: unsafe extern "system" fn (this: *const nsIAsyncInputStream, aCallback: *const nsIInputStreamCallback, aFlags: u32, aRequestedCount: u32, aEventTarget: *const nsIEventTarget) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAsyncInputStream {
    /// ```text
    /// /**
    ///      * If passed to asyncWait, this flag overrides the default behavior,
    ///      * causing the OnInputStreamReady notification to be suppressed until the
    ///      * stream becomes closed (either as a result of closeWithStatus/close being
        ///      * called on the stream or possibly due to some error in the underlying
        ///      * stream).
    ///      */
    /// ```
    ///

    pub const WAIT_CLOSURE_ONLY: i64 = 1;

    /// ```text
    /// /**
    ///      * This method closes the stream and sets its internal status.  If the
    ///      * stream is already closed, then this method is ignored.  Once the stream
    ///      * is closed, the stream's status cannot be changed.  Any successful status
    ///      * code passed to this method is treated as NS_BASE_STREAM_CLOSED, which
    ///      * has an effect equivalent to nsIInputStream::close.
    ///      *
    ///      * NOTE: this method exists in part to support pipes, which have both an
    ///      * input end and an output end.  If the input end of a pipe is closed, then
    ///      * writes to the output end of the pipe will fail.  The error code returned
    ///      * when an attempt is made to write to a "broken" pipe corresponds to the
    ///      * status code passed in when the input end of the pipe was closed, which
    ///      * greatly simplifies working with pipes in some cases.
    ///      *
    ///      * @param aStatus
    ///      *        The error that will be reported if this stream is accessed after
    ///      *        it has been closed.
    ///      */
    /// ```
    ///

    /// `void closeWithStatus (in nsresult aStatus);`
    #[inline]
    pub unsafe fn CloseWithStatus(&self, aStatus: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).CloseWithStatus)(self, aStatus)
    }


    /// ```text
    /// /**
    ///      * Asynchronously wait for the stream to be readable or closed.  The
    ///      * notification is one-shot, meaning that each asyncWait call will result
    ///      * in exactly one notification callback.  After the OnInputStreamReady event
    ///      * is dispatched, the stream releases its reference to the
    ///      * nsIInputStreamCallback object.  It is safe to call asyncWait again from the
    ///      * notification handler.
    ///      *
    ///      * This method may be called at any time (even if read has not been called).
    ///      * In other words, this method may be called when the stream already has
    ///      * data to read.  It may also be called when the stream is closed and will NOT
    ///      * result in an error return, e.g., NS_BASE_STREAM_CLOSED.  If the stream is
    ///      * already readable or closed when AsyncWait is called, then the
    ///      * OnInputStreamReady event will be dispatched immediately.  Otherwise, the
    ///      * event will be dispatched when the stream becomes readable or closed.
    ///      *
    ///      * @param aCallback
    ///      *        This object is notified when the stream becomes ready.  This
    ///      *        parameter may be null to clear an existing callback.
    ///      * @param aFlags
    ///      *        This parameter specifies optional flags passed in to configure
    ///      *        the behavior of this method.  Pass zero to specify no flags.
    ///      * @param aRequestedCount
    ///      *        Wait until at least this many bytes can be read.  This is only
    ///      *        a suggestion to the underlying stream; it may be ignored.  The
    ///      *        caller may pass zero to indicate no preference.
    ///      * @param aEventTarget
    ///      *        Specify NULL to receive notification on ANY thread (possibly even
        ///      *        recursively on the calling thread -- i.e., synchronously), or
    ///      *        specify that the notification be delivered to a specific event
    ///      *        target.
    ///      */
    /// ```
    ///

    /// `void asyncWait (in nsIInputStreamCallback aCallback, in unsigned long aFlags, in unsigned long aRequestedCount, in nsIEventTarget aEventTarget);`
    #[inline]
    pub unsafe fn AsyncWait(&self, aCallback: *const nsIInputStreamCallback, aFlags: u32, aRequestedCount: u32, aEventTarget: *const nsIEventTarget) -> ::nserror::nsresult {
        ((*self.vtable).AsyncWait)(self, aCallback, aFlags, aRequestedCount, aEventTarget)
    }


}


/// `interface nsIInputStreamCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIInputStreamCallback {
    vtable: *const nsIInputStreamCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIInputStreamCallback.
unsafe impl XpCom for nsIInputStreamCallback {
    const IID: nsIID = nsID(0xd1f28e94, 0x3a6e, 0x4050,
        [0xa5, 0xf5, 0x2e, 0x81, 0xb1, 0xfc, 0x2a, 0x43]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIInputStreamCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIInputStreamCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIInputStreamCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIInputStreamCallback`.
    fn coerce_from(v: &nsIInputStreamCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIInputStreamCallbackCoerce for nsIInputStreamCallback {
    #[inline]
    fn coerce_from(v: &nsIInputStreamCallback) -> &Self {
        v
    }
}

impl nsIInputStreamCallback {
    /// Cast this `nsIInputStreamCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIInputStreamCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIInputStreamCallback {
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
impl<T: nsISupportsCoerce> nsIInputStreamCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIInputStreamCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIInputStreamCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIInputStreamCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onInputStreamReady (in nsIAsyncInputStream aStream); */
    pub OnInputStreamReady: unsafe extern "system" fn (this: *const nsIInputStreamCallback, aStream: *const nsIAsyncInputStream) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIInputStreamCallback {

    /// ```text
    /// /**
    ///  * This is a companion interface for nsIAsyncInputStream::asyncWait.
    ///  */
    /// /**
    ///      * Called to indicate that the stream is either readable or closed.
    ///      *
    ///      * @param aStream
    ///      *        The stream whose asyncWait method was called.
    ///      */
    /// ```
    ///

    /// `void onInputStreamReady (in nsIAsyncInputStream aStream);`
    #[inline]
    pub unsafe fn OnInputStreamReady(&self, aStream: *const nsIAsyncInputStream) -> ::nserror::nsresult {
        ((*self.vtable).OnInputStreamReady)(self, aStream)
    }


}


