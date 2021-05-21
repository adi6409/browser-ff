//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIRequest.idl
//


/// `typedef uint32_t  nsLoadFlags;`
///


pub type nsLoadFlags = u32;


/// `interface nsIRequest : nsISupports`
///

/// ```text
/// /**
///  * nsIRequest
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIRequest {
    vtable: *const nsIRequestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIRequest.
unsafe impl XpCom for nsIRequest {
    const IID: nsIID = nsID(0xef6bfbd2, 0xfd46, 0x48d8,
        [0x96, 0xb7, 0x9f, 0x8f, 0x0f, 0xd3, 0x87, 0xfe]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIRequest {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIRequest.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIRequestCoerce {
    /// Cheaply cast a value of this type from a `nsIRequest`.
    fn coerce_from(v: &nsIRequest) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIRequestCoerce for nsIRequest {
    #[inline]
    fn coerce_from(v: &nsIRequest) -> &Self {
        v
    }
}

impl nsIRequest {
    /// Cast this `nsIRequest` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIRequestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIRequest {
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
impl<T: nsISupportsCoerce> nsIRequestCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRequest) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIRequest
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIRequestVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AUTF8String name; */
    pub GetName: unsafe extern "system" fn (this: *const nsIRequest, aName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* boolean isPending (); */
    pub IsPending: unsafe extern "system" fn (this: *const nsIRequest, _retval: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute nsresult status; */
    pub GetStatus: unsafe extern "system" fn (this: *const nsIRequest, aStatus: *mut ::nserror::nsresult) -> ::nserror::nsresult,

    /* void cancel (in nsresult aStatus); */
    pub Cancel: unsafe extern "system" fn (this: *const nsIRequest, aStatus: ::nserror::nsresult) -> ::nserror::nsresult,

    /* void suspend (); */
    pub Suspend: unsafe extern "system" fn (this: *const nsIRequest) -> ::nserror::nsresult,

    /* void resume (); */
    pub Resume: unsafe extern "system" fn (this: *const nsIRequest) -> ::nserror::nsresult,

    /* attribute nsILoadGroup loadGroup; */
    pub GetLoadGroup: unsafe extern "system" fn (this: *const nsIRequest, aLoadGroup: *mut*const nsILoadGroup) -> ::nserror::nsresult,

    /* attribute nsILoadGroup loadGroup; */
    pub SetLoadGroup: unsafe extern "system" fn (this: *const nsIRequest, aLoadGroup: *const nsILoadGroup) -> ::nserror::nsresult,

    /* attribute nsLoadFlags loadFlags; */
    pub GetLoadFlags: unsafe extern "system" fn (this: *const nsIRequest, aLoadFlags: *mut nsLoadFlags) -> ::nserror::nsresult,

    /* attribute nsLoadFlags loadFlags; */
    pub SetLoadFlags: unsafe extern "system" fn (this: *const nsIRequest, aLoadFlags: nsLoadFlags) -> ::nserror::nsresult,

    /* nsIRequest_TRRMode getTRRMode (); */
    pub GetTRRMode: unsafe extern "system" fn (this: *const nsIRequest, _retval: *mut u8) -> ::nserror::nsresult,

    /* void setTRRMode (in nsIRequest_TRRMode mode); */
    pub SetTRRMode: unsafe extern "system" fn (this: *const nsIRequest, mode:  u8) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIRequest {
    /// ```text
    /// /**
    ///      * Mask defining the bits reserved for nsIRequest LoadFlags
    ///      */
    /// ```
    ///

    pub const LOAD_REQUESTMASK: i64 = 65535;

    /// ```text
    /// /**************************************************************************
    ///      * Listed below are the various load flags which may be or'd together.
    ///      */
    /// /**
    ///      * No special load flags:
    ///      */
    /// ```
    ///

    pub const LOAD_NORMAL: i64 = 0;

    /// ```text
    /// /**
    ///      * Do not deliver status notifications to the nsIProgressEventSink and
    ///      * do not block the loadgroup from completing (should this load belong to one).
    ///      * Note: Progress notifications will still be delivered.
    ///      */
    /// ```
    ///

    pub const LOAD_BACKGROUND: i64 = 1;

    /// ```text
    /// /**
    ///      * This flag marks the request as being made to load the data for an html
    ///      * <object> tag. This means that the LOAD_DOCUMENT_URI flag may be set after
    ///      * the channel has been provided with the MIME type.
    ///      */
    /// ```
    ///

    pub const LOAD_HTML_OBJECT_DATA: i64 = 2;

    /// ```text
    /// /**
    ///      * This flag marks the request as belonging to a document that requires access
    ///      * to the document.cookies API.
    ///      */
    /// ```
    ///

    pub const LOAD_DOCUMENT_NEEDS_COOKIE: i64 = 4;

    /// ```text
    /// /**
    ///      * These two bits encode the TRR mode.
    ///      * Do not get/set manually, rather use the getTRRMode/setTRRMode methods.
    ///      */
    /// ```
    ///

    pub const LOAD_TRR_MASK: i64 = 24;


    pub const LOAD_TRR_DISABLED_MODE: i64 = 8;


    pub const LOAD_TRR_FIRST_MODE: i64 = 16;


    pub const LOAD_TRR_ONLY_MODE: i64 = 24;

    /// ```text
    /// /**
    ///      * This is used for a temporary workaround for a web-compat issue. The flag is
    ///      * only set on CORS preflight request to allowed sending client certificates
    ///      * on a connection for an anonymous request.
    ///      */
    /// ```
    ///

    pub const LOAD_ANONYMOUS_ALLOW_CLIENT_CERT: i64 = 32;

    /// ```text
    /// /**************************************************************************
    ///      * The following flags control the flow of data into the cache.
    ///      */
    /// /**
    ///      * This flag prevents caching of any kind.  It does not, however, prevent
    ///      * cached content from being used to satisfy this request.
    ///      */
    /// ```
    ///

    pub const INHIBIT_CACHING: i64 = 128;

    /// ```text
    /// /**
    ///      * This flag prevents caching on disk (or other persistent media), which
    ///      * may be needed to preserve privacy.
    ///      */
    /// ```
    ///

    pub const INHIBIT_PERSISTENT_CACHING: i64 = 256;

    /// ```text
    /// /**************************************************************************
    ///      * The following flags control what happens when the cache contains data
    ///      * that could perhaps satisfy this request.  They are listed in descending
    ///      * order of precidence.
    ///      */
    /// /**
    ///      * Force an end-to-end download of content data from the origin server.
    ///      * This flag is used for a shift-reload.
    ///      */
    /// ```
    ///

    pub const LOAD_BYPASS_CACHE: i64 = 512;

    /// ```text
    /// /**
    ///      * Attempt to force a load from the cache, bypassing ALL validation logic
    ///      * (note: this is stronger than VALIDATE_NEVER, which still validates for
        ///      * certain conditions).
    ///      *
    ///      * If the resource is not present in cache, it will be loaded from the
    ///      * network.  Combine this flag with LOAD_ONLY_FROM_CACHE if you wish to
    ///      * perform cache-only loads without validation checks.
    ///      *
    ///      * This flag is used when browsing via history.  It is not recommended for
    ///      * normal browsing as it may likely violate reasonable assumptions made by
    ///      * the server and confuse users.
    ///      */
    /// ```
    ///

    pub const LOAD_FROM_CACHE: i64 = 1024;

    /// ```text
    /// /**
    ///      * The following flags control the frequency of cached content validation
    ///      * when neither LOAD_BYPASS_CACHE or LOAD_FROM_CACHE are set.  By default,
    ///      * cached content is automatically validated if necessary before reuse.
    ///      *
    ///      * VALIDATE_ALWAYS forces validation of any cached content independent of
    ///      * its expiration time (unless it is https with Cache-Control: immutable)
    ///      *
    ///      * VALIDATE_NEVER disables validation of cached content, unless it arrived
    ///      * with the "Cache: no-store" header, or arrived via HTTPS with the
    ///      * "Cache: no-cache" header.
    ///      *
    ///      * VALIDATE_ONCE_PER_SESSION disables validation of expired content,
    ///      * provided it has already been validated (at least once) since the start
    ///      * of this session.
    ///      *
    ///      * NOTE TO IMPLEMENTORS:
    ///      *   These flags are intended for normal browsing, and they should therefore
    ///      *   not apply to content that must be validated before each use.  Consider,
    ///      *   for example, a HTTP response with a "Cache-control: no-cache" header.
    ///      *   According to RFC2616, this response must be validated before it can
    ///      *   be taken from a cache.  Breaking this requirement could result in
    ///      *   incorrect and potentially undesirable side-effects.
    ///      */
    /// ```
    ///

    pub const VALIDATE_ALWAYS: i64 = 2048;


    pub const VALIDATE_NEVER: i64 = 4096;


    pub const VALIDATE_ONCE_PER_SESSION: i64 = 8192;

    /// ```text
    /// /**
    ///      * When set, this flag indicates that no user-specific data should be added
    ///      * to the request when opened. This means that things like authorization
    ///      * tokens or cookie headers should not be added.
    ///      */
    /// ```
    ///

    pub const LOAD_ANONYMOUS: i64 = 16384;

    /// ```text
    /// /**
    ///      * When set, this flag indicates that caches of network connections,
    ///      * particularly HTTP persistent connections, should not be used.
    ///      * Use this together with LOAD_INITIAL_DOCUMENT_URI as otherwise it has no
    ///      * effect.
    ///      */
    /// ```
    ///

    pub const LOAD_FRESH_CONNECTION: i64 = 32768;

    /// ```text
    /// /**
    ///      * The name of the request.  Often this is the URI of the request.
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String name;`
    #[inline]
    pub unsafe fn GetName(&self, aName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetName)(self, aName)
    }


    /// ```text
    /// /**
    ///      * Indicates whether the request is pending. nsIRequest::isPending is
    ///      * true when there is an outstanding asynchronous event that will make
    ///      * the request no longer be pending.  Requests do not necessarily start
    ///      * out pending; in some cases, requests have to be explicitly initiated
    ///      * (e.g. nsIChannel implementations are only pending once asyncOpen
        ///      * returns successfully).
    ///      *
    ///      * Requests can become pending multiple times during their lifetime.
    ///      *
    ///      * @return TRUE if the request has yet to reach completion.
    ///      * @return FALSE if the request has reached completion (e.g., after
        ///      *   OnStopRequest has fired).
    ///      * @note Suspended requests are still considered pending.
    ///      */
    /// ```
    ///

    /// `boolean isPending ();`
    #[inline]
    pub unsafe fn IsPending(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsPending)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * The error status associated with the request.
    ///      */
    /// ```
    ///

    /// `readonly attribute nsresult status;`
    #[inline]
    pub unsafe fn GetStatus(&self, aStatus: *mut ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).GetStatus)(self, aStatus)
    }


    /// ```text
    /// /**
    ///      * Cancels the current request.  This will close any open input or
    ///      * output streams and terminate any async requests.  Users should
    ///      * normally pass NS_BINDING_ABORTED, although other errors may also
    ///      * be passed.  The error passed in will become the value of the
    ///      * status attribute.
    ///      *
    ///      * Implementations must not send any notifications (e.g. via
        ///      * nsIRequestObserver) synchronously from this function. Similarly,
    ///      * removal from the load group (if any) must also happen asynchronously.
    ///      *
    ///      * Requests that use nsIStreamListener must not call onDataAvailable
    ///      * anymore after cancel has been called.
    ///      *
    ///      * @param aStatus the reason for canceling this request.
    ///      *
    ///      * NOTE: most nsIRequest implementations expect aStatus to be a
    ///      * failure code; however, some implementations may allow aStatus to
    ///      * be a success code such as NS_OK.  In general, aStatus should be
    ///      * a failure code.
    ///      */
    /// ```
    ///

    /// `void cancel (in nsresult aStatus);`
    #[inline]
    pub unsafe fn Cancel(&self, aStatus: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).Cancel)(self, aStatus)
    }


    /// ```text
    /// /**
    ///      * Suspends the current request.  This may have the effect of closing
    ///      * any underlying transport (in order to free up resources), although
    ///      * any open streams remain logically opened and will continue delivering
    ///      * data when the transport is resumed.
    ///      *
    ///      * Calling cancel() on a suspended request must not send any
    ///      * notifications (such as onstopRequest) until the request is resumed.
    ///      *
    ///      * NOTE: some implementations are unable to immediately suspend, and
    ///      * may continue to deliver events already posted to an event queue. In
    ///      * general, callers should be capable of handling events even after
    ///      * suspending a request.
    ///      */
    /// ```
    ///

    /// `void suspend ();`
    #[inline]
    pub unsafe fn Suspend(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Suspend)(self, )
    }


    /// ```text
    /// /**
    ///      * Resumes the current request.  This may have the effect of re-opening
    ///      * any underlying transport and will resume the delivery of data to
    ///      * any open streams.
    ///      */
    /// ```
    ///

    /// `void resume ();`
    #[inline]
    pub unsafe fn Resume(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Resume)(self, )
    }


    /// ```text
    /// /**
    ///      * The load group of this request.  While pending, the request is a
    ///      * member of the load group.  It is the responsibility of the request
    ///      * to implement this policy.
    ///      */
    /// ```
    ///

    /// `attribute nsILoadGroup loadGroup;`
    #[inline]
    pub unsafe fn GetLoadGroup(&self, aLoadGroup: *mut*const nsILoadGroup) -> ::nserror::nsresult {
        ((*self.vtable).GetLoadGroup)(self, aLoadGroup)
    }


    /// ```text
    /// /**
    ///      * The load group of this request.  While pending, the request is a
    ///      * member of the load group.  It is the responsibility of the request
    ///      * to implement this policy.
    ///      */
    /// ```
    ///

    /// `attribute nsILoadGroup loadGroup;`
    #[inline]
    pub unsafe fn SetLoadGroup(&self, aLoadGroup: *const nsILoadGroup) -> ::nserror::nsresult {
        ((*self.vtable).SetLoadGroup)(self, aLoadGroup)
    }


    /// ```text
    /// /**
    ///      * The load flags of this request.  Bits 0-15 are reserved.
    ///      *
    ///      * When added to a load group, this request's load flags are merged with
    ///      * the load flags of the load group.
    ///      */
    /// ```
    ///

    /// `attribute nsLoadFlags loadFlags;`
    #[inline]
    pub unsafe fn GetLoadFlags(&self, aLoadFlags: *mut nsLoadFlags) -> ::nserror::nsresult {
        ((*self.vtable).GetLoadFlags)(self, aLoadFlags)
    }


    /// ```text
    /// /**
    ///      * The load flags of this request.  Bits 0-15 are reserved.
    ///      *
    ///      * When added to a load group, this request's load flags are merged with
    ///      * the load flags of the load group.
    ///      */
    /// ```
    ///

    /// `attribute nsLoadFlags loadFlags;`
    #[inline]
    pub unsafe fn SetLoadFlags(&self, aLoadFlags: nsLoadFlags) -> ::nserror::nsresult {
        ((*self.vtable).SetLoadFlags)(self, aLoadFlags)
    }


    /// ```text
    /// /**
    ///      * These methods encode/decode the TRR mode to/from the loadFlags.
    ///      * Helper methods Get/SetTRRModeImpl are provided so implementations don't
    ///      * need to duplicate code.
    ///      *
    ///      * Requests with TRR_DEFAULT_MODE will use the mode indicated by the pref
    ///      *   - see network.trr.mode in all.js
    ///      * Requests with TRR_DISABLED_MODE will always use native DNS, even if the
    ///      *   pref is set to mode3 (TRR-only).
    ///      * Requests with TRR_DISABLED_MODE will first use TRR then fallback to
    ///      *   regular DNS, unless TRR is disabled by setting the pref to mode5,
    ///      *   parental control being enabled, or the domain being in the exclusion
    ///      *   list.
    ///      * Requests with TRR_ONLY_MODE will only use TRR, unless not allowed by
    ///      *   the same conditions mentioned above.
    ///      */
    /// ```
    ///

    /// `nsIRequest_TRRMode getTRRMode ();`
    #[inline]
    pub unsafe fn GetTRRMode(&self, _retval: *mut u8) -> ::nserror::nsresult {
        ((*self.vtable).GetTRRMode)(self, _retval)
    }



    /// `void setTRRMode (in nsIRequest_TRRMode mode);`
    #[inline]
    pub unsafe fn SetTRRMode(&self, mode:  u8) -> ::nserror::nsresult {
        ((*self.vtable).SetTRRMode)(self, mode)
    }


}


