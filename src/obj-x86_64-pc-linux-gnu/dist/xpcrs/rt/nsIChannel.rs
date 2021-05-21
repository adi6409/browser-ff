//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIChannel.idl
//


/// `interface nsIChannel : nsIRequest`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIChannel {
    vtable: *const nsIChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIChannel.
unsafe impl XpCom for nsIChannel {
    const IID: nsIID = nsID(0x2c389865, 0x23db, 0x4aa7,
        [0x9f, 0xe5, 0x60, 0xcc, 0x7b, 0x00, 0x69, 0x7e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIChannel {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIChannel.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIChannelCoerce {
    /// Cheaply cast a value of this type from a `nsIChannel`.
    fn coerce_from(v: &nsIChannel) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIChannelCoerce for nsIChannel {
    #[inline]
    fn coerce_from(v: &nsIChannel) -> &Self {
        v
    }
}

impl nsIChannel {
    /// Cast this `nsIChannel` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIChannel {
    type Target = nsIRequest;
    #[inline]
    fn deref(&self) -> &nsIRequest {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIRequestCoerce> nsIChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIChannel) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIChannel
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIChannelVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIRequestVTable,

    /* attribute nsIURI originalURI; */
    pub GetOriginalURI: unsafe extern "system" fn (this: *const nsIChannel, aOriginalURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* attribute nsIURI originalURI; */
    pub SetOriginalURI: unsafe extern "system" fn (this: *const nsIChannel, aOriginalURI: *const nsIURI) -> ::nserror::nsresult,

    /* readonly attribute nsIURI URI; */
    pub GetURI: unsafe extern "system" fn (this: *const nsIChannel, aURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* attribute nsISupports owner; */
    pub GetOwner: unsafe extern "system" fn (this: *const nsIChannel, aOwner: *mut *const nsISupports) -> ::nserror::nsresult,

    /* attribute nsISupports owner; */
    pub SetOwner: unsafe extern "system" fn (this: *const nsIChannel, aOwner: *const nsISupports) -> ::nserror::nsresult,

    /* attribute nsIInterfaceRequestor notificationCallbacks; */
    pub GetNotificationCallbacks: unsafe extern "system" fn (this: *const nsIChannel, aNotificationCallbacks: *mut*const nsIInterfaceRequestor) -> ::nserror::nsresult,

    /* attribute nsIInterfaceRequestor notificationCallbacks; */
    pub SetNotificationCallbacks: unsafe extern "system" fn (this: *const nsIChannel, aNotificationCallbacks: *const nsIInterfaceRequestor) -> ::nserror::nsresult,

    /* readonly attribute nsISupports securityInfo; */
    pub GetSecurityInfo: unsafe extern "system" fn (this: *const nsIChannel, aSecurityInfo: *mut *const nsISupports) -> ::nserror::nsresult,

    /* attribute ACString contentType; */
    pub GetContentType: unsafe extern "system" fn (this: *const nsIChannel, aContentType: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute ACString contentType; */
    pub SetContentType: unsafe extern "system" fn (this: *const nsIChannel, aContentType: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute ACString contentCharset; */
    pub GetContentCharset: unsafe extern "system" fn (this: *const nsIChannel, aContentCharset: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute ACString contentCharset; */
    pub SetContentCharset: unsafe extern "system" fn (this: *const nsIChannel, aContentCharset: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute int64_t contentLength; */
    pub GetContentLength: unsafe extern "system" fn (this: *const nsIChannel, aContentLength: *mut int64_t) -> ::nserror::nsresult,

    /* attribute int64_t contentLength; */
    pub SetContentLength: unsafe extern "system" fn (this: *const nsIChannel, aContentLength: int64_t) -> ::nserror::nsresult,

    /* nsIInputStream open (); */
    pub Open: unsafe extern "system" fn (this: *const nsIChannel, _retval: *mut*const nsIInputStream) -> ::nserror::nsresult,

    /* void asyncOpen (in nsIStreamListener aListener); */
    pub AsyncOpen: unsafe extern "system" fn (this: *const nsIChannel, aListener: *const nsIStreamListener) -> ::nserror::nsresult,

    /* [must_use] readonly attribute boolean canceled; */
    pub GetCanceled: unsafe extern "system" fn (this: *const nsIChannel, aCanceled: *mut bool) -> ::nserror::nsresult,

    /* attribute unsigned long contentDisposition; */
    pub GetContentDisposition: unsafe extern "system" fn (this: *const nsIChannel, aContentDisposition: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long contentDisposition; */
    pub SetContentDisposition: unsafe extern "system" fn (this: *const nsIChannel, aContentDisposition: u32) -> ::nserror::nsresult,

    /* attribute AString contentDispositionFilename; */
    pub GetContentDispositionFilename: unsafe extern "system" fn (this: *const nsIChannel, aContentDispositionFilename: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString contentDispositionFilename; */
    pub SetContentDispositionFilename: unsafe extern "system" fn (this: *const nsIChannel, aContentDispositionFilename: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute ACString contentDispositionHeader; */
    pub GetContentDispositionHeader: unsafe extern "system" fn (this: *const nsIChannel, aContentDispositionHeader: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute nsILoadInfo loadInfo; */
    pub GetLoadInfo: unsafe extern "system" fn (this: *const nsIChannel, aLoadInfo: *mut *const nsILoadInfo) -> ::nserror::nsresult,

    /* attribute nsILoadInfo loadInfo; */
    pub SetLoadInfo: unsafe extern "system" fn (this: *const nsIChannel, aLoadInfo: *const nsILoadInfo) -> ::nserror::nsresult,

    /* readonly attribute bool isDocument; */
    pub GetIsDocument: unsafe extern "system" fn (this: *const nsIChannel, aIsDocument: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIChannel {
    /// ```text
    /// /**************************************************************************
    ///      * Channel specific load flags:
    ///      *
    ///      * Bits 16-31 are reserved for future use by this interface or one of its
    ///      * derivatives (e.g., see nsICachingChannel).
    ///      */
    /// /**
    ///      * Set (e.g., by the docshell) to indicate whether or not the channel
    ///      * corresponds to a document URI.
    ///      * While setting this flag is sufficient to mark a channel as a document
    ///      * load, _checking_ whether the channel is a document load requires the use
    ///      * of the new channel.isDocument
    ///      */
    /// ```
    ///

    pub const LOAD_DOCUMENT_URI: i64 = 65536;

    /// ```text
    /// /**
    ///      * If the end consumer for this load has been retargeted after discovering
    ///      * its content, this flag will be set:
    ///      */
    /// ```
    ///

    pub const LOAD_RETARGETED_DOCUMENT_URI: i64 = 131072;

    /// ```text
    /// /**
    ///      * This flag is set to indicate that this channel is replacing another
    ///      * channel.  This means that:
    ///      *
    ///      * 1) the stream listener this channel will be notifying was initially
///      *    passed to the asyncOpen method of some other channel
///      *
///      *   and
///      *
///      * 2) this channel's URI is a better identifier of the resource being
///      *    accessed than this channel's originalURI.
///      *
///      * This flag can be set, for example, for redirects or for cases when a
///      * single channel has multiple parts to it (and thus can follow
///      * onStopRequest with another onStartRequest/onStopRequest pair, each pair
///      * for a different request).
///      */
/// ```
///

pub const LOAD_REPLACE: i64 = 262144;

/// ```text
/// /**
///      * Set (e.g., by the docshell) to indicate whether or not the channel
///      * corresponds to an initial document URI load (e.g., link click).
///      */
/// ```
///

pub const LOAD_INITIAL_DOCUMENT_URI: i64 = 524288;

/// ```text
/// /**
///      * Set (e.g., by the URILoader) to indicate whether or not the end consumer
///      * for this load has been determined.
///      */
/// ```
///

pub const LOAD_TARGETED: i64 = 1048576;

/// ```text
/// /**
///      * If this flag is set, the channel should call the content sniffers as
///      * described in nsNetCID.h about NS_CONTENT_SNIFFER_CATEGORY.
///      *
///      * Note: Channels may ignore this flag; however, new channel implementations
///      * should only do so with good reason.
///      */
/// ```
///

pub const LOAD_CALL_CONTENT_SNIFFERS: i64 = 2097152;

/// ```text
/// /**
///      * This flag tells the channel to bypass URL classifier service check
///      * when opening the channel.
///      */
/// ```
///

pub const LOAD_BYPASS_URL_CLASSIFIER: i64 = 4194304;

/// ```text
/// /**
///      * If this flag is set, the media-type content sniffer will be allowed
///      * to override any server-set content-type. Otherwise it will only
///      * be allowed to override "no content type" and application/octet-stream.
///      */
/// ```
///

pub const LOAD_MEDIA_SNIFFER_OVERRIDES_CONTENT_TYPE: i64 = 8388608;

/// ```text
/// /**
///      * Set to let explicitely provided credentials be used over credentials
///      * we have cached previously. In some situations like form login using HTTP
///      * auth via XMLHttpRequest we need to let consumers override the cached
///      * credentials explicitely. For form login 403 response instead of 401 is
///      * usually used to prevent an auth dialog. But any code other then 401/7
///      * will leave original credentials in the cache and there is then no way
///      * to override them for the same user name.
///      */
/// ```
///

pub const LOAD_EXPLICIT_CREDENTIALS: i64 = 16777216;

/// ```text
/// /**
///      * Set to force bypass of any service worker interception of the channel.
///      */
/// ```
///

pub const LOAD_BYPASS_SERVICE_WORKER: i64 = 33554432;


pub const DISPOSITION_INLINE: i64 = 0;


pub const DISPOSITION_ATTACHMENT: i64 = 1;

/// ```text
/// /**
///  * The nsIChannel interface allows clients to construct "GET" requests for
///  * specific protocols, and manage them in a uniform way.  Once a channel is
///  * created (via nsIIOService::newChannel), parameters for that request may
///  * be set by using the channel attributes, or by QI'ing to a subclass of
///  * nsIChannel for protocol-specific parameters.  Then, the URI can be fetched
///  * by calling nsIChannel::open or nsIChannel::asyncOpen.
///  *
///  * After a request has been completed, the channel is still valid for accessing
///  * protocol-specific results.  For example, QI'ing to nsIHttpChannel allows
///  * response headers to be retrieved for the corresponding http transaction.
///  *
///  * This interface must be used only from the XPCOM main thread.
///  */
/// /**
///      * The original URI used to construct the channel. This is used in
///      * the case of a redirect or URI "resolution" (e.g. resolving a
///      * resource: URI to a file: URI) so that the original pre-redirect
///      * URI can still be obtained.  This is never null.  Attempts to
///      * set it to null must throw.
///      *
///      * NOTE: this is distinctly different from the http Referer (referring URI),
///      * which is typically the page that contained the original URI (accessible
///      * from nsIHttpChannel).
///      */
/// ```
///

/// `attribute nsIURI originalURI;`
#[inline]
pub unsafe fn GetOriginalURI(&self, aOriginalURI: *mut*const nsIURI) -> ::nserror::nsresult {
((*self.vtable).GetOriginalURI)(self, aOriginalURI)
}


/// ```text
/// /**
///  * The nsIChannel interface allows clients to construct "GET" requests for
///  * specific protocols, and manage them in a uniform way.  Once a channel is
///  * created (via nsIIOService::newChannel), parameters for that request may
///  * be set by using the channel attributes, or by QI'ing to a subclass of
///  * nsIChannel for protocol-specific parameters.  Then, the URI can be fetched
///  * by calling nsIChannel::open or nsIChannel::asyncOpen.
///  *
///  * After a request has been completed, the channel is still valid for accessing
///  * protocol-specific results.  For example, QI'ing to nsIHttpChannel allows
///  * response headers to be retrieved for the corresponding http transaction.
///  *
///  * This interface must be used only from the XPCOM main thread.
///  */
/// /**
///      * The original URI used to construct the channel. This is used in
///      * the case of a redirect or URI "resolution" (e.g. resolving a
///      * resource: URI to a file: URI) so that the original pre-redirect
///      * URI can still be obtained.  This is never null.  Attempts to
///      * set it to null must throw.
///      *
///      * NOTE: this is distinctly different from the http Referer (referring URI),
///      * which is typically the page that contained the original URI (accessible
///      * from nsIHttpChannel).
///      */
/// ```
///

/// `attribute nsIURI originalURI;`
#[inline]
pub unsafe fn SetOriginalURI(&self, aOriginalURI: *const nsIURI) -> ::nserror::nsresult {
((*self.vtable).SetOriginalURI)(self, aOriginalURI)
}


/// ```text
/// /**
///      * The URI corresponding to the channel.  Its value is immutable.
///      */
/// ```
///

/// `readonly attribute nsIURI URI;`
#[inline]
pub unsafe fn GetURI(&self, aURI: *mut*const nsIURI) -> ::nserror::nsresult {
((*self.vtable).GetURI)(self, aURI)
}


/// ```text
/// /**
///      * The owner, corresponding to the entity that is responsible for this
///      * channel.  Used by the security manager to grant or deny privileges to
///      * mobile code loaded from this channel.
///      *
///      * NOTE: this is a strong reference to the owner, so if the owner is also
///      * holding a strong reference to the channel, care must be taken to
///      * explicitly drop its reference to the channel.
///      */
/// ```
///

/// `attribute nsISupports owner;`
#[inline]
pub unsafe fn GetOwner(&self, aOwner: *mut *const nsISupports) -> ::nserror::nsresult {
((*self.vtable).GetOwner)(self, aOwner)
}


/// ```text
/// /**
///      * The owner, corresponding to the entity that is responsible for this
///      * channel.  Used by the security manager to grant or deny privileges to
///      * mobile code loaded from this channel.
///      *
///      * NOTE: this is a strong reference to the owner, so if the owner is also
///      * holding a strong reference to the channel, care must be taken to
///      * explicitly drop its reference to the channel.
///      */
/// ```
///

/// `attribute nsISupports owner;`
#[inline]
pub unsafe fn SetOwner(&self, aOwner: *const nsISupports) -> ::nserror::nsresult {
((*self.vtable).SetOwner)(self, aOwner)
}


/// ```text
/// /**
///      * The notification callbacks for the channel.  This is set by clients, who
///      * wish to provide a means to receive progress, status and protocol-specific
///      * notifications.  If this value is NULL, the channel implementation may use
///      * the notification callbacks from its load group.  The channel may also
///      * query the notification callbacks from its load group if its notification
///      * callbacks do not supply the requested interface.
///      *
///      * Interfaces commonly requested include: nsIProgressEventSink, nsIPrompt,
///      * and nsIAuthPrompt/nsIAuthPrompt2.
///      *
///      * When the channel is done, it must not continue holding references to
///      * this object.
///      *
///      * NOTE: A channel implementation should take care when "caching" an
///      * interface pointer queried from its notification callbacks.  If the
///      * notification callbacks are changed, then a cached interface pointer may
///      * become invalid and may therefore need to be re-queried.
///      */
/// ```
///

/// `attribute nsIInterfaceRequestor notificationCallbacks;`
#[inline]
pub unsafe fn GetNotificationCallbacks(&self, aNotificationCallbacks: *mut*const nsIInterfaceRequestor) -> ::nserror::nsresult {
((*self.vtable).GetNotificationCallbacks)(self, aNotificationCallbacks)
}


/// ```text
/// /**
///      * The notification callbacks for the channel.  This is set by clients, who
///      * wish to provide a means to receive progress, status and protocol-specific
///      * notifications.  If this value is NULL, the channel implementation may use
///      * the notification callbacks from its load group.  The channel may also
///      * query the notification callbacks from its load group if its notification
///      * callbacks do not supply the requested interface.
///      *
///      * Interfaces commonly requested include: nsIProgressEventSink, nsIPrompt,
///      * and nsIAuthPrompt/nsIAuthPrompt2.
///      *
///      * When the channel is done, it must not continue holding references to
///      * this object.
///      *
///      * NOTE: A channel implementation should take care when "caching" an
///      * interface pointer queried from its notification callbacks.  If the
///      * notification callbacks are changed, then a cached interface pointer may
///      * become invalid and may therefore need to be re-queried.
///      */
/// ```
///

/// `attribute nsIInterfaceRequestor notificationCallbacks;`
#[inline]
pub unsafe fn SetNotificationCallbacks(&self, aNotificationCallbacks: *const nsIInterfaceRequestor) -> ::nserror::nsresult {
((*self.vtable).SetNotificationCallbacks)(self, aNotificationCallbacks)
}


/// ```text
/// /**
///      * Transport-level security information (if any) corresponding to the
///      * channel.
///      *
///      * NOTE: In some circumstances TLS information is propagated onto
///      * non-nsIHttpChannel objects to indicate that their contents were likely
///      * delivered over TLS all the same.
///      *
///      * FIXME(bz, bug 1528449) is that still true now that
///      * document.open() doesn't do this?
///      */
/// ```
///

/// `readonly attribute nsISupports securityInfo;`
#[inline]
pub unsafe fn GetSecurityInfo(&self, aSecurityInfo: *mut *const nsISupports) -> ::nserror::nsresult {
((*self.vtable).GetSecurityInfo)(self, aSecurityInfo)
}


/// ```text
/// /**
///      * The MIME type of the channel's content if available.
///      *
///      * NOTE: the content type can often be wrongly specified (e.g., wrong file
///      * extension, wrong MIME type, wrong document type stored on a server, etc.),
///      * and the caller most likely wants to verify with the actual data.
///      *
///      * Setting contentType before the channel has been opened provides a hint
///      * to the channel as to what the MIME type is.  The channel may ignore this
///      * hint in deciding on the actual MIME type that it will report.
///      *
///      * Setting contentType after onStartRequest has been fired or after open()
///      * is called will override the type determined by the channel.
///      *
///      * Setting contentType between the time that asyncOpen() is called and the
///      * time when onStartRequest is fired has undefined behavior at this time.
///      *
///      * The value of the contentType attribute is a lowercase string.  A value
///      * assigned to this attribute will be parsed and normalized as follows:
///      *  1- any parameters (delimited with a ';') will be stripped.
///      *  2- if a charset parameter is given, then its value will replace the
///      *     the contentCharset attribute of the channel.
///      *  3- the stripped contentType will be lowercased.
///      * Any implementation of nsIChannel must follow these rules.
///      */
/// ```
///

/// `attribute ACString contentType;`
#[inline]
pub unsafe fn GetContentType(&self, aContentType: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
((*self.vtable).GetContentType)(self, aContentType)
}


/// ```text
/// /**
///      * The MIME type of the channel's content if available.
///      *
///      * NOTE: the content type can often be wrongly specified (e.g., wrong file
///      * extension, wrong MIME type, wrong document type stored on a server, etc.),
///      * and the caller most likely wants to verify with the actual data.
///      *
///      * Setting contentType before the channel has been opened provides a hint
///      * to the channel as to what the MIME type is.  The channel may ignore this
///      * hint in deciding on the actual MIME type that it will report.
///      *
///      * Setting contentType after onStartRequest has been fired or after open()
///      * is called will override the type determined by the channel.
///      *
///      * Setting contentType between the time that asyncOpen() is called and the
///      * time when onStartRequest is fired has undefined behavior at this time.
///      *
///      * The value of the contentType attribute is a lowercase string.  A value
///      * assigned to this attribute will be parsed and normalized as follows:
///      *  1- any parameters (delimited with a ';') will be stripped.
///      *  2- if a charset parameter is given, then its value will replace the
///      *     the contentCharset attribute of the channel.
///      *  3- the stripped contentType will be lowercased.
///      * Any implementation of nsIChannel must follow these rules.
///      */
/// ```
///

/// `attribute ACString contentType;`
#[inline]
pub unsafe fn SetContentType(&self, aContentType: *const ::nsstring::nsACString) -> ::nserror::nsresult {
((*self.vtable).SetContentType)(self, aContentType)
}


/// ```text
/// /**
///      * The character set of the channel's content if available and if applicable.
///      * This attribute only applies to textual data.
///      *
///      * The value of the contentCharset attribute is a mixedcase string.
///      */
/// ```
///

/// `attribute ACString contentCharset;`
#[inline]
pub unsafe fn GetContentCharset(&self, aContentCharset: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
((*self.vtable).GetContentCharset)(self, aContentCharset)
}


/// ```text
/// /**
///      * The character set of the channel's content if available and if applicable.
///      * This attribute only applies to textual data.
///      *
///      * The value of the contentCharset attribute is a mixedcase string.
///      */
/// ```
///

/// `attribute ACString contentCharset;`
#[inline]
pub unsafe fn SetContentCharset(&self, aContentCharset: *const ::nsstring::nsACString) -> ::nserror::nsresult {
((*self.vtable).SetContentCharset)(self, aContentCharset)
}


/// ```text
/// /**
///      * The length of the data associated with the channel if available.  A value
///      * of -1 indicates that the content length is unknown. Note that this is a
///      * 64-bit value and obsoletes the "content-length" property used on some
///      * channels.
///      */
/// ```
///

/// `attribute int64_t contentLength;`
#[inline]
pub unsafe fn GetContentLength(&self, aContentLength: *mut int64_t) -> ::nserror::nsresult {
((*self.vtable).GetContentLength)(self, aContentLength)
}


/// ```text
/// /**
///      * The length of the data associated with the channel if available.  A value
///      * of -1 indicates that the content length is unknown. Note that this is a
///      * 64-bit value and obsoletes the "content-length" property used on some
///      * channels.
///      */
/// ```
///

/// `attribute int64_t contentLength;`
#[inline]
pub unsafe fn SetContentLength(&self, aContentLength: int64_t) -> ::nserror::nsresult {
((*self.vtable).SetContentLength)(self, aContentLength)
}


/// ```text
/// /**
///      * Synchronously open the channel.
///      *
///      * @return blocking input stream to the channel's data.
///      *
///      * NOTE: nsIChannel implementations are not required to implement this
///      * method.  Moreover, since this method may block the calling thread, it
///      * should not be called on a thread that processes UI events.  Like any
///      * other nsIChannel method it must not be called on any thread other
///      * than the XPCOM main thread.
///      *
///      * NOTE: Implementations should throw NS_ERROR_IN_PROGRESS if the channel
///      * is reopened.
///      */
/// ```
///

/// `nsIInputStream open ();`
#[inline]
pub unsafe fn Open(&self, _retval: *mut*const nsIInputStream) -> ::nserror::nsresult {
((*self.vtable).Open)(self, _retval)
}


/// ```text
/// /**
///      * Asynchronously open this channel.  Data is fed to the specified stream
///      * listener as it becomes available.  The stream listener's methods are
///      * called on the thread that calls asyncOpen and are not called until
///      * after asyncOpen returns.  If asyncOpen returns successfully, the
///      * channel promises to call at least onStartRequest and onStopRequest.
///      *
///      * If the nsIRequest object passed to the stream listener's methods is not
///      * this channel, an appropriate onChannelRedirect notification needs to be
///      * sent to the notification callbacks before onStartRequest is called.
///      * Once onStartRequest is called, all following method calls on aListener
///      * will get the request that was passed to onStartRequest.
///      *
///      * If the channel's and loadgroup's notification callbacks do not provide
///      * an nsIChannelEventSink when onChannelRedirect would be called, that's
///      * equivalent to having called onChannelRedirect.
///      *
///      * If asyncOpen returns successfully, the channel is responsible for
///      * keeping itself alive until it has called onStopRequest on aListener or
///      * called onChannelRedirect.
///      *
///      * Implementations are allowed to synchronously add themselves to the
///      * associated load group (if any).
///      *
///      * NOTE: Implementations should throw NS_ERROR_ALREADY_OPENED if the
///      * channel is reopened.
///      * NOTE: Implementations should throw an error if the channel has been
///      * cancelled prior asyncOpen being called.
///      *
///      * @param aListener the nsIStreamListener implementation
///      * @see nsIChannelEventSink for onChannelRedirect
///      */
/// ```
///

/// `void asyncOpen (in nsIStreamListener aListener);`
#[inline]
pub unsafe fn AsyncOpen(&self, aListener: *const nsIStreamListener) -> ::nserror::nsresult {
((*self.vtable).AsyncOpen)(self, aListener)
}


/// ```text
/// /**
///      * True if the channel has been canceled.
///      */
/// ```
///

/// `[must_use] readonly attribute boolean canceled;`
#[inline]
pub unsafe fn GetCanceled(&self, aCanceled: *mut bool) -> ::nserror::nsresult {
((*self.vtable).GetCanceled)(self, aCanceled)
}


/// ```text
/// /**
///      * Access to the type implied or stated by the Content-Disposition header
///      * if available and if applicable. This allows determining inline versus
///      * attachment.
///      *
///      * Setting contentDisposition provides a hint to the channel about the
///      * disposition.  If the hint is DISPOSITION_ATTACHMENT and a normal
///      * Content-Disposition header is present, the hinted value will always be
///      * used.  The value from Content-Disposition header is only used when
///      * the hinted value is not DISPOSITION_ATTACHMENT.
///      * If the header is missing the hinted value will be used if set.
///      *
///      * Implementations should throw NS_ERROR_NOT_AVAILABLE if the header either
///      * doesn't exist for this type of channel or is empty, and return
///      * DISPOSITION_ATTACHMENT if an invalid/noncompliant value is present.
///      */
/// ```
///

/// `attribute unsigned long contentDisposition;`
#[inline]
pub unsafe fn GetContentDisposition(&self, aContentDisposition: *mut u32) -> ::nserror::nsresult {
((*self.vtable).GetContentDisposition)(self, aContentDisposition)
}


/// ```text
/// /**
///      * Access to the type implied or stated by the Content-Disposition header
///      * if available and if applicable. This allows determining inline versus
///      * attachment.
///      *
///      * Setting contentDisposition provides a hint to the channel about the
///      * disposition.  If the hint is DISPOSITION_ATTACHMENT and a normal
///      * Content-Disposition header is present, the hinted value will always be
///      * used.  The value from Content-Disposition header is only used when
///      * the hinted value is not DISPOSITION_ATTACHMENT.
///      * If the header is missing the hinted value will be used if set.
///      *
///      * Implementations should throw NS_ERROR_NOT_AVAILABLE if the header either
///      * doesn't exist for this type of channel or is empty, and return
///      * DISPOSITION_ATTACHMENT if an invalid/noncompliant value is present.
///      */
/// ```
///

/// `attribute unsigned long contentDisposition;`
#[inline]
pub unsafe fn SetContentDisposition(&self, aContentDisposition: u32) -> ::nserror::nsresult {
((*self.vtable).SetContentDisposition)(self, aContentDisposition)
}


/// ```text
/// /**
///      * Access to the filename portion of the Content-Disposition header if
///      * available and if applicable. This allows getting the preferred filename
///      * without having to parse it out yourself.
///      *
///      * Setting contentDispositionFilename provides a hint to the channel about
///      * the disposition.  If a normal Content-Disposition header is present its
///      * value will always be used.  If it is missing the hinted value will be
///      * used if set.
///      *
///      * Implementations should throw NS_ERROR_NOT_AVAILABLE if the header doesn't
///      * exist for this type of channel, if the header is empty, if the header
///      * doesn't contain a filename portion, or the value of the filename
///      * attribute is empty/missing.
///      */
/// ```
///

/// `attribute AString contentDispositionFilename;`
#[inline]
pub unsafe fn GetContentDispositionFilename(&self, aContentDispositionFilename: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
((*self.vtable).GetContentDispositionFilename)(self, aContentDispositionFilename)
}


/// ```text
/// /**
///      * Access to the filename portion of the Content-Disposition header if
///      * available and if applicable. This allows getting the preferred filename
///      * without having to parse it out yourself.
///      *
///      * Setting contentDispositionFilename provides a hint to the channel about
///      * the disposition.  If a normal Content-Disposition header is present its
///      * value will always be used.  If it is missing the hinted value will be
///      * used if set.
///      *
///      * Implementations should throw NS_ERROR_NOT_AVAILABLE if the header doesn't
///      * exist for this type of channel, if the header is empty, if the header
///      * doesn't contain a filename portion, or the value of the filename
///      * attribute is empty/missing.
///      */
/// ```
///

/// `attribute AString contentDispositionFilename;`
#[inline]
pub unsafe fn SetContentDispositionFilename(&self, aContentDispositionFilename: *const ::nsstring::nsAString) -> ::nserror::nsresult {
((*self.vtable).SetContentDispositionFilename)(self, aContentDispositionFilename)
}


/// ```text
/// /**
///      * Access to the raw Content-Disposition header if available and applicable.
///      *
///      * Implementations should throw NS_ERROR_NOT_AVAILABLE if the header either
///      * doesn't exist for this type of channel or is empty.
///      *
///      * @deprecated Use contentDisposition/contentDispositionFilename instead.
///      */
/// ```
///

/// `readonly attribute ACString contentDispositionHeader;`
#[inline]
pub unsafe fn GetContentDispositionHeader(&self, aContentDispositionHeader: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
((*self.vtable).GetContentDispositionHeader)(self, aContentDispositionHeader)
}


/// ```text
/// /**
///      * The LoadInfo object contains information about a network load, why it
///      * was started, and how we plan on using the resulting response.
///      * If a network request is redirected, the new channel will receive a new
///      * LoadInfo object. The new object will contain mostly the same
///      * information as the pre-redirect one, but updated as appropriate.
///      * For detailed information about what parts of LoadInfo are updated on
///      * redirect, see documentation on individual properties.
///      */
/// ```
///

/// `attribute nsILoadInfo loadInfo;`
#[inline]
pub unsafe fn GetLoadInfo(&self, aLoadInfo: *mut *const nsILoadInfo) -> ::nserror::nsresult {
((*self.vtable).GetLoadInfo)(self, aLoadInfo)
}


/// ```text
/// /**
///      * The LoadInfo object contains information about a network load, why it
///      * was started, and how we plan on using the resulting response.
///      * If a network request is redirected, the new channel will receive a new
///      * LoadInfo object. The new object will contain mostly the same
///      * information as the pre-redirect one, but updated as appropriate.
///      * For detailed information about what parts of LoadInfo are updated on
///      * redirect, see documentation on individual properties.
///      */
/// ```
///

/// `attribute nsILoadInfo loadInfo;`
#[inline]
pub unsafe fn SetLoadInfo(&self, aLoadInfo: *const nsILoadInfo) -> ::nserror::nsresult {
((*self.vtable).SetLoadInfo)(self, aLoadInfo)
}


/// ```text
/// /**
///      * Returns true if the channel is used to create a document.
///      * It returns true if the loadFlags have LOAD_DOCUMENT_URI set, or if
///      * LOAD_HTML_OBJECT_DATA is set and the channel has the appropriate
///      * MIME type.
///      * Note: May have the wrong value if called before OnStartRequest as we
///      * don't know the MIME type yet.
///      */
/// ```
///

/// `readonly attribute bool isDocument;`
#[inline]
pub unsafe fn GetIsDocument(&self, aIsDocument: *mut bool) -> ::nserror::nsresult {
((*self.vtable).GetIsDocument)(self, aIsDocument)
}


}


/// `interface nsIIdentChannel : nsIChannel`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIIdentChannel {
vtable: *const nsIIdentChannelVTable,

/// This field is a phantomdata to ensure that the VTable type and any
/// struct containing it is not safe to send across threads, as XPCOM is
/// generally not threadsafe.
///
/// XPCOM interfaces in general are not safe to send across threads.
__nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIIdentChannel.
unsafe impl XpCom for nsIIdentChannel {
const IID: nsIID = nsID(0x1ebbff64, 0xd742, 0x4f4a,
[0xaa, 0xd5, 0x4d, 0xf2, 0xd1, 0xeb, 0x93, 0x7a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIIdentChannel {
#[inline]
unsafe fn addref(&self) {
self.AddRef();
}
#[inline]
unsafe fn release(&self) {
self.Release();
}
}

// This trait is implemented on all types which can be coerced to from nsIIdentChannel.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIIdentChannelCoerce {
/// Cheaply cast a value of this type from a `nsIIdentChannel`.
fn coerce_from(v: &nsIIdentChannel) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIIdentChannelCoerce for nsIIdentChannel {
#[inline]
fn coerce_from(v: &nsIIdentChannel) -> &Self {
v
}
}

impl nsIIdentChannel {
/// Cast this `nsIIdentChannel` to one of its base interfaces.
#[inline]
pub fn coerce<T: nsIIdentChannelCoerce>(&self) -> &T {
T::coerce_from(self)
}
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIIdentChannel {
type Target = nsIChannel;
#[inline]
fn deref(&self) -> &nsIChannel {
unsafe {
    ::std::mem::transmute(self)
}
}
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIChannelCoerce> nsIIdentChannelCoerce for T {
#[inline]
fn coerce_from(v: &nsIIdentChannel) -> &Self {
T::coerce_from(v)
}
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIIdentChannel
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIIdentChannelVTable {
/// We need to include the members from the base interface's vtable at the start
/// of the VTable definition.
pub __base: nsIChannelVTable,

/* [must_use] attribute uint64_t channelId; */
pub GetChannelId: unsafe extern "system" fn (this: *const nsIIdentChannel, aChannelId: *mut uint64_t) -> ::nserror::nsresult,

/* [must_use] attribute uint64_t channelId; */
pub SetChannelId: unsafe extern "system" fn (this: *const nsIIdentChannel, aChannelId: uint64_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIIdentChannel {

/// ```text
/// /**
///      * Unique ID of the channel, shared between parent and child. Needed if
///      * the channel activity needs to be monitored across process boundaries,
///      * like in devtools net monitor. See bug 1274556.
///      */
/// ```
///

/// `[must_use] attribute uint64_t channelId;`
#[inline]
pub unsafe fn GetChannelId(&self, aChannelId: *mut uint64_t) -> ::nserror::nsresult {
((*self.vtable).GetChannelId)(self, aChannelId)
}


/// ```text
/// /**
///      * Unique ID of the channel, shared between parent and child. Needed if
///      * the channel activity needs to be monitored across process boundaries,
///      * like in devtools net monitor. See bug 1274556.
///      */
/// ```
///

/// `[must_use] attribute uint64_t channelId;`
#[inline]
pub unsafe fn SetChannelId(&self, aChannelId: uint64_t) -> ::nserror::nsresult {
((*self.vtable).SetChannelId)(self, aChannelId)
}


}


