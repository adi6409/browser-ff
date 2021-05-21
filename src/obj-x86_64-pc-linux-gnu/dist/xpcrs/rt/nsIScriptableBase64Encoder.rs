//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIScriptableBase64Encoder.idl
//


/// `interface nsIScriptableBase64Encoder : nsISupports`
///

/// ```text
/// /**
///  * nsIScriptableBase64Encoder efficiently encodes the contents
///  * of a nsIInputStream to a Base64 string.  This avoids the need
///  * to read the entire stream into a buffer, and only then do the
///  * Base64 encoding.
///  *
///  *  If you already have a buffer full of data, you should use
///  *  btoa instead!
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIScriptableBase64Encoder {
    vtable: *const nsIScriptableBase64EncoderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIScriptableBase64Encoder.
unsafe impl XpCom for nsIScriptableBase64Encoder {
    const IID: nsIID = nsID(0x9479c864, 0xd1f9, 0x45ab,
        [0xb7, 0xb9, 0x28, 0xb9, 0x07, 0xbd, 0x2b, 0xa9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIScriptableBase64Encoder {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIScriptableBase64Encoder.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIScriptableBase64EncoderCoerce {
    /// Cheaply cast a value of this type from a `nsIScriptableBase64Encoder`.
    fn coerce_from(v: &nsIScriptableBase64Encoder) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIScriptableBase64EncoderCoerce for nsIScriptableBase64Encoder {
    #[inline]
    fn coerce_from(v: &nsIScriptableBase64Encoder) -> &Self {
        v
    }
}

impl nsIScriptableBase64Encoder {
    /// Cast this `nsIScriptableBase64Encoder` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIScriptableBase64EncoderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIScriptableBase64Encoder {
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
impl<T: nsISupportsCoerce> nsIScriptableBase64EncoderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIScriptableBase64Encoder) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIScriptableBase64Encoder
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIScriptableBase64EncoderVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* ACString encodeToCString (in nsIInputStream stream, in unsigned long length); */
    pub EncodeToCString: unsafe extern "system" fn (this: *const nsIScriptableBase64Encoder, stream: *const nsIInputStream, length: u32, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* AString encodeToString (in nsIInputStream stream, in unsigned long length); */
    pub EncodeToString: unsafe extern "system" fn (this: *const nsIScriptableBase64Encoder, stream: *const nsIInputStream, length: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIScriptableBase64Encoder {

    /// ```text
    /// /**
    ///    *  These methods take an nsIInputStream and return a narrow or wide
    ///    *  string with the contents of the nsIInputStream base64 encoded.
    ///    *
    ///    *  The stream passed in must support ReadSegments and must not be
    ///    *  a non-blocking stream that will return NS_BASE_STREAM_WOULD_BLOCK.
    ///    *  If either of these restrictions are violated we will abort.
    ///    */
    /// ```
    ///

    /// `ACString encodeToCString (in nsIInputStream stream, in unsigned long length);`
    #[inline]
    pub unsafe fn EncodeToCString(&self, stream: *const nsIInputStream, length: u32, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).EncodeToCString)(self, stream, length, _retval)
    }



    /// `AString encodeToString (in nsIInputStream stream, in unsigned long length);`
    #[inline]
    pub unsafe fn EncodeToString(&self, stream: *const nsIInputStream, length: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).EncodeToString)(self, stream, length, _retval)
    }


}


