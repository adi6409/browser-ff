//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/webbrowserpersist/nsIWebBrowserPersist.idl
//


/// `interface nsIWebBrowserPersist : nsICancelable`
///

/// ```text
/// /**
///  * Interface for persisting DOM documents and URIs to local or remote storage.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWebBrowserPersist {
    vtable: *const nsIWebBrowserPersistVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWebBrowserPersist.
unsafe impl XpCom for nsIWebBrowserPersist {
    const IID: nsIID = nsID(0x8cd752a4, 0x60b1, 0x42c3,
        [0xa8, 0x19, 0x65, 0xc7, 0xa1, 0x13, 0x8a, 0x28]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWebBrowserPersist {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWebBrowserPersist.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWebBrowserPersistCoerce {
    /// Cheaply cast a value of this type from a `nsIWebBrowserPersist`.
    fn coerce_from(v: &nsIWebBrowserPersist) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWebBrowserPersistCoerce for nsIWebBrowserPersist {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserPersist) -> &Self {
        v
    }
}

impl nsIWebBrowserPersist {
    /// Cast this `nsIWebBrowserPersist` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWebBrowserPersistCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWebBrowserPersist {
    type Target = nsICancelable;
    #[inline]
    fn deref(&self) -> &nsICancelable {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsICancelableCoerce> nsIWebBrowserPersistCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserPersist) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWebBrowserPersist
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWebBrowserPersistVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsICancelableVTable,

    /* attribute unsigned long persistFlags; */
    pub GetPersistFlags: unsafe extern "system" fn (this: *const nsIWebBrowserPersist, aPersistFlags: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long persistFlags; */
    pub SetPersistFlags: unsafe extern "system" fn (this: *const nsIWebBrowserPersist, aPersistFlags: u32) -> ::nserror::nsresult,

    /* readonly attribute unsigned long currentState; */
    pub GetCurrentState: unsafe extern "system" fn (this: *const nsIWebBrowserPersist, aCurrentState: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute nsresult result; */
    pub GetResult: unsafe extern "system" fn (this: *const nsIWebBrowserPersist, aResult: *mut ::nserror::nsresult) -> ::nserror::nsresult,

    /* attribute nsIWebProgressListener progressListener; */
    pub GetProgressListener: unsafe extern "system" fn (this: *const nsIWebBrowserPersist, aProgressListener: *mut*const nsIWebProgressListener) -> ::nserror::nsresult,

    /* attribute nsIWebProgressListener progressListener; */
    pub SetProgressListener: unsafe extern "system" fn (this: *const nsIWebBrowserPersist, aProgressListener: *const nsIWebProgressListener) -> ::nserror::nsresult,

    /* void saveURI (in nsIURI aURI, in nsIPrincipal aTriggeringPrincipal, in unsigned long aCacheKey, in nsIReferrerInfo aReferrerInfo, in nsICookieJarSettings aCookieJarSettings, in nsIInputStream aPostData, in string aExtraHeaders, in nsISupports aFile, in nsContentPolicyType aContentPolicyType, in nsILoadContext aPrivacyContext); */
    pub SaveURI: unsafe extern "system" fn (this: *const nsIWebBrowserPersist, aURI: *const nsIURI, aTriggeringPrincipal: *const nsIPrincipal, aCacheKey: u32, aReferrerInfo: *const nsIReferrerInfo, aCookieJarSettings: *const nsICookieJarSettings, aPostData: *const nsIInputStream, aExtraHeaders: *const libc::c_char, aFile: *const nsISupports, aContentPolicyType: nsContentPolicyType, aPrivacyContext: *const nsILoadContext) -> ::nserror::nsresult,

    /* void savePrivacyAwareURI (in nsIURI aURI, in nsIPrincipal aTriggeringPrincipal, in unsigned long aCacheKey, in nsIReferrerInfo aReferrerInfo, in nsICookieJarSettings aCookieJarSettings, in nsIInputStream aPostData, in string aExtraHeaders, in nsISupports aFile, in nsContentPolicyType aContentPolicyType, in boolean aIsPrivate); */
    pub SavePrivacyAwareURI: unsafe extern "system" fn (this: *const nsIWebBrowserPersist, aURI: *const nsIURI, aTriggeringPrincipal: *const nsIPrincipal, aCacheKey: u32, aReferrerInfo: *const nsIReferrerInfo, aCookieJarSettings: *const nsICookieJarSettings, aPostData: *const nsIInputStream, aExtraHeaders: *const libc::c_char, aFile: *const nsISupports, aContentPolicyType: nsContentPolicyType, aIsPrivate: bool) -> ::nserror::nsresult,

    /* void saveChannel (in nsIChannel aChannel, in nsISupports aFile); */
    pub SaveChannel: unsafe extern "system" fn (this: *const nsIWebBrowserPersist, aChannel: *const nsIChannel, aFile: *const nsISupports) -> ::nserror::nsresult,

    /* void saveDocument (in nsISupports aDocument, in nsISupports aFile, in nsISupports aDataPath, in string aOutputContentType, in unsigned long aEncodingFlags, in unsigned long aWrapColumn); */
    pub SaveDocument: unsafe extern "system" fn (this: *const nsIWebBrowserPersist, aDocument: *const nsISupports, aFile: *const nsISupports, aDataPath: *const nsISupports, aOutputContentType: *const libc::c_char, aEncodingFlags: u32, aWrapColumn: u32) -> ::nserror::nsresult,

    /* void cancelSave (); */
    pub CancelSave: unsafe extern "system" fn (this: *const nsIWebBrowserPersist) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWebBrowserPersist {
    /// ```text
    /// /** No special persistence behaviour. */
    /// ```
    ///

    pub const PERSIST_FLAGS_NONE: i64 = 0;

    /// ```text
    /// /** Use cached data if present (skipping validation), else load from network */
    /// ```
    ///

    pub const PERSIST_FLAGS_FROM_CACHE: i64 = 1;

    /// ```text
    /// /** Bypass the cached data. */
    /// ```
    ///

    pub const PERSIST_FLAGS_BYPASS_CACHE: i64 = 2;

    /// ```text
    /// /** Ignore any redirected data (usually adverts). */
    /// ```
    ///

    pub const PERSIST_FLAGS_IGNORE_REDIRECTED_DATA: i64 = 4;

    /// ```text
    /// /** Ignore IFRAME content (usually adverts). */
    /// ```
    ///

    pub const PERSIST_FLAGS_IGNORE_IFRAMES: i64 = 8;

    /// ```text
    /// /** Do not run the incoming data through a content converter e.g. to decompress it */
    /// ```
    ///

    pub const PERSIST_FLAGS_NO_CONVERSION: i64 = 16;

    /// ```text
    /// /** Replace existing files on the disk (use with due diligence!) */
    /// ```
    ///

    pub const PERSIST_FLAGS_REPLACE_EXISTING_FILES: i64 = 32;

    /// ```text
    /// /** Don't modify or add base tags */
    /// ```
    ///

    pub const PERSIST_FLAGS_NO_BASE_TAG_MODIFICATIONS: i64 = 64;

    /// ```text
    /// /** Make changes to original dom rather than cloning nodes */
    /// ```
    ///

    pub const PERSIST_FLAGS_FIXUP_ORIGINAL_DOM: i64 = 128;

    /// ```text
    /// /** Fix links relative to destination location (not origin) */
    /// ```
    ///

    pub const PERSIST_FLAGS_FIXUP_LINKS_TO_DESTINATION: i64 = 256;

    /// ```text
    /// /** Don't make any adjustments to links */
    /// ```
    ///

    pub const PERSIST_FLAGS_DONT_FIXUP_LINKS: i64 = 512;

    /// ```text
    /// /** Force serialization of output (one file at a time; not concurrent) */
    /// ```
    ///

    pub const PERSIST_FLAGS_SERIALIZE_OUTPUT: i64 = 1024;

    /// ```text
    /// /** Don't make any adjustments to filenames */
    /// ```
    ///

    pub const PERSIST_FLAGS_DONT_CHANGE_FILENAMES: i64 = 2048;

    /// ```text
    /// /** Fail on broken inline links */
    /// ```
    ///

    pub const PERSIST_FLAGS_FAIL_ON_BROKEN_LINKS: i64 = 4096;

    /// ```text
    /// /**
    ///    * Automatically cleanup after a failed or cancelled operation, deleting all
    ///    * created files and directories. This flag does nothing for failed upload
    ///    * operations to remote servers.
    ///    */
    /// ```
    ///

    pub const PERSIST_FLAGS_CLEANUP_ON_FAILURE: i64 = 8192;

    /// ```text
    /// /**
    ///    * Let the WebBrowserPersist decide whether the incoming data is encoded
    ///    * and whether it needs to go through a content converter e.g. to
    ///    * decompress it.
    ///    */
    /// ```
    ///

    pub const PERSIST_FLAGS_AUTODETECT_APPLY_CONVERSION: i64 = 16384;

    /// ```text
    /// /**
    ///    * Append the downloaded data to the target file.
    ///    * This can only be used when persisting to a local file.
    ///    */
    /// ```
    ///

    pub const PERSIST_FLAGS_APPEND_TO_FILE: i64 = 32768;

    /// ```text
    /// /** Persister is ready to save data */
    /// ```
    ///

    pub const PERSIST_STATE_READY: i64 = 1;

    /// ```text
    /// /** Persister is saving data */
    /// ```
    ///

    pub const PERSIST_STATE_SAVING: i64 = 2;

    /// ```text
    /// /** Persister has finished saving data */
    /// ```
    ///

    pub const PERSIST_STATE_FINISHED: i64 = 3;

    /// ```text
    /// /** Output only the current selection as opposed to the whole document. */
    /// ```
    ///

    pub const ENCODE_FLAGS_SELECTION_ONLY: i64 = 1;

    /// ```text
    /// /**
    ///    * For plaintext output. Convert html to plaintext that looks like the html.
    ///    * Implies wrap (except inside &lt;pre&gt;), since html wraps.
    ///    * HTML output: always do prettyprinting, ignoring existing formatting.
    ///    */
    /// ```
    ///

    pub const ENCODE_FLAGS_FORMATTED: i64 = 2;

    /// ```text
    /// /**
    ///    * Output without formatting or wrapping the content. This flag
    ///    * may be used to preserve the original formatting as much as possible.
    ///    */
    /// ```
    ///

    pub const ENCODE_FLAGS_RAW: i64 = 4;

    /// ```text
    /// /** Output only the body section, no HTML tags. */
    /// ```
    ///

    pub const ENCODE_FLAGS_BODY_ONLY: i64 = 8;

    /// ```text
    /// /** Wrap even if when not doing formatted output (e.g. for text fields). */
    /// ```
    ///

    pub const ENCODE_FLAGS_PREFORMATTED: i64 = 16;

    /// ```text
    /// /** Wrap documents at the specified column. */
    /// ```
    ///

    pub const ENCODE_FLAGS_WRAP: i64 = 32;

    /// ```text
    /// /**
    ///    * For plaintext output. Output for format flowed (RFC 2646). This is used
    ///    * when converting to text for mail sending. This differs just slightly
    ///    * but in an important way from normal formatted, and that is that
    ///    * lines are space stuffed. This can't (correctly) be done later.
    ///    */
    /// ```
    ///

    pub const ENCODE_FLAGS_FORMAT_FLOWED: i64 = 64;

    /// ```text
    /// /** Convert links to absolute links where possible. */
    /// ```
    ///

    pub const ENCODE_FLAGS_ABSOLUTE_LINKS: i64 = 128;

    /// ```text
    /// /**
    ///    * Output with carriage return line breaks. May also be combined with
    ///    * ENCODE_FLAGS_LF_LINEBREAKS and if neither is specified, the platform
    ///    * default format is used.
    ///    */
    /// ```
    ///

    pub const ENCODE_FLAGS_CR_LINEBREAKS: i64 = 512;

    /// ```text
    /// /**
    ///    * Output with linefeed line breaks. May also be combined with
    ///    * ENCODE_FLAGS_CR_LINEBREAKS and if neither is specified, the platform
    ///    * default format is used.
    ///    */
    /// ```
    ///

    pub const ENCODE_FLAGS_LF_LINEBREAKS: i64 = 1024;

    /// ```text
    /// /** For plaintext output. Output the content of noscript elements. */
    /// ```
    ///

    pub const ENCODE_FLAGS_NOSCRIPT_CONTENT: i64 = 2048;

    /// ```text
    /// /** For plaintext output. Output the content of noframes elements. */
    /// ```
    ///

    pub const ENCODE_FLAGS_NOFRAMES_CONTENT: i64 = 4096;

    /// ```text
    /// /**
    ///    * Encode basic entities, e.g. output &nbsp; instead of character code 0xa0.
    ///    * The basic set is just &nbsp; &amp; &lt; &gt; &quot; for interoperability
    ///    * with older products that don't support &alpha; and friends.
    ///    */
    /// ```
    ///

    pub const ENCODE_FLAGS_ENCODE_BASIC_ENTITIES: i64 = 8192;

    /// ```text
    /// /**
    ///    * Flags governing how data is fetched and saved from the network.
    ///    * It is best to set this value explicitly unless you are prepared
    ///    * to accept the default values.
    ///    */
    /// ```
    ///

    /// `attribute unsigned long persistFlags;`
    #[inline]
    pub unsafe fn GetPersistFlags(&self, aPersistFlags: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetPersistFlags)(self, aPersistFlags)
    }


    /// ```text
    /// /**
    ///    * Flags governing how data is fetched and saved from the network.
    ///    * It is best to set this value explicitly unless you are prepared
    ///    * to accept the default values.
    ///    */
    /// ```
    ///

    /// `attribute unsigned long persistFlags;`
    #[inline]
    pub unsafe fn SetPersistFlags(&self, aPersistFlags: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetPersistFlags)(self, aPersistFlags)
    }


    /// ```text
    /// /**
    ///    * Current state of the persister object.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long currentState;`
    #[inline]
    pub unsafe fn GetCurrentState(&self, aCurrentState: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetCurrentState)(self, aCurrentState)
    }


    /// ```text
    /// /**
    ///    * Value indicating the success or failure of the persist
    ///    * operation.
    ///    *
    ///    * @throws NS_BINDING_ABORTED Operation cancelled.
    ///    * @throws NS_ERROR_FAILURE Non-specific failure.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsresult result;`
    #[inline]
    pub unsafe fn GetResult(&self, aResult: *mut ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).GetResult)(self, aResult)
    }


    /// ```text
    /// /**
    ///    * Callback listener for progress notifications. The object that the
    ///    * embbedder supplies may also implement nsIInterfaceRequestor and be
    ///    * prepared to return nsIAuthPrompt or other interfaces that may be required
    ///    * to download data.
    ///    *
    ///    * @see nsIAuthPrompt
    ///    * @see nsIInterfaceRequestor
    ///    */
    /// ```
    ///

    /// `attribute nsIWebProgressListener progressListener;`
    #[inline]
    pub unsafe fn GetProgressListener(&self, aProgressListener: *mut*const nsIWebProgressListener) -> ::nserror::nsresult {
        ((*self.vtable).GetProgressListener)(self, aProgressListener)
    }


    /// ```text
    /// /**
    ///    * Callback listener for progress notifications. The object that the
    ///    * embbedder supplies may also implement nsIInterfaceRequestor and be
    ///    * prepared to return nsIAuthPrompt or other interfaces that may be required
    ///    * to download data.
    ///    *
    ///    * @see nsIAuthPrompt
    ///    * @see nsIInterfaceRequestor
    ///    */
    /// ```
    ///

    /// `attribute nsIWebProgressListener progressListener;`
    #[inline]
    pub unsafe fn SetProgressListener(&self, aProgressListener: *const nsIWebProgressListener) -> ::nserror::nsresult {
        ((*self.vtable).SetProgressListener)(self, aProgressListener)
    }


    /// ```text
    /// /**
    ///    * Save the specified URI to file.
    ///    *
    ///    * @param aURI       URI to save to file. Some implementations of this interface
    ///    *                   may also support <CODE>nullptr</CODE> to imply the currently
    ///    *                   loaded URI.
    ///    * @param aTriggeringPrincipal
    ///    *                   The triggering principal for the URI we're saving.
    ///    * @param aCacheKey  The necko cache key integer.
    ///    * @param aReferrerInfo  The referrer info for compute and send referrer via
    ///    *                   HTTP Referer header.
    ///    * @param aCookieJarSettings The cookieJarSettings for the HTTP channel which
    ///    *                   is saving the URI.
    ///    * @param aPostData  Post data to pass with an HTTP request or
    ///    *                   <CODE>nullptr</CODE>.
    ///    * @param aExtraHeaders Additional headers to supply with an HTTP request
    ///    *                   or <CODE>nullptr</CODE>.
    ///    * @param aFile      Target file. This may be a nsIFile object or an
    ///    *                   nsIURI object with a file scheme or a scheme that
    ///    *                   supports uploading (e.g. ftp).
    ///    * @param aContentPolicyType The type of content we're saving.
    ///    * @param aPrivacyContext A context from which the privacy status of this
    ///    *                   save operation can be determined. Must only be null
    ///    *                   in situations in which no such context is available
    ///    *                   (eg. the operation has no logical association with any
        ///    *                   window or document)
    ///    *
    ///    * @see nsIFile
    ///    * @see nsIURI
    ///    * @see nsIInputStream
    ///    *
    ///    * @throws NS_ERROR_INVALID_ARG One or more arguments was invalid.
    ///    */
    /// ```
    ///

    /// `void saveURI (in nsIURI aURI, in nsIPrincipal aTriggeringPrincipal, in unsigned long aCacheKey, in nsIReferrerInfo aReferrerInfo, in nsICookieJarSettings aCookieJarSettings, in nsIInputStream aPostData, in string aExtraHeaders, in nsISupports aFile, in nsContentPolicyType aContentPolicyType, in nsILoadContext aPrivacyContext);`
    #[inline]
    pub unsafe fn SaveURI(&self, aURI: *const nsIURI, aTriggeringPrincipal: *const nsIPrincipal, aCacheKey: u32, aReferrerInfo: *const nsIReferrerInfo, aCookieJarSettings: *const nsICookieJarSettings, aPostData: *const nsIInputStream, aExtraHeaders: *const libc::c_char, aFile: *const nsISupports, aContentPolicyType: nsContentPolicyType, aPrivacyContext: *const nsILoadContext) -> ::nserror::nsresult {
        ((*self.vtable).SaveURI)(self, aURI, aTriggeringPrincipal, aCacheKey, aReferrerInfo, aCookieJarSettings, aPostData, aExtraHeaders, aFile, aContentPolicyType, aPrivacyContext)
    }


    /// ```text
    /// /**
    ///    * @param aIsPrivate Treat the save operation as private (ie. with
        ///    *                   regards to networking operations and persistence
        ///    *                   of intermediate data, etc.)
    ///    * @see saveURI for all other parameter descriptions
    ///    */
    /// ```
    ///

    /// `void savePrivacyAwareURI (in nsIURI aURI, in nsIPrincipal aTriggeringPrincipal, in unsigned long aCacheKey, in nsIReferrerInfo aReferrerInfo, in nsICookieJarSettings aCookieJarSettings, in nsIInputStream aPostData, in string aExtraHeaders, in nsISupports aFile, in nsContentPolicyType aContentPolicyType, in boolean aIsPrivate);`
    #[inline]
    pub unsafe fn SavePrivacyAwareURI(&self, aURI: *const nsIURI, aTriggeringPrincipal: *const nsIPrincipal, aCacheKey: u32, aReferrerInfo: *const nsIReferrerInfo, aCookieJarSettings: *const nsICookieJarSettings, aPostData: *const nsIInputStream, aExtraHeaders: *const libc::c_char, aFile: *const nsISupports, aContentPolicyType: nsContentPolicyType, aIsPrivate: bool) -> ::nserror::nsresult {
        ((*self.vtable).SavePrivacyAwareURI)(self, aURI, aTriggeringPrincipal, aCacheKey, aReferrerInfo, aCookieJarSettings, aPostData, aExtraHeaders, aFile, aContentPolicyType, aIsPrivate)
    }


    /// ```text
    /// /**
    ///    * Save a channel to a file. It must not be opened yet.
    ///    * @see saveURI
    ///    */
    /// ```
    ///

    /// `void saveChannel (in nsIChannel aChannel, in nsISupports aFile);`
    #[inline]
    pub unsafe fn SaveChannel(&self, aChannel: *const nsIChannel, aFile: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).SaveChannel)(self, aChannel, aFile)
    }


    /// ```text
    /// /**
    ///    * Save the specified DOM document to file and optionally all linked files
    ///    * (e.g. images, CSS, JS & subframes). Do not call this method until the
    ///    * document has finished loading!
    ///    *
    ///    * @param aDocument          Document to save to file. Some implementations of
    ///    *                           this interface may also support <CODE>nullptr</CODE>
    ///    *                           to imply the currently loaded document.  Can be an
    ///    *                           nsIWebBrowserPersistDocument or Document.
    ///    * @param aFile              Target local file. This may be a nsIFile object or an
    ///    *                           nsIURI object with a file scheme or a scheme that
    ///    *                           supports uploading (e.g. ftp).
    ///    * @param aDataPath          Path to directory where URIs linked to the document
    ///    *                           are saved or nullptr if no linked URIs should be saved.
    ///    *                           This may be a nsIFile object or an nsIURI object
    ///    *                           with a file scheme.
    ///    * @param aOutputContentType The desired MIME type format to save the
    ///    *                           document and all subdocuments into or nullptr to use
    ///    *                           the default behaviour.
    ///    * @param aEncodingFlags     Flags to pass to the encoder.
    ///    * @param aWrapColumn        For text documents, indicates the desired width to
    ///    *                           wrap text at. Parameter is ignored if wrapping is not
    ///    *                           specified by the encoding flags.
    ///    *
    ///    * @see nsIWebBrowserPersistDocument
    ///    * @see WebBrowserPersistable
    ///    * @see nsIFile
    ///    * @see nsIURI
    ///    *
    ///    * @throws NS_ERROR_INVALID_ARG One or more arguments was invalid.
    ///    */
    /// ```
    ///

    /// `void saveDocument (in nsISupports aDocument, in nsISupports aFile, in nsISupports aDataPath, in string aOutputContentType, in unsigned long aEncodingFlags, in unsigned long aWrapColumn);`
    #[inline]
    pub unsafe fn SaveDocument(&self, aDocument: *const nsISupports, aFile: *const nsISupports, aDataPath: *const nsISupports, aOutputContentType: *const libc::c_char, aEncodingFlags: u32, aWrapColumn: u32) -> ::nserror::nsresult {
        ((*self.vtable).SaveDocument)(self, aDocument, aFile, aDataPath, aOutputContentType, aEncodingFlags, aWrapColumn)
    }


    /// ```text
    /// /**
    ///    * Cancels the current operation. The caller is responsible for cleaning up
    ///    * partially written files or directories. This has the same effect as calling
    ///    * cancel with an argument of NS_BINDING_ABORTED.
    ///    */
    /// ```
    ///

    /// `void cancelSave ();`
    #[inline]
    pub unsafe fn CancelSave(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).CancelSave)(self, )
    }


}


