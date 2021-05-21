//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIStorageStream.idl
//


/// `interface nsIStorageStream : nsISupports`
///

/// ```text
/// /**
///  * The nsIStorageStream interface maintains an internal data buffer that can be
///  * filled using a single output stream.  One or more independent input streams
///  * can be created to read the data from the buffer non-destructively.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIStorageStream {
    vtable: *const nsIStorageStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIStorageStream.
unsafe impl XpCom for nsIStorageStream {
    const IID: nsIID = nsID(0x44a200fe, 0x6c2b, 0x4b41,
        [0xb4, 0xe3, 0x63, 0xe8, 0xc1, 0x4e, 0x7c, 0x0d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIStorageStream {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIStorageStream.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIStorageStreamCoerce {
    /// Cheaply cast a value of this type from a `nsIStorageStream`.
    fn coerce_from(v: &nsIStorageStream) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIStorageStreamCoerce for nsIStorageStream {
    #[inline]
    fn coerce_from(v: &nsIStorageStream) -> &Self {
        v
    }
}

impl nsIStorageStream {
    /// Cast this `nsIStorageStream` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIStorageStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIStorageStream {
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
impl<T: nsISupportsCoerce> nsIStorageStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStorageStream) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIStorageStream
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIStorageStreamVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void init (in uint32_t segmentSize, in uint32_t maxSize); */
    pub Init: unsafe extern "system" fn (this: *const nsIStorageStream, segmentSize: uint32_t, maxSize: uint32_t) -> ::nserror::nsresult,

    /* nsIOutputStream getOutputStream (in int32_t startPosition); */
    pub GetOutputStream: unsafe extern "system" fn (this: *const nsIStorageStream, startPosition: int32_t, _retval: *mut*const nsIOutputStream) -> ::nserror::nsresult,

    /* nsIInputStream newInputStream (in int32_t startPosition); */
    pub NewInputStream: unsafe extern "system" fn (this: *const nsIStorageStream, startPosition: int32_t, _retval: *mut*const nsIInputStream) -> ::nserror::nsresult,

    /* attribute uint32_t length; */
    pub GetLength: unsafe extern "system" fn (this: *const nsIStorageStream, aLength: *mut uint32_t) -> ::nserror::nsresult,

    /* attribute uint32_t length; */
    pub SetLength: unsafe extern "system" fn (this: *const nsIStorageStream, aLength: uint32_t) -> ::nserror::nsresult,

    /* readonly attribute boolean writeInProgress; */
    pub GetWriteInProgress: unsafe extern "system" fn (this: *const nsIStorageStream, aWriteInProgress: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIStorageStream {

    /// ```text
    /// /**
    ///      *
    ///      * Initialize the stream, setting up the amount of space that will be
    ///      * allocated for the stream's backing-store.
    ///      *
    ///      * @param segmentSize
    ///      *        Size of each segment. Must be a power of two.
    ///      * @param maxSize
    ///      *        Maximum total size of this stream. length will always be less
    ///      *        than or equal to this value. Passing UINT32_MAX is safe.
    ///      */
    /// ```
    ///

    /// `void init (in uint32_t segmentSize, in uint32_t maxSize);`
    #[inline]
    pub unsafe fn Init(&self, segmentSize: uint32_t, maxSize: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, segmentSize, maxSize)
    }


    /// ```text
    /// /**
    ///      * Get a reference to the one and only output stream for this instance.
    ///      * The zero-based startPosition argument is used is used to set the initial
    ///      * write cursor position.  The startPosition cannot be set larger than the
    ///      * current buffer length.  Calling this method has the side-effect of
    ///      * truncating the internal buffer to startPosition bytes.
    ///      */
    /// ```
    ///

    /// `nsIOutputStream getOutputStream (in int32_t startPosition);`
    #[inline]
    pub unsafe fn GetOutputStream(&self, startPosition: int32_t, _retval: *mut*const nsIOutputStream) -> ::nserror::nsresult {
        ((*self.vtable).GetOutputStream)(self, startPosition, _retval)
    }


    /// ```text
    /// /**
    ///      * Create a new input stream to read data (written by the singleton output
        ///      * stream) from the internal buffer.  Multiple, independent input streams
    ///      * can be created.
    ///      */
    /// ```
    ///

    /// `nsIInputStream newInputStream (in int32_t startPosition);`
    #[inline]
    pub unsafe fn NewInputStream(&self, startPosition: int32_t, _retval: *mut*const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).NewInputStream)(self, startPosition, _retval)
    }


    /// ```text
    /// /**
    ///      * The length attribute indicates the total number of bytes stored in the
    ///      * nsIStorageStream internal buffer, regardless of any consumption by input
    ///      * streams.  Assigning to the length field can be used to truncate the
    ///      * buffer data, but can not be used when either the instance's output
    ///      * stream is in use.
    ///      *
    ///      * @See #writeInProgress */
    /// ```
    ///

    /// `attribute uint32_t length;`
    #[inline]
    pub unsafe fn GetLength(&self, aLength: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetLength)(self, aLength)
    }


    /// ```text
    /// /**
    ///      * The length attribute indicates the total number of bytes stored in the
    ///      * nsIStorageStream internal buffer, regardless of any consumption by input
    ///      * streams.  Assigning to the length field can be used to truncate the
    ///      * buffer data, but can not be used when either the instance's output
    ///      * stream is in use.
    ///      *
    ///      * @See #writeInProgress */
    /// ```
    ///

    /// `attribute uint32_t length;`
    #[inline]
    pub unsafe fn SetLength(&self, aLength: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).SetLength)(self, aLength)
    }


    /// ```text
    /// /**
    ///      * True, when output stream has not yet been Close'ed
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean writeInProgress;`
    #[inline]
    pub unsafe fn GetWriteInProgress(&self, aWriteInProgress: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetWriteInProgress)(self, aWriteInProgress)
    }


}


