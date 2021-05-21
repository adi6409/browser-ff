//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIHttpChannel.idl
//


/// `interface nsIHttpChannel : nsIIdentChannel`
///

/// ```text
/// /**
///  * nsIHttpChannel
///  *
///  * This interface allows for the modification of HTTP request parameters and
///  * the inspection of the resulting HTTP response status and headers when they
///  * become available.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHttpChannel {
    vtable: *const nsIHttpChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHttpChannel.
unsafe impl XpCom for nsIHttpChannel {
    const IID: nsIID = nsID(0xc5a4a073, 0x4539, 0x49c7,
        [0xa3, 0xf2, 0xce, 0xc3, 0xf0, 0x61, 0x9c, 0x6c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHttpChannel {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHttpChannel.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHttpChannelCoerce {
    /// Cheaply cast a value of this type from a `nsIHttpChannel`.
    fn coerce_from(v: &nsIHttpChannel) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHttpChannelCoerce for nsIHttpChannel {
    #[inline]
    fn coerce_from(v: &nsIHttpChannel) -> &Self {
        v
    }
}

impl nsIHttpChannel {
    /// Cast this `nsIHttpChannel` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHttpChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHttpChannel {
    type Target = nsIIdentChannel;
    #[inline]
    fn deref(&self) -> &nsIIdentChannel {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIIdentChannelCoerce> nsIHttpChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpChannel) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHttpChannel
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHttpChannelVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIIdentChannelVTable,

    /* [must_use] attribute ACString requestMethod; */
    pub GetRequestMethod: unsafe extern "system" fn (this: *const nsIHttpChannel, aRequestMethod: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] attribute ACString requestMethod; */
    pub SetRequestMethod: unsafe extern "system" fn (this: *const nsIHttpChannel, aRequestMethod: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [infallible,must_use] attribute nsIReferrerInfo referrerInfo; */
    pub GetReferrerInfo: unsafe extern "system" fn (this: *const nsIHttpChannel, aReferrerInfo: *mut *const nsIReferrerInfo) -> ::nserror::nsresult,

    /* [infallible,must_use] attribute nsIReferrerInfo referrerInfo; */
    pub SetReferrerInfo: unsafe extern "system" fn (this: *const nsIHttpChannel, aReferrerInfo: *const nsIReferrerInfo) -> ::nserror::nsresult,

    /* [must_use,noscript] void setReferrerInfoWithoutClone (in nsIReferrerInfo aReferrerInfo); */
    pub SetReferrerInfoWithoutClone: unsafe extern "system" fn (this: *const nsIHttpChannel, aReferrerInfo: *const nsIReferrerInfo) -> ::nserror::nsresult,

    /* [must_use] readonly attribute ACString protocolVersion; */
    pub GetProtocolVersion: unsafe extern "system" fn (this: *const nsIHttpChannel, aProtocolVersion: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute uint64_t transferSize; */
    pub GetTransferSize: unsafe extern "system" fn (this: *const nsIHttpChannel, aTransferSize: *mut uint64_t) -> ::nserror::nsresult,

    /* [must_use] readonly attribute uint64_t requestSize; */
    pub GetRequestSize: unsafe extern "system" fn (this: *const nsIHttpChannel, aRequestSize: *mut uint64_t) -> ::nserror::nsresult,

    /* [must_use] readonly attribute uint64_t decodedBodySize; */
    pub GetDecodedBodySize: unsafe extern "system" fn (this: *const nsIHttpChannel, aDecodedBodySize: *mut uint64_t) -> ::nserror::nsresult,

    /* [must_use] readonly attribute uint64_t encodedBodySize; */
    pub GetEncodedBodySize: unsafe extern "system" fn (this: *const nsIHttpChannel, aEncodedBodySize: *mut uint64_t) -> ::nserror::nsresult,

    /* [must_use] ACString getRequestHeader (in ACString aHeader); */
    pub GetRequestHeader: unsafe extern "system" fn (this: *const nsIHttpChannel, aHeader: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] void setRequestHeader (in ACString aHeader, in ACString aValue, in boolean aMerge); */
    pub SetRequestHeader: unsafe extern "system" fn (this: *const nsIHttpChannel, aHeader: *const ::nsstring::nsACString, aValue: *const ::nsstring::nsACString, aMerge: bool) -> ::nserror::nsresult,

    /* [must_use] void setNewReferrerInfo (in ACString aUrl, in nsIReferrerInfo_ReferrerPolicyIDL aPolicy, in boolean aSendReferrer); */
    pub SetNewReferrerInfo: unsafe extern "system" fn (this: *const nsIHttpChannel, aUrl: *const ::nsstring::nsACString, aPolicy:  u8, aSendReferrer: bool) -> ::nserror::nsresult,

    /* [must_use] void setEmptyRequestHeader (in ACString aHeader); */
    pub SetEmptyRequestHeader: unsafe extern "system" fn (this: *const nsIHttpChannel, aHeader: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] void visitRequestHeaders (in nsIHttpHeaderVisitor aVisitor); */
    pub VisitRequestHeaders: unsafe extern "system" fn (this: *const nsIHttpChannel, aVisitor: *const nsIHttpHeaderVisitor) -> ::nserror::nsresult,

    /* [must_use] void visitNonDefaultRequestHeaders (in nsIHttpHeaderVisitor aVisitor); */
    pub VisitNonDefaultRequestHeaders: unsafe extern "system" fn (this: *const nsIHttpChannel, aVisitor: *const nsIHttpHeaderVisitor) -> ::nserror::nsresult,

    /* [must_use] bool ShouldStripRequestBodyHeader (in ACString aMethod); */
    pub ShouldStripRequestBodyHeader: unsafe extern "system" fn (this: *const nsIHttpChannel, aMethod: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,

    /* [must_use] attribute boolean allowPipelining; */
    pub GetAllowPipelining: unsafe extern "system" fn (this: *const nsIHttpChannel, aAllowPipelining: *mut bool) -> ::nserror::nsresult,

    /* [must_use] attribute boolean allowPipelining; */
    pub SetAllowPipelining: unsafe extern "system" fn (this: *const nsIHttpChannel, aAllowPipelining: bool) -> ::nserror::nsresult,

    /* [must_use] attribute boolean allowSTS; */
    pub GetAllowSTS: unsafe extern "system" fn (this: *const nsIHttpChannel, aAllowSTS: *mut bool) -> ::nserror::nsresult,

    /* [must_use] attribute boolean allowSTS; */
    pub SetAllowSTS: unsafe extern "system" fn (this: *const nsIHttpChannel, aAllowSTS: bool) -> ::nserror::nsresult,

    /* [must_use] attribute unsigned long redirectionLimit; */
    pub GetRedirectionLimit: unsafe extern "system" fn (this: *const nsIHttpChannel, aRedirectionLimit: *mut u32) -> ::nserror::nsresult,

    /* [must_use] attribute unsigned long redirectionLimit; */
    pub SetRedirectionLimit: unsafe extern "system" fn (this: *const nsIHttpChannel, aRedirectionLimit: u32) -> ::nserror::nsresult,

    /* [must_use] readonly attribute unsigned long responseStatus; */
    pub GetResponseStatus: unsafe extern "system" fn (this: *const nsIHttpChannel, aResponseStatus: *mut u32) -> ::nserror::nsresult,

    /* [must_use] readonly attribute ACString responseStatusText; */
    pub GetResponseStatusText: unsafe extern "system" fn (this: *const nsIHttpChannel, aResponseStatusText: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute boolean requestSucceeded; */
    pub GetRequestSucceeded: unsafe extern "system" fn (this: *const nsIHttpChannel, aRequestSucceeded: *mut bool) -> ::nserror::nsresult,

    /* [must_use] attribute boolean isMainDocumentChannel; */
    pub GetIsMainDocumentChannel: unsafe extern "system" fn (this: *const nsIHttpChannel, aIsMainDocumentChannel: *mut bool) -> ::nserror::nsresult,

    /* [must_use] attribute boolean isMainDocumentChannel; */
    pub SetIsMainDocumentChannel: unsafe extern "system" fn (this: *const nsIHttpChannel, aIsMainDocumentChannel: bool) -> ::nserror::nsresult,

    /* [must_use] ACString getResponseHeader (in ACString header); */
    pub GetResponseHeader: unsafe extern "system" fn (this: *const nsIHttpChannel, header: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] void setResponseHeader (in ACString header, in ACString value, in boolean merge); */
    pub SetResponseHeader: unsafe extern "system" fn (this: *const nsIHttpChannel, header: *const ::nsstring::nsACString, value: *const ::nsstring::nsACString, merge: bool) -> ::nserror::nsresult,

    /* [must_use] void visitResponseHeaders (in nsIHttpHeaderVisitor aVisitor); */
    pub VisitResponseHeaders: unsafe extern "system" fn (this: *const nsIHttpChannel, aVisitor: *const nsIHttpHeaderVisitor) -> ::nserror::nsresult,

    /* [must_use] void getOriginalResponseHeader (in ACString aHeader, in nsIHttpHeaderVisitor aVisitor); */
    pub GetOriginalResponseHeader: unsafe extern "system" fn (this: *const nsIHttpChannel, aHeader: *const ::nsstring::nsACString, aVisitor: *const nsIHttpHeaderVisitor) -> ::nserror::nsresult,

    /* [must_use] void visitOriginalResponseHeaders (in nsIHttpHeaderVisitor aVisitor); */
    pub VisitOriginalResponseHeaders: unsafe extern "system" fn (this: *const nsIHttpChannel, aVisitor: *const nsIHttpHeaderVisitor) -> ::nserror::nsresult,

    /* [must_use] boolean isNoStoreResponse (); */
    pub IsNoStoreResponse: unsafe extern "system" fn (this: *const nsIHttpChannel, _retval: *mut bool) -> ::nserror::nsresult,

    /* [must_use] boolean isNoCacheResponse (); */
    pub IsNoCacheResponse: unsafe extern "system" fn (this: *const nsIHttpChannel, _retval: *mut bool) -> ::nserror::nsresult,

    /* [must_use] boolean isPrivateResponse (); */
    pub IsPrivateResponse: unsafe extern "system" fn (this: *const nsIHttpChannel, _retval: *mut bool) -> ::nserror::nsresult,

    /* [must_use] void redirectTo (in nsIURI aTargetURI); */
    pub RedirectTo: unsafe extern "system" fn (this: *const nsIHttpChannel, aTargetURI: *const nsIURI) -> ::nserror::nsresult,

    /* [must_use] void upgradeToSecure (); */
    pub UpgradeToSecure: unsafe extern "system" fn (this: *const nsIHttpChannel) -> ::nserror::nsresult,

    /* [must_use,noscript] attribute uint64_t requestContextID; */
    pub GetRequestContextID: unsafe extern "system" fn (this: *const nsIHttpChannel, aRequestContextID: *mut uint64_t) -> ::nserror::nsresult,

    /* [must_use,noscript] attribute uint64_t requestContextID; */
    pub SetRequestContextID: unsafe extern "system" fn (this: *const nsIHttpChannel, aRequestContextID: uint64_t) -> ::nserror::nsresult,

    /* [must_use] attribute uint64_t topLevelContentWindowId; */
    pub GetTopLevelContentWindowId: unsafe extern "system" fn (this: *const nsIHttpChannel, aTopLevelContentWindowId: *mut uint64_t) -> ::nserror::nsresult,

    /* [must_use] attribute uint64_t topLevelContentWindowId; */
    pub SetTopLevelContentWindowId: unsafe extern "system" fn (this: *const nsIHttpChannel, aTopLevelContentWindowId: uint64_t) -> ::nserror::nsresult,

    /* [infallible] readonly attribute nsIHttpChannel_FlashPluginState flashPluginState; */
    pub GetFlashPluginState: unsafe extern "system" fn (this: *const nsIHttpChannel, aFlashPluginState: *mut u8) -> ::nserror::nsresult,

    /* [must_use] attribute uint64_t topLevelOuterContentWindowId; */
    pub GetTopLevelOuterContentWindowId: unsafe extern "system" fn (this: *const nsIHttpChannel, aTopLevelOuterContentWindowId: *mut uint64_t) -> ::nserror::nsresult,

    /* [must_use] attribute uint64_t topLevelOuterContentWindowId; */
    pub SetTopLevelOuterContentWindowId: unsafe extern "system" fn (this: *const nsIHttpChannel, aTopLevelOuterContentWindowId: uint64_t) -> ::nserror::nsresult,

    /* void logBlockedCORSRequest (in AString aMessage, in ACString aCategory); */
    pub LogBlockedCORSRequest: unsafe extern "system" fn (this: *const nsIHttpChannel, aMessage: *const ::nsstring::nsAString, aCategory: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void logMimeTypeMismatch (in ACString aMessageName, in boolean aWarning, in AString aURL, in AString aContentType); */
    pub LogMimeTypeMismatch: unsafe extern "system" fn (this: *const nsIHttpChannel, aMessageName: *const ::nsstring::nsACString, aWarning: bool, aURL: *const ::nsstring::nsAString, aContentType: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHttpChannel {

    /// ```text
    /// /**************************************************************************
    ///      * REQUEST CONFIGURATION
    ///      *
    ///      * Modifying request parameters after asyncOpen has been called is an error.
    ///      */
    /// /**
    ///      * Set/get the HTTP request method (default is "GET").  Both setter and
    ///      * getter are case sensitive.
    ///      *
    ///      * This attribute may only be set before the channel is opened.
    ///      *
    ///      * NOTE: The data for a "POST" or "PUT" request can be configured via
    ///      * nsIUploadChannel; however, after setting the upload data, it may be
    ///      * necessary to set the request method explicitly.  The documentation
    ///      * for nsIUploadChannel has further details.
    ///      *
    ///      * @throws NS_ERROR_IN_PROGRESS if set after the channel has been opened.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute ACString requestMethod;`
    #[inline]
    pub unsafe fn GetRequestMethod(&self, aRequestMethod: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetRequestMethod)(self, aRequestMethod)
    }


    /// ```text
    /// /**************************************************************************
    ///      * REQUEST CONFIGURATION
    ///      *
    ///      * Modifying request parameters after asyncOpen has been called is an error.
    ///      */
    /// /**
    ///      * Set/get the HTTP request method (default is "GET").  Both setter and
    ///      * getter are case sensitive.
    ///      *
    ///      * This attribute may only be set before the channel is opened.
    ///      *
    ///      * NOTE: The data for a "POST" or "PUT" request can be configured via
    ///      * nsIUploadChannel; however, after setting the upload data, it may be
    ///      * necessary to set the request method explicitly.  The documentation
    ///      * for nsIUploadChannel has further details.
    ///      *
    ///      * @throws NS_ERROR_IN_PROGRESS if set after the channel has been opened.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute ACString requestMethod;`
    #[inline]
    pub unsafe fn SetRequestMethod(&self, aRequestMethod: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetRequestMethod)(self, aRequestMethod)
    }


    /// ```text
    /// /**
    ///      * Get/set the referrer information.  This contains the referrer (URI) of the
    ///      * resource from which this channel's URI was obtained (see RFC2616 section
        ///      * 14.36) and the referrer policy applied to the referrer.
    ///      *
    ///      * This attribute may only be set before the channel is opened.
    ///      *
    ///      * Setting this attribute will clone new referrerInfo object by default.
    ///      *
    ///      * NOTE: The channel may silently refuse to set the Referer header if the
    ///      * URI does not pass certain security checks (e.g., a "https://" URL will
        ///      * never be sent as the referrer for a plaintext HTTP request).  The
    ///      * implementation is not required to throw an exception when the referrer
    ///      * URI is rejected.
    ///      *
    ///      * @throws NS_ERROR_IN_PROGRESS if set after the channel has been opened.
    ///      * @throws NS_ERROR_FAILURE if used for setting referrer during
    ///      *         visitRequestHeaders. Getting the value will not throw.
    ///      */
    /// ```
    ///

    /// `[infallible,must_use] attribute nsIReferrerInfo referrerInfo;`
    #[inline]
    pub unsafe fn GetReferrerInfo(&self, aReferrerInfo: *mut *const nsIReferrerInfo) -> ::nserror::nsresult {
        ((*self.vtable).GetReferrerInfo)(self, aReferrerInfo)
    }


    /// ```text
    /// /**
    ///      * Get/set the referrer information.  This contains the referrer (URI) of the
    ///      * resource from which this channel's URI was obtained (see RFC2616 section
        ///      * 14.36) and the referrer policy applied to the referrer.
    ///      *
    ///      * This attribute may only be set before the channel is opened.
    ///      *
    ///      * Setting this attribute will clone new referrerInfo object by default.
    ///      *
    ///      * NOTE: The channel may silently refuse to set the Referer header if the
    ///      * URI does not pass certain security checks (e.g., a "https://" URL will
        ///      * never be sent as the referrer for a plaintext HTTP request).  The
    ///      * implementation is not required to throw an exception when the referrer
    ///      * URI is rejected.
    ///      *
    ///      * @throws NS_ERROR_IN_PROGRESS if set after the channel has been opened.
    ///      * @throws NS_ERROR_FAILURE if used for setting referrer during
    ///      *         visitRequestHeaders. Getting the value will not throw.
    ///      */
    /// ```
    ///

    /// `[infallible,must_use] attribute nsIReferrerInfo referrerInfo;`
    #[inline]
    pub unsafe fn SetReferrerInfo(&self, aReferrerInfo: *const nsIReferrerInfo) -> ::nserror::nsresult {
        ((*self.vtable).SetReferrerInfo)(self, aReferrerInfo)
    }


    /// ```text
    /// /**
    ///      * Set referrer Info without clone new object.
    ///      * Use this api only when you are passing a referrerInfo to the channel with
    ///      * 1-1 relationship. Don't use this api if you will reuse the referrer info
    ///      * object later. For example when to use:
    ///      * channel.setReferrerInfoWithoutClone(new ReferrerInfo());
    ///      *
    ///      */
    /// ```
    ///

    /// `[must_use,noscript] void setReferrerInfoWithoutClone (in nsIReferrerInfo aReferrerInfo);`
    #[inline]
    pub unsafe fn SetReferrerInfoWithoutClone(&self, aReferrerInfo: *const nsIReferrerInfo) -> ::nserror::nsresult {
        ((*self.vtable).SetReferrerInfoWithoutClone)(self, aReferrerInfo)
    }


    /// ```text
    /// /**
    ///      * Returns the network protocol used to fetch the resource as identified
    ///      * by the ALPN Protocol ID.
    ///      *
    ///      * @throws NS_ERROR_NOT_AVAILABLE if called before the response
    ///      *         has been received (before onStartRequest).
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute ACString protocolVersion;`
    #[inline]
    pub unsafe fn GetProtocolVersion(&self, aProtocolVersion: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetProtocolVersion)(self, aProtocolVersion)
    }


    /// ```text
    /// /**
    ///      * size consumed by the response header fields and the response payload body
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute uint64_t transferSize;`
    #[inline]
    pub unsafe fn GetTransferSize(&self, aTransferSize: *mut uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetTransferSize)(self, aTransferSize)
    }


    /// ```text
    /// /**
    ///      * size consumed by the request header fields and the request payload body
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute uint64_t requestSize;`
    #[inline]
    pub unsafe fn GetRequestSize(&self, aRequestSize: *mut uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetRequestSize)(self, aRequestSize)
    }


    /// ```text
    /// /**
    ///      * The size of the message body received by the client,
    ///      * after removing any applied content-codings
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute uint64_t decodedBodySize;`
    #[inline]
    pub unsafe fn GetDecodedBodySize(&self, aDecodedBodySize: *mut uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetDecodedBodySize)(self, aDecodedBodySize)
    }


    /// ```text
    /// /**
    ///      * The size in octets of the payload body, prior to removing content-codings
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute uint64_t encodedBodySize;`
    #[inline]
    pub unsafe fn GetEncodedBodySize(&self, aEncodedBodySize: *mut uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetEncodedBodySize)(self, aEncodedBodySize)
    }


    /// ```text
    /// /**
    ///      * Get the value of a particular request header.
    ///      *
    ///      * @param aHeader
    ///      *        The case-insensitive name of the request header to query (e.g.,
        ///      *        "Cache-Control").
    ///      *
    ///      * @return the value of the request header.
    ///      * @throws NS_ERROR_NOT_AVAILABLE if the header is not set.
    ///      */
    /// ```
    ///

    /// `[must_use] ACString getRequestHeader (in ACString aHeader);`
    #[inline]
    pub unsafe fn GetRequestHeader(&self, aHeader: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetRequestHeader)(self, aHeader, _retval)
    }


    /// ```text
    /// /**
    ///      * Set the value of a particular request header.
    ///      *
    ///      * This method allows, for example, the cookies module to add "Cookie"
    ///      * headers to the outgoing HTTP request.
    ///      *
    ///      * This method may only be called before the channel is opened.
    ///      *
    ///      * @param aHeader
    ///      *        The case-insensitive name of the request header to set (e.g.,
        ///      *        "Cookie").
    ///      * @param aValue
    ///      *        The request header value to set (e.g., "X=1").
    ///      * @param aMerge
    ///      *        If true, the new header value will be merged with any existing
    ///      *        values for the specified header.  This flag is ignored if the
    ///      *        specified header does not support merging (e.g., the "Content-
        ///      *        Type" header can only have one value).  The list of headers for
    ///      *        which this flag is ignored is an implementation detail.  If this
    ///      *        flag is false, then the header value will be replaced with the
    ///      *        contents of |aValue|.
    ///      *
    ///      * If aValue is empty and aMerge is false, the header will be cleared.
    ///      *
    ///      * @throws NS_ERROR_IN_PROGRESS if called after the channel has been
    ///      *         opened.
    ///      * @throws NS_ERROR_FAILURE if called during visitRequestHeaders.
    ///      */
    /// ```
    ///

    /// `[must_use] void setRequestHeader (in ACString aHeader, in ACString aValue, in boolean aMerge);`
    #[inline]
    pub unsafe fn SetRequestHeader(&self, aHeader: *const ::nsstring::nsACString, aValue: *const ::nsstring::nsACString, aMerge: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetRequestHeader)(self, aHeader, aValue, aMerge)
    }


    /// ```text
    /// /**
    ///      * Creates and sets new ReferrerInfo object
    ///      * @param aUrl          referrer url
    ///      * @param aPolicy       referrer policy of the created object
    ///      * @param aSendReferrer indicates if the referrer should not be sent or not
    ///      *                      even when it's available.
    ///      */
    /// ```
    ///

    /// `[must_use] void setNewReferrerInfo (in ACString aUrl, in nsIReferrerInfo_ReferrerPolicyIDL aPolicy, in boolean aSendReferrer);`
    #[inline]
    pub unsafe fn SetNewReferrerInfo(&self, aUrl: *const ::nsstring::nsACString, aPolicy:  u8, aSendReferrer: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetNewReferrerInfo)(self, aUrl, aPolicy, aSendReferrer)
    }


    /// ```text
    /// /**
    ///      * Set a request header with empty value.
    ///      *
    ///      * This should be used with caution in the cases where the behavior of
    ///      * setRequestHeader ignoring empty header values is undesirable.
    ///      *
    ///      * This method may only be called before the channel is opened.
    ///      *
    ///      * @param aHeader
    ///      *        The case-insensitive name of the request header to set (e.g.,
        ///      *        "Cookie").
    ///      *
    ///      * @throws NS_ERROR_IN_PROGRESS if called after the channel has been
    ///      *         opened.
    ///      * @throws NS_ERROR_FAILURE if called during visitRequestHeaders.
    ///      */
    /// ```
    ///

    /// `[must_use] void setEmptyRequestHeader (in ACString aHeader);`
    #[inline]
    pub unsafe fn SetEmptyRequestHeader(&self, aHeader: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetEmptyRequestHeader)(self, aHeader)
    }


    /// ```text
    /// /**
    ///      * Call this method to visit all request headers.  Calling setRequestHeader
    ///      * while visiting request headers has undefined behavior.  Don't do it!
    ///      *
    ///      * @param aVisitor
    ///      *        the header visitor instance.
    ///      */
    /// ```
    ///

    /// `[must_use] void visitRequestHeaders (in nsIHttpHeaderVisitor aVisitor);`
    #[inline]
    pub unsafe fn VisitRequestHeaders(&self, aVisitor: *const nsIHttpHeaderVisitor) -> ::nserror::nsresult {
        ((*self.vtable).VisitRequestHeaders)(self, aVisitor)
    }


    /// ```text
    /// /**
    ///      * Call this method to visit all non-default (UA-provided) request headers.
    ///      * Calling setRequestHeader while visiting request headers has undefined
    ///      * behavior. Don't do it!
    ///      *
    ///      * @param aVisitor
    ///      *        the header visitor instance.
    ///      */
    /// ```
    ///

    /// `[must_use] void visitNonDefaultRequestHeaders (in nsIHttpHeaderVisitor aVisitor);`
    #[inline]
    pub unsafe fn VisitNonDefaultRequestHeaders(&self, aVisitor: *const nsIHttpHeaderVisitor) -> ::nserror::nsresult {
        ((*self.vtable).VisitNonDefaultRequestHeaders)(self, aVisitor)
    }


    /// ```text
    /// /**
    ///      * Call this method to see if we need to strip the request body headers
    ///      * for the new http channel. This should be called during redirection.
    ///      */
    /// ```
    ///

    /// `[must_use] bool ShouldStripRequestBodyHeader (in ACString aMethod);`
    #[inline]
    pub unsafe fn ShouldStripRequestBodyHeader(&self, aMethod: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ShouldStripRequestBodyHeader)(self, aMethod, _retval)
    }


    /// ```text
    /// /**
    ///      * This attribute no longer has any effect, it remains for backwards compat
    ///      *
    ///      * @throws NS_ERROR_FAILURE if set after the channel has been opened.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute boolean allowPipelining;`
    #[inline]
    pub unsafe fn GetAllowPipelining(&self, aAllowPipelining: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAllowPipelining)(self, aAllowPipelining)
    }


    /// ```text
    /// /**
    ///      * This attribute no longer has any effect, it remains for backwards compat
    ///      *
    ///      * @throws NS_ERROR_FAILURE if set after the channel has been opened.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute boolean allowPipelining;`
    #[inline]
    pub unsafe fn SetAllowPipelining(&self, aAllowPipelining: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetAllowPipelining)(self, aAllowPipelining)
    }


    /// ```text
    /// /**
    ///      * This attribute of the channel indicates whether or not
    ///      * the underlying HTTP transaction should be honor stored Strict Transport
    ///      * Security directives for its principal. It defaults to true. Using
    ///      * OCSP to bootstrap the HTTPs is the likely use case for setting it to
    ///      * false.
    ///      *
    ///      * This attribute may only be set before the channel is opened.
    ///      *
    ///      * @throws NS_ERROR_IN_PROGRESS or NS_ERROR_ALREADY_OPENED
    ///      *         if called after the channel has been opened.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute boolean allowSTS;`
    #[inline]
    pub unsafe fn GetAllowSTS(&self, aAllowSTS: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAllowSTS)(self, aAllowSTS)
    }


    /// ```text
    /// /**
    ///      * This attribute of the channel indicates whether or not
    ///      * the underlying HTTP transaction should be honor stored Strict Transport
    ///      * Security directives for its principal. It defaults to true. Using
    ///      * OCSP to bootstrap the HTTPs is the likely use case for setting it to
    ///      * false.
    ///      *
    ///      * This attribute may only be set before the channel is opened.
    ///      *
    ///      * @throws NS_ERROR_IN_PROGRESS or NS_ERROR_ALREADY_OPENED
    ///      *         if called after the channel has been opened.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute boolean allowSTS;`
    #[inline]
    pub unsafe fn SetAllowSTS(&self, aAllowSTS: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetAllowSTS)(self, aAllowSTS)
    }


    /// ```text
    /// /**
    ///      * This attribute specifies the number of redirects this channel is allowed
    ///      * to make.  If zero, the channel will fail to redirect and will generate
    ///      * a NS_ERROR_REDIRECT_LOOP failure status.
    ///      *
    ///      * NOTE: An HTTP redirect results in a new channel being created.  If the
    ///      * new channel supports nsIHttpChannel, then it will be assigned a value
    ///      * to its |redirectionLimit| attribute one less than the value of the
    ///      * redirected channel's |redirectionLimit| attribute.  The initial value
    ///      * for this attribute may be a configurable preference (depending on the
        ///      * implementation).
    ///      */
    /// ```
    ///

    /// `[must_use] attribute unsigned long redirectionLimit;`
    #[inline]
    pub unsafe fn GetRedirectionLimit(&self, aRedirectionLimit: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetRedirectionLimit)(self, aRedirectionLimit)
    }


    /// ```text
    /// /**
    ///      * This attribute specifies the number of redirects this channel is allowed
    ///      * to make.  If zero, the channel will fail to redirect and will generate
    ///      * a NS_ERROR_REDIRECT_LOOP failure status.
    ///      *
    ///      * NOTE: An HTTP redirect results in a new channel being created.  If the
    ///      * new channel supports nsIHttpChannel, then it will be assigned a value
    ///      * to its |redirectionLimit| attribute one less than the value of the
    ///      * redirected channel's |redirectionLimit| attribute.  The initial value
    ///      * for this attribute may be a configurable preference (depending on the
        ///      * implementation).
    ///      */
    /// ```
    ///

    /// `[must_use] attribute unsigned long redirectionLimit;`
    #[inline]
    pub unsafe fn SetRedirectionLimit(&self, aRedirectionLimit: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetRedirectionLimit)(self, aRedirectionLimit)
    }


    /// ```text
    /// /**************************************************************************
    ///      * RESPONSE INFO
    ///      *
    ///      * Accessing response info before the onStartRequest event is an error.
    ///      */
    /// /**
    ///      * Get the HTTP response code (e.g., 200).
    ///      *
    ///      * @throws NS_ERROR_NOT_AVAILABLE if called before the response
    ///      *         has been received (before onStartRequest).
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute unsigned long responseStatus;`
    #[inline]
    pub unsafe fn GetResponseStatus(&self, aResponseStatus: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetResponseStatus)(self, aResponseStatus)
    }


    /// ```text
    /// /**
    ///      * Get the HTTP response status text (e.g., "OK").
    ///      *
    ///      * NOTE: This returns the raw (possibly 8-bit) text from the server.  There
    ///      * are no assumptions made about the charset of the returned text.  You
    ///      * have been warned!
    ///      *
    ///      * @throws NS_ERROR_NOT_AVAILABLE if called before the response
    ///      *         has been received (before onStartRequest).
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute ACString responseStatusText;`
    #[inline]
    pub unsafe fn GetResponseStatusText(&self, aResponseStatusText: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetResponseStatusText)(self, aResponseStatusText)
    }


    /// ```text
    /// /**
    ///      * Returns true if the HTTP response code indicates success.  The value of
    ///      * nsIRequest::status will be NS_OK even when processing a 404 response
    ///      * because a 404 response may include a message body that (in some cases)
    ///      * should be shown to the user.
    ///      *
    ///      * Use this attribute to distinguish server error pages from normal pages,
    ///      * instead of comparing the response status manually against the set of
    ///      * valid response codes, if that is required by your application.
    ///      *
    ///      * @throws NS_ERROR_NOT_AVAILABLE if called before the response
    ///      *         has been received (before onStartRequest).
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute boolean requestSucceeded;`
    #[inline]
    pub unsafe fn GetRequestSucceeded(&self, aRequestSucceeded: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetRequestSucceeded)(self, aRequestSucceeded)
    }


    /// ```text
    /// /** Indicates whether channel should be treated as the main one for the
    ///     *  current document.  If manually set to true, will always remain true.  Otherwise,
    ///     *  will be true if LOAD_DOCUMENT_URI is set in the channel's loadflags.
    ///     */
    /// ```
    ///

    /// `[must_use] attribute boolean isMainDocumentChannel;`
    #[inline]
    pub unsafe fn GetIsMainDocumentChannel(&self, aIsMainDocumentChannel: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsMainDocumentChannel)(self, aIsMainDocumentChannel)
    }


    /// ```text
    /// /** Indicates whether channel should be treated as the main one for the
    ///     *  current document.  If manually set to true, will always remain true.  Otherwise,
    ///     *  will be true if LOAD_DOCUMENT_URI is set in the channel's loadflags.
    ///     */
    /// ```
    ///

    /// `[must_use] attribute boolean isMainDocumentChannel;`
    #[inline]
    pub unsafe fn SetIsMainDocumentChannel(&self, aIsMainDocumentChannel: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetIsMainDocumentChannel)(self, aIsMainDocumentChannel)
    }


    /// ```text
    /// /**
    ///      * Get the value of a particular response header.
    ///      *
    ///      * @param aHeader
    ///      *        The case-insensitive name of the response header to query (e.g.,
        ///      *        "Set-Cookie").
    ///      *
    ///      * @return the value of the response header.
    ///      *
    ///      * @throws NS_ERROR_NOT_AVAILABLE if called before the response
    ///      *         has been received (before onStartRequest) or if the header is
    ///      *         not set in the response.
    ///      */
    /// ```
    ///

    /// `[must_use] ACString getResponseHeader (in ACString header);`
    #[inline]
    pub unsafe fn GetResponseHeader(&self, header: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetResponseHeader)(self, header, _retval)
    }


    /// ```text
    /// /**
    ///      * Set the value of a particular response header.
    ///      *
    ///      * This method allows, for example, the HTML content sink to inform the HTTP
    ///      * channel about HTTP-EQUIV headers found in HTML <META> tags.
    ///      *
    ///      * @param aHeader
    ///      *        The case-insensitive name of the response header to set (e.g.,
        ///      *        "Cache-control").
    ///      * @param aValue
    ///      *        The response header value to set (e.g., "no-cache").
    ///      * @param aMerge
    ///      *        If true, the new header value will be merged with any existing
    ///      *        values for the specified header.  This flag is ignored if the
    ///      *        specified header does not support merging (e.g., the "Content-
        ///      *        Type" header can only have one value).  The list of headers for
    ///      *        which this flag is ignored is an implementation detail.  If this
    ///      *        flag is false, then the header value will be replaced with the
    ///      *        contents of |aValue|.
    ///      *
    ///      * If aValue is empty and aMerge is false, the header will be cleared.
    ///      *
    ///      * @throws NS_ERROR_NOT_AVAILABLE if called before the response
    ///      *         has been received (before onStartRequest).
    ///      * @throws NS_ERROR_ILLEGAL_VALUE if changing the value of this response
    ///      *         header is not allowed.
    ///      * @throws NS_ERROR_FAILURE if called during visitResponseHeaders,
    ///      *         VisitOriginalResponseHeaders or getOriginalResponseHeader.
    ///      */
    /// ```
    ///

    /// `[must_use] void setResponseHeader (in ACString header, in ACString value, in boolean merge);`
    #[inline]
    pub unsafe fn SetResponseHeader(&self, header: *const ::nsstring::nsACString, value: *const ::nsstring::nsACString, merge: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetResponseHeader)(self, header, value, merge)
    }


    /// ```text
    /// /**
    ///      * Call this method to visit all response headers.  Calling
    ///      * setResponseHeader while visiting response headers has undefined
    ///      * behavior.  Don't do it!
    ///      *
    ///      * @param aVisitor
    ///      *        the header visitor instance.
    ///      *
    ///      * @throws NS_ERROR_NOT_AVAILABLE if called before the response
    ///      *         has been received (before onStartRequest).
    ///      */
    /// ```
    ///

    /// `[must_use] void visitResponseHeaders (in nsIHttpHeaderVisitor aVisitor);`
    #[inline]
    pub unsafe fn VisitResponseHeaders(&self, aVisitor: *const nsIHttpHeaderVisitor) -> ::nserror::nsresult {
        ((*self.vtable).VisitResponseHeaders)(self, aVisitor)
    }


    /// ```text
    /// /**
    ///      * Get the value(s) of a particular response header in the form and order
    ///      * it has been received from the remote peer. There can be multiple headers
    ///      * with the same name.
    ///      *
    ///      * @param aHeader
    ///      *        The case-insensitive name of the response header to query (e.g.,
        ///      *        "Set-Cookie").
    ///      *
    ///      * @param aVisitor
    ///      *        the header visitor instance.
    ///      *
    ///      * @throws NS_ERROR_NOT_AVAILABLE if called before the response
    ///      *         has been received (before onStartRequest) or if the header is
    ///      *         not set in the response.
    ///      */
    /// ```
    ///

    /// `[must_use] void getOriginalResponseHeader (in ACString aHeader, in nsIHttpHeaderVisitor aVisitor);`
    #[inline]
    pub unsafe fn GetOriginalResponseHeader(&self, aHeader: *const ::nsstring::nsACString, aVisitor: *const nsIHttpHeaderVisitor) -> ::nserror::nsresult {
        ((*self.vtable).GetOriginalResponseHeader)(self, aHeader, aVisitor)
    }


    /// ```text
    /// /**
    ///      * Call this method to visit all response headers in the form and order as
    ///      * they have been received from the remote peer.
    ///      * Calling setResponseHeader while visiting response headers has undefined
    ///      * behavior.  Don't do it!
    ///      *
    ///      * @param aVisitor
    ///      *        the header visitor instance.
    ///      *
    ///      * @throws NS_ERROR_NOT_AVAILABLE if called before the response
    ///      *         has been received (before onStartRequest).
    ///      */
    /// ```
    ///

    /// `[must_use] void visitOriginalResponseHeaders (in nsIHttpHeaderVisitor aVisitor);`
    #[inline]
    pub unsafe fn VisitOriginalResponseHeaders(&self, aVisitor: *const nsIHttpHeaderVisitor) -> ::nserror::nsresult {
        ((*self.vtable).VisitOriginalResponseHeaders)(self, aVisitor)
    }


    /// ```text
    /// /**
    ///      * Returns true if the server sent a "Cache-Control: no-store" response
    ///      * header.
    ///      *
    ///      * @throws NS_ERROR_NOT_AVAILABLE if called before the response
    ///      *         has been received (before onStartRequest).
    ///      */
    /// ```
    ///

    /// `[must_use] boolean isNoStoreResponse ();`
    #[inline]
    pub unsafe fn IsNoStoreResponse(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsNoStoreResponse)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * Returns true if the server sent the equivalent of a "Cache-control:
    ///      * no-cache" response header.  Equivalent response headers include:
    ///      * "Pragma: no-cache", "Expires: 0", and "Expires" with a date value
    ///      * in the past relative to the value of the "Date" header.
    ///      *
    ///      * @throws NS_ERROR_NOT_AVAILABLE if called before the response
    ///      *         has been received (before onStartRequest).
    ///      */
    /// ```
    ///

    /// `[must_use] boolean isNoCacheResponse ();`
    #[inline]
    pub unsafe fn IsNoCacheResponse(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsNoCacheResponse)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * Returns true if the server sent a "Cache-Control: private" response
    ///      * header.
    ///      *
    ///      * @throws NS_ERROR_NOT_AVAILABLE if called before the response
    ///      *         has been received (before onStartRequest).
    ///      */
    /// ```
    ///

    /// `[must_use] boolean isPrivateResponse ();`
    #[inline]
    pub unsafe fn IsPrivateResponse(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsPrivateResponse)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * Instructs the channel to immediately redirect to a new destination.
    ///      * Can only be called on channels that have not yet called their
    ///      * listener's OnStartRequest(). Generally that means the latest time
    ///      * this can be used is one of:
    ///      *    "http-on-examine-response"
    ///      *    "http-on-examine-merged-response"
    ///      *    "http-on-examine-cached-response"
    ///      *
    ///      * When non-null URL is set before AsyncOpen:
    ///      *  we attempt to redirect to the targetURI before we even start building
    ///      *  and sending the request to the cache or the origin server.
    ///      *  If the redirect is vetoed, we fail the channel.
    ///      *
    ///      * When set between AsyncOpen and first call to OnStartRequest being called:
    ///      *  we attempt to redirect before we start delivery of network or cached
    ///      *  response to the listener.  If vetoed, we continue with delivery of
    ///      *  the original content to the channel listener.
    ///      *
    ///      * When passed aTargetURI is null the channel behaves normally (can be
        ///      * rewritten).
    ///      *
    ///      * This method provides no explicit conflict resolution. The last
    ///      * caller to call it wins.
    ///      *
    ///      * @throws NS_ERROR_NOT_AVAILABLE if called after the channel has already
    ///      *         started to deliver the content to its listener.
    ///      */
    /// ```
    ///

    /// `[must_use] void redirectTo (in nsIURI aTargetURI);`
    #[inline]
    pub unsafe fn RedirectTo(&self, aTargetURI: *const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).RedirectTo)(self, aTargetURI)
    }


    /// ```text
    /// /**
    ///      * Flags a channel to be upgraded to HTTPS.
    ///      *
    ///      * Upgrading to a secure channel must happen before or during
    ///      * "http-on-modify-request". If redirectTo is called early as well, it
    ///      * will win and upgradeToSecure will be a no-op.
    ///      *
    ///      * @throws NS_ERROR_NOT_AVAILABLE if called after the channel has already
    ///      *         started to deliver the content to its listener.
    ///      */
    /// ```
    ///

    /// `[must_use] void upgradeToSecure ();`
    #[inline]
    pub unsafe fn UpgradeToSecure(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).UpgradeToSecure)(self, )
    }


    /// ```text
    /// /**
    ///      * Identifies the request context for this load.
    ///      */
    /// ```
    ///

    /// `[must_use,noscript] attribute uint64_t requestContextID;`
    #[inline]
    pub unsafe fn GetRequestContextID(&self, aRequestContextID: *mut uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetRequestContextID)(self, aRequestContextID)
    }


    /// ```text
    /// /**
    ///      * Identifies the request context for this load.
    ///      */
    /// ```
    ///

    /// `[must_use,noscript] attribute uint64_t requestContextID;`
    #[inline]
    pub unsafe fn SetRequestContextID(&self, aRequestContextID: uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).SetRequestContextID)(self, aRequestContextID)
    }


    /// ```text
    /// /**
    ///      * ID of the top-level document's inner window.  Identifies the content
    ///      * this channels is being load in.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute uint64_t topLevelContentWindowId;`
    #[inline]
    pub unsafe fn GetTopLevelContentWindowId(&self, aTopLevelContentWindowId: *mut uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetTopLevelContentWindowId)(self, aTopLevelContentWindowId)
    }


    /// ```text
    /// /**
    ///      * ID of the top-level document's inner window.  Identifies the content
    ///      * this channels is being load in.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute uint64_t topLevelContentWindowId;`
    #[inline]
    pub unsafe fn SetTopLevelContentWindowId(&self, aTopLevelContentWindowId: uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).SetTopLevelContentWindowId)(self, aTopLevelContentWindowId)
    }



    /// `[infallible] readonly attribute nsIHttpChannel_FlashPluginState flashPluginState;`
    #[inline]
    pub unsafe fn GetFlashPluginState(&self, aFlashPluginState: *mut u8) -> ::nserror::nsresult {
        ((*self.vtable).GetFlashPluginState)(self, aFlashPluginState)
    }


    /// ```text
    /// /**
    ///      * ID of the top-level outer content window. Identifies this channel's
    ///      * top-level window it comes from.
    ///      *
    ///      * NOTE: The setter of this attribute is currently for xpcshell test only.
    ///      *       Don't alter it otherwise.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute uint64_t topLevelOuterContentWindowId;`
    #[inline]
    pub unsafe fn GetTopLevelOuterContentWindowId(&self, aTopLevelOuterContentWindowId: *mut uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetTopLevelOuterContentWindowId)(self, aTopLevelOuterContentWindowId)
    }


    /// ```text
    /// /**
    ///      * ID of the top-level outer content window. Identifies this channel's
    ///      * top-level window it comes from.
    ///      *
    ///      * NOTE: The setter of this attribute is currently for xpcshell test only.
    ///      *       Don't alter it otherwise.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute uint64_t topLevelOuterContentWindowId;`
    #[inline]
    pub unsafe fn SetTopLevelOuterContentWindowId(&self, aTopLevelOuterContentWindowId: uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).SetTopLevelOuterContentWindowId)(self, aTopLevelOuterContentWindowId)
    }


    /// ```text
    /// /**
    ///      * In e10s, the information that the CORS response blocks the load is in the
    ///      * parent, which doesn't know the true window id of the request, so we may
    ///      * need to proxy the request to the child.
    ///      *
    ///      * @param aMessage
    ///      *        The message to print in the console.
    ///      *
    ///      * @param aCategory
    ///      *        The category under which the message should be displayed.
    ///      */
    /// ```
    ///

    /// `void logBlockedCORSRequest (in AString aMessage, in ACString aCategory);`
    #[inline]
    pub unsafe fn LogBlockedCORSRequest(&self, aMessage: *const ::nsstring::nsAString, aCategory: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).LogBlockedCORSRequest)(self, aMessage, aCategory)
    }



    /// `void logMimeTypeMismatch (in ACString aMessageName, in boolean aWarning, in AString aURL, in AString aContentType);`
    #[inline]
    pub unsafe fn LogMimeTypeMismatch(&self, aMessageName: *const ::nsstring::nsACString, aWarning: bool, aURL: *const ::nsstring::nsAString, aContentType: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).LogMimeTypeMismatch)(self, aMessageName, aWarning, aURL, aContentType)
    }


}


