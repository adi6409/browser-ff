//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIConverterInputStream.idl
//


/// `interface nsIConverterInputStream : nsIUnicharInputStream`
///

/// ```text
/// /**
///  * A unichar input stream that wraps an input stream.
///  * This allows reading unicode strings from a stream, automatically converting
///  * the bytes from a selected character encoding.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIConverterInputStream {
    vtable: *const nsIConverterInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIConverterInputStream.
unsafe impl XpCom for nsIConverterInputStream {
    const IID: nsIID = nsID(0xfc66ffb6, 0x5404, 0x4908,
        [0xa4, 0xa3, 0x27, 0xf9, 0x2f, 0xa0, 0x57, 0x9d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIConverterInputStream {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIConverterInputStream.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIConverterInputStreamCoerce {
    /// Cheaply cast a value of this type from a `nsIConverterInputStream`.
    fn coerce_from(v: &nsIConverterInputStream) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIConverterInputStreamCoerce for nsIConverterInputStream {
    #[inline]
    fn coerce_from(v: &nsIConverterInputStream) -> &Self {
        v
    }
}

impl nsIConverterInputStream {
    /// Cast this `nsIConverterInputStream` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIConverterInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIConverterInputStream {
    type Target = nsIUnicharInputStream;
    #[inline]
    fn deref(&self) -> &nsIUnicharInputStream {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIUnicharInputStreamCoerce> nsIConverterInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIConverterInputStream) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIConverterInputStream
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIConverterInputStreamVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIUnicharInputStreamVTable,

    /* void init (in nsIInputStream aStream, in string aCharset, in long aBufferSize, in char16_t aReplacementChar); */
    pub Init: unsafe extern "system" fn (this: *const nsIConverterInputStream, aStream: *const nsIInputStream, aCharset: *const libc::c_char, aBufferSize: i32, aReplacementChar: char16_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIConverterInputStream {
    /// ```text
    /// /**
    ///      * Default replacement char value, U+FFFD REPLACEMENT CHARACTER.
    ///      */
    /// ```
    ///

    pub const DEFAULT_REPLACEMENT_CHARACTER: i64 = 65533;

    /// ```text
    /// /**
    ///      * Special replacement character value that requests errors to
    ///      * be treated as fatal.
    ///      */
    /// ```
    ///

    pub const ERRORS_ARE_FATAL: i64 = 0;

    /// ```text
    /// /**
    ///      * Initialize this stream.
    ///      * @param aStream
    ///      *        The underlying stream to read from.
    ///      * @param aCharset
    ///      *        The character encoding to use for converting the bytes of the
    ///      *        stream. A null charset will be interpreted as UTF-8.
    ///      * @param aBufferSize
    ///      *        How many bytes to buffer.
    ///      * @param aReplacementChar
    ///      *        The character to replace unknown byte sequences in the stream
    ///      *        with. The standard replacement character is U+FFFD.
    ///      *        A value of 0x0000 will cause an exception to be thrown if unknown
    ///      *        byte sequences are encountered in the stream.
    ///      */
    /// ```
    ///

    /// `void init (in nsIInputStream aStream, in string aCharset, in long aBufferSize, in char16_t aReplacementChar);`
    #[inline]
    pub unsafe fn Init(&self, aStream: *const nsIInputStream, aCharset: *const libc::c_char, aBufferSize: i32, aReplacementChar: char16_t) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aStream, aCharset, aBufferSize, aReplacementChar)
    }


}


