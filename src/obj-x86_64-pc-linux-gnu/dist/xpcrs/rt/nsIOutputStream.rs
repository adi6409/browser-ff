//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIOutputStream.idl
//


/// `interface nsIOutputStream : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIOutputStream {
    vtable: *const nsIOutputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIOutputStream.
unsafe impl XpCom for nsIOutputStream {
    const IID: nsIID = nsID(0x0d0acd2a, 0x61b4, 0x11d4,
        [0x98, 0x77, 0x00, 0xc0, 0x4f, 0xa0, 0xcf, 0x4a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIOutputStream {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIOutputStream.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIOutputStreamCoerce {
    /// Cheaply cast a value of this type from a `nsIOutputStream`.
    fn coerce_from(v: &nsIOutputStream) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIOutputStreamCoerce for nsIOutputStream {
    #[inline]
    fn coerce_from(v: &nsIOutputStream) -> &Self {
        v
    }
}

impl nsIOutputStream {
    /// Cast this `nsIOutputStream` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIOutputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIOutputStream {
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
impl<T: nsISupportsCoerce> nsIOutputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIOutputStream) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIOutputStream
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIOutputStreamVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void close (); */
    pub Close: unsafe extern "system" fn (this: *const nsIOutputStream) -> ::nserror::nsresult,

    /* void flush (); */
    pub Flush: unsafe extern "system" fn (this: *const nsIOutputStream) -> ::nserror::nsresult,

    /* unsigned long write (in string aBuf, in unsigned long aCount); */
    pub Write: unsafe extern "system" fn (this: *const nsIOutputStream, aBuf: *const libc::c_char, aCount: u32, _retval: *mut u32) -> ::nserror::nsresult,

    /* unsigned long writeFrom (in nsIInputStream aFromStream, in unsigned long aCount); */
    pub WriteFrom: unsafe extern "system" fn (this: *const nsIOutputStream, aFromStream: *const nsIInputStream, aCount: u32, _retval: *mut u32) -> ::nserror::nsresult,

    /* [noscript] unsigned long writeSegments (in nsReadSegmentFun aReader, in voidPtr aClosure, in unsigned long aCount); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub WriteSegments: *const ::libc::c_void,

    /* boolean isNonBlocking (); */
    pub IsNonBlocking: unsafe extern "system" fn (this: *const nsIOutputStream, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIOutputStream {

    /// ```text
    /// /**
    ///  * nsIOutputStream
    ///  *
    ///  * An interface describing a writable stream of data.  An output stream may be
    ///  * "blocking" or "non-blocking" (see the IsNonBlocking method).  A blocking
    ///  * output stream may suspend the calling thread in order to satisfy a call to
    ///  * Close, Flush, Write, WriteFrom, or WriteSegments.  A non-blocking output
    ///  * stream, on the other hand, must not block the calling thread of execution.
    ///  *
    ///  * NOTE: blocking output streams are often written to on a background thread to
    ///  * avoid locking up the main application thread.  For this reason, it is
    ///  * generally the case that a blocking output stream should be implemented using
    ///  * thread- safe AddRef and Release.
    ///  */
    /// /**
    ///      * Close the stream. Forces the output stream to flush any buffered data.
    ///      *
    ///      * @throws NS_BASE_STREAM_WOULD_BLOCK if unable to flush without blocking
    ///      *   the calling thread (non-blocking mode only)
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
    ///      * Flush the stream.
    ///      *
    ///      * @throws NS_BASE_STREAM_WOULD_BLOCK if unable to flush without blocking
    ///      *   the calling thread (non-blocking mode only)
    ///      */
    /// ```
    ///

    /// `void flush ();`
    #[inline]
    pub unsafe fn Flush(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Flush)(self, )
    }


    /// ```text
    /// /**
    ///      * Write data into the stream.
    ///      *
    ///      * @param aBuf the buffer containing the data to be written
    ///      * @param aCount the maximum number of bytes to be written
    ///      *
    ///      * @return number of bytes written (may be less than aCount)
    ///      *
    ///      * @throws NS_BASE_STREAM_WOULD_BLOCK if writing to the output stream would
    ///      *   block the calling thread (non-blocking mode only)
    ///      * @throws <other-error> on failure
    ///      */
    /// ```
    ///

    /// `unsigned long write (in string aBuf, in unsigned long aCount);`
    #[inline]
    pub unsafe fn Write(&self, aBuf: *const libc::c_char, aCount: u32, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).Write)(self, aBuf, aCount, _retval)
    }


    /// ```text
    /// /**
    ///      * Writes data into the stream from an input stream.
    ///      *
    ///      * @param aFromStream the stream containing the data to be written
    ///      * @param aCount the maximum number of bytes to be written
    ///      *
    ///      * @return number of bytes written (may be less than aCount)
    ///      *
    ///      * @throws NS_BASE_STREAM_WOULD_BLOCK if writing to the output stream would
    ///      *    block the calling thread (non-blocking mode only). This failure
    ///      *    means no bytes were transferred.
    ///      * @throws <other-error> on failure
    ///      *
    ///      * NOTE: This method is defined by this interface in order to allow the
    ///      * output stream to efficiently copy the data from the input stream into
    ///      * its internal buffer (if any). If this method was provided as an external
    ///      * facility, a separate char* buffer would need to be used in order to call
    ///      * the output stream's other Write method.
    ///      */
    /// ```
    ///

    /// `unsigned long writeFrom (in nsIInputStream aFromStream, in unsigned long aCount);`
    #[inline]
    pub unsafe fn WriteFrom(&self, aFromStream: *const nsIInputStream, aCount: u32, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).WriteFrom)(self, aFromStream, aCount, _retval)
    }


    /// ```text
    /// /**
    ///      * Low-level write method that has access to the stream's underlying buffer.
    ///      * The reader function may be called multiple times for segmented buffers.
    ///      * WriteSegments is expected to keep calling the reader until either there
    ///      * is nothing left to write or the reader returns an error.  WriteSegments
    ///      * should not call the reader with zero bytes to provide.
    ///      *
    ///      * @param aReader the "provider" of the data to be written
    ///      * @param aClosure opaque parameter passed to reader
    ///      * @param aCount the maximum number of bytes to be written
    ///      *
    ///      * @return number of bytes written (may be less than aCount)
    ///      *
    ///      * @throws NS_BASE_STREAM_WOULD_BLOCK if writing to the output stream would
    ///      *    block the calling thread (non-blocking mode only). This failure
    ///      *    means no bytes were transferred.
    ///      * @throws NS_ERROR_NOT_IMPLEMENTED if the stream has no underlying buffer
    ///      * @throws <other-error> on failure
    ///      *
    ///      * NOTE: this function may be unimplemented if a stream has no underlying
    ///      * buffer (e.g., socket output stream).
    ///      */
    /// ```
    ///

    /// `[noscript] unsigned long writeSegments (in nsReadSegmentFun aReader, in voidPtr aClosure, in unsigned long aCount);`
    const _WriteSegments: () = ();

    /// ```text
    /// /**
    ///      * @return true if stream is non-blocking
    ///      *
    ///      * NOTE: writing to a blocking output stream will block the calling thread
    ///      * until all given data can be consumed by the stream.
    ///      *
    ///      * NOTE: a non-blocking output stream may implement nsIAsyncOutputStream to
    ///      * provide consumers with a way to wait for the stream to accept more data
    ///      * once its write method is unable to accept any data without blocking.
    ///      */
    /// ```
    ///

    /// `boolean isNonBlocking ();`
    #[inline]
    pub unsafe fn IsNonBlocking(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsNonBlocking)(self, _retval)
    }


}


