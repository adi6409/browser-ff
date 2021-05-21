//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIAsyncOutputStream.idl
//


/// `interface nsIAsyncOutputStream : nsIOutputStream`
///

/// ```text
/// /**
///  * If an output stream is non-blocking, it may return NS_BASE_STREAM_WOULD_BLOCK
///  * when written to.  The caller must then wait for the stream to become
///  * writable.  If the stream implements nsIAsyncOutputStream, then the caller can
///  * use this interface to request an asynchronous notification when the stream
///  * becomes writable or closed (via the AsyncWait method).
///  *
///  * While this interface is almost exclusively used with non-blocking streams, it
///  * is not necessary that nsIOutputStream::isNonBlocking return true.  Nor is it
///  * necessary that a non-blocking nsIOutputStream implementation also implement
///  * nsIAsyncOutputStream.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAsyncOutputStream {
    vtable: *const nsIAsyncOutputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAsyncOutputStream.
unsafe impl XpCom for nsIAsyncOutputStream {
    const IID: nsIID = nsID(0xbeb632d3, 0xd77a, 0x4e90,
        [0x91, 0x34, 0xf9, 0xec, 0xe6, 0x9e, 0x82, 0x00]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAsyncOutputStream {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAsyncOutputStream.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAsyncOutputStreamCoerce {
    /// Cheaply cast a value of this type from a `nsIAsyncOutputStream`.
    fn coerce_from(v: &nsIAsyncOutputStream) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAsyncOutputStreamCoerce for nsIAsyncOutputStream {
    #[inline]
    fn coerce_from(v: &nsIAsyncOutputStream) -> &Self {
        v
    }
}

impl nsIAsyncOutputStream {
    /// Cast this `nsIAsyncOutputStream` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAsyncOutputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAsyncOutputStream {
    type Target = nsIOutputStream;
    #[inline]
    fn deref(&self) -> &nsIOutputStream {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIOutputStreamCoerce> nsIAsyncOutputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAsyncOutputStream) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAsyncOutputStream
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAsyncOutputStreamVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIOutputStreamVTable,

    /* void closeWithStatus (in nsresult reason); */
    pub CloseWithStatus: unsafe extern "system" fn (this: *const nsIAsyncOutputStream, reason: ::nserror::nsresult) -> ::nserror::nsresult,

    /* void asyncWait (in nsIOutputStreamCallback aCallback, in unsigned long aFlags, in unsigned long aRequestedCount, in nsIEventTarget aEventTarget); */
    pub AsyncWait: unsafe extern "system" fn (this: *const nsIAsyncOutputStream, aCallback: *const nsIOutputStreamCallback, aFlags: u32, aRequestedCount: u32, aEventTarget: *const nsIEventTarget) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAsyncOutputStream {
    /// ```text
    /// /**
    ///      * If passed to asyncWait, this flag overrides the default behavior,
    ///      * causing the OnOutputStreamReady notification to be suppressed until the
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
    ///      * is equivalent to nsIInputStream::close.
    ///      *
    ///      * NOTE: this method exists in part to support pipes, which have both an
    ///      * input end and an output end.  If the output end of a pipe is closed, then
    ///      * reads from the input end of the pipe will fail.  The error code returned
    ///      * when an attempt is made to read from a "closed" pipe corresponds to the
    ///      * status code passed in when the output end of the pipe is closed, which
    ///      * greatly simplifies working with pipes in some cases.
    ///      *
    ///      * @param aStatus
    ///      *        The error that will be reported if this stream is accessed after
    ///      *        it has been closed.
    ///      */
    /// ```
    ///

    /// `void closeWithStatus (in nsresult reason);`
    #[inline]
    pub unsafe fn CloseWithStatus(&self, reason: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).CloseWithStatus)(self, reason)
    }


    /// ```text
    /// /**
    ///      * Asynchronously wait for the stream to be writable or closed.  The
    ///      * notification is one-shot, meaning that each asyncWait call will result
    ///      * in exactly one notification callback.  After the OnOutputStreamReady event
    ///      * is dispatched, the stream releases its reference to the
    ///      * nsIOutputStreamCallback object.  It is safe to call asyncWait again from the
    ///      * notification handler.
    ///      *
    ///      * This method may be called at any time (even if write has not been called).
    ///      * In other words, this method may be called when the stream already has
    ///      * room for more data.  It may also be called when the stream is closed.  If
    ///      * the stream is already writable or closed when AsyncWait is called, then the
    ///      * OnOutputStreamReady event will be dispatched immediately.  Otherwise, the
    ///      * event will be dispatched when the stream becomes writable or closed.
    ///      *
    ///      * @param aCallback
    ///      *        This object is notified when the stream becomes ready.  This
    ///      *        parameter may be null to clear an existing callback.
    ///      * @param aFlags
    ///      *        This parameter specifies optional flags passed in to configure
    ///      *        the behavior of this method.  Pass zero to specify no flags.
    ///      * @param aRequestedCount
    ///      *        Wait until at least this many bytes can be written.  This is only
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

    /// `void asyncWait (in nsIOutputStreamCallback aCallback, in unsigned long aFlags, in unsigned long aRequestedCount, in nsIEventTarget aEventTarget);`
    #[inline]
    pub unsafe fn AsyncWait(&self, aCallback: *const nsIOutputStreamCallback, aFlags: u32, aRequestedCount: u32, aEventTarget: *const nsIEventTarget) -> ::nserror::nsresult {
        ((*self.vtable).AsyncWait)(self, aCallback, aFlags, aRequestedCount, aEventTarget)
    }


}


/// `interface nsIOutputStreamCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIOutputStreamCallback {
    vtable: *const nsIOutputStreamCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIOutputStreamCallback.
unsafe impl XpCom for nsIOutputStreamCallback {
    const IID: nsIID = nsID(0x40dbcdff, 0x9053, 0x42c5,
        [0xa5, 0x7c, 0x3e, 0xc9, 0x10, 0xd0, 0xf1, 0x48]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIOutputStreamCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIOutputStreamCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIOutputStreamCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIOutputStreamCallback`.
    fn coerce_from(v: &nsIOutputStreamCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIOutputStreamCallbackCoerce for nsIOutputStreamCallback {
    #[inline]
    fn coerce_from(v: &nsIOutputStreamCallback) -> &Self {
        v
    }
}

impl nsIOutputStreamCallback {
    /// Cast this `nsIOutputStreamCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIOutputStreamCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIOutputStreamCallback {
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
impl<T: nsISupportsCoerce> nsIOutputStreamCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIOutputStreamCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIOutputStreamCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIOutputStreamCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onOutputStreamReady (in nsIAsyncOutputStream aStream); */
    pub OnOutputStreamReady: unsafe extern "system" fn (this: *const nsIOutputStreamCallback, aStream: *const nsIAsyncOutputStream) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIOutputStreamCallback {

    /// ```text
    /// /**
    ///  * This is a companion interface for nsIAsyncOutputStream::asyncWait.
    ///  */
    /// /**
    ///      * Called to indicate that the stream is either writable or closed.
    ///      *
    ///      * @param aStream
    ///      *        The stream whose asyncWait method was called.
    ///      */
    /// ```
    ///

    /// `void onOutputStreamReady (in nsIAsyncOutputStream aStream);`
    #[inline]
    pub unsafe fn OnOutputStreamReady(&self, aStream: *const nsIAsyncOutputStream) -> ::nserror::nsresult {
        ((*self.vtable).OnOutputStreamReady)(self, aStream)
    }


}


