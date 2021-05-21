//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIBinaryInputStream.idl
//


/// `interface nsIBinaryInputStream : nsIInputStream`
///

/// ```text
/// /**
///  * This interface allows consumption of primitive data types from a "binary
///  * stream" containing untagged, big-endian binary data, i.e. as produced by an
///  * implementation of nsIBinaryOutputStream.  This might be used, for example,
///  * to implement network protocols or to read from architecture-neutral disk
///  * files, i.e. ones that can be read and written by both big-endian and
///  * little-endian platforms.
///  *
///  * @See nsIBinaryOutputStream
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIBinaryInputStream {
    vtable: *const nsIBinaryInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIBinaryInputStream.
unsafe impl XpCom for nsIBinaryInputStream {
    const IID: nsIID = nsID(0x899b826b, 0x2eb3, 0x469c,
        [0x8b, 0x31, 0x4c, 0x29, 0xf5, 0xd3, 0x41, 0xa6]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIBinaryInputStream {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIBinaryInputStream.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIBinaryInputStreamCoerce {
    /// Cheaply cast a value of this type from a `nsIBinaryInputStream`.
    fn coerce_from(v: &nsIBinaryInputStream) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIBinaryInputStreamCoerce for nsIBinaryInputStream {
    #[inline]
    fn coerce_from(v: &nsIBinaryInputStream) -> &Self {
        v
    }
}

impl nsIBinaryInputStream {
    /// Cast this `nsIBinaryInputStream` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIBinaryInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIBinaryInputStream {
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
impl<T: nsIInputStreamCoerce> nsIBinaryInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBinaryInputStream) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIBinaryInputStream
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIBinaryInputStreamVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIInputStreamVTable,

    /* void setInputStream (in nsIInputStream aInputStream); */
    pub SetInputStream: unsafe extern "system" fn (this: *const nsIBinaryInputStream, aInputStream: *const nsIInputStream) -> ::nserror::nsresult,

    /* boolean readBoolean (); */
    pub ReadBoolean: unsafe extern "system" fn (this: *const nsIBinaryInputStream, _retval: *mut bool) -> ::nserror::nsresult,

    /* uint8_t read8 (); */
    pub Read8: unsafe extern "system" fn (this: *const nsIBinaryInputStream, _retval: *mut uint8_t) -> ::nserror::nsresult,

    /* uint16_t read16 (); */
    pub Read16: unsafe extern "system" fn (this: *const nsIBinaryInputStream, _retval: *mut uint16_t) -> ::nserror::nsresult,

    /* uint32_t read32 (); */
    pub Read32: unsafe extern "system" fn (this: *const nsIBinaryInputStream, _retval: *mut uint32_t) -> ::nserror::nsresult,

    /* uint64_t read64 (); */
    pub Read64: unsafe extern "system" fn (this: *const nsIBinaryInputStream, _retval: *mut uint64_t) -> ::nserror::nsresult,

    /* float readFloat (); */
    pub ReadFloat: unsafe extern "system" fn (this: *const nsIBinaryInputStream, _retval: *mut libc::c_float) -> ::nserror::nsresult,

    /* double readDouble (); */
    pub ReadDouble: unsafe extern "system" fn (this: *const nsIBinaryInputStream, _retval: *mut libc::c_double) -> ::nserror::nsresult,

    /* ACString readCString (); */
    pub ReadCString: unsafe extern "system" fn (this: *const nsIBinaryInputStream, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* AString readString (); */
    pub ReadString: unsafe extern "system" fn (this: *const nsIBinaryInputStream, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void readBytes (in uint32_t aLength, [size_is (aLength), retval] out string aString); */
    pub ReadBytes: unsafe extern "system" fn (this: *const nsIBinaryInputStream, aLength: uint32_t, aString: *mut *const libc::c_char) -> ::nserror::nsresult,

    /* Array<uint8_t> readByteArray (in uint32_t aLength); */
    pub ReadByteArray: unsafe extern "system" fn (this: *const nsIBinaryInputStream, aLength: uint32_t, _retval: *mut thin_vec::ThinVec<uint8_t>) -> ::nserror::nsresult,

    /* [implicit_jscontext] uint64_t readArrayBuffer (in uint64_t aLength, in jsval aArrayBuffer); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub ReadArrayBuffer: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIBinaryInputStream {


    /// `void setInputStream (in nsIInputStream aInputStream);`
    #[inline]
    pub unsafe fn SetInputStream(&self, aInputStream: *const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).SetInputStream)(self, aInputStream)
    }


    /// ```text
    /// /**
    ///      * Read 8-bits from the stream.
    ///      *
    ///      * @return that byte to be treated as a boolean.
    ///      */
    /// ```
    ///

    /// `boolean readBoolean ();`
    #[inline]
    pub unsafe fn ReadBoolean(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ReadBoolean)(self, _retval)
    }



    /// `uint8_t read8 ();`
    #[inline]
    pub unsafe fn Read8(&self, _retval: *mut uint8_t) -> ::nserror::nsresult {
        ((*self.vtable).Read8)(self, _retval)
    }



    /// `uint16_t read16 ();`
    #[inline]
    pub unsafe fn Read16(&self, _retval: *mut uint16_t) -> ::nserror::nsresult {
        ((*self.vtable).Read16)(self, _retval)
    }



    /// `uint32_t read32 ();`
    #[inline]
    pub unsafe fn Read32(&self, _retval: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).Read32)(self, _retval)
    }



    /// `uint64_t read64 ();`
    #[inline]
    pub unsafe fn Read64(&self, _retval: *mut uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).Read64)(self, _retval)
    }



    /// `float readFloat ();`
    #[inline]
    pub unsafe fn ReadFloat(&self, _retval: *mut libc::c_float) -> ::nserror::nsresult {
        ((*self.vtable).ReadFloat)(self, _retval)
    }



    /// `double readDouble ();`
    #[inline]
    pub unsafe fn ReadDouble(&self, _retval: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).ReadDouble)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * Read an 8-bit pascal style string from the stream.
    ///      * 32-bit length field, followed by length 8-bit chars.
    ///      */
    /// ```
    ///

    /// `ACString readCString ();`
    #[inline]
    pub unsafe fn ReadCString(&self, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).ReadCString)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * Read an 16-bit pascal style string from the stream.
    ///      * 32-bit length field, followed by length PRUnichars.
    ///      */
    /// ```
    ///

    /// `AString readString ();`
    #[inline]
    pub unsafe fn ReadString(&self, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).ReadString)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * Read an opaque byte array from the stream.
    ///      *
    ///      * @param aLength the number of bytes that must be read.
    ///      *
    ///      * @throws NS_ERROR_FAILURE if it can't read aLength bytes
    ///      */
    /// ```
    ///

    /// `void readBytes (in uint32_t aLength, [size_is (aLength), retval] out string aString);`
    #[inline]
    pub unsafe fn ReadBytes(&self, aLength: uint32_t, aString: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).ReadBytes)(self, aLength, aString)
    }


    /// ```text
    /// /**
    ///      * Read an opaque byte array from the stream, storing the results
    ///      * as an array of PRUint8s.
    ///      *
    ///      * @param aLength the number of bytes that must be read.
    ///      *
    ///      * @throws NS_ERROR_FAILURE if it can't read aLength bytes
    ///      */
    /// ```
    ///

    /// `Array<uint8_t> readByteArray (in uint32_t aLength);`
    #[inline]
    pub unsafe fn ReadByteArray(&self, aLength: uint32_t, _retval: *mut thin_vec::ThinVec<uint8_t>) -> ::nserror::nsresult {
        ((*self.vtable).ReadByteArray)(self, aLength, _retval)
    }


    /// ```text
    /// /**
    ///      * Read opaque bytes from the stream, storing the results in an ArrayBuffer.
    ///      *
    ///      * @param aLength the number of bytes that must be read
    ///      * @param aArrayBuffer the arraybuffer in which to store the results
    ///      * Note: passing view.buffer, where view is an ArrayBufferView of an
    ///      *       ArrayBuffer, is not valid unless view.byteOffset == 0.
    ///      *
    ///      * @return The number of bytes actually read into aArrayBuffer.
    ///      */
    /// ```
    ///

    /// `[implicit_jscontext] uint64_t readArrayBuffer (in uint64_t aLength, in jsval aArrayBuffer);`
    const _ReadArrayBuffer: () = ();

}


