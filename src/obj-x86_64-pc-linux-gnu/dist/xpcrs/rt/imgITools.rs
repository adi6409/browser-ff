//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/image/imgITools.idl
//


/// `interface imgITools : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct imgITools {
    vtable: *const imgIToolsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for imgITools.
unsafe impl XpCom for imgITools {
    const IID: nsIID = nsID(0x4c2383a4, 0x931c, 0x484d,
        [0x8c, 0x4a, 0x97, 0x35, 0x90, 0xf6, 0x6e, 0x3f]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for imgITools {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from imgITools.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait imgIToolsCoerce {
    /// Cheaply cast a value of this type from a `imgITools`.
    fn coerce_from(v: &imgITools) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl imgIToolsCoerce for imgITools {
    #[inline]
    fn coerce_from(v: &imgITools) -> &Self {
        v
    }
}

impl imgITools {
    /// Cast this `imgITools` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: imgIToolsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for imgITools {
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
impl<T: nsISupportsCoerce> imgIToolsCoerce for T {
    #[inline]
    fn coerce_from(v: &imgITools) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every imgITools
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct imgIToolsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* imgIContainer decodeImageFromBuffer (in string aBuffer, in unsigned long aSize, in ACString aMimeType); */
    pub DecodeImageFromBuffer: unsafe extern "system" fn (this: *const imgITools, aBuffer: *const libc::c_char, aSize: u32, aMimeType: *const ::nsstring::nsACString, _retval: *mut*const imgIContainer) -> ::nserror::nsresult,

    /* [implicit_jscontext] imgIContainer decodeImageFromArrayBuffer (in jsval aArrayBuffer, in ACString aMimeType); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub DecodeImageFromArrayBuffer: *const ::libc::c_void,

    /* void decodeImageFromChannelAsync (in nsIURI aURI, in nsIChannel aChannel, in imgIContainerCallback aCallback, in imgINotificationObserver aObserver); */
    pub DecodeImageFromChannelAsync: unsafe extern "system" fn (this: *const imgITools, aURI: *const nsIURI, aChannel: *const nsIChannel, aCallback: *const imgIContainerCallback, aObserver: *const imgINotificationObserver) -> ::nserror::nsresult,

    /* void decodeImageAsync (in nsIInputStream aStream, in ACString aMimeType, in imgIContainerCallback aCallback, in nsIEventTarget aEventTarget); */
    pub DecodeImageAsync: unsafe extern "system" fn (this: *const imgITools, aStream: *const nsIInputStream, aMimeType: *const ::nsstring::nsACString, aCallback: *const imgIContainerCallback, aEventTarget: *const nsIEventTarget) -> ::nserror::nsresult,

    /* nsIInputStream encodeImage (in imgIContainer aContainer, in ACString aMimeType, [optional] in AString outputOptions); */
    pub EncodeImage: unsafe extern "system" fn (this: *const imgITools, aContainer: *const imgIContainer, aMimeType: *const ::nsstring::nsACString, outputOptions: *const ::nsstring::nsAString, _retval: *mut*const nsIInputStream) -> ::nserror::nsresult,

    /* nsIInputStream encodeScaledImage (in imgIContainer aContainer, in ACString aMimeType, in long aWidth, in long aHeight, [optional] in AString outputOptions); */
    pub EncodeScaledImage: unsafe extern "system" fn (this: *const imgITools, aContainer: *const imgIContainer, aMimeType: *const ::nsstring::nsACString, aWidth: i32, aHeight: i32, outputOptions: *const ::nsstring::nsAString, _retval: *mut*const nsIInputStream) -> ::nserror::nsresult,

    /* imgILoader getImgLoaderForDocument (in Document doc); */
    pub GetImgLoaderForDocument: unsafe extern "system" fn (this: *const imgITools, doc: *const libc::c_void, _retval: *mut*const imgILoader) -> ::nserror::nsresult,

    /* imgICache getImgCacheForDocument (in Document doc); */
    pub GetImgCacheForDocument: unsafe extern "system" fn (this: *const imgITools, doc: *const libc::c_void, _retval: *mut*const imgICache) -> ::nserror::nsresult,

    /* nsIInputStream encodeCroppedImage (in imgIContainer aContainer, in ACString aMimeType, in long aOffsetX, in long aOffsetY, in long aWidth, in long aHeight, [optional] in AString outputOptions); */
    pub EncodeCroppedImage: unsafe extern "system" fn (this: *const imgITools, aContainer: *const imgIContainer, aMimeType: *const ::nsstring::nsACString, aOffsetX: i32, aOffsetY: i32, aWidth: i32, aHeight: i32, outputOptions: *const ::nsstring::nsAString, _retval: *mut*const nsIInputStream) -> ::nserror::nsresult,

    /* imgINotificationObserver createScriptedObserver (in imgIScriptedNotificationObserver aObserver); */
    pub CreateScriptedObserver: unsafe extern "system" fn (this: *const imgITools, aObserver: *const imgIScriptedNotificationObserver, _retval: *mut*const imgINotificationObserver) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl imgITools {

    /// ```text
    /// /**
    ///      * decodeImageFromBuffer
    ///      * Caller provides an buffer, a buffer size and a mimetype. We read from
    ///      * the stream and decompress it (according to the specified mime type) and
    ///      * return the resulting imgIContainer.
    ///      *
    ///      * @param aBuffer
    ///      *        Data in memory.
    ///      * @param aSize
    ///      *        Buffer size.
    ///      * @param aMimeType
    ///      *        Type of image in the stream.
    ///      */
    /// ```
    ///

    /// `imgIContainer decodeImageFromBuffer (in string aBuffer, in unsigned long aSize, in ACString aMimeType);`
    #[inline]
    pub unsafe fn DecodeImageFromBuffer(&self, aBuffer: *const libc::c_char, aSize: u32, aMimeType: *const ::nsstring::nsACString, _retval: *mut*const imgIContainer) -> ::nserror::nsresult {
        ((*self.vtable).DecodeImageFromBuffer)(self, aBuffer, aSize, aMimeType, _retval)
    }


    /// ```text
    /// /**
    ///      * decodeImageFromArrayBuffer
    ///      * Caller provides an ArrayBuffer and a mimetype. We read from
    ///      * the stream and decompress it (according to the specified mime type) and
    ///      * return the resulting imgIContainer.
    ///      *
    ///      * @param aArrayBuffer
    ///      *        An ArrayBuffer.
    ///      * @param aMimeType
    ///      *        Type of image in the stream.
    ///      */
    /// ```
    ///

    /// `[implicit_jscontext] imgIContainer decodeImageFromArrayBuffer (in jsval aArrayBuffer, in ACString aMimeType);`
    const _DecodeImageFromArrayBuffer: () = ();

    /// ```text
    /// /**
    ///      * decodeImageFromChannelAsync
    ///      * See decodeImage. The main difference between this method and decodeImage
    ///      * is that here the operation is done async on a thread from the decode
    ///      * pool. When the operation is completed, the callback is executed with the
    ///      * result.
    ///      *
    ///      * @param aURI
    ///      *        The original URI of the image
    ///      * @param aChannel
    ///      *        Channel to the image to be decoded.
    ///      * @param aCallback
    ///      *        The callback is executed when the imgContainer is fully created.
    ///      * @param aObserver
    ///      *        Optional observer for the decoded image, the caller should make
    ///      *        sure the observer is kept alive as long as necessary, as ImageLib
    ///      *        does not keep a strong reference to the observer.
    ///      */
    /// ```
    ///

    /// `void decodeImageFromChannelAsync (in nsIURI aURI, in nsIChannel aChannel, in imgIContainerCallback aCallback, in imgINotificationObserver aObserver);`
    #[inline]
    pub unsafe fn DecodeImageFromChannelAsync(&self, aURI: *const nsIURI, aChannel: *const nsIChannel, aCallback: *const imgIContainerCallback, aObserver: *const imgINotificationObserver) -> ::nserror::nsresult {
        ((*self.vtable).DecodeImageFromChannelAsync)(self, aURI, aChannel, aCallback, aObserver)
    }


    /// ```text
    /// /**
    ///      * decodeImageAsync
    ///      * See decodeImage. The main difference between this method and decodeImage
    ///      * is that here the operation is done async on a thread from the decode
    ///      * pool. When the operation is completed, the callback is executed with the
    ///      * result.
    ///      *
    ///      * @param aStream
    ///      *        An input stream for an encoded image file.
    ///      * @param aMimeType
    ///      *        Type of image in the stream.
    ///      * @param aCallback
    ///      *        The callback is executed when the imgContainer is fully created.
    ///      * @param aEventTarget
    ///      *        This eventTarget is used to execute aCallback
    ///      */
    /// ```
    ///

    /// `void decodeImageAsync (in nsIInputStream aStream, in ACString aMimeType, in imgIContainerCallback aCallback, in nsIEventTarget aEventTarget);`
    #[inline]
    pub unsafe fn DecodeImageAsync(&self, aStream: *const nsIInputStream, aMimeType: *const ::nsstring::nsACString, aCallback: *const imgIContainerCallback, aEventTarget: *const nsIEventTarget) -> ::nserror::nsresult {
        ((*self.vtable).DecodeImageAsync)(self, aStream, aMimeType, aCallback, aEventTarget)
    }


    /// ```text
    /// /**
    ///      * encodeImage
    ///      * Caller provides an image container, and the mime type it should be
    ///      * encoded to. We return an input stream for the encoded image data.
    ///      *
    ///      * @param aContainer
    ///      *        An image container.
    ///      * @param aMimeType
    ///      *        Type of encoded image desired (eg "image/png").
    ///      * @param outputOptions
    ///      *        Encoder-specific output options.
    ///      */
    /// ```
    ///

    /// `nsIInputStream encodeImage (in imgIContainer aContainer, in ACString aMimeType, [optional] in AString outputOptions);`
    #[inline]
    pub unsafe fn EncodeImage(&self, aContainer: *const imgIContainer, aMimeType: *const ::nsstring::nsACString, outputOptions: *const ::nsstring::nsAString, _retval: *mut*const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).EncodeImage)(self, aContainer, aMimeType, outputOptions, _retval)
    }


    /// ```text
    /// /**
    ///      * encodeScaledImage
    ///      * Caller provides an image container, and the mime type it should be
    ///      * encoded to. We return an input stream for the encoded image data.
    ///      * The encoded image is scaled to the specified dimensions.
    ///      *
    ///      * @param aContainer
    ///      *        An image container.
    ///      * @param aMimeType
    ///      *        Type of encoded image desired (eg "image/png").
    ///      * @param aWidth, aHeight
    ///      *        The size (in pixels) desired for the resulting image. Specify 0 to
    ///      *        use the given image's width or height. Values must be >= 0.
    ///      * @param outputOptions
    ///      *        Encoder-specific output options.
    ///      */
    /// ```
    ///

    /// `nsIInputStream encodeScaledImage (in imgIContainer aContainer, in ACString aMimeType, in long aWidth, in long aHeight, [optional] in AString outputOptions);`
    #[inline]
    pub unsafe fn EncodeScaledImage(&self, aContainer: *const imgIContainer, aMimeType: *const ::nsstring::nsACString, aWidth: i32, aHeight: i32, outputOptions: *const ::nsstring::nsAString, _retval: *mut*const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).EncodeScaledImage)(self, aContainer, aMimeType, aWidth, aHeight, outputOptions, _retval)
    }


    /// ```text
    /// /**
    ///      * getImgLoaderForDocument
    ///      * Retrieve an image loader that reflects the privacy status of the given
    ///      * document.
    ///      *
    ///      * @param doc
    ///      *        A document. Must not be null.
    ///      */
    /// ```
    ///

    /// `imgILoader getImgLoaderForDocument (in Document doc);`
    #[inline]
    pub unsafe fn GetImgLoaderForDocument(&self, doc: *const libc::c_void, _retval: *mut*const imgILoader) -> ::nserror::nsresult {
        ((*self.vtable).GetImgLoaderForDocument)(self, doc, _retval)
    }


    /// ```text
    /// /**
    ///      * getImgLoaderForDocument
    ///      * Retrieve an image cache that reflects the privacy status of the given
    ///      * document.
    ///      *
    ///      * @param doc
    ///      *        A document. Null is allowed, but must _only_ be passed
    ///      *        when there is no way to obtain a relevant document for
    ///      *        the current context in which a cache is desired.
    ///      */
    /// ```
    ///

    /// `imgICache getImgCacheForDocument (in Document doc);`
    #[inline]
    pub unsafe fn GetImgCacheForDocument(&self, doc: *const libc::c_void, _retval: *mut*const imgICache) -> ::nserror::nsresult {
        ((*self.vtable).GetImgCacheForDocument)(self, doc, _retval)
    }


    /// ```text
    /// /**
    ///      * encodeCroppedImage
    ///      * Caller provides an image container, and the mime type it should be
    ///      * encoded to. We return an input stream for the encoded image data.
    ///      * The encoded image is cropped to the specified dimensions.
    ///      *
    ///      * The given offset and size must not exceed the image bounds.
    ///      *
    ///      * @param aContainer
    ///      *        An image container.
    ///      * @param aMimeType
    ///      *        Type of encoded image desired (eg "image/png").
    ///      * @param aOffsetX, aOffsetY
    ///      *        The crop offset (in pixels). Values must be >= 0.
    ///      * @param aWidth, aHeight
    ///      *        The size (in pixels) desired for the resulting image. Specify 0 to
    ///      *        use the given image's width or height. Values must be >= 0.
    ///      * @param outputOptions
    ///      *        Encoder-specific output options.
    ///      */
    /// ```
    ///

    /// `nsIInputStream encodeCroppedImage (in imgIContainer aContainer, in ACString aMimeType, in long aOffsetX, in long aOffsetY, in long aWidth, in long aHeight, [optional] in AString outputOptions);`
    #[inline]
    pub unsafe fn EncodeCroppedImage(&self, aContainer: *const imgIContainer, aMimeType: *const ::nsstring::nsACString, aOffsetX: i32, aOffsetY: i32, aWidth: i32, aHeight: i32, outputOptions: *const ::nsstring::nsAString, _retval: *mut*const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).EncodeCroppedImage)(self, aContainer, aMimeType, aOffsetX, aOffsetY, aWidth, aHeight, outputOptions, _retval)
    }


    /// ```text
    /// /**
    ///      * Create a wrapper around a scripted notification observer (ordinarily
        ///      * imgINotificationObserver cannot be implemented from scripts).
    ///      *
    ///      * @param aObserver The scripted observer to wrap
    ///      */
    /// ```
    ///

    /// `imgINotificationObserver createScriptedObserver (in imgIScriptedNotificationObserver aObserver);`
    #[inline]
    pub unsafe fn CreateScriptedObserver(&self, aObserver: *const imgIScriptedNotificationObserver, _retval: *mut*const imgINotificationObserver) -> ::nserror::nsresult {
        ((*self.vtable).CreateScriptedObserver)(self, aObserver, _retval)
    }


}


/// `interface imgIContainerCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct imgIContainerCallback {
    vtable: *const imgIContainerCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for imgIContainerCallback.
unsafe impl XpCom for imgIContainerCallback {
    const IID: nsIID = nsID(0xf195772c, 0xa4c0, 0x47ae,
        [0x80, 0xca, 0x21, 0x1e, 0x00, 0x1c, 0x67, 0xbe]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for imgIContainerCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from imgIContainerCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait imgIContainerCallbackCoerce {
    /// Cheaply cast a value of this type from a `imgIContainerCallback`.
    fn coerce_from(v: &imgIContainerCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl imgIContainerCallbackCoerce for imgIContainerCallback {
    #[inline]
    fn coerce_from(v: &imgIContainerCallback) -> &Self {
        v
    }
}

impl imgIContainerCallback {
    /// Cast this `imgIContainerCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: imgIContainerCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for imgIContainerCallback {
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
impl<T: nsISupportsCoerce> imgIContainerCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &imgIContainerCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every imgIContainerCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct imgIContainerCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onImageReady (in imgIContainer aImage, in nsresult aStatus); */
    pub OnImageReady: unsafe extern "system" fn (this: *const imgIContainerCallback, aImage: *const imgIContainer, aStatus: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl imgIContainerCallback {

    /// ```text
    /// /**
    ///  * This is a companion interface for nsIAsyncInputStream::asyncWait.
    ///  */
    /// ```
    ///

    /// `void onImageReady (in imgIContainer aImage, in nsresult aStatus);`
    #[inline]
    pub unsafe fn OnImageReady(&self, aImage: *const imgIContainer, aStatus: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).OnImageReady)(self, aImage, aStatus)
    }


}


