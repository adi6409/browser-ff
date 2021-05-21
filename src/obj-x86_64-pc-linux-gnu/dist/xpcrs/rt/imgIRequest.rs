//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/image/imgIRequest.idl
//


/// `interface imgIRequest : nsIRequest`
///

/// ```text
/// /**
///  * imgIRequest interface
///  *
///  * @author Stuart Parmenter <stuart@mozilla.com>
///  * @version 0.1
///  * @see imagelib2
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct imgIRequest {
    vtable: *const imgIRequestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for imgIRequest.
unsafe impl XpCom for imgIRequest {
    const IID: nsIID = nsID(0xdb0a945c, 0x3883, 0x424a,
        [0x98, 0xd0, 0x2e, 0xe0, 0x52, 0x3b, 0x02, 0x55]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for imgIRequest {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from imgIRequest.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait imgIRequestCoerce {
    /// Cheaply cast a value of this type from a `imgIRequest`.
    fn coerce_from(v: &imgIRequest) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl imgIRequestCoerce for imgIRequest {
    #[inline]
    fn coerce_from(v: &imgIRequest) -> &Self {
        v
    }
}

impl imgIRequest {
    /// Cast this `imgIRequest` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: imgIRequestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for imgIRequest {
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
impl<T: nsIRequestCoerce> imgIRequestCoerce for T {
    #[inline]
    fn coerce_from(v: &imgIRequest) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every imgIRequest
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct imgIRequestVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIRequestVTable,

    /* readonly attribute imgIContainer image; */
    pub GetImage: unsafe extern "system" fn (this: *const imgIRequest, aImage: *mut *const imgIContainer) -> ::nserror::nsresult,

    /* [infallible] readonly attribute unsigned long producerId; */
    pub GetProducerId: unsafe extern "system" fn (this: *const imgIRequest, aProducerId: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute unsigned long imageStatus; */
    pub GetImageStatus: unsafe extern "system" fn (this: *const imgIRequest, aImageStatus: *mut u32) -> ::nserror::nsresult,

    /* [noscript] readonly attribute nsresult imageErrorCode; */
    pub GetImageErrorCode: unsafe extern "system" fn (this: *const imgIRequest, aImageErrorCode: *mut ::nserror::nsresult) -> ::nserror::nsresult,

    /* readonly attribute nsIURI URI; */
    pub GetURI: unsafe extern "system" fn (this: *const imgIRequest, aURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* readonly attribute nsIURI finalURI; */
    pub GetFinalURI: unsafe extern "system" fn (this: *const imgIRequest, aFinalURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* readonly attribute imgINotificationObserver notificationObserver; */
    pub GetNotificationObserver: unsafe extern "system" fn (this: *const imgIRequest, aNotificationObserver: *mut*const imgINotificationObserver) -> ::nserror::nsresult,

    /* readonly attribute string mimeType; */
    pub GetMimeType: unsafe extern "system" fn (this: *const imgIRequest, aMimeType: *mut *const libc::c_char) -> ::nserror::nsresult,

    /* imgIRequest clone (in imgINotificationObserver aObserver); */
    pub Clone: unsafe extern "system" fn (this: *const imgIRequest, aObserver: *const imgINotificationObserver, _retval: *mut *const imgIRequest) -> ::nserror::nsresult,

    /* readonly attribute nsIPrincipal imagePrincipal; */
    pub GetImagePrincipal: unsafe extern "system" fn (this: *const imgIRequest, aImagePrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult,

    /* readonly attribute bool hadCrossOriginRedirects; */
    pub GetHadCrossOriginRedirects: unsafe extern "system" fn (this: *const imgIRequest, aHadCrossOriginRedirects: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute bool multipart; */
    pub GetMultipart: unsafe extern "system" fn (this: *const imgIRequest, aMultipart: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute long CORSMode; */
    pub GetCORSMode: unsafe extern "system" fn (this: *const imgIRequest, aCORSMode: *mut i32) -> ::nserror::nsresult,

    /* void cancelAndForgetObserver (in nsresult aStatus); */
    pub CancelAndForgetObserver: unsafe extern "system" fn (this: *const imgIRequest, aStatus: ::nserror::nsresult) -> ::nserror::nsresult,

    /* void startDecoding (in uint32_t aFlags); */
    pub StartDecoding: unsafe extern "system" fn (this: *const imgIRequest, aFlags: uint32_t) -> ::nserror::nsresult,

    /* [noscript,notxpcom] boolean startDecodingWithResult (in uint32_t aFlags); */
    pub StartDecodingWithResult: unsafe extern "system" fn (this: *const imgIRequest, aFlags: uint32_t) -> bool,

    /* [noscript,notxpcom] imgIContainer_DecodeResult requestDecodeWithResult (in uint32_t aFlags); */
    pub RequestDecodeWithResult: unsafe extern "system" fn (this: *const imgIRequest, aFlags: uint32_t) -> u8,

    /* void lockImage (); */
    pub LockImage: unsafe extern "system" fn (this: *const imgIRequest) -> ::nserror::nsresult,

    /* void unlockImage (); */
    pub UnlockImage: unsafe extern "system" fn (this: *const imgIRequest) -> ::nserror::nsresult,

    /* void requestDiscard (); */
    pub RequestDiscard: unsafe extern "system" fn (this: *const imgIRequest) -> ::nserror::nsresult,

    /* imgIRequest getStaticRequest (); */
    pub GetStaticRequest: unsafe extern "system" fn (this: *const imgIRequest, _retval: *mut *const imgIRequest) -> ::nserror::nsresult,

    /* void incrementAnimationConsumers (); */
    pub IncrementAnimationConsumers: unsafe extern "system" fn (this: *const imgIRequest) -> ::nserror::nsresult,

    /* void decrementAnimationConsumers (); */
    pub DecrementAnimationConsumers: unsafe extern "system" fn (this: *const imgIRequest) -> ::nserror::nsresult,

    /* void boostPriority (in uint32_t aCategory); */
    pub BoostPriority: unsafe extern "system" fn (this: *const imgIRequest, aCategory: uint32_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl imgIRequest {
    /// ```text
    /// /**
    ///    * Bits set in the return value from imageStatus
    ///    * @name statusflags
    ///    *
    ///    * Meanings:
    ///    *
    ///    * STATUS_NONE: Nothing to report.
    ///    *
    ///    * STATUS_SIZE_AVAILABLE: We received enough image data
    ///    * from the network or filesystem that we know the width
    ///    * and height of the image, and have thus called SetSize()
    ///    * on the container.
    ///    *
    ///    * STATUS_LOAD_COMPLETE: The data has been fully loaded
    ///    * to memory, but not necessarily fully decoded.
    ///    *
    ///    * STATUS_ERROR: An error occurred loading the image.
    ///    *
    ///    * STATUS_FRAME_COMPLETE: The first frame has been
    ///    * completely decoded.
    ///    *
    ///    * STATUS_DECODE_COMPLETE: The whole image has been decoded.
    ///    *
    ///    * STATUS_IS_ANIMATED: The image is animated.
    ///    *
    ///    * STATUS_HAS_TRANSPARENCY: The image is partially or completely transparent.
    ///    */
    /// ```
    ///

    pub const STATUS_NONE: i64 = 0;


    pub const STATUS_SIZE_AVAILABLE: i64 = 1;


    pub const STATUS_LOAD_COMPLETE: i64 = 2;


    pub const STATUS_ERROR: i64 = 4;


    pub const STATUS_FRAME_COMPLETE: i64 = 8;


    pub const STATUS_DECODE_COMPLETE: i64 = 16;


    pub const STATUS_IS_ANIMATED: i64 = 32;


    pub const STATUS_HAS_TRANSPARENCY: i64 = 64;

    /// ```text
    /// /**
    ///    * CORS modes images can be loaded with.
    ///    *
    ///    * By default, all images are loaded with CORS_NONE and cannot be used
    ///    * cross-origin in context like WebGL.
    ///    *
    ///    * If an HTML img element has the crossorigin attribute set, the imgIRequest
    ///    * will be validated for cross-origin usage with CORS, and, if successful,
    ///    * will have its CORS mode set to the relevant type.
    ///    */
    /// ```
    ///

    pub const CORS_NONE: i64 = 1;


    pub const CORS_ANONYMOUS: i64 = 2;


    pub const CORS_USE_CREDENTIALS: i64 = 3;

    /// ```text
    /// /**
    ///    * Request loading priority boost to requested category, each category
    ///    * of request increases priority only one time.
    ///    *
    ///    * CATEGORY_FRAME_INIT: increase priority when the imgRequest is associated
    ///    * with an nsImageFrame.
    ///    *
    ///    * CATEGORY_FRAME_STYLE: increase priority when the imgRequest is for a CSS
    ///    * background-image, list-style-image, etc. on a ComputedStyle, and a frame
    ///    * has been assigned this ComputedStyle.
    ///    *
    ///    * CATEGORY_SIZE_QUERY: increase priority when size decoding is necessary to
    ///    * determine the layout size of an associated nsImageFrame.
    ///    *
    ///    * CATEGORY_DISPLAY: increase priority when the image is about to be displayed
    ///    * in the viewport.
    ///    */
    /// ```
    ///

    pub const CATEGORY_FRAME_INIT: i64 = 1;


    pub const CATEGORY_FRAME_STYLE: i64 = 2;


    pub const CATEGORY_SIZE_QUERY: i64 = 4;


    pub const CATEGORY_DISPLAY: i64 = 8;

    /// ```text
    /// /**
    ///    * the image container...
    ///    * @return the image object associated with the request.
    ///    * @attention NEED DOCS
    ///    */
    /// ```
    ///

    /// `readonly attribute imgIContainer image;`
    #[inline]
    pub unsafe fn GetImage(&self, aImage: *mut *const imgIContainer) -> ::nserror::nsresult {
        ((*self.vtable).GetImage)(self, aImage)
    }


    /// ```text
    /// /**
    ///    * Producer ID for image containers created by this image.
    ///    */
    /// ```
    ///

    /// `[infallible] readonly attribute unsigned long producerId;`
    #[inline]
    pub unsafe fn GetProducerId(&self) -> u32 {
        let mut result = <u32 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetProducerId)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///    * Status flags of the STATUS_* variety.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long imageStatus;`
    #[inline]
    pub unsafe fn GetImageStatus(&self, aImageStatus: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetImageStatus)(self, aImageStatus)
    }



    /// `[noscript] readonly attribute nsresult imageErrorCode;`
    #[inline]
    pub unsafe fn GetImageErrorCode(&self, aImageErrorCode: *mut ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).GetImageErrorCode)(self, aImageErrorCode)
    }


    /// ```text
    /// /**
    ///    * The URI the image load was started with.  Note that this might not be the
    ///    * actual URI for the image (e.g. if HTTP redirects happened during the
        ///    * load).
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIURI URI;`
    #[inline]
    pub unsafe fn GetURI(&self, aURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetURI)(self, aURI)
    }


    /// ```text
    /// /**
    ///    * The URI of the resource we ended up loading after all redirects, etc.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIURI finalURI;`
    #[inline]
    pub unsafe fn GetFinalURI(&self, aFinalURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetFinalURI)(self, aFinalURI)
    }



    /// `readonly attribute imgINotificationObserver notificationObserver;`
    #[inline]
    pub unsafe fn GetNotificationObserver(&self, aNotificationObserver: *mut*const imgINotificationObserver) -> ::nserror::nsresult {
        ((*self.vtable).GetNotificationObserver)(self, aNotificationObserver)
    }



    /// `readonly attribute string mimeType;`
    #[inline]
    pub unsafe fn GetMimeType(&self, aMimeType: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).GetMimeType)(self, aMimeType)
    }


    /// ```text
    /// /**
    ///    * Clone this request; the returned request will have aObserver as the
    ///    * observer.  aObserver will be notified synchronously (before the clone()
        ///    * call returns) with all the notifications that have already been dispatched
    ///    * for this image load.
    ///    */
    /// ```
    ///

    /// `imgIRequest clone (in imgINotificationObserver aObserver);`
    #[inline]
    pub unsafe fn Clone(&self, aObserver: *const imgINotificationObserver, _retval: *mut *const imgIRequest) -> ::nserror::nsresult {
        ((*self.vtable).Clone)(self, aObserver, _retval)
    }


    /// ```text
    /// /**
    ///    * The principal gotten from the channel the image was loaded from.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIPrincipal imagePrincipal;`
    #[inline]
    pub unsafe fn GetImagePrincipal(&self, aImagePrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).GetImagePrincipal)(self, aImagePrincipal)
    }


    /// ```text
    /// /**
    ///    * true if the loading of the image required cross-origin redirects.
    ///    */
    /// ```
    ///

    /// `readonly attribute bool hadCrossOriginRedirects;`
    #[inline]
    pub unsafe fn GetHadCrossOriginRedirects(&self, aHadCrossOriginRedirects: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetHadCrossOriginRedirects)(self, aHadCrossOriginRedirects)
    }


    /// ```text
    /// /**
    ///    * Whether the request is multipart (ie, multipart/x-mixed-replace)
    ///    */
    /// ```
    ///

    /// `readonly attribute bool multipart;`
    #[inline]
    pub unsafe fn GetMultipart(&self, aMultipart: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetMultipart)(self, aMultipart)
    }


    /// ```text
    /// /**
    ///    * The CORS mode that this image was loaded with.
    ///    */
    /// ```
    ///

    /// `readonly attribute long CORSMode;`
    #[inline]
    pub unsafe fn GetCORSMode(&self, aCORSMode: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetCORSMode)(self, aCORSMode)
    }


    /// ```text
    /// /**
    ///    * Cancels this request as in nsIRequest::Cancel(); further, also nulls out
    ///    * decoderObserver so it gets no further notifications from us.
    ///    *
    ///    * NOTE: You should not use this in any new code; instead, use cancel(). Note
    ///    * that cancel() is asynchronous, which means that some time after you call
    ///    * it, the listener/observer will get an OnStopRequest(). This means that, if
    ///    * you're the observer, you can't call cancel() from your destructor.
    ///    */
    /// ```
    ///

    /// `void cancelAndForgetObserver (in nsresult aStatus);`
    #[inline]
    pub unsafe fn CancelAndForgetObserver(&self, aStatus: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).CancelAndForgetObserver)(self, aStatus)
    }


    /// ```text
    /// /**
    ///    * Requests a synchronous decode for the image.
    ///    *
    ///    * imgIContainer has a startDecoding() method, but callers may want to request
    ///    * a decode before the container has necessarily been instantiated. Calling
    ///    * startDecoding() on the imgIRequest simply forwards along the request if the
    ///    * container already exists, or calls it once the container becomes available
    ///    * if it does not yet exist.
    ///    */
    /// ```
    ///

    /// `void startDecoding (in uint32_t aFlags);`
    #[inline]
    pub unsafe fn StartDecoding(&self, aFlags: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).StartDecoding)(self, aFlags)
    }


    /// ```text
    /// /**
    ///    * Exactly like startDecoding above except returns whether the current frame
    ///    * of the image is complete or not.
    ///    *
    ///    * @param aFlags Flags of the FLAG_* variety. Only FLAG_ASYNC_NOTIFY
    ///    *               is accepted; all others are ignored.
    ///    */
    /// ```
    ///

    /// `[noscript,notxpcom] boolean startDecodingWithResult (in uint32_t aFlags);`
    #[inline]
    pub unsafe fn StartDecodingWithResult(&self, aFlags: uint32_t) -> bool {
        ((*self.vtable).StartDecodingWithResult)(self, aFlags)
    }


    /// ```text
    /// /**
    ///    * This method triggers decoding for an image, but unlike startDecoding() it
    ///    * enables the caller to provide more detailed information about the decode
    ///    * request.
    ///    *
    ///    * @param aFlags Flags of the FLAG_* variety.
    ///    * @return DECODE_SURFACE_AVAILABLE if is a surface that satisfies the
    ///    *         request and it is fully decoded.
    ///    *         DECODE_REQUESTED if we requested a decode.
    ///    *         DECODE_REQUEST_FAILED if we failed to request a decode. This means
    ///    *         that either there is an error in the image or we cannot allocate a
    ///    *         surface that big.
    ///    */
    /// ```
    ///

    /// `[noscript,notxpcom] imgIContainer_DecodeResult requestDecodeWithResult (in uint32_t aFlags);`
    #[inline]
    pub unsafe fn RequestDecodeWithResult(&self, aFlags: uint32_t) -> u8 {
        ((*self.vtable).RequestDecodeWithResult)(self, aFlags)
    }


    /// ```text
    /// /**
    ///    * Locks an image. If the image does not exist yet, locks it once it becomes
    ///    * available. The lock persists for the lifetime of the imgIRequest (until
        ///    * unlockImage is called) even if the underlying image changes.
    ///    *
    ///    * If you don't call unlockImage() by the time this imgIRequest goes away, it
    ///    * will be called for you automatically.
    ///    *
    ///    * @see imgIContainer::lockImage for documentation of the underlying call.
    ///    */
    /// ```
    ///

    /// `void lockImage ();`
    #[inline]
    pub unsafe fn LockImage(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).LockImage)(self, )
    }


    /// ```text
    /// /**
    ///    * Unlocks an image.
    ///    *
    ///    * @see imgIContainer::unlockImage for documentation of the underlying call.
    ///    */
    /// ```
    ///

    /// `void unlockImage ();`
    #[inline]
    pub unsafe fn UnlockImage(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).UnlockImage)(self, )
    }


    /// ```text
    /// /**
    ///    * If this image is unlocked, discard the image's decoded data.  If the image
    ///    * is locked or is already discarded, do nothing.
    ///    */
    /// ```
    ///

    /// `void requestDiscard ();`
    #[inline]
    pub unsafe fn RequestDiscard(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).RequestDiscard)(self, )
    }


    /// ```text
    /// /**
    ///    * If this request is for an animated image, the method creates a new
    ///    * request which contains the current frame of the image.
    ///    * Otherwise returns the same request.
    ///    */
    /// ```
    ///

    /// `imgIRequest getStaticRequest ();`
    #[inline]
    pub unsafe fn GetStaticRequest(&self, _retval: *mut *const imgIRequest) -> ::nserror::nsresult {
        ((*self.vtable).GetStaticRequest)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Requests that the image animate (if it has an animation).
    ///    *
    ///    * @see Image::IncrementAnimationConsumers for documentation of the
    ///    * underlying call.
    ///    */
    /// ```
    ///

    /// `void incrementAnimationConsumers ();`
    #[inline]
    pub unsafe fn IncrementAnimationConsumers(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).IncrementAnimationConsumers)(self, )
    }


    /// ```text
    /// /**
    ///    * Tell the image it can forget about a request that the image animate.
    ///    *
    ///    * @see Image::DecrementAnimationConsumers for documentation of the
    ///    * underlying call.
    ///    */
    /// ```
    ///

    /// `void decrementAnimationConsumers ();`
    #[inline]
    pub unsafe fn DecrementAnimationConsumers(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).DecrementAnimationConsumers)(self, )
    }



    /// `void boostPriority (in uint32_t aCategory);`
    #[inline]
    pub unsafe fn BoostPriority(&self, aCategory: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).BoostPriority)(self, aCategory)
    }


}


