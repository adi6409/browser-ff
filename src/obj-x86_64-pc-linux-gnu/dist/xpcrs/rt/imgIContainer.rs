//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/image/imgIContainer.idl
//


/// `interface imgIContainer : nsISupports`
///

/// ```text
/// /**
///  * imgIContainer is the interface that represents an image. It allows
///  * access to frames as Thebes surfaces. It also allows drawing of images
///  * onto Thebes contexts.
///  *
///  * Internally, imgIContainer also manages animation of images.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct imgIContainer {
    vtable: *const imgIContainerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for imgIContainer.
unsafe impl XpCom for imgIContainer {
    const IID: nsIID = nsID(0xa8dbee24, 0xff86, 0x4755,
        [0xb4, 0x0e, 0x51, 0x17, 0x5c, 0xaf, 0x31, 0xaf]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for imgIContainer {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from imgIContainer.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait imgIContainerCoerce {
    /// Cheaply cast a value of this type from a `imgIContainer`.
    fn coerce_from(v: &imgIContainer) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl imgIContainerCoerce for imgIContainer {
    #[inline]
    fn coerce_from(v: &imgIContainer) -> &Self {
        v
    }
}

impl imgIContainer {
    /// Cast this `imgIContainer` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: imgIContainerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for imgIContainer {
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
impl<T: nsISupportsCoerce> imgIContainerCoerce for T {
    #[inline]
    fn coerce_from(v: &imgIContainer) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every imgIContainer
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct imgIContainerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute int32_t width; */
    pub GetWidth: unsafe extern "system" fn (this: *const imgIContainer, aWidth: *mut int32_t) -> ::nserror::nsresult,

    /* readonly attribute int32_t height; */
    pub GetHeight: unsafe extern "system" fn (this: *const imgIContainer, aHeight: *mut int32_t) -> ::nserror::nsresult,

    /* [noscript] readonly attribute nsSize intrinsicSize; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetIntrinsicSize: *const ::libc::c_void,

    /* [nostdcall,notxpcom] readonly attribute MaybeAspectRatio intrinsicRatio; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetIntrinsicRatio: *const ::libc::c_void,

    /* readonly attribute int32_t hotspotX; */
    pub GetHotspotX: unsafe extern "system" fn (this: *const imgIContainer, aHotspotX: *mut int32_t) -> ::nserror::nsresult,

    /* readonly attribute int32_t hotspotY; */
    pub GetHotspotY: unsafe extern "system" fn (this: *const imgIContainer, aHotspotY: *mut int32_t) -> ::nserror::nsresult,

    /* [nostdcall,notxpcom] nsIntSizeByVal optimalImageSizeForDest ([const] in gfxSize aDest, in uint32_t aWhichFrame, in SamplingFilter aSamplingFilter, in uint32_t aFlags); */
    /// Unable to generate binding because `native type gfxSize unsupported`
    pub OptimalImageSizeForDest: *const ::libc::c_void,

    /* [infallible] readonly attribute unsigned short type; */
    pub GetType: unsafe extern "system" fn (this: *const imgIContainer, aType: *mut u16) -> ::nserror::nsresult,

    /* readonly attribute boolean animated; */
    pub GetAnimated: unsafe extern "system" fn (this: *const imgIContainer, aAnimated: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute unsigned long producerId; */
    pub GetProducerId: unsafe extern "system" fn (this: *const imgIContainer, aProducerId: *mut u32) -> ::nserror::nsresult,

    /* [noscript,notxpcom] TempRefSourceSurface getFrame (in uint32_t aWhichFrame, in uint32_t aFlags); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetFrame: *const ::libc::c_void,

    /* [noscript,notxpcom] TempRefSourceSurface getFrameAtSize ([const] in nsIntSize aSize, in uint32_t aWhichFrame, in uint32_t aFlags); */
    /// Unable to generate binding because `native type nsIntSize unsupported`
    pub GetFrameAtSize: *const ::libc::c_void,

    /* [noscript,notxpcom] boolean willDrawOpaqueNow (); */
    pub WillDrawOpaqueNow: unsafe extern "system" fn (this: *const imgIContainer) -> bool,

    /* [noscript,notxpcom] boolean isImageContainerAvailable (in LayerManager aManager, in uint32_t aFlags); */
    /// Unable to generate binding because `native type mozilla::layers::LayerManager unsupported`
    pub IsImageContainerAvailable: *const ::libc::c_void,

    /* [noscript,notxpcom] TempRefImageContainer getImageContainer (in LayerManager aManager, in uint32_t aFlags); */
    /// Unable to generate binding because `native type mozilla::layers::LayerManager unsupported`
    pub GetImageContainer: *const ::libc::c_void,

    /* [noscript,notxpcom] ImgDrawResult getImageContainerAtSize (in LayerManager aManager, [const] in nsIntSize aSize, [const] in MaybeSVGImageContext aSVGContext, in uint32_t aFlags, out ImageContainer aOutContainer); */
    /// Unable to generate binding because `native type mozilla::layers::LayerManager unsupported`
    pub GetImageContainerAtSize: *const ::libc::c_void,

    /* [noscript,notxpcom] boolean isImageContainerAvailableAtSize (in LayerManager aManager, [const] in nsIntSize aSize, in uint32_t aFlags); */
    /// Unable to generate binding because `native type mozilla::layers::LayerManager unsupported`
    pub IsImageContainerAvailableAtSize: *const ::libc::c_void,

    /* [noscript,notxpcom] ImgDrawResult draw (in gfxContext aContext, [const] in nsIntSize aSize, [const] in ImageRegion aRegion, in uint32_t aWhichFrame, in SamplingFilter aSamplingFilter, [const] in MaybeSVGImageContext aSVGContext, in uint32_t aFlags, in float aOpacity); */
    /// Unable to generate binding because `native type gfxContext unsupported`
    pub Draw: *const ::libc::c_void,

    /* [noscript] void startDecoding (in uint32_t aFlags, in uint32_t aWhichFrame); */
    pub StartDecoding: unsafe extern "system" fn (this: *const imgIContainer, aFlags: uint32_t, aWhichFrame: uint32_t) -> ::nserror::nsresult,

    /* [noscript,notxpcom] boolean startDecodingWithResult (in uint32_t aFlags, in uint32_t aWhichFrame); */
    pub StartDecodingWithResult: unsafe extern "system" fn (this: *const imgIContainer, aFlags: uint32_t, aWhichFrame: uint32_t) -> bool,

    /* [noscript,notxpcom] imgIContainer_DecodeResult requestDecodeWithResult (in uint32_t aFlags, in uint32_t aWhichFrame); */
    pub RequestDecodeWithResult: unsafe extern "system" fn (this: *const imgIContainer, aFlags: uint32_t, aWhichFrame: uint32_t) -> u8,

    /* [noscript] void requestDecodeForSize ([const] in nsIntSize aSize, in uint32_t aFlags, in uint32_t aWhichFrame); */
    /// Unable to generate binding because `native type nsIntSize unsupported`
    pub RequestDecodeForSize: *const ::libc::c_void,

    /* void lockImage (); */
    pub LockImage: unsafe extern "system" fn (this: *const imgIContainer) -> ::nserror::nsresult,

    /* void unlockImage (); */
    pub UnlockImage: unsafe extern "system" fn (this: *const imgIContainer) -> ::nserror::nsresult,

    /* void requestDiscard (); */
    pub RequestDiscard: unsafe extern "system" fn (this: *const imgIContainer) -> ::nserror::nsresult,

    /* [notxpcom] void requestRefresh ([const] in TimeStamp aTime); */
    /// Unable to generate binding because `native type mozilla::TimeStamp unsupported`
    pub RequestRefresh: *const ::libc::c_void,

    /* attribute unsigned short animationMode; */
    pub GetAnimationMode: unsafe extern "system" fn (this: *const imgIContainer, aAnimationMode: *mut u16) -> ::nserror::nsresult,

    /* attribute unsigned short animationMode; */
    pub SetAnimationMode: unsafe extern "system" fn (this: *const imgIContainer, aAnimationMode: u16) -> ::nserror::nsresult,

    /* void resetAnimation (); */
    pub ResetAnimation: unsafe extern "system" fn (this: *const imgIContainer) -> ::nserror::nsresult,

    /* [notxpcom] float getFrameIndex (in uint32_t aWhichFrame); */
    pub GetFrameIndex: unsafe extern "system" fn (this: *const imgIContainer, aWhichFrame: uint32_t) -> libc::c_float,

    /* [notxpcom] Orientation getOrientation (); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetOrientation: *const ::libc::c_void,

    /* [notxpcom] bool handledOrientation (); */
    pub HandledOrientation: unsafe extern "system" fn (this: *const imgIContainer) -> bool,

    /* [notxpcom] int32_t getFirstFrameDelay (); */
    pub GetFirstFrameDelay: unsafe extern "system" fn (this: *const imgIContainer) -> int32_t,

    /* [notxpcom] void setAnimationStartTime ([const] in TimeStamp aTime); */
    /// Unable to generate binding because `native type mozilla::TimeStamp unsupported`
    pub SetAnimationStartTime: *const ::libc::c_void,

    /* [notxpcom] nsIntRectByVal getImageSpaceInvalidationRect ([const] in nsIntRect aRect); */
    /// Unable to generate binding because `native type nsIntRect unsupported`
    pub GetImageSpaceInvalidationRect: *const ::libc::c_void,

    /* [nostdcall,notxpcom] TempRefImgIContainer unwrap (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub Unwrap: *const ::libc::c_void,

    /* [noscript,notxpcom] void propagateUseCounters (in Document aReferencingDocument); */
    pub PropagateUseCounters: unsafe extern "system" fn (this: *const imgIContainer, aReferencingDocument: *const libc::c_void) -> libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl imgIContainer {
    /// ```text
    /// /**
    ///     * Enumerated values for the 'type' attribute (below).
    ///     */
    /// ```
    ///

    pub const TYPE_RASTER: i64 = 0;


    pub const TYPE_VECTOR: i64 = 1;


    pub const TYPE_REQUEST: i64 = 2;

    /// ```text
    /// /**
    ///    * Flags for imgIContainer operations.
    ///    *
    ///    * Meanings:
    ///    *
    ///    * FLAG_NONE: Lack of flags.
    ///    *
    ///    * FLAG_SYNC_DECODE: Forces synchronous/non-progressive decode of all
    ///    * available data before the call returns.
    ///    *
    ///    * FLAG_SYNC_DECODE_IF_FAST: Like FLAG_SYNC_DECODE, but requests a sync decode
    ///    * be performed only if ImageLib estimates it can be completed very quickly.
    ///    *
    ///    * FLAG_ASYNC_NOTIFY: Send notifications asynchronously, even if we decode
    ///    * synchronously because of FLAG_SYNC_DECODE or FLAG_SYNC_DECODE_IF_FAST.
    ///    *
    ///    * FLAG_DECODE_NO_PREMULTIPLY_ALPHA: Do not premultiply alpha if
    ///    * it's not already premultiplied in the image data.
    ///    *
    ///    * FLAG_DECODE_NO_COLORSPACE_CONVERSION: Do not do any colorspace conversion;
    ///    * ignore any embedded profiles, and don't convert to any particular
    ///    * destination space.
    ///    *
    ///    * FLAG_CLAMP: Extend the image to the fill area by clamping image sample
    ///    * coordinates instead of by tiling. This only affects 'draw'.
    ///    *
    ///    * FLAG_HIGH_QUALITY_SCALING: A hint as to whether this image should be
    ///    * scaled using the high quality scaler. Do not set this if not drawing to
    ///    * a window or not listening to invalidations. Passing this flag will do two
    ///    * things: 1) request a decode of the image at the size asked for by the
///    * caller if one isn't already started or complete, and 2) allows a decoded
///    * frame of any size (it could be neither the requested size, nor the
///    * intrinsic size) to be substituted.
///    *
///    * FLAG_WANT_DATA_SURFACE: Can be passed to GetFrame when the caller wants a
///    * DataSourceSurface instead of a hardware accelerated surface. This can be
///    * important for performance (by avoiding an upload to/readback from the GPU)
///    * when the caller knows they want a SourceSurface of type DATA.
///    *
///    * FLAG_BYPASS_SURFACE_CACHE: Forces drawing to happen rather than taking
///    * cached rendering from the surface cache. This is used when we are printing,
///    * for example, where we want the vector commands from VectorImages to end up
///    * in the PDF output rather than a cached rendering at screen resolution.
///    *
///    * FLAG_FORCE_PRESERVEASPECTRATIO_NONE: Force scaling this image
///    * non-uniformly if necessary. This flag is for vector image only. A raster
///    * image should ignore this flag. While drawing a vector image with this
///    * flag, do not force uniform scaling even if its root <svg> node has a
///    * preserveAspectRatio attribute that would otherwise require uniform
///    * scaling , such as xMinYMin/ xMidYMin. Always scale the graphic content of
///    * the given image non-uniformly if necessary such that the image's
///    * viewBox (if specified or implied by height/width attributes) exactly
///    * matches the viewport rectangle.
///    *
///    * FLAG_FORCE_UNIFORM_SCALING: Signal to ClippedImage::OptimalSizeForDest that
///    * its returned size can only scale the image's size *uniformly* (by the same
///    * factor in each dimension). We need this flag when painting border-image
///    * section with SVG image source-data, if the SVG image has no viewBox and no
///    * intrinsic size. In such a case, we synthesize a viewport for the SVG image
///    * (a "window into SVG space") based on the border image area, and we need to
///    * be sure we don't subsequently scale that viewport in a way that distorts
///    * its contents by stretching them more in one dimension than the other.
///    *
///    * FLAG_AVOID_REDECODE_FOR_SIZE: If there is already a raster surface
///    * available for this image, but it is not the same size as requested, skip
///    * starting a new decode for said size.
///    *
///    * FLAG_DECODE_TO_SRGB_COLORSPACE: Instead of converting the colorspace to
///    * the display's colorspace, use sRGB.
///    */
/// ```
///

pub const FLAG_NONE: i64 = 0;


pub const FLAG_SYNC_DECODE: i64 = 1;


pub const FLAG_SYNC_DECODE_IF_FAST: i64 = 2;


pub const FLAG_ASYNC_NOTIFY: i64 = 4;


pub const FLAG_DECODE_NO_PREMULTIPLY_ALPHA: i64 = 8;


pub const FLAG_DECODE_NO_COLORSPACE_CONVERSION: i64 = 16;


pub const FLAG_CLAMP: i64 = 32;


pub const FLAG_HIGH_QUALITY_SCALING: i64 = 64;


pub const FLAG_WANT_DATA_SURFACE: i64 = 128;


pub const FLAG_BYPASS_SURFACE_CACHE: i64 = 256;


pub const FLAG_FORCE_PRESERVEASPECTRATIO_NONE: i64 = 512;


pub const FLAG_FORCE_UNIFORM_SCALING: i64 = 1024;


pub const FLAG_AVOID_REDECODE_FOR_SIZE: i64 = 2048;


pub const FLAG_DECODE_TO_SRGB_COLORSPACE: i64 = 4096;

/// ```text
/// /**
///    * A constant specifying the default set of decode flags (i.e., the default
///    * values for FLAG_DECODE_*).
///    */
/// ```
///

pub const DECODE_FLAGS_DEFAULT: i64 = 0;

/// ```text
/// /**
///    * A constant specifying the decode flags recommended to be used when
///    * re-encoding an image, or with the clipboard.
///    */
/// ```
///

pub const DECODE_FLAGS_FOR_REENCODE: i64 = 4104;

/// ```text
/// /**
///     * Constants for specifying various "special" frames.
///     *
///     * FRAME_FIRST: The first frame
///     * FRAME_CURRENT: The current frame
///     *
///     * FRAME_MAX_VALUE should be set to the value of the maximum constant above,
///     * as it is used for ensuring that a valid value was passed in.
///     */
/// ```
///

pub const FRAME_FIRST: i64 = 0;


pub const FRAME_CURRENT: i64 = 1;


pub const FRAME_MAX_VALUE: i64 = 1;

/// ```text
/// /**
///    * Animation mode Constants
///    *   0 = normal
///    *   1 = don't animate
///    *   2 = loop once
///    */
/// ```
///

pub const kNormalAnimMode: i64 = 0;


pub const kDontAnimMode: i64 = 1;


pub const kLoopOnceAnimMode: i64 = 2;

/// ```text
/// /**
///    * The width of the container rectangle.  In the case of any error,
///    * zero is returned, and an exception will be thrown.
///    */
/// ```
///

/// `readonly attribute int32_t width;`
#[inline]
pub unsafe fn GetWidth(&self, aWidth: *mut int32_t) -> ::nserror::nsresult {
((*self.vtable).GetWidth)(self, aWidth)
}


/// ```text
/// /**
///    * The height of the container rectangle.  In the case of any error,
///    * zero is returned, and an exception will be thrown.
///    */
/// ```
///

/// `readonly attribute int32_t height;`
#[inline]
pub unsafe fn GetHeight(&self, aHeight: *mut int32_t) -> ::nserror::nsresult {
((*self.vtable).GetHeight)(self, aHeight)
}


/// ```text
/// /**
///    * The intrinsic size of this image in appunits. If the image has no intrinsic
///    * size in a dimension, -1 will be returned for that dimension. In the case of
///    * any error, an exception will be thrown.
///    */
/// ```
///

/// `[noscript] readonly attribute nsSize intrinsicSize;`
const _GetIntrinsicSize: () = ();

/// ```text
/// /**
///    * The (dimensionless) intrinsic ratio of this image. In the case of any
///    * error, Nothing() will be returned.
///    */
/// ```
///

/// `[nostdcall,notxpcom] readonly attribute MaybeAspectRatio intrinsicRatio;`
const _GetIntrinsicRatio: () = ();

/// ```text
/// /**
///    * The x coordinate of the image's hotspot, or 0 if there is no hotspot.
///    */
/// ```
///

/// `readonly attribute int32_t hotspotX;`
#[inline]
pub unsafe fn GetHotspotX(&self, aHotspotX: *mut int32_t) -> ::nserror::nsresult {
((*self.vtable).GetHotspotX)(self, aHotspotX)
}


/// ```text
/// /**
///    * The y coordinate of the image's hotspot, or 0 if there is no hotspot.
///    */
/// ```
///

/// `readonly attribute int32_t hotspotY;`
#[inline]
pub unsafe fn GetHotspotY(&self, aHotspotY: *mut int32_t) -> ::nserror::nsresult {
((*self.vtable).GetHotspotY)(self, aHotspotY)
}


/// ```text
/// /**
///    * Given a size at which this image will be displayed, and the drawing
///    * parameters affecting how it will be drawn, returns the image size which
///    * should be used to draw to produce the highest quality result. This is the
///    * appropriate size, for example, to use as an input to the pixel snapping
///    * algorithm.
///    *
///    * For best results the size returned by this method should not be cached. It
///    * can change over time due to changes in the internal state of the image.
///    *
///    * @param aDest The size of the destination rect into which this image will be
///    *              drawn, in device pixels.
///    * @param aWhichFrame Frame specifier of the FRAME_* variety.
///    * @param aSamplingFilter The filter to be used if we're scaling the image.
///    * @param aFlags Flags of the FLAG_* variety
///    */
/// ```
///

/// `[nostdcall,notxpcom] nsIntSizeByVal optimalImageSizeForDest ([const] in gfxSize aDest, in uint32_t aWhichFrame, in SamplingFilter aSamplingFilter, in uint32_t aFlags);`
const _OptimalImageSizeForDest: () = ();

/// ```text
/// /**
///    * The type of this image (one of the TYPE_* values above).
///    */
/// ```
///

/// `[infallible] readonly attribute unsigned short type;`
#[inline]
pub unsafe fn GetType(&self) -> u16 {
let mut result = <u16 as ::std::default::Default>::default();
let _rv = ((*self.vtable).GetType)(self, &mut result);
debug_assert!(_rv.succeeded());
result
}


/// ```text
/// /**
///    * Whether this image is animated. You can only be guaranteed that querying
///    * this will not throw if STATUS_DECODE_COMPLETE is set on the imgIRequest.
///    *
///    * @throws NS_ERROR_NOT_AVAILABLE if the animated state cannot be determined.
///    */
/// ```
///

/// `readonly attribute boolean animated;`
#[inline]
pub unsafe fn GetAnimated(&self, aAnimated: *mut bool) -> ::nserror::nsresult {
((*self.vtable).GetAnimated)(self, aAnimated)
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
///    * Get a surface for the given frame. This may be a platform-native,
///    * optimized surface, so you cannot inspect its pixel data. If you
///    * need that, use SourceSurface::GetDataSurface.
///    *
///    * @param aWhichFrame Frame specifier of the FRAME_* variety.
///    * @param aFlags Flags of the FLAG_* variety
///    */
/// ```
///

/// `[noscript,notxpcom] TempRefSourceSurface getFrame (in uint32_t aWhichFrame, in uint32_t aFlags);`
const _GetFrame: () = ();

/// ```text
/// /**
///    * Get a surface for the given frame at the specified size. Matching the
///    * requested size is best effort; it's not guaranteed that the surface you get
///    * will be a perfect match. (Some reasons you may get a surface of a different
///    * size include: if you requested upscaling, if downscale-during-decode is
///    * disabled, or if you didn't request the first frame.)
///    *
///    * @param aSize The desired size.
///    * @param aWhichFrame Frame specifier of the FRAME_* variety.
///    * @param aFlags Flags of the FLAG_* variety
///    */
/// ```
///

/// `[noscript,notxpcom] TempRefSourceSurface getFrameAtSize ([const] in nsIntSize aSize, in uint32_t aWhichFrame, in uint32_t aFlags);`
const _GetFrameAtSize: () = ();

/// ```text
/// /**
///    * Returns true if this image will draw opaquely right now if asked to draw
///    * with FLAG_HIGH_QUALITY_SCALING and otherwise default flags. If this image
///    * (when decoded) is opaque but no decoded frames are available then
///    * willDrawOpaqueNow will return false.
///    */
/// ```
///

/// `[noscript,notxpcom] boolean willDrawOpaqueNow ();`
#[inline]
pub unsafe fn WillDrawOpaqueNow(&self, ) -> bool {
((*self.vtable).WillDrawOpaqueNow)(self, )
}


/// ```text
/// /**
///    * @return true if getImageContainer() is expected to return a valid
///    *         ImageContainer when passed the given @Manager and @Flags
///    *         parameters.
///    */
/// ```
///

/// `[noscript,notxpcom] boolean isImageContainerAvailable (in LayerManager aManager, in uint32_t aFlags);`
const _IsImageContainerAvailable: () = ();

/// ```text
/// /**
///    * Attempts to create an ImageContainer (and Image) containing the current
///    * frame at its native size.
///    *
///    * Avoid calling this unless you're actually going to layerize this image.
///    *
///    * @param aManager The LayerManager which will be used to create the
///    *                 ImageContainer.
///    * @param aFlags Decoding / drawing flags (in other words, FLAG_* flags).
///    *               Currently only FLAG_SYNC_DECODE and FLAG_SYNC_DECODE_IF_FAST
///    *               are supported.
///    * @return An ImageContainer for the current frame, or nullptr if one could
///    *         not be created.
///    */
/// ```
///

/// `[noscript,notxpcom] TempRefImageContainer getImageContainer (in LayerManager aManager, in uint32_t aFlags);`
const _GetImageContainer: () = ();

/// ```text
/// /**
///    * Attempts to create an ImageContainer (and Image) containing the current
///    * frame at the given size. Match the requested size is best effort; it's
///    * not guaranteed that the surface you get will be a perfect match. (Some
///    * reasons you may get a surface of a different size include: if you
///    * requested upscaling, or if downscale-during-decode is disabled.)
///    *
///    * Avoid calling this unless you're actually going to layerize this image.
///    *
///    * @param aManager The LayerManager which will be used to create the
///    *                 ImageContainer.
///    * @param aSVGContext If specified, SVG-related rendering context, such as
///    *                    overridden attributes on the image document's root <svg>
///    *                    node, and the size of the viewport that the full image
///    *                    would occupy. Ignored for raster images.
///    * @param aFlags Decoding / drawing flags (in other words, FLAG_* flags).
///    *               Currently only FLAG_SYNC_DECODE and FLAG_SYNC_DECODE_IF_FAST
///    *               are supported.
///    * @param aContainer Return value for ImageContainer for the current frame.
///    *                   May be null depending on the draw result.
///    * @return The draw result for the current frame.
///    */
/// ```
///

/// `[noscript,notxpcom] ImgDrawResult getImageContainerAtSize (in LayerManager aManager, [const] in nsIntSize aSize, [const] in MaybeSVGImageContext aSVGContext, in uint32_t aFlags, out ImageContainer aOutContainer);`
const _GetImageContainerAtSize: () = ();

/// ```text
/// /**
///    * @return true if getImageContainer() is expected to return a valid
///    *         ImageContainer when passed the given @Manager, @Size and @Flags
///    *         parameters.
///    */
/// ```
///

/// `[noscript,notxpcom] boolean isImageContainerAvailableAtSize (in LayerManager aManager, [const] in nsIntSize aSize, in uint32_t aFlags);`
const _IsImageContainerAvailableAtSize: () = ();

/// ```text
/// /**
///    * Draw the requested frame of this image onto the context specified.
///    *
///    * Drawing an image involves scaling it to a certain size (which may be
///    * implemented as a "smart" scale by substituting an HQ-scaled frame or
///    * rendering at a high DPI), and then selecting a region of that image to
///    * draw. That region is drawn onto the graphics context and in the process
///    * transformed by the context matrix, which determines the final area that is
///    * filled. The basic process looks like this:
///    *
///    *                           +------------------+
///    *                           |      Image       |
///    *                           |                  |
///    *                           | intrinsic width  |
///    *                           |        X         |
///    *                           | intrinsic height |
///    *                           +------------------+
///    *                          /                    \
///    *                         /                      \
///    *                        /    (scale to aSize)    \
///    *                       /                          \
///    *                      +----------------------------+
///    *                      |                            |
///    *                      |        Scaled Image        |
///    *                      | aSize.width X aSize.height |
///    *                      |                            |
///    *                      |       +---------+          |
///    *                      |       | aRegion |          |
///    *                      |       +---------+          |
///    *                      +-------(---------(----------+
    ///    *                              |         |
    ///    *                             /           \
    ///    *                            |  (transform |
        ///    *                           /  by aContext  \
        ///    *                          |     matrix)     |
    ///    *                         /                   \
    ///    *                        +---------------------+
    ///    *                        |                     |
    ///    *                        |      Fill Rect      |
    ///    *                        |                     |
    ///    *                        +---------------------+
    ///    *
    ///    * The region may extend outside of the scaled image's boundaries. It's
    ///    * actually a region in tiled image space, which is formed by tiling the
    ///    * scaled image infinitely in every direction. Drawing with a region larger
    ///    * than the scaled image thus causes the filled area to contain multiple tiled
    ///    * copies of the image, which looks like this:
    ///    *
    ///    *           ....................................................
    ///    *           :                :                :                :
    ///    *           :      Tile      :      Tile      :      Tile      :
    ///    *           :        +------------[aRegion]------------+       :
    ///    *           :........|.......:................:........|.......:
    ///    *           :        |       :                :        |       :
    ///    *           :      Ti|le     :  Scaled Image  :      Ti|le     :
    ///    *           :        |       :                :        |       :
    ///    *           :........|.......:................:........|.......:
    ///    *           :        +---------------------------------+       :
    ///    *           :      Ti|le     :      Tile      :      Ti|le     :
    ///    *           :       /        :                :         \      :
    ///    *           :......(.........:................:..........).....:
    ///    *                  |                                     |
    ///    *                 /                                       \
    ///    *                |      (transform by aContext matrix)     |
    ///    *               /                                           \
    ///    *              +---------------------------------------------+
    ///    *              |     :                                 :     |
    ///    *              |.....:.................................:.....|
    ///    *              |     :                                 :     |
    ///    *              |     :           Tiled Fill            :     |
    ///    *              |     :                                 :     |
    ///    *              |.....:.................................:.....|
    ///    *              |     :                                 :     |
    ///    *              +---------------------------------------------+
    ///    *
    ///    *
    ///    * @param aContext The Thebes context to draw the image to.
    ///    * @param aSize The size to which the image should be scaled before drawing.
    ///    *              This requirement may be satisfied using HQ scaled frames,
    ///    *              selecting from different resolution layers, drawing at a
    ///    *              higher DPI, or just performing additional scaling on the
    ///    *              graphics context. Callers can use optimalImageSizeForDest()
    ///    *              to determine the best choice for this parameter if they have
    ///    *              no special size requirements.
    ///    * @param aRegion The region in tiled image space which will be drawn onto the
    ///    *                graphics context. aRegion is in the coordinate space of the
    ///    *                image after it has been scaled to aSize - that is, the image
    ///    *                is scaled first, and then aRegion is applied. When aFlags
    ///    *                includes FLAG_CLAMP, the image will be extended to this area
    ///    *                by clamping image sample coordinates. Otherwise, the image
    ///    *                will be automatically tiled as necessary. aRegion can also
    ///    *                optionally contain a second region which restricts the set
    ///    *                of pixels we're allowed to sample from when drawing; this
    ///    *                is only of use to callers which need to draw with pixel
    ///    *                snapping.
    ///    * @param aWhichFrame Frame specifier of the FRAME_* variety.
    ///    * @param aSamplingFilter The filter to be used if we're scaling the image.
    ///    * @param aSVGContext If specified, SVG-related rendering context, such as
    ///    *                    overridden attributes on the image document's root <svg>
    ///    *                    node, and the size of the viewport that the full image
    ///    *                    would occupy. Ignored for raster images.
    ///    * @param aFlags Flags of the FLAG_* variety
    ///    * @return A ImgDrawResult value indicating whether and to what degree the
    ///    *         drawing operation was successful.
    ///    */
    /// ```
    ///

    /// `[noscript,notxpcom] ImgDrawResult draw (in gfxContext aContext, [const] in nsIntSize aSize, [const] in ImageRegion aRegion, in uint32_t aWhichFrame, in SamplingFilter aSamplingFilter, [const] in MaybeSVGImageContext aSVGContext, in uint32_t aFlags, in float aOpacity);`
    const _Draw: () = ();


    /// `[noscript] void startDecoding (in uint32_t aFlags, in uint32_t aWhichFrame);`
    #[inline]
    pub unsafe fn StartDecoding(&self, aFlags: uint32_t, aWhichFrame: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).StartDecoding)(self, aFlags, aWhichFrame)
    }



    /// `[noscript,notxpcom] boolean startDecodingWithResult (in uint32_t aFlags, in uint32_t aWhichFrame);`
    #[inline]
    pub unsafe fn StartDecodingWithResult(&self, aFlags: uint32_t, aWhichFrame: uint32_t) -> bool {
        ((*self.vtable).StartDecodingWithResult)(self, aFlags, aWhichFrame)
    }



    /// `[noscript,notxpcom] imgIContainer_DecodeResult requestDecodeWithResult (in uint32_t aFlags, in uint32_t aWhichFrame);`
    #[inline]
    pub unsafe fn RequestDecodeWithResult(&self, aFlags: uint32_t, aWhichFrame: uint32_t) -> u8 {
        ((*self.vtable).RequestDecodeWithResult)(self, aFlags, aWhichFrame)
    }



    /// `[noscript] void requestDecodeForSize ([const] in nsIntSize aSize, in uint32_t aFlags, in uint32_t aWhichFrame);`
    const _RequestDecodeForSize: () = ();

    /// ```text
    /// /**
    ///     * Increments the lock count on the image. An image will not be discarded
    ///     * as long as the lock count is nonzero. Note that it is still possible for
    ///     * the image to be undecoded if decode-on-draw is enabled and the image
    ///     * was never drawn.
    ///     *
    ///     * Upon instantiation images have a lock count of zero.
    ///     */
    /// ```
    ///

    /// `void lockImage ();`
    #[inline]
    pub unsafe fn LockImage(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).LockImage)(self, )
    }


    /// ```text
    /// /**
    ///     * Decreases the lock count on the image. If the lock count drops to zero,
    ///     * the image is allowed to discard its frame data to save memory.
    ///     *
    ///     * Upon instantiation images have a lock count of zero. It is an error to
    ///     * call this method without first having made a matching lockImage() call.
    ///     * In other words, the lock count is not allowed to be negative.
    ///     */
    /// ```
    ///

    /// `void unlockImage ();`
    #[inline]
    pub unsafe fn UnlockImage(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).UnlockImage)(self, )
    }


    /// ```text
    /// /**
    ///    * If this image is unlocked, discard its decoded data.  If the image is
    ///    * locked or has already been discarded, do nothing.
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
    ///     * Indicates that this imgIContainer has been triggered to update
    ///     * its internal animation state. Likely this should only be called
    ///     * from within nsImageFrame or objects of similar type.
    ///     */
    /// ```
    ///

    /// `[notxpcom] void requestRefresh ([const] in TimeStamp aTime);`
    const _RequestRefresh: () = ();


    /// `attribute unsigned short animationMode;`
    #[inline]
    pub unsafe fn GetAnimationMode(&self, aAnimationMode: *mut u16) -> ::nserror::nsresult {
        ((*self.vtable).GetAnimationMode)(self, aAnimationMode)
    }



    /// `attribute unsigned short animationMode;`
    #[inline]
    pub unsafe fn SetAnimationMode(&self, aAnimationMode: u16) -> ::nserror::nsresult {
        ((*self.vtable).SetAnimationMode)(self, aAnimationMode)
    }



    /// `void resetAnimation ();`
    #[inline]
    pub unsafe fn ResetAnimation(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ResetAnimation)(self, )
    }



    /// `[notxpcom] float getFrameIndex (in uint32_t aWhichFrame);`
    #[inline]
    pub unsafe fn GetFrameIndex(&self, aWhichFrame: uint32_t) -> libc::c_float {
        ((*self.vtable).GetFrameIndex)(self, aWhichFrame)
    }



    /// `[notxpcom] Orientation getOrientation ();`
    const _GetOrientation: () = ();


    /// `[notxpcom] bool handledOrientation ();`
    #[inline]
    pub unsafe fn HandledOrientation(&self, ) -> bool {
        ((*self.vtable).HandledOrientation)(self, )
    }



    /// `[notxpcom] int32_t getFirstFrameDelay ();`
    #[inline]
    pub unsafe fn GetFirstFrameDelay(&self, ) -> int32_t {
        ((*self.vtable).GetFirstFrameDelay)(self, )
    }



    /// `[notxpcom] void setAnimationStartTime ([const] in TimeStamp aTime);`
    const _SetAnimationStartTime: () = ();


    /// `[notxpcom] nsIntRectByVal getImageSpaceInvalidationRect ([const] in nsIntRect aRect);`
    const _GetImageSpaceInvalidationRect: () = ();


    /// `[nostdcall,notxpcom] TempRefImgIContainer unwrap ();`
    const _Unwrap: () = ();


    /// `[noscript,notxpcom] void propagateUseCounters (in Document aReferencingDocument);`
    #[inline]
    pub unsafe fn PropagateUseCounters(&self, aReferencingDocument: *const libc::c_void) -> libc::c_void {
        ((*self.vtable).PropagateUseCounters)(self, aReferencingDocument)
    }


}


