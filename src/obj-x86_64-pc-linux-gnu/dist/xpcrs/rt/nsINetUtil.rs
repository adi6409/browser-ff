//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsINetUtil.idl
//


/// `interface nsINetUtil : nsISupports`
///

/// ```text
/// /**
///  * nsINetUtil provides various network-related utility methods.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINetUtil {
    vtable: *const nsINetUtilVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINetUtil.
unsafe impl XpCom for nsINetUtil {
    const IID: nsIID = nsID(0xfe2625ec, 0xb884, 0x4df1,
        [0xb3, 0x9c, 0x9e, 0x83, 0x0e, 0x47, 0xaa, 0x94]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINetUtil {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINetUtil.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINetUtilCoerce {
    /// Cheaply cast a value of this type from a `nsINetUtil`.
    fn coerce_from(v: &nsINetUtil) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINetUtilCoerce for nsINetUtil {
    #[inline]
    fn coerce_from(v: &nsINetUtil) -> &Self {
        v
    }
}

impl nsINetUtil {
    /// Cast this `nsINetUtil` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINetUtilCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINetUtil {
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
impl<T: nsISupportsCoerce> nsINetUtilCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINetUtil) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINetUtil
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINetUtilVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* AUTF8String parseRequestContentType (in AUTF8String aTypeHeader, out AUTF8String aCharset, out boolean aHadCharset); */
    pub ParseRequestContentType: unsafe extern "system" fn (this: *const nsINetUtil, aTypeHeader: *const ::nsstring::nsACString, aCharset: *mut ::nsstring::nsACString, aHadCharset: *mut bool, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* AUTF8String parseResponseContentType (in AUTF8String aTypeHeader, out AUTF8String aCharset, out boolean aHadCharset); */
    pub ParseResponseContentType: unsafe extern "system" fn (this: *const nsINetUtil, aTypeHeader: *const ::nsstring::nsACString, aCharset: *mut ::nsstring::nsACString, aHadCharset: *mut bool, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* boolean protocolHasFlags (in nsIURI aURI, in unsigned long aFlag); */
    pub ProtocolHasFlags: unsafe extern "system" fn (this: *const nsINetUtil, aURI: *const nsIURI, aFlag: u32, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean URIChainHasFlags (in nsIURI aURI, in unsigned long aFlags); */
    pub URIChainHasFlags: unsafe extern "system" fn (this: *const nsINetUtil, aURI: *const nsIURI, aFlags: u32, _retval: *mut bool) -> ::nserror::nsresult,

    /* ACString escapeString (in ACString aString, in unsigned long aEscapeType); */
    pub EscapeString: unsafe extern "system" fn (this: *const nsINetUtil, aString: *const ::nsstring::nsACString, aEscapeType: u32, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* ACString escapeURL (in ACString aStr, in unsigned long aFlags); */
    pub EscapeURL: unsafe extern "system" fn (this: *const nsINetUtil, aStr: *const ::nsstring::nsACString, aFlags: u32, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* ACString unescapeString (in AUTF8String aStr, in unsigned long aFlags); */
    pub UnescapeString: unsafe extern "system" fn (this: *const nsINetUtil, aStr: *const ::nsstring::nsACString, aFlags: u32, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* boolean extractCharsetFromContentType (in AUTF8String aTypeHeader, out AUTF8String aCharset, out long aCharsetStart, out long aCharsetEnd); */
    pub ExtractCharsetFromContentType: unsafe extern "system" fn (this: *const nsINetUtil, aTypeHeader: *const ::nsstring::nsACString, aCharset: *mut ::nsstring::nsACString, aCharsetStart: *mut i32, aCharsetEnd: *mut i32, _retval: *mut bool) -> ::nserror::nsresult,

    /* void socketProcessTelemetryPing (); */
    pub SocketProcessTelemetryPing: unsafe extern "system" fn (this: *const nsINetUtil) -> ::nserror::nsresult,

    /* void notImplemented (); */
    pub NotImplemented: unsafe extern "system" fn (this: *const nsINetUtil) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINetUtil {
    /// ```text
    /// /** Escape every character with its %XX-escaped equivalent */
    /// ```
    ///

    pub const ESCAPE_ALL: i64 = 0;

    /// ```text
    /// /** Leave alphanumeric characters intact and %XX-escape all others */
    /// ```
    ///

    pub const ESCAPE_XALPHAS: i64 = 1;

    /// ```text
    /// /** Leave alphanumeric characters intact, convert spaces to '+',
    ///       %XX-escape all others */
    /// ```
    ///

    pub const ESCAPE_XPALPHAS: i64 = 2;

    /// ```text
    /// /** Leave alphanumeric characters and forward slashes intact,
    ///       %XX-escape all others */
    /// ```
    ///

    pub const ESCAPE_URL_PATH: i64 = 4;

    /// ```text
    /// /** %XX-escape URL scheme */
    /// ```
    ///

    pub const ESCAPE_URL_SCHEME: i64 = 1;

    /// ```text
    /// /** %XX-escape username in the URL */
    /// ```
    ///

    pub const ESCAPE_URL_USERNAME: i64 = 2;

    /// ```text
    /// /** %XX-escape password in the URL */
    /// ```
    ///

    pub const ESCAPE_URL_PASSWORD: i64 = 4;

    /// ```text
    /// /** %XX-escape URL host */
    /// ```
    ///

    pub const ESCAPE_URL_HOST: i64 = 8;

    /// ```text
    /// /** %XX-escape URL directory */
    /// ```
    ///

    pub const ESCAPE_URL_DIRECTORY: i64 = 16;

    /// ```text
    /// /** %XX-escape file basename in the URL */
    /// ```
    ///

    pub const ESCAPE_URL_FILE_BASENAME: i64 = 32;

    /// ```text
    /// /** %XX-escape file extension in the URL */
    /// ```
    ///

    pub const ESCAPE_URL_FILE_EXTENSION: i64 = 64;

    /// ```text
    /// /** %XX-escape URL parameters */
    /// ```
    ///

    pub const ESCAPE_URL_PARAM: i64 = 128;

    /// ```text
    /// /** %XX-escape URL query */
    /// ```
    ///

    pub const ESCAPE_URL_QUERY: i64 = 256;

    /// ```text
    /// /** %XX-escape URL ref */
    /// ```
    ///

    pub const ESCAPE_URL_REF: i64 = 512;

    /// ```text
    /// /** %XX-escape URL path - same as escaping directory, basename and extension */
    /// ```
    ///

    pub const ESCAPE_URL_FILEPATH: i64 = 112;

    /// ```text
    /// /** %XX-escape scheme, username, password, host, path, params, query and ref */
    /// ```
    ///

    pub const ESCAPE_URL_MINIMAL: i64 = 1023;

    /// ```text
    /// /** Force %XX-escaping of already escaped sequences */
    /// ```
    ///

    pub const ESCAPE_URL_FORCED: i64 = 1024;

    /// ```text
    /// /** Skip non-ascii octets, %XX-escape all others */
    /// ```
    ///

    pub const ESCAPE_URL_ONLY_ASCII: i64 = 2048;

    /// ```text
    /// /**
    ///    * Skip graphic octets (0x20-0x7E) when escaping
    ///    * Skips all ASCII octets (0x00-0x7F) when unescaping
    ///    */
    /// ```
    ///

    pub const ESCAPE_URL_ONLY_NONASCII: i64 = 4096;

    /// ```text
    /// /** Force %XX-escape of colon */
    /// ```
    ///

    pub const ESCAPE_URL_COLON: i64 = 16384;

    /// ```text
    /// /** Skip C0 and DEL from unescaping */
    /// ```
    ///

    pub const ESCAPE_URL_SKIP_CONTROL: i64 = 32768;

    /// ```text
    /// /**
    ///    * Parse a Content-Type header value in strict mode.  This is a more
    ///    * conservative parser that reject things that violate RFC 7231 section
    ///    * 3.1.1.1.  This is typically useful for parsing Content-Type header values
    ///    * that are used for HTTP requests, and those that are used to make security
    ///    * decisions.
    ///    *
    ///    * @param aTypeHeader the header string to parse
    ///    * @param [out] aCharset the charset parameter specified in the
    ///    *              header, if any.
    ///    * @param [out] aHadCharset whether a charset was explicitly specified.
    ///    * @return the MIME type specified in the header, in lower-case.
    ///    */
    /// ```
    ///

    /// `AUTF8String parseRequestContentType (in AUTF8String aTypeHeader, out AUTF8String aCharset, out boolean aHadCharset);`
    #[inline]
    pub unsafe fn ParseRequestContentType(&self, aTypeHeader: *const ::nsstring::nsACString, aCharset: *mut ::nsstring::nsACString, aHadCharset: *mut bool, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).ParseRequestContentType)(self, aTypeHeader, aCharset, aHadCharset, _retval)
    }


    /// ```text
    /// /**
    ///    * Parse a Content-Type header value in relaxed mode.  This is a more
    ///    * permissive parser that ignores things that go against RFC 7231 section
    ///    * 3.1.1.1.  This is typically useful for parsing Content-Type header values
    ///    * received from web servers where we want to make a best effort attempt
    ///    * at extracting a useful MIME type and charset.
    ///    *
    ///    * NOTE: DO NOT USE THIS if you're going to make security decisions
    ///    * based on the result.
    ///    *
    ///    * @param aTypeHeader the header string to parse
    ///    * @param [out] aCharset the charset parameter specified in the
    ///    *              header, if any.
    ///    * @param [out] aHadCharset whether a charset was explicitly specified.
    ///    * @return the MIME type specified in the header, in lower-case.
    ///    */
    /// ```
    ///

    /// `AUTF8String parseResponseContentType (in AUTF8String aTypeHeader, out AUTF8String aCharset, out boolean aHadCharset);`
    #[inline]
    pub unsafe fn ParseResponseContentType(&self, aTypeHeader: *const ::nsstring::nsACString, aCharset: *mut ::nsstring::nsACString, aHadCharset: *mut bool, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).ParseResponseContentType)(self, aTypeHeader, aCharset, aHadCharset, _retval)
    }


    /// ```text
    /// /**
    ///    * Test whether the given URI's handler has the given protocol flags.
    ///    *
    ///    * @param aURI the URI in question
    ///    * @param aFlags the flags we're testing for.
    ///    *
    ///    * @return whether the protocol handler for aURI has all the flags
    ///    *         in aFlags.
    ///    */
    /// ```
    ///

    /// `boolean protocolHasFlags (in nsIURI aURI, in unsigned long aFlag);`
    #[inline]
    pub unsafe fn ProtocolHasFlags(&self, aURI: *const nsIURI, aFlag: u32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ProtocolHasFlags)(self, aURI, aFlag, _retval)
    }


    /// ```text
    /// /**
    ///    * Test whether the protocol handler for this URI or that for any of
    ///    * its inner URIs has the given protocol flags.  This will QI aURI to
    ///    * nsINestedURI and walk the nested URI chain.
    ///    *
    ///    * @param aURI the URI in question
    ///    * @param aFlags the flags we're testing for.
    ///    *
    ///    * @return whether any of the protocol handlers involved have all the flags
    ///    *         in aFlags.
    ///    */
    /// ```
    ///

    /// `boolean URIChainHasFlags (in nsIURI aURI, in unsigned long aFlags);`
    #[inline]
    pub unsafe fn URIChainHasFlags(&self, aURI: *const nsIURI, aFlags: u32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).URIChainHasFlags)(self, aURI, aFlags, _retval)
    }


    /// ```text
    /// /**
    ///    * escape a string with %00-style escaping
    ///    */
    /// ```
    ///

    /// `ACString escapeString (in ACString aString, in unsigned long aEscapeType);`
    #[inline]
    pub unsafe fn EscapeString(&self, aString: *const ::nsstring::nsACString, aEscapeType: u32, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).EscapeString)(self, aString, aEscapeType, _retval)
    }


    /// ```text
    /// /**
    ///    * %XX-Escape invalid chars in a URL segment.
    ///    *
    ///    * @param aStr the URL to be escaped
    ///    * @param aFlags the URL segment type flags
    ///    *
    ///    * @return the escaped string (the string itself if escaping did not happen)
    ///    *
    ///    */
    /// ```
    ///

    /// `ACString escapeURL (in ACString aStr, in unsigned long aFlags);`
    #[inline]
    pub unsafe fn EscapeURL(&self, aStr: *const ::nsstring::nsACString, aFlags: u32, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).EscapeURL)(self, aStr, aFlags, _retval)
    }


    /// ```text
    /// /**
    ///    * Expands URL escape sequences
    ///    *
    ///    * @param aStr the URL to be unescaped
    ///    * @param aFlags only ESCAPE_URL_ONLY_NONASCII and ESCAPE_URL_SKIP_CONTROL
    ///    *               are recognized.  If |aFlags| is 0 all escape sequences are
    ///    *               unescaped
    ///    * @return unescaped string
    ///    */
    /// ```
    ///

    /// `ACString unescapeString (in AUTF8String aStr, in unsigned long aFlags);`
    #[inline]
    pub unsafe fn UnescapeString(&self, aStr: *const ::nsstring::nsACString, aFlags: u32, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).UnescapeString)(self, aStr, aFlags, _retval)
    }


    /// ```text
    /// /**
    ///    * Extract the charset parameter location and value from a content-type
    ///    * header.
    ///    *
    ///    * @param aTypeHeader the header string to parse
    ///    * @param [out] aCharset the charset parameter specified in the
    ///    *              header, if any.
    ///    * @param [out] aCharsetStart index of the start of the charset parameter
    ///    *              (the ';' separating it from what came before) in aTypeHeader.
    ///    *              If this function returns false, this argument will still be
    ///    *              set, to the index of the location where a new charset should
    ///    *              be inserted.
    ///    * @param [out] aCharsetEnd index of the end of the charset parameter (the
        ///    *              ';' separating it from what comes after, or the end
        ///    *              of the string) in aTypeHeader.  If this function returns
    ///    *              false, this argument will still be set, to the index of the
    ///    *              location where a new charset should be inserted.
    ///    *
    ///    * @return whether a charset parameter was found.  This can be false even in
    ///    * cases when parseContentType would claim to have a charset, if the type
    ///    * that won out does not have a charset parameter specified.
    ///    */
    /// ```
    ///

    /// `boolean extractCharsetFromContentType (in AUTF8String aTypeHeader, out AUTF8String aCharset, out long aCharsetStart, out long aCharsetEnd);`
    #[inline]
    pub unsafe fn ExtractCharsetFromContentType(&self, aTypeHeader: *const ::nsstring::nsACString, aCharset: *mut ::nsstring::nsACString, aCharsetStart: *mut i32, aCharsetEnd: *mut i32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ExtractCharsetFromContentType)(self, aTypeHeader, aCharset, aCharsetStart, aCharsetEnd, _retval)
    }


    /// ```text
    /// /**
    ///    * This is test-only. Send an IPC message to let socket process send a
    ///    * telemetry.
    ///    */
    /// ```
    ///

    /// `void socketProcessTelemetryPing ();`
    #[inline]
    pub unsafe fn SocketProcessTelemetryPing(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SocketProcessTelemetryPing)(self, )
    }


    /// ```text
    /// /**
    ///    * This is a void method that is C++ implemented and always
    ///    * returns NS_ERROR_NOT_IMPLEMENTED. To be used for testing.
    ///    */
    /// ```
    ///

    /// `void notImplemented ();`
    #[inline]
    pub unsafe fn NotImplemented(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).NotImplemented)(self, )
    }


}


