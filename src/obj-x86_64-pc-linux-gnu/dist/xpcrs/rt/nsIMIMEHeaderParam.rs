//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/mime/nsIMIMEHeaderParam.idl
//


/// `interface nsIMIMEHeaderParam : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIMIMEHeaderParam {
    vtable: *const nsIMIMEHeaderParamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIMIMEHeaderParam.
unsafe impl XpCom for nsIMIMEHeaderParam {
    const IID: nsIID = nsID(0x9c9252a1, 0xfdaf, 0x40a2,
        [0x9c, 0x2b, 0xa3, 0xdc, 0x45, 0xe2, 0x8d, 0xde]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIMIMEHeaderParam {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIMIMEHeaderParam.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIMIMEHeaderParamCoerce {
    /// Cheaply cast a value of this type from a `nsIMIMEHeaderParam`.
    fn coerce_from(v: &nsIMIMEHeaderParam) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIMIMEHeaderParamCoerce for nsIMIMEHeaderParam {
    #[inline]
    fn coerce_from(v: &nsIMIMEHeaderParam) -> &Self {
        v
    }
}

impl nsIMIMEHeaderParam {
    /// Cast this `nsIMIMEHeaderParam` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIMIMEHeaderParamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIMIMEHeaderParam {
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
impl<T: nsISupportsCoerce> nsIMIMEHeaderParamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMIMEHeaderParam) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIMIMEHeaderParam
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIMIMEHeaderParamVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* AString getParameter (in ACString aHeaderVal, in string aParamName, in ACString aFallbackCharset, in boolean aTryLocaleCharset, out string aLang); */
    pub GetParameter: unsafe extern "system" fn (this: *const nsIMIMEHeaderParam, aHeaderVal: *const ::nsstring::nsACString, aParamName: *const libc::c_char, aFallbackCharset: *const ::nsstring::nsACString, aTryLocaleCharset: bool, aLang: *mut *const libc::c_char, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString getParameterHTTP (in ACString aHeaderVal, in string aParamName, in ACString aFallbackCharset, in boolean aTryLocaleCharset, out string aLang); */
    pub GetParameterHTTP: unsafe extern "system" fn (this: *const nsIMIMEHeaderParam, aHeaderVal: *const ::nsstring::nsACString, aParamName: *const libc::c_char, aFallbackCharset: *const ::nsstring::nsACString, aTryLocaleCharset: bool, aLang: *mut *const libc::c_char, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString decodeRFC5987Param (in ACString aParamVal, out ACString aLang); */
    pub DecodeRFC5987Param: unsafe extern "system" fn (this: *const nsIMIMEHeaderParam, aParamVal: *const ::nsstring::nsACString, aLang: *mut ::nsstring::nsACString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [noscript] string getParameterInternal (in string aHeaderVal, in string aParamName, out string aCharset, out string aLang); */
    pub GetParameterInternal: unsafe extern "system" fn (this: *const nsIMIMEHeaderParam, aHeaderVal: *const libc::c_char, aParamName: *const libc::c_char, aCharset: *mut *const libc::c_char, aLang: *mut *const libc::c_char, _retval: *mut *const libc::c_char) -> ::nserror::nsresult,

    /* [noscript] ACString decodeRFC2047Header (in string aHeaderVal, in string aDefaultCharset, in boolean aOverrideCharset, in boolean aEatContinuation); */
    pub DecodeRFC2047Header: unsafe extern "system" fn (this: *const nsIMIMEHeaderParam, aHeaderVal: *const libc::c_char, aDefaultCharset: *const libc::c_char, aOverrideCharset: bool, aEatContinuation: bool, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [noscript] ACString decodeParameter (in ACString aParamValue, in string aCharset, in string aDefaultCharset, in boolean aOverrideCharset); */
    pub DecodeParameter: unsafe extern "system" fn (this: *const nsIMIMEHeaderParam, aParamValue: *const ::nsstring::nsACString, aCharset: *const libc::c_char, aDefaultCharset: *const libc::c_char, aOverrideCharset: bool, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIMIMEHeaderParam {

    /// ```text
    /// /**
    ///    * Given the value of a single header field  (such as
        ///    * Content-Disposition and Content-Type) and the name of a parameter
    ///    * (e.g. filename, name, charset), returns the value of the parameter.
    ///    * The value is obtained by decoding RFC 2231/5987-style encoding,
    ///    * RFC 2047-style encoding, and converting to UniChar(UTF-16)
    ///    * from charset specified in RFC 2231/2047 encoding, UTF-8,
    ///    * <code>aFallbackCharset</code>, the locale charset as fallback if
    ///    * <code>TryLocaleCharset</code> is set, and null-padding as last resort
    ///    * if all else fails.
    ///    *
    ///    * <p>
    ///    * This method internally invokes <code>getParameterInternal</code>,
    ///    * However, it does not stop at decoding RFC 2231 (the task for
        ///    * <code>getParameterInternal</code> but tries to cope
        ///    * with several non-standard-compliant cases mentioned below.
        ///    *
        ///    * <p>
        ///    * Note that a lot of MUAs put RFC 2047-encoded parameters. Unfortunately,
        ///    * this includes Mozilla as of 2003-05-30. Even more standard-ignorant MUAs,
        ///    * web servers and application servers put 'raw 8bit characters'. This will
        ///    * try to cope with all these cases as gracefully as possible. Additionally,
        ///    * it returns the language tag if the parameter is encoded per RFC 2231 and
        ///    * includes lang.
        ///    *
        ///    * <p>
        ///    * Note that GetParameterHTTP skips some of the workarounds used for
        ///    * mail (MIME) header fields, and thus SHOULD be used from non-mail
        ///    * code.
        ///    *
        ///    *
        ///    * @param  aHeaderVal        a header string to get the value of a parameter
        ///    *                           from.
        ///    * @param  aParamName        the name of a MIME header parameter (e.g.
            ///    *                           filename, name, charset). If empty,  returns
        ///    *                           the first (possibly) _unnamed_ 'parameter'.
        ///    * @param  aFallbackCharset  fallback charset to try if  the string after
        ///    *                           RFC 2231/2047 decoding or the raw 8bit
        ///    *                           string is not UTF-8
        ///    * @param  aTryLocaleCharset If set, makes yet another attempt
        ///    *                           with the locale charset.
        ///    * @param  aLang             If non-null, assigns it to a pointer
        ///    *                           to a string containing the value of language
        ///    *                           obtained from RFC 2231 parsing. Caller has to
        ///    *                           free it.
        ///    * @return the value of <code>aParamName</code> in Unichar(UTF-16).
        ///    */
        /// ```
        ///

        /// `AString getParameter (in ACString aHeaderVal, in string aParamName, in ACString aFallbackCharset, in boolean aTryLocaleCharset, out string aLang);`
        #[inline]
        pub unsafe fn GetParameter(&self, aHeaderVal: *const ::nsstring::nsACString, aParamName: *const libc::c_char, aFallbackCharset: *const ::nsstring::nsACString, aTryLocaleCharset: bool, aLang: *mut *const libc::c_char, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
            ((*self.vtable).GetParameter)(self, aHeaderVal, aParamName, aFallbackCharset, aTryLocaleCharset, aLang, _retval)
        }


        /// ```text
        /// /**
        ///    * Like getParameter, but disabling encodings and workarounds specific to
        ///    * MIME (as opposed to HTTP).
        ///    */
        /// ```
        ///

        /// `AString getParameterHTTP (in ACString aHeaderVal, in string aParamName, in ACString aFallbackCharset, in boolean aTryLocaleCharset, out string aLang);`
        #[inline]
        pub unsafe fn GetParameterHTTP(&self, aHeaderVal: *const ::nsstring::nsACString, aParamName: *const libc::c_char, aFallbackCharset: *const ::nsstring::nsACString, aTryLocaleCharset: bool, aLang: *mut *const libc::c_char, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
            ((*self.vtable).GetParameterHTTP)(self, aHeaderVal, aParamName, aFallbackCharset, aTryLocaleCharset, aLang, _retval)
        }


        /// ```text
        /// /**
        ///    * Given the value of a header field parameter using the encoding
        ///    * defined in RFC 5987, decode the value into a Unicode string, and extract
        ///    * the optional language parameter.
        ///    *
        ///    * <p>
        ///    * This function is purposefully picky; it will abort for all (most?)
        ///    * invalid inputs. This is by design. In particular, it does not support
        ///    * any character encodings other than UTF-8, in order not to promote
        ///    * non-interoperable usage.
        ///    *
        ///    * <p>
        ///    * Code that parses HTTP header fields (as opposed to MIME header fields)
        ///    * should use this function.
        ///    *
        ///    * @param  aParamVal         a header field parameter to decode.
        ///    * @param  aLang             will be set to the language part (possibly
            ///    *                           empty).
        ///    * @return the decoded parameter value.
        ///    */
        /// ```
        ///

        /// `AString decodeRFC5987Param (in ACString aParamVal, out ACString aLang);`
        #[inline]
        pub unsafe fn DecodeRFC5987Param(&self, aParamVal: *const ::nsstring::nsACString, aLang: *mut ::nsstring::nsACString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
            ((*self.vtable).DecodeRFC5987Param)(self, aParamVal, aLang, _retval)
        }


        /// ```text
        /// /**
        ///    * Given the value of a single header field  (such as
            ///    * Content-Disposition and Content-Type) and the name of a parameter
        ///    * (e.g. filename, name, charset), returns the value of the parameter
        ///    * after decoding RFC 2231-style encoding.
        ///    * <p>
        ///    * For <strong>internal use only</strong>. The only other place where
        ///    * this needs to be  invoked  is  |MimeHeaders_get_parameter| in
        ///    * mailnews/mime/src/mimehdrs.cpp defined as
        ///    * char * MimeHeaders_get_parameter (const char *header_value,
            ///    *                                   const char *parm_name,
            ///    *                                   char **charset, char **language)
        ///    *
        ///    * Otherwise, this method would have been made static.
        ///    *
        ///    * @param  aHeaderVal  a header string to get the value of a parameter from.
        ///    * @param  aParamName  the name of a MIME header parameter (e.g.
            ///    *                     filename, name, charset). If empty,  returns
        ///    *                     the first (possibly) _unnamed_ 'parameter'.
        ///    * @param  aCharset    If non-null, it gets assigned a new pointer
        ///    *                     to a string containing the value of charset obtained
        ///    *                     from RFC 2231 parsing. Caller has to free it.
        ///    * @param  aLang       If non-null, it gets assigned a new pointer
        ///    *                     to a string containing the value of language obtained
        ///    *                     from RFC 2231 parsing. Caller has to free it.
        ///    * @return             the value of <code>aParamName</code> after
        ///    *                     RFC 2231 decoding but without charset conversion.
        ///    */
        /// ```
        ///

        /// `[noscript] string getParameterInternal (in string aHeaderVal, in string aParamName, out string aCharset, out string aLang);`
        #[inline]
        pub unsafe fn GetParameterInternal(&self, aHeaderVal: *const libc::c_char, aParamName: *const libc::c_char, aCharset: *mut *const libc::c_char, aLang: *mut *const libc::c_char, _retval: *mut *const libc::c_char) -> ::nserror::nsresult {
            ((*self.vtable).GetParameterInternal)(self, aHeaderVal, aParamName, aCharset, aLang, _retval)
        }


        /// ```text
        /// /**
        ///    * Given a header value, decodes RFC 2047-style encoding and
        ///    * returns the decoded header value in UTF-8 if either it's
        ///    * RFC-2047-encoded or aDefaultCharset is given. Otherwise,
        ///    * returns the input header value (in whatever encoding)
        ///    * as it is except that  RFC 822 (using backslash) quotation and
        ///    * CRLF (if aEatContinuation is set) are stripped away
        ///    * <p>
        ///    * For internal use only. The only other place where this needs to be
        ///    * invoked  is  <code>MIME_DecodeMimeHeader</code> in
        ///    * mailnews/mime/src/mimehdrs.cpp defined as
        ///    * char * Mime_DecodeMimeHeader(char *header_val, const char *charset,
            ///    *                              bool override, bool eatcontinuation)
        ///    *
        ///    * @param aHeaderVal       a header value to decode
        ///    * @param aDefaultCharset  MIME charset to use in place of MIME charset
        ///    *                         specified in RFC 2047 style encoding
        ///    *                         when <code>aOverrideCharset</code> is set.
        ///    * @param aOverrideCharset When set, overrides MIME charset specified
        ///    *                         in RFC 2047 style encoding with <code>aDefaultCharset</code>
        ///    * @param aEatContinuation When set, removes CR/LF
        ///    * @return                 decoded header value
        ///    */
        /// ```
        ///

        /// `[noscript] ACString decodeRFC2047Header (in string aHeaderVal, in string aDefaultCharset, in boolean aOverrideCharset, in boolean aEatContinuation);`
        #[inline]
        pub unsafe fn DecodeRFC2047Header(&self, aHeaderVal: *const libc::c_char, aDefaultCharset: *const libc::c_char, aOverrideCharset: bool, aEatContinuation: bool, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
            ((*self.vtable).DecodeRFC2047Header)(self, aHeaderVal, aDefaultCharset, aOverrideCharset, aEatContinuation, _retval)
        }


        /// ```text
        /// /**
        ///    * Given a header parameter, decodes RFC 2047 style encoding (if it's
            ///    * not obtained from RFC 2231 encoding),  converts it to
        ///    * UTF-8 and returns the result in UTF-8 if an attempt to extract
        ///    * charset info. from a few different sources succeeds.
        ///    * Otherwise,  returns the input header value (in whatever encoding)
        ///    * as it is except that  RFC 822 (using backslash) quotation is
        ///    * stripped off.
        ///    * <p>
        ///    * For internal use only. The only other place where this needs to be
        ///    * invoked  is  <code>mime_decode_filename</code> in
        ///    * mailnews/mime/src/mimehdrs.cpp defined as
        ///    * char * mime_decode_filename(char *name, const char *charset,
            ///    *                             MimeDisplayOptions *opt)
        ///    *
        ///    * @param aParamValue      the value of a parameter to decode and convert
        ///    * @param aCharset         charset obtained from RFC 2231 decoding  in which
        ///    *                         <code>aParamValue</code> is encoded. If null,
        ///    *                         indicates that it needs to try RFC 2047, instead.
        ///    * @param aDefaultCharset  MIME charset to use when aCharset is null and
        ///    *                         cannot be obtained per RFC 2047 (most likely
            ///    *                         because 'bare' string is  used.)  Besides, it
        ///    *                         overrides aCharset/MIME charset obtained from
        ///    *                         RFC 2047 if <code>aOverrideCharset</code>  is set.
        ///    * @param aOverrideCharset When set, overrides MIME charset specified
        ///    *                         in RFC 2047 style encoding with
        ///    *                         <code>aDefaultCharset</code>
        ///    * @return                 decoded parameter
        ///    */
        /// ```
        ///

        /// `[noscript] ACString decodeParameter (in ACString aParamValue, in string aCharset, in string aDefaultCharset, in boolean aOverrideCharset);`
        #[inline]
        pub unsafe fn DecodeParameter(&self, aParamValue: *const ::nsstring::nsACString, aCharset: *const libc::c_char, aDefaultCharset: *const libc::c_char, aOverrideCharset: bool, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
            ((*self.vtable).DecodeParameter)(self, aParamValue, aCharset, aDefaultCharset, aOverrideCharset, _retval)
        }


    }


