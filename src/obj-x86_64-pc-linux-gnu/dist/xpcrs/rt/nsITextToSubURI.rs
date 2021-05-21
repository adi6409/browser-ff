//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/intl/uconv/nsITextToSubURI.idl
//


/// `interface nsITextToSubURI : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITextToSubURI {
    vtable: *const nsITextToSubURIVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITextToSubURI.
unsafe impl XpCom for nsITextToSubURI {
    const IID: nsIID = nsID(0x8b042e24, 0x6f87, 0x11d3,
        [0xb3, 0xc8, 0x00, 0x80, 0x5f, 0x8a, 0x66, 0x70]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITextToSubURI {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITextToSubURI.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITextToSubURICoerce {
    /// Cheaply cast a value of this type from a `nsITextToSubURI`.
    fn coerce_from(v: &nsITextToSubURI) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITextToSubURICoerce for nsITextToSubURI {
    #[inline]
    fn coerce_from(v: &nsITextToSubURI) -> &Self {
        v
    }
}

impl nsITextToSubURI {
    /// Cast this `nsITextToSubURI` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITextToSubURICoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITextToSubURI {
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
impl<T: nsISupportsCoerce> nsITextToSubURICoerce for T {
    #[inline]
    fn coerce_from(v: &nsITextToSubURI) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITextToSubURI
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITextToSubURIVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* ACString ConvertAndEscape (in ACString charset, in AString text); */
    pub ConvertAndEscape: unsafe extern "system" fn (this: *const nsITextToSubURI, charset: *const ::nsstring::nsACString, text: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* AString UnEscapeAndConvert (in ACString charset, in ACString text); */
    pub UnEscapeAndConvert: unsafe extern "system" fn (this: *const nsITextToSubURI, charset: *const ::nsstring::nsACString, text: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString unEscapeURIForUI (in AUTF8String aURIFragment); */
    pub UnEscapeURIForUI: unsafe extern "system" fn (this: *const nsITextToSubURI, aURIFragment: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString unEscapeNonAsciiURI (in ACString aCharset, in AUTF8String aURIFragment); */
    pub UnEscapeNonAsciiURI: unsafe extern "system" fn (this: *const nsITextToSubURI, aCharset: *const ::nsstring::nsACString, aURIFragment: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITextToSubURI {


    /// `ACString ConvertAndEscape (in ACString charset, in AString text);`
    #[inline]
    pub unsafe fn ConvertAndEscape(&self, charset: *const ::nsstring::nsACString, text: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).ConvertAndEscape)(self, charset, text, _retval)
    }



    /// `AString UnEscapeAndConvert (in ACString charset, in ACString text);`
    #[inline]
    pub unsafe fn UnEscapeAndConvert(&self, charset: *const ::nsstring::nsACString, text: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).UnEscapeAndConvert)(self, charset, text, _retval)
    }


    /// ```text
    /// /**
    ///    * Unescapes the given URI fragment (for UI purpose only)
    ///    * Note:
    ///    * <ul>
    ///    *  <li> escaping back the result (unescaped string) is not guaranteed to
    ///    *       give the original escaped string
    ///    *  <li> The URI fragment (escaped) is assumed to be in UTF-8 and converted
    ///    *       to AString (UTF-16)
    ///    *  <li> In case of successful conversion any resulting character listed
    ///    *       in netwerk/dns/IDNCharacterBlocklist.inc (except space) is escaped
    ///    *  <li> Always succeeeds (callers don't need to do error checking)
    ///    * </ul>
    ///    *
    ///    * @param aURIFragment the URI (or URI fragment) to unescape
    ///    * @return Unescaped aURIFragment  converted to unicode
    ///    */
    /// ```
    ///

    /// `AString unEscapeURIForUI (in AUTF8String aURIFragment);`
    #[inline]
    pub unsafe fn UnEscapeURIForUI(&self, aURIFragment: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).UnEscapeURIForUI)(self, aURIFragment, _retval)
    }


    /// ```text
    /// /**
    ///    * Unescapes only non ASCII characters in the given URI fragment
    ///    * note: this method assumes the URI as UTF-8 and fallbacks to the given
    ///    * charset if the charset is an ASCII superset
    ///    *
    ///    * @param aCharset the charset to convert from
    ///    * @param aURIFragment the URI (or URI fragment) to unescape
    ///    * @return Unescaped aURIFragment  converted to unicode
    ///    * @throws NS_ERROR_UCONV_NOCONV when there is no decoder for aCharset
    ///    *         or NS_ERROR_UDEC_ILLEGALINPUT in case of conversion failure
    ///    */
    /// ```
    ///

    /// `AString unEscapeNonAsciiURI (in ACString aCharset, in AUTF8String aURIFragment);`
    #[inline]
    pub unsafe fn UnEscapeNonAsciiURI(&self, aCharset: *const ::nsstring::nsACString, aURIFragment: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).UnEscapeNonAsciiURI)(self, aCharset, aURIFragment, _retval)
    }


}


