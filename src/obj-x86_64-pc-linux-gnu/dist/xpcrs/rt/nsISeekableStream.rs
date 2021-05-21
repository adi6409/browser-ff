//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsISeekableStream.idl
//


/// `interface nsISeekableStream : nsITellableStream`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISeekableStream {
    vtable: *const nsISeekableStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISeekableStream.
unsafe impl XpCom for nsISeekableStream {
    const IID: nsIID = nsID(0x8429d350, 0x1040, 0x4661,
        [0x8b, 0x71, 0xf2, 0xa6, 0xba, 0x45, 0x59, 0x80]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISeekableStream {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISeekableStream.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISeekableStreamCoerce {
    /// Cheaply cast a value of this type from a `nsISeekableStream`.
    fn coerce_from(v: &nsISeekableStream) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISeekableStreamCoerce for nsISeekableStream {
    #[inline]
    fn coerce_from(v: &nsISeekableStream) -> &Self {
        v
    }
}

impl nsISeekableStream {
    /// Cast this `nsISeekableStream` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISeekableStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISeekableStream {
    type Target = nsITellableStream;
    #[inline]
    fn deref(&self) -> &nsITellableStream {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsITellableStreamCoerce> nsISeekableStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISeekableStream) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISeekableStream
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISeekableStreamVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsITellableStreamVTable,

    /* void seek (in long whence, in long long offset); */
    pub Seek: unsafe extern "system" fn (this: *const nsISeekableStream, whence: i32, offset: i64) -> ::nserror::nsresult,

    /* void setEOF (); */
    pub SetEOF: unsafe extern "system" fn (this: *const nsISeekableStream) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISeekableStream {

    pub const NS_SEEK_SET: i64 = 0;


    pub const NS_SEEK_CUR: i64 = 1;


    pub const NS_SEEK_END: i64 = 2;

    /// ```text
    /// /**
    ///      *  seek
    ///      *
    ///      *  This method moves the stream offset of the steam implementing this
    ///      *  interface.
    ///      *
    ///      *   @param whence specifies how to interpret the 'offset' parameter in
    ///      *                 setting the stream offset associated with the implementing
    ///      *                 stream.
    ///      *
    ///      *   @param offset specifies a value, in bytes, that is used in conjunction
    ///      *                 with the 'whence' parameter to set the stream offset of the
    ///      *                 implementing stream.  A negative value causes seeking in
    ///      *                 the reverse direction.
    ///      *
    ///      *   @throws NS_BASE_STREAM_CLOSED if called on a closed stream.
    ///      */
    /// ```
    ///

    /// `void seek (in long whence, in long long offset);`
    #[inline]
    pub unsafe fn Seek(&self, whence: i32, offset: i64) -> ::nserror::nsresult {
        ((*self.vtable).Seek)(self, whence, offset)
    }


    /// ```text
    /// /**
    ///      *  setEOF
    ///      *
    ///      *  This method truncates the stream at the current offset.
    ///      *
    ///      *   @throws NS_BASE_STREAM_CLOSED if called on a closed stream.
    ///      */
    /// ```
    ///

    /// `void setEOF ();`
    #[inline]
    pub unsafe fn SetEOF(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SetEOF)(self, )
    }


}


