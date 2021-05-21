//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIConverterOutputStream.idl
//


/// `interface nsIConverterOutputStream : nsIUnicharOutputStream`
///

/// ```text
/// /**
///  * This interface allows writing strings to a stream, doing automatic
///  * character encoding conversion.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIConverterOutputStream {
    vtable: *const nsIConverterOutputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIConverterOutputStream.
unsafe impl XpCom for nsIConverterOutputStream {
    const IID: nsIID = nsID(0x4b71113a, 0xcb0d, 0x479f,
        [0x8e, 0xd5, 0x01, 0xda, 0xeb, 0xa2, 0xe8, 0xd4]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIConverterOutputStream {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIConverterOutputStream.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIConverterOutputStreamCoerce {
    /// Cheaply cast a value of this type from a `nsIConverterOutputStream`.
    fn coerce_from(v: &nsIConverterOutputStream) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIConverterOutputStreamCoerce for nsIConverterOutputStream {
    #[inline]
    fn coerce_from(v: &nsIConverterOutputStream) -> &Self {
        v
    }
}

impl nsIConverterOutputStream {
    /// Cast this `nsIConverterOutputStream` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIConverterOutputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIConverterOutputStream {
    type Target = nsIUnicharOutputStream;
    #[inline]
    fn deref(&self) -> &nsIUnicharOutputStream {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIUnicharOutputStreamCoerce> nsIConverterOutputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIConverterOutputStream) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIConverterOutputStream
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIConverterOutputStreamVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIUnicharOutputStreamVTable,

    /* void init (in nsIOutputStream aOutStream, in string aCharset); */
    pub Init: unsafe extern "system" fn (this: *const nsIConverterOutputStream, aOutStream: *const nsIOutputStream, aCharset: *const libc::c_char) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIConverterOutputStream {

    /// ```text
    /// /**
    ///      * Initialize this stream. Must be called before any other method on this
    ///      * interface, or you will crash. The output stream passed to this method
    ///      * must not be null, or you will crash.
    ///      *
    ///      * @param aOutStream
    ///      *        The underlying output stream to which the converted strings will
    ///      *        be written.
    ///      * @param aCharset
    ///      *        The character set to use for encoding the characters. A null
    ///      *        charset will be interpreted as UTF-8.
    ///      */
    /// ```
    ///

    /// `void init (in nsIOutputStream aOutStream, in string aCharset);`
    #[inline]
    pub unsafe fn Init(&self, aOutStream: *const nsIOutputStream, aCharset: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aOutStream, aCharset)
    }


}


