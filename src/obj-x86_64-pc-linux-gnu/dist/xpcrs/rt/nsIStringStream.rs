//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIStringStream.idl
//


/// `interface nsIStringInputStream : nsIInputStream`
///

/// ```text
/// /**
///  * nsIStringInputStream
///  *
///  * Provides scriptable and specialized C++-only methods for initializing a
///  * nsIInputStream implementation with a simple character array.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIStringInputStream {
    vtable: *const nsIStringInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIStringInputStream.
unsafe impl XpCom for nsIStringInputStream {
    const IID: nsIID = nsID(0x450cd2d4, 0xf0fd, 0x424d,
        [0xb3, 0x65, 0xb1, 0x25, 0x1f, 0x80, 0xfd, 0x53]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIStringInputStream {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIStringInputStream.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIStringInputStreamCoerce {
    /// Cheaply cast a value of this type from a `nsIStringInputStream`.
    fn coerce_from(v: &nsIStringInputStream) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIStringInputStreamCoerce for nsIStringInputStream {
    #[inline]
    fn coerce_from(v: &nsIStringInputStream) -> &Self {
        v
    }
}

impl nsIStringInputStream {
    /// Cast this `nsIStringInputStream` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIStringInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIStringInputStream {
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
impl<T: nsIInputStreamCoerce> nsIStringInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStringInputStream) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIStringInputStream
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIStringInputStreamVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIInputStreamVTable,

    /* void setData (in string data, in long dataLen); */
    pub SetData: unsafe extern "system" fn (this: *const nsIStringInputStream, data: *const libc::c_char, dataLen: i32) -> ::nserror::nsresult,

    /* void setUTF8Data (in AUTF8String data); */
    pub SetUTF8Data: unsafe extern "system" fn (this: *const nsIStringInputStream, data: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [noscript] void adoptData (in charPtr data, in long dataLen); */
    /// Unable to generate binding because `native type char unsupported`
    pub AdoptData: *const ::libc::c_void,

    /* [noscript] void shareData (in string data, in long dataLen); */
    pub ShareData: unsafe extern "system" fn (this: *const nsIStringInputStream, data: *const libc::c_char, dataLen: i32) -> ::nserror::nsresult,

    /* [noscript,notxpcom] size_t SizeOfIncludingThisIfUnshared (in MallocSizeOf aMallocSizeOf); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub SizeOfIncludingThisIfUnshared: *const ::libc::c_void,

    /* [noscript,notxpcom] size_t SizeOfIncludingThisEvenIfShared (in MallocSizeOf aMallocSizeOf); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub SizeOfIncludingThisEvenIfShared: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIStringInputStream {

    /// ```text
    /// /**
    ///      * SetData - assign data to the input stream (copied on assignment).
    ///      *
    ///      * @param data    - stream data
    ///      * @param dataLen - stream data length (-1 if length should be computed)
    ///      *
    ///      * NOTE: C++ code should consider using AdoptData or ShareData to avoid
    ///      * making an extra copy of the stream data.
    ///      *
    ///      * NOTE: For JS callers, the given data must not contain null characters
    ///      * (other than a null terminator) because a null character in the middle of
    ///      * the data string will be seen as a terminator when the data is converted
    ///      * from a JS string to a C++ character array.
    ///      */
    /// ```
    ///

    /// `void setData (in string data, in long dataLen);`
    #[inline]
    pub unsafe fn SetData(&self, data: *const libc::c_char, dataLen: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetData)(self, data, dataLen)
    }


    /// ```text
    /// /**
    ///      * SetUTF8Data - encode input data to UTF-8 and assign it to the input
    ///      * stream.
    ///      *
    ///      * @param data    - stream data
    ///      *
    ///      * NOTE: This method is meant to be used by JS callers,
    ///      */
    /// ```
    ///

    /// `void setUTF8Data (in AUTF8String data);`
    #[inline]
    pub unsafe fn SetUTF8Data(&self, data: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetUTF8Data)(self, data)
    }


    /// ```text
    /// /**
    ///      * NOTE: the following methods are designed to give C++ code added control
    ///      * over the ownership and lifetime of the stream data.  Use with care :-)
///      */
/// /**
///      * AdoptData - assign data to the input stream.  the input stream takes
///      * ownership of the given data buffer and will free it when
///      * the input stream is destroyed.
///      *
///      * @param data      - stream data
///      * @param dataLen   - stream data length (-1 if length should be computed)
///      */
/// ```
///

/// `[noscript] void adoptData (in charPtr data, in long dataLen);`
const _AdoptData: () = ();

/// ```text
/// /**
///      * ShareData - assign data to the input stream.  the input stream references
///      * the given data buffer until the input stream is destroyed.  the given
///      * data buffer must outlive the input stream.
///      *
///      * @param data      - stream data
///      * @param dataLen   - stream data length (-1 if length should be computed)
///      */
/// ```
///

/// `[noscript] void shareData (in string data, in long dataLen);`
#[inline]
pub unsafe fn ShareData(&self, data: *const libc::c_char, dataLen: i32) -> ::nserror::nsresult {
    ((*self.vtable).ShareData)(self, data, dataLen)
}



/// `[noscript,notxpcom] size_t SizeOfIncludingThisIfUnshared (in MallocSizeOf aMallocSizeOf);`
const _SizeOfIncludingThisIfUnshared: () = ();


/// `[noscript,notxpcom] size_t SizeOfIncludingThisEvenIfShared (in MallocSizeOf aMallocSizeOf);`
const _SizeOfIncludingThisEvenIfShared: () = ();

}


