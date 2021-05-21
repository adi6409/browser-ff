//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/intl/uconv/nsIScriptableUConv.idl
//


/// `interface nsIScriptableUnicodeConverter : nsISupports`
///

/// ```text
/// /**
///  * In new code, please use the WebIDL TextDecoder and TextEncoder
///  * instead. They represent bytes as Uint8Array (or as view to such
    ///  * array), which is the current best practice for representing bytes
///  * in JavaScript.
///  *
///  * This interface converts between UTF-16 in JavaScript strings
///  * and bytes transported as the unsigned value of each byte
///  * transported in a code unit of the same numeric value in
///  * a JavaScript string.
///  *
///  * @created         8/Jun/2000
///  * @author          Makoto Kato [m_kato@ga2.so-net.ne.jp]
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIScriptableUnicodeConverter {
    vtable: *const nsIScriptableUnicodeConverterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIScriptableUnicodeConverter.
unsafe impl XpCom for nsIScriptableUnicodeConverter {
    const IID: nsIID = nsID(0xf36ee324, 0x5c1c, 0x437f,
        [0xba, 0x10, 0x2b, 0x4d, 0xb7, 0xa1, 0x80, 0x31]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIScriptableUnicodeConverter {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIScriptableUnicodeConverter.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIScriptableUnicodeConverterCoerce {
    /// Cheaply cast a value of this type from a `nsIScriptableUnicodeConverter`.
    fn coerce_from(v: &nsIScriptableUnicodeConverter) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIScriptableUnicodeConverterCoerce for nsIScriptableUnicodeConverter {
    #[inline]
    fn coerce_from(v: &nsIScriptableUnicodeConverter) -> &Self {
        v
    }
}

impl nsIScriptableUnicodeConverter {
    /// Cast this `nsIScriptableUnicodeConverter` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIScriptableUnicodeConverterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIScriptableUnicodeConverter {
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
impl<T: nsISupportsCoerce> nsIScriptableUnicodeConverterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIScriptableUnicodeConverter) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIScriptableUnicodeConverter
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIScriptableUnicodeConverterVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* ACString ConvertFromUnicode (in AString aSrc); */
    pub ConvertFromUnicode: unsafe extern "system" fn (this: *const nsIScriptableUnicodeConverter, aSrc: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* ACString Finish (); */
    pub Finish: unsafe extern "system" fn (this: *const nsIScriptableUnicodeConverter, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* AString ConvertToUnicode (in ACString aSrc); */
    pub ConvertToUnicode: unsafe extern "system" fn (this: *const nsIScriptableUnicodeConverter, aSrc: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void convertToByteArray (in AString aString, [optional] out unsigned long aLen, [array, size_is (aLen), retval] out octet aData); */
    pub ConvertToByteArray: unsafe extern "system" fn (this: *const nsIScriptableUnicodeConverter, aString: *const ::nsstring::nsAString, aLen: *mut u32, aData: *mut *mut u8) -> ::nserror::nsresult,

    /* nsIInputStream convertToInputStream (in AString aString); */
    pub ConvertToInputStream: unsafe extern "system" fn (this: *const nsIScriptableUnicodeConverter, aString: *const ::nsstring::nsAString, _retval: *mut*const nsIInputStream) -> ::nserror::nsresult,

    /* attribute ACString charset; */
    pub GetCharset: unsafe extern "system" fn (this: *const nsIScriptableUnicodeConverter, aCharset: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute ACString charset; */
    pub SetCharset: unsafe extern "system" fn (this: *const nsIScriptableUnicodeConverter, aCharset: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute boolean isInternal; */
    pub GetIsInternal: unsafe extern "system" fn (this: *const nsIScriptableUnicodeConverter, aIsInternal: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean isInternal; */
    pub SetIsInternal: unsafe extern "system" fn (this: *const nsIScriptableUnicodeConverter, aIsInternal: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIScriptableUnicodeConverter {

    /// ```text
    /// /**
    ///    * Converts the data from Unicode to one Charset.
    ///    * Returns the converted string. After converting, Finish should be called
    ///    * and its return value appended to this return value.
    ///    */
    /// ```
    ///

    /// `ACString ConvertFromUnicode (in AString aSrc);`
    #[inline]
    pub unsafe fn ConvertFromUnicode(&self, aSrc: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).ConvertFromUnicode)(self, aSrc, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns the terminator string.
    ///    * Should be called after ConvertFromUnicode() and appended to that
    ///    * function's return value.
    ///    */
    /// ```
    ///

    /// `ACString Finish ();`
    #[inline]
    pub unsafe fn Finish(&self, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Finish)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Converts the data from one Charset to Unicode.
    ///    */
    /// ```
    ///

    /// `AString ConvertToUnicode (in ACString aSrc);`
    #[inline]
    pub unsafe fn ConvertToUnicode(&self, aSrc: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).ConvertToUnicode)(self, aSrc, _retval)
    }


    /// ```text
    /// /**
    ///    * Convert a unicode string to an array of bytes. Finish does not need to be
    ///    * called.
    ///    */
    /// ```
    ///

    /// `void convertToByteArray (in AString aString, [optional] out unsigned long aLen, [array, size_is (aLen), retval] out octet aData);`
    #[inline]
    pub unsafe fn ConvertToByteArray(&self, aString: *const ::nsstring::nsAString, aLen: *mut u32, aData: *mut *mut u8) -> ::nserror::nsresult {
        ((*self.vtable).ConvertToByteArray)(self, aString, aLen, aData)
    }


    /// ```text
    /// /**
    ///    * Converts a unicode string to an input stream. The bytes in the stream are
    ///    * encoded according to the charset attribute.
    ///    * The returned stream will be nonblocking.
    ///    */
    /// ```
    ///

    /// `nsIInputStream convertToInputStream (in AString aString);`
    #[inline]
    pub unsafe fn ConvertToInputStream(&self, aString: *const ::nsstring::nsAString, _retval: *mut*const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).ConvertToInputStream)(self, aString, _retval)
    }


    /// ```text
    /// /**
    ///    * Current character set.
    ///    *
    ///    * @throw NS_ERROR_UCONV_NOCONV
    ///    *        The requested charset is not supported.
    ///    */
    /// ```
    ///

    /// `attribute ACString charset;`
    #[inline]
    pub unsafe fn GetCharset(&self, aCharset: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetCharset)(self, aCharset)
    }


    /// ```text
    /// /**
    ///    * Current character set.
    ///    *
    ///    * @throw NS_ERROR_UCONV_NOCONV
    ///    *        The requested charset is not supported.
    ///    */
    /// ```
    ///

    /// `attribute ACString charset;`
    #[inline]
    pub unsafe fn SetCharset(&self, aCharset: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetCharset)(self, aCharset)
    }


    /// ```text
    /// /**
    ///    * Meaningless
    ///    */
    /// ```
    ///

    /// `attribute boolean isInternal;`
    #[inline]
    pub unsafe fn GetIsInternal(&self, aIsInternal: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsInternal)(self, aIsInternal)
    }


    /// ```text
    /// /**
    ///    * Meaningless
    ///    */
    /// ```
    ///

    /// `attribute boolean isInternal;`
    #[inline]
    pub unsafe fn SetIsInternal(&self, aIsInternal: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetIsInternal)(self, aIsInternal)
    }


}


