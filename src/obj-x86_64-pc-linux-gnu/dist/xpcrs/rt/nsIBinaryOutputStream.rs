//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIBinaryOutputStream.idl
//


/// `interface nsIBinaryOutputStream : nsIOutputStream`
///

/// ```text
/// /**
///  * This interface allows writing of primitive data types (integers,
    ///  * floating-point values, booleans, etc.) to a stream in a binary, untagged,
///  * fixed-endianness format.  This might be used, for example, to implement
///  * network protocols or to produce architecture-neutral binary disk files,
///  * i.e. ones that can be read and written by both big-endian and little-endian
///  * platforms.  Output is written in big-endian order (high-order byte first),
///  * as this is traditional network order.
///  *
///  * @See nsIBinaryInputStream
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIBinaryOutputStream {
    vtable: *const nsIBinaryOutputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIBinaryOutputStream.
unsafe impl XpCom for nsIBinaryOutputStream {
    const IID: nsIID = nsID(0x204ee610, 0x8765, 0x11d3,
        [0x90, 0xcf, 0x00, 0x40, 0x05, 0x6a, 0x90, 0x6e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIBinaryOutputStream {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIBinaryOutputStream.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIBinaryOutputStreamCoerce {
    /// Cheaply cast a value of this type from a `nsIBinaryOutputStream`.
    fn coerce_from(v: &nsIBinaryOutputStream) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIBinaryOutputStreamCoerce for nsIBinaryOutputStream {
    #[inline]
    fn coerce_from(v: &nsIBinaryOutputStream) -> &Self {
        v
    }
}

impl nsIBinaryOutputStream {
    /// Cast this `nsIBinaryOutputStream` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIBinaryOutputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIBinaryOutputStream {
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
impl<T: nsIOutputStreamCoerce> nsIBinaryOutputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBinaryOutputStream) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIBinaryOutputStream
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIBinaryOutputStreamVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIOutputStreamVTable,

    /* void setOutputStream (in nsIOutputStream aOutputStream); */
    pub SetOutputStream: unsafe extern "system" fn (this: *const nsIBinaryOutputStream, aOutputStream: *const nsIOutputStream) -> ::nserror::nsresult,

    /* void writeBoolean (in boolean aBoolean); */
    pub WriteBoolean: unsafe extern "system" fn (this: *const nsIBinaryOutputStream, aBoolean: bool) -> ::nserror::nsresult,

    /* void write8 (in uint8_t aByte); */
    pub Write8: unsafe extern "system" fn (this: *const nsIBinaryOutputStream, aByte: uint8_t) -> ::nserror::nsresult,

    /* void write16 (in uint16_t a16); */
    pub Write16: unsafe extern "system" fn (this: *const nsIBinaryOutputStream, a16: uint16_t) -> ::nserror::nsresult,

    /* void write32 (in uint32_t a32); */
    pub Write32: unsafe extern "system" fn (this: *const nsIBinaryOutputStream, a32: uint32_t) -> ::nserror::nsresult,

    /* void write64 (in uint64_t a64); */
    pub Write64: unsafe extern "system" fn (this: *const nsIBinaryOutputStream, a64: uint64_t) -> ::nserror::nsresult,

    /* void writeFloat (in float aFloat); */
    pub WriteFloat: unsafe extern "system" fn (this: *const nsIBinaryOutputStream, aFloat: libc::c_float) -> ::nserror::nsresult,

    /* void writeDouble (in double aDouble); */
    pub WriteDouble: unsafe extern "system" fn (this: *const nsIBinaryOutputStream, aDouble: libc::c_double) -> ::nserror::nsresult,

    /* void writeStringZ (in string aString); */
    pub WriteStringZ: unsafe extern "system" fn (this: *const nsIBinaryOutputStream, aString: *const libc::c_char) -> ::nserror::nsresult,

    /* void writeWStringZ (in wstring aString); */
    pub WriteWStringZ: unsafe extern "system" fn (this: *const nsIBinaryOutputStream, aString: *const i16) -> ::nserror::nsresult,

    /* void writeUtf8Z (in wstring aString); */
    pub WriteUtf8Z: unsafe extern "system" fn (this: *const nsIBinaryOutputStream, aString: *const i16) -> ::nserror::nsresult,

    /* [binaryname(WriteBytesFromJS)] void writeBytes ([size_is (aLength)] in string aString, [optional] in uint32_t aLength); */
    pub WriteBytesFromJS: unsafe extern "system" fn (this: *const nsIBinaryOutputStream, aString: *const libc::c_char, aLength: uint32_t) -> ::nserror::nsresult,

    /* [binaryname(WriteBytes),noscript,nostdcall] void writeBytesNative (in Bytes aBytes); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub WriteBytes: *const ::libc::c_void,

    /* void writeByteArray (in Array<uint8_t> aBytes); */
    pub WriteByteArray: unsafe extern "system" fn (this: *const nsIBinaryOutputStream, aBytes: *const thin_vec::ThinVec<uint8_t>) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIBinaryOutputStream {


    /// `void setOutputStream (in nsIOutputStream aOutputStream);`
    #[inline]
    pub unsafe fn SetOutputStream(&self, aOutputStream: *const nsIOutputStream) -> ::nserror::nsresult {
        ((*self.vtable).SetOutputStream)(self, aOutputStream)
    }


    /// ```text
    /// /**
    ///      * Write a boolean as an 8-bit char to the stream.
    ///      */
    /// ```
    ///

    /// `void writeBoolean (in boolean aBoolean);`
    #[inline]
    pub unsafe fn WriteBoolean(&self, aBoolean: bool) -> ::nserror::nsresult {
        ((*self.vtable).WriteBoolean)(self, aBoolean)
    }



    /// `void write8 (in uint8_t aByte);`
    #[inline]
    pub unsafe fn Write8(&self, aByte: uint8_t) -> ::nserror::nsresult {
        ((*self.vtable).Write8)(self, aByte)
    }



    /// `void write16 (in uint16_t a16);`
    #[inline]
    pub unsafe fn Write16(&self, a16: uint16_t) -> ::nserror::nsresult {
        ((*self.vtable).Write16)(self, a16)
    }



    /// `void write32 (in uint32_t a32);`
    #[inline]
    pub unsafe fn Write32(&self, a32: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).Write32)(self, a32)
    }



    /// `void write64 (in uint64_t a64);`
    #[inline]
    pub unsafe fn Write64(&self, a64: uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).Write64)(self, a64)
    }



    /// `void writeFloat (in float aFloat);`
    #[inline]
    pub unsafe fn WriteFloat(&self, aFloat: libc::c_float) -> ::nserror::nsresult {
        ((*self.vtable).WriteFloat)(self, aFloat)
    }



    /// `void writeDouble (in double aDouble);`
    #[inline]
    pub unsafe fn WriteDouble(&self, aDouble: libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).WriteDouble)(self, aDouble)
    }


    /// ```text
    /// /**
    ///      * Write an 8-bit pascal style string to the stream.
    ///      * 32-bit length field, followed by length 8-bit chars.
    ///      */
    /// ```
    ///

    /// `void writeStringZ (in string aString);`
    #[inline]
    pub unsafe fn WriteStringZ(&self, aString: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).WriteStringZ)(self, aString)
    }


    /// ```text
    /// /**
    ///      * Write a 16-bit pascal style string to the stream.
    ///      * 32-bit length field, followed by length PRUnichars.
    ///      */
    /// ```
    ///

    /// `void writeWStringZ (in wstring aString);`
    #[inline]
    pub unsafe fn WriteWStringZ(&self, aString: *const i16) -> ::nserror::nsresult {
        ((*self.vtable).WriteWStringZ)(self, aString)
    }


    /// ```text
    /// /**
    ///      * Write an 8-bit pascal style string (UTF8-encoded) to the stream.
    ///      * 32-bit length field, followed by length 8-bit chars.
    ///      */
    /// ```
    ///

    /// `void writeUtf8Z (in wstring aString);`
    #[inline]
    pub unsafe fn WriteUtf8Z(&self, aString: *const i16) -> ::nserror::nsresult {
        ((*self.vtable).WriteUtf8Z)(self, aString)
    }


    /// ```text
    /// /**
    ///      * Write an opaque byte array to the stream.
    ///      */
    /// ```
    ///

    /// `[binaryname(WriteBytesFromJS)] void writeBytes ([size_is (aLength)] in string aString, [optional] in uint32_t aLength);`
    #[inline]
    pub unsafe fn WriteBytesFromJS(&self, aString: *const libc::c_char, aLength: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).WriteBytesFromJS)(self, aString, aLength)
    }


    /// ```text
    /// /**
    ///      * Non-scriptable and saner-signature version of the same.
    ///      */
    /// ```
    ///

    /// `[binaryname(WriteBytes),noscript,nostdcall] void writeBytesNative (in Bytes aBytes);`
    const _WriteBytes: () = ();

    /// ```text
    /// /**
    ///      * Write an opaque byte array to the stream.
    ///      */
    /// ```
    ///

    /// `void writeByteArray (in Array<uint8_t> aBytes);`
    #[inline]
    pub unsafe fn WriteByteArray(&self, aBytes: *const thin_vec::ThinVec<uint8_t>) -> ::nserror::nsresult {
        ((*self.vtable).WriteByteArray)(self, aBytes)
    }


}


