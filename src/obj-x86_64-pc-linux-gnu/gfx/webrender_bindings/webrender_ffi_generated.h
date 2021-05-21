/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/* Generated with cbindgen:0.16.0 */

/* DO NOT MODIFY THIS MANUALLY! This file was generated using cbindgen.
 * To generate this file:
 *   1. Get the latest cbindgen using `cargo install --force cbindgen`
 *      a. Alternatively, you can clone `https://github.com/eqrion/cbindgen` and use a tagged release
 *   2. Run `rustup run nightly cbindgen toolkit/library/rust/ --lockfile Cargo.lock --crate webrender_bindings -o gfx/webrender_bindings/webrender_ffi_generated.h`
 */

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

namespace mozilla {
namespace wr {

static const uintptr_t BudgetType_COUNT = 7;

/// Whether a border should be antialiased.
enum class AntialiasBorder {
  No = 0,
  Yes,
  /// Must be last for serialization purposes
  Sentinel,
};

inline std::ostream& operator<<(std::ostream& aStream, const AntialiasBorder& aInstance) {
  switch (aInstance) {
    case AntialiasBorder::No: aStream << "No"; break;
    case AntialiasBorder::Yes: aStream << "Yes"; break;
    case AntialiasBorder::Sentinel: aStream << "Sentinel"; break;
  }
  return aStream;
}

enum class BorderStyle : uint32_t {
  None = 0,
  Solid = 1,
  Double = 2,
  Dotted = 3,
  Dashed = 4,
  Hidden = 5,
  Groove = 6,
  Ridge = 7,
  Inset = 8,
  Outset = 9,
  /// Must be last for serialization purposes
  Sentinel,
};

inline std::ostream& operator<<(std::ostream& aStream, const BorderStyle& aInstance) {
  switch (aInstance) {
    case BorderStyle::None: aStream << "None"; break;
    case BorderStyle::Solid: aStream << "Solid"; break;
    case BorderStyle::Double: aStream << "Double"; break;
    case BorderStyle::Dotted: aStream << "Dotted"; break;
    case BorderStyle::Dashed: aStream << "Dashed"; break;
    case BorderStyle::Hidden: aStream << "Hidden"; break;
    case BorderStyle::Groove: aStream << "Groove"; break;
    case BorderStyle::Ridge: aStream << "Ridge"; break;
    case BorderStyle::Inset: aStream << "Inset"; break;
    case BorderStyle::Outset: aStream << "Outset"; break;
    case BorderStyle::Sentinel: aStream << "Sentinel"; break;
  }
  return aStream;
}

enum class BoxShadowClipMode : uint8_t {
  Outset = 0,
  Inset = 1,
  /// Must be last for serialization purposes
  Sentinel,
};

inline std::ostream& operator<<(std::ostream& aStream, const BoxShadowClipMode& aInstance) {
  switch (aInstance) {
    case BoxShadowClipMode::Outset: aStream << "Outset"; break;
    case BoxShadowClipMode::Inset: aStream << "Inset"; break;
    case BoxShadowClipMode::Sentinel: aStream << "Sentinel"; break;
  }
  return aStream;
}

/// A stage of the rendering pipeline.
enum class Checkpoint : uint32_t {
  ///
  SceneBuilt,
  ///
  FrameBuilt,
  ///
  FrameTexturesUpdated,
  ///
  FrameRendered,
  /// NotificationRequests get notified with this if they get dropped without having been
  /// notified. This provides the guarantee that if a request is created it will get notified.
  TransactionDropped,
  /// Must be last for serialization purposes
  Sentinel,
};

inline std::ostream& operator<<(std::ostream& aStream, const Checkpoint& aInstance) {
  switch (aInstance) {
    case Checkpoint::SceneBuilt: aStream << "SceneBuilt"; break;
    case Checkpoint::FrameBuilt: aStream << "FrameBuilt"; break;
    case Checkpoint::FrameTexturesUpdated: aStream << "FrameTexturesUpdated"; break;
    case Checkpoint::FrameRendered: aStream << "FrameRendered"; break;
    case Checkpoint::TransactionDropped: aStream << "TransactionDropped"; break;
    case Checkpoint::Sentinel: aStream << "Sentinel"; break;
  }
  return aStream;
}

enum class ClipMode {
  Clip,
  ClipOut,
  /// Must be last for serialization purposes
  Sentinel,
};

inline std::ostream& operator<<(std::ostream& aStream, const ClipMode& aInstance) {
  switch (aInstance) {
    case ClipMode::Clip: aStream << "Clip"; break;
    case ClipMode::ClipOut: aStream << "ClipOut"; break;
    case ClipMode::Sentinel: aStream << "Sentinel"; break;
  }
  return aStream;
}

/// Specifies the color depth of an image. Currently only used for YUV images.
enum class ColorDepth : uint8_t {
  /// 8 bits image (most common)
  Color8,
  /// 10 bits image
  Color10,
  /// 12 bits image
  Color12,
  /// 16 bits image
  Color16,
  /// Must be last for serialization purposes
  Sentinel,
};

inline std::ostream& operator<<(std::ostream& aStream, const ColorDepth& aInstance) {
  switch (aInstance) {
    case ColorDepth::Color8: aStream << "Color8"; break;
    case ColorDepth::Color10: aStream << "Color10"; break;
    case ColorDepth::Color12: aStream << "Color12"; break;
    case ColorDepth::Color16: aStream << "Color16"; break;
    case ColorDepth::Sentinel: aStream << "Sentinel"; break;
  }
  return aStream;
}

enum class ColorRange : uint8_t {
  Limited = 0,
  Full = 1,
  /// Must be last for serialization purposes
  Sentinel,
};

inline std::ostream& operator<<(std::ostream& aStream, const ColorRange& aInstance) {
  switch (aInstance) {
    case ColorRange::Limited: aStream << "Limited"; break;
    case ColorRange::Full: aStream << "Full"; break;
    case ColorRange::Sentinel: aStream << "Sentinel"; break;
  }
  return aStream;
}

enum class ComponentTransferFuncType : uint8_t {
  Identity = 0,
  Table = 1,
  Discrete = 2,
  Linear = 3,
  Gamma = 4,
  /// Must be last for serialization purposes
  Sentinel,
};

inline std::ostream& operator<<(std::ostream& aStream, const ComponentTransferFuncType& aInstance) {
  switch (aInstance) {
    case ComponentTransferFuncType::Identity: aStream << "Identity"; break;
    case ComponentTransferFuncType::Table: aStream << "Table"; break;
    case ComponentTransferFuncType::Discrete: aStream << "Discrete"; break;
    case ComponentTransferFuncType::Linear: aStream << "Linear"; break;
    case ComponentTransferFuncType::Gamma: aStream << "Gamma"; break;
    case ComponentTransferFuncType::Sentinel: aStream << "Sentinel"; break;
  }
  return aStream;
}

/// Crash annotations included in crash reports.
enum class CrashAnnotation {
  CompileShader = 0,
  /// Must be last for serialization purposes
  Sentinel,
};

inline std::ostream& operator<<(std::ostream& aStream, const CrashAnnotation& aInstance) {
  switch (aInstance) {
    case CrashAnnotation::CompileShader: aStream << "CompileShader"; break;
    case CrashAnnotation::Sentinel: aStream << "Sentinel"; break;
  }
  return aStream;
}

enum class ExtendMode : uint8_t {
  Clamp,
  Repeat,
  /// Must be last for serialization purposes
  Sentinel,
};

inline std::ostream& operator<<(std::ostream& aStream, const ExtendMode& aInstance) {
  switch (aInstance) {
    case ExtendMode::Clamp: aStream << "Clamp"; break;
    case ExtendMode::Repeat: aStream << "Repeat"; break;
    case ExtendMode::Sentinel: aStream << "Sentinel"; break;
  }
  return aStream;
}

#if !(defined(XP_MACOSX) || defined(XP_WIN))
enum class FontHinting : uint8_t {
  None,
  Mono,
  Light,
  Normal,
  LCD,
  /// Must be last for serialization purposes
  Sentinel,
};

inline std::ostream& operator<<(std::ostream& aStream, const FontHinting& aInstance) {
  switch (aInstance) {
    case FontHinting::None: aStream << "None"; break;
    case FontHinting::Mono: aStream << "Mono"; break;
    case FontHinting::Light: aStream << "Light"; break;
    case FontHinting::Normal: aStream << "Normal"; break;
    case FontHinting::LCD: aStream << "LCD"; break;
    case FontHinting::Sentinel: aStream << "Sentinel"; break;
  }
  return aStream;
}
#endif

#if !(defined(XP_MACOSX) || defined(XP_WIN))
enum class FontLCDFilter : uint8_t {
  None,
  Default,
  Light,
  Legacy,
  /// Must be last for serialization purposes
  Sentinel,
};

inline std::ostream& operator<<(std::ostream& aStream, const FontLCDFilter& aInstance) {
  switch (aInstance) {
    case FontLCDFilter::None: aStream << "None"; break;
    case FontLCDFilter::Default: aStream << "Default"; break;
    case FontLCDFilter::Light: aStream << "Light"; break;
    case FontLCDFilter::Legacy: aStream << "Legacy"; break;
    case FontLCDFilter::Sentinel: aStream << "Sentinel"; break;
  }
  return aStream;
}
#endif

enum class FontRenderMode : uint8_t {
  Mono = 0,
  Alpha,
  Subpixel,
  /// Must be last for serialization purposes
  Sentinel,
};

inline std::ostream& operator<<(std::ostream& aStream, const FontRenderMode& aInstance) {
  switch (aInstance) {
    case FontRenderMode::Mono: aStream << "Mono"; break;
    case FontRenderMode::Alpha: aStream << "Alpha"; break;
    case FontRenderMode::Subpixel: aStream << "Subpixel"; break;
    case FontRenderMode::Sentinel: aStream << "Sentinel"; break;
  }
  return aStream;
}

/// Specifies the type of texture target in driver terms.
enum class ImageBufferKind : uint8_t {
  /// Standard texture. This maps to GL_TEXTURE_2D in OpenGL.
  Texture2D = 0,
  /// Rectangle texture. This maps to GL_TEXTURE_RECTANGLE in OpenGL. This
  /// is similar to a standard texture, with a few subtle differences
  /// (no mipmaps, non-power-of-two dimensions, different coordinate space)
  /// that make it useful for representing the kinds of textures we use
  /// in WebRender. See https://www.khronos.org/opengl/wiki/Rectangle_Texture
  /// for background on Rectangle textures.
  TextureRect = 1,
  /// External texture. This maps to GL_TEXTURE_EXTERNAL_OES in OpenGL, which
  /// is an extension. This is used for image formats that OpenGL doesn't
  /// understand, particularly YUV. See
  /// https://www.khronos.org/registry/OpenGL/extensions/OES/OES_EGL_image_external.txt
  TextureExternal = 2,
  /// Array texture. This maps to GL_TEXTURE_2D_ARRAY in OpenGL. See
  /// https://www.khronos.org/opengl/wiki/Array_Texture for background
  /// on Array textures.
  Texture2DArray = 3,
  /// Must be last for serialization purposes
  Sentinel,
};

inline std::ostream& operator<<(std::ostream& aStream, const ImageBufferKind& aInstance) {
  switch (aInstance) {
    case ImageBufferKind::Texture2D: aStream << "Texture2D"; break;
    case ImageBufferKind::TextureRect: aStream << "TextureRect"; break;
    case ImageBufferKind::TextureExternal: aStream << "TextureExternal"; break;
    case ImageBufferKind::Texture2DArray: aStream << "Texture2DArray"; break;
    case ImageBufferKind::Sentinel: aStream << "Sentinel"; break;
  }
  return aStream;
}

/// Specifies the format of a series of pixels, in driver terms.
enum class ImageFormat : uint8_t {
  /// One-channel, byte storage. The "red" doesn't map to the color
  /// red per se, and is just the way that OpenGL has historically referred
  /// to single-channel buffers.
  R8 = 1,
  /// One-channel, short storage
  R16 = 2,
  /// Four channels, byte storage.
  BGRA8 = 3,
  /// Four channels, float storage.
  RGBAF32 = 4,
  /// Two-channels, byte storage. Similar to `R8`, this just means
  /// "two channels" rather than "red and green".
  RG8 = 5,
  /// Two-channels, byte storage. Similar to `R16`, this just means
  /// "two channels" rather than "red and green".
  RG16 = 6,
  /// Four channels, signed integer storage.
  RGBAI32 = 7,
  /// Four channels, byte storage.
  RGBA8 = 8,
  /// Must be last for serialization purposes
  Sentinel,
};

inline std::ostream& operator<<(std::ostream& aStream, const ImageFormat& aInstance) {
  switch (aInstance) {
    case ImageFormat::R8: aStream << "R8"; break;
    case ImageFormat::R16: aStream << "R16"; break;
    case ImageFormat::BGRA8: aStream << "BGRA8"; break;
    case ImageFormat::RGBAF32: aStream << "RGBAF32"; break;
    case ImageFormat::RG8: aStream << "RG8"; break;
    case ImageFormat::RG16: aStream << "RG16"; break;
    case ImageFormat::RGBAI32: aStream << "RGBAI32"; break;
    case ImageFormat::RGBA8: aStream << "RGBA8"; break;
    case ImageFormat::Sentinel: aStream << "Sentinel"; break;
  }
  return aStream;
}

enum class ImageRendering : uint8_t {
  Auto = 0,
  CrispEdges = 1,
  Pixelated = 2,
  /// Must be last for serialization purposes
  Sentinel,
};

inline std::ostream& operator<<(std::ostream& aStream, const ImageRendering& aInstance) {
  switch (aInstance) {
    case ImageRendering::Auto: aStream << "Auto"; break;
    case ImageRendering::CrispEdges: aStream << "CrispEdges"; break;
    case ImageRendering::Pixelated: aStream << "Pixelated"; break;
    case ImageRendering::Sentinel: aStream << "Sentinel"; break;
  }
  return aStream;
}

enum class LineOrientation : uint8_t {
  Vertical,
  Horizontal,
  /// Must be last for serialization purposes
  Sentinel,
};

inline std::ostream& operator<<(std::ostream& aStream, const LineOrientation& aInstance) {
  switch (aInstance) {
    case LineOrientation::Vertical: aStream << "Vertical"; break;
    case LineOrientation::Horizontal: aStream << "Horizontal"; break;
    case LineOrientation::Sentinel: aStream << "Sentinel"; break;
  }
  return aStream;
}

enum class LineStyle : uint8_t {
  Solid,
  Dotted,
  Dashed,
  Wavy,
  /// Must be last for serialization purposes
  Sentinel,
};

inline std::ostream& operator<<(std::ostream& aStream, const LineStyle& aInstance) {
  switch (aInstance) {
    case LineStyle::Solid: aStream << "Solid"; break;
    case LineStyle::Dotted: aStream << "Dotted"; break;
    case LineStyle::Dashed: aStream << "Dashed"; break;
    case LineStyle::Wavy: aStream << "Wavy"; break;
    case LineStyle::Sentinel: aStream << "Sentinel"; break;
  }
  return aStream;
}

enum class MixBlendMode : uint8_t {
  Normal = 0,
  Multiply = 1,
  Screen = 2,
  Overlay = 3,
  Darken = 4,
  Lighten = 5,
  ColorDodge = 6,
  ColorBurn = 7,
  HardLight = 8,
  SoftLight = 9,
  Difference = 10,
  Exclusion = 11,
  Hue = 12,
  Saturation = 13,
  Color = 14,
  Luminosity = 15,
  /// Must be last for serialization purposes
  Sentinel,
};

inline std::ostream& operator<<(std::ostream& aStream, const MixBlendMode& aInstance) {
  switch (aInstance) {
    case MixBlendMode::Normal: aStream << "Normal"; break;
    case MixBlendMode::Multiply: aStream << "Multiply"; break;
    case MixBlendMode::Screen: aStream << "Screen"; break;
    case MixBlendMode::Overlay: aStream << "Overlay"; break;
    case MixBlendMode::Darken: aStream << "Darken"; break;
    case MixBlendMode::Lighten: aStream << "Lighten"; break;
    case MixBlendMode::ColorDodge: aStream << "ColorDodge"; break;
    case MixBlendMode::ColorBurn: aStream << "ColorBurn"; break;
    case MixBlendMode::HardLight: aStream << "HardLight"; break;
    case MixBlendMode::SoftLight: aStream << "SoftLight"; break;
    case MixBlendMode::Difference: aStream << "Difference"; break;
    case MixBlendMode::Exclusion: aStream << "Exclusion"; break;
    case MixBlendMode::Hue: aStream << "Hue"; break;
    case MixBlendMode::Saturation: aStream << "Saturation"; break;
    case MixBlendMode::Color: aStream << "Color"; break;
    case MixBlendMode::Luminosity: aStream << "Luminosity"; break;
    case MixBlendMode::Sentinel: aStream << "Sentinel"; break;
  }
  return aStream;
}

/// Used to indicate if an image is opaque, or has an alpha channel.
enum class OpacityType : uint8_t {
  Opaque = 0,
  HasAlphaChannel = 1,
  /// Must be last for serialization purposes
  Sentinel,
};

inline std::ostream& operator<<(std::ostream& aStream, const OpacityType& aInstance) {
  switch (aInstance) {
    case OpacityType::Opaque: aStream << "Opaque"; break;
    case OpacityType::HasAlphaChannel: aStream << "HasAlphaChannel"; break;
    case OpacityType::Sentinel: aStream << "Sentinel"; break;
  }
  return aStream;
}

enum class RepeatMode : uint8_t {
  Stretch,
  Repeat,
  Round,
  Space,
  /// Must be last for serialization purposes
  Sentinel,
};

inline std::ostream& operator<<(std::ostream& aStream, const RepeatMode& aInstance) {
  switch (aInstance) {
    case RepeatMode::Stretch: aStream << "Stretch"; break;
    case RepeatMode::Repeat: aStream << "Repeat"; break;
    case RepeatMode::Round: aStream << "Round"; break;
    case RepeatMode::Space: aStream << "Space"; break;
    case RepeatMode::Sentinel: aStream << "Sentinel"; break;
  }
  return aStream;
}

enum class TelemetryProbe {
  SceneBuildTime = 0,
  SceneSwapTime = 1,
  FrameBuildTime = 2,
  /// Must be last for serialization purposes
  Sentinel,
};

inline std::ostream& operator<<(std::ostream& aStream, const TelemetryProbe& aInstance) {
  switch (aInstance) {
    case TelemetryProbe::SceneBuildTime: aStream << "SceneBuildTime"; break;
    case TelemetryProbe::SceneSwapTime: aStream << "SceneSwapTime"; break;
    case TelemetryProbe::FrameBuildTime: aStream << "FrameBuildTime"; break;
    case TelemetryProbe::Sentinel: aStream << "Sentinel"; break;
  }
  return aStream;
}

enum class TransformStyle : uint8_t {
  Flat = 0,
  Preserve3D = 1,
  /// Must be last for serialization purposes
  Sentinel,
};

inline std::ostream& operator<<(std::ostream& aStream, const TransformStyle& aInstance) {
  switch (aInstance) {
    case TransformStyle::Flat: aStream << "Flat"; break;
    case TransformStyle::Preserve3D: aStream << "Preserve3D"; break;
    case TransformStyle::Sentinel: aStream << "Sentinel"; break;
  }
  return aStream;
}

enum class WrAnimationType : uint32_t {
  Transform = 0,
  Opacity = 1,
  BackgroundColor = 2,
  /// Must be last for serialization purposes
  Sentinel,
};

inline std::ostream& operator<<(std::ostream& aStream, const WrAnimationType& aInstance) {
  switch (aInstance) {
    case WrAnimationType::Transform: aStream << "Transform"; break;
    case WrAnimationType::Opacity: aStream << "Opacity"; break;
    case WrAnimationType::BackgroundColor: aStream << "BackgroundColor"; break;
    case WrAnimationType::Sentinel: aStream << "Sentinel"; break;
  }
  return aStream;
}

enum class WrExternalImageType : uint32_t {
  RawData,
  NativeTexture,
  Invalid,
  /// Must be last for serialization purposes
  Sentinel,
};

inline std::ostream& operator<<(std::ostream& aStream, const WrExternalImageType& aInstance) {
  switch (aInstance) {
    case WrExternalImageType::RawData: aStream << "RawData"; break;
    case WrExternalImageType::NativeTexture: aStream << "NativeTexture"; break;
    case WrExternalImageType::Invalid: aStream << "Invalid"; break;
    case WrExternalImageType::Sentinel: aStream << "Sentinel"; break;
  }
  return aStream;
}

enum class WrReferenceFrameKind : uint8_t {
  Transform,
  Perspective,
  Zoom,
  /// Must be last for serialization purposes
  Sentinel,
};

inline std::ostream& operator<<(std::ostream& aStream, const WrReferenceFrameKind& aInstance) {
  switch (aInstance) {
    case WrReferenceFrameKind::Transform: aStream << "Transform"; break;
    case WrReferenceFrameKind::Perspective: aStream << "Perspective"; break;
    case WrReferenceFrameKind::Zoom: aStream << "Zoom"; break;
    case WrReferenceFrameKind::Sentinel: aStream << "Sentinel"; break;
  }
  return aStream;
}

enum class WrRotation : uint8_t {
  Degree0,
  Degree90,
  Degree180,
  Degree270,
  /// Must be last for serialization purposes
  Sentinel,
};

inline std::ostream& operator<<(std::ostream& aStream, const WrRotation& aInstance) {
  switch (aInstance) {
    case WrRotation::Degree0: aStream << "Degree0"; break;
    case WrRotation::Degree90: aStream << "Degree90"; break;
    case WrRotation::Degree180: aStream << "Degree180"; break;
    case WrRotation::Degree270: aStream << "Degree270"; break;
    case WrRotation::Sentinel: aStream << "Sentinel"; break;
  }
  return aStream;
}

enum class YuvColorSpace : uint8_t {
  Rec601 = 0,
  Rec709 = 1,
  Rec2020 = 2,
  Identity = 3,
  /// Must be last for serialization purposes
  Sentinel,
};

inline std::ostream& operator<<(std::ostream& aStream, const YuvColorSpace& aInstance) {
  switch (aInstance) {
    case YuvColorSpace::Rec601: aStream << "Rec601"; break;
    case YuvColorSpace::Rec709: aStream << "Rec709"; break;
    case YuvColorSpace::Rec2020: aStream << "Rec2020"; break;
    case YuvColorSpace::Identity: aStream << "Identity"; break;
    case YuvColorSpace::Sentinel: aStream << "Sentinel"; break;
  }
  return aStream;
}

template<typename T = void>
struct Arc;

/// Features of the batch that, if not requested, may allow a fast-path.
///
/// Rather than breaking batches when primitives request different features,
/// we always request the minimum amount of features to satisfy all items in
/// the batch.
/// The goal is to let the renderer be optionally select more specialized
/// versions of a shader if the batch doesn't require code certain code paths.
/// Not all shaders necessarily implement all of these features.
struct BatchFeatures;

/// A set of flags describing why a picture may need a backing surface.
struct BlitReason;

/// Flags that define how the common brush shader
/// code should process this instance.
struct BrushFlags;

/// Bit flags for WR stages to store in a capture.
struct CaptureBits;

/// Mask for clearing caches in debug commands.
struct ClearCache;

struct ClipNodeFlags;

/// A set of flags describing why a picture may need a backing surface.
struct ClusterFlags;

struct Device;

/// Geometry in the coordinate system of the render target (screen or intermediate
/// surface) in physical pixels.
struct DevicePixel;

struct DocumentHandle;

/// Each bit of the edge AA mask is:
/// 0, when the edge of the primitive needs to be considered for AA
/// 1, when the edge of the segment needs to be considered for AA
///
/// *Note*: the bit values have to match the shader logic in
/// `write_transform_vertex()` function.
struct EdgeAaSegmentMask;

/// Various flags that are part of an image descriptor.
struct ImageDescriptorFlags;

struct ItemFlags;

/// Geometry in a stacking context's local coordinate space (logical pixels).
struct LayoutPixel;

/// A set of bitflags that can be set in the visibility information
/// for a primitive instance. This can be used to control how primitives
/// are treated during batching.
struct PrimitiveVisibilityFlags;

/// The renderer is responsible for submitting to the GPU the work prepared by the
/// RenderBackend.
///
/// We have a separate `Renderer` instance for each instance of WebRender (generally
/// one per OS window), and all instances share the same thread.
struct Renderer;

/// Flags that control how shaders are pre-cached, if at all.
struct ShaderPrecacheFlags;

/// Slice flags
struct SliceFlags;

/// Startup parameters for the texture cache.
///
/// Texture sizes must be at least 512.
struct TextureCacheConfig;

struct TextureFlags;

/// Unit for tile coordinates.
struct TileCoordinate;

/// A Transaction is a group of commands to apply atomically to a document.
///
/// This mechanism ensures that:
///  - no other message can be interleaved between two commands that need to be applied together.
///  - no redundant work is performed if two commands in the same transaction cause the scene or
///    the frame to be rebuilt.
struct Transaction;

template<typename T = void>
struct Vec;

/// Geometry in the document's coordinate space (logical pixels).
struct WorldPixel;

struct WrProgramCache;

/// A wrapper around a strong reference to a Shaders object.
struct WrShaders;

struct WrState;

struct WrThreadPool;

struct WrVecU8 {
  uint8_t *data;
  uintptr_t length;
  uintptr_t capacity;

  friend std::ostream& operator<<(std::ostream& aStream, const WrVecU8& aInstance) {
    return aStream << "{ " << "data=" << aInstance.data << ", "
                           << "length=" << aInstance.length << ", "
                           << "capacity=" << aInstance.capacity << " }";
  }
  bool operator==(const WrVecU8& aOther) const {
    return data == aOther.data &&
           length == aOther.length &&
           capacity == aOther.capacity;
  }
};

struct ByteSlice {
  const uint8_t *buffer;
  uintptr_t len;

  friend std::ostream& operator<<(std::ostream& aStream, const ByteSlice& aInstance) {
    return aStream << "{ " << "buffer=" << aInstance.buffer << ", "
                           << "len=" << aInstance.len << " }";
  }
  bool operator==(const ByteSlice& aOther) const {
    return buffer == aOther.buffer &&
           len == aOther.len;
  }
};

struct WrExternalImage {
  WrExternalImageType image_type;
  uint32_t handle;
  float u0;
  float v0;
  float u1;
  float v1;
  const uint8_t *buff;
  uintptr_t size;

  friend std::ostream& operator<<(std::ostream& aStream, const WrExternalImage& aInstance) {
    return aStream << "{ " << "image_type=" << aInstance.image_type << ", "
                           << "handle=" << aInstance.handle << ", "
                           << "u0=" << aInstance.u0 << ", "
                           << "v0=" << aInstance.v0 << ", "
                           << "u1=" << aInstance.u1 << ", "
                           << "v1=" << aInstance.v1 << ", "
                           << "buff=" << aInstance.buff << ", "
                           << "size=" << aInstance.size << " }";
  }
  bool operator==(const WrExternalImage& aOther) const {
    return image_type == aOther.image_type &&
           handle == aOther.handle &&
           u0 == aOther.u0 &&
           v0 == aOther.v0 &&
           u1 == aOther.u1 &&
           v1 == aOther.v1 &&
           buff == aOther.buff &&
           size == aOther.size;
  }
};

/// An arbitrary identifier for an external image provided by the
/// application. It must be a unique identifier for each external
/// image.
struct ExternalImageId {
  uint64_t _0;

  friend std::ostream& operator<<(std::ostream& aStream, const ExternalImageId& aInstance) {
    return aStream << "{ " << "_0=" << aInstance._0 << " }";
  }
  bool operator==(const ExternalImageId& aOther) const {
    return _0 == aOther._0;
  }
};

struct WrWindowId {
  uint64_t mHandle;

  friend std::ostream& operator<<(std::ostream& aStream, const WrWindowId& aInstance) {
    return aStream << "{ " << "mHandle=" << aInstance.mHandle << " }";
  }
  bool operator==(const WrWindowId& aOther) const {
    return mHandle == aOther.mHandle;
  }
  bool operator<(const WrWindowId& aOther) const {
    return mHandle < aOther.mHandle;
  }
  bool operator<=(const WrWindowId& aOther) const {
    return mHandle <= aOther.mHandle;
  }
};

/// This type carries no valuable semantics for WR. However, it reflects the fact that
/// clients (Servo) may generate pipelines by different semi-independent sources.
/// These pipelines still belong to the same `IdNamespace` and the same `DocumentId`.
/// Having this extra Id field enables them to generate `PipelineId` without collision.
using PipelineSourceId = uint32_t;

/// From the point of view of WR, `PipelineId` is completely opaque and generic as long as
/// it's clonable, serializable, comparable, and hashable.
struct PipelineId {
  PipelineSourceId mNamespace;
  uint32_t mHandle;

  friend std::ostream& operator<<(std::ostream& aStream, const PipelineId& aInstance) {
    return aStream << "{ " << "mNamespace=" << aInstance.mNamespace << ", "
                           << "mHandle=" << aInstance.mHandle << " }";
  }
  bool operator==(const PipelineId& aOther) const {
    return mNamespace == aOther.mNamespace &&
           mHandle == aOther.mHandle;
  }
};

using WrPipelineId = PipelineId;

/// ID namespaces uniquely identify different users of WebRender's API.
///
/// For example in Gecko each content process uses a separate id namespace.
struct IdNamespace {
  uint32_t mHandle;

  friend std::ostream& operator<<(std::ostream& aStream, const IdNamespace& aInstance) {
    return aStream << "{ " << "mHandle=" << aInstance.mHandle << " }";
  }
  bool operator==(const IdNamespace& aOther) const {
    return mHandle == aOther.mHandle;
  }
  bool operator!=(const IdNamespace& aOther) const {
    return mHandle != aOther.mHandle;
  }
  bool operator<(const IdNamespace& aOther) const {
    return mHandle < aOther.mHandle;
  }
  bool operator<=(const IdNamespace& aOther) const {
    return mHandle <= aOther.mHandle;
  }
};

/// A key uniquely identifying a WebRender document.
///
/// Instances can manage one or several documents (using the same render backend thread).
/// Each document will internally correspond to a single scene, and scenes are made of
/// one or several pipelines.
struct DocumentId {
  ///
  IdNamespace mNamespace;
  ///
  uint32_t mHandle;

  friend std::ostream& operator<<(std::ostream& aStream, const DocumentId& aInstance) {
    return aStream << "{ " << "mNamespace=" << aInstance.mNamespace << ", "
                           << "mHandle=" << aInstance.mHandle << " }";
  }
  bool operator==(const DocumentId& aOther) const {
    return mNamespace == aOther.mNamespace &&
           mHandle == aOther.mHandle;
  }
};

using WrDocumentId = DocumentId;

/// An epoch identifies the state of a pipeline in time.
///
/// This is mostly used as a synchronization mechanism to observe how/when particular pipeline
/// updates propagate through WebRender and are applied at various stages.
struct Epoch {
  uint32_t mHandle;

  friend std::ostream& operator<<(std::ostream& aStream, const Epoch& aInstance) {
    return aStream << "{ " << "mHandle=" << aInstance.mHandle << " }";
  }
  bool operator==(const Epoch& aOther) const {
    return mHandle == aOther.mHandle;
  }
  bool operator!=(const Epoch& aOther) const {
    return mHandle != aOther.mHandle;
  }
  bool operator<(const Epoch& aOther) const {
    return mHandle < aOther.mHandle;
  }
  bool operator<=(const Epoch& aOther) const {
    return mHandle <= aOther.mHandle;
  }
};

using WrEpoch = Epoch;

struct WrPipelineEpoch {
  WrPipelineId pipeline_id;
  WrDocumentId document_id;
  WrEpoch epoch;

  friend std::ostream& operator<<(std::ostream& aStream, const WrPipelineEpoch& aInstance) {
    return aStream << "{ " << "pipeline_id=" << aInstance.pipeline_id << ", "
                           << "document_id=" << aInstance.document_id << ", "
                           << "epoch=" << aInstance.epoch << " }";
  }
  bool operator==(const WrPipelineEpoch& aOther) const {
    return pipeline_id == aOther.pipeline_id &&
           document_id == aOther.document_id &&
           epoch == aOther.epoch;
  }
};

struct WrRemovedPipeline {
  WrPipelineId pipeline_id;
  WrDocumentId document_id;

  friend std::ostream& operator<<(std::ostream& aStream, const WrRemovedPipeline& aInstance) {
    return aStream << "{ " << "pipeline_id=" << aInstance.pipeline_id << ", "
                           << "document_id=" << aInstance.document_id << " }";
  }
  bool operator==(const WrRemovedPipeline& aOther) const {
    return pipeline_id == aOther.pipeline_id &&
           document_id == aOther.document_id;
  }
};

struct WrPipelineInfo {
  /// This contains an entry for each pipeline that was rendered, along with
  /// the epoch at which it was rendered. Rendered pipelines include the root
  /// pipeline and any other pipelines that were reachable via IFrame display
  /// items from the root pipeline.
  nsTArray<WrPipelineEpoch> epochs;
  /// This contains an entry for each pipeline that was removed during the
  /// last transaction. These pipelines would have been explicitly removed by
  /// calling remove_pipeline on the transaction object; the pipeline showing
  /// up in this array means that the data structures have been torn down on
  /// the webrender side, and so any remaining data structures on the caller
  /// side can now be torn down also.
  nsTArray<WrRemovedPipeline> removed_pipelines;

  friend std::ostream& operator<<(std::ostream& aStream, const WrPipelineInfo& aInstance) {
    return aStream << "{ " << "epochs=" << aInstance.epochs << ", "
                           << "removed_pipelines=" << aInstance.removed_pipelines << " }";
  }
  bool operator==(const WrPipelineInfo& aOther) const {
    return epochs == aOther.epochs &&
           removed_pipelines == aOther.removed_pipelines;
  }
};

/// Represents RGBA screen colors with floating point numbers.
///
/// All components must be between 0.0 and 1.0.
/// An alpha value of 1.0 is opaque while 0.0 is fully transparent.
struct ColorF {
  float r;
  float g;
  float b;
  float a;

  friend std::ostream& operator<<(std::ostream& aStream, const ColorF& aInstance) {
    return aStream << "{ " << "r=" << aInstance.r << ", "
                           << "g=" << aInstance.g << ", "
                           << "b=" << aInstance.b << ", "
                           << "a=" << aInstance.a << " }";
  }
  bool operator==(const ColorF& aOther) const {
    return r == aOther.r &&
           g == aOther.g &&
           b == aOther.b &&
           a == aOther.a;
  }
  static const ColorF BLACK;
  static const ColorF TRANSPARENT;
  static const ColorF WHITE;
};
inline const ColorF ColorF::BLACK = ColorF{ /* .r = */ 0.0, /* .g = */ 0.0, /* .b = */ 0.0, /* .a = */ 1.0 };
inline const ColorF ColorF::TRANSPARENT = ColorF{ /* .r = */ 0.0, /* .g = */ 0.0, /* .b = */ 0.0, /* .a = */ 0.0 };
inline const ColorF ColorF::WHITE = ColorF{ /* .r = */ 1.0, /* .g = */ 1.0, /* .b = */ 1.0, /* .a = */ 1.0 };

struct WrExternalImageHandler {
  void *external_image_obj;

  friend std::ostream& operator<<(std::ostream& aStream, const WrExternalImageHandler& aInstance) {
    return aStream << "{ " << "external_image_obj=" << aInstance.external_image_obj << " }";
  }
  bool operator==(const WrExternalImageHandler& aOther) const {
    return external_image_obj == aOther.external_image_obj;
  }
};

/// Some basic statistics about the rendered scene, used in Gecko, as
/// well as in wrench reftests to ensure that tests are batching and/or
/// allocating on render targets as we expect them to.
struct RendererStats {
  uintptr_t total_draw_calls;
  uintptr_t alpha_target_count;
  uintptr_t color_target_count;
  double texture_upload_mb;
  double resource_upload_time;
  double gpu_cache_upload_time;

  friend std::ostream& operator<<(std::ostream& aStream, const RendererStats& aInstance) {
    return aStream << "{ " << "total_draw_calls=" << aInstance.total_draw_calls << ", "
                           << "alpha_target_count=" << aInstance.alpha_target_count << ", "
                           << "color_target_count=" << aInstance.color_target_count << ", "
                           << "texture_upload_mb=" << aInstance.texture_upload_mb << ", "
                           << "resource_upload_time=" << aInstance.resource_upload_time << ", "
                           << "gpu_cache_upload_time=" << aInstance.gpu_cache_upload_time << " }";
  }
  bool operator==(const RendererStats& aOther) const {
    return total_draw_calls == aOther.total_draw_calls &&
           alpha_target_count == aOther.alpha_target_count &&
           color_target_count == aOther.color_target_count &&
           texture_upload_mb == aOther.texture_upload_mb &&
           resource_upload_time == aOther.resource_upload_time &&
           gpu_cache_upload_time == aOther.gpu_cache_upload_time;
  }
};

/// A 2d Point tagged with a unit.
template<typename T, typename U>
struct Point2D {
  T x;
  T y;

  friend std::ostream& operator<<(std::ostream& aStream, const Point2D& aInstance) {
    return aStream << "{ " << "x=" << aInstance.x << ", "
                           << "y=" << aInstance.y << " }";
  }
  bool operator==(const Point2D& aOther) const {
    return x == aOther.x &&
           y == aOther.y;
  }
};

/// A 2d size tagged with a unit.
template<typename T, typename U>
struct Size2D {
  /// The extent of the element in the `U` units along the `x` axis (usually horizontal).
  T width;
  /// The extent of the element in the `U` units along the `y` axis (usually vertical).
  T height;

  friend std::ostream& operator<<(std::ostream& aStream, const Size2D& aInstance) {
    return aStream << "{ " << "width=" << aInstance.width << ", "
                           << "height=" << aInstance.height << " }";
  }
  bool operator==(const Size2D& aOther) const {
    return width == aOther.width &&
           height == aOther.height;
  }
};

/// A 2d Rectangle optionally tagged with a unit.
///
/// # Representation
///
/// `Rect` is represented by an origin point and a size.
///
/// See [`Rect`] for a rectangle represented by two endpoints.
///
/// # Empty rectangle
///
/// A rectangle is considered empty (see [`is_empty`]) if any of the following is true:
/// - it's area is empty,
/// - it's area is negative (`size.x < 0` or `size.y < 0`),
/// - it contains NaNs.
///
/// [`is_empty`]: #method.is_empty
/// [`Box2D`]: struct.Box2D.html
template<typename T, typename U>
struct Rect {
  Point2D<T, U> origin;
  Size2D<T, U> size;

  friend std::ostream& operator<<(std::ostream& aStream, const Rect& aInstance) {
    return aStream << "{ " << "origin=" << aInstance.origin << ", "
                           << "size=" << aInstance.size << " }";
  }
  bool operator==(const Rect& aOther) const {
    return origin == aOther.origin &&
           size == aOther.size;
  }
};

using DeviceIntRect = Rect<int32_t, DevicePixel>;

/// A handle to a recorded frame that was captured.
struct RecordedFrameHandle {
  uintptr_t _0;

  friend std::ostream& operator<<(std::ostream& aStream, const RecordedFrameHandle& aInstance) {
    return aStream << "{ " << "_0=" << aInstance._0 << " }";
  }
  bool operator==(const RecordedFrameHandle& aOther) const {
    return _0 == aOther._0;
  }
};

/// A handle to a screenshot that is being asynchronously captured and scaled.
struct AsyncScreenshotHandle {
  uintptr_t _0;

  friend std::ostream& operator<<(std::ostream& aStream, const AsyncScreenshotHandle& aInstance) {
    return aStream << "{ " << "_0=" << aInstance._0 << " }";
  }
  bool operator==(const AsyncScreenshotHandle& aOther) const {
    return _0 == aOther._0;
  }
};

/// Memory report for interning-related data structures.
struct InterningMemoryReport {
  ///
  InternerSubReport interners;
  ///
  InternerSubReport data_stores;
};

/// Collection of heap sizes, in bytes.
struct MemoryReport {
  uintptr_t clip_stores;
  uintptr_t gpu_cache_metadata;
  uintptr_t gpu_cache_cpu_mirror;
  uintptr_t render_tasks;
  uintptr_t hit_testers;
  uintptr_t fonts;
  uintptr_t weak_fonts;
  uintptr_t images;
  uintptr_t rasterized_blobs;
  uintptr_t shader_cache;
  InterningMemoryReport interning;
  uintptr_t display_list;
  uintptr_t upload_staging_memory;
  uintptr_t gpu_cache_textures;
  uintptr_t vertex_data_textures;
  uintptr_t render_target_textures;
  uintptr_t texture_cache_textures;
  uintptr_t texture_cache_structures;
  uintptr_t depth_target_textures;
  uintptr_t texture_upload_pbos;
  uintptr_t swap_chain;
  uintptr_t render_texture_hosts;
  uintptr_t upload_staging_textures;
};

/// An arbitrary identifier for a native (OS compositor) surface
struct NativeSurfaceId {
  uint64_t _0;

  friend std::ostream& operator<<(std::ostream& aStream, const NativeSurfaceId& aInstance) {
    return aStream << "{ " << "_0=" << aInstance._0 << " }";
  }
  bool operator==(const NativeSurfaceId& aOther) const {
    return _0 == aOther._0;
  }
};

using DeviceIntPoint = Point2D<int32_t, DevicePixel>;

using DeviceIntSize = Size2D<int32_t, DevicePixel>;

struct NativeTileId {
  NativeSurfaceId surface_id;
  int32_t x;
  int32_t y;

  friend std::ostream& operator<<(std::ostream& aStream, const NativeTileId& aInstance) {
    return aStream << "{ " << "surface_id=" << aInstance.surface_id << ", "
                           << "x=" << aInstance.x << ", "
                           << "y=" << aInstance.y << " }";
  }
  bool operator==(const NativeTileId& aOther) const {
    return surface_id == aOther.surface_id &&
           x == aOther.x &&
           y == aOther.y;
  }
};

/// A 3d transform stored as a column-major 4 by 4 matrix.
///
/// Transforms can be parametrized over the source and destination units, to describe a
/// transformation from a space to another.
/// For example, `Transform3D<f32, WorldSpace, ScreenSpace>::transform_point3d`
/// takes a `Point3D<f32, WorldSpace>` and returns a `Point3D<f32, ScreenSpace>`.
///
/// Transforms expose a set of convenience methods for pre- and post-transformations.
/// Pre-transformations (`pre_*` methods) correspond to adding an operation that is
/// applied before the rest of the transformation, while post-transformations (`then_*`
/// methods) add an operation that is applied after.
///
/// When translating Transform3D into general matrix representations, consider that the
/// representation follows the column major notation with column vectors.
///
/// ```text
///  |x'|   | m11 m12 m13 m14 |   |x|
///  |y'|   | m21 m22 m23 m24 |   |y|
///  |z'| = | m31 m32 m33 m34 | x |y|
///  |w |   | m41 m42 m43 m44 |   |1|
/// ```
///
/// The translation terms are m41, m42 and m43.
template<typename T, typename Src, typename Dst>
struct Transform3D {
  T m11;
  T m12;
  T m13;
  T m14;
  T m21;
  T m22;
  T m23;
  T m24;
  T m31;
  T m32;
  T m33;
  T m34;
  T m41;
  T m42;
  T m43;
  T m44;

  friend std::ostream& operator<<(std::ostream& aStream, const Transform3D& aInstance) {
    return aStream << "{ " << "m11=" << aInstance.m11 << ", "
                           << "m12=" << aInstance.m12 << ", "
                           << "m13=" << aInstance.m13 << ", "
                           << "m14=" << aInstance.m14 << ", "
                           << "m21=" << aInstance.m21 << ", "
                           << "m22=" << aInstance.m22 << ", "
                           << "m23=" << aInstance.m23 << ", "
                           << "m24=" << aInstance.m24 << ", "
                           << "m31=" << aInstance.m31 << ", "
                           << "m32=" << aInstance.m32 << ", "
                           << "m33=" << aInstance.m33 << ", "
                           << "m34=" << aInstance.m34 << ", "
                           << "m41=" << aInstance.m41 << ", "
                           << "m42=" << aInstance.m42 << ", "
                           << "m43=" << aInstance.m43 << ", "
                           << "m44=" << aInstance.m44 << " }";
  }
  bool operator==(const Transform3D& aOther) const {
    return m11 == aOther.m11 &&
           m12 == aOther.m12 &&
           m13 == aOther.m13 &&
           m14 == aOther.m14 &&
           m21 == aOther.m21 &&
           m22 == aOther.m22 &&
           m23 == aOther.m23 &&
           m24 == aOther.m24 &&
           m31 == aOther.m31 &&
           m32 == aOther.m32 &&
           m33 == aOther.m33 &&
           m34 == aOther.m34 &&
           m41 == aOther.m41 &&
           m42 == aOther.m42 &&
           m43 == aOther.m43 &&
           m44 == aOther.m44;
  }
};

/// The transform type to apply to Compositor surfaces.
using CompositorSurfaceTransform = Transform3D<float, DevicePixel, DevicePixel>;

struct CompositorCapabilities {
  /// The virtual surface size used by the underlying platform.
  int32_t virtual_surface_size;
  /// Whether the compositor requires redrawing on invalidation.
  bool redraw_on_invalidation;

  friend std::ostream& operator<<(std::ostream& aStream, const CompositorCapabilities& aInstance) {
    return aStream << "{ " << "virtual_surface_size=" << aInstance.virtual_surface_size << ", "
                           << "redraw_on_invalidation=" << aInstance.redraw_on_invalidation << " }";
  }
  bool operator==(const CompositorCapabilities& aOther) const {
    return virtual_surface_size == aOther.virtual_surface_size &&
           redraw_on_invalidation == aOther.redraw_on_invalidation;
  }
};

/// Descriptor for a locked surface that will be directly composited by SWGL.
struct SWGLCompositeSurfaceInfo {
  /// The number of YUV planes in the surface. 0 indicates non-YUV BGRA.
  /// 1 is interleaved YUV. 2 is NV12. 3 is planar YUV.
  uint32_t yuv_planes;
  /// Textures for planes of the surface, or 0 if not applicable.
  uint32_t textures[3];
  /// Color space of surface if using a YUV format.
  YuvColorSpace color_space;
  /// Color depth of surface if using a YUV format.
  ColorDepth color_depth;
  /// The actual source surface size before transformation.
  DeviceIntSize size;

  friend std::ostream& operator<<(std::ostream& aStream, const SWGLCompositeSurfaceInfo& aInstance) {
    return aStream << "{ " << "yuv_planes=" << aInstance.yuv_planes << ", "
                           << "textures=" << aInstance.textures << ", "
                           << "color_space=" << aInstance.color_space << ", "
                           << "color_depth=" << aInstance.color_depth << ", "
                           << "size=" << aInstance.size << " }";
  }
};

/// A C function that takes a pointer to a heap allocation and returns its size.
///
/// This is borrowed from the malloc_size_of crate, upon which we want to avoid
/// a dependency from WebRender.
using VoidPtrToSizeFn = uintptr_t(*)(const void *ptr);

/// Flags to enable/disable various builtin debugging tools.
struct DebugFlags {
  uint32_t bits;

  explicit operator bool() const {
    return !!bits;
  }
  DebugFlags operator~() const {
    return {static_cast<decltype(bits)>(~bits)};
  }
  DebugFlags operator|(const DebugFlags& aOther) const {
    return {static_cast<decltype(bits)>(this->bits | aOther.bits)};
  }
  DebugFlags& operator|=(const DebugFlags& aOther) {
    *this = (*this | aOther);
    return *this;
  }
  DebugFlags operator&(const DebugFlags& aOther) const {
    return {static_cast<decltype(bits)>(this->bits & aOther.bits)};
  }
  DebugFlags& operator&=(const DebugFlags& aOther) {
    *this = (*this & aOther);
    return *this;
  }
  DebugFlags operator^(const DebugFlags& aOther) const {
    return {static_cast<decltype(bits)>(this->bits ^ aOther.bits)};
  }
  DebugFlags& operator^=(const DebugFlags& aOther) {
    *this = (*this ^ aOther);
    return *this;
  }
  friend std::ostream& operator<<(std::ostream& aStream, const DebugFlags& aInstance) {
    return aStream << "{ " << "bits=" << aInstance.bits << " }";
  }
  bool operator==(const DebugFlags& aOther) const {
    return bits == aOther.bits;
  }
  static const DebugFlags PROFILER_DBG;
  static const DebugFlags RENDER_TARGET_DBG;
  static const DebugFlags TEXTURE_CACHE_DBG;
  static const DebugFlags GPU_TIME_QUERIES;
  static const DebugFlags GPU_SAMPLE_QUERIES;
  static const DebugFlags DISABLE_BATCHING;
  static const DebugFlags EPOCHS;
  static const DebugFlags ECHO_DRIVER_MESSAGES;
  static const DebugFlags SHOW_OVERDRAW;
  static const DebugFlags GPU_CACHE_DBG;
  static const DebugFlags TEXTURE_CACHE_DBG_CLEAR_EVICTED;
  static const DebugFlags PICTURE_CACHING_DBG;
  static const DebugFlags PRIMITIVE_DBG;
  static const DebugFlags ZOOM_DBG;
  static const DebugFlags SMALL_SCREEN;
  static const DebugFlags DISABLE_OPAQUE_PASS;
  static const DebugFlags DISABLE_ALPHA_PASS;
  static const DebugFlags DISABLE_CLIP_MASKS;
  static const DebugFlags DISABLE_TEXT_PRIMS;
  static const DebugFlags DISABLE_GRADIENT_PRIMS;
  static const DebugFlags OBSCURE_IMAGES;
  static const DebugFlags GLYPH_FLASHING;
  static const DebugFlags SMART_PROFILER;
  static const DebugFlags INVALIDATION_DBG;
  static const DebugFlags TILE_CACHE_LOGGING_DBG;
  static const DebugFlags PROFILER_CAPTURE;
  static const DebugFlags FORCE_PICTURE_INVALIDATION;
  static const DebugFlags USE_BATCHED_TEXTURE_UPLOADS;
  static const DebugFlags USE_DRAW_CALLS_FOR_TEXTURE_COPY;
};
/// Display the frame profiler on screen.
inline const DebugFlags DebugFlags::PROFILER_DBG = DebugFlags{ /* .bits = */ (uint32_t)(1 << 0) };
/// Display intermediate render targets on screen.
inline const DebugFlags DebugFlags::RENDER_TARGET_DBG = DebugFlags{ /* .bits = */ (uint32_t)(1 << 1) };
/// Display all texture cache pages on screen.
inline const DebugFlags DebugFlags::TEXTURE_CACHE_DBG = DebugFlags{ /* .bits = */ (uint32_t)(1 << 2) };
/// Display GPU timing results.
inline const DebugFlags DebugFlags::GPU_TIME_QUERIES = DebugFlags{ /* .bits = */ (uint32_t)(1 << 3) };
/// Query the number of pixels that pass the depth test divided and show it
/// in the profiler as a percentage of the number of pixels in the screen
/// (window width times height).
inline const DebugFlags DebugFlags::GPU_SAMPLE_QUERIES = DebugFlags{ /* .bits = */ (uint32_t)(1 << 4) };
/// Render each quad with their own draw call.
///
/// Terrible for performance but can help with understanding the drawing
/// order when inspecting renderdoc or apitrace recordings.
inline const DebugFlags DebugFlags::DISABLE_BATCHING = DebugFlags{ /* .bits = */ (uint32_t)(1 << 5) };
/// Display the pipeline epochs.
inline const DebugFlags DebugFlags::EPOCHS = DebugFlags{ /* .bits = */ (uint32_t)(1 << 6) };
/// Print driver messages to stdout.
inline const DebugFlags DebugFlags::ECHO_DRIVER_MESSAGES = DebugFlags{ /* .bits = */ (uint32_t)(1 << 7) };
/// Show an overlay displaying overdraw amount.
inline const DebugFlags DebugFlags::SHOW_OVERDRAW = DebugFlags{ /* .bits = */ (uint32_t)(1 << 8) };
/// Display the contents of GPU cache.
inline const DebugFlags DebugFlags::GPU_CACHE_DBG = DebugFlags{ /* .bits = */ (uint32_t)(1 << 9) };
/// Clear evicted parts of the texture cache for debugging purposes.
inline const DebugFlags DebugFlags::TEXTURE_CACHE_DBG_CLEAR_EVICTED = DebugFlags{ /* .bits = */ (uint32_t)(1 << 10) };
/// Show picture caching debug overlay
inline const DebugFlags DebugFlags::PICTURE_CACHING_DBG = DebugFlags{ /* .bits = */ (uint32_t)(1 << 11) };
/// Highlight all primitives with colors based on kind.
inline const DebugFlags DebugFlags::PRIMITIVE_DBG = DebugFlags{ /* .bits = */ (uint32_t)(1 << 12) };
/// Draw a zoom widget showing part of the framebuffer zoomed in.
inline const DebugFlags DebugFlags::ZOOM_DBG = DebugFlags{ /* .bits = */ (uint32_t)(1 << 13) };
/// Scale the debug renderer down for a smaller screen. This will disrupt
/// any mapping between debug display items and page content, so shouldn't
/// be used with overlays like the picture caching or primitive display.
inline const DebugFlags DebugFlags::SMALL_SCREEN = DebugFlags{ /* .bits = */ (uint32_t)(1 << 14) };
/// Disable various bits of the WebRender pipeline, to help narrow
/// down where slowness might be coming from.
inline const DebugFlags DebugFlags::DISABLE_OPAQUE_PASS = DebugFlags{ /* .bits = */ (uint32_t)(1 << 15) };
///
inline const DebugFlags DebugFlags::DISABLE_ALPHA_PASS = DebugFlags{ /* .bits = */ (uint32_t)(1 << 16) };
///
inline const DebugFlags DebugFlags::DISABLE_CLIP_MASKS = DebugFlags{ /* .bits = */ (uint32_t)(1 << 17) };
///
inline const DebugFlags DebugFlags::DISABLE_TEXT_PRIMS = DebugFlags{ /* .bits = */ (uint32_t)(1 << 18) };
///
inline const DebugFlags DebugFlags::DISABLE_GRADIENT_PRIMS = DebugFlags{ /* .bits = */ (uint32_t)(1 << 19) };
///
inline const DebugFlags DebugFlags::OBSCURE_IMAGES = DebugFlags{ /* .bits = */ (uint32_t)(1 << 20) };
/// Taint the transparent area of the glyphs with a random opacity to easily
/// see when glyphs are re-rasterized.
inline const DebugFlags DebugFlags::GLYPH_FLASHING = DebugFlags{ /* .bits = */ (uint32_t)(1 << 21) };
/// The profiler only displays information that is out of the ordinary.
inline const DebugFlags DebugFlags::SMART_PROFILER = DebugFlags{ /* .bits = */ (uint32_t)(1 << 22) };
/// If set, dump picture cache invalidation debug to console.
inline const DebugFlags DebugFlags::INVALIDATION_DBG = DebugFlags{ /* .bits = */ (uint32_t)(1 << 23) };
/// Log tile cache to memory for later saving as part of wr-capture
inline const DebugFlags DebugFlags::TILE_CACHE_LOGGING_DBG = DebugFlags{ /* .bits = */ (uint32_t)(1 << 24) };
/// Collect and dump profiler statistics to captures.
inline const DebugFlags DebugFlags::PROFILER_CAPTURE = DebugFlags{ /* .bits = */ (uint32_t)((uint32_t)1 << 25) };
/// Invalidate picture tiles every frames (useful when inspecting GPU work in external tools).
inline const DebugFlags DebugFlags::FORCE_PICTURE_INVALIDATION = DebugFlags{ /* .bits = */ (uint32_t)((uint32_t)1 << 26) };
inline const DebugFlags DebugFlags::USE_BATCHED_TEXTURE_UPLOADS = DebugFlags{ /* .bits = */ (uint32_t)((uint32_t)1 << 27) };
inline const DebugFlags DebugFlags::USE_DRAW_CALLS_FOR_TEXTURE_COPY = DebugFlags{ /* .bits = */ (uint32_t)((uint32_t)1 << 28) };

using LayoutSize = Size2D<float, LayoutPixel>;

/// Describes the memory layout of a display list.
///
/// A display list consists of some number of display list items, followed by a number of display
/// items.
struct BuiltDisplayListDescriptor {
  /// The first IPC time stamp: before any work has been done
  uint64_t builder_start_time;
  /// The second IPC time stamp: after serialization
  uint64_t builder_finish_time;
  /// The third IPC time stamp: just before sending
  uint64_t send_start_time;
  /// The amount of clipping nodes created while building this display list.
  uintptr_t total_clip_nodes;
  /// The amount of spatial nodes created while building this display list.
  uintptr_t total_spatial_nodes;
  /// The size of the cache for this display list.
  uintptr_t cache_size;
  /// The offset for additional display list data.
  uintptr_t extra_data_offset;

  friend std::ostream& operator<<(std::ostream& aStream, const BuiltDisplayListDescriptor& aInstance) {
    return aStream << "{ " << "builder_start_time=" << aInstance.builder_start_time << ", "
                           << "builder_finish_time=" << aInstance.builder_finish_time << ", "
                           << "send_start_time=" << aInstance.send_start_time << ", "
                           << "total_clip_nodes=" << aInstance.total_clip_nodes << ", "
                           << "total_spatial_nodes=" << aInstance.total_spatial_nodes << ", "
                           << "cache_size=" << aInstance.cache_size << ", "
                           << "extra_data_offset=" << aInstance.extra_data_offset << " }";
  }
  bool operator==(const BuiltDisplayListDescriptor& aOther) const {
    return builder_start_time == aOther.builder_start_time &&
           builder_finish_time == aOther.builder_finish_time &&
           send_start_time == aOther.send_start_time &&
           total_clip_nodes == aOther.total_clip_nodes &&
           total_spatial_nodes == aOther.total_spatial_nodes &&
           cache_size == aOther.cache_size &&
           extra_data_offset == aOther.extra_data_offset;
  }
};

template<typename T>
struct WrAnimationPropertyValue {
  uint64_t id;
  T value;

  friend std::ostream& operator<<(std::ostream& aStream, const WrAnimationPropertyValue& aInstance) {
    return aStream << "{ " << "id=" << aInstance.id << ", "
                           << "value=" << aInstance.value << " }";
  }
};

using WrOpacityProperty = WrAnimationPropertyValue<float>;

using LayoutTransform = Transform3D<float, LayoutPixel, LayoutPixel>;

using WrTransformProperty = WrAnimationPropertyValue<LayoutTransform>;

using WrColorProperty = WrAnimationPropertyValue<ColorF>;

using LayoutPoint = Point2D<float, LayoutPixel>;

/// An opaque identifier describing an image registered with WebRender.
/// This is used as a handle to reference images, and is used as the
/// hash map key for the actual image storage in the `ResourceCache`.
struct ImageKey {
  IdNamespace mNamespace;
  uint32_t mHandle;

  friend std::ostream& operator<<(std::ostream& aStream, const ImageKey& aInstance) {
    return aStream << "{ " << "mNamespace=" << aInstance.mNamespace << ", "
                           << "mHandle=" << aInstance.mHandle << " }";
  }
  bool operator==(const ImageKey& aOther) const {
    return mNamespace == aOther.mNamespace &&
           mHandle == aOther.mHandle;
  }
  bool operator!=(const ImageKey& aOther) const {
    return mNamespace != aOther.mNamespace ||
           mHandle != aOther.mHandle;
  }
};

using WrImageKey = ImageKey;

struct WrImageDescriptor {
  ImageFormat format;
  int32_t width;
  int32_t height;
  int32_t stride;
  OpacityType opacity;
  bool prefer_compositor_surface;

  friend std::ostream& operator<<(std::ostream& aStream, const WrImageDescriptor& aInstance) {
    return aStream << "{ " << "format=" << aInstance.format << ", "
                           << "width=" << aInstance.width << ", "
                           << "height=" << aInstance.height << ", "
                           << "stride=" << aInstance.stride << ", "
                           << "opacity=" << aInstance.opacity << ", "
                           << "prefer_compositor_surface=" << aInstance.prefer_compositor_surface << " }";
  }
  bool operator==(const WrImageDescriptor& aOther) const {
    return format == aOther.format &&
           width == aOther.width &&
           height == aOther.height &&
           stride == aOther.stride &&
           opacity == aOther.opacity &&
           prefer_compositor_surface == aOther.prefer_compositor_surface;
  }
};

/// An opaque identifier describing a blob image registered with WebRender.
/// This is used as a handle to reference blob images, and can be used as an
/// image in display items.
struct BlobImageKey {
  ImageKey _0;

  friend std::ostream& operator<<(std::ostream& aStream, const BlobImageKey& aInstance) {
    return aStream << "{ " << "_0=" << aInstance._0 << " }";
  }
  bool operator==(const BlobImageKey& aOther) const {
    return _0 == aOther._0;
  }
};

/// Storage format identifier for externally-managed images.
union ExternalImageType {
  enum class Tag : uint8_t {
    /// The image is texture-backed.
    TextureHandle,
    /// The image is heap-allocated by the embedding.
    Buffer,
    /// Must be last for serialization purposes
    Sentinel,
  };

  friend std::ostream& operator<<(std::ostream& aStream, const Tag& aInstance) {
    using Tag = ExternalImageType::Tag;
    switch (aInstance) {
      case Tag::TextureHandle: aStream << "TextureHandle"; break;
      case Tag::Buffer: aStream << "Buffer"; break;
      case Tag::Sentinel: aStream << "Sentinel"; break;
    }
    return aStream;
  }

  friend std::ostream& operator<<(std::ostream& aStream, const ExternalImageType& aInstance) {
    using Tag = ExternalImageType::Tag;
    switch (aInstance.tag) {
      case Tag::TextureHandle: aStream << aInstance.texture_handle; break;
      case Tag::Buffer: aStream << "Buffer"; break;
      case Tag::Sentinel: aStream << "Sentinel"; break;
    }
    return aStream;
  }

  struct TextureHandle_Body {
    Tag tag;
    ImageBufferKind _0;

    friend std::ostream& operator<<(std::ostream& aStream, const TextureHandle_Body& aInstance) {
      return aStream << "{ " << "tag=" << aInstance.tag << ", "
                             << "_0=" << aInstance._0 << " }";
    }
    bool operator==(const TextureHandle_Body& aOther) const {
      return _0 == aOther._0;
    }
  };

  struct {
    Tag tag;
  };
  TextureHandle_Body texture_handle;

  static ExternalImageType TextureHandle(const ImageBufferKind &a0) {
    ExternalImageType result;
    ::new (&result.texture_handle._0) (ImageBufferKind)(a0);
    result.tag = Tag::TextureHandle;
    return result;
  }

  bool IsTextureHandle() const {
    return tag == Tag::TextureHandle;
  }

  static ExternalImageType Buffer() {
    ExternalImageType result;
    result.tag = Tag::Buffer;
    return result;
  }

  bool IsBuffer() const {
    return tag == Tag::Buffer;
  }

  static ExternalImageType Sentinel() {
    ExternalImageType result;
    result.tag = Tag::Sentinel;
    return result;
  }

  bool IsSentinel() const {
    return tag == Tag::Sentinel;
  }

  bool operator==(const ExternalImageType& aOther) const {
    if (tag != aOther.tag) {
      return false;
    }
    switch (tag) {
      case Tag::TextureHandle: return texture_handle == aOther.texture_handle;
      default: break;
    }
    return true;
  }
};

using LayoutIntRect = Rect<int32_t, LayoutPixel>;

struct FontKey {
  IdNamespace mNamespace;
  uint32_t mHandle;

  friend std::ostream& operator<<(std::ostream& aStream, const FontKey& aInstance) {
    return aStream << "{ " << "mNamespace=" << aInstance.mNamespace << ", "
                           << "mHandle=" << aInstance.mHandle << " }";
  }
  bool operator==(const FontKey& aOther) const {
    return mNamespace == aOther.mNamespace &&
           mHandle == aOther.mHandle;
  }
};

using WrFontKey = FontKey;

struct FontInstanceKey {
  IdNamespace mNamespace;
  uint32_t mHandle;

  friend std::ostream& operator<<(std::ostream& aStream, const FontInstanceKey& aInstance) {
    return aStream << "{ " << "mNamespace=" << aInstance.mNamespace << ", "
                           << "mHandle=" << aInstance.mHandle << " }";
  }
  bool operator==(const FontInstanceKey& aOther) const {
    return mNamespace == aOther.mNamespace &&
           mHandle == aOther.mHandle;
  }
};

using WrFontInstanceKey = FontInstanceKey;

struct FontInstanceFlags {
  uint32_t bits;

  explicit operator bool() const {
    return !!bits;
  }
  FontInstanceFlags operator~() const {
    return {static_cast<decltype(bits)>(~bits)};
  }
  FontInstanceFlags operator|(const FontInstanceFlags& aOther) const {
    return {static_cast<decltype(bits)>(this->bits | aOther.bits)};
  }
  FontInstanceFlags& operator|=(const FontInstanceFlags& aOther) {
    *this = (*this | aOther);
    return *this;
  }
  FontInstanceFlags operator&(const FontInstanceFlags& aOther) const {
    return {static_cast<decltype(bits)>(this->bits & aOther.bits)};
  }
  FontInstanceFlags& operator&=(const FontInstanceFlags& aOther) {
    *this = (*this & aOther);
    return *this;
  }
  FontInstanceFlags operator^(const FontInstanceFlags& aOther) const {
    return {static_cast<decltype(bits)>(this->bits ^ aOther.bits)};
  }
  FontInstanceFlags& operator^=(const FontInstanceFlags& aOther) {
    *this = (*this ^ aOther);
    return *this;
  }
  friend std::ostream& operator<<(std::ostream& aStream, const FontInstanceFlags& aInstance) {
    return aStream << "{ " << "bits=" << aInstance.bits << " }";
  }
  bool operator==(const FontInstanceFlags& aOther) const {
    return bits == aOther.bits;
  }
  static const FontInstanceFlags SYNTHETIC_BOLD;
  static const FontInstanceFlags EMBEDDED_BITMAPS;
  static const FontInstanceFlags SUBPIXEL_BGR;
  static const FontInstanceFlags TRANSPOSE;
  static const FontInstanceFlags FLIP_X;
  static const FontInstanceFlags FLIP_Y;
  static const FontInstanceFlags SUBPIXEL_POSITION;
  static const FontInstanceFlags VERTICAL;
  static const FontInstanceFlags TRANSFORM_GLYPHS;
  static const FontInstanceFlags TEXTURE_PADDING;
  static const FontInstanceFlags FORCE_GDI;
  static const FontInstanceFlags FORCE_SYMMETRIC;
  static const FontInstanceFlags NO_SYMMETRIC;
  static const FontInstanceFlags FONT_SMOOTHING;
  static const FontInstanceFlags FORCE_AUTOHINT;
  static const FontInstanceFlags NO_AUTOHINT;
  static const FontInstanceFlags VERTICAL_LAYOUT;
  static const FontInstanceFlags LCD_VERTICAL;
};
inline const FontInstanceFlags FontInstanceFlags::SYNTHETIC_BOLD = FontInstanceFlags{ /* .bits = */ (uint32_t)(1 << 1) };
inline const FontInstanceFlags FontInstanceFlags::EMBEDDED_BITMAPS = FontInstanceFlags{ /* .bits = */ (uint32_t)(1 << 2) };
inline const FontInstanceFlags FontInstanceFlags::SUBPIXEL_BGR = FontInstanceFlags{ /* .bits = */ (uint32_t)(1 << 3) };
inline const FontInstanceFlags FontInstanceFlags::TRANSPOSE = FontInstanceFlags{ /* .bits = */ (uint32_t)(1 << 4) };
inline const FontInstanceFlags FontInstanceFlags::FLIP_X = FontInstanceFlags{ /* .bits = */ (uint32_t)(1 << 5) };
inline const FontInstanceFlags FontInstanceFlags::FLIP_Y = FontInstanceFlags{ /* .bits = */ (uint32_t)(1 << 6) };
inline const FontInstanceFlags FontInstanceFlags::SUBPIXEL_POSITION = FontInstanceFlags{ /* .bits = */ (uint32_t)(1 << 7) };
inline const FontInstanceFlags FontInstanceFlags::VERTICAL = FontInstanceFlags{ /* .bits = */ (uint32_t)(1 << 8) };
inline const FontInstanceFlags FontInstanceFlags::TRANSFORM_GLYPHS = FontInstanceFlags{ /* .bits = */ (uint32_t)(1 << 12) };
inline const FontInstanceFlags FontInstanceFlags::TEXTURE_PADDING = FontInstanceFlags{ /* .bits = */ (uint32_t)(1 << 13) };
inline const FontInstanceFlags FontInstanceFlags::FORCE_GDI = FontInstanceFlags{ /* .bits = */ (uint32_t)(1 << 16) };
inline const FontInstanceFlags FontInstanceFlags::FORCE_SYMMETRIC = FontInstanceFlags{ /* .bits = */ (uint32_t)(1 << 17) };
inline const FontInstanceFlags FontInstanceFlags::NO_SYMMETRIC = FontInstanceFlags{ /* .bits = */ (uint32_t)(1 << 18) };
inline const FontInstanceFlags FontInstanceFlags::FONT_SMOOTHING = FontInstanceFlags{ /* .bits = */ (uint32_t)(1 << 16) };
inline const FontInstanceFlags FontInstanceFlags::FORCE_AUTOHINT = FontInstanceFlags{ /* .bits = */ (uint32_t)(1 << 16) };
inline const FontInstanceFlags FontInstanceFlags::NO_AUTOHINT = FontInstanceFlags{ /* .bits = */ (uint32_t)(1 << 17) };
inline const FontInstanceFlags FontInstanceFlags::VERTICAL_LAYOUT = FontInstanceFlags{ /* .bits = */ (uint32_t)(1 << 18) };
inline const FontInstanceFlags FontInstanceFlags::LCD_VERTICAL = FontInstanceFlags{ /* .bits = */ (uint32_t)(1 << 19) };

/// Represents RGBA screen colors with one byte per channel.
///
/// If the alpha value `a` is 255 the color is opaque.
struct ColorU {
  uint8_t r;
  uint8_t g;
  uint8_t b;
  uint8_t a;

  friend std::ostream& operator<<(std::ostream& aStream, const ColorU& aInstance) {
    return aStream << "{ " << "r=" << aInstance.r << ", "
                           << "g=" << aInstance.g << ", "
                           << "b=" << aInstance.b << ", "
                           << "a=" << aInstance.a << " }";
  }
  bool operator==(const ColorU& aOther) const {
    return r == aOther.r &&
           g == aOther.g &&
           b == aOther.b &&
           a == aOther.a;
  }
};

struct SyntheticItalics {
  int16_t angle;

  friend std::ostream& operator<<(std::ostream& aStream, const SyntheticItalics& aInstance) {
    return aStream << "{ " << "angle=" << aInstance.angle << " }";
  }
  bool operator==(const SyntheticItalics& aOther) const {
    return angle == aOther.angle;
  }
  static const float ANGLE_SCALE;
};
inline const float SyntheticItalics::ANGLE_SCALE = 256.0;

struct FontInstanceOptions {
  FontRenderMode render_mode;
  FontInstanceFlags flags;
  /// When bg_color.a is != 0 and render_mode is FontRenderMode::Subpixel,
  /// the text will be rendered with bg_color.r/g/b as an opaque estimated
  /// background color.
  ColorU bg_color;
  SyntheticItalics synthetic_italics;

  friend std::ostream& operator<<(std::ostream& aStream, const FontInstanceOptions& aInstance) {
    return aStream << "{ " << "render_mode=" << aInstance.render_mode << ", "
                           << "flags=" << aInstance.flags << ", "
                           << "bg_color=" << aInstance.bg_color << ", "
                           << "synthetic_italics=" << aInstance.synthetic_italics << " }";
  }
  bool operator==(const FontInstanceOptions& aOther) const {
    return render_mode == aOther.render_mode &&
           flags == aOther.flags &&
           bg_color == aOther.bg_color &&
           synthetic_italics == aOther.synthetic_italics;
  }
};

#if defined(XP_WIN)
struct FontInstancePlatformOptions {
  uint16_t gamma;
  uint8_t contrast;
  uint8_t cleartype_level;

  friend std::ostream& operator<<(std::ostream& aStream, const FontInstancePlatformOptions& aInstance) {
    return aStream << "{ " << "gamma=" << aInstance.gamma << ", "
                           << "contrast=" << aInstance.contrast << ", "
                           << "cleartype_level=" << aInstance.cleartype_level << " }";
  }
  bool operator==(const FontInstancePlatformOptions& aOther) const {
    return gamma == aOther.gamma &&
           contrast == aOther.contrast &&
           cleartype_level == aOther.cleartype_level;
  }
};
#endif

#if defined(XP_MACOSX)
struct FontInstancePlatformOptions {
  uint32_t unused;

  friend std::ostream& operator<<(std::ostream& aStream, const FontInstancePlatformOptions& aInstance) {
    return aStream << "{ " << "unused=" << aInstance.unused << " }";
  }
  bool operator==(const FontInstancePlatformOptions& aOther) const {
    return unused == aOther.unused;
  }
};
#endif

#if !(defined(XP_MACOSX) || defined(XP_WIN))
struct FontInstancePlatformOptions {
  FontLCDFilter lcd_filter;
  FontHinting hinting;

  friend std::ostream& operator<<(std::ostream& aStream, const FontInstancePlatformOptions& aInstance) {
    return aStream << "{ " << "lcd_filter=" << aInstance.lcd_filter << ", "
                           << "hinting=" << aInstance.hinting << " }";
  }
  bool operator==(const FontInstancePlatformOptions& aOther) const {
    return lcd_filter == aOther.lcd_filter &&
           hinting == aOther.hinting;
  }
};
#endif

using WrIdNamespace = IdNamespace;

struct WrSpatialId {
  uintptr_t id;

  friend std::ostream& operator<<(std::ostream& aStream, const WrSpatialId& aInstance) {
    return aStream << "{ " << "id=" << aInstance.id << " }";
  }
  bool operator==(const WrSpatialId& aOther) const {
    return id == aOther.id;
  }
};

using LayoutRect = Rect<float, LayoutPixel>;

struct WrClipId {
  uintptr_t id;

  friend std::ostream& operator<<(std::ostream& aStream, const WrClipId& aInstance) {
    return aStream << "{ " << "id=" << aInstance.id << " }";
  }
  bool operator==(const WrClipId& aOther) const {
    return id == aOther.id;
  }
};

struct WrStackingContextClip {
  enum class Tag {
    None,
    ClipId,
    ClipChain,
    /// Must be last for serialization purposes
    Sentinel,
  };

  friend std::ostream& operator<<(std::ostream& aStream, const Tag& aInstance) {
    using Tag = WrStackingContextClip::Tag;
    switch (aInstance) {
      case Tag::None: aStream << "None"; break;
      case Tag::ClipId: aStream << "ClipId"; break;
      case Tag::ClipChain: aStream << "ClipChain"; break;
      case Tag::Sentinel: aStream << "Sentinel"; break;
    }
    return aStream;
  }

  friend std::ostream& operator<<(std::ostream& aStream, const WrStackingContextClip& aInstance) {
    using Tag = WrStackingContextClip::Tag;
    switch (aInstance.tag) {
      case Tag::None: aStream << "None"; break;
      case Tag::ClipId: aStream << "ClipId" << aInstance.clip_id; break;
      case Tag::ClipChain: aStream << "ClipChain" << aInstance.clip_chain; break;
      case Tag::Sentinel: aStream << "Sentinel"; break;
    }
    return aStream;
  }

  struct ClipId_Body {
    WrClipId _0;

    friend std::ostream& operator<<(std::ostream& aStream, const ClipId_Body& aInstance) {
      return aStream << "{ " << "_0=" << aInstance._0 << " }";
    }
    bool operator==(const ClipId_Body& aOther) const {
      return _0 == aOther._0;
    }
  };

  struct ClipChain_Body {
    uint64_t _0;

    friend std::ostream& operator<<(std::ostream& aStream, const ClipChain_Body& aInstance) {
      return aStream << "{ " << "_0=" << aInstance._0 << " }";
    }
    bool operator==(const ClipChain_Body& aOther) const {
      return _0 == aOther._0;
    }
  };

  Tag tag;
  union {
    ClipId_Body clip_id;
    ClipChain_Body clip_chain;
  };

  static WrStackingContextClip None() {
    WrStackingContextClip result;
    result.tag = Tag::None;
    return result;
  }

  bool IsNone() const {
    return tag == Tag::None;
  }

  static WrStackingContextClip ClipId(const WrClipId &a0) {
    WrStackingContextClip result;
    ::new (&result.clip_id._0) (WrClipId)(a0);
    result.tag = Tag::ClipId;
    return result;
  }

  bool IsClipId() const {
    return tag == Tag::ClipId;
  }

  static WrStackingContextClip ClipChain(const uint64_t &a0) {
    WrStackingContextClip result;
    ::new (&result.clip_chain._0) (uint64_t)(a0);
    result.tag = Tag::ClipChain;
    return result;
  }

  bool IsClipChain() const {
    return tag == Tag::ClipChain;
  }

  static WrStackingContextClip Sentinel() {
    WrStackingContextClip result;
    result.tag = Tag::Sentinel;
    return result;
  }

  bool IsSentinel() const {
    return tag == Tag::Sentinel;
  }

  bool operator==(const WrStackingContextClip& aOther) const {
    if (tag != aOther.tag) {
      return false;
    }
    switch (tag) {
      case Tag::ClipId: return clip_id == aOther.clip_id;
      case Tag::ClipChain: return clip_chain == aOther.clip_chain;
      default: break;
    }
    return true;
  }
};

struct WrAnimationProperty {
  WrAnimationType effect_type;
  uint64_t id;

  friend std::ostream& operator<<(std::ostream& aStream, const WrAnimationProperty& aInstance) {
    return aStream << "{ " << "effect_type=" << aInstance.effect_type << ", "
                           << "id=" << aInstance.id << " }";
  }
  bool operator==(const WrAnimationProperty& aOther) const {
    return effect_type == aOther.effect_type &&
           id == aOther.id;
  }
};

struct WrComputedTransformData {
  LayoutSize scale_from;
  bool vertical_flip;
  WrRotation rotation;

  friend std::ostream& operator<<(std::ostream& aStream, const WrComputedTransformData& aInstance) {
    return aStream << "{ " << "scale_from=" << aInstance.scale_from << ", "
                           << "vertical_flip=" << aInstance.vertical_flip << ", "
                           << "rotation=" << aInstance.rotation << " }";
  }
  bool operator==(const WrComputedTransformData& aOther) const {
    return scale_from == aOther.scale_from &&
           vertical_flip == aOther.vertical_flip &&
           rotation == aOther.rotation;
  }
};

struct PrimitiveFlags {
  uint8_t bits;

  explicit operator bool() const {
    return !!bits;
  }
  PrimitiveFlags operator~() const {
    return {static_cast<decltype(bits)>(~bits)};
  }
  PrimitiveFlags operator|(const PrimitiveFlags& aOther) const {
    return {static_cast<decltype(bits)>(this->bits | aOther.bits)};
  }
  PrimitiveFlags& operator|=(const PrimitiveFlags& aOther) {
    *this = (*this | aOther);
    return *this;
  }
  PrimitiveFlags operator&(const PrimitiveFlags& aOther) const {
    return {static_cast<decltype(bits)>(this->bits & aOther.bits)};
  }
  PrimitiveFlags& operator&=(const PrimitiveFlags& aOther) {
    *this = (*this & aOther);
    return *this;
  }
  PrimitiveFlags operator^(const PrimitiveFlags& aOther) const {
    return {static_cast<decltype(bits)>(this->bits ^ aOther.bits)};
  }
  PrimitiveFlags& operator^=(const PrimitiveFlags& aOther) {
    *this = (*this ^ aOther);
    return *this;
  }
  friend std::ostream& operator<<(std::ostream& aStream, const PrimitiveFlags& aInstance) {
    return aStream << "{ " << "bits=" << aInstance.bits << " }";
  }
  bool operator==(const PrimitiveFlags& aOther) const {
    return bits == aOther.bits;
  }
  static const PrimitiveFlags IS_BACKFACE_VISIBLE;
  static const PrimitiveFlags IS_SCROLLBAR_CONTAINER;
  static const PrimitiveFlags IS_SCROLLBAR_THUMB;
  static const PrimitiveFlags PREFER_COMPOSITOR_SURFACE;
  static const PrimitiveFlags SUPPORTS_EXTERNAL_COMPOSITOR_SURFACE;
};
/// The CSS backface-visibility property (yes, it can be really granular)
inline const PrimitiveFlags PrimitiveFlags::IS_BACKFACE_VISIBLE = PrimitiveFlags{ /* .bits = */ (uint8_t)(1 << 0) };
/// If set, this primitive represents a scroll bar container
inline const PrimitiveFlags PrimitiveFlags::IS_SCROLLBAR_CONTAINER = PrimitiveFlags{ /* .bits = */ (uint8_t)(1 << 1) };
/// If set, this primitive represents a scroll bar thumb
inline const PrimitiveFlags PrimitiveFlags::IS_SCROLLBAR_THUMB = PrimitiveFlags{ /* .bits = */ (uint8_t)(1 << 2) };
/// This is used as a performance hint - this primitive may be promoted to a native
/// compositor surface under certain (implementation specific) conditions. This
/// is typically used for large videos, and canvas elements.
inline const PrimitiveFlags PrimitiveFlags::PREFER_COMPOSITOR_SURFACE = PrimitiveFlags{ /* .bits = */ (uint8_t)(1 << 3) };
/// If set, this primitive can be passed directly to the compositor via its
/// ExternalImageId, and the compositor will use the native image directly.
/// Used as a further extension on top of PREFER_COMPOSITOR_SURFACE.
inline const PrimitiveFlags PrimitiveFlags::SUPPORTS_EXTERNAL_COMPOSITOR_SURFACE = PrimitiveFlags{ /* .bits = */ (uint8_t)(1 << 4) };

struct StackingContextFlags {
  uint8_t bits;

  explicit operator bool() const {
    return !!bits;
  }
  StackingContextFlags operator~() const {
    return {static_cast<decltype(bits)>(~bits)};
  }
  StackingContextFlags operator|(const StackingContextFlags& aOther) const {
    return {static_cast<decltype(bits)>(this->bits | aOther.bits)};
  }
  StackingContextFlags& operator|=(const StackingContextFlags& aOther) {
    *this = (*this | aOther);
    return *this;
  }
  StackingContextFlags operator&(const StackingContextFlags& aOther) const {
    return {static_cast<decltype(bits)>(this->bits & aOther.bits)};
  }
  StackingContextFlags& operator&=(const StackingContextFlags& aOther) {
    *this = (*this & aOther);
    return *this;
  }
  StackingContextFlags operator^(const StackingContextFlags& aOther) const {
    return {static_cast<decltype(bits)>(this->bits ^ aOther.bits)};
  }
  StackingContextFlags& operator^=(const StackingContextFlags& aOther) {
    *this = (*this ^ aOther);
    return *this;
  }
  friend std::ostream& operator<<(std::ostream& aStream, const StackingContextFlags& aInstance) {
    return aStream << "{ " << "bits=" << aInstance.bits << " }";
  }
  bool operator==(const StackingContextFlags& aOther) const {
    return bits == aOther.bits;
  }
  static const StackingContextFlags IS_BACKDROP_ROOT;
  static const StackingContextFlags IS_BLEND_CONTAINER;
};
/// If true, this stacking context represents a backdrop root, per the CSS
/// filter-effects specification (see https://drafts.fxtf.org/filter-effects-2/#BackdropRoot).
inline const StackingContextFlags StackingContextFlags::IS_BACKDROP_ROOT = StackingContextFlags{ /* .bits = */ (uint8_t)(1 << 0) };
/// If true, this stacking context is a blend container than contains
/// mix-blend-mode children (and should thus be isolated).
inline const StackingContextFlags StackingContextFlags::IS_BLEND_CONTAINER = StackingContextFlags{ /* .bits = */ (uint8_t)(1 << 1) };

/// IMPORTANT: If you add fields to this struct, you need to also add initializers
/// for those fields in WebRenderAPI.h.
struct WrStackingContextParams {
  WrStackingContextClip clip;
  const WrAnimationProperty *animation;
  const float *opacity;
  const WrComputedTransformData *computed_transform;
  TransformStyle transform_style;
  WrReferenceFrameKind reference_frame_kind;
  const uint64_t *scrolling_relative_to;
  PrimitiveFlags prim_flags;
  MixBlendMode mix_blend_mode;
  StackingContextFlags flags;

  friend std::ostream& operator<<(std::ostream& aStream, const WrStackingContextParams& aInstance) {
    return aStream << "{ " << "clip=" << aInstance.clip << ", "
                           << "animation=" << aInstance.animation << ", "
                           << "opacity=" << aInstance.opacity << ", "
                           << "computed_transform=" << aInstance.computed_transform << ", "
                           << "transform_style=" << aInstance.transform_style << ", "
                           << "reference_frame_kind=" << aInstance.reference_frame_kind << ", "
                           << "scrolling_relative_to=" << aInstance.scrolling_relative_to << ", "
                           << "prim_flags=" << aInstance.prim_flags << ", "
                           << "mix_blend_mode=" << aInstance.mix_blend_mode << ", "
                           << "flags=" << aInstance.flags << " }";
  }
  bool operator==(const WrStackingContextParams& aOther) const {
    return clip == aOther.clip &&
           animation == aOther.animation &&
           opacity == aOther.opacity &&
           computed_transform == aOther.computed_transform &&
           transform_style == aOther.transform_style &&
           reference_frame_kind == aOther.reference_frame_kind &&
           scrolling_relative_to == aOther.scrolling_relative_to &&
           prim_flags == aOther.prim_flags &&
           mix_blend_mode == aOther.mix_blend_mode &&
           flags == aOther.flags;
  }
};

/// A key to identify an animated property binding.
struct PropertyBindingId {
  IdNamespace namespace_;
  uint32_t uid;

  friend std::ostream& operator<<(std::ostream& aStream, const PropertyBindingId& aInstance) {
    return aStream << "{ " << "namespace_=" << aInstance.namespace_ << ", "
                           << "uid=" << aInstance.uid << " }";
  }
  bool operator==(const PropertyBindingId& aOther) const {
    return namespace_ == aOther.namespace_ &&
           uid == aOther.uid;
  }
};

/// A unique key that is used for connecting animated property
/// values to bindings in the display list.
template<typename T>
struct PropertyBindingKey {
  ///
  PropertyBindingId id;

  friend std::ostream& operator<<(std::ostream& aStream, const PropertyBindingKey& aInstance) {
    return aStream << "{ " << "id=" << aInstance.id << " }";
  }
  bool operator==(const PropertyBindingKey& aOther) const {
    return id == aOther.id;
  }
};

/// A binding property can either be a specific value
/// (the normal, non-animated case) or point to a binding location
/// to fetch the current value from.
/// Note that Binding has also a non-animated value, the value is
/// used for the case where the animation is still in-delay phase
/// (i.e. the animation doesn't produce any animation values).
template<typename T>
struct PropertyBinding {
  enum class Tag {
    /// Non-animated value.
    Value,
    /// Animated binding.
    Binding,
    /// Must be last for serialization purposes
    Sentinel,
  };

  friend std::ostream& operator<<(std::ostream& aStream, const Tag& aInstance) {
    using Tag = PropertyBinding::Tag;
    switch (aInstance) {
      case Tag::Value: aStream << "Value"; break;
      case Tag::Binding: aStream << "Binding"; break;
      case Tag::Sentinel: aStream << "Sentinel"; break;
    }
    return aStream;
  }

  friend std::ostream& operator<<(std::ostream& aStream, const PropertyBinding& aInstance) {
    using Tag = PropertyBinding::Tag;
    switch (aInstance.tag) {
      case Tag::Value: aStream << "Value" << aInstance.value; break;
      case Tag::Binding: aStream << "Binding" << aInstance.binding; break;
      case Tag::Sentinel: aStream << "Sentinel"; break;
    }
    return aStream;
  }

  struct Value_Body {
    T _0;

    friend std::ostream& operator<<(std::ostream& aStream, const Value_Body& aInstance) {
      return aStream << "{ " << "_0=" << aInstance._0 << " }";
    }
    bool operator==(const Value_Body& aOther) const {
      return _0 == aOther._0;
    }
  };

  struct Binding_Body {
    PropertyBindingKey<T> _0;
    T _1;

    friend std::ostream& operator<<(std::ostream& aStream, const Binding_Body& aInstance) {
      return aStream << "{ " << "_0=" << aInstance._0 << ", "
                             << "_1=" << aInstance._1 << " }";
    }
    bool operator==(const Binding_Body& aOther) const {
      return _0 == aOther._0 &&
             _1 == aOther._1;
    }
  };

  Tag tag;
  union {
    Value_Body value;
    Binding_Body binding;
  };

  static PropertyBinding Value(const T &a0) {
    PropertyBinding result;
    ::new (&result.value._0) (T)(a0);
    result.tag = Tag::Value;
    return result;
  }

  bool IsValue() const {
    return tag == Tag::Value;
  }

  static PropertyBinding Binding(const PropertyBindingKey<T> &a0,
                                 const T &a1) {
    PropertyBinding result;
    ::new (&result.binding._0) (PropertyBindingKey<T>)(a0);
    ::new (&result.binding._1) (T)(a1);
    result.tag = Tag::Binding;
    return result;
  }

  bool IsBinding() const {
    return tag == Tag::Binding;
  }

  static PropertyBinding Sentinel() {
    PropertyBinding result;
    result.tag = Tag::Sentinel;
    return result;
  }

  bool IsSentinel() const {
    return tag == Tag::Sentinel;
  }

  bool operator==(const PropertyBinding& aOther) const {
    if (tag != aOther.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Value: return value == aOther.value;
      case Tag::Binding: return binding == aOther.binding;
      default: break;
    }
    return true;
  }
};

/// A 2d Vector tagged with a unit.
template<typename T, typename U>
struct Vector2D {
  /// The `x` (traditionally, horizontal) coordinate.
  T x;
  /// The `y` (traditionally, vertical) coordinate.
  T y;

  friend std::ostream& operator<<(std::ostream& aStream, const Vector2D& aInstance) {
    return aStream << "{ " << "x=" << aInstance.x << ", "
                           << "y=" << aInstance.y << " }";
  }
  bool operator==(const Vector2D& aOther) const {
    return x == aOther.x &&
           y == aOther.y;
  }
};

using LayoutVector2D = Vector2D<float, LayoutPixel>;

struct Shadow {
  LayoutVector2D offset;
  ColorF color;
  float blur_radius;

  friend std::ostream& operator<<(std::ostream& aStream, const Shadow& aInstance) {
    return aStream << "{ " << "offset=" << aInstance.offset << ", "
                           << "color=" << aInstance.color << ", "
                           << "blur_radius=" << aInstance.blur_radius << " }";
  }
  bool operator==(const Shadow& aOther) const {
    return offset == aOther.offset &&
           color == aOther.color &&
           blur_radius == aOther.blur_radius;
  }
};

/// CSS filter.
struct FilterOp {
  enum class Tag {
    /// Filter that does no transformation of the colors, needed for
    /// debug purposes only.
    Identity,
    Blur,
    Brightness,
    Contrast,
    Grayscale,
    HueRotate,
    Invert,
    Opacity,
    Saturate,
    Sepia,
    DropShadow,
    ColorMatrix,
    SrgbToLinear,
    LinearToSrgb,
    ComponentTransfer,
    Flood,
    /// Must be last for serialization purposes
    Sentinel,
  };

  friend std::ostream& operator<<(std::ostream& aStream, const Tag& aInstance) {
    using Tag = FilterOp::Tag;
    switch (aInstance) {
      case Tag::Identity: aStream << "Identity"; break;
      case Tag::Blur: aStream << "Blur"; break;
      case Tag::Brightness: aStream << "Brightness"; break;
      case Tag::Contrast: aStream << "Contrast"; break;
      case Tag::Grayscale: aStream << "Grayscale"; break;
      case Tag::HueRotate: aStream << "HueRotate"; break;
      case Tag::Invert: aStream << "Invert"; break;
      case Tag::Opacity: aStream << "Opacity"; break;
      case Tag::Saturate: aStream << "Saturate"; break;
      case Tag::Sepia: aStream << "Sepia"; break;
      case Tag::DropShadow: aStream << "DropShadow"; break;
      case Tag::ColorMatrix: aStream << "ColorMatrix"; break;
      case Tag::SrgbToLinear: aStream << "SrgbToLinear"; break;
      case Tag::LinearToSrgb: aStream << "LinearToSrgb"; break;
      case Tag::ComponentTransfer: aStream << "ComponentTransfer"; break;
      case Tag::Flood: aStream << "Flood"; break;
      case Tag::Sentinel: aStream << "Sentinel"; break;
    }
    return aStream;
  }

  friend std::ostream& operator<<(std::ostream& aStream, const FilterOp& aInstance) {
    using Tag = FilterOp::Tag;
    switch (aInstance.tag) {
      case Tag::Identity: aStream << "Identity"; break;
      case Tag::Blur: aStream << "Blur" << aInstance.blur; break;
      case Tag::Brightness: aStream << "Brightness" << aInstance.brightness; break;
      case Tag::Contrast: aStream << "Contrast" << aInstance.contrast; break;
      case Tag::Grayscale: aStream << "Grayscale" << aInstance.grayscale; break;
      case Tag::HueRotate: aStream << "HueRotate" << aInstance.hue_rotate; break;
      case Tag::Invert: aStream << "Invert" << aInstance.invert; break;
      case Tag::Opacity: aStream << "Opacity" << aInstance.opacity; break;
      case Tag::Saturate: aStream << "Saturate" << aInstance.saturate; break;
      case Tag::Sepia: aStream << "Sepia" << aInstance.sepia; break;
      case Tag::DropShadow: aStream << "DropShadow" << aInstance.drop_shadow; break;
      case Tag::ColorMatrix: aStream << "ColorMatrix" << aInstance.color_matrix; break;
      case Tag::SrgbToLinear: aStream << "SrgbToLinear"; break;
      case Tag::LinearToSrgb: aStream << "LinearToSrgb"; break;
      case Tag::ComponentTransfer: aStream << "ComponentTransfer"; break;
      case Tag::Flood: aStream << "Flood" << aInstance.flood; break;
      case Tag::Sentinel: aStream << "Sentinel"; break;
    }
    return aStream;
  }

  struct Blur_Body {
    float _0;
    float _1;

    friend std::ostream& operator<<(std::ostream& aStream, const Blur_Body& aInstance) {
      return aStream << "{ " << "_0=" << aInstance._0 << ", "
                             << "_1=" << aInstance._1 << " }";
    }
    bool operator==(const Blur_Body& aOther) const {
      return _0 == aOther._0 &&
             _1 == aOther._1;
    }
  };

  struct Brightness_Body {
    float _0;

    friend std::ostream& operator<<(std::ostream& aStream, const Brightness_Body& aInstance) {
      return aStream << "{ " << "_0=" << aInstance._0 << " }";
    }
    bool operator==(const Brightness_Body& aOther) const {
      return _0 == aOther._0;
    }
  };

  struct Contrast_Body {
    float _0;

    friend std::ostream& operator<<(std::ostream& aStream, const Contrast_Body& aInstance) {
      return aStream << "{ " << "_0=" << aInstance._0 << " }";
    }
    bool operator==(const Contrast_Body& aOther) const {
      return _0 == aOther._0;
    }
  };

  struct Grayscale_Body {
    float _0;

    friend std::ostream& operator<<(std::ostream& aStream, const Grayscale_Body& aInstance) {
      return aStream << "{ " << "_0=" << aInstance._0 << " }";
    }
    bool operator==(const Grayscale_Body& aOther) const {
      return _0 == aOther._0;
    }
  };

  struct HueRotate_Body {
    float _0;

    friend std::ostream& operator<<(std::ostream& aStream, const HueRotate_Body& aInstance) {
      return aStream << "{ " << "_0=" << aInstance._0 << " }";
    }
    bool operator==(const HueRotate_Body& aOther) const {
      return _0 == aOther._0;
    }
  };

  struct Invert_Body {
    float _0;

    friend std::ostream& operator<<(std::ostream& aStream, const Invert_Body& aInstance) {
      return aStream << "{ " << "_0=" << aInstance._0 << " }";
    }
    bool operator==(const Invert_Body& aOther) const {
      return _0 == aOther._0;
    }
  };

  struct Opacity_Body {
    PropertyBinding<float> _0;
    float _1;

    friend std::ostream& operator<<(std::ostream& aStream, const Opacity_Body& aInstance) {
      return aStream << "{ " << "_0=" << aInstance._0 << ", "
                             << "_1=" << aInstance._1 << " }";
    }
    bool operator==(const Opacity_Body& aOther) const {
      return _0 == aOther._0 &&
             _1 == aOther._1;
    }
  };

  struct Saturate_Body {
    float _0;

    friend std::ostream& operator<<(std::ostream& aStream, const Saturate_Body& aInstance) {
      return aStream << "{ " << "_0=" << aInstance._0 << " }";
    }
    bool operator==(const Saturate_Body& aOther) const {
      return _0 == aOther._0;
    }
  };

  struct Sepia_Body {
    float _0;

    friend std::ostream& operator<<(std::ostream& aStream, const Sepia_Body& aInstance) {
      return aStream << "{ " << "_0=" << aInstance._0 << " }";
    }
    bool operator==(const Sepia_Body& aOther) const {
      return _0 == aOther._0;
    }
  };

  struct DropShadow_Body {
    Shadow _0;

    friend std::ostream& operator<<(std::ostream& aStream, const DropShadow_Body& aInstance) {
      return aStream << "{ " << "_0=" << aInstance._0 << " }";
    }
    bool operator==(const DropShadow_Body& aOther) const {
      return _0 == aOther._0;
    }
  };

  struct ColorMatrix_Body {
    float _0[20];

    friend std::ostream& operator<<(std::ostream& aStream, const ColorMatrix_Body& aInstance) {
      return aStream << "{ " << "_0=" << aInstance._0 << " }";
    }
  };

  struct Flood_Body {
    ColorF _0;

    friend std::ostream& operator<<(std::ostream& aStream, const Flood_Body& aInstance) {
      return aStream << "{ " << "_0=" << aInstance._0 << " }";
    }
    bool operator==(const Flood_Body& aOther) const {
      return _0 == aOther._0;
    }
  };

  Tag tag;
  union {
    Blur_Body blur;
    Brightness_Body brightness;
    Contrast_Body contrast;
    Grayscale_Body grayscale;
    HueRotate_Body hue_rotate;
    Invert_Body invert;
    Opacity_Body opacity;
    Saturate_Body saturate;
    Sepia_Body sepia;
    DropShadow_Body drop_shadow;
    ColorMatrix_Body color_matrix;
    Flood_Body flood;
  };

  static FilterOp Identity() {
    FilterOp result;
    result.tag = Tag::Identity;
    return result;
  }

  bool IsIdentity() const {
    return tag == Tag::Identity;
  }

  static FilterOp Blur(const float &a0,
                       const float &a1) {
    FilterOp result;
    ::new (&result.blur._0) (float)(a0);
    ::new (&result.blur._1) (float)(a1);
    result.tag = Tag::Blur;
    return result;
  }

  bool IsBlur() const {
    return tag == Tag::Blur;
  }

  static FilterOp Brightness(const float &a0) {
    FilterOp result;
    ::new (&result.brightness._0) (float)(a0);
    result.tag = Tag::Brightness;
    return result;
  }

  bool IsBrightness() const {
    return tag == Tag::Brightness;
  }

  static FilterOp Contrast(const float &a0) {
    FilterOp result;
    ::new (&result.contrast._0) (float)(a0);
    result.tag = Tag::Contrast;
    return result;
  }

  bool IsContrast() const {
    return tag == Tag::Contrast;
  }

  static FilterOp Grayscale(const float &a0) {
    FilterOp result;
    ::new (&result.grayscale._0) (float)(a0);
    result.tag = Tag::Grayscale;
    return result;
  }

  bool IsGrayscale() const {
    return tag == Tag::Grayscale;
  }

  static FilterOp HueRotate(const float &a0) {
    FilterOp result;
    ::new (&result.hue_rotate._0) (float)(a0);
    result.tag = Tag::HueRotate;
    return result;
  }

  bool IsHueRotate() const {
    return tag == Tag::HueRotate;
  }

  static FilterOp Invert(const float &a0) {
    FilterOp result;
    ::new (&result.invert._0) (float)(a0);
    result.tag = Tag::Invert;
    return result;
  }

  bool IsInvert() const {
    return tag == Tag::Invert;
  }

  static FilterOp Opacity(const PropertyBinding<float> &a0,
                          const float &a1) {
    FilterOp result;
    ::new (&result.opacity._0) (PropertyBinding<float>)(a0);
    ::new (&result.opacity._1) (float)(a1);
    result.tag = Tag::Opacity;
    return result;
  }

  bool IsOpacity() const {
    return tag == Tag::Opacity;
  }

  static FilterOp Saturate(const float &a0) {
    FilterOp result;
    ::new (&result.saturate._0) (float)(a0);
    result.tag = Tag::Saturate;
    return result;
  }

  bool IsSaturate() const {
    return tag == Tag::Saturate;
  }

  static FilterOp Sepia(const float &a0) {
    FilterOp result;
    ::new (&result.sepia._0) (float)(a0);
    result.tag = Tag::Sepia;
    return result;
  }

  bool IsSepia() const {
    return tag == Tag::Sepia;
  }

  static FilterOp DropShadow(const Shadow &a0) {
    FilterOp result;
    ::new (&result.drop_shadow._0) (Shadow)(a0);
    result.tag = Tag::DropShadow;
    return result;
  }

  bool IsDropShadow() const {
    return tag == Tag::DropShadow;
  }

  static FilterOp ColorMatrix(const float (&a0)[20]) {
    FilterOp result;
    for (int i = 0; i < 20; i++) {
      ::new (&result.color_matrix._0[i]) (float)(a0[i]);
    }
    result.tag = Tag::ColorMatrix;
    return result;
  }

  bool IsColorMatrix() const {
    return tag == Tag::ColorMatrix;
  }

  static FilterOp SrgbToLinear() {
    FilterOp result;
    result.tag = Tag::SrgbToLinear;
    return result;
  }

  bool IsSrgbToLinear() const {
    return tag == Tag::SrgbToLinear;
  }

  static FilterOp LinearToSrgb() {
    FilterOp result;
    result.tag = Tag::LinearToSrgb;
    return result;
  }

  bool IsLinearToSrgb() const {
    return tag == Tag::LinearToSrgb;
  }

  static FilterOp ComponentTransfer() {
    FilterOp result;
    result.tag = Tag::ComponentTransfer;
    return result;
  }

  bool IsComponentTransfer() const {
    return tag == Tag::ComponentTransfer;
  }

  static FilterOp Flood(const ColorF &a0) {
    FilterOp result;
    ::new (&result.flood._0) (ColorF)(a0);
    result.tag = Tag::Flood;
    return result;
  }

  bool IsFlood() const {
    return tag == Tag::Flood;
  }

  static FilterOp Sentinel() {
    FilterOp result;
    result.tag = Tag::Sentinel;
    return result;
  }

  bool IsSentinel() const {
    return tag == Tag::Sentinel;
  }
};

struct WrFilterData {
  ComponentTransferFuncType funcR_type;
  float *R_values;
  uintptr_t R_values_count;
  ComponentTransferFuncType funcG_type;
  float *G_values;
  uintptr_t G_values_count;
  ComponentTransferFuncType funcB_type;
  float *B_values;
  uintptr_t B_values_count;
  ComponentTransferFuncType funcA_type;
  float *A_values;
  uintptr_t A_values_count;

  friend std::ostream& operator<<(std::ostream& aStream, const WrFilterData& aInstance) {
    return aStream << "{ " << "funcR_type=" << aInstance.funcR_type << ", "
                           << "R_values=" << aInstance.R_values << ", "
                           << "R_values_count=" << aInstance.R_values_count << ", "
                           << "funcG_type=" << aInstance.funcG_type << ", "
                           << "G_values=" << aInstance.G_values << ", "
                           << "G_values_count=" << aInstance.G_values_count << ", "
                           << "funcB_type=" << aInstance.funcB_type << ", "
                           << "B_values=" << aInstance.B_values << ", "
                           << "B_values_count=" << aInstance.B_values_count << ", "
                           << "funcA_type=" << aInstance.funcA_type << ", "
                           << "A_values=" << aInstance.A_values << ", "
                           << "A_values_count=" << aInstance.A_values_count << " }";
  }
  bool operator==(const WrFilterData& aOther) const {
    return funcR_type == aOther.funcR_type &&
           R_values == aOther.R_values &&
           R_values_count == aOther.R_values_count &&
           funcG_type == aOther.funcG_type &&
           G_values == aOther.G_values &&
           G_values_count == aOther.G_values_count &&
           funcB_type == aOther.funcB_type &&
           B_values == aOther.B_values &&
           B_values_count == aOther.B_values_count &&
           funcA_type == aOther.funcA_type &&
           A_values == aOther.A_values &&
           A_values_count == aOther.A_values_count;
  }
};

/// Configure whether the contents of a stacking context
/// should be rasterized in local space or screen space.
/// Local space rasterized pictures are typically used
/// when we want to cache the output, and performance is
/// important. Note that this is a performance hint only,
/// which WR may choose to ignore.
union RasterSpace {
  enum class Tag : uint8_t {
    Local,
    Screen,
    /// Must be last for serialization purposes
    Sentinel,
  };

  friend std::ostream& operator<<(std::ostream& aStream, const Tag& aInstance) {
    using Tag = RasterSpace::Tag;
    switch (aInstance) {
      case Tag::Local: aStream << "Local"; break;
      case Tag::Screen: aStream << "Screen"; break;
      case Tag::Sentinel: aStream << "Sentinel"; break;
    }
    return aStream;
  }

  friend std::ostream& operator<<(std::ostream& aStream, const RasterSpace& aInstance) {
    using Tag = RasterSpace::Tag;
    switch (aInstance.tag) {
      case Tag::Local: aStream << aInstance.local; break;
      case Tag::Screen: aStream << "Screen"; break;
      case Tag::Sentinel: aStream << "Sentinel"; break;
    }
    return aStream;
  }

  struct Local_Body {
    Tag tag;
    float _0;

    friend std::ostream& operator<<(std::ostream& aStream, const Local_Body& aInstance) {
      return aStream << "{ " << "tag=" << aInstance.tag << ", "
                             << "_0=" << aInstance._0 << " }";
    }
    bool operator==(const Local_Body& aOther) const {
      return _0 == aOther._0;
    }
  };

  struct {
    Tag tag;
  };
  Local_Body local;

  static RasterSpace Local(const float &a0) {
    RasterSpace result;
    ::new (&result.local._0) (float)(a0);
    result.tag = Tag::Local;
    return result;
  }

  bool IsLocal() const {
    return tag == Tag::Local;
  }

  static RasterSpace Screen() {
    RasterSpace result;
    result.tag = Tag::Screen;
    return result;
  }

  bool IsScreen() const {
    return tag == Tag::Screen;
  }

  static RasterSpace Sentinel() {
    RasterSpace result;
    result.tag = Tag::Sentinel;
    return result;
  }

  bool IsSentinel() const {
    return tag == Tag::Sentinel;
  }

  bool operator==(const RasterSpace& aOther) const {
    if (tag != aOther.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Local: return local == aOther.local;
      default: break;
    }
    return true;
  }
};

struct WrSpaceAndClip {
  WrSpatialId space;
  WrClipId clip;

  friend std::ostream& operator<<(std::ostream& aStream, const WrSpaceAndClip& aInstance) {
    return aStream << "{ " << "space=" << aInstance.space << ", "
                           << "clip=" << aInstance.clip << " }";
  }
  bool operator==(const WrSpaceAndClip& aOther) const {
    return space == aOther.space &&
           clip == aOther.clip;
  }
};

struct BorderRadius {
  LayoutSize top_left;
  LayoutSize top_right;
  LayoutSize bottom_left;
  LayoutSize bottom_right;

  friend std::ostream& operator<<(std::ostream& aStream, const BorderRadius& aInstance) {
    return aStream << "{ " << "top_left=" << aInstance.top_left << ", "
                           << "top_right=" << aInstance.top_right << ", "
                           << "bottom_left=" << aInstance.bottom_left << ", "
                           << "bottom_right=" << aInstance.bottom_right << " }";
  }
  bool operator==(const BorderRadius& aOther) const {
    return top_left == aOther.top_left &&
           top_right == aOther.top_right &&
           bottom_left == aOther.bottom_left &&
           bottom_right == aOther.bottom_right;
  }
};

struct ComplexClipRegion {
  /// The boundaries of the rectangle.
  LayoutRect rect;
  /// Border radii of this rectangle.
  BorderRadius radii;
  /// Whether we are clipping inside or outside
  /// the region.
  ClipMode mode;

  friend std::ostream& operator<<(std::ostream& aStream, const ComplexClipRegion& aInstance) {
    return aStream << "{ " << "rect=" << aInstance.rect << ", "
                           << "radii=" << aInstance.radii << ", "
                           << "mode=" << aInstance.mode << " }";
  }
  bool operator==(const ComplexClipRegion& aOther) const {
    return rect == aOther.rect &&
           radii == aOther.radii &&
           mode == aOther.mode;
  }
};

struct WrSpaceAndClipChain {
  WrSpatialId space;
  uint64_t clip_chain;

  friend std::ostream& operator<<(std::ostream& aStream, const WrSpaceAndClipChain& aInstance) {
    return aStream << "{ " << "space=" << aInstance.space << ", "
                           << "clip_chain=" << aInstance.clip_chain << " }";
  }
  bool operator==(const WrSpaceAndClipChain& aOther) const {
    return space == aOther.space &&
           clip_chain == aOther.clip_chain;
  }
};

struct ImageMask {
  ImageKey image;
  LayoutRect rect;
  bool repeat;

  friend std::ostream& operator<<(std::ostream& aStream, const ImageMask& aInstance) {
    return aStream << "{ " << "image=" << aInstance.image << ", "
                           << "rect=" << aInstance.rect << ", "
                           << "repeat=" << aInstance.repeat << " }";
  }
  bool operator==(const ImageMask& aOther) const {
    return image == aOther.image &&
           rect == aOther.rect &&
           repeat == aOther.repeat;
  }
};

/// The minimum and maximum allowable offset for a sticky frame in a single dimension.
struct StickyOffsetBounds {
  /// The minimum offset for this frame, typically a negative value, which specifies how
  /// far in the negative direction the sticky frame can offset its contents in this
  /// dimension.
  float min;
  /// The maximum offset for this frame, typically a positive value, which specifies how
  /// far in the positive direction the sticky frame can offset its contents in this
  /// dimension.
  float max;

  friend std::ostream& operator<<(std::ostream& aStream, const StickyOffsetBounds& aInstance) {
    return aStream << "{ " << "min=" << aInstance.min << ", "
                           << "max=" << aInstance.max << " }";
  }
  bool operator==(const StickyOffsetBounds& aOther) const {
    return min == aOther.min &&
           max == aOther.max;
  }
};

using WrColorDepth = ColorDepth;

using WrYuvColorSpace = YuvColorSpace;

using WrColorRange = ColorRange;

using GlyphIndex = uint32_t;

struct GlyphInstance {
  GlyphIndex index;
  LayoutPoint point;

  friend std::ostream& operator<<(std::ostream& aStream, const GlyphInstance& aInstance) {
    return aStream << "{ " << "index=" << aInstance.index << ", "
                           << "point=" << aInstance.point << " }";
  }
  bool operator==(const GlyphInstance& aOther) const {
    return index == aOther.index &&
           point == aOther.point;
  }
};

struct GlyphOptions {
  FontRenderMode render_mode;
  FontInstanceFlags flags;

  friend std::ostream& operator<<(std::ostream& aStream, const GlyphOptions& aInstance) {
    return aStream << "{ " << "render_mode=" << aInstance.render_mode << ", "
                           << "flags=" << aInstance.flags << " }";
  }
  bool operator==(const GlyphOptions& aOther) const {
    return render_mode == aOther.render_mode &&
           flags == aOther.flags;
  }
};

/// A group of 2D side offsets, which correspond to top/right/bottom/left for borders, padding,
/// and margins in CSS, optionally tagged with a unit.
template<typename T, typename U>
struct SideOffsets2D {
  T top;
  T right;
  T bottom;
  T left;

  friend std::ostream& operator<<(std::ostream& aStream, const SideOffsets2D& aInstance) {
    return aStream << "{ " << "top=" << aInstance.top << ", "
                           << "right=" << aInstance.right << ", "
                           << "bottom=" << aInstance.bottom << ", "
                           << "left=" << aInstance.left << " }";
  }
  bool operator==(const SideOffsets2D& aOther) const {
    return top == aOther.top &&
           right == aOther.right &&
           bottom == aOther.bottom &&
           left == aOther.left;
  }
};

using LayoutSideOffsets = SideOffsets2D<float, LayoutPixel>;

struct BorderSide {
  ColorF color;
  BorderStyle style;

  friend std::ostream& operator<<(std::ostream& aStream, const BorderSide& aInstance) {
    return aStream << "{ " << "color=" << aInstance.color << ", "
                           << "style=" << aInstance.style << " }";
  }
  bool operator==(const BorderSide& aOther) const {
    return color == aOther.color &&
           style == aOther.style;
  }
};

using DeviceIntSideOffsets = SideOffsets2D<int32_t, DevicePixel>;

struct WrBorderImage {
  LayoutSideOffsets widths;
  WrImageKey image;
  int32_t width;
  int32_t height;
  bool fill;
  DeviceIntSideOffsets slice;
  LayoutSideOffsets outset;
  RepeatMode repeat_horizontal;
  RepeatMode repeat_vertical;

  friend std::ostream& operator<<(std::ostream& aStream, const WrBorderImage& aInstance) {
    return aStream << "{ " << "widths=" << aInstance.widths << ", "
                           << "image=" << aInstance.image << ", "
                           << "width=" << aInstance.width << ", "
                           << "height=" << aInstance.height << ", "
                           << "fill=" << aInstance.fill << ", "
                           << "slice=" << aInstance.slice << ", "
                           << "outset=" << aInstance.outset << ", "
                           << "repeat_horizontal=" << aInstance.repeat_horizontal << ", "
                           << "repeat_vertical=" << aInstance.repeat_vertical << " }";
  }
  bool operator==(const WrBorderImage& aOther) const {
    return widths == aOther.widths &&
           image == aOther.image &&
           width == aOther.width &&
           height == aOther.height &&
           fill == aOther.fill &&
           slice == aOther.slice &&
           outset == aOther.outset &&
           repeat_horizontal == aOther.repeat_horizontal &&
           repeat_vertical == aOther.repeat_vertical;
  }
};

struct GradientStop {
  float offset;
  ColorF color;

  friend std::ostream& operator<<(std::ostream& aStream, const GradientStop& aInstance) {
    return aStream << "{ " << "offset=" << aInstance.offset << ", "
                           << "color=" << aInstance.color << " }";
  }
  bool operator==(const GradientStop& aOther) const {
    return offset == aOther.offset &&
           color == aOther.color;
  }
};

/// An identifier used to refer to previously sent display items. Currently it
/// refers to individual display items, but this may change later.
using ItemKey = uint16_t;

using WorldPoint = Point2D<float, WorldPixel>;

struct HitResult {
  WrPipelineId pipeline_id;
  uint64_t scroll_id;
  uint16_t hit_info;

  friend std::ostream& operator<<(std::ostream& aStream, const HitResult& aInstance) {
    return aStream << "{ " << "pipeline_id=" << aInstance.pipeline_id << ", "
                           << "scroll_id=" << aInstance.scroll_id << ", "
                           << "hit_info=" << aInstance.hit_info << " }";
  }
  bool operator==(const HitResult& aOther) const {
    return pipeline_id == aOther.pipeline_id &&
           scroll_id == aOther.scroll_id &&
           hit_info == aOther.hit_info;
  }
};

using VecU8 = Vec<uint8_t>;

using ArcVecU8 = Arc<VecU8>;

using TileOffset = Point2D<int32_t, TileCoordinate>;

struct MutByteSlice {
  uint8_t *buffer;
  uintptr_t len;

  friend std::ostream& operator<<(std::ostream& aStream, const MutByteSlice& aInstance) {
    return aStream << "{ " << "buffer=" << aInstance.buffer << ", "
                           << "len=" << aInstance.len << " }";
  }
  bool operator==(const MutByteSlice& aOther) const {
    return buffer == aOther.buffer &&
           len == aOther.len;
  }
};

struct FontVariation {
  uint32_t tag;
  float value;

  friend std::ostream& operator<<(std::ostream& aStream, const FontVariation& aInstance) {
    return aStream << "{ " << "tag=" << aInstance.tag << ", "
                           << "value=" << aInstance.value << " }";
  }
  bool operator==(const FontVariation& aOther) const {
    return tag == aOther.tag &&
           value == aOther.value;
  }
};



































































































extern "C" {

#if defined(ANDROID)
extern int __android_log_write(int aPrio,
                               const char *aTag,
                               const char *aText);
#endif

void wr_vec_u8_push_bytes(WrVecU8 *aV,
                          ByteSlice aBytes);

void wr_vec_u8_reserve(WrVecU8 *aV,
                       uintptr_t aLen);

void wr_vec_u8_free(WrVecU8 aV);

extern WrExternalImage wr_renderer_lock_external_image(void *aRenderer,
                                                       ExternalImageId aExternalImageId,
                                                       uint8_t aChannelIndex,
                                                       ImageRendering aRendering);

extern void wr_renderer_unlock_external_image(void *aRenderer,
                                              ExternalImageId aExternalImageId,
                                              uint8_t aChannelIndex);

extern bool is_in_compositor_thread();

extern bool is_in_render_thread();

extern bool is_in_main_thread();

extern bool is_glcontext_gles(void *aGlcontextPtr);

extern bool is_glcontext_angle(void *aGlcontextPtr);

extern const char *gfx_wr_resource_path_override();

extern bool gfx_wr_use_optimized_shaders();

extern void gfx_critical_error(const char *aMsg);

extern void gfx_critical_note(const char *aMsg);

extern void record_telemetry_time(TelemetryProbe aProbe,
                                  uint64_t aTimeNs);

extern void gfx_wr_set_crash_annotation(CrashAnnotation aAnnotation,
                                        const char *aValue);

extern void gfx_wr_clear_crash_annotation(CrashAnnotation aAnnotation);

extern void wr_notifier_wake_up(WrWindowId aWindowId,
                                bool aCompositeNeeded);

extern void wr_notifier_new_frame_ready(WrWindowId aWindowId);

extern void wr_notifier_nop_frame_done(WrWindowId aWindowId);

extern void wr_notifier_external_event(WrWindowId aWindowId,
                                       uintptr_t aRawEvent);

extern void wr_schedule_render(WrWindowId aWindowId);

extern void wr_finished_scene_build(WrWindowId aWindowId,
                                    WrPipelineInfo *aPipelineInfo);

extern void wr_transaction_notification_notified(uintptr_t aHandler,
                                                 Checkpoint aWhen);

void wr_renderer_set_clear_color(Renderer *aRenderer,
                                 ColorF aColor);

void wr_renderer_set_external_image_handler(Renderer *aRenderer,
                                            WrExternalImageHandler *aExternalImageHandler);

void wr_renderer_update(Renderer *aRenderer);

bool wr_renderer_render(Renderer *aRenderer,
                        int32_t aWidth,
                        int32_t aHeight,
                        uintptr_t aBufferAge,
                        RendererStats *aOutStats,
                        nsTArray<DeviceIntRect> *aOutDirtyRects);

void wr_renderer_force_redraw(Renderer *aRenderer);

bool wr_renderer_record_frame(Renderer *aRenderer,
                              ImageFormat aImageFormat,
                              RecordedFrameHandle *aOutHandle,
                              int32_t *aOutWidth,
                              int32_t *aOutHeight);

bool wr_renderer_map_recorded_frame(Renderer *aRenderer,
                                    RecordedFrameHandle aHandle,
                                    uint8_t *aDstBuffer,
                                    uintptr_t aDstBufferLen,
                                    uintptr_t aDstStride);

void wr_renderer_release_composition_recorder_structures(Renderer *aRenderer);

AsyncScreenshotHandle wr_renderer_get_screenshot_async(Renderer *aRenderer,
                                                       int32_t aWindowX,
                                                       int32_t aWindowY,
                                                       int32_t aWindowWidth,
                                                       int32_t aWindowHeight,
                                                       int32_t aBufferWidth,
                                                       int32_t aBufferHeight,
                                                       ImageFormat aImageFormat,
                                                       int32_t *aScreenshotWidth,
                                                       int32_t *aScreenshotHeight);

bool wr_renderer_map_and_recycle_screenshot(Renderer *aRenderer,
                                            AsyncScreenshotHandle aHandle,
                                            uint8_t *aDstBuffer,
                                            uintptr_t aDstBufferLen,
                                            uintptr_t aDstStride);

void wr_renderer_release_profiler_structures(Renderer *aRenderer);

void wr_renderer_readback(Renderer *aRenderer,
                          int32_t aWidth,
                          int32_t aHeight,
                          ImageFormat aFormat,
                          uint8_t *aDstBuffer,
                          uintptr_t aBufferSize);

void wr_renderer_set_profiler_ui(Renderer *aRenderer,
                                 const uint8_t *aUiStr,
                                 uintptr_t aUiStrLen);

void wr_renderer_delete(Renderer *aRenderer);

void wr_renderer_accumulate_memory_report(Renderer *aRenderer,
                                          MemoryReport *aReport);

void wr_renderer_flush_pipeline_info(Renderer *aRenderer,
                                     WrPipelineInfo *aOut);

extern void gecko_profiler_start_marker(const char *aName);

extern void gecko_profiler_end_marker(const char *aName);

extern void gecko_profiler_event_marker(const char *aName);

extern void gecko_profiler_add_text_marker(const char *aName,
                                           const char *aTextBytes,
                                           uintptr_t aTextLen,
                                           uint64_t aMicroseconds);

extern bool gecko_profiler_thread_is_being_profiled();

extern void apz_register_updater(WrWindowId aWindowId);

extern void apz_pre_scene_swap(WrWindowId aWindowId);

extern void apz_post_scene_swap(WrWindowId aWindowId,
                                const WrPipelineInfo *aPipelineInfo);

extern void apz_run_updater(WrWindowId aWindowId);

extern void apz_deregister_updater(WrWindowId aWindowId);

extern void apz_register_sampler(WrWindowId aWindowId);

extern void apz_sample_transforms(WrWindowId aWindowId,
                                  const uint64_t *aGeneratedFrameId,
                                  Transaction *aTransaction);

extern void apz_deregister_sampler(WrWindowId aWindowId);

extern void omta_register_sampler(WrWindowId aWindowId);

extern void omta_sample(WrWindowId aWindowId,
                        Transaction *aTransaction);

extern void omta_deregister_sampler(WrWindowId aWindowId);

extern void wr_register_thread_local_arena();

WrThreadPool *wr_thread_pool_new(bool aLowPriority);

void wr_thread_pool_delete(WrThreadPool *aThreadPool);

WrProgramCache *wr_program_cache_new(const nsAString *aProfPath,
                                     WrThreadPool *aThreadPool);

void wr_program_cache_delete(WrProgramCache *aProgramCache);

void wr_try_load_startup_shaders_from_disk(WrProgramCache *aProgramCache);

bool remove_program_binary_disk_cache(const nsAString *aProfPath);

extern void wr_compositor_create_surface(void *aCompositor,
                                         NativeSurfaceId aId,
                                         DeviceIntPoint aVirtualOffset,
                                         DeviceIntSize aTileSize,
                                         bool aIsOpaque);

extern void wr_compositor_create_external_surface(void *aCompositor,
                                                  NativeSurfaceId aId,
                                                  bool aIsOpaque);

extern void wr_compositor_destroy_surface(void *aCompositor,
                                          NativeSurfaceId aId);

extern void wr_compositor_create_tile(void *aCompositor,
                                      NativeSurfaceId aId,
                                      int32_t aX,
                                      int32_t aY);

extern void wr_compositor_destroy_tile(void *aCompositor,
                                       NativeSurfaceId aId,
                                       int32_t aX,
                                       int32_t aY);

extern void wr_compositor_attach_external_image(void *aCompositor,
                                                NativeSurfaceId aId,
                                                ExternalImageId aExternalImage);

extern void wr_compositor_bind(void *aCompositor,
                               NativeTileId aId,
                               DeviceIntPoint *aOffset,
                               uint32_t *aFboId,
                               DeviceIntRect aDirtyRect,
                               DeviceIntRect aValidRect);

extern void wr_compositor_unbind(void *aCompositor);

extern void wr_compositor_begin_frame(void *aCompositor);

extern void wr_compositor_add_surface(void *aCompositor,
                                      NativeSurfaceId aId,
                                      const CompositorSurfaceTransform *aTransform,
                                      DeviceIntRect aClipRect,
                                      ImageRendering aImageRendering);

extern void wr_compositor_start_compositing(void *aCompositor,
                                            const DeviceIntRect *aDirtyRects,
                                            uintptr_t aNumDirtyRects,
                                            const DeviceIntRect *aOpaqueRects,
                                            uintptr_t aNumOpaqueRects);

extern void wr_compositor_end_frame(void *aCompositor);

extern void wr_compositor_enable_native_compositor(void *aCompositor,
                                                   bool aEnable);

extern void wr_compositor_deinit(void *aCompositor);

extern void wr_compositor_get_capabilities(void *aCompositor,
                                           CompositorCapabilities *aCaps);

extern void wr_compositor_map_tile(void *aCompositor,
                                   NativeTileId aId,
                                   DeviceIntRect aDirtyRect,
                                   DeviceIntRect aValidRect,
                                   void **aData,
                                   int32_t *aStride);

extern void wr_compositor_unmap_tile(void *aCompositor);

extern void wr_partial_present_compositor_set_buffer_damage_region(void *aCompositor,
                                                                   const DeviceIntRect *aRects,
                                                                   uintptr_t aNRects);

extern bool wr_swgl_lock_composite_surface(void *aCtx,
                                           ExternalImageId aExternalImageId,
                                           SWGLCompositeSurfaceInfo *aCompositeInfo);

extern void wr_swgl_unlock_composite_surface(void *aCtx,
                                             ExternalImageId aExternalImageId);

bool wr_window_new(WrWindowId aWindowId,
                   int32_t aWindowWidth,
                   int32_t aWindowHeight,
                   bool aIsMainWindow,
                   bool aSupportLowPriorityTransactions,
                   bool aSupportLowPriorityThreadpool,
                   bool aAllowTextureSwizzling,
                   bool aAllowScissoredCacheClears,
                   bool aStartDebugServer,
                   void *aSwglContext,
                   void *aGlContext,
                   bool aSurfaceOriginIsTopLeft,
                   WrProgramCache *aProgramCache,
                   WrShaders *aShaders,
                   WrThreadPool *aThreadPool,
                   WrThreadPool *aThreadPoolLowPriority,
                   VoidPtrToSizeFn aSizeOfOp,
                   VoidPtrToSizeFn aEnclosingSizeOfOp,
                   uint32_t aDocumentId,
                   void *aCompositor,
                   bool aUseNativeCompositor,
                   uintptr_t aMaxUpdateRects,
                   bool aUsePartialPresent,
                   uintptr_t aMaxPartialPresentRects,
                   bool aDrawPreviousPartialPresentRegions,
                   DocumentHandle **aOutHandle,
                   Renderer **aOutRenderer,
                   int32_t *aOutMaxTextureSize,
                   char **aOutErr,
                   bool aEnableGpuMarkers,
                   bool aPanicOnGlError,
                   int32_t aPictureTileWidth,
                   int32_t aPictureTileHeight);

void wr_api_free_error_msg(char *aMsg);

void wr_api_delete_document(DocumentHandle *aDh);

void wr_api_clone(DocumentHandle *aDh,
                  DocumentHandle **aOutHandle);

void wr_api_delete(DocumentHandle *aDh);

void wr_api_shut_down(DocumentHandle *aDh);

void wr_api_notify_memory_pressure(DocumentHandle *aDh);

void wr_api_set_debug_flags(DocumentHandle *aDh,
                            DebugFlags aFlags);

void wr_api_accumulate_memory_report(DocumentHandle *aDh,
                                     MemoryReport *aReport,
                                     uintptr_t (*aSizeOfOp)(const void *ptr),
                                     uintptr_t (*aEnclosingSizeOfOp)(const void *ptr));

void wr_api_clear_all_caches(DocumentHandle *aDh);

void wr_api_enable_native_compositor(DocumentHandle *aDh,
                                     bool aEnable);

void wr_api_enable_multithreading(DocumentHandle *aDh,
                                  bool aEnable);

void wr_api_set_batching_lookback(DocumentHandle *aDh,
                                  uint32_t aCount);

Transaction *wr_transaction_new(bool aDoAsync);

void wr_transaction_delete(Transaction *aTxn);

void wr_transaction_set_low_priority(Transaction *aTxn,
                                     bool aLowPriority);

bool wr_transaction_is_empty(const Transaction *aTxn);

bool wr_transaction_resource_updates_is_empty(const Transaction *aTxn);

bool wr_transaction_is_rendered_frame_invalidated(const Transaction *aTxn);

void wr_transaction_notify(Transaction *aTxn,
                           Checkpoint aWhen,
                           uintptr_t aEvent);

void wr_transaction_update_epoch(Transaction *aTxn,
                                 WrPipelineId aPipelineId,
                                 WrEpoch aEpoch);

void wr_transaction_set_root_pipeline(Transaction *aTxn,
                                      WrPipelineId aPipelineId);

void wr_transaction_remove_pipeline(Transaction *aTxn,
                                    WrPipelineId aPipelineId);

void wr_transaction_set_display_list(Transaction *aTxn,
                                     WrEpoch aEpoch,
                                     ColorF aBackground,
                                     LayoutSize aViewportSize,
                                     WrPipelineId aPipelineId,
                                     BuiltDisplayListDescriptor aDlDescriptor,
                                     WrVecU8 *aDlData);

void wr_transaction_set_document_view(Transaction *aTxn,
                                      const DeviceIntRect *aDocRect);

void wr_transaction_generate_frame(Transaction *aTxn,
                                   uint64_t aId);

void wr_transaction_invalidate_rendered_frame(Transaction *aTxn);

void wr_transaction_update_dynamic_properties(Transaction *aTxn,
                                              const WrOpacityProperty *aOpacityArray,
                                              uintptr_t aOpacityCount,
                                              const WrTransformProperty *aTransformArray,
                                              uintptr_t aTransformCount,
                                              const WrColorProperty *aColorArray,
                                              uintptr_t aColorCount);

void wr_transaction_append_transform_properties(Transaction *aTxn,
                                                const WrTransformProperty *aTransformArray,
                                                uintptr_t aTransformCount);

void wr_transaction_scroll_layer(Transaction *aTxn,
                                 WrPipelineId aPipelineId,
                                 uint64_t aScrollId,
                                 LayoutPoint aNewScrollOrigin);

void wr_transaction_pinch_zoom(Transaction *aTxn,
                               float aPinchZoom);

void wr_transaction_set_is_transform_async_zooming(Transaction *aTxn,
                                                   uint64_t aAnimationId,
                                                   bool aIsZooming);

void wr_transaction_set_quality_settings(Transaction *aTxn,
                                         bool aForceSubpixelAaWherePossible);

void wr_resource_updates_add_image(Transaction *aTxn,
                                   WrImageKey aImageKey,
                                   const WrImageDescriptor *aDescriptor,
                                   WrVecU8 *aBytes);

void wr_resource_updates_add_blob_image(Transaction *aTxn,
                                        BlobImageKey aImageKey,
                                        const WrImageDescriptor *aDescriptor,
                                        WrVecU8 *aBytes,
                                        DeviceIntRect aVisibleRect);

void wr_resource_updates_add_external_image(Transaction *aTxn,
                                            WrImageKey aImageKey,
                                            const WrImageDescriptor *aDescriptor,
                                            ExternalImageId aExternalImageId,
                                            const ExternalImageType *aImageType,
                                            uint8_t aChannelIndex);

void wr_resource_updates_update_image(Transaction *aTxn,
                                      WrImageKey aKey,
                                      const WrImageDescriptor *aDescriptor,
                                      WrVecU8 *aBytes);

void wr_resource_updates_set_blob_image_visible_area(Transaction *aTxn,
                                                     BlobImageKey aKey,
                                                     const DeviceIntRect *aArea);

void wr_resource_updates_update_external_image(Transaction *aTxn,
                                               WrImageKey aKey,
                                               const WrImageDescriptor *aDescriptor,
                                               ExternalImageId aExternalImageId,
                                               const ExternalImageType *aImageType,
                                               uint8_t aChannelIndex);

void wr_resource_updates_update_external_image_with_dirty_rect(Transaction *aTxn,
                                                               WrImageKey aKey,
                                                               const WrImageDescriptor *aDescriptor,
                                                               ExternalImageId aExternalImageId,
                                                               const ExternalImageType *aImageType,
                                                               uint8_t aChannelIndex,
                                                               DeviceIntRect aDirtyRect);

void wr_resource_updates_update_blob_image(Transaction *aTxn,
                                           BlobImageKey aImageKey,
                                           const WrImageDescriptor *aDescriptor,
                                           WrVecU8 *aBytes,
                                           DeviceIntRect aVisibleRect,
                                           LayoutIntRect aDirtyRect);

void wr_resource_updates_delete_image(Transaction *aTxn,
                                      WrImageKey aKey);

void wr_resource_updates_delete_blob_image(Transaction *aTxn,
                                           BlobImageKey aKey);

void wr_api_send_transaction(DocumentHandle *aDh,
                             Transaction *aTransaction,
                             bool aIsAsync);

void wr_transaction_clear_display_list(Transaction *aTxn,
                                       WrEpoch aEpoch,
                                       WrPipelineId aPipelineId);

void wr_api_send_external_event(DocumentHandle *aDh,
                                uintptr_t aEvt);

void wr_resource_updates_add_raw_font(Transaction *aTxn,
                                      WrFontKey aKey,
                                      WrVecU8 *aBytes,
                                      uint32_t aIndex);

void wr_api_capture(DocumentHandle *aDh,
                    const char *aPath,
                    uint32_t aBitsRaw);

void wr_api_start_capture_sequence(DocumentHandle *aDh,
                                   const char *aPath,
                                   uint32_t aBitsRaw);

void wr_api_stop_capture_sequence(DocumentHandle *aDh);

void wr_resource_updates_add_font_descriptor(Transaction *aTxn,
                                             WrFontKey aKey,
                                             WrVecU8 *aBytes,
                                             uint32_t aIndex);

void wr_resource_updates_delete_font(Transaction *aTxn,
                                     WrFontKey aKey);

void wr_resource_updates_add_font_instance(Transaction *aTxn,
                                           WrFontInstanceKey aKey,
                                           WrFontKey aFontKey,
                                           float aGlyphSize,
                                           const FontInstanceOptions *aOptions,
                                           const FontInstancePlatformOptions *aPlatformOptions,
                                           WrVecU8 *aVariations);

void wr_resource_updates_delete_font_instance(Transaction *aTxn,
                                              WrFontInstanceKey aKey);

void wr_resource_updates_clear(Transaction *aTxn);

WrIdNamespace wr_api_get_namespace(DocumentHandle *aDh);

void wr_api_wake_scene_builder(DocumentHandle *aDh);

void wr_api_flush_scene_builder(DocumentHandle *aDh);

WrState *wr_state_new(WrPipelineId aPipelineId,
                      uintptr_t aCapacity);

void wr_state_delete(WrState *aState);

void wr_dp_save(WrState *aState);

void wr_dp_restore(WrState *aState);

void wr_dp_clear_save(WrState *aState);

WrSpatialId wr_dp_push_stacking_context(WrState *aState,
                                        LayoutRect aBounds,
                                        WrSpatialId aSpatialId,
                                        const WrStackingContextParams *aParams,
                                        const LayoutTransform *aTransform,
                                        const FilterOp *aFilters,
                                        uintptr_t aFilterCount,
                                        const WrFilterData *aFilterDatas,
                                        uintptr_t aFilterDatasCount,
                                        RasterSpace aGlyphRasterSpace);

void wr_dp_pop_stacking_context(WrState *aState,
                                bool aIsReferenceFrame);

uint64_t wr_dp_define_clipchain(WrState *aState,
                                const uint64_t *aParentClipchainId,
                                const WrClipId *aClips,
                                uintptr_t aClipsCount);

WrClipId wr_dp_define_clip_with_parent_clip(WrState *aState,
                                            const WrSpaceAndClip *aParent,
                                            LayoutRect aClipRect,
                                            const ComplexClipRegion *aComplex,
                                            uintptr_t aComplexCount);

WrClipId wr_dp_define_clip_with_parent_clip_chain(WrState *aState,
                                                  const WrSpaceAndClipChain *aParent,
                                                  LayoutRect aClipRect,
                                                  const ComplexClipRegion *aComplex,
                                                  uintptr_t aComplexCount);

WrClipId wr_dp_define_image_mask_clip_with_parent_clip_chain(WrState *aState,
                                                             const WrSpaceAndClipChain *aParent,
                                                             ImageMask aMask);

WrClipId wr_dp_define_rounded_rect_clip_with_parent_clip_chain(WrState *aState,
                                                               const WrSpaceAndClipChain *aParent,
                                                               ComplexClipRegion aComplex);

WrClipId wr_dp_define_rect_clip_with_parent_clip_chain(WrState *aState,
                                                       const WrSpaceAndClipChain *aParent,
                                                       LayoutRect aClipRect);

WrSpatialId wr_dp_define_sticky_frame(WrState *aState,
                                      WrSpatialId aParentSpatialId,
                                      LayoutRect aContentRect,
                                      const float *aTopMargin,
                                      const float *aRightMargin,
                                      const float *aBottomMargin,
                                      const float *aLeftMargin,
                                      StickyOffsetBounds aVerticalBounds,
                                      StickyOffsetBounds aHorizontalBounds,
                                      LayoutVector2D aAppliedOffset);

WrSpaceAndClip wr_dp_define_scroll_layer(WrState *aState,
                                         uint64_t aExternalScrollId,
                                         const WrSpaceAndClip *aParent,
                                         LayoutRect aContentRect,
                                         LayoutRect aClipRect,
                                         LayoutPoint aScrollOffset);

void wr_dp_push_iframe(WrState *aState,
                       LayoutRect aRect,
                       LayoutRect aClip,
                       bool aIsBackfaceVisible,
                       const WrSpaceAndClipChain *aParent,
                       WrPipelineId aPipelineId,
                       bool aIgnoreMissingPipeline);

void wr_dp_push_rect(WrState *aState,
                     LayoutRect aRect,
                     LayoutRect aClip,
                     bool aIsBackfaceVisible,
                     const WrSpaceAndClipChain *aParent,
                     ColorF aColor);

void wr_dp_push_rect_with_animation(WrState *aState,
                                    LayoutRect aRect,
                                    LayoutRect aClip,
                                    bool aIsBackfaceVisible,
                                    const WrSpaceAndClipChain *aParent,
                                    ColorF aColor,
                                    const WrAnimationProperty *aAnimation);

void wr_dp_push_rect_with_parent_clip(WrState *aState,
                                      LayoutRect aRect,
                                      LayoutRect aClipRect,
                                      bool aIsBackfaceVisible,
                                      const WrSpaceAndClip *aParent,
                                      ColorF aColor);

void wr_dp_push_backdrop_filter_with_parent_clip(WrState *aState,
                                                 LayoutRect aRect,
                                                 LayoutRect aClip,
                                                 bool aIsBackfaceVisible,
                                                 const WrSpaceAndClip *aParent,
                                                 const FilterOp *aFilters,
                                                 uintptr_t aFilterCount,
                                                 const WrFilterData *aFilterDatas,
                                                 uintptr_t aFilterDatasCount);

void wr_dp_push_clear_rect(WrState *aState,
                           LayoutRect aRect,
                           LayoutRect aClipRect,
                           const WrSpaceAndClipChain *aParent);

void wr_dp_push_hit_test(WrState *aState,
                         LayoutRect aRect,
                         LayoutRect aClip,
                         bool aIsBackfaceVisible,
                         const WrSpaceAndClipChain *aParent,
                         uint64_t aScrollId,
                         uint16_t aHitInfo);

void wr_dp_push_clear_rect_with_parent_clip(WrState *aState,
                                            LayoutRect aRect,
                                            LayoutRect aClipRect,
                                            const WrSpaceAndClip *aParent);

void wr_dp_push_image(WrState *aState,
                      LayoutRect aBounds,
                      LayoutRect aClip,
                      bool aIsBackfaceVisible,
                      const WrSpaceAndClipChain *aParent,
                      ImageRendering aImageRendering,
                      WrImageKey aKey,
                      bool aPremultipliedAlpha,
                      ColorF aColor,
                      bool aPreferCompositorSurface,
                      bool aSupportsExternalCompositing);

void wr_dp_push_repeating_image(WrState *aState,
                                LayoutRect aBounds,
                                LayoutRect aClip,
                                bool aIsBackfaceVisible,
                                const WrSpaceAndClipChain *aParent,
                                LayoutSize aStretchSize,
                                LayoutSize aTileSpacing,
                                ImageRendering aImageRendering,
                                WrImageKey aKey,
                                bool aPremultipliedAlpha,
                                ColorF aColor);

/// Push a 3 planar yuv image.
void wr_dp_push_yuv_planar_image(WrState *aState,
                                 LayoutRect aBounds,
                                 LayoutRect aClip,
                                 bool aIsBackfaceVisible,
                                 const WrSpaceAndClipChain *aParent,
                                 WrImageKey aImageKey0,
                                 WrImageKey aImageKey1,
                                 WrImageKey aImageKey2,
                                 WrColorDepth aColorDepth,
                                 WrYuvColorSpace aColorSpace,
                                 WrColorRange aColorRange,
                                 ImageRendering aImageRendering,
                                 bool aPreferCompositorSurface,
                                 bool aSupportsExternalCompositing);

/// Push a 2 planar NV12 image.
void wr_dp_push_yuv_NV12_image(WrState *aState,
                               LayoutRect aBounds,
                               LayoutRect aClip,
                               bool aIsBackfaceVisible,
                               const WrSpaceAndClipChain *aParent,
                               WrImageKey aImageKey0,
                               WrImageKey aImageKey1,
                               WrColorDepth aColorDepth,
                               WrYuvColorSpace aColorSpace,
                               WrColorRange aColorRange,
                               ImageRendering aImageRendering,
                               bool aPreferCompositorSurface,
                               bool aSupportsExternalCompositing);

/// Push a yuv interleaved image.
void wr_dp_push_yuv_interleaved_image(WrState *aState,
                                      LayoutRect aBounds,
                                      LayoutRect aClip,
                                      bool aIsBackfaceVisible,
                                      const WrSpaceAndClipChain *aParent,
                                      WrImageKey aImageKey0,
                                      WrColorDepth aColorDepth,
                                      WrYuvColorSpace aColorSpace,
                                      WrColorRange aColorRange,
                                      ImageRendering aImageRendering,
                                      bool aPreferCompositorSurface,
                                      bool aSupportsExternalCompositing);

void wr_dp_push_text(WrState *aState,
                     LayoutRect aBounds,
                     LayoutRect aClip,
                     bool aIsBackfaceVisible,
                     const WrSpaceAndClipChain *aParent,
                     ColorF aColor,
                     WrFontInstanceKey aFontKey,
                     const GlyphInstance *aGlyphs,
                     uint32_t aGlyphCount,
                     const GlyphOptions *aGlyphOptions);

void wr_dp_push_shadow(WrState *aState,
                       LayoutRect aBounds,
                       LayoutRect aClip,
                       bool aIsBackfaceVisible,
                       const WrSpaceAndClipChain *aParent,
                       Shadow aShadow,
                       bool aShouldInflate);

void wr_dp_pop_all_shadows(WrState *aState);

void wr_dp_push_line(WrState *aState,
                     const LayoutRect *aClip,
                     bool aIsBackfaceVisible,
                     const WrSpaceAndClipChain *aParent,
                     const LayoutRect *aBounds,
                     float aWavyLineThickness,
                     LineOrientation aOrientation,
                     const ColorF *aColor,
                     LineStyle aStyle);

void wr_dp_push_border(WrState *aState,
                       LayoutRect aRect,
                       LayoutRect aClip,
                       bool aIsBackfaceVisible,
                       const WrSpaceAndClipChain *aParent,
                       AntialiasBorder aDoAa,
                       LayoutSideOffsets aWidths,
                       BorderSide aTop,
                       BorderSide aRight,
                       BorderSide aBottom,
                       BorderSide aLeft,
                       BorderRadius aRadius);

void wr_dp_push_border_image(WrState *aState,
                             LayoutRect aRect,
                             LayoutRect aClip,
                             bool aIsBackfaceVisible,
                             const WrSpaceAndClipChain *aParent,
                             const WrBorderImage *aParams);

void wr_dp_push_border_gradient(WrState *aState,
                                LayoutRect aRect,
                                LayoutRect aClip,
                                bool aIsBackfaceVisible,
                                const WrSpaceAndClipChain *aParent,
                                LayoutSideOffsets aWidths,
                                int32_t aWidth,
                                int32_t aHeight,
                                bool aFill,
                                DeviceIntSideOffsets aSlice,
                                LayoutPoint aStartPoint,
                                LayoutPoint aEndPoint,
                                const GradientStop *aStops,
                                uintptr_t aStopsCount,
                                ExtendMode aExtendMode,
                                LayoutSideOffsets aOutset);

void wr_dp_push_border_radial_gradient(WrState *aState,
                                       LayoutRect aRect,
                                       LayoutRect aClip,
                                       bool aIsBackfaceVisible,
                                       const WrSpaceAndClipChain *aParent,
                                       LayoutSideOffsets aWidths,
                                       bool aFill,
                                       LayoutPoint aCenter,
                                       LayoutSize aRadius,
                                       const GradientStop *aStops,
                                       uintptr_t aStopsCount,
                                       ExtendMode aExtendMode,
                                       LayoutSideOffsets aOutset);

void wr_dp_push_border_conic_gradient(WrState *aState,
                                      LayoutRect aRect,
                                      LayoutRect aClip,
                                      bool aIsBackfaceVisible,
                                      const WrSpaceAndClipChain *aParent,
                                      LayoutSideOffsets aWidths,
                                      bool aFill,
                                      LayoutPoint aCenter,
                                      float aAngle,
                                      const GradientStop *aStops,
                                      uintptr_t aStopsCount,
                                      ExtendMode aExtendMode,
                                      LayoutSideOffsets aOutset);

void wr_dp_push_linear_gradient(WrState *aState,
                                LayoutRect aRect,
                                LayoutRect aClip,
                                bool aIsBackfaceVisible,
                                const WrSpaceAndClipChain *aParent,
                                LayoutPoint aStartPoint,
                                LayoutPoint aEndPoint,
                                const GradientStop *aStops,
                                uintptr_t aStopsCount,
                                ExtendMode aExtendMode,
                                LayoutSize aTileSize,
                                LayoutSize aTileSpacing);

void wr_dp_push_radial_gradient(WrState *aState,
                                LayoutRect aRect,
                                LayoutRect aClip,
                                bool aIsBackfaceVisible,
                                const WrSpaceAndClipChain *aParent,
                                LayoutPoint aCenter,
                                LayoutSize aRadius,
                                const GradientStop *aStops,
                                uintptr_t aStopsCount,
                                ExtendMode aExtendMode,
                                LayoutSize aTileSize,
                                LayoutSize aTileSpacing);

void wr_dp_push_conic_gradient(WrState *aState,
                               LayoutRect aRect,
                               LayoutRect aClip,
                               bool aIsBackfaceVisible,
                               const WrSpaceAndClipChain *aParent,
                               LayoutPoint aCenter,
                               float aAngle,
                               const GradientStop *aStops,
                               uintptr_t aStopsCount,
                               ExtendMode aExtendMode,
                               LayoutSize aTileSize,
                               LayoutSize aTileSpacing);

void wr_dp_push_box_shadow(WrState *aState,
                           LayoutRect aRect,
                           LayoutRect aClip,
                           bool aIsBackfaceVisible,
                           const WrSpaceAndClipChain *aParent,
                           LayoutRect aBoxBounds,
                           LayoutVector2D aOffset,
                           ColorF aColor,
                           float aBlurRadius,
                           float aSpreadRadius,
                           BorderRadius aBorderRadius,
                           BoxShadowClipMode aClipMode);

void wr_dp_start_item_group(WrState *aState);

void wr_dp_cancel_item_group(WrState *aState,
                             bool aDiscard);

bool wr_dp_finish_item_group(WrState *aState,
                             ItemKey aKey);

void wr_dp_push_reuse_items(WrState *aState,
                            ItemKey aKey);

void wr_dp_set_cache_size(WrState *aState,
                          uintptr_t aCacheSize);

uintptr_t wr_dump_display_list(WrState *aState,
                               uintptr_t aIndent,
                               const uintptr_t *aStart,
                               const uintptr_t *aEnd);

void wr_dump_serialized_display_list(WrState *aState);

void wr_api_finalize_builder(WrState *aState,
                             BuiltDisplayListDescriptor *aDlDescriptor,
                             WrVecU8 *aDlData);

void wr_api_hit_test(DocumentHandle *aDh,
                     WorldPoint aPoint,
                     nsTArray<HitResult> *aOutResults);

const VecU8 *wr_add_ref_arc(const ArcVecU8 *aArc);

void wr_dec_ref_arc(const VecU8 *aArc);

extern bool wr_moz2d_render_cb(ByteSlice aBlob,
                               ImageFormat aFormat,
                               const LayoutIntRect *aRenderRect,
                               const DeviceIntRect *aVisibleRect,
                               uint16_t aTileSize,
                               const TileOffset *aTileOffset,
                               const LayoutIntRect *aDirtyRect,
                               MutByteSlice aOutput);

WrSpatialId wr_root_scroll_node_id();

WrClipId wr_root_clip_id();

void wr_device_delete(Device *aDevice);

WrShaders *wr_shaders_new(void *aGlContext,
                          WrProgramCache *aProgramCache,
                          bool aPrecacheShaders);

void wr_shaders_delete(WrShaders *aShaders,
                       void *aGlContext);

uintptr_t wr_program_cache_report_memory(const WrProgramCache *aCache,
                                         VoidPtrToSizeFn aSizeOfOp);

extern bool HasFontData(WrFontKey aKey);

extern void AddFontData(WrFontKey aKey,
                        const uint8_t *aData,
                        uintptr_t aSize,
                        uint32_t aIndex,
                        const ArcVecU8 *aVec);

extern void AddNativeFontHandle(WrFontKey aKey,
                                void *aHandle,
                                uint32_t aIndex);

extern void DeleteFontData(WrFontKey aKey);

extern void AddBlobFont(WrFontInstanceKey aInstanceKey,
                        WrFontKey aFontKey,
                        float aSize,
                        const FontInstanceOptions *aOptions,
                        const FontInstancePlatformOptions *aPlatformOptions,
                        const FontVariation *aVariations,
                        uintptr_t aNumVariations);

extern void DeleteBlobFont(WrFontInstanceKey aKey);

extern void ClearBlobImageResources(WrIdNamespace aNamespace);

void *wr_swgl_create_context();

void wr_swgl_reference_context(void *aCtx);

void wr_swgl_destroy_context(void *aCtx);

void wr_swgl_make_current(void *aCtx);

void wr_swgl_init_default_framebuffer(void *aCtx,
                                      int32_t aX,
                                      int32_t aY,
                                      int32_t aWidth,
                                      int32_t aHeight,
                                      int32_t aStride,
                                      void *aBuf);

uint32_t wr_swgl_gen_texture(void *aCtx);

void wr_swgl_delete_texture(void *aCtx,
                            uint32_t aTex);

void wr_swgl_set_texture_parameter(void *aCtx,
                                   uint32_t aTex,
                                   uint32_t aPname,
                                   int32_t aParam);

void wr_swgl_set_texture_buffer(void *aCtx,
                                uint32_t aTex,
                                uint32_t aInternalFormat,
                                int32_t aWidth,
                                int32_t aHeight,
                                int32_t aStride,
                                void *aBuf,
                                int32_t aMinWidth,
                                int32_t aMinHeight);

void wr_swgl_clear_color_rect(void *aCtx,
                              uint32_t aFbo,
                              int32_t aX,
                              int32_t aY,
                              int32_t aWidth,
                              int32_t aHeight,
                              float aR,
                              float aG,
                              float aB,
                              float aA);

} // extern "C"

} // namespace wr
} // namespace mozilla
