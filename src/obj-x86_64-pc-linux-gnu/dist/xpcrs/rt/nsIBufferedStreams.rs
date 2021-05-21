//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIBufferedStreams.idl
//


/// `interface nsIBufferedInputStream : nsIInputStream`
///

/// ```text
/// /**
///  * An input stream that reads ahead and keeps a buffer coming from another input
///  * stream so that fewer accesses to the underlying stream are necessary.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIBufferedInputStream {
    vtable: *const nsIBufferedInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIBufferedInputStream.
unsafe impl XpCom for nsIBufferedInputStream {
    const IID: nsIID = nsID(0x616f5b48, 0xda09, 0x11d3,
        [0x8c, 0xda, 0x00, 0x60, 0xb0, 0xfc, 0x14, 0xa3]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIBufferedInputStream {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIBufferedInputStream.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIBufferedInputStreamCoerce {
    /// Cheaply cast a value of this type from a `nsIBufferedInputStream`.
    fn coerce_from(v: &nsIBufferedInputStream) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIBufferedInputStreamCoerce for nsIBufferedInputStream {
    #[inline]
    fn coerce_from(v: &nsIBufferedInputStream) -> &Self {
        v
    }
}

impl nsIBufferedInputStream {
    /// Cast this `nsIBufferedInputStream` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIBufferedInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIBufferedInputStream {
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
impl<T: nsIInputStreamCoerce> nsIBufferedInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBufferedInputStream) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIBufferedInputStream
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIBufferedInputStreamVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIInputStreamVTable,

    /* void init (in nsIInputStream fillFromStream, in unsigned long bufferSize); */
    pub Init: unsafe extern "system" fn (this: *const nsIBufferedInputStream, fillFromStream: *const nsIInputStream, bufferSize: u32) -> ::nserror::nsresult,

    /* readonly attribute nsIInputStream data; */
    pub GetData: unsafe extern "system" fn (this: *const nsIBufferedInputStream, aData: *mut *const nsIInputStream) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIBufferedInputStream {

    /// ```text
    /// /**
    ///      * @param fillFromStream - add buffering to this stream
    ///      * @param bufferSize     - specifies the maximum buffer size
    ///      */
    /// ```
    ///

    /// `void init (in nsIInputStream fillFromStream, in unsigned long bufferSize);`
    #[inline]
    pub unsafe fn Init(&self, fillFromStream: *const nsIInputStream, bufferSize: u32) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, fillFromStream, bufferSize)
    }


    /// ```text
    /// /**
    ///      * Get the wrapped data stream
    ///      */
    /// ```
    ///

    /// `readonly attribute nsIInputStream data;`
    #[inline]
    pub unsafe fn GetData(&self, aData: *mut *const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).GetData)(self, aData)
    }


}


/// `interface nsIBufferedOutputStream : nsIOutputStream`
///

/// ```text
/// /**
///  * An output stream that stores up data to write out to another output stream
///  * and does the entire write only when the buffer is full, so that fewer writes
///  * to the underlying output stream are necessary.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIBufferedOutputStream {
    vtable: *const nsIBufferedOutputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIBufferedOutputStream.
unsafe impl XpCom for nsIBufferedOutputStream {
    const IID: nsIID = nsID(0x6476378a, 0xda09, 0x11d3,
        [0x8c, 0xda, 0x00, 0x60, 0xb0, 0xfc, 0x14, 0xa3]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIBufferedOutputStream {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIBufferedOutputStream.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIBufferedOutputStreamCoerce {
    /// Cheaply cast a value of this type from a `nsIBufferedOutputStream`.
    fn coerce_from(v: &nsIBufferedOutputStream) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIBufferedOutputStreamCoerce for nsIBufferedOutputStream {
    #[inline]
    fn coerce_from(v: &nsIBufferedOutputStream) -> &Self {
        v
    }
}

impl nsIBufferedOutputStream {
    /// Cast this `nsIBufferedOutputStream` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIBufferedOutputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIBufferedOutputStream {
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
impl<T: nsIOutputStreamCoerce> nsIBufferedOutputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBufferedOutputStream) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIBufferedOutputStream
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIBufferedOutputStreamVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIOutputStreamVTable,

    /* void init (in nsIOutputStream sinkToStream, in unsigned long bufferSize); */
    pub Init: unsafe extern "system" fn (this: *const nsIBufferedOutputStream, sinkToStream: *const nsIOutputStream, bufferSize: u32) -> ::nserror::nsresult,

    /* readonly attribute nsIOutputStream data; */
    pub GetData: unsafe extern "system" fn (this: *const nsIBufferedOutputStream, aData: *mut *const nsIOutputStream) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIBufferedOutputStream {

    /// ```text
    /// /**
    ///      * @param sinkToStream - add buffering to this stream
    ///      * @param bufferSize   - specifies the maximum buffer size
    ///      */
    /// ```
    ///

    /// `void init (in nsIOutputStream sinkToStream, in unsigned long bufferSize);`
    #[inline]
    pub unsafe fn Init(&self, sinkToStream: *const nsIOutputStream, bufferSize: u32) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, sinkToStream, bufferSize)
    }


    /// ```text
    /// /**
    ///      * Get the wrapped data stream
    ///      */
    /// ```
    ///

    /// `readonly attribute nsIOutputStream data;`
    #[inline]
    pub unsafe fn GetData(&self, aData: *mut *const nsIOutputStream) -> ::nserror::nsresult {
        ((*self.vtable).GetData)(self, aData)
    }


}


