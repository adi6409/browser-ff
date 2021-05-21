//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIStreamBufferAccess.idl
//


/// `interface nsIStreamBufferAccess : nsISupports`
///

/// ```text
/// /**
///  * An interface for access to a buffering stream implementation's underlying
///  * memory buffer.
///  *
///  * Stream implementations that QueryInterface to nsIStreamBufferAccess must
///  * ensure that all buffers are aligned on the most restrictive type size for
///  * the current architecture (e.g., sizeof(double) for RISCy CPUs).  malloc(3)
///  * satisfies this requirement.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIStreamBufferAccess {
    vtable: *const nsIStreamBufferAccessVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIStreamBufferAccess.
unsafe impl XpCom for nsIStreamBufferAccess {
    const IID: nsIID = nsID(0xac923b72, 0xac87, 0x4892,
        [0xac, 0x7a, 0xca, 0x38, 0x5d, 0x42, 0x94, 0x35]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIStreamBufferAccess {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIStreamBufferAccess.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIStreamBufferAccessCoerce {
    /// Cheaply cast a value of this type from a `nsIStreamBufferAccess`.
    fn coerce_from(v: &nsIStreamBufferAccess) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIStreamBufferAccessCoerce for nsIStreamBufferAccess {
    #[inline]
    fn coerce_from(v: &nsIStreamBufferAccess) -> &Self {
        v
    }
}

impl nsIStreamBufferAccess {
    /// Cast this `nsIStreamBufferAccess` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIStreamBufferAccessCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIStreamBufferAccess {
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
impl<T: nsISupportsCoerce> nsIStreamBufferAccessCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStreamBufferAccess) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIStreamBufferAccess
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIStreamBufferAccessVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [noscript,notxpcom] charPtr getBuffer (in uint32_t aLength, in uint32_t aAlignMask); */
    /// Unable to generate binding because `native type char unsupported`
    pub GetBuffer: *const ::libc::c_void,

    /* [noscript,notxpcom] void putBuffer (in charPtr aBuffer, in uint32_t aLength); */
    /// Unable to generate binding because `native type char unsupported`
    pub PutBuffer: *const ::libc::c_void,

    /* void disableBuffering (); */
    pub DisableBuffering: unsafe extern "system" fn (this: *const nsIStreamBufferAccess) -> ::nserror::nsresult,

    /* void enableBuffering (); */
    pub EnableBuffering: unsafe extern "system" fn (this: *const nsIStreamBufferAccess) -> ::nserror::nsresult,

    /* readonly attribute nsISupports unbufferedStream; */
    pub GetUnbufferedStream: unsafe extern "system" fn (this: *const nsIStreamBufferAccess, aUnbufferedStream: *mut *const nsISupports) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIStreamBufferAccess {

    /// ```text
    /// /**
    ///      * Get access to a contiguous, aligned run of bytes in the stream's buffer.
    ///      * Exactly one successful getBuffer call must occur before a putBuffer call
    ///      * taking the non-null pointer returned by the successful getBuffer.
    ///      *
    ///      * The run of bytes are the next bytes (modulo alignment padding) to read
    ///      * for an input stream, and the next bytes (modulo alignment padding) to
    ///      * store before (eventually) writing buffered data to an output stream.
    ///      * There can be space beyond this run of bytes in the buffer for further
    ///      * accesses before the fill or flush point is reached.
    ///      *
    ///      * @param aLength
    ///      *    Count of contiguous bytes requested at the address A that satisfies
    ///      *    (A & aAlignMask) == 0 in the buffer, starting from the current stream
    ///      *    position, mapped to a buffer address B.  The stream implementation
    ///      *    must pad from B to A by skipping bytes (if input stream) or storing
    ///      *    zero bytes (if output stream).
    ///      *
    ///      * @param aAlignMask
    ///      *    Bit-mask computed by subtracting 1 from the power-of-two alignment
    ///      *    modulus (e.g., 3 or sizeof(uint32_t)-1 for uint32_t alignment).
    ///      *
    ///      * @return
    ///      *    The aligned pointer to aLength bytes in the buffer, or null if the
    ///      *    buffer has no room for aLength bytes starting at the next address A
    ///      *    after the current position that satisfies (A & aAlignMask) == 0.
    ///      */
    /// ```
    ///

    /// `[noscript,notxpcom] charPtr getBuffer (in uint32_t aLength, in uint32_t aAlignMask);`
    const _GetBuffer: () = ();

    /// ```text
    /// /**
    ///      * Relinquish access to the stream's buffer, filling if at end of an input
    ///      * buffer, flushing if completing an output buffer.  After a getBuffer call
    ///      * that returns non-null, putBuffer must be called.
    ///      *
    ///      * @param aBuffer
    ///      *    A non-null pointer returned by getBuffer on the same stream buffer
    ///      *    access object.
    ///      *
    ///      * @param aLength
    ///      *    The same count of contiguous bytes passed to the getBuffer call that
    ///      *    returned aBuffer.
    ///      */
    /// ```
    ///

    /// `[noscript,notxpcom] void putBuffer (in charPtr aBuffer, in uint32_t aLength);`
    const _PutBuffer: () = ();

    /// ```text
    /// /**
    ///      * Disable and enable buffering on the stream implementing this interface.
    ///      * DisableBuffering flushes an output stream's buffer, and invalidates an
    ///      * input stream's buffer.
    ///      */
    /// ```
    ///

    /// `void disableBuffering ();`
    #[inline]
    pub unsafe fn DisableBuffering(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).DisableBuffering)(self, )
    }



    /// `void enableBuffering ();`
    #[inline]
    pub unsafe fn EnableBuffering(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).EnableBuffering)(self, )
    }


    /// ```text
    /// /**
    ///      * The underlying, unbuffered input or output stream.
    ///      */
    /// ```
    ///

    /// `readonly attribute nsISupports unbufferedStream;`
    #[inline]
    pub unsafe fn GetUnbufferedStream(&self, aUnbufferedStream: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).GetUnbufferedStream)(self, aUnbufferedStream)
    }


}


