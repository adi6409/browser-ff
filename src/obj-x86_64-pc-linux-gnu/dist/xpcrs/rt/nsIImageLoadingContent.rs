//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsIImageLoadingContent.idl
//


/// `interface nsIImageLoadingContent : imgINotificationObserver`
///

/// ```text
/// /**
///  * This interface represents a content node that loads images.  The interface
///  * exists to allow getting information on the images that the content node
///  * loads and to allow registration of observers for the image loads.
///  *
///  * Implementors of this interface should handle all the mechanics of actually
///  * loading an image -- getting the URI, checking with content policies and
///  * the security manager to see whether loading the URI is allowed, performing
///  * the load, firing any DOM events as needed.
///  *
///  * An implementation of this interface may support the concepts of a
///  * "current" image and a "pending" image.  If it does, a request to change
///  * the currently loaded image will start a "pending" request which will
///  * become current only when the image is loaded.  It is the responsibility of
///  * observers to check which request they are getting notifications for.
///  *
///  * Please make sure to update the MozImageLoadingContent WebIDL
///  * mixin to mirror this interface when changing it.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIImageLoadingContent {
    vtable: *const nsIImageLoadingContentVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIImageLoadingContent.
unsafe impl XpCom for nsIImageLoadingContent {
    const IID: nsIID = nsID(0x0357123d, 0x9224, 0x4d12,
        [0xa4, 0x7e, 0x86, 0x8c, 0x32, 0x68, 0x97, 0x77]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIImageLoadingContent {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIImageLoadingContent.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIImageLoadingContentCoerce {
    /// Cheaply cast a value of this type from a `nsIImageLoadingContent`.
    fn coerce_from(v: &nsIImageLoadingContent) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIImageLoadingContentCoerce for nsIImageLoadingContent {
    #[inline]
    fn coerce_from(v: &nsIImageLoadingContent) -> &Self {
        v
    }
}

impl nsIImageLoadingContent {
    /// Cast this `nsIImageLoadingContent` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIImageLoadingContentCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIImageLoadingContent {
    type Target = imgINotificationObserver;
    #[inline]
    fn deref(&self) -> &imgINotificationObserver {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: imgINotificationObserverCoerce> nsIImageLoadingContentCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIImageLoadingContent) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIImageLoadingContent
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIImageLoadingContentVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: imgINotificationObserverVTable,

    /* [nostdcall,notxpcom] void setLoadingEnabled (in boolean aEnabled); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub SetLoadingEnabled: *const ::libc::c_void,

    /* [nostdcall,notxpcom] void addNativeObserver (in imgINotificationObserver aObserver); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub AddNativeObserver: *const ::libc::c_void,

    /* [nostdcall,notxpcom] void removeNativeObserver (in imgINotificationObserver aObserver); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub RemoveNativeObserver: *const ::libc::c_void,

    /* [noscript] imgIRequest getRequest (in long aRequestType); */
    pub GetRequest: unsafe extern "system" fn (this: *const nsIImageLoadingContent, aRequestType: i32, _retval: *mut*const imgIRequest) -> ::nserror::nsresult,

    /* [notxpcom] void frameCreated (in nsIFrame aFrame); */
    /// Unable to generate binding because `forward declaration nsIFrame is unsupported`
    pub FrameCreated: *const ::libc::c_void,

    /* [notxpcom] void frameDestroyed (in nsIFrame aFrame); */
    /// Unable to generate binding because `forward declaration nsIFrame is unsupported`
    pub FrameDestroyed: *const ::libc::c_void,

    /* [noscript] long getRequestType (in imgIRequest aRequest); */
    pub GetRequestType: unsafe extern "system" fn (this: *const nsIImageLoadingContent, aRequest: *const imgIRequest, _retval: *mut i32) -> ::nserror::nsresult,

    /* [infallible,noscript] readonly attribute nsIURI currentURI; */
    pub GetCurrentURI: unsafe extern "system" fn (this: *const nsIImageLoadingContent, aCurrentURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* [noscript] nsIStreamListener loadImageWithChannel (in nsIChannel aChannel); */
    pub LoadImageWithChannel: unsafe extern "system" fn (this: *const nsIImageLoadingContent, aChannel: *const nsIChannel, _retval: *mut*const nsIStreamListener) -> ::nserror::nsresult,

    /* [nostdcall,notxpcom] void forceImageState (in boolean aForce, in unsigned long long aState); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub ForceImageState: *const ::libc::c_void,

    /* [noscript,notxpcom] void onVisibilityChange (in Visibility aNewVisibility, in MaybeOnNonvisible aNonvisibleAction); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub OnVisibilityChange: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIImageLoadingContent {
    /// ```text
    /// /**
    ///    * Request types.  Image loading content nodes attempt to do atomic
    ///    * image changes when the image url is changed.  This means that
    ///    * when the url changes the new image load will start, but the old
    ///    * image will remain the "current" request until the new image is
    ///    * fully loaded.  At that point, the old "current" request will be
    ///    * discarded and the "pending" request will become "current".
    ///    */
    /// ```
    ///

    pub const UNKNOWN_REQUEST: i64 = -1;


    pub const CURRENT_REQUEST: i64 = 0;


    pub const PENDING_REQUEST: i64 = 1;

    /// ```text
    /// /**
    ///    * setLoadingEnabled is used to enable and disable loading in
    ///    * situations where loading images is unwanted.  Note that enabling
    ///    * loading will *not* automatically trigger an image load.
    ///    */
    /// ```
    ///

    /// `[nostdcall,notxpcom] void setLoadingEnabled (in boolean aEnabled);`
    const _SetLoadingEnabled: () = ();

    /// ```text
    /// /**
    ///    * Used to register an image decoder observer.  Typically, this will
    ///    * be a proxy for a frame that wants to paint the image.
    ///    * Notifications from ongoing image loads will be passed to all
    ///    * registered observers.  Notifications for all request types,
    ///    * current and pending, will be passed through.
    ///    *
    ///    * @param aObserver the observer to register
    ///    */
    /// ```
    ///

    /// `[nostdcall,notxpcom] void addNativeObserver (in imgINotificationObserver aObserver);`
    const _AddNativeObserver: () = ();

    /// ```text
    /// /**
    ///    * Used to unregister an image decoder observer.
    ///    *
    ///    * @param aObserver the observer to unregister
    ///    */
    /// ```
    ///

    /// `[nostdcall,notxpcom] void removeNativeObserver (in imgINotificationObserver aObserver);`
    const _RemoveNativeObserver: () = ();

    /// ```text
    /// /**
    ///    * Accessor to get the image requests
    ///    *
    ///    * @param aRequestType a value saying which request is wanted
    ///    *
    ///    * @return the imgIRequest object (may be null, even when no error
        ///    * is thrown)
    ///    *
    ///    * @throws NS_ERROR_UNEXPECTED if the request type requested is not
    ///    * known
    ///    */
    /// ```
    ///

    /// `[noscript] imgIRequest getRequest (in long aRequestType);`
    #[inline]
    pub unsafe fn GetRequest(&self, aRequestType: i32, _retval: *mut*const imgIRequest) -> ::nserror::nsresult {
        ((*self.vtable).GetRequest)(self, aRequestType, _retval)
    }


    /// ```text
    /// /**
    ///    * Used to notify the image loading content node that a frame has been
    ///    * created.
    ///    */
    /// ```
    ///

    /// `[notxpcom] void frameCreated (in nsIFrame aFrame);`
    const _FrameCreated: () = ();

    /// ```text
    /// /**
    ///    * Used to notify the image loading content node that a frame has been
    ///    * destroyed.
    ///    */
    /// ```
    ///

    /// `[notxpcom] void frameDestroyed (in nsIFrame aFrame);`
    const _FrameDestroyed: () = ();

    /// ```text
    /// /**
    ///    * Used to find out what type of request one is dealing with (eg
        ///    * which request got passed through to the imgINotificationObserver
        ///    * interface of an observer)
    ///    *
    ///    * @param aRequest the request whose type we want to know
    ///    *
    ///    * @return an enum value saying what type this request is
    ///    *
    ///    * @throws NS_ERROR_UNEXPECTED if aRequest is not known
    ///    */
    /// ```
    ///

    /// `[noscript] long getRequestType (in imgIRequest aRequest);`
    #[inline]
    pub unsafe fn GetRequestType(&self, aRequest: *const imgIRequest, _retval: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetRequestType)(self, aRequest, _retval)
    }


    /// ```text
    /// /**
    ///    * Gets the URI of the current request, if available.
    ///    * Otherwise, returns the last URI that this content tried to load, or
    ///    * null if there haven't been any such attempts.
    ///    */
    /// ```
    ///

    /// `[infallible,noscript] readonly attribute nsIURI currentURI;`
    #[inline]
    pub unsafe fn GetCurrentURI(&self, aCurrentURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetCurrentURI)(self, aCurrentURI)
    }


    /// ```text
    /// /**
    ///    * loadImageWithChannel allows data from an existing channel to be
    ///    * used as the image data for this content node.
    ///    *
    ///    * @param aChannel the channel that will deliver the data
    ///    *
    ///    * @return a stream listener to pump the image data into
    ///    *
    ///    * @see imgILoader::loadImageWithChannel
    ///    *
    ///    * @throws NS_ERROR_NULL_POINTER if aChannel is null
    ///    */
    /// ```
    ///

    /// `[noscript] nsIStreamListener loadImageWithChannel (in nsIChannel aChannel);`
    #[inline]
    pub unsafe fn LoadImageWithChannel(&self, aChannel: *const nsIChannel, _retval: *mut*const nsIStreamListener) -> ::nserror::nsresult {
        ((*self.vtable).LoadImageWithChannel)(self, aChannel, _retval)
    }


    /// ```text
    /// /**
    ///    * Enables/disables image state forcing. When |aForce| is true, we force
    ///    * nsImageLoadingContent::ImageState() to return |aState|. Call again with |aForce|
    ///    * as false to revert ImageState() to its original behaviour.
    ///    */
    /// ```
    ///

    /// `[nostdcall,notxpcom] void forceImageState (in boolean aForce, in unsigned long long aState);`
    const _ForceImageState: () = ();

    /// ```text
    /// /**
    ///    * Called by layout to announce when the frame associated with this content
    ///    * has changed its visibility state.
    ///    *
    ///    * @param aNewVisibility    The new visibility state.
    ///    * @param aNonvisibleAction A requested action if the frame has become
    ///    *                          nonvisible. If Nothing(), no action is
    ///    *                          requested. If DISCARD_IMAGES is specified, the
    ///    *                          frame is requested to ask any images it's
    ///    *                          associated with to discard their surfaces if
    ///    *                          possible.
    ///    */
    /// ```
    ///

    /// `[noscript,notxpcom] void onVisibilityChange (in Visibility aNewVisibility, in MaybeOnNonvisible aNonvisibleAction);`
    const _OnVisibilityChange: () = ();

}


