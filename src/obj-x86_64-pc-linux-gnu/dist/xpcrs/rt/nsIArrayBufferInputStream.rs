//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIArrayBufferInputStream.idl
//


/// `interface nsIArrayBufferInputStream : nsIInputStream`
///

/// ```text
/// /**
///  * nsIArrayBufferInputStream
///  *
///  * Provides scriptable methods for initializing a nsIInputStream
///  * implementation with an ArrayBuffer.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIArrayBufferInputStream {
    vtable: *const nsIArrayBufferInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIArrayBufferInputStream.
unsafe impl XpCom for nsIArrayBufferInputStream {
    const IID: nsIID = nsID(0x3014dde6, 0xaa1c, 0x41db,
        [0x87, 0xd0, 0x48, 0x76, 0x4a, 0x37, 0x10, 0xf6]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIArrayBufferInputStream {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIArrayBufferInputStream.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIArrayBufferInputStreamCoerce {
    /// Cheaply cast a value of this type from a `nsIArrayBufferInputStream`.
    fn coerce_from(v: &nsIArrayBufferInputStream) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIArrayBufferInputStreamCoerce for nsIArrayBufferInputStream {
    #[inline]
    fn coerce_from(v: &nsIArrayBufferInputStream) -> &Self {
        v
    }
}

impl nsIArrayBufferInputStream {
    /// Cast this `nsIArrayBufferInputStream` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIArrayBufferInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIArrayBufferInputStream {
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
impl<T: nsIInputStreamCoerce> nsIArrayBufferInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIArrayBufferInputStream) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIArrayBufferInputStream
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIArrayBufferInputStreamVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIInputStreamVTable,

    /* void setData (in jsval buffer, in uint64_t byteOffset, in uint64_t byteLen); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub SetData: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIArrayBufferInputStream {

    /// ```text
    /// /**
    ///      * SetData - assign an ArrayBuffer to the input stream.
    ///      *
    ///      * @param buffer    - stream data
    ///      * @param byteOffset - stream data offset
    ///      * @param byteLen - stream data length
    ///      */
    /// ```
    ///

    /// `void setData (in jsval buffer, in uint64_t byteOffset, in uint64_t byteLen);`
    const _SetData: () = ();

}


