//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIUnicharInputStream.idl
//


/// `interface nsIUnicharInputStream : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUnicharInputStream {
    vtable: *const nsIUnicharInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUnicharInputStream.
unsafe impl XpCom for nsIUnicharInputStream {
    const IID: nsIID = nsID(0xd5e3bd80, 0x6723, 0x4b92,
        [0xb0, 0xc9, 0x22, 0xf6, 0x16, 0x2f, 0xd9, 0x4f]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUnicharInputStream {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUnicharInputStream.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUnicharInputStreamCoerce {
    /// Cheaply cast a value of this type from a `nsIUnicharInputStream`.
    fn coerce_from(v: &nsIUnicharInputStream) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUnicharInputStreamCoerce for nsIUnicharInputStream {
    #[inline]
    fn coerce_from(v: &nsIUnicharInputStream) -> &Self {
        v
    }
}

impl nsIUnicharInputStream {
    /// Cast this `nsIUnicharInputStream` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUnicharInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUnicharInputStream {
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
impl<T: nsISupportsCoerce> nsIUnicharInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUnicharInputStream) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUnicharInputStream
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUnicharInputStreamVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [noscript] unsigned long read ([array, size_is (aCount)] in char16_t aBuf, in unsigned long aCount); */
    pub Read: unsafe extern "system" fn (this: *const nsIUnicharInputStream, aBuf: *mut char16_t, aCount: u32, _retval: *mut u32) -> ::nserror::nsresult,

    /* [noscript] unsigned long readSegments (in nsWriteUnicharSegmentFun aWriter, in voidPtr aClosure, in unsigned long aCount); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub ReadSegments: *const ::libc::c_void,

    /* unsigned long readString (in unsigned long aCount, out AString aString); */
    pub ReadString: unsafe extern "system" fn (this: *const nsIUnicharInputStream, aCount: u32, aString: *mut ::nsstring::nsAString, _retval: *mut u32) -> ::nserror::nsresult,

    /* void close (); */
    pub Close: unsafe extern "system" fn (this: *const nsIUnicharInputStream) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUnicharInputStream {

    /// ```text
    /// /**
    ///  * Abstract unicode character input stream
    ///  * @see nsIInputStream
    ///  */
    /// /**
    ///    * Reads into a caller-provided character array.
    ///    *
    ///    * @return The number of characters that were successfully read. May be less
    ///    *         than aCount, even if there is more data in the input stream.
    ///    *         A return value of 0 means EOF.
    ///    *
    ///    * @note To read more than 2^32 characters, call this method multiple times.
    ///    */
    /// ```
    ///

    /// `[noscript] unsigned long read ([array, size_is (aCount)] in char16_t aBuf, in unsigned long aCount);`
    #[inline]
    pub unsafe fn Read(&self, aBuf: *mut char16_t, aCount: u32, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).Read)(self, aBuf, aCount, _retval)
    }


    /// ```text
    /// /**
    ///    * Low-level read method that has access to the stream's underlying buffer.
    ///    * The writer function may be called multiple times for segmented buffers.
    ///    * ReadSegments is expected to keep calling the writer until either there is
    ///    * nothing left to read or the writer returns an error.  ReadSegments should
    ///    * not call the writer with zero characters to consume.
    ///    *
    ///    * @param aWriter the "consumer" of the data to be read
    ///    * @param aClosure opaque parameter passed to writer
    ///    * @param aCount the maximum number of characters to be read
    ///    *
    ///    * @return number of characters read (may be less than aCount)
    ///    * @return 0 if reached end of file (or if aWriter refused to consume data)
    ///    *
    ///    * @throws NS_BASE_STREAM_WOULD_BLOCK if reading from the input stream would
    ///    *   block the calling thread (non-blocking mode only)
    ///    * @throws <other-error> on failure
    ///    *
    ///    * NOTE: this function may be unimplemented if a stream has no underlying
    ///    * buffer
    ///    */
    /// ```
    ///

    /// `[noscript] unsigned long readSegments (in nsWriteUnicharSegmentFun aWriter, in voidPtr aClosure, in unsigned long aCount);`
    const _ReadSegments: () = ();

    /// ```text
    /// /**
    ///    * Read into a string object.
    ///    * @param aCount The number of characters that should be read
    ///    * @return The number of characters that were read.
    ///    */
    /// ```
    ///

    /// `unsigned long readString (in unsigned long aCount, out AString aString);`
    #[inline]
    pub unsafe fn ReadString(&self, aCount: u32, aString: *mut ::nsstring::nsAString, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).ReadString)(self, aCount, aString, _retval)
    }


    /// ```text
    /// /**
    ///   * Close the stream and free associated resources. This also closes the
    ///   * underlying stream, if any.
    ///   */
    /// ```
    ///

    /// `void close ();`
    #[inline]
    pub unsafe fn Close(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Close)(self, )
    }


}


