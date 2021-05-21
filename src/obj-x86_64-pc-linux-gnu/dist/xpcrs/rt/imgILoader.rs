//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/image/imgILoader.idl
//


/// `interface imgILoader : nsISupports`
///

/// ```text
/// /**
///  * imgILoader interface
///  *
///  * @author Stuart Parmenter <pavlov@netscape.com>
///  * @version 0.3
///  * @see imagelib2
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct imgILoader {
    vtable: *const imgILoaderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for imgILoader.
unsafe impl XpCom for imgILoader {
    const IID: nsIID = nsID(0xe61377d2, 0x910e, 0x4c65,
        [0xa6, 0x4b, 0x42, 0x8d, 0x15, 0x0e, 0x1f, 0xd1]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for imgILoader {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from imgILoader.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait imgILoaderCoerce {
    /// Cheaply cast a value of this type from a `imgILoader`.
    fn coerce_from(v: &imgILoader) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl imgILoaderCoerce for imgILoader {
    #[inline]
    fn coerce_from(v: &imgILoader) -> &Self {
        v
    }
}

impl imgILoader {
    /// Cast this `imgILoader` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: imgILoaderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for imgILoader {
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
impl<T: nsISupportsCoerce> imgILoaderCoerce for T {
    #[inline]
    fn coerce_from(v: &imgILoader) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every imgILoader
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct imgILoaderVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* imgIRequest loadImageXPCOM (in nsIURI aURI, in nsIURI aInitialDocumentURL, in nsIReferrerInfo aReferrerInfo, in nsIPrincipal aLoadingPrincipal, in nsILoadGroup aLoadGroup, in imgINotificationObserver aObserver, in Document aLoadingDocument, in nsLoadFlags aLoadFlags, in nsISupports cacheKey, [optional] in nsContentPolicyType aContentPolicyType); */
    pub LoadImageXPCOM: unsafe extern "system" fn (this: *const imgILoader, aURI: *const nsIURI, aInitialDocumentURL: *const nsIURI, aReferrerInfo: *const nsIReferrerInfo, aLoadingPrincipal: *const nsIPrincipal, aLoadGroup: *const nsILoadGroup, aObserver: *const imgINotificationObserver, aLoadingDocument: *const libc::c_void, aLoadFlags: nsLoadFlags, cacheKey: *const nsISupports, aContentPolicyType: nsContentPolicyType, _retval: *mut*const imgIRequest) -> ::nserror::nsresult,

    /* imgIRequest loadImageWithChannelXPCOM (in nsIChannel aChannel, in imgINotificationObserver aObserver, in Document aLoadingDocument, out nsIStreamListener aListener); */
    pub LoadImageWithChannelXPCOM: unsafe extern "system" fn (this: *const imgILoader, aChannel: *const nsIChannel, aObserver: *const imgINotificationObserver, aLoadingDocument: *const libc::c_void, aListener: *mut*const nsIStreamListener, _retval: *mut*const imgIRequest) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl imgILoader {

    pub const LOAD_CORS_ANONYMOUS: i64 = 65536;


    pub const LOAD_CORS_USE_CREDENTIALS: i64 = 131072;

    /// ```text
    /// /**
    ///    * Start the load and decode of an image.
    ///    * @param aURI the URI to load
    ///    * @param aInitialDocumentURI the URI that 'initiated' the load -- used for
    ///    *           3rd party cookie blocking
    ///    * @param aReferrerInfo the referrer info to compute sending referrer.
    ///    * @param aLoadingPrincipal the principal of the loading document
    ///    * @param aLoadGroup Loadgroup to put the image load into
    ///    * @param aObserver the observer (may be null)
    ///    * @param aLoadingDocument loading document
    ///    * @param aLoadFlags Load flags for the request
    ///    * @param aCacheKey cache key to use for a load if the original
    ///    *                  image came from a request that had post data
    ///    * @param aContentPolicyType [optional] the nsContentPolicyType to
    ///    *                           use for this load. Defaults to
    ///    *                           nsIContentPolicy::TYPE_IMAGE
    ///
    ///
    ///    * ImageLib does NOT keep a strong ref to the observer; this prevents
    ///    * reference cycles.  This means that callers of loadImage should
    ///    * make sure to Cancel() the resulting request before the observer
    ///    * goes away.
    ///    */
    /// ```
    ///

    /// `imgIRequest loadImageXPCOM (in nsIURI aURI, in nsIURI aInitialDocumentURL, in nsIReferrerInfo aReferrerInfo, in nsIPrincipal aLoadingPrincipal, in nsILoadGroup aLoadGroup, in imgINotificationObserver aObserver, in Document aLoadingDocument, in nsLoadFlags aLoadFlags, in nsISupports cacheKey, [optional] in nsContentPolicyType aContentPolicyType);`
    #[inline]
    pub unsafe fn LoadImageXPCOM(&self, aURI: *const nsIURI, aInitialDocumentURL: *const nsIURI, aReferrerInfo: *const nsIReferrerInfo, aLoadingPrincipal: *const nsIPrincipal, aLoadGroup: *const nsILoadGroup, aObserver: *const imgINotificationObserver, aLoadingDocument: *const libc::c_void, aLoadFlags: nsLoadFlags, cacheKey: *const nsISupports, aContentPolicyType: nsContentPolicyType, _retval: *mut*const imgIRequest) -> ::nserror::nsresult {
        ((*self.vtable).LoadImageXPCOM)(self, aURI, aInitialDocumentURL, aReferrerInfo, aLoadingPrincipal, aLoadGroup, aObserver, aLoadingDocument, aLoadFlags, cacheKey, aContentPolicyType, _retval)
    }


    /// ```text
    /// /**
    ///    * Start the load and decode of an image.
    ///    * @param aChannel the channel to load the image from.  This must
    ///    *                 already be opened before this method is called, and there
    ///    *                 must have been no OnDataAvailable calls for it yet.
    ///    * @param aObserver the observer (may be null)
    ///    * @param aLoadingDocument loading document
    ///    * @param aListener [out]
    ///    *        A listener that you must send the channel's notifications and data
    ///    *        to.  Can be null, in which case imagelib has found a cached image
    ///    *        and is not interested in the data. @aChannel will be canceled for
    ///    *        you in this case.
    ///    *
    ///    * ImageLib does NOT keep a strong ref to the observer; this prevents
    ///    * reference cycles.  This means that callers of loadImageWithChannel should
    ///    * make sure to Cancel() the resulting request before the observer goes away.
    ///    */
    /// ```
    ///

    /// `imgIRequest loadImageWithChannelXPCOM (in nsIChannel aChannel, in imgINotificationObserver aObserver, in Document aLoadingDocument, out nsIStreamListener aListener);`
    #[inline]
    pub unsafe fn LoadImageWithChannelXPCOM(&self, aChannel: *const nsIChannel, aObserver: *const imgINotificationObserver, aLoadingDocument: *const libc::c_void, aListener: *mut*const nsIStreamListener, _retval: *mut*const imgIRequest) -> ::nserror::nsresult {
        ((*self.vtable).LoadImageWithChannelXPCOM)(self, aChannel, aObserver, aLoadingDocument, aListener, _retval)
    }


}


