/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/image/imgIContainer.idl
 */

#ifndef __gen_imgIContainer_h__
#define __gen_imgIContainer_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "mozilla/AlreadyAddRefed.h"
#include "mozilla/Assertions.h"
#include "mozilla/DebugOnly.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
namespace dom {
class Document; /* webidl Document */
} // namespace dom
} // namespace mozilla

#include "ImgDrawResult.h"
#include "gfxPoint.h"
#include "mozilla/gfx/Types.h"
#include "mozilla/AspectRatio.h"
#include "mozilla/Maybe.h"
#include "mozilla/RefPtr.h"
#include "nsRect.h"
#include "nsSize.h"
#include "nsTArray.h"
#include "limits.h"
class gfxContext;
namespace mozilla {
struct AspectRatio;
namespace gfx {
class SourceSurface;
}
namespace layers {
class LayerManager;
class ImageContainer;
}
}
class nsIFrame;
namespace mozilla {
class TimeStamp;
class SVGImageContext;
struct MediaFeatureChange;
}
namespace mozilla {
namespace image {
class ImageRegion;
struct Orientation;
}
}

/* starting interface:    imgIContainer */
#define IMGICONTAINER_IID_STR "a8dbee24-ff86-4755-b40e-51175caf31af"

#define IMGICONTAINER_IID \
  {0xa8dbee24, 0xff86, 0x4755, \
    { 0xb4, 0x0e, 0x51, 0x17, 0x5c, 0xaf, 0x31, 0xaf }}

class imgIContainer : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(IMGICONTAINER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = imgIContainer;

  /* readonly attribute int32_t width; */
  NS_IMETHOD GetWidth(int32_t *aWidth) = 0;

  /* readonly attribute int32_t height; */
  NS_IMETHOD GetHeight(int32_t *aHeight) = 0;

  /* [noscript] readonly attribute nsSize intrinsicSize; */
  NS_IMETHOD GetIntrinsicSize(nsSize * aIntrinsicSize) = 0;

  /* [nostdcall,notxpcom] readonly attribute MaybeAspectRatio intrinsicRatio; */
  virtual mozilla::Maybe<mozilla::AspectRatio> GetIntrinsicRatio() = 0;

  /* readonly attribute int32_t hotspotX; */
  NS_IMETHOD GetHotspotX(int32_t *aHotspotX) = 0;

  /* readonly attribute int32_t hotspotY; */
  NS_IMETHOD GetHotspotY(int32_t *aHotspotY) = 0;

  /* [nostdcall,notxpcom] nsIntSizeByVal optimalImageSizeForDest ([const] in gfxSize aDest, in uint32_t aWhichFrame, in SamplingFilter aSamplingFilter, in uint32_t aFlags); */
  virtual nsIntSize OptimalImageSizeForDest(const gfxSize & aDest, uint32_t aWhichFrame, mozilla::gfx::SamplingFilter aSamplingFilter, uint32_t aFlags) = 0;

  enum {
    TYPE_RASTER = 0U,
    TYPE_VECTOR = 1U,
    TYPE_REQUEST = 2U
  };

  /* [infallible] readonly attribute unsigned short type; */
  NS_IMETHOD GetType(uint16_t *aType) = 0;
  inline uint16_t  GetType()
  {
    uint16_t result;
    mozilla::DebugOnly<nsresult> rv = GetType(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* readonly attribute boolean animated; */
  NS_IMETHOD GetAnimated(bool *aAnimated) = 0;

  /* [infallible] readonly attribute unsigned long producerId; */
  NS_IMETHOD GetProducerId(uint32_t *aProducerId) = 0;
  inline uint32_t  GetProducerId()
  {
    uint32_t result;
    mozilla::DebugOnly<nsresult> rv = GetProducerId(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  enum {
    FLAG_NONE = 0U,
    FLAG_SYNC_DECODE = 1U,
    FLAG_SYNC_DECODE_IF_FAST = 2U,
    FLAG_ASYNC_NOTIFY = 4U,
    FLAG_DECODE_NO_PREMULTIPLY_ALPHA = 8U,
    FLAG_DECODE_NO_COLORSPACE_CONVERSION = 16U,
    FLAG_CLAMP = 32U,
    FLAG_HIGH_QUALITY_SCALING = 64U,
    FLAG_WANT_DATA_SURFACE = 128U,
    FLAG_BYPASS_SURFACE_CACHE = 256U,
    FLAG_FORCE_PRESERVEASPECTRATIO_NONE = 512U,
    FLAG_FORCE_UNIFORM_SCALING = 1024U,
    FLAG_AVOID_REDECODE_FOR_SIZE = 2048U,
    FLAG_DECODE_TO_SRGB_COLORSPACE = 4096U,
    DECODE_FLAGS_DEFAULT = 0U,
    DECODE_FLAGS_FOR_REENCODE = 4104U,
    FRAME_FIRST = 0U,
    FRAME_CURRENT = 1U,
    FRAME_MAX_VALUE = 1U
  };

  /* [noscript,notxpcom] TempRefSourceSurface getFrame (in uint32_t aWhichFrame, in uint32_t aFlags); */
  NS_IMETHOD_(already_AddRefed<mozilla::gfx::SourceSurface>) GetFrame(uint32_t aWhichFrame, uint32_t aFlags) = 0;

  /* [noscript,notxpcom] TempRefSourceSurface getFrameAtSize ([const] in nsIntSize aSize, in uint32_t aWhichFrame, in uint32_t aFlags); */
  NS_IMETHOD_(already_AddRefed<mozilla::gfx::SourceSurface>) GetFrameAtSize(const nsIntSize & aSize, uint32_t aWhichFrame, uint32_t aFlags) = 0;

  /* [noscript,notxpcom] boolean willDrawOpaqueNow (); */
  NS_IMETHOD_(bool) WillDrawOpaqueNow(void) = 0;

  /* [noscript,notxpcom] boolean isImageContainerAvailable (in LayerManager aManager, in uint32_t aFlags); */
  NS_IMETHOD_(bool) IsImageContainerAvailable(mozilla::layers::LayerManager * aManager, uint32_t aFlags) = 0;

  /* [noscript,notxpcom] TempRefImageContainer getImageContainer (in LayerManager aManager, in uint32_t aFlags); */
  NS_IMETHOD_(already_AddRefed<mozilla::layers::ImageContainer>) GetImageContainer(mozilla::layers::LayerManager * aManager, uint32_t aFlags) = 0;

  /* [noscript,notxpcom] ImgDrawResult getImageContainerAtSize (in LayerManager aManager, [const] in nsIntSize aSize, [const] in MaybeSVGImageContext aSVGContext, in uint32_t aFlags, out ImageContainer aOutContainer); */
  NS_IMETHOD_(mozilla::image::ImgDrawResult) GetImageContainerAtSize(mozilla::layers::LayerManager * aManager, const nsIntSize & aSize, const mozilla::Maybe<mozilla::SVGImageContext> & aSVGContext, uint32_t aFlags, mozilla::layers::ImageContainer * * aOutContainer) = 0;

  /* [noscript,notxpcom] boolean isImageContainerAvailableAtSize (in LayerManager aManager, [const] in nsIntSize aSize, in uint32_t aFlags); */
  NS_IMETHOD_(bool) IsImageContainerAvailableAtSize(mozilla::layers::LayerManager * aManager, const nsIntSize & aSize, uint32_t aFlags) = 0;

  /* [noscript,notxpcom] ImgDrawResult draw (in gfxContext aContext, [const] in nsIntSize aSize, [const] in ImageRegion aRegion, in uint32_t aWhichFrame, in SamplingFilter aSamplingFilter, [const] in MaybeSVGImageContext aSVGContext, in uint32_t aFlags, in float aOpacity); */
  NS_IMETHOD_(mozilla::image::ImgDrawResult) Draw(gfxContext * aContext, const nsIntSize & aSize, const mozilla::image::ImageRegion & aRegion, uint32_t aWhichFrame, mozilla::gfx::SamplingFilter aSamplingFilter, const mozilla::Maybe<mozilla::SVGImageContext> & aSVGContext, uint32_t aFlags, float aOpacity) = 0;

  /* [noscript] void startDecoding (in uint32_t aFlags, in uint32_t aWhichFrame); */
  NS_IMETHOD StartDecoding(uint32_t aFlags, uint32_t aWhichFrame) = 0;

   nsresult StartDecoding(uint32_t aFlags) {
    return StartDecoding(aFlags, FRAME_CURRENT);
  }
  /* [noscript,notxpcom] boolean startDecodingWithResult (in uint32_t aFlags, in uint32_t aWhichFrame); */
  NS_IMETHOD_(bool) StartDecodingWithResult(uint32_t aFlags, uint32_t aWhichFrame) = 0;

   bool StartDecodingWithResult(uint32_t aFlags) {
    return StartDecodingWithResult(aFlags, FRAME_CURRENT);
  }
  enum DecodeResult : uint8_t {
    DECODE_SURFACE_AVAILABLE = 0,
    DECODE_REQUESTED = 1,
    DECODE_REQUEST_FAILED = 2,
  };

  /* [noscript,notxpcom] imgIContainer_DecodeResult requestDecodeWithResult (in uint32_t aFlags, in uint32_t aWhichFrame); */
  NS_IMETHOD_(imgIContainer::DecodeResult) RequestDecodeWithResult(uint32_t aFlags, uint32_t aWhichFrame) = 0;

   DecodeResult RequestDecodeWithResult(uint32_t aFlags) {
    return RequestDecodeWithResult(aFlags, FRAME_CURRENT);
  }
  /* [noscript] void requestDecodeForSize ([const] in nsIntSize aSize, in uint32_t aFlags, in uint32_t aWhichFrame); */
  NS_IMETHOD RequestDecodeForSize(const nsIntSize & aSize, uint32_t aFlags, uint32_t aWhichFrame) = 0;

   nsresult RequestDecodeForSize(const nsIntSize& aSize, uint32_t aFlags) {
    return RequestDecodeForSize(aSize, aFlags, FRAME_CURRENT);
  }
  /* void lockImage (); */
  NS_IMETHOD LockImage(void) = 0;

  /* void unlockImage (); */
  NS_IMETHOD UnlockImage(void) = 0;

  /* void requestDiscard (); */
  NS_IMETHOD RequestDiscard(void) = 0;

  /* [notxpcom] void requestRefresh ([const] in TimeStamp aTime); */
  NS_IMETHOD_(void) RequestRefresh(const mozilla::TimeStamp & aTime) = 0;

  enum {
    kNormalAnimMode = 0,
    kDontAnimMode = 1,
    kLoopOnceAnimMode = 2
  };

  /* attribute unsigned short animationMode; */
  NS_IMETHOD GetAnimationMode(uint16_t *aAnimationMode) = 0;
  NS_IMETHOD SetAnimationMode(uint16_t aAnimationMode) = 0;

  /* void resetAnimation (); */
  NS_IMETHOD ResetAnimation(void) = 0;

  /* [notxpcom] float getFrameIndex (in uint32_t aWhichFrame); */
  NS_IMETHOD_(float) GetFrameIndex(uint32_t aWhichFrame) = 0;

  /* [notxpcom] Orientation getOrientation (); */
  NS_IMETHOD_(mozilla::image::Orientation) GetOrientation(void) = 0;

  /* [notxpcom] bool handledOrientation (); */
  NS_IMETHOD_(bool) HandledOrientation(void) = 0;

  /* [notxpcom] int32_t getFirstFrameDelay (); */
  NS_IMETHOD_(int32_t) GetFirstFrameDelay(void) = 0;

  /* [notxpcom] void setAnimationStartTime ([const] in TimeStamp aTime); */
  NS_IMETHOD_(void) SetAnimationStartTime(const mozilla::TimeStamp & aTime) = 0;

  /* [notxpcom] nsIntRectByVal getImageSpaceInvalidationRect ([const] in nsIntRect aRect); */
  NS_IMETHOD_(nsIntRect) GetImageSpaceInvalidationRect(const nsIntRect & aRect) = 0;

  /* [nostdcall,notxpcom] TempRefImgIContainer unwrap (); */
  virtual already_AddRefed<imgIContainer> Unwrap(void) = 0;

  /* [noscript,notxpcom] void propagateUseCounters (in Document aReferencingDocument); */
  NS_IMETHOD_(void) PropagateUseCounters(mozilla::dom::Document *aReferencingDocument) = 0;

   /*
   * Called when media feature values that apply to all documents (such as
   * those based on system metrics) have changed.  If this image is a type
   * that can respond to media queries (i.e., an SVG image), this function
   * is overridden to handle restyling and invalidating the image.
   */
  virtual void MediaFeatureValuesChangedAllDocuments(const mozilla::MediaFeatureChange& aChange) {}
  /*
   * Get the set of sizes the image can decode to natively.
   */
  virtual nsresult GetNativeSizes(nsTArray<nsIntSize>& aNativeSizes) const = 0;
  virtual size_t GetNativeSizesLength() const = 0;
  };

  NS_DEFINE_STATIC_IID_ACCESSOR(imgIContainer, IMGICONTAINER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_IMGICONTAINER \
  NS_IMETHOD GetWidth(int32_t *aWidth) override; \
  NS_IMETHOD GetHeight(int32_t *aHeight) override; \
  NS_IMETHOD GetIntrinsicSize(nsSize * aIntrinsicSize) override; \
  virtual mozilla::Maybe<mozilla::AspectRatio> GetIntrinsicRatio() override; \
  NS_IMETHOD GetHotspotX(int32_t *aHotspotX) override; \
  NS_IMETHOD GetHotspotY(int32_t *aHotspotY) override; \
  virtual nsIntSize OptimalImageSizeForDest(const gfxSize & aDest, uint32_t aWhichFrame, mozilla::gfx::SamplingFilter aSamplingFilter, uint32_t aFlags) override; \
  using imgIContainer::GetType; \
  NS_IMETHOD GetType(uint16_t *aType) override; \
  NS_IMETHOD GetAnimated(bool *aAnimated) override; \
  using imgIContainer::GetProducerId; \
  NS_IMETHOD GetProducerId(uint32_t *aProducerId) override; \
  NS_IMETHOD_(already_AddRefed<mozilla::gfx::SourceSurface>) GetFrame(uint32_t aWhichFrame, uint32_t aFlags) override; \
  NS_IMETHOD_(already_AddRefed<mozilla::gfx::SourceSurface>) GetFrameAtSize(const nsIntSize & aSize, uint32_t aWhichFrame, uint32_t aFlags) override; \
  NS_IMETHOD_(bool) WillDrawOpaqueNow(void) override; \
  NS_IMETHOD_(bool) IsImageContainerAvailable(mozilla::layers::LayerManager * aManager, uint32_t aFlags) override; \
  NS_IMETHOD_(already_AddRefed<mozilla::layers::ImageContainer>) GetImageContainer(mozilla::layers::LayerManager * aManager, uint32_t aFlags) override; \
  NS_IMETHOD_(mozilla::image::ImgDrawResult) GetImageContainerAtSize(mozilla::layers::LayerManager * aManager, const nsIntSize & aSize, const mozilla::Maybe<mozilla::SVGImageContext> & aSVGContext, uint32_t aFlags, mozilla::layers::ImageContainer * * aOutContainer) override; \
  NS_IMETHOD_(bool) IsImageContainerAvailableAtSize(mozilla::layers::LayerManager * aManager, const nsIntSize & aSize, uint32_t aFlags) override; \
  NS_IMETHOD_(mozilla::image::ImgDrawResult) Draw(gfxContext * aContext, const nsIntSize & aSize, const mozilla::image::ImageRegion & aRegion, uint32_t aWhichFrame, mozilla::gfx::SamplingFilter aSamplingFilter, const mozilla::Maybe<mozilla::SVGImageContext> & aSVGContext, uint32_t aFlags, float aOpacity) override; \
  NS_IMETHOD StartDecoding(uint32_t aFlags, uint32_t aWhichFrame) override; \
  NS_IMETHOD_(bool) StartDecodingWithResult(uint32_t aFlags, uint32_t aWhichFrame) override; \
  NS_IMETHOD_(imgIContainer::DecodeResult) RequestDecodeWithResult(uint32_t aFlags, uint32_t aWhichFrame) override; \
  NS_IMETHOD RequestDecodeForSize(const nsIntSize & aSize, uint32_t aFlags, uint32_t aWhichFrame) override; \
  NS_IMETHOD LockImage(void) override; \
  NS_IMETHOD UnlockImage(void) override; \
  NS_IMETHOD RequestDiscard(void) override; \
  NS_IMETHOD_(void) RequestRefresh(const mozilla::TimeStamp & aTime) override; \
  NS_IMETHOD GetAnimationMode(uint16_t *aAnimationMode) override; \
  NS_IMETHOD SetAnimationMode(uint16_t aAnimationMode) override; \
  NS_IMETHOD ResetAnimation(void) override; \
  NS_IMETHOD_(float) GetFrameIndex(uint32_t aWhichFrame) override; \
  NS_IMETHOD_(mozilla::image::Orientation) GetOrientation(void) override; \
  NS_IMETHOD_(bool) HandledOrientation(void) override; \
  NS_IMETHOD_(int32_t) GetFirstFrameDelay(void) override; \
  NS_IMETHOD_(void) SetAnimationStartTime(const mozilla::TimeStamp & aTime) override; \
  NS_IMETHOD_(nsIntRect) GetImageSpaceInvalidationRect(const nsIntRect & aRect) override; \
  virtual already_AddRefed<imgIContainer> Unwrap(void) override; \
  NS_IMETHOD_(void) PropagateUseCounters(mozilla::dom::Document *aReferencingDocument) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_IMGICONTAINER \
  nsresult GetWidth(int32_t *aWidth); \
  nsresult GetHeight(int32_t *aHeight); \
  nsresult GetIntrinsicSize(nsSize * aIntrinsicSize); \
  mozilla::Maybe<mozilla::AspectRatio> GetIntrinsicRatio(); \
  nsresult GetHotspotX(int32_t *aHotspotX); \
  nsresult GetHotspotY(int32_t *aHotspotY); \
  nsIntSize OptimalImageSizeForDest(const gfxSize & aDest, uint32_t aWhichFrame, mozilla::gfx::SamplingFilter aSamplingFilter, uint32_t aFlags); \
  using imgIContainer::GetType; \
  nsresult GetType(uint16_t *aType); \
  nsresult GetAnimated(bool *aAnimated); \
  using imgIContainer::GetProducerId; \
  nsresult GetProducerId(uint32_t *aProducerId); \
  nsresult_(already_AddRefed<mozilla::gfx::SourceSurface>) GetFrame(uint32_t aWhichFrame, uint32_t aFlags); \
  nsresult_(already_AddRefed<mozilla::gfx::SourceSurface>) GetFrameAtSize(const nsIntSize & aSize, uint32_t aWhichFrame, uint32_t aFlags); \
  nsresult_(bool) WillDrawOpaqueNow(void); \
  nsresult_(bool) IsImageContainerAvailable(mozilla::layers::LayerManager * aManager, uint32_t aFlags); \
  nsresult_(already_AddRefed<mozilla::layers::ImageContainer>) GetImageContainer(mozilla::layers::LayerManager * aManager, uint32_t aFlags); \
  nsresult_(mozilla::image::ImgDrawResult) GetImageContainerAtSize(mozilla::layers::LayerManager * aManager, const nsIntSize & aSize, const mozilla::Maybe<mozilla::SVGImageContext> & aSVGContext, uint32_t aFlags, mozilla::layers::ImageContainer * * aOutContainer); \
  nsresult_(bool) IsImageContainerAvailableAtSize(mozilla::layers::LayerManager * aManager, const nsIntSize & aSize, uint32_t aFlags); \
  nsresult_(mozilla::image::ImgDrawResult) Draw(gfxContext * aContext, const nsIntSize & aSize, const mozilla::image::ImageRegion & aRegion, uint32_t aWhichFrame, mozilla::gfx::SamplingFilter aSamplingFilter, const mozilla::Maybe<mozilla::SVGImageContext> & aSVGContext, uint32_t aFlags, float aOpacity); \
  nsresult StartDecoding(uint32_t aFlags, uint32_t aWhichFrame); \
  nsresult_(bool) StartDecodingWithResult(uint32_t aFlags, uint32_t aWhichFrame); \
  nsresult_(imgIContainer::DecodeResult) RequestDecodeWithResult(uint32_t aFlags, uint32_t aWhichFrame); \
  nsresult RequestDecodeForSize(const nsIntSize & aSize, uint32_t aFlags, uint32_t aWhichFrame); \
  nsresult LockImage(void); \
  nsresult UnlockImage(void); \
  nsresult RequestDiscard(void); \
  nsresult_(void) RequestRefresh(const mozilla::TimeStamp & aTime); \
  nsresult GetAnimationMode(uint16_t *aAnimationMode); \
  nsresult SetAnimationMode(uint16_t aAnimationMode); \
  nsresult ResetAnimation(void); \
  nsresult_(float) GetFrameIndex(uint32_t aWhichFrame); \
  nsresult_(mozilla::image::Orientation) GetOrientation(void); \
  nsresult_(bool) HandledOrientation(void); \
  nsresult_(int32_t) GetFirstFrameDelay(void); \
  nsresult_(void) SetAnimationStartTime(const mozilla::TimeStamp & aTime); \
  nsresult_(nsIntRect) GetImageSpaceInvalidationRect(const nsIntRect & aRect); \
  already_AddRefed<imgIContainer> Unwrap(void); \
  nsresult_(void) PropagateUseCounters(mozilla::dom::Document *aReferencingDocument); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_IMGICONTAINER(_to) \
  NS_IMETHOD GetWidth(int32_t *aWidth) override { return _to GetWidth(aWidth); } \
  NS_IMETHOD GetHeight(int32_t *aHeight) override { return _to GetHeight(aHeight); } \
  NS_IMETHOD GetIntrinsicSize(nsSize * aIntrinsicSize) override { return _to GetIntrinsicSize(aIntrinsicSize); } \
  virtual mozilla::Maybe<mozilla::AspectRatio> GetIntrinsicRatio() override { return _to GetIntrinsicRatio(); } \
  NS_IMETHOD GetHotspotX(int32_t *aHotspotX) override { return _to GetHotspotX(aHotspotX); } \
  NS_IMETHOD GetHotspotY(int32_t *aHotspotY) override { return _to GetHotspotY(aHotspotY); } \
  virtual nsIntSize OptimalImageSizeForDest(const gfxSize & aDest, uint32_t aWhichFrame, mozilla::gfx::SamplingFilter aSamplingFilter, uint32_t aFlags) override { return _to OptimalImageSizeForDest(aDest, aWhichFrame, aSamplingFilter, aFlags); } \
  using imgIContainer::GetType; \
  NS_IMETHOD GetType(uint16_t *aType) override { return _to GetType(aType); } \
  NS_IMETHOD GetAnimated(bool *aAnimated) override { return _to GetAnimated(aAnimated); } \
  using imgIContainer::GetProducerId; \
  NS_IMETHOD GetProducerId(uint32_t *aProducerId) override { return _to GetProducerId(aProducerId); } \
  NS_IMETHOD_(already_AddRefed<mozilla::gfx::SourceSurface>) GetFrame(uint32_t aWhichFrame, uint32_t aFlags) override { return _to GetFrame(aWhichFrame, aFlags); } \
  NS_IMETHOD_(already_AddRefed<mozilla::gfx::SourceSurface>) GetFrameAtSize(const nsIntSize & aSize, uint32_t aWhichFrame, uint32_t aFlags) override { return _to GetFrameAtSize(aSize, aWhichFrame, aFlags); } \
  NS_IMETHOD_(bool) WillDrawOpaqueNow(void) override { return _to WillDrawOpaqueNow(); } \
  NS_IMETHOD_(bool) IsImageContainerAvailable(mozilla::layers::LayerManager * aManager, uint32_t aFlags) override { return _to IsImageContainerAvailable(aManager, aFlags); } \
  NS_IMETHOD_(already_AddRefed<mozilla::layers::ImageContainer>) GetImageContainer(mozilla::layers::LayerManager * aManager, uint32_t aFlags) override { return _to GetImageContainer(aManager, aFlags); } \
  NS_IMETHOD_(mozilla::image::ImgDrawResult) GetImageContainerAtSize(mozilla::layers::LayerManager * aManager, const nsIntSize & aSize, const mozilla::Maybe<mozilla::SVGImageContext> & aSVGContext, uint32_t aFlags, mozilla::layers::ImageContainer * * aOutContainer) override { return _to GetImageContainerAtSize(aManager, aSize, aSVGContext, aFlags, aOutContainer); } \
  NS_IMETHOD_(bool) IsImageContainerAvailableAtSize(mozilla::layers::LayerManager * aManager, const nsIntSize & aSize, uint32_t aFlags) override { return _to IsImageContainerAvailableAtSize(aManager, aSize, aFlags); } \
  NS_IMETHOD_(mozilla::image::ImgDrawResult) Draw(gfxContext * aContext, const nsIntSize & aSize, const mozilla::image::ImageRegion & aRegion, uint32_t aWhichFrame, mozilla::gfx::SamplingFilter aSamplingFilter, const mozilla::Maybe<mozilla::SVGImageContext> & aSVGContext, uint32_t aFlags, float aOpacity) override { return _to Draw(aContext, aSize, aRegion, aWhichFrame, aSamplingFilter, aSVGContext, aFlags, aOpacity); } \
  NS_IMETHOD StartDecoding(uint32_t aFlags, uint32_t aWhichFrame) override { return _to StartDecoding(aFlags, aWhichFrame); } \
  NS_IMETHOD_(bool) StartDecodingWithResult(uint32_t aFlags, uint32_t aWhichFrame) override { return _to StartDecodingWithResult(aFlags, aWhichFrame); } \
  NS_IMETHOD_(imgIContainer::DecodeResult) RequestDecodeWithResult(uint32_t aFlags, uint32_t aWhichFrame) override { return _to RequestDecodeWithResult(aFlags, aWhichFrame); } \
  NS_IMETHOD RequestDecodeForSize(const nsIntSize & aSize, uint32_t aFlags, uint32_t aWhichFrame) override { return _to RequestDecodeForSize(aSize, aFlags, aWhichFrame); } \
  NS_IMETHOD LockImage(void) override { return _to LockImage(); } \
  NS_IMETHOD UnlockImage(void) override { return _to UnlockImage(); } \
  NS_IMETHOD RequestDiscard(void) override { return _to RequestDiscard(); } \
  NS_IMETHOD_(void) RequestRefresh(const mozilla::TimeStamp & aTime) override { return _to RequestRefresh(aTime); } \
  NS_IMETHOD GetAnimationMode(uint16_t *aAnimationMode) override { return _to GetAnimationMode(aAnimationMode); } \
  NS_IMETHOD SetAnimationMode(uint16_t aAnimationMode) override { return _to SetAnimationMode(aAnimationMode); } \
  NS_IMETHOD ResetAnimation(void) override { return _to ResetAnimation(); } \
  NS_IMETHOD_(float) GetFrameIndex(uint32_t aWhichFrame) override { return _to GetFrameIndex(aWhichFrame); } \
  NS_IMETHOD_(mozilla::image::Orientation) GetOrientation(void) override { return _to GetOrientation(); } \
  NS_IMETHOD_(bool) HandledOrientation(void) override { return _to HandledOrientation(); } \
  NS_IMETHOD_(int32_t) GetFirstFrameDelay(void) override { return _to GetFirstFrameDelay(); } \
  NS_IMETHOD_(void) SetAnimationStartTime(const mozilla::TimeStamp & aTime) override { return _to SetAnimationStartTime(aTime); } \
  NS_IMETHOD_(nsIntRect) GetImageSpaceInvalidationRect(const nsIntRect & aRect) override { return _to GetImageSpaceInvalidationRect(aRect); } \
  virtual already_AddRefed<imgIContainer> Unwrap(void) override { return _to Unwrap(); } \
  NS_IMETHOD_(void) PropagateUseCounters(mozilla::dom::Document *aReferencingDocument) override { return _to PropagateUseCounters(aReferencingDocument); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_IMGICONTAINER(_to) \
  NS_IMETHOD GetWidth(int32_t *aWidth) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWidth(aWidth); } \
  NS_IMETHOD GetHeight(int32_t *aHeight) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHeight(aHeight); } \
  NS_IMETHOD GetIntrinsicSize(nsSize * aIntrinsicSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIntrinsicSize(aIntrinsicSize); } \
  virtual mozilla::Maybe<mozilla::AspectRatio> GetIntrinsicRatio() override; \
  NS_IMETHOD GetHotspotX(int32_t *aHotspotX) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHotspotX(aHotspotX); } \
  NS_IMETHOD GetHotspotY(int32_t *aHotspotY) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHotspotY(aHotspotY); } \
  virtual nsIntSize OptimalImageSizeForDest(const gfxSize & aDest, uint32_t aWhichFrame, mozilla::gfx::SamplingFilter aSamplingFilter, uint32_t aFlags) override; \
  NS_IMETHOD GetType(uint16_t *aType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetType(aType); } \
  NS_IMETHOD GetAnimated(bool *aAnimated) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAnimated(aAnimated); } \
  NS_IMETHOD GetProducerId(uint32_t *aProducerId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProducerId(aProducerId); } \
  NS_IMETHOD_(already_AddRefed<mozilla::gfx::SourceSurface>) GetFrame(uint32_t aWhichFrame, uint32_t aFlags) override; \
  NS_IMETHOD_(already_AddRefed<mozilla::gfx::SourceSurface>) GetFrameAtSize(const nsIntSize & aSize, uint32_t aWhichFrame, uint32_t aFlags) override; \
  NS_IMETHOD_(bool) WillDrawOpaqueNow(void) override; \
  NS_IMETHOD_(bool) IsImageContainerAvailable(mozilla::layers::LayerManager * aManager, uint32_t aFlags) override; \
  NS_IMETHOD_(already_AddRefed<mozilla::layers::ImageContainer>) GetImageContainer(mozilla::layers::LayerManager * aManager, uint32_t aFlags) override; \
  NS_IMETHOD_(mozilla::image::ImgDrawResult) GetImageContainerAtSize(mozilla::layers::LayerManager * aManager, const nsIntSize & aSize, const mozilla::Maybe<mozilla::SVGImageContext> & aSVGContext, uint32_t aFlags, mozilla::layers::ImageContainer * * aOutContainer) override; \
  NS_IMETHOD_(bool) IsImageContainerAvailableAtSize(mozilla::layers::LayerManager * aManager, const nsIntSize & aSize, uint32_t aFlags) override; \
  NS_IMETHOD_(mozilla::image::ImgDrawResult) Draw(gfxContext * aContext, const nsIntSize & aSize, const mozilla::image::ImageRegion & aRegion, uint32_t aWhichFrame, mozilla::gfx::SamplingFilter aSamplingFilter, const mozilla::Maybe<mozilla::SVGImageContext> & aSVGContext, uint32_t aFlags, float aOpacity) override; \
  NS_IMETHOD StartDecoding(uint32_t aFlags, uint32_t aWhichFrame) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StartDecoding(aFlags, aWhichFrame); } \
  NS_IMETHOD_(bool) StartDecodingWithResult(uint32_t aFlags, uint32_t aWhichFrame) override; \
  NS_IMETHOD_(imgIContainer::DecodeResult) RequestDecodeWithResult(uint32_t aFlags, uint32_t aWhichFrame) override; \
  NS_IMETHOD RequestDecodeForSize(const nsIntSize & aSize, uint32_t aFlags, uint32_t aWhichFrame) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RequestDecodeForSize(aSize, aFlags, aWhichFrame); } \
  NS_IMETHOD LockImage(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LockImage(); } \
  NS_IMETHOD UnlockImage(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UnlockImage(); } \
  NS_IMETHOD RequestDiscard(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RequestDiscard(); } \
  NS_IMETHOD_(void) RequestRefresh(const mozilla::TimeStamp & aTime) override; \
  NS_IMETHOD GetAnimationMode(uint16_t *aAnimationMode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAnimationMode(aAnimationMode); } \
  NS_IMETHOD SetAnimationMode(uint16_t aAnimationMode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAnimationMode(aAnimationMode); } \
  NS_IMETHOD ResetAnimation(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ResetAnimation(); } \
  NS_IMETHOD_(float) GetFrameIndex(uint32_t aWhichFrame) override; \
  NS_IMETHOD_(mozilla::image::Orientation) GetOrientation(void) override; \
  NS_IMETHOD_(bool) HandledOrientation(void) override; \
  NS_IMETHOD_(int32_t) GetFirstFrameDelay(void) override; \
  NS_IMETHOD_(void) SetAnimationStartTime(const mozilla::TimeStamp & aTime) override; \
  NS_IMETHOD_(nsIntRect) GetImageSpaceInvalidationRect(const nsIntRect & aRect) override; \
  virtual already_AddRefed<imgIContainer> Unwrap(void) override; \
  NS_IMETHOD_(void) PropagateUseCounters(mozilla::dom::Document *aReferencingDocument) override; \


#endif /* __gen_imgIContainer_h__ */
