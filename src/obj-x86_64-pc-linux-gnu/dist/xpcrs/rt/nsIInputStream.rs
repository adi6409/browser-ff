//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIInputStream.idl
//


/// `interface nsIInputStream : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIInputStream {
    vtable: *const nsIInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIInputStream.
unsafe impl XpCom for nsIInputStream {
    const IID: nsIID = nsID(0x53cdbc97, 0xc2d7, 0x4e30,
        [0xb2, 0xc3, 0x45, 0xb2, 0xee, 0x79, 0xdb, 0x18]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIInputStream {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIInputStream.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIInputStreamCoerce {
    /// Cheaply cast a value of this type from a `nsIInputStream`.
    fn coerce_from(v: &nsIInputStream) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIInputStreamCoerce for nsIInputStream {
    #[inline]
    fn coerce_from(v: &nsIInputStream) -> &Self {
        v
    }
}

impl nsIInputStream {
    /// Cast this `nsIInputStream` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIInputStream {
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
impl<T: nsISupportsCoerce> nsIInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIInputStream) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIInputStream
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIInputStreamVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void close (); */
    pub Close: unsafe extern "system" fn (this: *const nsIInputStream) -> ::nserror::nsresult,

    /* unsigned long long available (); */
    pub Available: unsafe extern "system" fn (this: *const nsIInputStream, _retval: *mut u64) -> ::nserror::nsresult,

    /* [noscript] unsigned long read (in charPtr aBuf, in unsigned long aCount); */
    /// Unable to generate binding because `native type char unsupported`
    pub Read: *const ::libc::c_void,

    /* [noscript] unsigned long readSegments (in nsWriteSegmentFun aWriter, in voidPtr aClosure, in unsigned long aCount); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub ReadSegments: *const ::libc::c_void,

    /* boolean isNonBlocking (); */
    pub IsNonBlocking: unsafe extern "system" fn (this: *const nsIInputStream, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIInputStream {

    /// ```text
    /// /**
    ///  * nsIInputStream
    ///  *
    ///  * An interface describing a readable stream of data.  An input stream may be
    ///  * "blocking" or "non-blocking" (see the IsNonBlocking method).  A blocking
    ///  * input stream may suspend the calling thread in order to satisfy a call to
    ///  * Close, Available, Read, or ReadSegments.  A non-blocking input stream, on
    ///  * the other hand, must not block the calling thread of execution.
    ///  *
    ///  * NOTE: blocking input streams are often read on a background thread to avoid
    ///  * locking up the main application thread.  For this reason, it is generally
    ///  * the case that a blocking input stream should be implemented using thread-
    ///  * safe AddRef and Release.
    ///  */
    /// /**
    ///      * Close the stream.  This method causes subsequent calls to Read and
    ///      * ReadSegments to return 0 bytes read to indicate end-of-file.  Any
    ///      * subsequent calls to Available should throw NS_BASE_STREAM_CLOSED.
    ///      */
    /// ```
    ///

    /// `void close ();`
    #[inline]
    pub unsafe fn Close(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Close)(self, )
    }


    /// ```text
    /// /**
    ///      * Determine number of bytes available in the stream.  A non-blocking
    ///      * stream that does not yet have any data to read should return 0 bytes
    ///      * from this method (i.e., it must not throw the NS_BASE_STREAM_WOULD_BLOCK
        ///      * exception).
    ///      *
    ///      * In addition to the number of bytes available in the stream, this method
    ///      * also informs the caller of the current status of the stream.  A stream
    ///      * that is closed will throw an exception when this method is called.  That
    ///      * enables the caller to know the condition of the stream before attempting
    ///      * to read from it.  If a stream is at end-of-file, but not closed, then
    ///      * this method returns 0 bytes available.  (Note: some nsIInputStream
        ///      * implementations automatically close when eof is reached; some do not).
    ///      *
    ///      * @return number of bytes currently available in the stream.
    ///      *
    ///      * @throws NS_BASE_STREAM_CLOSED if the stream is closed normally.
    ///      * @throws <other-error> if the stream is closed due to some error
    ///      *   condition
    ///      */
    /// ```
    ///

    /// `unsigned long long available ();`
    #[inline]
    pub unsafe fn Available(&self, _retval: *mut u64) -> ::nserror::nsresult {
        ((*self.vtable).Available)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * Read data from the stream.
    ///      *
    ///      * @param aBuf the buffer into which the data is to be read
    ///      * @param aCount the maximum number of bytes to be read
    ///      *
    ///      * @return number of bytes read (may be less than aCount).
    ///      * @return 0 if reached end-of-file
    ///      *
    ///      * @throws NS_BASE_STREAM_WOULD_BLOCK if reading from the input stream would
    ///      *   block the calling thread (non-blocking mode only)
    ///      * @throws <other-error> on failure
    ///      *
    ///      * NOTE: this method should not throw NS_BASE_STREAM_CLOSED.
    ///      */
    /// ```
    ///

    /// `[noscript] unsigned long read (in charPtr aBuf, in unsigned long aCount);`
    const _Read: () = ();

    /// ```text
    /// /**
    ///      * Low-level read method that provides access to the stream's underlying
    ///      * buffer.  The writer function may be called multiple times for segmented
    ///      * buffers.  ReadSegments is expected to keep calling the writer until
    ///      * either there is nothing left to read or the writer returns an error.
    ///      * ReadSegments should not call the writer with zero bytes to consume.
    ///      *
    ///      * @param aWriter the "consumer" of the data to be read
    ///      * @param aClosure opaque parameter passed to writer
    ///      * @param aCount the maximum number of bytes to be read
    ///      *
    ///      * @return number of bytes read (may be less than aCount)
    ///      * @return 0 if reached end-of-file (or if aWriter refused to consume data)
    ///      *
    ///      * @throws NS_BASE_STREAM_WOULD_BLOCK if reading from the input stream would
    ///      *   block the calling thread (non-blocking mode only)
    ///      * @throws NS_ERROR_NOT_IMPLEMENTED if the stream has no underlying buffer
    ///      * @throws <other-error> on failure
    ///      *
    ///      * NOTE: this function may be unimplemented if a stream has no underlying
    ///      * buffer (e.g., socket input stream).
    ///      *
    ///      * NOTE: this method should not throw NS_BASE_STREAM_CLOSED.
    ///      */
    /// ```
    ///

    /// `[noscript] unsigned long readSegments (in nsWriteSegmentFun aWriter, in voidPtr aClosure, in unsigned long aCount);`
    const _ReadSegments: () = ();

    /// ```text
    /// /**
    ///      * @return true if stream is non-blocking
    ///      *
    ///      * NOTE: reading from a blocking input stream will block the calling thread
    ///      * until at least one byte of data can be extracted from the stream.
    ///      *
    ///      * NOTE: a non-blocking input stream may implement nsIAsyncInputStream to
    ///      * provide consumers with a way to wait for the stream to have more data
    ///      * once its read method is unable to return any data without blocking.
    ///      */
    /// ```
    ///

    /// `boolean isNonBlocking ();`
    #[inline]
    pub unsafe fn IsNonBlocking(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsNonBlocking)(self, _retval)
    }


}


