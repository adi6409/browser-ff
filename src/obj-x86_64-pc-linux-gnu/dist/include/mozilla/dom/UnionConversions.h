#ifndef mozilla_dom_UnionConversions_h
#define mozilla_dom_UnionConversions_h

#include "AnimatableBinding.h"
#include "AudioTrackBinding.h"
#include "BaseKeyframeTypesBinding.h"
#include "BlobBinding.h"
#include "CSSStyleSheetBinding.h"
#include "CanvasRenderingContext2DBinding.h"
#include "ChromeUtilsBinding.h"
#include "ClientBinding.h"
#include "ClipboardBinding.h"
#include "ConsoleBinding.h"
#include "DOMMatrixBinding.h"
#include "DirectoryBinding.h"
#include "DocumentBinding.h"
#include "DocumentFragmentBinding.h"
#include "ElementBinding.h"
#include "EventBinding.h"
#include "EventHandlerBinding.h"
#include "EventTargetBinding.h"
#include "ExtendableMessageEventBinding.h"
#include "ExternalBinding.h"
#include "FetchBinding.h"
#include "FileBinding.h"
#include "FluentBinding.h"
#include "FontFaceBinding.h"
#include "FormDataBinding.h"
#include "GeometryUtilsBinding.h"
#include "HTMLAllCollectionBinding.h"
#include "HTMLCanvasElementBinding.h"
#include "HTMLCollectionBinding.h"
#include "HTMLElementBinding.h"
#include "HTMLFormControlsCollectionBinding.h"
#include "HTMLImageElementBinding.h"
#include "HTMLOptGroupElementBinding.h"
#include "HTMLOptionElementBinding.h"
#include "HTMLVideoElementBinding.h"
#include "HeadersBinding.h"
#include "IDBCursorBinding.h"
#include "IDBFileHandleBinding.h"
#include "IDBIndexBinding.h"
#include "IDBObjectStoreBinding.h"
#include "IDBRequestBinding.h"
#include "ImageBitmapBinding.h"
#include "ImageDataBinding.h"
#include "InstallTriggerBinding.h"
#include "IntersectionObserverBinding.h"
#include "KeyframeAnimationOptionsBinding.h"
#include "KeyframeEffectBinding.h"
#include "LocalizationBinding.h"
#include "MatchGlobBinding.h"
#include "MatchPatternBinding.h"
#include "MediaListBinding.h"
#include "MediaStreamBinding.h"
#include "MediaStreamTrackBinding.h"
#include "MessageEventBinding.h"
#include "MessagePortBinding.h"
#include "NodeBinding.h"
#include "OffscreenCanvasBinding.h"
#include "PushEventBinding.h"
#include "PushManagerBinding.h"
#include "RTCIceCandidateBinding.h"
#include "RTCPeerConnectionBinding.h"
#include "RadioNodeListBinding.h"
#include "RequestBinding.h"
#include "ResponseBinding.h"
#include "SVGImageElementBinding.h"
#include "SanitizerBinding.h"
#include "ServiceWorkerBinding.h"
#include "SessionStoreUtilsBinding.h"
#include "SharedWorkerBinding.h"
#include "StreamFilterBinding.h"
#include "SubtleCryptoBinding.h"
#include "TextBinding.h"
#include "TextTrackBinding.h"
#include "TrackEventBinding.h"
#include "UDPSocketBinding.h"
#include "URLSearchParamsBinding.h"
#include "VTTCueBinding.h"
#include "VideoTrackBinding.h"
#include "WebExtensionContentScriptBinding.h"
#include "WebGL2RenderingContextBinding.h"
#include "WebGLRenderingContextBinding.h"
#include "WebGPUBinding.h"
#include "WebXRBinding.h"
#include "WindowBinding.h"
#include "WorkerBinding.h"
#include "XMLHttpRequestBinding.h"
#include "XULPopupElementBinding.h"
#include "js/ForOfIterator.h"
#include "js/RootingAPI.h"
#include "mozilla/FloatingPoint.h"
#include "mozilla/dom/BindingCallContext.h"
#include "mozilla/dom/PrimitiveConversions.h"
#include "mozilla/dom/ReadableStream.h"
#include "mozilla/dom/Record.h"
#include "mozilla/dom/TypedArray.h"
#include "mozilla/dom/UnionTypes.h"
#include "mozilla/dom/WindowProxyHolder.h"
#include "nsDebug.h"

namespace mozilla {
namespace dom {
class AddEventListenerOptionsOrBooleanArgument
{
  AddEventListenerOptionsOrBoolean& mUnion;

  AddEventListenerOptionsOrBooleanArgument(const AddEventListenerOptionsOrBooleanArgument&) = delete;
  AddEventListenerOptionsOrBooleanArgument& operator=(const AddEventListenerOptionsOrBooleanArgument&) = delete;
public:
  explicit inline AddEventListenerOptionsOrBooleanArgument(const AddEventListenerOptionsOrBoolean& aUnion)
    : mUnion(const_cast<AddEventListenerOptionsOrBoolean&>(aUnion))
  {
  }

  inline bool
  TrySetToAddEventListenerOptions(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FastAddEventListenerOptions& memberSlot = RawSetAsAddEventListenerOptions();
      if (!IsConvertibleToDictionary(value)) {
        mUnion.DestroyAddEventListenerOptions();
        tryNext = true;
        return true;
      }
      if (!memberSlot.Init(cx, value, "AddEventListenerOptions branch of (AddEventListenerOptions or boolean)", passedToJSImpl)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToAddEventListenerOptions(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToAddEventListenerOptions(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToBoolean(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      bool& memberSlot = RawSetAsBoolean();
      if (!ValueToPrimitive<bool, eDefault>(cx, value, "Boolean branch of (AddEventListenerOptions or boolean)", &memberSlot)) {
        return false;
      }
    }
    return true;
  }

private:
  inline binding_detail::FastAddEventListenerOptions&
  RawSetAsAddEventListenerOptions()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eAddEventListenerOptions;
    return mUnion.mValue.mAddEventListenerOptions.SetValue();
  }

  inline bool&
  RawSetAsBoolean()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eBoolean;
    return mUnion.mValue.mBoolean.SetValue();
  }
};

class ArrayBufferOrLongArgument
{
  ArrayBufferOrLong& mUnion;

  ArrayBufferOrLongArgument(const ArrayBufferOrLongArgument&) = delete;
  ArrayBufferOrLongArgument& operator=(const ArrayBufferOrLongArgument&) = delete;
public:
  explicit inline ArrayBufferOrLongArgument(const ArrayBufferOrLong& aUnion)
    : mUnion(const_cast<ArrayBufferOrLong&>(aUnion))
  {
  }

  inline bool
  TrySetToArrayBuffer(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      RootedSpiderMonkeyInterface<ArrayBuffer>& memberSlot = RawSetAsArrayBuffer(cx);
      if (!memberSlot.Init(&value.toObject())) {
        mUnion.DestroyArrayBuffer();
        tryNext = true;
        return true;
      }
      if (JS::IsSharedArrayBufferObject(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_SHARED>("ArrayBuffer branch of (ArrayBuffer or long)");
        return false;
      }
      if (JS::IsLargeArrayBufferMaybeShared(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_LARGE>("ArrayBuffer branch of (ArrayBuffer or long)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToArrayBuffer(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToArrayBuffer(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToLong(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      int32_t& memberSlot = RawSetAsLong();
      if (!ValueToPrimitive<int32_t, eDefault>(cx, value, "Long branch of (ArrayBuffer or long)", &memberSlot)) {
        return false;
      }
    }
    return true;
  }

private:
  inline RootedSpiderMonkeyInterface<ArrayBuffer>&
  RawSetAsArrayBuffer(JSContext* cx)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eArrayBuffer;
    return mUnion.mValue.mArrayBuffer.SetValue(cx);
  }

  inline int32_t&
  RawSetAsLong()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eLong;
    return mUnion.mValue.mLong.SetValue();
  }
};

class ArrayBufferOrUint8ArrayArgument
{
  ArrayBufferOrUint8Array& mUnion;

  ArrayBufferOrUint8ArrayArgument(const ArrayBufferOrUint8ArrayArgument&) = delete;
  ArrayBufferOrUint8ArrayArgument& operator=(const ArrayBufferOrUint8ArrayArgument&) = delete;
public:
  explicit inline ArrayBufferOrUint8ArrayArgument(const ArrayBufferOrUint8Array& aUnion)
    : mUnion(const_cast<ArrayBufferOrUint8Array&>(aUnion))
  {
  }

  inline bool
  TrySetToArrayBuffer(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      RootedSpiderMonkeyInterface<ArrayBuffer>& memberSlot = RawSetAsArrayBuffer(cx);
      if (!memberSlot.Init(&value.toObject())) {
        mUnion.DestroyArrayBuffer();
        tryNext = true;
        return true;
      }
      if (JS::IsSharedArrayBufferObject(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_SHARED>("ArrayBuffer branch of (ArrayBuffer or Uint8Array)");
        return false;
      }
      if (JS::IsLargeArrayBufferMaybeShared(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_LARGE>("ArrayBuffer branch of (ArrayBuffer or Uint8Array)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToArrayBuffer(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToArrayBuffer(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToUint8Array(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      RootedSpiderMonkeyInterface<Uint8Array>& memberSlot = RawSetAsUint8Array(cx);
      if (!memberSlot.Init(&value.toObject())) {
        mUnion.DestroyUint8Array();
        tryNext = true;
        return true;
      }
      if (JS::IsArrayBufferViewShared(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_SHARED>("Uint8Array branch of (ArrayBuffer or Uint8Array)");
        return false;
      }
      if (JS::IsLargeArrayBufferView(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_LARGE>("Uint8Array branch of (ArrayBuffer or Uint8Array)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToUint8Array(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToUint8Array(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline RootedSpiderMonkeyInterface<ArrayBuffer>&
  RawSetAsArrayBuffer(JSContext* cx)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eArrayBuffer;
    return mUnion.mValue.mArrayBuffer.SetValue(cx);
  }

  inline RootedSpiderMonkeyInterface<Uint8Array>&
  RawSetAsUint8Array(JSContext* cx)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUint8Array;
    return mUnion.mValue.mUint8Array.SetValue(cx);
  }
};

class ArrayBufferViewOrArrayBufferArgument
{
  ArrayBufferViewOrArrayBuffer& mUnion;

  ArrayBufferViewOrArrayBufferArgument(const ArrayBufferViewOrArrayBufferArgument&) = delete;
  ArrayBufferViewOrArrayBufferArgument& operator=(const ArrayBufferViewOrArrayBufferArgument&) = delete;
public:
  explicit inline ArrayBufferViewOrArrayBufferArgument(const ArrayBufferViewOrArrayBuffer& aUnion)
    : mUnion(const_cast<ArrayBufferViewOrArrayBuffer&>(aUnion))
  {
  }

  inline bool
  TrySetToArrayBufferView(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      RootedSpiderMonkeyInterface<ArrayBufferView>& memberSlot = RawSetAsArrayBufferView(cx);
      if (!memberSlot.Init(&value.toObject())) {
        mUnion.DestroyArrayBufferView();
        tryNext = true;
        return true;
      }
      if (JS::IsArrayBufferViewShared(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_SHARED>("ArrayBufferView branch of (ArrayBufferView or ArrayBuffer)");
        return false;
      }
      if (JS::IsLargeArrayBufferView(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_LARGE>("ArrayBufferView branch of (ArrayBufferView or ArrayBuffer)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToArrayBufferView(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToArrayBufferView(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToArrayBuffer(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      RootedSpiderMonkeyInterface<ArrayBuffer>& memberSlot = RawSetAsArrayBuffer(cx);
      if (!memberSlot.Init(&value.toObject())) {
        mUnion.DestroyArrayBuffer();
        tryNext = true;
        return true;
      }
      if (JS::IsSharedArrayBufferObject(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_SHARED>("ArrayBuffer branch of (ArrayBufferView or ArrayBuffer)");
        return false;
      }
      if (JS::IsLargeArrayBufferMaybeShared(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_LARGE>("ArrayBuffer branch of (ArrayBufferView or ArrayBuffer)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToArrayBuffer(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToArrayBuffer(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline RootedSpiderMonkeyInterface<ArrayBufferView>&
  RawSetAsArrayBufferView(JSContext* cx)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eArrayBufferView;
    return mUnion.mValue.mArrayBufferView.SetValue(cx);
  }

  inline RootedSpiderMonkeyInterface<ArrayBuffer>&
  RawSetAsArrayBuffer(JSContext* cx)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eArrayBuffer;
    return mUnion.mValue.mArrayBuffer.SetValue(cx);
  }
};

class ArrayBufferViewOrArrayBufferOrBlobOrUSVStringArgument
{
  ArrayBufferViewOrArrayBufferOrBlobOrUSVString& mUnion;

  ArrayBufferViewOrArrayBufferOrBlobOrUSVStringArgument(const ArrayBufferViewOrArrayBufferOrBlobOrUSVStringArgument&) = delete;
  ArrayBufferViewOrArrayBufferOrBlobOrUSVStringArgument& operator=(const ArrayBufferViewOrArrayBufferOrBlobOrUSVStringArgument&) = delete;
public:
  explicit inline ArrayBufferViewOrArrayBufferOrBlobOrUSVStringArgument(const ArrayBufferViewOrArrayBufferOrBlobOrUSVString& aUnion)
    : mUnion(const_cast<ArrayBufferViewOrArrayBufferOrBlobOrUSVString&>(aUnion))
  {
  }

  inline bool
  TrySetToArrayBufferView(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      RootedSpiderMonkeyInterface<ArrayBufferView>& memberSlot = RawSetAsArrayBufferView(cx);
      if (!memberSlot.Init(&value.toObject())) {
        mUnion.DestroyArrayBufferView();
        tryNext = true;
        return true;
      }
      if (JS::IsArrayBufferViewShared(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_SHARED>("ArrayBufferView branch of ((ArrayBufferView or ArrayBuffer) or Blob or USVString)");
        return false;
      }
      if (JS::IsLargeArrayBufferView(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_LARGE>("ArrayBufferView branch of ((ArrayBufferView or ArrayBuffer) or Blob or USVString)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToArrayBufferView(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToArrayBufferView(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToArrayBuffer(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      RootedSpiderMonkeyInterface<ArrayBuffer>& memberSlot = RawSetAsArrayBuffer(cx);
      if (!memberSlot.Init(&value.toObject())) {
        mUnion.DestroyArrayBuffer();
        tryNext = true;
        return true;
      }
      if (JS::IsSharedArrayBufferObject(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_SHARED>("ArrayBuffer branch of ((ArrayBufferView or ArrayBuffer) or Blob or USVString)");
        return false;
      }
      if (JS::IsLargeArrayBufferMaybeShared(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_LARGE>("ArrayBuffer branch of ((ArrayBufferView or ArrayBuffer) or Blob or USVString)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToArrayBuffer(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToArrayBuffer(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToBlob(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::Blob>& memberSlot = RawSetAsBlob();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::Blob, mozilla::dom::Blob>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyBlob();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToBlob(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToBlob(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToUSVString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsUSVString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
      if (!NormalizeUSVString(memberSlot)) {
        JS_ReportOutOfMemory(cx);
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsUSVString().AssignLiteral(aData);
  }

private:
  inline RootedSpiderMonkeyInterface<ArrayBufferView>&
  RawSetAsArrayBufferView(JSContext* cx)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eArrayBufferView;
    return mUnion.mValue.mArrayBufferView.SetValue(cx);
  }

  inline RootedSpiderMonkeyInterface<ArrayBuffer>&
  RawSetAsArrayBuffer(JSContext* cx)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eArrayBuffer;
    return mUnion.mValue.mArrayBuffer.SetValue(cx);
  }

  inline NonNull<mozilla::dom::Blob>&
  RawSetAsBlob()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eBlob;
    return mUnion.mValue.mBlob.SetValue();
  }

  inline binding_detail::FakeString<char16_t>&
  RawSetAsUSVString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUSVString;
    return mUnion.mValue.mUSVString.SetValue();
  }
};

class ArrayBufferViewOrArrayBufferOrStringArgument
{
  ArrayBufferViewOrArrayBufferOrString& mUnion;

  ArrayBufferViewOrArrayBufferOrStringArgument(const ArrayBufferViewOrArrayBufferOrStringArgument&) = delete;
  ArrayBufferViewOrArrayBufferOrStringArgument& operator=(const ArrayBufferViewOrArrayBufferOrStringArgument&) = delete;
public:
  explicit inline ArrayBufferViewOrArrayBufferOrStringArgument(const ArrayBufferViewOrArrayBufferOrString& aUnion)
    : mUnion(const_cast<ArrayBufferViewOrArrayBufferOrString&>(aUnion))
  {
  }

  inline bool
  TrySetToArrayBufferView(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      RootedSpiderMonkeyInterface<ArrayBufferView>& memberSlot = RawSetAsArrayBufferView(cx);
      if (!memberSlot.Init(&value.toObject())) {
        mUnion.DestroyArrayBufferView();
        tryNext = true;
        return true;
      }
      if (JS::IsArrayBufferViewShared(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_SHARED>("ArrayBufferView branch of ((ArrayBufferView or ArrayBuffer) or DOMString)");
        return false;
      }
      if (JS::IsLargeArrayBufferView(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_LARGE>("ArrayBufferView branch of ((ArrayBufferView or ArrayBuffer) or DOMString)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToArrayBufferView(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToArrayBufferView(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToArrayBuffer(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      RootedSpiderMonkeyInterface<ArrayBuffer>& memberSlot = RawSetAsArrayBuffer(cx);
      if (!memberSlot.Init(&value.toObject())) {
        mUnion.DestroyArrayBuffer();
        tryNext = true;
        return true;
      }
      if (JS::IsSharedArrayBufferObject(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_SHARED>("ArrayBuffer branch of ((ArrayBufferView or ArrayBuffer) or DOMString)");
        return false;
      }
      if (JS::IsLargeArrayBufferMaybeShared(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_LARGE>("ArrayBuffer branch of ((ArrayBufferView or ArrayBuffer) or DOMString)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToArrayBuffer(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToArrayBuffer(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsString().AssignLiteral(aData);
  }

private:
  inline RootedSpiderMonkeyInterface<ArrayBufferView>&
  RawSetAsArrayBufferView(JSContext* cx)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eArrayBufferView;
    return mUnion.mValue.mArrayBufferView.SetValue(cx);
  }

  inline RootedSpiderMonkeyInterface<ArrayBuffer>&
  RawSetAsArrayBuffer(JSContext* cx)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eArrayBuffer;
    return mUnion.mValue.mArrayBuffer.SetValue(cx);
  }

  inline binding_detail::FakeString<char16_t>&
  RawSetAsString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eString;
    return mUnion.mValue.mString.SetValue();
  }
};

class ArrayBufferViewOrArrayBufferOrUSVStringArgument
{
  ArrayBufferViewOrArrayBufferOrUSVString& mUnion;

  ArrayBufferViewOrArrayBufferOrUSVStringArgument(const ArrayBufferViewOrArrayBufferOrUSVStringArgument&) = delete;
  ArrayBufferViewOrArrayBufferOrUSVStringArgument& operator=(const ArrayBufferViewOrArrayBufferOrUSVStringArgument&) = delete;
public:
  explicit inline ArrayBufferViewOrArrayBufferOrUSVStringArgument(const ArrayBufferViewOrArrayBufferOrUSVString& aUnion)
    : mUnion(const_cast<ArrayBufferViewOrArrayBufferOrUSVString&>(aUnion))
  {
  }

  inline bool
  TrySetToArrayBufferView(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      RootedSpiderMonkeyInterface<ArrayBufferView>& memberSlot = RawSetAsArrayBufferView(cx);
      if (!memberSlot.Init(&value.toObject())) {
        mUnion.DestroyArrayBufferView();
        tryNext = true;
        return true;
      }
      if (JS::IsArrayBufferViewShared(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_SHARED>("ArrayBufferView branch of ((ArrayBufferView or ArrayBuffer) or USVString)");
        return false;
      }
      if (JS::IsLargeArrayBufferView(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_LARGE>("ArrayBufferView branch of ((ArrayBufferView or ArrayBuffer) or USVString)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToArrayBufferView(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToArrayBufferView(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToArrayBuffer(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      RootedSpiderMonkeyInterface<ArrayBuffer>& memberSlot = RawSetAsArrayBuffer(cx);
      if (!memberSlot.Init(&value.toObject())) {
        mUnion.DestroyArrayBuffer();
        tryNext = true;
        return true;
      }
      if (JS::IsSharedArrayBufferObject(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_SHARED>("ArrayBuffer branch of ((ArrayBufferView or ArrayBuffer) or USVString)");
        return false;
      }
      if (JS::IsLargeArrayBufferMaybeShared(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_LARGE>("ArrayBuffer branch of ((ArrayBufferView or ArrayBuffer) or USVString)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToArrayBuffer(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToArrayBuffer(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToUSVString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsUSVString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
      if (!NormalizeUSVString(memberSlot)) {
        JS_ReportOutOfMemory(cx);
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsUSVString().AssignLiteral(aData);
  }

private:
  inline RootedSpiderMonkeyInterface<ArrayBufferView>&
  RawSetAsArrayBufferView(JSContext* cx)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eArrayBufferView;
    return mUnion.mValue.mArrayBufferView.SetValue(cx);
  }

  inline RootedSpiderMonkeyInterface<ArrayBuffer>&
  RawSetAsArrayBuffer(JSContext* cx)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eArrayBuffer;
    return mUnion.mValue.mArrayBuffer.SetValue(cx);
  }

  inline binding_detail::FakeString<char16_t>&
  RawSetAsUSVString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUSVString;
    return mUnion.mValue.mUSVString.SetValue();
  }
};

class BlobOrArrayBufferViewOrArrayBufferOrFormDataOrURLSearchParamsOrReadableStreamOrUSVStringArgument
{
  BlobOrArrayBufferViewOrArrayBufferOrFormDataOrURLSearchParamsOrReadableStreamOrUSVString& mUnion;

  BlobOrArrayBufferViewOrArrayBufferOrFormDataOrURLSearchParamsOrReadableStreamOrUSVStringArgument(const BlobOrArrayBufferViewOrArrayBufferOrFormDataOrURLSearchParamsOrReadableStreamOrUSVStringArgument&) = delete;
  BlobOrArrayBufferViewOrArrayBufferOrFormDataOrURLSearchParamsOrReadableStreamOrUSVStringArgument& operator=(const BlobOrArrayBufferViewOrArrayBufferOrFormDataOrURLSearchParamsOrReadableStreamOrUSVStringArgument&) = delete;
public:
  explicit inline BlobOrArrayBufferViewOrArrayBufferOrFormDataOrURLSearchParamsOrReadableStreamOrUSVStringArgument(const BlobOrArrayBufferViewOrArrayBufferOrFormDataOrURLSearchParamsOrReadableStreamOrUSVString& aUnion)
    : mUnion(const_cast<BlobOrArrayBufferViewOrArrayBufferOrFormDataOrURLSearchParamsOrReadableStreamOrUSVString&>(aUnion))
  {
  }

  inline bool
  TrySetToBlob(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::Blob>& memberSlot = RawSetAsBlob();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::Blob, mozilla::dom::Blob>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyBlob();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToBlob(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToBlob(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToArrayBufferView(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      RootedSpiderMonkeyInterface<ArrayBufferView>& memberSlot = RawSetAsArrayBufferView(cx);
      if (!memberSlot.Init(&value.toObject())) {
        mUnion.DestroyArrayBufferView();
        tryNext = true;
        return true;
      }
      if (JS::IsArrayBufferViewShared(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_SHARED>("ArrayBufferView branch of (Blob or (ArrayBufferView or ArrayBuffer) or FormData or URLSearchParams or ReadableStream or USVString)");
        return false;
      }
      if (JS::IsLargeArrayBufferView(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_LARGE>("ArrayBufferView branch of (Blob or (ArrayBufferView or ArrayBuffer) or FormData or URLSearchParams or ReadableStream or USVString)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToArrayBufferView(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToArrayBufferView(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToArrayBuffer(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      RootedSpiderMonkeyInterface<ArrayBuffer>& memberSlot = RawSetAsArrayBuffer(cx);
      if (!memberSlot.Init(&value.toObject())) {
        mUnion.DestroyArrayBuffer();
        tryNext = true;
        return true;
      }
      if (JS::IsSharedArrayBufferObject(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_SHARED>("ArrayBuffer branch of (Blob or (ArrayBufferView or ArrayBuffer) or FormData or URLSearchParams or ReadableStream or USVString)");
        return false;
      }
      if (JS::IsLargeArrayBufferMaybeShared(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_LARGE>("ArrayBuffer branch of (Blob or (ArrayBufferView or ArrayBuffer) or FormData or URLSearchParams or ReadableStream or USVString)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToArrayBuffer(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToArrayBuffer(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToFormData(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::FormData>& memberSlot = RawSetAsFormData();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::FormData, mozilla::dom::FormData>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyFormData();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToFormData(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToFormData(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToURLSearchParams(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::URLSearchParams>& memberSlot = RawSetAsURLSearchParams();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::URLSearchParams, mozilla::dom::URLSearchParams>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyURLSearchParams();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToURLSearchParams(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToURLSearchParams(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToReadableStream(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      RootedSpiderMonkeyInterface<ReadableStream>& memberSlot = RawSetAsReadableStream(cx);
      if (!memberSlot.Init(&value.toObject())) {
        mUnion.DestroyReadableStream();
        tryNext = true;
        return true;
      }
    }
    return true;
  }

  inline bool
  TrySetToReadableStream(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToReadableStream(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToUSVString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsUSVString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
      if (!NormalizeUSVString(memberSlot)) {
        JS_ReportOutOfMemory(cx);
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsUSVString().AssignLiteral(aData);
  }

private:
  inline NonNull<mozilla::dom::Blob>&
  RawSetAsBlob()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eBlob;
    return mUnion.mValue.mBlob.SetValue();
  }

  inline RootedSpiderMonkeyInterface<ArrayBufferView>&
  RawSetAsArrayBufferView(JSContext* cx)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eArrayBufferView;
    return mUnion.mValue.mArrayBufferView.SetValue(cx);
  }

  inline RootedSpiderMonkeyInterface<ArrayBuffer>&
  RawSetAsArrayBuffer(JSContext* cx)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eArrayBuffer;
    return mUnion.mValue.mArrayBuffer.SetValue(cx);
  }

  inline NonNull<mozilla::dom::FormData>&
  RawSetAsFormData()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eFormData;
    return mUnion.mValue.mFormData.SetValue();
  }

  inline NonNull<mozilla::dom::URLSearchParams>&
  RawSetAsURLSearchParams()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eURLSearchParams;
    return mUnion.mValue.mURLSearchParams.SetValue();
  }

  inline RootedSpiderMonkeyInterface<ReadableStream>&
  RawSetAsReadableStream(JSContext* cx)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eReadableStream;
    return mUnion.mValue.mReadableStream.SetValue(cx);
  }

  inline binding_detail::FakeString<char16_t>&
  RawSetAsUSVString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUSVString;
    return mUnion.mValue.mUSVString.SetValue();
  }
};

class BlobOrArrayBufferViewOrArrayBufferOrFormDataOrURLSearchParamsOrUSVStringArgument
{
  BlobOrArrayBufferViewOrArrayBufferOrFormDataOrURLSearchParamsOrUSVString& mUnion;

  BlobOrArrayBufferViewOrArrayBufferOrFormDataOrURLSearchParamsOrUSVStringArgument(const BlobOrArrayBufferViewOrArrayBufferOrFormDataOrURLSearchParamsOrUSVStringArgument&) = delete;
  BlobOrArrayBufferViewOrArrayBufferOrFormDataOrURLSearchParamsOrUSVStringArgument& operator=(const BlobOrArrayBufferViewOrArrayBufferOrFormDataOrURLSearchParamsOrUSVStringArgument&) = delete;
public:
  explicit inline BlobOrArrayBufferViewOrArrayBufferOrFormDataOrURLSearchParamsOrUSVStringArgument(const BlobOrArrayBufferViewOrArrayBufferOrFormDataOrURLSearchParamsOrUSVString& aUnion)
    : mUnion(const_cast<BlobOrArrayBufferViewOrArrayBufferOrFormDataOrURLSearchParamsOrUSVString&>(aUnion))
  {
  }

  inline bool
  TrySetToBlob(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::Blob>& memberSlot = RawSetAsBlob();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::Blob, mozilla::dom::Blob>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyBlob();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToBlob(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToBlob(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToArrayBufferView(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      RootedSpiderMonkeyInterface<ArrayBufferView>& memberSlot = RawSetAsArrayBufferView(cx);
      if (!memberSlot.Init(&value.toObject())) {
        mUnion.DestroyArrayBufferView();
        tryNext = true;
        return true;
      }
      if (JS::IsArrayBufferViewShared(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_SHARED>("ArrayBufferView branch of (Blob or (ArrayBufferView or ArrayBuffer) or FormData or URLSearchParams or USVString)");
        return false;
      }
      if (JS::IsLargeArrayBufferView(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_LARGE>("ArrayBufferView branch of (Blob or (ArrayBufferView or ArrayBuffer) or FormData or URLSearchParams or USVString)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToArrayBufferView(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToArrayBufferView(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToArrayBuffer(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      RootedSpiderMonkeyInterface<ArrayBuffer>& memberSlot = RawSetAsArrayBuffer(cx);
      if (!memberSlot.Init(&value.toObject())) {
        mUnion.DestroyArrayBuffer();
        tryNext = true;
        return true;
      }
      if (JS::IsSharedArrayBufferObject(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_SHARED>("ArrayBuffer branch of (Blob or (ArrayBufferView or ArrayBuffer) or FormData or URLSearchParams or USVString)");
        return false;
      }
      if (JS::IsLargeArrayBufferMaybeShared(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_LARGE>("ArrayBuffer branch of (Blob or (ArrayBufferView or ArrayBuffer) or FormData or URLSearchParams or USVString)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToArrayBuffer(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToArrayBuffer(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToFormData(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::FormData>& memberSlot = RawSetAsFormData();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::FormData, mozilla::dom::FormData>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyFormData();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToFormData(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToFormData(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToURLSearchParams(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::URLSearchParams>& memberSlot = RawSetAsURLSearchParams();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::URLSearchParams, mozilla::dom::URLSearchParams>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyURLSearchParams();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToURLSearchParams(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToURLSearchParams(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToUSVString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsUSVString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
      if (!NormalizeUSVString(memberSlot)) {
        JS_ReportOutOfMemory(cx);
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsUSVString().AssignLiteral(aData);
  }

private:
  inline NonNull<mozilla::dom::Blob>&
  RawSetAsBlob()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eBlob;
    return mUnion.mValue.mBlob.SetValue();
  }

  inline RootedSpiderMonkeyInterface<ArrayBufferView>&
  RawSetAsArrayBufferView(JSContext* cx)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eArrayBufferView;
    return mUnion.mValue.mArrayBufferView.SetValue(cx);
  }

  inline RootedSpiderMonkeyInterface<ArrayBuffer>&
  RawSetAsArrayBuffer(JSContext* cx)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eArrayBuffer;
    return mUnion.mValue.mArrayBuffer.SetValue(cx);
  }

  inline NonNull<mozilla::dom::FormData>&
  RawSetAsFormData()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eFormData;
    return mUnion.mValue.mFormData.SetValue();
  }

  inline NonNull<mozilla::dom::URLSearchParams>&
  RawSetAsURLSearchParams()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eURLSearchParams;
    return mUnion.mValue.mURLSearchParams.SetValue();
  }

  inline binding_detail::FakeString<char16_t>&
  RawSetAsUSVString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUSVString;
    return mUnion.mValue.mUSVString.SetValue();
  }
};

class BlobOrDirectoryOrUSVStringArgument
{
  BlobOrDirectoryOrUSVString& mUnion;

  BlobOrDirectoryOrUSVStringArgument(const BlobOrDirectoryOrUSVStringArgument&) = delete;
  BlobOrDirectoryOrUSVStringArgument& operator=(const BlobOrDirectoryOrUSVStringArgument&) = delete;
public:
  explicit inline BlobOrDirectoryOrUSVStringArgument(const BlobOrDirectoryOrUSVString& aUnion)
    : mUnion(const_cast<BlobOrDirectoryOrUSVString&>(aUnion))
  {
  }

  inline bool
  TrySetToBlob(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::Blob>& memberSlot = RawSetAsBlob();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::Blob, mozilla::dom::Blob>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyBlob();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToBlob(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToBlob(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToDirectory(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::Directory>& memberSlot = RawSetAsDirectory();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::Directory, mozilla::dom::Directory>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyDirectory();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToDirectory(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToDirectory(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToUSVString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsUSVString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
      if (!NormalizeUSVString(memberSlot)) {
        JS_ReportOutOfMemory(cx);
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsUSVString().AssignLiteral(aData);
  }

private:
  inline NonNull<mozilla::dom::Blob>&
  RawSetAsBlob()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eBlob;
    return mUnion.mValue.mBlob.SetValue();
  }

  inline NonNull<mozilla::dom::Directory>&
  RawSetAsDirectory()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eDirectory;
    return mUnion.mValue.mDirectory.SetValue();
  }

  inline binding_detail::FakeString<char16_t>&
  RawSetAsUSVString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUSVString;
    return mUnion.mValue.mUSVString.SetValue();
  }
};

class BooleanOrConstrainBooleanParametersArgument
{
  BooleanOrConstrainBooleanParameters& mUnion;

  BooleanOrConstrainBooleanParametersArgument(const BooleanOrConstrainBooleanParametersArgument&) = delete;
  BooleanOrConstrainBooleanParametersArgument& operator=(const BooleanOrConstrainBooleanParametersArgument&) = delete;
public:
  explicit inline BooleanOrConstrainBooleanParametersArgument(const BooleanOrConstrainBooleanParameters& aUnion)
    : mUnion(const_cast<BooleanOrConstrainBooleanParameters&>(aUnion))
  {
  }

  inline bool
  TrySetToBoolean(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      bool& memberSlot = RawSetAsBoolean();
      if (!ValueToPrimitive<bool, eDefault>(cx, value, "Boolean branch of (boolean or ConstrainBooleanParameters)", &memberSlot)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToConstrainBooleanParameters(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FastConstrainBooleanParameters& memberSlot = RawSetAsConstrainBooleanParameters();
      if (!IsConvertibleToDictionary(value)) {
        mUnion.DestroyConstrainBooleanParameters();
        tryNext = true;
        return true;
      }
      if (!memberSlot.Init(cx, value, "ConstrainBooleanParameters branch of (boolean or ConstrainBooleanParameters)", passedToJSImpl)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToConstrainBooleanParameters(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToConstrainBooleanParameters(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline bool&
  RawSetAsBoolean()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eBoolean;
    return mUnion.mValue.mBoolean.SetValue();
  }

  inline binding_detail::FastConstrainBooleanParameters&
  RawSetAsConstrainBooleanParameters()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eConstrainBooleanParameters;
    return mUnion.mValue.mConstrainBooleanParameters.SetValue();
  }
};

class BooleanOrMediaTrackConstraintsArgument
{
  BooleanOrMediaTrackConstraints& mUnion;

  BooleanOrMediaTrackConstraintsArgument(const BooleanOrMediaTrackConstraintsArgument&) = delete;
  BooleanOrMediaTrackConstraintsArgument& operator=(const BooleanOrMediaTrackConstraintsArgument&) = delete;
public:
  explicit inline BooleanOrMediaTrackConstraintsArgument(const BooleanOrMediaTrackConstraints& aUnion)
    : mUnion(const_cast<BooleanOrMediaTrackConstraints&>(aUnion))
  {
  }

  inline bool
  TrySetToBoolean(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      bool& memberSlot = RawSetAsBoolean();
      if (!ValueToPrimitive<bool, eDefault>(cx, value, "Boolean branch of (boolean or MediaTrackConstraints)", &memberSlot)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToMediaTrackConstraints(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FastMediaTrackConstraints& memberSlot = RawSetAsMediaTrackConstraints();
      if (!IsConvertibleToDictionary(value)) {
        mUnion.DestroyMediaTrackConstraints();
        tryNext = true;
        return true;
      }
      if (!memberSlot.Init(cx, value, "MediaTrackConstraints branch of (boolean or MediaTrackConstraints)", passedToJSImpl)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToMediaTrackConstraints(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToMediaTrackConstraints(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline bool&
  RawSetAsBoolean()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eBoolean;
    return mUnion.mValue.mBoolean.SetValue();
  }

  inline binding_detail::FastMediaTrackConstraints&
  RawSetAsMediaTrackConstraints()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eMediaTrackConstraints;
    return mUnion.mValue.mMediaTrackConstraints.SetValue();
  }
};

class BooleanOrScrollIntoViewOptionsArgument
{
  BooleanOrScrollIntoViewOptions& mUnion;

  BooleanOrScrollIntoViewOptionsArgument(const BooleanOrScrollIntoViewOptionsArgument&) = delete;
  BooleanOrScrollIntoViewOptionsArgument& operator=(const BooleanOrScrollIntoViewOptionsArgument&) = delete;
public:
  explicit inline BooleanOrScrollIntoViewOptionsArgument(const BooleanOrScrollIntoViewOptions& aUnion)
    : mUnion(const_cast<BooleanOrScrollIntoViewOptions&>(aUnion))
  {
  }

  inline bool
  TrySetToBoolean(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      bool& memberSlot = RawSetAsBoolean();
      if (!ValueToPrimitive<bool, eDefault>(cx, value, "Boolean branch of (boolean or ScrollIntoViewOptions)", &memberSlot)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToScrollIntoViewOptions(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FastScrollIntoViewOptions& memberSlot = RawSetAsScrollIntoViewOptions();
      if (!IsConvertibleToDictionary(value)) {
        mUnion.DestroyScrollIntoViewOptions();
        tryNext = true;
        return true;
      }
      if (!memberSlot.Init(cx, value, "ScrollIntoViewOptions branch of (boolean or ScrollIntoViewOptions)", passedToJSImpl)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToScrollIntoViewOptions(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToScrollIntoViewOptions(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline bool&
  RawSetAsBoolean()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eBoolean;
    return mUnion.mValue.mBoolean.SetValue();
  }

  inline binding_detail::FastScrollIntoViewOptions&
  RawSetAsScrollIntoViewOptions()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eScrollIntoViewOptions;
    return mUnion.mValue.mScrollIntoViewOptions.SetValue();
  }
};

class ByteStringOrLongArgument
{
  ByteStringOrLong& mUnion;

  ByteStringOrLongArgument(const ByteStringOrLongArgument&) = delete;
  ByteStringOrLongArgument& operator=(const ByteStringOrLongArgument&) = delete;
public:
  explicit inline ByteStringOrLongArgument(const ByteStringOrLong& aUnion)
    : mUnion(const_cast<ByteStringOrLong&>(aUnion))
  {
  }

  inline bool
  TrySetToByteString(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      nsCString& memberSlot = RawSetAsByteString();
      if (!ConvertJSValueToByteString(cx, value, false, "ByteString branch of (ByteString or long)", memberSlot)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToByteString(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToByteString(cx, value, tryNext, passedToJSImpl);
  }

  template <int N>
  inline void
  SetStringLiteral(const nsCString::char_type (&aData)[N])
  {
    RawSetAsByteString().AssignLiteral(aData);
  }

  inline bool
  TrySetToLong(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      int32_t& memberSlot = RawSetAsLong();
      if (!ValueToPrimitive<int32_t, eDefault>(cx, value, "Long branch of (ByteString or long)", &memberSlot)) {
        return false;
      }
    }
    return true;
  }

private:
  inline nsCString&
  RawSetAsByteString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eByteString;
    return mUnion.mValue.mByteString.SetValue();
  }

  inline int32_t&
  RawSetAsLong()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eLong;
    return mUnion.mValue.mLong.SetValue();
  }
};

class ByteStringSequenceSequenceOrByteStringByteStringRecordArgument
{
  ByteStringSequenceSequenceOrByteStringByteStringRecord& mUnion;

  ByteStringSequenceSequenceOrByteStringByteStringRecordArgument(const ByteStringSequenceSequenceOrByteStringByteStringRecordArgument&) = delete;
  ByteStringSequenceSequenceOrByteStringByteStringRecordArgument& operator=(const ByteStringSequenceSequenceOrByteStringByteStringRecordArgument&) = delete;
public:
  explicit inline ByteStringSequenceSequenceOrByteStringByteStringRecordArgument(const ByteStringSequenceSequenceOrByteStringByteStringRecord& aUnion)
    : mUnion(const_cast<ByteStringSequenceSequenceOrByteStringByteStringRecord&>(aUnion))
  {
  }

  inline bool
  TrySetToByteStringSequenceSequence(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::AutoSequence<Sequence<nsCString>>& memberSlot = RawSetAsByteStringSequenceSequence();
      JS::ForOfIterator iter(cx);
      if (!iter.init(value, JS::ForOfIterator::AllowNonIterable)) {
        return false;
      }
      if (!iter.valueIsIterable()) {
        mUnion.DestroyByteStringSequenceSequence();
        tryNext = true;
        return true;
      }
      binding_detail::AutoSequence<Sequence<nsCString>> &arr = memberSlot;
      JS::Rooted<JS::Value> temp(cx);
      while (true) {
        bool done;
        if (!iter.next(&temp, &done)) {
          return false;
        }
        if (done) {
          break;
        }
        Sequence<nsCString>* slotPtr = arr.AppendElement(mozilla::fallible);
        if (!slotPtr) {
          JS_ReportOutOfMemory(cx);
          return false;
        }
        Sequence<nsCString>& slot = *slotPtr;
        if (temp.isObject()) {
          JS::ForOfIterator iter1(cx);
          if (!iter1.init(temp, JS::ForOfIterator::AllowNonIterable)) {
            return false;
          }
          if (!iter1.valueIsIterable()) {
            cx.ThrowErrorMessage<MSG_NOT_SEQUENCE>("Element of sequence<sequence<ByteString>> branch of (sequence<sequence<ByteString>> or record<ByteString, ByteString>)");
            return false;
          }
          Sequence<nsCString> &arr1 = slot;
          JS::Rooted<JS::Value> temp1(cx);
          while (true) {
            bool done1;
            if (!iter1.next(&temp1, &done1)) {
              return false;
            }
            if (done1) {
              break;
            }
            nsCString* slotPtr1 = arr1.AppendElement(mozilla::fallible);
            if (!slotPtr1) {
              JS_ReportOutOfMemory(cx);
              return false;
            }
            nsCString& slot1 = *slotPtr1;
            if (!ConvertJSValueToByteString(cx, temp1, false, "element of element of sequence<sequence<ByteString>> branch of (sequence<sequence<ByteString>> or record<ByteString, ByteString>)", slot1)) {
              return false;
            }
          }
        } else {
          cx.ThrowErrorMessage<MSG_NOT_SEQUENCE>("Element of sequence<sequence<ByteString>> branch of (sequence<sequence<ByteString>> or record<ByteString, ByteString>)");
          return false;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToByteStringSequenceSequence(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToByteStringSequenceSequence(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToByteStringByteStringRecord(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      Record<nsCString, nsCString>& memberSlot = RawSetAsByteStringByteStringRecord();
      auto& recordEntries = memberSlot.Entries();

      JS::Rooted<JSObject*> recordObj(cx, &value.toObject());
      JS::RootedVector<jsid> ids(cx);
      if (!js::GetPropertyKeys(cx, recordObj,
                               JSITER_OWNONLY | JSITER_HIDDEN | JSITER_SYMBOLS, &ids)) {
        return false;
      }
      if (!recordEntries.SetCapacity(ids.length(), mozilla::fallible)) {
        JS_ReportOutOfMemory(cx);
        return false;
      }
      JS::Rooted<JS::Value> propNameValue(cx);
      JS::Rooted<JS::Value> temp(cx);
      JS::Rooted<jsid> curId(cx);
      JS::Rooted<JS::Value> idVal(cx);
      // Use a hashset to keep track of ids seen, to avoid
      // introducing nasty O(N^2) behavior scanning for them all the
      // time.  Ideally we'd use a data structure with O(1) lookup
      // _and_ ordering for the MozMap, but we don't have one lying
      // around.
      nsTHashtable<nsCStringHashKey> idsSeen;
      for (size_t i = 0; i < ids.length(); ++i) {
        curId = ids[i];

        JS::Rooted<JS::PropertyDescriptor> desc(cx);
        if (!JS_GetOwnPropertyDescriptorById(cx, recordObj, curId,
                                             &desc)) {
          return false;
        }

        if (!desc.object() /* == undefined in spec terms */ ||
            !desc.enumerable()) {
          continue;
        }

        idVal = js::IdToValue(curId);
        nsCString propName;
        // This will just throw if idVal is a Symbol, like the spec says
        // to do.
        if (!ConvertJSValueToByteString(cx, idVal, "key of record<ByteString, ByteString> branch of (sequence<sequence<ByteString>> or record<ByteString, ByteString>)", propName)) {
          return false;
        }

        if (!JS_GetPropertyById(cx, recordObj, curId, &temp)) {
          return false;
        }

        Record<nsCString, nsCString>::EntryType* entry;
        if (!idsSeen.EnsureInserted(propName)) {
          // Find the existing entry.
          auto idx = recordEntries.IndexOf(propName);
          MOZ_ASSERT(idx != recordEntries.NoIndex,
                     "Why is it not found?");
          // Now blow it away to make it look like it was just added
          // to the array, because it's not obvious that it's
          // safe to write to its already-initialized mValue via our
          // normal codegen conversions.  For example, the value
          // could be a union and this would change its type, but
          // codegen assumes we won't do that.
          entry = recordEntries.ReconstructElementAt(idx);
        } else {
          // Safe to do an infallible append here, because we did a
          // SetCapacity above to the right capacity.
          entry = recordEntries.AppendElement();
        }
        entry->mKey = propName;
        nsCString& slot = entry->mValue;
        if (!ConvertJSValueToByteString(cx, temp, false, "value in record<ByteString, ByteString> branch of (sequence<sequence<ByteString>> or record<ByteString, ByteString>)", slot)) {
          return false;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToByteStringByteStringRecord(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToByteStringByteStringRecord(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline binding_detail::AutoSequence<Sequence<nsCString>>&
  RawSetAsByteStringSequenceSequence()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eByteStringSequenceSequence;
    return mUnion.mValue.mByteStringSequenceSequence.SetValue();
  }

  inline Record<nsCString, nsCString>&
  RawSetAsByteStringByteStringRecord()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eByteStringByteStringRecord;
    return mUnion.mValue.mByteStringByteStringRecord.SetValue();
  }
};

class CanvasPatternOrCanvasGradientArgument
{
  CanvasPatternOrCanvasGradient& mUnion;

  CanvasPatternOrCanvasGradientArgument(const CanvasPatternOrCanvasGradientArgument&) = delete;
  CanvasPatternOrCanvasGradientArgument& operator=(const CanvasPatternOrCanvasGradientArgument&) = delete;
public:
  explicit inline CanvasPatternOrCanvasGradientArgument(const CanvasPatternOrCanvasGradient& aUnion)
    : mUnion(const_cast<CanvasPatternOrCanvasGradient&>(aUnion))
  {
  }

  inline bool
  TrySetToCanvasPattern(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::CanvasPattern>& memberSlot = RawSetAsCanvasPattern();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::CanvasPattern, mozilla::dom::CanvasPattern>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyCanvasPattern();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToCanvasPattern(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToCanvasPattern(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToCanvasGradient(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::CanvasGradient>& memberSlot = RawSetAsCanvasGradient();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::CanvasGradient, mozilla::dom::CanvasGradient>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyCanvasGradient();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToCanvasGradient(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToCanvasGradient(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline NonNull<mozilla::dom::CanvasPattern>&
  RawSetAsCanvasPattern()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eCanvasPattern;
    return mUnion.mValue.mCanvasPattern.SetValue();
  }

  inline NonNull<mozilla::dom::CanvasGradient>&
  RawSetAsCanvasGradient()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eCanvasGradient;
    return mUnion.mValue.mCanvasGradient.SetValue();
  }
};

class CanvasPatternOrNullOrCanvasGradientArgument
{
  CanvasPatternOrNullOrCanvasGradient& mUnion;

  CanvasPatternOrNullOrCanvasGradientArgument(const CanvasPatternOrNullOrCanvasGradientArgument&) = delete;
  CanvasPatternOrNullOrCanvasGradientArgument& operator=(const CanvasPatternOrNullOrCanvasGradientArgument&) = delete;
public:
  explicit inline CanvasPatternOrNullOrCanvasGradientArgument(const CanvasPatternOrNullOrCanvasGradient& aUnion)
    : mUnion(const_cast<CanvasPatternOrNullOrCanvasGradient&>(aUnion))
  {
  }

  inline bool
  SetNull()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eNull;
    return true;
  }

  inline bool
  TrySetToCanvasPattern(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::CanvasPattern>& memberSlot = RawSetAsCanvasPattern();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::CanvasPattern, mozilla::dom::CanvasPattern>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyCanvasPattern();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToCanvasPattern(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToCanvasPattern(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToCanvasGradient(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::CanvasGradient>& memberSlot = RawSetAsCanvasGradient();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::CanvasGradient, mozilla::dom::CanvasGradient>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyCanvasGradient();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToCanvasGradient(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToCanvasGradient(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline NonNull<mozilla::dom::CanvasPattern>&
  RawSetAsCanvasPattern()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eCanvasPattern;
    return mUnion.mValue.mCanvasPattern.SetValue();
  }

  inline NonNull<mozilla::dom::CanvasGradient>&
  RawSetAsCanvasGradient()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eCanvasGradient;
    return mUnion.mValue.mCanvasGradient.SetValue();
  }
};

class ClientOrServiceWorkerOrMessagePortArgument
{
  ClientOrServiceWorkerOrMessagePort& mUnion;

  ClientOrServiceWorkerOrMessagePortArgument(const ClientOrServiceWorkerOrMessagePortArgument&) = delete;
  ClientOrServiceWorkerOrMessagePortArgument& operator=(const ClientOrServiceWorkerOrMessagePortArgument&) = delete;
public:
  explicit inline ClientOrServiceWorkerOrMessagePortArgument(const ClientOrServiceWorkerOrMessagePort& aUnion)
    : mUnion(const_cast<ClientOrServiceWorkerOrMessagePort&>(aUnion))
  {
  }

  inline bool
  TrySetToClient(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::Client>& memberSlot = RawSetAsClient();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::Client, mozilla::dom::Client>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyClient();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToClient(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToClient(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToServiceWorker(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::ServiceWorker>& memberSlot = RawSetAsServiceWorker();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::ServiceWorker, mozilla::dom::ServiceWorker>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyServiceWorker();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToServiceWorker(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToServiceWorker(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToMessagePort(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::MessagePort>& memberSlot = RawSetAsMessagePort();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::MessagePort, mozilla::dom::MessagePort>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyMessagePort();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToMessagePort(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToMessagePort(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline NonNull<mozilla::dom::Client>&
  RawSetAsClient()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eClient;
    return mUnion.mValue.mClient.SetValue();
  }

  inline NonNull<mozilla::dom::ServiceWorker>&
  RawSetAsServiceWorker()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eServiceWorker;
    return mUnion.mValue.mServiceWorker.SetValue();
  }

  inline NonNull<mozilla::dom::MessagePort>&
  RawSetAsMessagePort()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eMessagePort;
    return mUnion.mValue.mMessagePort.SetValue();
  }
};

class CompositeOperationOrAutoOrCompositeOperationOrAutoSequenceArgument
{
  CompositeOperationOrAutoOrCompositeOperationOrAutoSequence& mUnion;

  CompositeOperationOrAutoOrCompositeOperationOrAutoSequenceArgument(const CompositeOperationOrAutoOrCompositeOperationOrAutoSequenceArgument&) = delete;
  CompositeOperationOrAutoOrCompositeOperationOrAutoSequenceArgument& operator=(const CompositeOperationOrAutoOrCompositeOperationOrAutoSequenceArgument&) = delete;
public:
  explicit inline CompositeOperationOrAutoOrCompositeOperationOrAutoSequenceArgument(const CompositeOperationOrAutoOrCompositeOperationOrAutoSequence& aUnion)
    : mUnion(const_cast<CompositeOperationOrAutoOrCompositeOperationOrAutoSequence&>(aUnion))
  {
  }

  inline bool
  TrySetToCompositeOperationOrAuto(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      CompositeOperationOrAuto& memberSlot = RawSetAsCompositeOperationOrAuto();
      {
        int index;
        if (!FindEnumStringIndex<true>(cx, value, CompositeOperationOrAutoValues::strings, "CompositeOperationOrAuto", "CompositeOperationOrAuto branch of (CompositeOperationOrAuto or sequence<CompositeOperationOrAuto>)", &index)) {
          return false;
        }
        MOZ_ASSERT(index >= 0);
        memberSlot = static_cast<CompositeOperationOrAuto>(index);
      }
    }
    return true;
  }

  inline bool
  TrySetToCompositeOperationOrAuto(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToCompositeOperationOrAuto(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToCompositeOperationOrAutoSequence(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::AutoSequence<CompositeOperationOrAuto>& memberSlot = RawSetAsCompositeOperationOrAutoSequence();
      JS::ForOfIterator iter(cx);
      if (!iter.init(value, JS::ForOfIterator::AllowNonIterable)) {
        return false;
      }
      if (!iter.valueIsIterable()) {
        mUnion.DestroyCompositeOperationOrAutoSequence();
        tryNext = true;
        return true;
      }
      binding_detail::AutoSequence<CompositeOperationOrAuto> &arr = memberSlot;
      JS::Rooted<JS::Value> temp(cx);
      while (true) {
        bool done;
        if (!iter.next(&temp, &done)) {
          return false;
        }
        if (done) {
          break;
        }
        CompositeOperationOrAuto* slotPtr = arr.AppendElement(mozilla::fallible);
        if (!slotPtr) {
          JS_ReportOutOfMemory(cx);
          return false;
        }
        CompositeOperationOrAuto& slot = *slotPtr;
        {
          int index;
          if (!FindEnumStringIndex<true>(cx, temp, CompositeOperationOrAutoValues::strings, "CompositeOperationOrAuto", "element of sequence<CompositeOperationOrAuto> branch of (CompositeOperationOrAuto or sequence<CompositeOperationOrAuto>)", &index)) {
            return false;
          }
          MOZ_ASSERT(index >= 0);
          slot = static_cast<CompositeOperationOrAuto>(index);
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToCompositeOperationOrAutoSequence(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToCompositeOperationOrAutoSequence(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline CompositeOperationOrAuto&
  RawSetAsCompositeOperationOrAuto()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eCompositeOperationOrAuto;
    return mUnion.mValue.mCompositeOperationOrAuto.SetValue();
  }

  inline binding_detail::AutoSequence<CompositeOperationOrAuto>&
  RawSetAsCompositeOperationOrAutoSequence()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eCompositeOperationOrAutoSequence;
    return mUnion.mValue.mCompositeOperationOrAutoSequence.SetValue();
  }
};

class DocumentOrBlobOrArrayBufferViewOrArrayBufferOrFormDataOrURLSearchParamsOrUSVStringArgument
{
  DocumentOrBlobOrArrayBufferViewOrArrayBufferOrFormDataOrURLSearchParamsOrUSVString& mUnion;

  DocumentOrBlobOrArrayBufferViewOrArrayBufferOrFormDataOrURLSearchParamsOrUSVStringArgument(const DocumentOrBlobOrArrayBufferViewOrArrayBufferOrFormDataOrURLSearchParamsOrUSVStringArgument&) = delete;
  DocumentOrBlobOrArrayBufferViewOrArrayBufferOrFormDataOrURLSearchParamsOrUSVStringArgument& operator=(const DocumentOrBlobOrArrayBufferViewOrArrayBufferOrFormDataOrURLSearchParamsOrUSVStringArgument&) = delete;
public:
  explicit inline DocumentOrBlobOrArrayBufferViewOrArrayBufferOrFormDataOrURLSearchParamsOrUSVStringArgument(const DocumentOrBlobOrArrayBufferViewOrArrayBufferOrFormDataOrURLSearchParamsOrUSVString& aUnion)
    : mUnion(const_cast<DocumentOrBlobOrArrayBufferViewOrArrayBufferOrFormDataOrURLSearchParamsOrUSVString&>(aUnion))
  {
  }

  inline bool
  TrySetToDocument(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::Document>& memberSlot = RawSetAsDocument();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::Document, mozilla::dom::Document>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyDocument();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToDocument(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToDocument(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToBlob(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::Blob>& memberSlot = RawSetAsBlob();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::Blob, mozilla::dom::Blob>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyBlob();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToBlob(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToBlob(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToArrayBufferView(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      RootedSpiderMonkeyInterface<ArrayBufferView>& memberSlot = RawSetAsArrayBufferView(cx);
      if (!memberSlot.Init(&value.toObject())) {
        mUnion.DestroyArrayBufferView();
        tryNext = true;
        return true;
      }
      if (JS::IsArrayBufferViewShared(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_SHARED>("ArrayBufferView branch of (Document or (Blob or (ArrayBufferView or ArrayBuffer) or FormData or URLSearchParams or USVString))");
        return false;
      }
      if (JS::IsLargeArrayBufferView(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_LARGE>("ArrayBufferView branch of (Document or (Blob or (ArrayBufferView or ArrayBuffer) or FormData or URLSearchParams or USVString))");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToArrayBufferView(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToArrayBufferView(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToArrayBuffer(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      RootedSpiderMonkeyInterface<ArrayBuffer>& memberSlot = RawSetAsArrayBuffer(cx);
      if (!memberSlot.Init(&value.toObject())) {
        mUnion.DestroyArrayBuffer();
        tryNext = true;
        return true;
      }
      if (JS::IsSharedArrayBufferObject(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_SHARED>("ArrayBuffer branch of (Document or (Blob or (ArrayBufferView or ArrayBuffer) or FormData or URLSearchParams or USVString))");
        return false;
      }
      if (JS::IsLargeArrayBufferMaybeShared(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_LARGE>("ArrayBuffer branch of (Document or (Blob or (ArrayBufferView or ArrayBuffer) or FormData or URLSearchParams or USVString))");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToArrayBuffer(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToArrayBuffer(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToFormData(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::FormData>& memberSlot = RawSetAsFormData();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::FormData, mozilla::dom::FormData>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyFormData();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToFormData(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToFormData(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToURLSearchParams(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::URLSearchParams>& memberSlot = RawSetAsURLSearchParams();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::URLSearchParams, mozilla::dom::URLSearchParams>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyURLSearchParams();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToURLSearchParams(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToURLSearchParams(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToUSVString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsUSVString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
      if (!NormalizeUSVString(memberSlot)) {
        JS_ReportOutOfMemory(cx);
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsUSVString().AssignLiteral(aData);
  }

private:
  inline NonNull<mozilla::dom::Document>&
  RawSetAsDocument()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eDocument;
    return mUnion.mValue.mDocument.SetValue();
  }

  inline NonNull<mozilla::dom::Blob>&
  RawSetAsBlob()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eBlob;
    return mUnion.mValue.mBlob.SetValue();
  }

  inline RootedSpiderMonkeyInterface<ArrayBufferView>&
  RawSetAsArrayBufferView(JSContext* cx)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eArrayBufferView;
    return mUnion.mValue.mArrayBufferView.SetValue(cx);
  }

  inline RootedSpiderMonkeyInterface<ArrayBuffer>&
  RawSetAsArrayBuffer(JSContext* cx)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eArrayBuffer;
    return mUnion.mValue.mArrayBuffer.SetValue(cx);
  }

  inline NonNull<mozilla::dom::FormData>&
  RawSetAsFormData()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eFormData;
    return mUnion.mValue.mFormData.SetValue();
  }

  inline NonNull<mozilla::dom::URLSearchParams>&
  RawSetAsURLSearchParams()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eURLSearchParams;
    return mUnion.mValue.mURLSearchParams.SetValue();
  }

  inline binding_detail::FakeString<char16_t>&
  RawSetAsUSVString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUSVString;
    return mUnion.mValue.mUSVString.SetValue();
  }
};

class DoubleOrAutoKeywordArgument
{
  DoubleOrAutoKeyword& mUnion;

  DoubleOrAutoKeywordArgument(const DoubleOrAutoKeywordArgument&) = delete;
  DoubleOrAutoKeywordArgument& operator=(const DoubleOrAutoKeywordArgument&) = delete;
public:
  explicit inline DoubleOrAutoKeywordArgument(const DoubleOrAutoKeyword& aUnion)
    : mUnion(const_cast<DoubleOrAutoKeyword&>(aUnion))
  {
  }

  inline bool
  TrySetToDouble(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      double& memberSlot = RawSetAsDouble();
      if (!ValueToPrimitive<double, eDefault>(cx, value, "Double branch of (double or AutoKeyword)", &memberSlot)) {
        return false;
      } else if (!mozilla::IsFinite(memberSlot)) {
        cx.ThrowErrorMessage<MSG_NOT_FINITE>("Double branch of (double or AutoKeyword)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToDouble(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToDouble(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToAutoKeyword(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      AutoKeyword& memberSlot = RawSetAsAutoKeyword();
      {
        int index;
        if (!FindEnumStringIndex<true>(cx, value, AutoKeywordValues::strings, "AutoKeyword", "AutoKeyword branch of (double or AutoKeyword)", &index)) {
          return false;
        }
        MOZ_ASSERT(index >= 0);
        memberSlot = static_cast<AutoKeyword>(index);
      }
    }
    return true;
  }

  inline bool
  TrySetToAutoKeyword(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToAutoKeyword(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline double&
  RawSetAsDouble()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eDouble;
    return mUnion.mValue.mDouble.SetValue();
  }

  inline AutoKeyword&
  RawSetAsAutoKeyword()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eAutoKeyword;
    return mUnion.mValue.mAutoKeyword.SetValue();
  }
};

class DoubleOrByteStringArgument
{
  DoubleOrByteString& mUnion;

  DoubleOrByteStringArgument(const DoubleOrByteStringArgument&) = delete;
  DoubleOrByteStringArgument& operator=(const DoubleOrByteStringArgument&) = delete;
public:
  explicit inline DoubleOrByteStringArgument(const DoubleOrByteString& aUnion)
    : mUnion(const_cast<DoubleOrByteString&>(aUnion))
  {
  }

  inline bool
  TrySetToDouble(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      double& memberSlot = RawSetAsDouble();
      if (!ValueToPrimitive<double, eDefault>(cx, value, "Double branch of (double or ByteString)", &memberSlot)) {
        return false;
      } else if (!mozilla::IsFinite(memberSlot)) {
        cx.ThrowErrorMessage<MSG_NOT_FINITE>("Double branch of (double or ByteString)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToDouble(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToDouble(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToByteString(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      nsCString& memberSlot = RawSetAsByteString();
      if (!ConvertJSValueToByteString(cx, value, false, "ByteString branch of (double or ByteString)", memberSlot)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToByteString(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToByteString(cx, value, tryNext, passedToJSImpl);
  }

  template <int N>
  inline void
  SetStringLiteral(const nsCString::char_type (&aData)[N])
  {
    RawSetAsByteString().AssignLiteral(aData);
  }

private:
  inline double&
  RawSetAsDouble()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eDouble;
    return mUnion.mValue.mDouble.SetValue();
  }

  inline nsCString&
  RawSetAsByteString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eByteString;
    return mUnion.mValue.mByteString.SetValue();
  }
};

class DoubleOrConstrainDoubleRangeArgument
{
  DoubleOrConstrainDoubleRange& mUnion;

  DoubleOrConstrainDoubleRangeArgument(const DoubleOrConstrainDoubleRangeArgument&) = delete;
  DoubleOrConstrainDoubleRangeArgument& operator=(const DoubleOrConstrainDoubleRangeArgument&) = delete;
public:
  explicit inline DoubleOrConstrainDoubleRangeArgument(const DoubleOrConstrainDoubleRange& aUnion)
    : mUnion(const_cast<DoubleOrConstrainDoubleRange&>(aUnion))
  {
  }

  inline bool
  TrySetToDouble(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      double& memberSlot = RawSetAsDouble();
      if (!ValueToPrimitive<double, eDefault>(cx, value, "Double branch of (double or ConstrainDoubleRange)", &memberSlot)) {
        return false;
      } else if (!mozilla::IsFinite(memberSlot)) {
        cx.ThrowErrorMessage<MSG_NOT_FINITE>("Double branch of (double or ConstrainDoubleRange)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToDouble(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToDouble(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToConstrainDoubleRange(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FastConstrainDoubleRange& memberSlot = RawSetAsConstrainDoubleRange();
      if (!IsConvertibleToDictionary(value)) {
        mUnion.DestroyConstrainDoubleRange();
        tryNext = true;
        return true;
      }
      if (!memberSlot.Init(cx, value, "ConstrainDoubleRange branch of (double or ConstrainDoubleRange)", passedToJSImpl)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToConstrainDoubleRange(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToConstrainDoubleRange(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline double&
  RawSetAsDouble()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eDouble;
    return mUnion.mValue.mDouble.SetValue();
  }

  inline binding_detail::FastConstrainDoubleRange&
  RawSetAsConstrainDoubleRange()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eConstrainDoubleRange;
    return mUnion.mValue.mConstrainDoubleRange.SetValue();
  }
};

class DoubleOrDoubleSequenceArgument
{
  DoubleOrDoubleSequence& mUnion;

  DoubleOrDoubleSequenceArgument(const DoubleOrDoubleSequenceArgument&) = delete;
  DoubleOrDoubleSequenceArgument& operator=(const DoubleOrDoubleSequenceArgument&) = delete;
public:
  explicit inline DoubleOrDoubleSequenceArgument(const DoubleOrDoubleSequence& aUnion)
    : mUnion(const_cast<DoubleOrDoubleSequence&>(aUnion))
  {
  }

  inline bool
  TrySetToDouble(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      double& memberSlot = RawSetAsDouble();
      if (!ValueToPrimitive<double, eDefault>(cx, value, "Double branch of (double or sequence<double>)", &memberSlot)) {
        return false;
      } else if (!mozilla::IsFinite(memberSlot)) {
        cx.ThrowErrorMessage<MSG_NOT_FINITE>("Double branch of (double or sequence<double>)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToDouble(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToDouble(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToDoubleSequence(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::AutoSequence<double>& memberSlot = RawSetAsDoubleSequence();
      JS::ForOfIterator iter(cx);
      if (!iter.init(value, JS::ForOfIterator::AllowNonIterable)) {
        return false;
      }
      if (!iter.valueIsIterable()) {
        mUnion.DestroyDoubleSequence();
        tryNext = true;
        return true;
      }
      binding_detail::AutoSequence<double> &arr = memberSlot;
      JS::Rooted<JS::Value> temp(cx);
      while (true) {
        bool done;
        if (!iter.next(&temp, &done)) {
          return false;
        }
        if (done) {
          break;
        }
        double* slotPtr = arr.AppendElement(mozilla::fallible);
        if (!slotPtr) {
          JS_ReportOutOfMemory(cx);
          return false;
        }
        double& slot = *slotPtr;
        if (!ValueToPrimitive<double, eDefault>(cx, temp, "Element of sequence<double> branch of (double or sequence<double>)", &slot)) {
          return false;
        } else if (!mozilla::IsFinite(slot)) {
          cx.ThrowErrorMessage<MSG_NOT_FINITE>("Element of sequence<double> branch of (double or sequence<double>)");
          return false;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToDoubleSequence(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToDoubleSequence(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline double&
  RawSetAsDouble()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eDouble;
    return mUnion.mValue.mDouble.SetValue();
  }

  inline binding_detail::AutoSequence<double>&
  RawSetAsDoubleSequence()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eDoubleSequence;
    return mUnion.mValue.mDoubleSequence.SetValue();
  }
};

class DoubleOrNullOrDoubleOrNullSequenceArgument
{
  DoubleOrNullOrDoubleOrNullSequence& mUnion;

  DoubleOrNullOrDoubleOrNullSequenceArgument(const DoubleOrNullOrDoubleOrNullSequenceArgument&) = delete;
  DoubleOrNullOrDoubleOrNullSequenceArgument& operator=(const DoubleOrNullOrDoubleOrNullSequenceArgument&) = delete;
public:
  explicit inline DoubleOrNullOrDoubleOrNullSequenceArgument(const DoubleOrNullOrDoubleOrNullSequence& aUnion)
    : mUnion(const_cast<DoubleOrNullOrDoubleOrNullSequence&>(aUnion))
  {
  }

  inline bool
  SetNull()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eNull;
    return true;
  }

  inline bool
  TrySetToDouble(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      double& memberSlot = RawSetAsDouble();
      if (!ValueToPrimitive<double, eDefault>(cx, value, "Double branch of (double? or sequence<double?>)", &memberSlot)) {
        return false;
      } else if (!mozilla::IsFinite(memberSlot)) {
        cx.ThrowErrorMessage<MSG_NOT_FINITE>("Double branch of (double? or sequence<double?>)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToDouble(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToDouble(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToDoubleOrNullSequence(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::AutoSequence<Nullable<double>>& memberSlot = RawSetAsDoubleOrNullSequence();
      JS::ForOfIterator iter(cx);
      if (!iter.init(value, JS::ForOfIterator::AllowNonIterable)) {
        return false;
      }
      if (!iter.valueIsIterable()) {
        mUnion.DestroyDoubleOrNullSequence();
        tryNext = true;
        return true;
      }
      binding_detail::AutoSequence<Nullable<double>> &arr = memberSlot;
      JS::Rooted<JS::Value> temp(cx);
      while (true) {
        bool done;
        if (!iter.next(&temp, &done)) {
          return false;
        }
        if (done) {
          break;
        }
        Nullable<double>* slotPtr = arr.AppendElement(mozilla::fallible);
        if (!slotPtr) {
          JS_ReportOutOfMemory(cx);
          return false;
        }
        Nullable<double>& slot = *slotPtr;
        if (temp.isNullOrUndefined()) {
          slot.SetNull();
        } else if (!ValueToPrimitive<double, eDefault>(cx, temp, "Element of sequence<double?> branch of (double? or sequence<double?>)", &slot.SetValue())) {
          return false;
        } else if (!mozilla::IsFinite(slot.Value())) {
          cx.ThrowErrorMessage<MSG_NOT_FINITE>("Element of sequence<double?> branch of (double? or sequence<double?>)");
          return false;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToDoubleOrNullSequence(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToDoubleOrNullSequence(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline double&
  RawSetAsDouble()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eDouble;
    return mUnion.mValue.mDouble.SetValue();
  }

  inline binding_detail::AutoSequence<Nullable<double>>&
  RawSetAsDoubleOrNullSequence()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eDoubleOrNullSequence;
    return mUnion.mValue.mDoubleOrNullSequence.SetValue();
  }
};

class DoubleOrStringArgument
{
  DoubleOrString& mUnion;

  DoubleOrStringArgument(const DoubleOrStringArgument&) = delete;
  DoubleOrStringArgument& operator=(const DoubleOrStringArgument&) = delete;
public:
  explicit inline DoubleOrStringArgument(const DoubleOrString& aUnion)
    : mUnion(const_cast<DoubleOrString&>(aUnion))
  {
  }

  inline bool
  TrySetToDouble(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      double& memberSlot = RawSetAsDouble();
      if (!ValueToPrimitive<double, eDefault>(cx, value, "Double branch of (double or DOMString)", &memberSlot)) {
        return false;
      } else if (!mozilla::IsFinite(memberSlot)) {
        cx.ThrowErrorMessage<MSG_NOT_FINITE>("Double branch of (double or DOMString)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToDouble(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToDouble(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsString().AssignLiteral(aData);
  }

private:
  inline double&
  RawSetAsDouble()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eDouble;
    return mUnion.mValue.mDouble.SetValue();
  }

  inline binding_detail::FakeString<char16_t>&
  RawSetAsString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eString;
    return mUnion.mValue.mString.SetValue();
  }
};

class DoubleOrSupportedTypeArgument
{
  DoubleOrSupportedType& mUnion;

  DoubleOrSupportedTypeArgument(const DoubleOrSupportedTypeArgument&) = delete;
  DoubleOrSupportedTypeArgument& operator=(const DoubleOrSupportedTypeArgument&) = delete;
public:
  explicit inline DoubleOrSupportedTypeArgument(const DoubleOrSupportedType& aUnion)
    : mUnion(const_cast<DoubleOrSupportedType&>(aUnion))
  {
  }

  inline bool
  TrySetToDouble(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      double& memberSlot = RawSetAsDouble();
      if (!ValueToPrimitive<double, eDefault>(cx, value, "Double branch of (double or SupportedType)", &memberSlot)) {
        return false;
      } else if (!mozilla::IsFinite(memberSlot)) {
        cx.ThrowErrorMessage<MSG_NOT_FINITE>("Double branch of (double or SupportedType)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToDouble(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToDouble(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToSupportedType(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      SupportedType& memberSlot = RawSetAsSupportedType();
      {
        int index;
        if (!FindEnumStringIndex<true>(cx, value, SupportedTypeValues::strings, "SupportedType", "SupportedType branch of (double or SupportedType)", &index)) {
          return false;
        }
        MOZ_ASSERT(index >= 0);
        memberSlot = static_cast<SupportedType>(index);
      }
    }
    return true;
  }

  inline bool
  TrySetToSupportedType(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToSupportedType(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline double&
  RawSetAsDouble()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eDouble;
    return mUnion.mValue.mDouble.SetValue();
  }

  inline SupportedType&
  RawSetAsSupportedType()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eSupportedType;
    return mUnion.mValue.mSupportedType.SetValue();
  }
};

class DoubleOrUSVStringArgument
{
  DoubleOrUSVString& mUnion;

  DoubleOrUSVStringArgument(const DoubleOrUSVStringArgument&) = delete;
  DoubleOrUSVStringArgument& operator=(const DoubleOrUSVStringArgument&) = delete;
public:
  explicit inline DoubleOrUSVStringArgument(const DoubleOrUSVString& aUnion)
    : mUnion(const_cast<DoubleOrUSVString&>(aUnion))
  {
  }

  inline bool
  TrySetToDouble(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      double& memberSlot = RawSetAsDouble();
      if (!ValueToPrimitive<double, eDefault>(cx, value, "Double branch of (double or USVString)", &memberSlot)) {
        return false;
      } else if (!mozilla::IsFinite(memberSlot)) {
        cx.ThrowErrorMessage<MSG_NOT_FINITE>("Double branch of (double or USVString)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToDouble(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToDouble(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToUSVString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsUSVString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
      if (!NormalizeUSVString(memberSlot)) {
        JS_ReportOutOfMemory(cx);
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsUSVString().AssignLiteral(aData);
  }

private:
  inline double&
  RawSetAsDouble()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eDouble;
    return mUnion.mValue.mDouble.SetValue();
  }

  inline binding_detail::FakeString<char16_t>&
  RawSetAsUSVString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUSVString;
    return mUnion.mValue.mUSVString.SetValue();
  }
};

class DoubleOrUTF8StringArgument
{
  DoubleOrUTF8String& mUnion;

  DoubleOrUTF8StringArgument(const DoubleOrUTF8StringArgument&) = delete;
  DoubleOrUTF8StringArgument& operator=(const DoubleOrUTF8StringArgument&) = delete;
public:
  explicit inline DoubleOrUTF8StringArgument(const DoubleOrUTF8String& aUnion)
    : mUnion(const_cast<DoubleOrUTF8String&>(aUnion))
  {
  }

  inline bool
  TrySetToDouble(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      double& memberSlot = RawSetAsDouble();
      if (!ValueToPrimitive<double, eDefault>(cx, value, "Double branch of (double or USVString)", &memberSlot)) {
        return false;
      } else if (!mozilla::IsFinite(memberSlot)) {
        cx.ThrowErrorMessage<MSG_NOT_FINITE>("Double branch of (double or USVString)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToDouble(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToDouble(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToUTF8String(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char>& memberSlot = RawSetAsUTF8String();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsCString::char_type (&aData)[N])
  {
    RawSetAsUTF8String().AssignLiteral(aData);
  }

private:
  inline double&
  RawSetAsDouble()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eDouble;
    return mUnion.mValue.mDouble.SetValue();
  }

  inline binding_detail::FakeString<char>&
  RawSetAsUTF8String()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUTF8String;
    return mUnion.mValue.mUTF8String.SetValue();
  }
};

class DoubleSequenceOrGPUColorDictArgument
{
  DoubleSequenceOrGPUColorDict& mUnion;

  DoubleSequenceOrGPUColorDictArgument(const DoubleSequenceOrGPUColorDictArgument&) = delete;
  DoubleSequenceOrGPUColorDictArgument& operator=(const DoubleSequenceOrGPUColorDictArgument&) = delete;
public:
  explicit inline DoubleSequenceOrGPUColorDictArgument(const DoubleSequenceOrGPUColorDict& aUnion)
    : mUnion(const_cast<DoubleSequenceOrGPUColorDict&>(aUnion))
  {
  }

  inline bool
  TrySetToDoubleSequence(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::AutoSequence<double>& memberSlot = RawSetAsDoubleSequence();
      JS::ForOfIterator iter(cx);
      if (!iter.init(value, JS::ForOfIterator::AllowNonIterable)) {
        return false;
      }
      if (!iter.valueIsIterable()) {
        mUnion.DestroyDoubleSequence();
        tryNext = true;
        return true;
      }
      binding_detail::AutoSequence<double> &arr = memberSlot;
      JS::Rooted<JS::Value> temp(cx);
      while (true) {
        bool done;
        if (!iter.next(&temp, &done)) {
          return false;
        }
        if (done) {
          break;
        }
        double* slotPtr = arr.AppendElement(mozilla::fallible);
        if (!slotPtr) {
          JS_ReportOutOfMemory(cx);
          return false;
        }
        double& slot = *slotPtr;
        if (!ValueToPrimitive<double, eDefault>(cx, temp, "Element of sequence<double> branch of (sequence<double> or GPUColorDict)", &slot)) {
          return false;
        } else if (!mozilla::IsFinite(slot)) {
          cx.ThrowErrorMessage<MSG_NOT_FINITE>("Element of sequence<double> branch of (sequence<double> or GPUColorDict)");
          return false;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToDoubleSequence(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToDoubleSequence(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToGPUColorDict(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FastGPUColorDict& memberSlot = RawSetAsGPUColorDict();
      if (!IsConvertibleToDictionary(value)) {
        mUnion.DestroyGPUColorDict();
        tryNext = true;
        return true;
      }
      if (!memberSlot.Init(cx, value, "GPUColorDict branch of (sequence<double> or GPUColorDict)", passedToJSImpl)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToGPUColorDict(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToGPUColorDict(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline binding_detail::AutoSequence<double>&
  RawSetAsDoubleSequence()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eDoubleSequence;
    return mUnion.mValue.mDoubleSequence.SetValue();
  }

  inline binding_detail::FastGPUColorDict&
  RawSetAsGPUColorDict()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eGPUColorDict;
    return mUnion.mValue.mGPUColorDict.SetValue();
  }
};

class ElementCreationOptionsOrStringArgument
{
  ElementCreationOptionsOrString& mUnion;

  ElementCreationOptionsOrStringArgument(const ElementCreationOptionsOrStringArgument&) = delete;
  ElementCreationOptionsOrStringArgument& operator=(const ElementCreationOptionsOrStringArgument&) = delete;
public:
  explicit inline ElementCreationOptionsOrStringArgument(const ElementCreationOptionsOrString& aUnion)
    : mUnion(const_cast<ElementCreationOptionsOrString&>(aUnion))
  {
  }

  inline bool
  TrySetToElementCreationOptions(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FastElementCreationOptions& memberSlot = RawSetAsElementCreationOptions();
      if (!IsConvertibleToDictionary(value)) {
        mUnion.DestroyElementCreationOptions();
        tryNext = true;
        return true;
      }
      if (!memberSlot.Init(cx, value, "ElementCreationOptions branch of (ElementCreationOptions or DOMString)", passedToJSImpl)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToElementCreationOptions(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToElementCreationOptions(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsString().AssignLiteral(aData);
  }

private:
  inline binding_detail::FastElementCreationOptions&
  RawSetAsElementCreationOptions()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eElementCreationOptions;
    return mUnion.mValue.mElementCreationOptions.SetValue();
  }

  inline binding_detail::FakeString<char16_t>&
  RawSetAsString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eString;
    return mUnion.mValue.mString.SetValue();
  }
};

class ElementOrDocumentArgument
{
  ElementOrDocument& mUnion;

  ElementOrDocumentArgument(const ElementOrDocumentArgument&) = delete;
  ElementOrDocumentArgument& operator=(const ElementOrDocumentArgument&) = delete;
public:
  explicit inline ElementOrDocumentArgument(const ElementOrDocument& aUnion)
    : mUnion(const_cast<ElementOrDocument&>(aUnion))
  {
  }

  inline bool
  TrySetToElement(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::Element>& memberSlot = RawSetAsElement();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::Element, mozilla::dom::Element>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyElement();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToElement(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToElement(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToDocument(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::Document>& memberSlot = RawSetAsDocument();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::Document, mozilla::dom::Document>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyDocument();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToDocument(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToDocument(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline NonNull<mozilla::dom::Element>&
  RawSetAsElement()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eElement;
    return mUnion.mValue.mElement.SetValue();
  }

  inline NonNull<mozilla::dom::Document>&
  RawSetAsDocument()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eDocument;
    return mUnion.mValue.mDocument.SetValue();
  }
};

class EventListenerOptionsOrBooleanArgument
{
  EventListenerOptionsOrBoolean& mUnion;

  EventListenerOptionsOrBooleanArgument(const EventListenerOptionsOrBooleanArgument&) = delete;
  EventListenerOptionsOrBooleanArgument& operator=(const EventListenerOptionsOrBooleanArgument&) = delete;
public:
  explicit inline EventListenerOptionsOrBooleanArgument(const EventListenerOptionsOrBoolean& aUnion)
    : mUnion(const_cast<EventListenerOptionsOrBoolean&>(aUnion))
  {
  }

  inline bool
  TrySetToEventListenerOptions(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FastEventListenerOptions& memberSlot = RawSetAsEventListenerOptions();
      if (!IsConvertibleToDictionary(value)) {
        mUnion.DestroyEventListenerOptions();
        tryNext = true;
        return true;
      }
      if (!memberSlot.Init(cx, value, "EventListenerOptions branch of (EventListenerOptions or boolean)", passedToJSImpl)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToEventListenerOptions(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToEventListenerOptions(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToBoolean(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      bool& memberSlot = RawSetAsBoolean();
      if (!ValueToPrimitive<bool, eDefault>(cx, value, "Boolean branch of (EventListenerOptions or boolean)", &memberSlot)) {
        return false;
      }
    }
    return true;
  }

private:
  inline binding_detail::FastEventListenerOptions&
  RawSetAsEventListenerOptions()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eEventListenerOptions;
    return mUnion.mValue.mEventListenerOptions.SetValue();
  }

  inline bool&
  RawSetAsBoolean()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eBoolean;
    return mUnion.mValue.mBoolean.SetValue();
  }
};

class EventOrStringArgument
{
  EventOrString& mUnion;

  EventOrStringArgument(const EventOrStringArgument&) = delete;
  EventOrStringArgument& operator=(const EventOrStringArgument&) = delete;
public:
  explicit inline EventOrStringArgument(const EventOrString& aUnion)
    : mUnion(const_cast<EventOrString&>(aUnion))
  {
  }

  inline bool
  TrySetToEvent(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::Event>& memberSlot = RawSetAsEvent();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::Event, mozilla::dom::Event>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyEvent();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToEvent(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToEvent(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsString().AssignLiteral(aData);
  }

private:
  inline NonNull<mozilla::dom::Event>&
  RawSetAsEvent()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eEvent;
    return mUnion.mValue.mEvent.SetValue();
  }

  inline binding_detail::FakeString<char16_t>&
  RawSetAsString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eString;
    return mUnion.mValue.mString.SetValue();
  }
};

class ExternalOrWindowProxyArgument
{
  ExternalOrWindowProxy& mUnion;

  ExternalOrWindowProxyArgument(const ExternalOrWindowProxyArgument&) = delete;
  ExternalOrWindowProxyArgument& operator=(const ExternalOrWindowProxyArgument&) = delete;
public:
  explicit inline ExternalOrWindowProxyArgument(const ExternalOrWindowProxy& aUnion)
    : mUnion(const_cast<ExternalOrWindowProxy&>(aUnion))
  {
  }

  inline bool
  TrySetToExternal(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::External>& memberSlot = RawSetAsExternal();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::External, mozilla::dom::External>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyExternal();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToExternal(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToExternal(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToWindowProxy(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      mozilla::dom::WindowProxyHolder& memberSlot = RawSetAsWindowProxy();
      JS::Rooted<JSObject*> source(cx, &value.toObject());
      if (NS_FAILED(UnwrapWindowProxyArg(cx, source, memberSlot))) {
          mUnion.DestroyWindowProxy();
          tryNext = true;
          return true;
      }
    }
    return true;
  }

  inline bool
  TrySetToWindowProxy(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToWindowProxy(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline NonNull<mozilla::dom::External>&
  RawSetAsExternal()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eExternal;
    return mUnion.mValue.mExternal.SetValue();
  }

  inline mozilla::dom::WindowProxyHolder&
  RawSetAsWindowProxy()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eWindowProxy;
    return mUnion.mValue.mWindowProxy.SetValue();
  }
};

class FileOrDirectoryArgument
{
  FileOrDirectory& mUnion;

  FileOrDirectoryArgument(const FileOrDirectoryArgument&) = delete;
  FileOrDirectoryArgument& operator=(const FileOrDirectoryArgument&) = delete;
public:
  explicit inline FileOrDirectoryArgument(const FileOrDirectory& aUnion)
    : mUnion(const_cast<FileOrDirectory&>(aUnion))
  {
  }

  inline bool
  TrySetToFile(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::File>& memberSlot = RawSetAsFile();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::File, mozilla::dom::File>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyFile();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToFile(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToFile(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToDirectory(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::Directory>& memberSlot = RawSetAsDirectory();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::Directory, mozilla::dom::Directory>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyDirectory();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToDirectory(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToDirectory(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline NonNull<mozilla::dom::File>&
  RawSetAsFile()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eFile;
    return mUnion.mValue.mFile.SetValue();
  }

  inline NonNull<mozilla::dom::Directory>&
  RawSetAsDirectory()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eDirectory;
    return mUnion.mValue.mDirectory.SetValue();
  }
};

class FloatOrStringArgument
{
  FloatOrString& mUnion;

  FloatOrStringArgument(const FloatOrStringArgument&) = delete;
  FloatOrStringArgument& operator=(const FloatOrStringArgument&) = delete;
public:
  explicit inline FloatOrStringArgument(const FloatOrString& aUnion)
    : mUnion(const_cast<FloatOrString&>(aUnion))
  {
  }

  inline bool
  TrySetToFloat(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      float& memberSlot = RawSetAsFloat();
      if (!ValueToPrimitive<float, eDefault>(cx, value, "Float branch of (float or DOMString)", &memberSlot)) {
        return false;
      } else if (!mozilla::IsFinite(memberSlot)) {
        cx.ThrowErrorMessage<MSG_NOT_FINITE>("Float branch of (float or DOMString)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToFloat(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToFloat(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsString().AssignLiteral(aData);
  }

private:
  inline float&
  RawSetAsFloat()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eFloat;
    return mUnion.mValue.mFloat.SetValue();
  }

  inline binding_detail::FakeString<char16_t>&
  RawSetAsString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eString;
    return mUnion.mValue.mString.SetValue();
  }
};

class GPULoadOpOrDoubleSequenceOrGPUColorDictArgument
{
  GPULoadOpOrDoubleSequenceOrGPUColorDict& mUnion;

  GPULoadOpOrDoubleSequenceOrGPUColorDictArgument(const GPULoadOpOrDoubleSequenceOrGPUColorDictArgument&) = delete;
  GPULoadOpOrDoubleSequenceOrGPUColorDictArgument& operator=(const GPULoadOpOrDoubleSequenceOrGPUColorDictArgument&) = delete;
public:
  explicit inline GPULoadOpOrDoubleSequenceOrGPUColorDictArgument(const GPULoadOpOrDoubleSequenceOrGPUColorDict& aUnion)
    : mUnion(const_cast<GPULoadOpOrDoubleSequenceOrGPUColorDict&>(aUnion))
  {
  }

  inline bool
  TrySetToGPULoadOp(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      GPULoadOp& memberSlot = RawSetAsGPULoadOp();
      {
        int index;
        if (!FindEnumStringIndex<true>(cx, value, GPULoadOpValues::strings, "GPULoadOp", "GPULoadOp branch of (GPULoadOp or (sequence<double> or GPUColorDict))", &index)) {
          return false;
        }
        MOZ_ASSERT(index >= 0);
        memberSlot = static_cast<GPULoadOp>(index);
      }
    }
    return true;
  }

  inline bool
  TrySetToGPULoadOp(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToGPULoadOp(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToDoubleSequence(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::AutoSequence<double>& memberSlot = RawSetAsDoubleSequence();
      JS::ForOfIterator iter(cx);
      if (!iter.init(value, JS::ForOfIterator::AllowNonIterable)) {
        return false;
      }
      if (!iter.valueIsIterable()) {
        mUnion.DestroyDoubleSequence();
        tryNext = true;
        return true;
      }
      binding_detail::AutoSequence<double> &arr = memberSlot;
      JS::Rooted<JS::Value> temp(cx);
      while (true) {
        bool done;
        if (!iter.next(&temp, &done)) {
          return false;
        }
        if (done) {
          break;
        }
        double* slotPtr = arr.AppendElement(mozilla::fallible);
        if (!slotPtr) {
          JS_ReportOutOfMemory(cx);
          return false;
        }
        double& slot = *slotPtr;
        if (!ValueToPrimitive<double, eDefault>(cx, temp, "Element of sequence<double> branch of (GPULoadOp or (sequence<double> or GPUColorDict))", &slot)) {
          return false;
        } else if (!mozilla::IsFinite(slot)) {
          cx.ThrowErrorMessage<MSG_NOT_FINITE>("Element of sequence<double> branch of (GPULoadOp or (sequence<double> or GPUColorDict))");
          return false;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToDoubleSequence(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToDoubleSequence(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToGPUColorDict(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FastGPUColorDict& memberSlot = RawSetAsGPUColorDict();
      if (!IsConvertibleToDictionary(value)) {
        mUnion.DestroyGPUColorDict();
        tryNext = true;
        return true;
      }
      if (!memberSlot.Init(cx, value, "GPUColorDict branch of (GPULoadOp or (sequence<double> or GPUColorDict))", passedToJSImpl)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToGPUColorDict(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToGPUColorDict(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline GPULoadOp&
  RawSetAsGPULoadOp()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eGPULoadOp;
    return mUnion.mValue.mGPULoadOp.SetValue();
  }

  inline binding_detail::AutoSequence<double>&
  RawSetAsDoubleSequence()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eDoubleSequence;
    return mUnion.mValue.mDoubleSequence.SetValue();
  }

  inline binding_detail::FastGPUColorDict&
  RawSetAsGPUColorDict()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eGPUColorDict;
    return mUnion.mValue.mGPUColorDict.SetValue();
  }
};

class GPULoadOpOrFloatArgument
{
  GPULoadOpOrFloat& mUnion;

  GPULoadOpOrFloatArgument(const GPULoadOpOrFloatArgument&) = delete;
  GPULoadOpOrFloatArgument& operator=(const GPULoadOpOrFloatArgument&) = delete;
public:
  explicit inline GPULoadOpOrFloatArgument(const GPULoadOpOrFloat& aUnion)
    : mUnion(const_cast<GPULoadOpOrFloat&>(aUnion))
  {
  }

  inline bool
  TrySetToGPULoadOp(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      GPULoadOp& memberSlot = RawSetAsGPULoadOp();
      {
        int index;
        if (!FindEnumStringIndex<true>(cx, value, GPULoadOpValues::strings, "GPULoadOp", "GPULoadOp branch of (GPULoadOp or float)", &index)) {
          return false;
        }
        MOZ_ASSERT(index >= 0);
        memberSlot = static_cast<GPULoadOp>(index);
      }
    }
    return true;
  }

  inline bool
  TrySetToGPULoadOp(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToGPULoadOp(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToFloat(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      float& memberSlot = RawSetAsFloat();
      if (!ValueToPrimitive<float, eDefault>(cx, value, "Float branch of (GPULoadOp or float)", &memberSlot)) {
        return false;
      } else if (!mozilla::IsFinite(memberSlot)) {
        cx.ThrowErrorMessage<MSG_NOT_FINITE>("Float branch of (GPULoadOp or float)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToFloat(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToFloat(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline GPULoadOp&
  RawSetAsGPULoadOp()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eGPULoadOp;
    return mUnion.mValue.mGPULoadOp.SetValue();
  }

  inline float&
  RawSetAsFloat()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eFloat;
    return mUnion.mValue.mFloat.SetValue();
  }
};

class GPULoadOpOrRangeEnforcedUnsignedLongArgument
{
  GPULoadOpOrRangeEnforcedUnsignedLong& mUnion;

  GPULoadOpOrRangeEnforcedUnsignedLongArgument(const GPULoadOpOrRangeEnforcedUnsignedLongArgument&) = delete;
  GPULoadOpOrRangeEnforcedUnsignedLongArgument& operator=(const GPULoadOpOrRangeEnforcedUnsignedLongArgument&) = delete;
public:
  explicit inline GPULoadOpOrRangeEnforcedUnsignedLongArgument(const GPULoadOpOrRangeEnforcedUnsignedLong& aUnion)
    : mUnion(const_cast<GPULoadOpOrRangeEnforcedUnsignedLong&>(aUnion))
  {
  }

  inline bool
  TrySetToGPULoadOp(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      GPULoadOp& memberSlot = RawSetAsGPULoadOp();
      {
        int index;
        if (!FindEnumStringIndex<true>(cx, value, GPULoadOpValues::strings, "GPULoadOp", "GPULoadOp branch of (GPULoadOp or unsigned long)", &index)) {
          return false;
        }
        MOZ_ASSERT(index >= 0);
        memberSlot = static_cast<GPULoadOp>(index);
      }
    }
    return true;
  }

  inline bool
  TrySetToGPULoadOp(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToGPULoadOp(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToRangeEnforcedUnsignedLong(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      uint32_t& memberSlot = RawSetAsRangeEnforcedUnsignedLong();
      if (!ValueToPrimitive<uint32_t, eEnforceRange>(cx, value, "Unsigned long branch of (GPULoadOp or unsigned long)", &memberSlot)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToRangeEnforcedUnsignedLong(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToRangeEnforcedUnsignedLong(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline GPULoadOp&
  RawSetAsGPULoadOp()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eGPULoadOp;
    return mUnion.mValue.mGPULoadOp.SetValue();
  }

  inline uint32_t&
  RawSetAsRangeEnforcedUnsignedLong()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eRangeEnforcedUnsignedLong;
    return mUnion.mValue.mRangeEnforcedUnsignedLong.SetValue();
  }
};

class GPUOutOfMemoryErrorOrGPUValidationErrorArgument
{
  GPUOutOfMemoryErrorOrGPUValidationError& mUnion;

  GPUOutOfMemoryErrorOrGPUValidationErrorArgument(const GPUOutOfMemoryErrorOrGPUValidationErrorArgument&) = delete;
  GPUOutOfMemoryErrorOrGPUValidationErrorArgument& operator=(const GPUOutOfMemoryErrorOrGPUValidationErrorArgument&) = delete;
public:
  explicit inline GPUOutOfMemoryErrorOrGPUValidationErrorArgument(const GPUOutOfMemoryErrorOrGPUValidationError& aUnion)
    : mUnion(const_cast<GPUOutOfMemoryErrorOrGPUValidationError&>(aUnion))
  {
  }

  inline bool
  TrySetToGPUOutOfMemoryError(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::webgpu::OutOfMemoryError>& memberSlot = RawSetAsGPUOutOfMemoryError();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::GPUOutOfMemoryError, mozilla::webgpu::OutOfMemoryError>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyGPUOutOfMemoryError();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToGPUOutOfMemoryError(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToGPUOutOfMemoryError(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToGPUValidationError(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::webgpu::ValidationError>& memberSlot = RawSetAsGPUValidationError();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::GPUValidationError, mozilla::webgpu::ValidationError>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyGPUValidationError();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToGPUValidationError(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToGPUValidationError(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline NonNull<mozilla::webgpu::OutOfMemoryError>&
  RawSetAsGPUOutOfMemoryError()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eGPUOutOfMemoryError;
    return mUnion.mValue.mGPUOutOfMemoryError.SetValue();
  }

  inline NonNull<mozilla::webgpu::ValidationError>&
  RawSetAsGPUValidationError()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eGPUValidationError;
    return mUnion.mValue.mGPUValidationError.SetValue();
  }
};

class GPUSamplerOrGPUTextureViewOrGPUBufferBindingArgument
{
  GPUSamplerOrGPUTextureViewOrGPUBufferBinding& mUnion;

  GPUSamplerOrGPUTextureViewOrGPUBufferBindingArgument(const GPUSamplerOrGPUTextureViewOrGPUBufferBindingArgument&) = delete;
  GPUSamplerOrGPUTextureViewOrGPUBufferBindingArgument& operator=(const GPUSamplerOrGPUTextureViewOrGPUBufferBindingArgument&) = delete;
public:
  explicit inline GPUSamplerOrGPUTextureViewOrGPUBufferBindingArgument(const GPUSamplerOrGPUTextureViewOrGPUBufferBinding& aUnion)
    : mUnion(const_cast<GPUSamplerOrGPUTextureViewOrGPUBufferBinding&>(aUnion))
  {
  }

  inline bool
  TrySetToGPUSampler(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::webgpu::Sampler>& memberSlot = RawSetAsGPUSampler();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::GPUSampler, mozilla::webgpu::Sampler>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyGPUSampler();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToGPUSampler(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToGPUSampler(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToGPUTextureView(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::webgpu::TextureView>& memberSlot = RawSetAsGPUTextureView();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::GPUTextureView, mozilla::webgpu::TextureView>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyGPUTextureView();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToGPUTextureView(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToGPUTextureView(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToGPUBufferBinding(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FastGPUBufferBinding& memberSlot = RawSetAsGPUBufferBinding();
      if (!IsConvertibleToDictionary(value)) {
        mUnion.DestroyGPUBufferBinding();
        tryNext = true;
        return true;
      }
      if (!memberSlot.Init(cx, value, "GPUBufferBinding branch of (GPUSampler or GPUTextureView or GPUBufferBinding)", passedToJSImpl)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToGPUBufferBinding(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToGPUBufferBinding(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline NonNull<mozilla::webgpu::Sampler>&
  RawSetAsGPUSampler()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eGPUSampler;
    return mUnion.mValue.mGPUSampler.SetValue();
  }

  inline NonNull<mozilla::webgpu::TextureView>&
  RawSetAsGPUTextureView()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eGPUTextureView;
    return mUnion.mValue.mGPUTextureView.SetValue();
  }

  inline binding_detail::FastGPUBufferBinding&
  RawSetAsGPUBufferBinding()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eGPUBufferBinding;
    return mUnion.mValue.mGPUBufferBinding.SetValue();
  }
};

class HTMLCanvasElementOrOffscreenCanvasArgument
{
  HTMLCanvasElementOrOffscreenCanvas& mUnion;

  HTMLCanvasElementOrOffscreenCanvasArgument(const HTMLCanvasElementOrOffscreenCanvasArgument&) = delete;
  HTMLCanvasElementOrOffscreenCanvasArgument& operator=(const HTMLCanvasElementOrOffscreenCanvasArgument&) = delete;
public:
  explicit inline HTMLCanvasElementOrOffscreenCanvasArgument(const HTMLCanvasElementOrOffscreenCanvas& aUnion)
    : mUnion(const_cast<HTMLCanvasElementOrOffscreenCanvas&>(aUnion))
  {
  }

  inline bool
  TrySetToHTMLCanvasElement(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::HTMLCanvasElement>& memberSlot = RawSetAsHTMLCanvasElement();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::HTMLCanvasElement, mozilla::dom::HTMLCanvasElement>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyHTMLCanvasElement();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToHTMLCanvasElement(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToHTMLCanvasElement(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToOffscreenCanvas(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::OffscreenCanvas>& memberSlot = RawSetAsOffscreenCanvas();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::OffscreenCanvas, mozilla::dom::OffscreenCanvas>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyOffscreenCanvas();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToOffscreenCanvas(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToOffscreenCanvas(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline NonNull<mozilla::dom::HTMLCanvasElement>&
  RawSetAsHTMLCanvasElement()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eHTMLCanvasElement;
    return mUnion.mValue.mHTMLCanvasElement.SetValue();
  }

  inline NonNull<mozilla::dom::OffscreenCanvas>&
  RawSetAsOffscreenCanvas()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eOffscreenCanvas;
    return mUnion.mValue.mOffscreenCanvas.SetValue();
  }
};

class HTMLCollectionOrElementArgument
{
  HTMLCollectionOrElement& mUnion;

  HTMLCollectionOrElementArgument(const HTMLCollectionOrElementArgument&) = delete;
  HTMLCollectionOrElementArgument& operator=(const HTMLCollectionOrElementArgument&) = delete;
public:
  explicit inline HTMLCollectionOrElementArgument(const HTMLCollectionOrElement& aUnion)
    : mUnion(const_cast<HTMLCollectionOrElement&>(aUnion))
  {
  }

  inline bool
  TrySetToHTMLCollection(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<nsIHTMLCollection>& memberSlot = RawSetAsHTMLCollection();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::HTMLCollection, nsIHTMLCollection>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyHTMLCollection();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToHTMLCollection(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToHTMLCollection(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToElement(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::Element>& memberSlot = RawSetAsElement();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::Element, mozilla::dom::Element>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyElement();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToElement(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToElement(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline NonNull<nsIHTMLCollection>&
  RawSetAsHTMLCollection()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eHTMLCollection;
    return mUnion.mValue.mHTMLCollection.SetValue();
  }

  inline NonNull<mozilla::dom::Element>&
  RawSetAsElement()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eElement;
    return mUnion.mValue.mElement.SetValue();
  }
};

class HTMLElementOrLongArgument
{
  HTMLElementOrLong& mUnion;

  HTMLElementOrLongArgument(const HTMLElementOrLongArgument&) = delete;
  HTMLElementOrLongArgument& operator=(const HTMLElementOrLongArgument&) = delete;
public:
  explicit inline HTMLElementOrLongArgument(const HTMLElementOrLong& aUnion)
    : mUnion(const_cast<HTMLElementOrLong&>(aUnion))
  {
  }

  inline bool
  TrySetToHTMLElement(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<nsGenericHTMLElement>& memberSlot = RawSetAsHTMLElement();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::HTMLElement, nsGenericHTMLElement>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyHTMLElement();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToHTMLElement(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToHTMLElement(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToLong(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      int32_t& memberSlot = RawSetAsLong();
      if (!ValueToPrimitive<int32_t, eDefault>(cx, value, "Long branch of (HTMLElement or long)", &memberSlot)) {
        return false;
      }
    }
    return true;
  }

private:
  inline NonNull<nsGenericHTMLElement>&
  RawSetAsHTMLElement()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eHTMLElement;
    return mUnion.mValue.mHTMLElement.SetValue();
  }

  inline int32_t&
  RawSetAsLong()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eLong;
    return mUnion.mValue.mLong.SetValue();
  }
};

class HTMLImageElementOrSVGImageElementOrHTMLCanvasElementOrHTMLVideoElementOrImageBitmapArgument
{
  HTMLImageElementOrSVGImageElementOrHTMLCanvasElementOrHTMLVideoElementOrImageBitmap& mUnion;

  HTMLImageElementOrSVGImageElementOrHTMLCanvasElementOrHTMLVideoElementOrImageBitmapArgument(const HTMLImageElementOrSVGImageElementOrHTMLCanvasElementOrHTMLVideoElementOrImageBitmapArgument&) = delete;
  HTMLImageElementOrSVGImageElementOrHTMLCanvasElementOrHTMLVideoElementOrImageBitmapArgument& operator=(const HTMLImageElementOrSVGImageElementOrHTMLCanvasElementOrHTMLVideoElementOrImageBitmapArgument&) = delete;
public:
  explicit inline HTMLImageElementOrSVGImageElementOrHTMLCanvasElementOrHTMLVideoElementOrImageBitmapArgument(const HTMLImageElementOrSVGImageElementOrHTMLCanvasElementOrHTMLVideoElementOrImageBitmap& aUnion)
    : mUnion(const_cast<HTMLImageElementOrSVGImageElementOrHTMLCanvasElementOrHTMLVideoElementOrImageBitmap&>(aUnion))
  {
  }

  inline bool
  TrySetToHTMLImageElement(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::HTMLImageElement>& memberSlot = RawSetAsHTMLImageElement();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::HTMLImageElement, mozilla::dom::HTMLImageElement>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyHTMLImageElement();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToHTMLImageElement(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToHTMLImageElement(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToSVGImageElement(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::SVGImageElement>& memberSlot = RawSetAsSVGImageElement();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::SVGImageElement, mozilla::dom::SVGImageElement>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroySVGImageElement();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToSVGImageElement(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToSVGImageElement(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToHTMLCanvasElement(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::HTMLCanvasElement>& memberSlot = RawSetAsHTMLCanvasElement();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::HTMLCanvasElement, mozilla::dom::HTMLCanvasElement>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyHTMLCanvasElement();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToHTMLCanvasElement(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToHTMLCanvasElement(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToHTMLVideoElement(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::HTMLVideoElement>& memberSlot = RawSetAsHTMLVideoElement();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::HTMLVideoElement, mozilla::dom::HTMLVideoElement>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyHTMLVideoElement();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToHTMLVideoElement(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToHTMLVideoElement(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToImageBitmap(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::ImageBitmap>& memberSlot = RawSetAsImageBitmap();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::ImageBitmap, mozilla::dom::ImageBitmap>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyImageBitmap();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToImageBitmap(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToImageBitmap(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline NonNull<mozilla::dom::HTMLImageElement>&
  RawSetAsHTMLImageElement()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eHTMLImageElement;
    return mUnion.mValue.mHTMLImageElement.SetValue();
  }

  inline NonNull<mozilla::dom::SVGImageElement>&
  RawSetAsSVGImageElement()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eSVGImageElement;
    return mUnion.mValue.mSVGImageElement.SetValue();
  }

  inline NonNull<mozilla::dom::HTMLCanvasElement>&
  RawSetAsHTMLCanvasElement()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eHTMLCanvasElement;
    return mUnion.mValue.mHTMLCanvasElement.SetValue();
  }

  inline NonNull<mozilla::dom::HTMLVideoElement>&
  RawSetAsHTMLVideoElement()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eHTMLVideoElement;
    return mUnion.mValue.mHTMLVideoElement.SetValue();
  }

  inline NonNull<mozilla::dom::ImageBitmap>&
  RawSetAsImageBitmap()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eImageBitmap;
    return mUnion.mValue.mImageBitmap.SetValue();
  }
};

class HTMLImageElementOrSVGImageElementOrHTMLCanvasElementOrHTMLVideoElementOrImageBitmapOrBlobOrCanvasRenderingContext2DOrImageDataArgument
{
  HTMLImageElementOrSVGImageElementOrHTMLCanvasElementOrHTMLVideoElementOrImageBitmapOrBlobOrCanvasRenderingContext2DOrImageData& mUnion;

  HTMLImageElementOrSVGImageElementOrHTMLCanvasElementOrHTMLVideoElementOrImageBitmapOrBlobOrCanvasRenderingContext2DOrImageDataArgument(const HTMLImageElementOrSVGImageElementOrHTMLCanvasElementOrHTMLVideoElementOrImageBitmapOrBlobOrCanvasRenderingContext2DOrImageDataArgument&) = delete;
  HTMLImageElementOrSVGImageElementOrHTMLCanvasElementOrHTMLVideoElementOrImageBitmapOrBlobOrCanvasRenderingContext2DOrImageDataArgument& operator=(const HTMLImageElementOrSVGImageElementOrHTMLCanvasElementOrHTMLVideoElementOrImageBitmapOrBlobOrCanvasRenderingContext2DOrImageDataArgument&) = delete;
public:
  explicit inline HTMLImageElementOrSVGImageElementOrHTMLCanvasElementOrHTMLVideoElementOrImageBitmapOrBlobOrCanvasRenderingContext2DOrImageDataArgument(const HTMLImageElementOrSVGImageElementOrHTMLCanvasElementOrHTMLVideoElementOrImageBitmapOrBlobOrCanvasRenderingContext2DOrImageData& aUnion)
    : mUnion(const_cast<HTMLImageElementOrSVGImageElementOrHTMLCanvasElementOrHTMLVideoElementOrImageBitmapOrBlobOrCanvasRenderingContext2DOrImageData&>(aUnion))
  {
  }

  inline bool
  TrySetToHTMLImageElement(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::HTMLImageElement>& memberSlot = RawSetAsHTMLImageElement();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::HTMLImageElement, mozilla::dom::HTMLImageElement>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyHTMLImageElement();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToHTMLImageElement(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToHTMLImageElement(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToSVGImageElement(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::SVGImageElement>& memberSlot = RawSetAsSVGImageElement();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::SVGImageElement, mozilla::dom::SVGImageElement>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroySVGImageElement();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToSVGImageElement(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToSVGImageElement(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToHTMLCanvasElement(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::HTMLCanvasElement>& memberSlot = RawSetAsHTMLCanvasElement();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::HTMLCanvasElement, mozilla::dom::HTMLCanvasElement>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyHTMLCanvasElement();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToHTMLCanvasElement(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToHTMLCanvasElement(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToHTMLVideoElement(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::HTMLVideoElement>& memberSlot = RawSetAsHTMLVideoElement();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::HTMLVideoElement, mozilla::dom::HTMLVideoElement>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyHTMLVideoElement();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToHTMLVideoElement(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToHTMLVideoElement(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToImageBitmap(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::ImageBitmap>& memberSlot = RawSetAsImageBitmap();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::ImageBitmap, mozilla::dom::ImageBitmap>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyImageBitmap();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToImageBitmap(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToImageBitmap(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToBlob(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::Blob>& memberSlot = RawSetAsBlob();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::Blob, mozilla::dom::Blob>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyBlob();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToBlob(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToBlob(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToCanvasRenderingContext2D(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::CanvasRenderingContext2D>& memberSlot = RawSetAsCanvasRenderingContext2D();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::CanvasRenderingContext2D, mozilla::dom::CanvasRenderingContext2D>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyCanvasRenderingContext2D();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToCanvasRenderingContext2D(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToCanvasRenderingContext2D(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToImageData(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::ImageData>& memberSlot = RawSetAsImageData();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::ImageData, mozilla::dom::ImageData>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyImageData();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToImageData(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToImageData(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline NonNull<mozilla::dom::HTMLImageElement>&
  RawSetAsHTMLImageElement()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eHTMLImageElement;
    return mUnion.mValue.mHTMLImageElement.SetValue();
  }

  inline NonNull<mozilla::dom::SVGImageElement>&
  RawSetAsSVGImageElement()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eSVGImageElement;
    return mUnion.mValue.mSVGImageElement.SetValue();
  }

  inline NonNull<mozilla::dom::HTMLCanvasElement>&
  RawSetAsHTMLCanvasElement()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eHTMLCanvasElement;
    return mUnion.mValue.mHTMLCanvasElement.SetValue();
  }

  inline NonNull<mozilla::dom::HTMLVideoElement>&
  RawSetAsHTMLVideoElement()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eHTMLVideoElement;
    return mUnion.mValue.mHTMLVideoElement.SetValue();
  }

  inline NonNull<mozilla::dom::ImageBitmap>&
  RawSetAsImageBitmap()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eImageBitmap;
    return mUnion.mValue.mImageBitmap.SetValue();
  }

  inline NonNull<mozilla::dom::Blob>&
  RawSetAsBlob()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eBlob;
    return mUnion.mValue.mBlob.SetValue();
  }

  inline NonNull<mozilla::dom::CanvasRenderingContext2D>&
  RawSetAsCanvasRenderingContext2D()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eCanvasRenderingContext2D;
    return mUnion.mValue.mCanvasRenderingContext2D.SetValue();
  }

  inline NonNull<mozilla::dom::ImageData>&
  RawSetAsImageData()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eImageData;
    return mUnion.mValue.mImageData.SetValue();
  }
};

class HTMLOptionElementOrHTMLOptGroupElementArgument
{
  HTMLOptionElementOrHTMLOptGroupElement& mUnion;

  HTMLOptionElementOrHTMLOptGroupElementArgument(const HTMLOptionElementOrHTMLOptGroupElementArgument&) = delete;
  HTMLOptionElementOrHTMLOptGroupElementArgument& operator=(const HTMLOptionElementOrHTMLOptGroupElementArgument&) = delete;
public:
  explicit inline HTMLOptionElementOrHTMLOptGroupElementArgument(const HTMLOptionElementOrHTMLOptGroupElement& aUnion)
    : mUnion(const_cast<HTMLOptionElementOrHTMLOptGroupElement&>(aUnion))
  {
  }

  inline bool
  TrySetToHTMLOptionElement(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::HTMLOptionElement>& memberSlot = RawSetAsHTMLOptionElement();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::HTMLOptionElement, mozilla::dom::HTMLOptionElement>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyHTMLOptionElement();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToHTMLOptionElement(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToHTMLOptionElement(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToHTMLOptGroupElement(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::HTMLOptGroupElement>& memberSlot = RawSetAsHTMLOptGroupElement();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::HTMLOptGroupElement, mozilla::dom::HTMLOptGroupElement>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyHTMLOptGroupElement();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToHTMLOptGroupElement(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToHTMLOptGroupElement(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline NonNull<mozilla::dom::HTMLOptionElement>&
  RawSetAsHTMLOptionElement()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eHTMLOptionElement;
    return mUnion.mValue.mHTMLOptionElement.SetValue();
  }

  inline NonNull<mozilla::dom::HTMLOptGroupElement>&
  RawSetAsHTMLOptGroupElement()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eHTMLOptGroupElement;
    return mUnion.mValue.mHTMLOptGroupElement.SetValue();
  }
};

class IDBObjectStoreOrIDBIndexArgument
{
  IDBObjectStoreOrIDBIndex& mUnion;

  IDBObjectStoreOrIDBIndexArgument(const IDBObjectStoreOrIDBIndexArgument&) = delete;
  IDBObjectStoreOrIDBIndexArgument& operator=(const IDBObjectStoreOrIDBIndexArgument&) = delete;
public:
  explicit inline IDBObjectStoreOrIDBIndexArgument(const IDBObjectStoreOrIDBIndex& aUnion)
    : mUnion(const_cast<IDBObjectStoreOrIDBIndex&>(aUnion))
  {
  }

  inline bool
  TrySetToIDBObjectStore(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::IDBObjectStore>& memberSlot = RawSetAsIDBObjectStore();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::IDBObjectStore, mozilla::dom::IDBObjectStore>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyIDBObjectStore();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToIDBObjectStore(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToIDBObjectStore(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToIDBIndex(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::IDBIndex>& memberSlot = RawSetAsIDBIndex();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::IDBIndex, mozilla::dom::IDBIndex>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyIDBIndex();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToIDBIndex(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToIDBIndex(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline NonNull<mozilla::dom::IDBObjectStore>&
  RawSetAsIDBObjectStore()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eIDBObjectStore;
    return mUnion.mValue.mIDBObjectStore.SetValue();
  }

  inline NonNull<mozilla::dom::IDBIndex>&
  RawSetAsIDBIndex()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eIDBIndex;
    return mUnion.mValue.mIDBIndex.SetValue();
  }
};

class IDBObjectStoreOrIDBIndexOrIDBCursorArgument
{
  IDBObjectStoreOrIDBIndexOrIDBCursor& mUnion;

  IDBObjectStoreOrIDBIndexOrIDBCursorArgument(const IDBObjectStoreOrIDBIndexOrIDBCursorArgument&) = delete;
  IDBObjectStoreOrIDBIndexOrIDBCursorArgument& operator=(const IDBObjectStoreOrIDBIndexOrIDBCursorArgument&) = delete;
public:
  explicit inline IDBObjectStoreOrIDBIndexOrIDBCursorArgument(const IDBObjectStoreOrIDBIndexOrIDBCursor& aUnion)
    : mUnion(const_cast<IDBObjectStoreOrIDBIndexOrIDBCursor&>(aUnion))
  {
  }

  inline bool
  TrySetToIDBObjectStore(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::IDBObjectStore>& memberSlot = RawSetAsIDBObjectStore();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::IDBObjectStore, mozilla::dom::IDBObjectStore>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyIDBObjectStore();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToIDBObjectStore(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToIDBObjectStore(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToIDBIndex(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::IDBIndex>& memberSlot = RawSetAsIDBIndex();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::IDBIndex, mozilla::dom::IDBIndex>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyIDBIndex();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToIDBIndex(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToIDBIndex(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToIDBCursor(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::IDBCursor>& memberSlot = RawSetAsIDBCursor();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::IDBCursor, mozilla::dom::IDBCursor>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyIDBCursor();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToIDBCursor(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToIDBCursor(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline NonNull<mozilla::dom::IDBObjectStore>&
  RawSetAsIDBObjectStore()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eIDBObjectStore;
    return mUnion.mValue.mIDBObjectStore.SetValue();
  }

  inline NonNull<mozilla::dom::IDBIndex>&
  RawSetAsIDBIndex()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eIDBIndex;
    return mUnion.mValue.mIDBIndex.SetValue();
  }

  inline NonNull<mozilla::dom::IDBCursor>&
  RawSetAsIDBCursor()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eIDBCursor;
    return mUnion.mValue.mIDBCursor.SetValue();
  }
};

class LongOrConstrainLongRangeArgument
{
  LongOrConstrainLongRange& mUnion;

  LongOrConstrainLongRangeArgument(const LongOrConstrainLongRangeArgument&) = delete;
  LongOrConstrainLongRangeArgument& operator=(const LongOrConstrainLongRangeArgument&) = delete;
public:
  explicit inline LongOrConstrainLongRangeArgument(const LongOrConstrainLongRange& aUnion)
    : mUnion(const_cast<LongOrConstrainLongRange&>(aUnion))
  {
  }

  inline bool
  TrySetToLong(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      int32_t& memberSlot = RawSetAsLong();
      if (!ValueToPrimitive<int32_t, eDefault>(cx, value, "Long branch of (long or ConstrainLongRange)", &memberSlot)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToConstrainLongRange(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FastConstrainLongRange& memberSlot = RawSetAsConstrainLongRange();
      if (!IsConvertibleToDictionary(value)) {
        mUnion.DestroyConstrainLongRange();
        tryNext = true;
        return true;
      }
      if (!memberSlot.Init(cx, value, "ConstrainLongRange branch of (long or ConstrainLongRange)", passedToJSImpl)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToConstrainLongRange(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToConstrainLongRange(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline int32_t&
  RawSetAsLong()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eLong;
    return mUnion.mValue.mLong.SetValue();
  }

  inline binding_detail::FastConstrainLongRange&
  RawSetAsConstrainLongRange()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eConstrainLongRange;
    return mUnion.mValue.mConstrainLongRange.SetValue();
  }
};

class LongOrStringAnyRecordArgument
{
  LongOrStringAnyRecord& mUnion;
  Maybe<RecordRooter<nsString, JS::Value>> mStringAnyRecordHolder;

  LongOrStringAnyRecordArgument(const LongOrStringAnyRecordArgument&) = delete;
  LongOrStringAnyRecordArgument& operator=(const LongOrStringAnyRecordArgument&) = delete;
public:
  explicit inline LongOrStringAnyRecordArgument(const LongOrStringAnyRecord& aUnion)
    : mUnion(const_cast<LongOrStringAnyRecord&>(aUnion))
  {
  }

  inline bool
  TrySetToLong(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      int32_t& memberSlot = RawSetAsLong();
      if (!ValueToPrimitive<int32_t, eDefault>(cx, value, "Long branch of (long or record<DOMString, any>)", &memberSlot)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToStringAnyRecord(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      Record<nsString, JS::Value>& memberSlot = RawSetAsStringAnyRecord();
      mStringAnyRecordHolder.emplace(cx, &memberSlot);
      auto& recordEntries = memberSlot.Entries();

      JS::Rooted<JSObject*> recordObj(cx, &value.toObject());
      JS::RootedVector<jsid> ids(cx);
      if (!js::GetPropertyKeys(cx, recordObj,
                               JSITER_OWNONLY | JSITER_HIDDEN | JSITER_SYMBOLS, &ids)) {
        return false;
      }
      if (!recordEntries.SetCapacity(ids.length(), mozilla::fallible)) {
        JS_ReportOutOfMemory(cx);
        return false;
      }
      JS::Rooted<JS::Value> propNameValue(cx);
      JS::Rooted<JS::Value> temp(cx);
      JS::Rooted<jsid> curId(cx);
      JS::Rooted<JS::Value> idVal(cx);
      // Use a hashset to keep track of ids seen, to avoid
      // introducing nasty O(N^2) behavior scanning for them all the
      // time.  Ideally we'd use a data structure with O(1) lookup
      // _and_ ordering for the MozMap, but we don't have one lying
      // around.
      nsTHashtable<nsStringHashKey> idsSeen;
      for (size_t i = 0; i < ids.length(); ++i) {
        curId = ids[i];

        JS::Rooted<JS::PropertyDescriptor> desc(cx);
        if (!JS_GetOwnPropertyDescriptorById(cx, recordObj, curId,
                                             &desc)) {
          return false;
        }

        if (!desc.object() /* == undefined in spec terms */ ||
            !desc.enumerable()) {
          continue;
        }

        idVal = js::IdToValue(curId);
        nsString propName;
        // This will just throw if idVal is a Symbol, like the spec says
        // to do.
        if (!ConvertJSValueToString(cx, idVal, "key of record<DOMString, any> branch of (long or record<DOMString, any>)", propName)) {
          return false;
        }

        if (!JS_GetPropertyById(cx, recordObj, curId, &temp)) {
          return false;
        }

        Record<nsString, JS::Value>::EntryType* entry;
        if (!idsSeen.EnsureInserted(propName)) {
          // Find the existing entry.
          auto idx = recordEntries.IndexOf(propName);
          MOZ_ASSERT(idx != recordEntries.NoIndex,
                     "Why is it not found?");
          // Now blow it away to make it look like it was just added
          // to the array, because it's not obvious that it's
          // safe to write to its already-initialized mValue via our
          // normal codegen conversions.  For example, the value
          // could be a union and this would change its type, but
          // codegen assumes we won't do that.
          entry = recordEntries.ReconstructElementAt(idx);
        } else {
          // Safe to do an infallible append here, because we did a
          // SetCapacity above to the right capacity.
          entry = recordEntries.AppendElement();
        }
        entry->mKey = propName;
        JS::Value& slot = entry->mValue;
#ifdef __clang__
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wunreachable-code"
#pragma clang diagnostic ignored "-Wunreachable-code-return"
#endif // __clang__
        if ((passedToJSImpl) && !CallerSubsumes(temp)) {
          cx.ThrowErrorMessage<MSG_PERMISSION_DENIED_TO_PASS_ARG>("value in record<DOMString, any> branch of (long or record<DOMString, any>)");
          return false;
        }
#ifdef __clang__
#pragma clang diagnostic pop
#endif // __clang__
        slot = temp;
      }
    }
    return true;
  }

  inline bool
  TrySetToStringAnyRecord(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToStringAnyRecord(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline int32_t&
  RawSetAsLong()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eLong;
    return mUnion.mValue.mLong.SetValue();
  }

  inline Record<nsString, JS::Value>&
  RawSetAsStringAnyRecord()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eStringAnyRecord;
    return mUnion.mValue.mStringAnyRecord.SetValue();
  }
};

class MatchGlobOrStringArgument
{
  MatchGlobOrString& mUnion;

  MatchGlobOrStringArgument(const MatchGlobOrStringArgument&) = delete;
  MatchGlobOrStringArgument& operator=(const MatchGlobOrStringArgument&) = delete;
public:
  explicit inline MatchGlobOrStringArgument(const MatchGlobOrString& aUnion)
    : mUnion(const_cast<MatchGlobOrString&>(aUnion))
  {
  }

  inline bool
  TrySetToMatchGlob(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::extensions::MatchGlob>& memberSlot = RawSetAsMatchGlob();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::MatchGlob, mozilla::extensions::MatchGlob>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyMatchGlob();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToMatchGlob(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToMatchGlob(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsString().AssignLiteral(aData);
  }

private:
  inline NonNull<mozilla::extensions::MatchGlob>&
  RawSetAsMatchGlob()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eMatchGlob;
    return mUnion.mValue.mMatchGlob.SetValue();
  }

  inline binding_detail::FakeString<char16_t>&
  RawSetAsString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eString;
    return mUnion.mValue.mString.SetValue();
  }
};

class MatchPatternSetOrStringSequenceArgument
{
  MatchPatternSetOrStringSequence& mUnion;

  MatchPatternSetOrStringSequenceArgument(const MatchPatternSetOrStringSequenceArgument&) = delete;
  MatchPatternSetOrStringSequenceArgument& operator=(const MatchPatternSetOrStringSequenceArgument&) = delete;
public:
  explicit inline MatchPatternSetOrStringSequenceArgument(const MatchPatternSetOrStringSequence& aUnion)
    : mUnion(const_cast<MatchPatternSetOrStringSequence&>(aUnion))
  {
  }

  inline bool
  TrySetToMatchPatternSet(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::extensions::MatchPatternSet>& memberSlot = RawSetAsMatchPatternSet();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::MatchPatternSet, mozilla::extensions::MatchPatternSet>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyMatchPatternSet();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToMatchPatternSet(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToMatchPatternSet(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToStringSequence(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::AutoSequence<nsString>& memberSlot = RawSetAsStringSequence();
      JS::ForOfIterator iter(cx);
      if (!iter.init(value, JS::ForOfIterator::AllowNonIterable)) {
        return false;
      }
      if (!iter.valueIsIterable()) {
        mUnion.DestroyStringSequence();
        tryNext = true;
        return true;
      }
      binding_detail::AutoSequence<nsString> &arr = memberSlot;
      JS::Rooted<JS::Value> temp(cx);
      while (true) {
        bool done;
        if (!iter.next(&temp, &done)) {
          return false;
        }
        if (done) {
          break;
        }
        nsString* slotPtr = arr.AppendElement(mozilla::fallible);
        if (!slotPtr) {
          JS_ReportOutOfMemory(cx);
          return false;
        }
        nsString& slot = *slotPtr;
        if (!ConvertJSValueToString(cx, temp, eStringify, eStringify, slot)) {
          return false;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToStringSequence(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToStringSequence(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline NonNull<mozilla::extensions::MatchPatternSet>&
  RawSetAsMatchPatternSet()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eMatchPatternSet;
    return mUnion.mValue.mMatchPatternSet.SetValue();
  }

  inline binding_detail::AutoSequence<nsString>&
  RawSetAsStringSequence()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eStringSequence;
    return mUnion.mValue.mStringSequence.SetValue();
  }
};

class MaybeSharedFloat32ArrayOrUnrestrictedFloatSequenceArgument
{
  MaybeSharedFloat32ArrayOrUnrestrictedFloatSequence& mUnion;

  MaybeSharedFloat32ArrayOrUnrestrictedFloatSequenceArgument(const MaybeSharedFloat32ArrayOrUnrestrictedFloatSequenceArgument&) = delete;
  MaybeSharedFloat32ArrayOrUnrestrictedFloatSequenceArgument& operator=(const MaybeSharedFloat32ArrayOrUnrestrictedFloatSequenceArgument&) = delete;
public:
  explicit inline MaybeSharedFloat32ArrayOrUnrestrictedFloatSequenceArgument(const MaybeSharedFloat32ArrayOrUnrestrictedFloatSequence& aUnion)
    : mUnion(const_cast<MaybeSharedFloat32ArrayOrUnrestrictedFloatSequence&>(aUnion))
  {
  }

  inline bool
  TrySetToFloat32Array(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      RootedSpiderMonkeyInterface<Float32Array>& memberSlot = RawSetAsFloat32Array(cx);
      if (!memberSlot.Init(&value.toObject())) {
        mUnion.DestroyFloat32Array();
        tryNext = true;
        return true;
      }
      if (JS::IsLargeArrayBufferView(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_LARGE>("Float32Array branch of (Float32Array or sequence<unrestricted float>)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToFloat32Array(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToFloat32Array(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToUnrestrictedFloatSequence(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::AutoSequence<float>& memberSlot = RawSetAsUnrestrictedFloatSequence();
      JS::ForOfIterator iter(cx);
      if (!iter.init(value, JS::ForOfIterator::AllowNonIterable)) {
        return false;
      }
      if (!iter.valueIsIterable()) {
        mUnion.DestroyUnrestrictedFloatSequence();
        tryNext = true;
        return true;
      }
      binding_detail::AutoSequence<float> &arr = memberSlot;
      JS::Rooted<JS::Value> temp(cx);
      while (true) {
        bool done;
        if (!iter.next(&temp, &done)) {
          return false;
        }
        if (done) {
          break;
        }
        float* slotPtr = arr.AppendElement(mozilla::fallible);
        if (!slotPtr) {
          JS_ReportOutOfMemory(cx);
          return false;
        }
        float& slot = *slotPtr;
        if (!ValueToPrimitive<float, eDefault>(cx, temp, "Element of sequence<unrestricted float> branch of (Float32Array or sequence<unrestricted float>)", &slot)) {
          return false;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToUnrestrictedFloatSequence(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToUnrestrictedFloatSequence(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline RootedSpiderMonkeyInterface<Float32Array>&
  RawSetAsFloat32Array(JSContext* cx)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eFloat32Array;
    return mUnion.mValue.mFloat32Array.SetValue(cx);
  }

  inline binding_detail::AutoSequence<float>&
  RawSetAsUnrestrictedFloatSequence()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUnrestrictedFloatSequence;
    return mUnion.mValue.mUnrestrictedFloatSequence.SetValue();
  }
};

class MaybeSharedInt32ArrayOrLongSequenceArgument
{
  MaybeSharedInt32ArrayOrLongSequence& mUnion;

  MaybeSharedInt32ArrayOrLongSequenceArgument(const MaybeSharedInt32ArrayOrLongSequenceArgument&) = delete;
  MaybeSharedInt32ArrayOrLongSequenceArgument& operator=(const MaybeSharedInt32ArrayOrLongSequenceArgument&) = delete;
public:
  explicit inline MaybeSharedInt32ArrayOrLongSequenceArgument(const MaybeSharedInt32ArrayOrLongSequence& aUnion)
    : mUnion(const_cast<MaybeSharedInt32ArrayOrLongSequence&>(aUnion))
  {
  }

  inline bool
  TrySetToInt32Array(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      RootedSpiderMonkeyInterface<Int32Array>& memberSlot = RawSetAsInt32Array(cx);
      if (!memberSlot.Init(&value.toObject())) {
        mUnion.DestroyInt32Array();
        tryNext = true;
        return true;
      }
      if (JS::IsLargeArrayBufferView(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_LARGE>("Int32Array branch of (Int32Array or sequence<long>)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToInt32Array(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToInt32Array(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToLongSequence(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::AutoSequence<int32_t>& memberSlot = RawSetAsLongSequence();
      JS::ForOfIterator iter(cx);
      if (!iter.init(value, JS::ForOfIterator::AllowNonIterable)) {
        return false;
      }
      if (!iter.valueIsIterable()) {
        mUnion.DestroyLongSequence();
        tryNext = true;
        return true;
      }
      binding_detail::AutoSequence<int32_t> &arr = memberSlot;
      JS::Rooted<JS::Value> temp(cx);
      while (true) {
        bool done;
        if (!iter.next(&temp, &done)) {
          return false;
        }
        if (done) {
          break;
        }
        int32_t* slotPtr = arr.AppendElement(mozilla::fallible);
        if (!slotPtr) {
          JS_ReportOutOfMemory(cx);
          return false;
        }
        int32_t& slot = *slotPtr;
        if (!ValueToPrimitive<int32_t, eDefault>(cx, temp, "Element of sequence<long> branch of (Int32Array or sequence<long>)", &slot)) {
          return false;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToLongSequence(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToLongSequence(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline RootedSpiderMonkeyInterface<Int32Array>&
  RawSetAsInt32Array(JSContext* cx)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eInt32Array;
    return mUnion.mValue.mInt32Array.SetValue(cx);
  }

  inline binding_detail::AutoSequence<int32_t>&
  RawSetAsLongSequence()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eLongSequence;
    return mUnion.mValue.mLongSequence.SetValue();
  }
};

class MaybeSharedUint32ArrayOrUnsignedLongSequenceArgument
{
  MaybeSharedUint32ArrayOrUnsignedLongSequence& mUnion;

  MaybeSharedUint32ArrayOrUnsignedLongSequenceArgument(const MaybeSharedUint32ArrayOrUnsignedLongSequenceArgument&) = delete;
  MaybeSharedUint32ArrayOrUnsignedLongSequenceArgument& operator=(const MaybeSharedUint32ArrayOrUnsignedLongSequenceArgument&) = delete;
public:
  explicit inline MaybeSharedUint32ArrayOrUnsignedLongSequenceArgument(const MaybeSharedUint32ArrayOrUnsignedLongSequence& aUnion)
    : mUnion(const_cast<MaybeSharedUint32ArrayOrUnsignedLongSequence&>(aUnion))
  {
  }

  inline bool
  TrySetToUint32Array(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      RootedSpiderMonkeyInterface<Uint32Array>& memberSlot = RawSetAsUint32Array(cx);
      if (!memberSlot.Init(&value.toObject())) {
        mUnion.DestroyUint32Array();
        tryNext = true;
        return true;
      }
      if (JS::IsLargeArrayBufferView(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_LARGE>("Uint32Array branch of (Uint32Array or sequence<unsigned long>)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToUint32Array(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToUint32Array(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToUnsignedLongSequence(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::AutoSequence<uint32_t>& memberSlot = RawSetAsUnsignedLongSequence();
      JS::ForOfIterator iter(cx);
      if (!iter.init(value, JS::ForOfIterator::AllowNonIterable)) {
        return false;
      }
      if (!iter.valueIsIterable()) {
        mUnion.DestroyUnsignedLongSequence();
        tryNext = true;
        return true;
      }
      binding_detail::AutoSequence<uint32_t> &arr = memberSlot;
      JS::Rooted<JS::Value> temp(cx);
      while (true) {
        bool done;
        if (!iter.next(&temp, &done)) {
          return false;
        }
        if (done) {
          break;
        }
        uint32_t* slotPtr = arr.AppendElement(mozilla::fallible);
        if (!slotPtr) {
          JS_ReportOutOfMemory(cx);
          return false;
        }
        uint32_t& slot = *slotPtr;
        if (!ValueToPrimitive<uint32_t, eDefault>(cx, temp, "Element of sequence<unsigned long> branch of (Uint32Array or sequence<unsigned long>)", &slot)) {
          return false;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToUnsignedLongSequence(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToUnsignedLongSequence(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline RootedSpiderMonkeyInterface<Uint32Array>&
  RawSetAsUint32Array(JSContext* cx)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUint32Array;
    return mUnion.mValue.mUint32Array.SetValue(cx);
  }

  inline binding_detail::AutoSequence<uint32_t>&
  RawSetAsUnsignedLongSequence()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUnsignedLongSequence;
    return mUnion.mValue.mUnsignedLongSequence.SetValue();
  }
};

class MediaListOrUTF8StringArgument
{
  MediaListOrUTF8String& mUnion;

  MediaListOrUTF8StringArgument(const MediaListOrUTF8StringArgument&) = delete;
  MediaListOrUTF8StringArgument& operator=(const MediaListOrUTF8StringArgument&) = delete;
public:
  explicit inline MediaListOrUTF8StringArgument(const MediaListOrUTF8String& aUnion)
    : mUnion(const_cast<MediaListOrUTF8String&>(aUnion))
  {
  }

  inline bool
  TrySetToMediaList(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::MediaList>& memberSlot = RawSetAsMediaList();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::MediaList, mozilla::dom::MediaList>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyMediaList();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToMediaList(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToMediaList(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToUTF8String(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char>& memberSlot = RawSetAsUTF8String();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsCString::char_type (&aData)[N])
  {
    RawSetAsUTF8String().AssignLiteral(aData);
  }

private:
  inline NonNull<mozilla::dom::MediaList>&
  RawSetAsMediaList()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eMediaList;
    return mUnion.mValue.mMediaList.SetValue();
  }

  inline binding_detail::FakeString<char>&
  RawSetAsUTF8String()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUTF8String;
    return mUnion.mValue.mUTF8String.SetValue();
  }
};

class MediaStreamTrackOrStringArgument
{
  MediaStreamTrackOrString& mUnion;

  MediaStreamTrackOrStringArgument(const MediaStreamTrackOrStringArgument&) = delete;
  MediaStreamTrackOrStringArgument& operator=(const MediaStreamTrackOrStringArgument&) = delete;
public:
  explicit inline MediaStreamTrackOrStringArgument(const MediaStreamTrackOrString& aUnion)
    : mUnion(const_cast<MediaStreamTrackOrString&>(aUnion))
  {
  }

  inline bool
  TrySetToMediaStreamTrack(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::MediaStreamTrack>& memberSlot = RawSetAsMediaStreamTrack();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::MediaStreamTrack, mozilla::dom::MediaStreamTrack>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyMediaStreamTrack();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToMediaStreamTrack(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToMediaStreamTrack(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsString().AssignLiteral(aData);
  }

private:
  inline NonNull<mozilla::dom::MediaStreamTrack>&
  RawSetAsMediaStreamTrack()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eMediaStreamTrack;
    return mUnion.mValue.mMediaStreamTrack.SetValue();
  }

  inline binding_detail::FakeString<char16_t>&
  RawSetAsString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eString;
    return mUnion.mValue.mString.SetValue();
  }
};

class NodeOrStringArgument
{
  NodeOrString& mUnion;

  NodeOrStringArgument(const NodeOrStringArgument&) = delete;
  NodeOrStringArgument& operator=(const NodeOrStringArgument&) = delete;
public:
  explicit inline NodeOrStringArgument(const NodeOrString& aUnion)
    : mUnion(const_cast<NodeOrString&>(aUnion))
  {
  }

  inline bool
  TrySetToNode(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<nsINode>& memberSlot = RawSetAsNode();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::Node, nsINode>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyNode();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToNode(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToNode(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsString().AssignLiteral(aData);
  }

private:
  inline NonNull<nsINode>&
  RawSetAsNode()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eNode;
    return mUnion.mValue.mNode.SetValue();
  }

  inline binding_detail::FakeString<char16_t>&
  RawSetAsString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eString;
    return mUnion.mValue.mString.SetValue();
  }
};

class ObjectOrLongArgument
{
  ObjectOrLong& mUnion;

  ObjectOrLongArgument(const ObjectOrLongArgument&) = delete;
  ObjectOrLongArgument& operator=(const ObjectOrLongArgument&) = delete;
public:
  explicit inline ObjectOrLongArgument(const ObjectOrLong& aUnion)
    : mUnion(const_cast<ObjectOrLong&>(aUnion))
  {
  }

  inline bool
  SetToObject(BindingCallContext& cx, JSObject* obj, bool passedToJSImpl = false)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mValue.mObject.SetValue(cx, obj);
    mUnion.mType = mUnion.eObject;
    if (passedToJSImpl && !CallerSubsumes(obj)) {
      cx.ThrowErrorMessage<MSG_PERMISSION_DENIED_TO_PASS_ARG>("object branch of (object or long)");
      return false;
    }
    return true;
  }

  inline bool
  TrySetToLong(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      int32_t& memberSlot = RawSetAsLong();
      if (!ValueToPrimitive<int32_t, eDefault>(cx, value, "Long branch of (object or long)", &memberSlot)) {
        return false;
      }
    }
    return true;
  }

private:
  inline int32_t&
  RawSetAsLong()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eLong;
    return mUnion.mValue.mLong.SetValue();
  }
};

class ObjectOrNullOrLongArgument
{
  ObjectOrNullOrLong& mUnion;

  ObjectOrNullOrLongArgument(const ObjectOrNullOrLongArgument&) = delete;
  ObjectOrNullOrLongArgument& operator=(const ObjectOrNullOrLongArgument&) = delete;
public:
  explicit inline ObjectOrNullOrLongArgument(const ObjectOrNullOrLong& aUnion)
    : mUnion(const_cast<ObjectOrNullOrLong&>(aUnion))
  {
  }

  inline bool
  SetNull()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eNull;
    return true;
  }

  inline bool
  SetToObject(BindingCallContext& cx, JSObject* obj, bool passedToJSImpl = false)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mValue.mObject.SetValue(cx, obj);
    mUnion.mType = mUnion.eObject;
    if (passedToJSImpl && !CallerSubsumes(obj)) {
      cx.ThrowErrorMessage<MSG_PERMISSION_DENIED_TO_PASS_ARG>("object branch of (object? or long)");
      return false;
    }
    return true;
  }

  inline bool
  TrySetToLong(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      int32_t& memberSlot = RawSetAsLong();
      if (!ValueToPrimitive<int32_t, eDefault>(cx, value, "Long branch of (object? or long)", &memberSlot)) {
        return false;
      }
    }
    return true;
  }

private:
  inline int32_t&
  RawSetAsLong()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eLong;
    return mUnion.mValue.mLong.SetValue();
  }
};

class ObjectOrStringArgument
{
  ObjectOrString& mUnion;

  ObjectOrStringArgument(const ObjectOrStringArgument&) = delete;
  ObjectOrStringArgument& operator=(const ObjectOrStringArgument&) = delete;
public:
  explicit inline ObjectOrStringArgument(const ObjectOrString& aUnion)
    : mUnion(const_cast<ObjectOrString&>(aUnion))
  {
  }

  inline bool
  SetToObject(BindingCallContext& cx, JSObject* obj, bool passedToJSImpl = false)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mValue.mObject.SetValue(cx, obj);
    mUnion.mType = mUnion.eObject;
    if (passedToJSImpl && !CallerSubsumes(obj)) {
      cx.ThrowErrorMessage<MSG_PERMISSION_DENIED_TO_PASS_ARG>("object branch of (object or DOMString)");
      return false;
    }
    return true;
  }

  inline bool
  TrySetToString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsString().AssignLiteral(aData);
  }

private:
  inline binding_detail::FakeString<char16_t>&
  RawSetAsString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eString;
    return mUnion.mValue.mString.SetValue();
  }
};

class ProfilerMarkerOptionsOrDoubleArgument
{
  ProfilerMarkerOptionsOrDouble& mUnion;

  ProfilerMarkerOptionsOrDoubleArgument(const ProfilerMarkerOptionsOrDoubleArgument&) = delete;
  ProfilerMarkerOptionsOrDoubleArgument& operator=(const ProfilerMarkerOptionsOrDoubleArgument&) = delete;
public:
  explicit inline ProfilerMarkerOptionsOrDoubleArgument(const ProfilerMarkerOptionsOrDouble& aUnion)
    : mUnion(const_cast<ProfilerMarkerOptionsOrDouble&>(aUnion))
  {
  }

  inline bool
  TrySetToProfilerMarkerOptions(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FastProfilerMarkerOptions& memberSlot = RawSetAsProfilerMarkerOptions();
      if (!IsConvertibleToDictionary(value)) {
        mUnion.DestroyProfilerMarkerOptions();
        tryNext = true;
        return true;
      }
      if (!memberSlot.Init(cx, value, "ProfilerMarkerOptions branch of (ProfilerMarkerOptions or double)", passedToJSImpl)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToProfilerMarkerOptions(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToProfilerMarkerOptions(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToDouble(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      double& memberSlot = RawSetAsDouble();
      if (!ValueToPrimitive<double, eDefault>(cx, value, "Double branch of (ProfilerMarkerOptions or double)", &memberSlot)) {
        return false;
      } else if (!mozilla::IsFinite(memberSlot)) {
        cx.ThrowErrorMessage<MSG_NOT_FINITE>("Double branch of (ProfilerMarkerOptions or double)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToDouble(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToDouble(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline binding_detail::FastProfilerMarkerOptions&
  RawSetAsProfilerMarkerOptions()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eProfilerMarkerOptions;
    return mUnion.mValue.mProfilerMarkerOptions.SetValue();
  }

  inline double&
  RawSetAsDouble()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eDouble;
    return mUnion.mValue.mDouble.SetValue();
  }
};

class RTCIceCandidateInitOrRTCIceCandidateArgument
{
  RTCIceCandidateInitOrRTCIceCandidate& mUnion;

  RTCIceCandidateInitOrRTCIceCandidateArgument(const RTCIceCandidateInitOrRTCIceCandidateArgument&) = delete;
  RTCIceCandidateInitOrRTCIceCandidateArgument& operator=(const RTCIceCandidateInitOrRTCIceCandidateArgument&) = delete;
public:
  explicit inline RTCIceCandidateInitOrRTCIceCandidateArgument(const RTCIceCandidateInitOrRTCIceCandidate& aUnion)
    : mUnion(const_cast<RTCIceCandidateInitOrRTCIceCandidate&>(aUnion))
  {
  }

  inline bool
  TrySetToRTCIceCandidateInit(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FastRTCIceCandidateInit& memberSlot = RawSetAsRTCIceCandidateInit();
      if (!IsConvertibleToDictionary(value)) {
        mUnion.DestroyRTCIceCandidateInit();
        tryNext = true;
        return true;
      }
      if (!memberSlot.Init(cx, value, "RTCIceCandidateInit branch of (RTCIceCandidateInit or RTCIceCandidate)", passedToJSImpl)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToRTCIceCandidateInit(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToRTCIceCandidateInit(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToRTCIceCandidate(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::RTCIceCandidate>& memberSlot = RawSetAsRTCIceCandidate();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::RTCIceCandidate, mozilla::dom::RTCIceCandidate>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyRTCIceCandidate();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToRTCIceCandidate(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToRTCIceCandidate(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline binding_detail::FastRTCIceCandidateInit&
  RawSetAsRTCIceCandidateInit()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eRTCIceCandidateInit;
    return mUnion.mValue.mRTCIceCandidateInit.SetValue();
  }

  inline NonNull<mozilla::dom::RTCIceCandidate>&
  RawSetAsRTCIceCandidate()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eRTCIceCandidate;
    return mUnion.mValue.mRTCIceCandidate.SetValue();
  }
};

class RadioNodeListOrElementArgument
{
  RadioNodeListOrElement& mUnion;

  RadioNodeListOrElementArgument(const RadioNodeListOrElementArgument&) = delete;
  RadioNodeListOrElementArgument& operator=(const RadioNodeListOrElementArgument&) = delete;
public:
  explicit inline RadioNodeListOrElementArgument(const RadioNodeListOrElement& aUnion)
    : mUnion(const_cast<RadioNodeListOrElement&>(aUnion))
  {
  }

  inline bool
  TrySetToRadioNodeList(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::RadioNodeList>& memberSlot = RawSetAsRadioNodeList();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::RadioNodeList, mozilla::dom::RadioNodeList>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyRadioNodeList();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToRadioNodeList(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToRadioNodeList(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToElement(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::Element>& memberSlot = RawSetAsElement();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::Element, mozilla::dom::Element>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyElement();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToElement(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToElement(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline NonNull<mozilla::dom::RadioNodeList>&
  RawSetAsRadioNodeList()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eRadioNodeList;
    return mUnion.mValue.mRadioNodeList.SetValue();
  }

  inline NonNull<mozilla::dom::Element>&
  RawSetAsElement()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eElement;
    return mUnion.mValue.mElement.SetValue();
  }
};

class RangeEnforcedUnsignedLongSequenceOrGPUExtent3DDictArgument
{
  RangeEnforcedUnsignedLongSequenceOrGPUExtent3DDict& mUnion;

  RangeEnforcedUnsignedLongSequenceOrGPUExtent3DDictArgument(const RangeEnforcedUnsignedLongSequenceOrGPUExtent3DDictArgument&) = delete;
  RangeEnforcedUnsignedLongSequenceOrGPUExtent3DDictArgument& operator=(const RangeEnforcedUnsignedLongSequenceOrGPUExtent3DDictArgument&) = delete;
public:
  explicit inline RangeEnforcedUnsignedLongSequenceOrGPUExtent3DDictArgument(const RangeEnforcedUnsignedLongSequenceOrGPUExtent3DDict& aUnion)
    : mUnion(const_cast<RangeEnforcedUnsignedLongSequenceOrGPUExtent3DDict&>(aUnion))
  {
  }

  inline bool
  TrySetToRangeEnforcedUnsignedLongSequence(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::AutoSequence<uint32_t>& memberSlot = RawSetAsRangeEnforcedUnsignedLongSequence();
      JS::ForOfIterator iter(cx);
      if (!iter.init(value, JS::ForOfIterator::AllowNonIterable)) {
        return false;
      }
      if (!iter.valueIsIterable()) {
        mUnion.DestroyRangeEnforcedUnsignedLongSequence();
        tryNext = true;
        return true;
      }
      binding_detail::AutoSequence<uint32_t> &arr = memberSlot;
      JS::Rooted<JS::Value> temp(cx);
      while (true) {
        bool done;
        if (!iter.next(&temp, &done)) {
          return false;
        }
        if (done) {
          break;
        }
        uint32_t* slotPtr = arr.AppendElement(mozilla::fallible);
        if (!slotPtr) {
          JS_ReportOutOfMemory(cx);
          return false;
        }
        uint32_t& slot = *slotPtr;
        if (!ValueToPrimitive<uint32_t, eEnforceRange>(cx, temp, "Element of sequence<unsigned long> branch of (sequence<unsigned long> or GPUExtent3DDict)", &slot)) {
          return false;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToRangeEnforcedUnsignedLongSequence(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToRangeEnforcedUnsignedLongSequence(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToGPUExtent3DDict(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FastGPUExtent3DDict& memberSlot = RawSetAsGPUExtent3DDict();
      if (!IsConvertibleToDictionary(value)) {
        mUnion.DestroyGPUExtent3DDict();
        tryNext = true;
        return true;
      }
      if (!memberSlot.Init(cx, value, "GPUExtent3DDict branch of (sequence<unsigned long> or GPUExtent3DDict)", passedToJSImpl)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToGPUExtent3DDict(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToGPUExtent3DDict(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline binding_detail::AutoSequence<uint32_t>&
  RawSetAsRangeEnforcedUnsignedLongSequence()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eRangeEnforcedUnsignedLongSequence;
    return mUnion.mValue.mRangeEnforcedUnsignedLongSequence.SetValue();
  }

  inline binding_detail::FastGPUExtent3DDict&
  RawSetAsGPUExtent3DDict()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eGPUExtent3DDict;
    return mUnion.mValue.mGPUExtent3DDict.SetValue();
  }
};

class RangeEnforcedUnsignedLongSequenceOrGPUOrigin2DDictArgument
{
  RangeEnforcedUnsignedLongSequenceOrGPUOrigin2DDict& mUnion;

  RangeEnforcedUnsignedLongSequenceOrGPUOrigin2DDictArgument(const RangeEnforcedUnsignedLongSequenceOrGPUOrigin2DDictArgument&) = delete;
  RangeEnforcedUnsignedLongSequenceOrGPUOrigin2DDictArgument& operator=(const RangeEnforcedUnsignedLongSequenceOrGPUOrigin2DDictArgument&) = delete;
public:
  explicit inline RangeEnforcedUnsignedLongSequenceOrGPUOrigin2DDictArgument(const RangeEnforcedUnsignedLongSequenceOrGPUOrigin2DDict& aUnion)
    : mUnion(const_cast<RangeEnforcedUnsignedLongSequenceOrGPUOrigin2DDict&>(aUnion))
  {
  }

  inline bool
  TrySetToRangeEnforcedUnsignedLongSequence(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::AutoSequence<uint32_t>& memberSlot = RawSetAsRangeEnforcedUnsignedLongSequence();
      JS::ForOfIterator iter(cx);
      if (!iter.init(value, JS::ForOfIterator::AllowNonIterable)) {
        return false;
      }
      if (!iter.valueIsIterable()) {
        mUnion.DestroyRangeEnforcedUnsignedLongSequence();
        tryNext = true;
        return true;
      }
      binding_detail::AutoSequence<uint32_t> &arr = memberSlot;
      JS::Rooted<JS::Value> temp(cx);
      while (true) {
        bool done;
        if (!iter.next(&temp, &done)) {
          return false;
        }
        if (done) {
          break;
        }
        uint32_t* slotPtr = arr.AppendElement(mozilla::fallible);
        if (!slotPtr) {
          JS_ReportOutOfMemory(cx);
          return false;
        }
        uint32_t& slot = *slotPtr;
        if (!ValueToPrimitive<uint32_t, eEnforceRange>(cx, temp, "Element of sequence<unsigned long> branch of (sequence<unsigned long> or GPUOrigin2DDict)", &slot)) {
          return false;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToRangeEnforcedUnsignedLongSequence(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToRangeEnforcedUnsignedLongSequence(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline binding_detail::AutoSequence<uint32_t>&
  RawSetAsRangeEnforcedUnsignedLongSequence()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eRangeEnforcedUnsignedLongSequence;
    return mUnion.mValue.mRangeEnforcedUnsignedLongSequence.SetValue();
  }

  inline binding_detail::FastGPUOrigin2DDict&
  RawSetAsGPUOrigin2DDict()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eGPUOrigin2DDict;
    return mUnion.mValue.mGPUOrigin2DDict.SetValue();
  }
};

class RangeEnforcedUnsignedLongSequenceOrGPUOrigin3DDictArgument
{
  RangeEnforcedUnsignedLongSequenceOrGPUOrigin3DDict& mUnion;

  RangeEnforcedUnsignedLongSequenceOrGPUOrigin3DDictArgument(const RangeEnforcedUnsignedLongSequenceOrGPUOrigin3DDictArgument&) = delete;
  RangeEnforcedUnsignedLongSequenceOrGPUOrigin3DDictArgument& operator=(const RangeEnforcedUnsignedLongSequenceOrGPUOrigin3DDictArgument&) = delete;
public:
  explicit inline RangeEnforcedUnsignedLongSequenceOrGPUOrigin3DDictArgument(const RangeEnforcedUnsignedLongSequenceOrGPUOrigin3DDict& aUnion)
    : mUnion(const_cast<RangeEnforcedUnsignedLongSequenceOrGPUOrigin3DDict&>(aUnion))
  {
  }

  inline bool
  TrySetToRangeEnforcedUnsignedLongSequence(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::AutoSequence<uint32_t>& memberSlot = RawSetAsRangeEnforcedUnsignedLongSequence();
      JS::ForOfIterator iter(cx);
      if (!iter.init(value, JS::ForOfIterator::AllowNonIterable)) {
        return false;
      }
      if (!iter.valueIsIterable()) {
        mUnion.DestroyRangeEnforcedUnsignedLongSequence();
        tryNext = true;
        return true;
      }
      binding_detail::AutoSequence<uint32_t> &arr = memberSlot;
      JS::Rooted<JS::Value> temp(cx);
      while (true) {
        bool done;
        if (!iter.next(&temp, &done)) {
          return false;
        }
        if (done) {
          break;
        }
        uint32_t* slotPtr = arr.AppendElement(mozilla::fallible);
        if (!slotPtr) {
          JS_ReportOutOfMemory(cx);
          return false;
        }
        uint32_t& slot = *slotPtr;
        if (!ValueToPrimitive<uint32_t, eEnforceRange>(cx, temp, "Element of sequence<unsigned long> branch of (sequence<unsigned long> or GPUOrigin3DDict)", &slot)) {
          return false;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToRangeEnforcedUnsignedLongSequence(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToRangeEnforcedUnsignedLongSequence(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToGPUOrigin3DDict(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FastGPUOrigin3DDict& memberSlot = RawSetAsGPUOrigin3DDict();
      if (!IsConvertibleToDictionary(value)) {
        mUnion.DestroyGPUOrigin3DDict();
        tryNext = true;
        return true;
      }
      if (!memberSlot.Init(cx, value, "GPUOrigin3DDict branch of (sequence<unsigned long> or GPUOrigin3DDict)", passedToJSImpl)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToGPUOrigin3DDict(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToGPUOrigin3DDict(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline binding_detail::AutoSequence<uint32_t>&
  RawSetAsRangeEnforcedUnsignedLongSequence()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eRangeEnforcedUnsignedLongSequence;
    return mUnion.mValue.mRangeEnforcedUnsignedLongSequence.SetValue();
  }

  inline binding_detail::FastGPUOrigin3DDict&
  RawSetAsGPUOrigin3DDict()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eGPUOrigin3DDict;
    return mUnion.mValue.mGPUOrigin3DDict.SetValue();
  }
};

class RequestOrUSVStringArgument
{
  RequestOrUSVString& mUnion;

  RequestOrUSVStringArgument(const RequestOrUSVStringArgument&) = delete;
  RequestOrUSVStringArgument& operator=(const RequestOrUSVStringArgument&) = delete;
public:
  explicit inline RequestOrUSVStringArgument(const RequestOrUSVString& aUnion)
    : mUnion(const_cast<RequestOrUSVString&>(aUnion))
  {
  }

  inline bool
  TrySetToRequest(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::Request>& memberSlot = RawSetAsRequest();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::Request, mozilla::dom::Request>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyRequest();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToRequest(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToRequest(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToUSVString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsUSVString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
      if (!NormalizeUSVString(memberSlot)) {
        JS_ReportOutOfMemory(cx);
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsUSVString().AssignLiteral(aData);
  }

private:
  inline NonNull<mozilla::dom::Request>&
  RawSetAsRequest()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eRequest;
    return mUnion.mValue.mRequest.SetValue();
  }

  inline binding_detail::FakeString<char16_t>&
  RawSetAsUSVString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUSVString;
    return mUnion.mValue.mUSVString.SetValue();
  }
};

class StringOrArrayBufferArgument
{
  StringOrArrayBuffer& mUnion;

  StringOrArrayBufferArgument(const StringOrArrayBufferArgument&) = delete;
  StringOrArrayBufferArgument& operator=(const StringOrArrayBufferArgument&) = delete;
public:
  explicit inline StringOrArrayBufferArgument(const StringOrArrayBuffer& aUnion)
    : mUnion(const_cast<StringOrArrayBuffer&>(aUnion))
  {
  }

  inline bool
  TrySetToString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsString().AssignLiteral(aData);
  }

  inline bool
  TrySetToArrayBuffer(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      RootedSpiderMonkeyInterface<ArrayBuffer>& memberSlot = RawSetAsArrayBuffer(cx);
      if (!memberSlot.Init(&value.toObject())) {
        mUnion.DestroyArrayBuffer();
        tryNext = true;
        return true;
      }
      if (JS::IsSharedArrayBufferObject(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_SHARED>("ArrayBuffer branch of (DOMString or ArrayBuffer)");
        return false;
      }
      if (JS::IsLargeArrayBufferMaybeShared(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_LARGE>("ArrayBuffer branch of (DOMString or ArrayBuffer)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToArrayBuffer(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToArrayBuffer(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline binding_detail::FakeString<char16_t>&
  RawSetAsString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eString;
    return mUnion.mValue.mString.SetValue();
  }

  inline RootedSpiderMonkeyInterface<ArrayBuffer>&
  RawSetAsArrayBuffer(JSContext* cx)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eArrayBuffer;
    return mUnion.mValue.mArrayBuffer.SetValue(cx);
  }
};

class StringOrArrayBufferOrArrayBufferViewOrBlobArgument
{
  StringOrArrayBufferOrArrayBufferViewOrBlob& mUnion;

  StringOrArrayBufferOrArrayBufferViewOrBlobArgument(const StringOrArrayBufferOrArrayBufferViewOrBlobArgument&) = delete;
  StringOrArrayBufferOrArrayBufferViewOrBlobArgument& operator=(const StringOrArrayBufferOrArrayBufferViewOrBlobArgument&) = delete;
public:
  explicit inline StringOrArrayBufferOrArrayBufferViewOrBlobArgument(const StringOrArrayBufferOrArrayBufferViewOrBlob& aUnion)
    : mUnion(const_cast<StringOrArrayBufferOrArrayBufferViewOrBlob&>(aUnion))
  {
  }

  inline bool
  TrySetToString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsString().AssignLiteral(aData);
  }

  inline bool
  TrySetToArrayBuffer(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      RootedSpiderMonkeyInterface<ArrayBuffer>& memberSlot = RawSetAsArrayBuffer(cx);
      if (!memberSlot.Init(&value.toObject())) {
        mUnion.DestroyArrayBuffer();
        tryNext = true;
        return true;
      }
      if (JS::IsSharedArrayBufferObject(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_SHARED>("ArrayBuffer branch of (DOMString or ArrayBuffer or ArrayBufferView or Blob)");
        return false;
      }
      if (JS::IsLargeArrayBufferMaybeShared(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_LARGE>("ArrayBuffer branch of (DOMString or ArrayBuffer or ArrayBufferView or Blob)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToArrayBuffer(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToArrayBuffer(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToArrayBufferView(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      RootedSpiderMonkeyInterface<ArrayBufferView>& memberSlot = RawSetAsArrayBufferView(cx);
      if (!memberSlot.Init(&value.toObject())) {
        mUnion.DestroyArrayBufferView();
        tryNext = true;
        return true;
      }
      if (JS::IsArrayBufferViewShared(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_SHARED>("ArrayBufferView branch of (DOMString or ArrayBuffer or ArrayBufferView or Blob)");
        return false;
      }
      if (JS::IsLargeArrayBufferView(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_LARGE>("ArrayBufferView branch of (DOMString or ArrayBuffer or ArrayBufferView or Blob)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToArrayBufferView(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToArrayBufferView(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToBlob(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::Blob>& memberSlot = RawSetAsBlob();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::Blob, mozilla::dom::Blob>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyBlob();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToBlob(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToBlob(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline binding_detail::FakeString<char16_t>&
  RawSetAsString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eString;
    return mUnion.mValue.mString.SetValue();
  }

  inline RootedSpiderMonkeyInterface<ArrayBuffer>&
  RawSetAsArrayBuffer(JSContext* cx)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eArrayBuffer;
    return mUnion.mValue.mArrayBuffer.SetValue(cx);
  }

  inline RootedSpiderMonkeyInterface<ArrayBufferView>&
  RawSetAsArrayBufferView(JSContext* cx)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eArrayBufferView;
    return mUnion.mValue.mArrayBufferView.SetValue(cx);
  }

  inline NonNull<mozilla::dom::Blob>&
  RawSetAsBlob()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eBlob;
    return mUnion.mValue.mBlob.SetValue();
  }
};

class StringOrBlobArgument
{
  StringOrBlob& mUnion;

  StringOrBlobArgument(const StringOrBlobArgument&) = delete;
  StringOrBlobArgument& operator=(const StringOrBlobArgument&) = delete;
public:
  explicit inline StringOrBlobArgument(const StringOrBlob& aUnion)
    : mUnion(const_cast<StringOrBlob&>(aUnion))
  {
  }

  inline bool
  TrySetToString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsString().AssignLiteral(aData);
  }

  inline bool
  TrySetToBlob(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::Blob>& memberSlot = RawSetAsBlob();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::Blob, mozilla::dom::Blob>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyBlob();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToBlob(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToBlob(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline binding_detail::FakeString<char16_t>&
  RawSetAsString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eString;
    return mUnion.mValue.mString.SetValue();
  }

  inline NonNull<mozilla::dom::Blob>&
  RawSetAsBlob()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eBlob;
    return mUnion.mValue.mBlob.SetValue();
  }
};

class StringOrBlobOrArrayBufferOrArrayBufferViewArgument
{
  StringOrBlobOrArrayBufferOrArrayBufferView& mUnion;

  StringOrBlobOrArrayBufferOrArrayBufferViewArgument(const StringOrBlobOrArrayBufferOrArrayBufferViewArgument&) = delete;
  StringOrBlobOrArrayBufferOrArrayBufferViewArgument& operator=(const StringOrBlobOrArrayBufferOrArrayBufferViewArgument&) = delete;
public:
  explicit inline StringOrBlobOrArrayBufferOrArrayBufferViewArgument(const StringOrBlobOrArrayBufferOrArrayBufferView& aUnion)
    : mUnion(const_cast<StringOrBlobOrArrayBufferOrArrayBufferView&>(aUnion))
  {
  }

  inline bool
  TrySetToString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsString().AssignLiteral(aData);
  }

  inline bool
  TrySetToBlob(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::Blob>& memberSlot = RawSetAsBlob();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::Blob, mozilla::dom::Blob>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyBlob();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToBlob(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToBlob(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToArrayBuffer(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      RootedSpiderMonkeyInterface<ArrayBuffer>& memberSlot = RawSetAsArrayBuffer(cx);
      if (!memberSlot.Init(&value.toObject())) {
        mUnion.DestroyArrayBuffer();
        tryNext = true;
        return true;
      }
      if (JS::IsSharedArrayBufferObject(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_SHARED>("ArrayBuffer branch of (DOMString or Blob or ArrayBuffer or ArrayBufferView)");
        return false;
      }
      if (JS::IsLargeArrayBufferMaybeShared(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_LARGE>("ArrayBuffer branch of (DOMString or Blob or ArrayBuffer or ArrayBufferView)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToArrayBuffer(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToArrayBuffer(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToArrayBufferView(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      RootedSpiderMonkeyInterface<ArrayBufferView>& memberSlot = RawSetAsArrayBufferView(cx);
      if (!memberSlot.Init(&value.toObject())) {
        mUnion.DestroyArrayBufferView();
        tryNext = true;
        return true;
      }
      if (JS::IsArrayBufferViewShared(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_SHARED>("ArrayBufferView branch of (DOMString or Blob or ArrayBuffer or ArrayBufferView)");
        return false;
      }
      if (JS::IsLargeArrayBufferView(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_LARGE>("ArrayBufferView branch of (DOMString or Blob or ArrayBuffer or ArrayBufferView)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToArrayBufferView(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToArrayBufferView(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline binding_detail::FakeString<char16_t>&
  RawSetAsString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eString;
    return mUnion.mValue.mString.SetValue();
  }

  inline NonNull<mozilla::dom::Blob>&
  RawSetAsBlob()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eBlob;
    return mUnion.mValue.mBlob.SetValue();
  }

  inline RootedSpiderMonkeyInterface<ArrayBuffer>&
  RawSetAsArrayBuffer(JSContext* cx)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eArrayBuffer;
    return mUnion.mValue.mArrayBuffer.SetValue(cx);
  }

  inline RootedSpiderMonkeyInterface<ArrayBufferView>&
  RawSetAsArrayBufferView(JSContext* cx)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eArrayBufferView;
    return mUnion.mValue.mArrayBufferView.SetValue(cx);
  }
};

class StringOrBooleanOrObjectArgument
{
  StringOrBooleanOrObject& mUnion;

  StringOrBooleanOrObjectArgument(const StringOrBooleanOrObjectArgument&) = delete;
  StringOrBooleanOrObjectArgument& operator=(const StringOrBooleanOrObjectArgument&) = delete;
public:
  explicit inline StringOrBooleanOrObjectArgument(const StringOrBooleanOrObject& aUnion)
    : mUnion(const_cast<StringOrBooleanOrObject&>(aUnion))
  {
  }

  inline bool
  TrySetToString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsString().AssignLiteral(aData);
  }

  inline bool
  TrySetToBoolean(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      bool& memberSlot = RawSetAsBoolean();
      if (!ValueToPrimitive<bool, eDefault>(cx, value, "Boolean branch of (DOMString or boolean or object)", &memberSlot)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  SetToObject(BindingCallContext& cx, JSObject* obj, bool passedToJSImpl = false)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mValue.mObject.SetValue(cx, obj);
    mUnion.mType = mUnion.eObject;
    if (passedToJSImpl && !CallerSubsumes(obj)) {
      cx.ThrowErrorMessage<MSG_PERMISSION_DENIED_TO_PASS_ARG>("object branch of (DOMString or boolean or object)");
      return false;
    }
    return true;
  }

private:
  inline binding_detail::FakeString<char16_t>&
  RawSetAsString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eString;
    return mUnion.mValue.mString.SetValue();
  }

  inline bool&
  RawSetAsBoolean()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eBoolean;
    return mUnion.mValue.mBoolean.SetValue();
  }
};

class StringOrDocumentFragmentOrDocumentArgument
{
  StringOrDocumentFragmentOrDocument& mUnion;

  StringOrDocumentFragmentOrDocumentArgument(const StringOrDocumentFragmentOrDocumentArgument&) = delete;
  StringOrDocumentFragmentOrDocumentArgument& operator=(const StringOrDocumentFragmentOrDocumentArgument&) = delete;
public:
  explicit inline StringOrDocumentFragmentOrDocumentArgument(const StringOrDocumentFragmentOrDocument& aUnion)
    : mUnion(const_cast<StringOrDocumentFragmentOrDocument&>(aUnion))
  {
  }

  inline bool
  TrySetToString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsString().AssignLiteral(aData);
  }

  inline bool
  TrySetToDocumentFragment(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::DocumentFragment>& memberSlot = RawSetAsDocumentFragment();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::DocumentFragment, mozilla::dom::DocumentFragment>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyDocumentFragment();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToDocumentFragment(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToDocumentFragment(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToDocument(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::Document>& memberSlot = RawSetAsDocument();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::Document, mozilla::dom::Document>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyDocument();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToDocument(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToDocument(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline binding_detail::FakeString<char16_t>&
  RawSetAsString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eString;
    return mUnion.mValue.mString.SetValue();
  }

  inline NonNull<mozilla::dom::DocumentFragment>&
  RawSetAsDocumentFragment()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eDocumentFragment;
    return mUnion.mValue.mDocumentFragment.SetValue();
  }

  inline NonNull<mozilla::dom::Document>&
  RawSetAsDocument()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eDocument;
    return mUnion.mValue.mDocument.SetValue();
  }
};

class StringOrInstallTriggerDataArgument
{
  StringOrInstallTriggerData& mUnion;

  StringOrInstallTriggerDataArgument(const StringOrInstallTriggerDataArgument&) = delete;
  StringOrInstallTriggerDataArgument& operator=(const StringOrInstallTriggerDataArgument&) = delete;
public:
  explicit inline StringOrInstallTriggerDataArgument(const StringOrInstallTriggerData& aUnion)
    : mUnion(const_cast<StringOrInstallTriggerData&>(aUnion))
  {
  }

  inline bool
  TrySetToString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsString().AssignLiteral(aData);
  }

  inline bool
  TrySetToInstallTriggerData(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FastInstallTriggerData& memberSlot = RawSetAsInstallTriggerData();
      if (!IsConvertibleToDictionary(value)) {
        mUnion.DestroyInstallTriggerData();
        tryNext = true;
        return true;
      }
      if (!memberSlot.Init(cx, value, "InstallTriggerData branch of (DOMString or InstallTriggerData)", passedToJSImpl)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToInstallTriggerData(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToInstallTriggerData(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline binding_detail::FakeString<char16_t>&
  RawSetAsString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eString;
    return mUnion.mValue.mString.SetValue();
  }

  inline binding_detail::FastInstallTriggerData&
  RawSetAsInstallTriggerData()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eInstallTriggerData;
    return mUnion.mValue.mInstallTriggerData.SetValue();
  }
};

class StringOrMatchPatternArgument
{
  StringOrMatchPattern& mUnion;

  StringOrMatchPatternArgument(const StringOrMatchPatternArgument&) = delete;
  StringOrMatchPatternArgument& operator=(const StringOrMatchPatternArgument&) = delete;
public:
  explicit inline StringOrMatchPatternArgument(const StringOrMatchPattern& aUnion)
    : mUnion(const_cast<StringOrMatchPattern&>(aUnion))
  {
  }

  inline bool
  TrySetToString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsString().AssignLiteral(aData);
  }

  inline bool
  TrySetToMatchPattern(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::extensions::MatchPattern>& memberSlot = RawSetAsMatchPattern();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::MatchPattern, mozilla::extensions::MatchPattern>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyMatchPattern();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToMatchPattern(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToMatchPattern(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline binding_detail::FakeString<char16_t>&
  RawSetAsString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eString;
    return mUnion.mValue.mString.SetValue();
  }

  inline NonNull<mozilla::extensions::MatchPattern>&
  RawSetAsMatchPattern()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eMatchPattern;
    return mUnion.mValue.mMatchPattern.SetValue();
  }
};

class StringOrMaybeSharedArrayBufferArgument
{
  StringOrMaybeSharedArrayBuffer& mUnion;

  StringOrMaybeSharedArrayBufferArgument(const StringOrMaybeSharedArrayBufferArgument&) = delete;
  StringOrMaybeSharedArrayBufferArgument& operator=(const StringOrMaybeSharedArrayBufferArgument&) = delete;
public:
  explicit inline StringOrMaybeSharedArrayBufferArgument(const StringOrMaybeSharedArrayBuffer& aUnion)
    : mUnion(const_cast<StringOrMaybeSharedArrayBuffer&>(aUnion))
  {
  }

  inline bool
  TrySetToString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsString().AssignLiteral(aData);
  }

  inline bool
  TrySetToArrayBuffer(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      RootedSpiderMonkeyInterface<ArrayBuffer>& memberSlot = RawSetAsArrayBuffer(cx);
      if (!memberSlot.Init(&value.toObject())) {
        mUnion.DestroyArrayBuffer();
        tryNext = true;
        return true;
      }
      if (JS::IsLargeArrayBufferMaybeShared(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_LARGE>("ArrayBuffer branch of (DOMString or ArrayBuffer)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToArrayBuffer(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToArrayBuffer(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline binding_detail::FakeString<char16_t>&
  RawSetAsString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eString;
    return mUnion.mValue.mString.SetValue();
  }

  inline RootedSpiderMonkeyInterface<ArrayBuffer>&
  RawSetAsArrayBuffer(JSContext* cx)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eArrayBuffer;
    return mUnion.mValue.mArrayBuffer.SetValue(cx);
  }
};

class StringOrObjectArgument
{
  StringOrObject& mUnion;

  StringOrObjectArgument(const StringOrObjectArgument&) = delete;
  StringOrObjectArgument& operator=(const StringOrObjectArgument&) = delete;
public:
  explicit inline StringOrObjectArgument(const StringOrObject& aUnion)
    : mUnion(const_cast<StringOrObject&>(aUnion))
  {
  }

  inline bool
  TrySetToString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsString().AssignLiteral(aData);
  }

  inline bool
  SetToObject(BindingCallContext& cx, JSObject* obj, bool passedToJSImpl = false)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mValue.mObject.SetValue(cx, obj);
    mUnion.mType = mUnion.eObject;
    if (passedToJSImpl && !CallerSubsumes(obj)) {
      cx.ThrowErrorMessage<MSG_PERMISSION_DENIED_TO_PASS_ARG>("object branch of (DOMString or object)");
      return false;
    }
    return true;
  }

private:
  inline binding_detail::FakeString<char16_t>&
  RawSetAsString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eString;
    return mUnion.mValue.mString.SetValue();
  }
};

class StringOrOpenPopupOptionsArgument
{
  StringOrOpenPopupOptions& mUnion;

  StringOrOpenPopupOptionsArgument(const StringOrOpenPopupOptionsArgument&) = delete;
  StringOrOpenPopupOptionsArgument& operator=(const StringOrOpenPopupOptionsArgument&) = delete;
public:
  explicit inline StringOrOpenPopupOptionsArgument(const StringOrOpenPopupOptions& aUnion)
    : mUnion(const_cast<StringOrOpenPopupOptions&>(aUnion))
  {
  }

  inline bool
  TrySetToString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsString().AssignLiteral(aData);
  }

  inline bool
  TrySetToOpenPopupOptions(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FastOpenPopupOptions& memberSlot = RawSetAsOpenPopupOptions();
      if (!IsConvertibleToDictionary(value)) {
        mUnion.DestroyOpenPopupOptions();
        tryNext = true;
        return true;
      }
      if (!memberSlot.Init(cx, value, "OpenPopupOptions branch of (DOMString or OpenPopupOptions)", passedToJSImpl)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToOpenPopupOptions(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToOpenPopupOptions(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline binding_detail::FakeString<char16_t>&
  RawSetAsString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eString;
    return mUnion.mValue.mString.SetValue();
  }

  inline binding_detail::FastOpenPopupOptions&
  RawSetAsOpenPopupOptions()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eOpenPopupOptions;
    return mUnion.mValue.mOpenPopupOptions.SetValue();
  }
};

class StringOrStringSequenceArgument
{
  StringOrStringSequence& mUnion;

  StringOrStringSequenceArgument(const StringOrStringSequenceArgument&) = delete;
  StringOrStringSequenceArgument& operator=(const StringOrStringSequenceArgument&) = delete;
public:
  explicit inline StringOrStringSequenceArgument(const StringOrStringSequence& aUnion)
    : mUnion(const_cast<StringOrStringSequence&>(aUnion))
  {
  }

  inline bool
  TrySetToString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsString().AssignLiteral(aData);
  }

  inline bool
  TrySetToStringSequence(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::AutoSequence<nsString>& memberSlot = RawSetAsStringSequence();
      JS::ForOfIterator iter(cx);
      if (!iter.init(value, JS::ForOfIterator::AllowNonIterable)) {
        return false;
      }
      if (!iter.valueIsIterable()) {
        mUnion.DestroyStringSequence();
        tryNext = true;
        return true;
      }
      binding_detail::AutoSequence<nsString> &arr = memberSlot;
      JS::Rooted<JS::Value> temp(cx);
      while (true) {
        bool done;
        if (!iter.next(&temp, &done)) {
          return false;
        }
        if (done) {
          break;
        }
        nsString* slotPtr = arr.AppendElement(mozilla::fallible);
        if (!slotPtr) {
          JS_ReportOutOfMemory(cx);
          return false;
        }
        nsString& slot = *slotPtr;
        if (!ConvertJSValueToString(cx, temp, eStringify, eStringify, slot)) {
          return false;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToStringSequence(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToStringSequence(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline binding_detail::FakeString<char16_t>&
  RawSetAsString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eString;
    return mUnion.mValue.mString.SetValue();
  }

  inline binding_detail::AutoSequence<nsString>&
  RawSetAsStringSequence()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eStringSequence;
    return mUnion.mValue.mStringSequence.SetValue();
  }
};

class StringOrStringSequenceOrConstrainDOMStringParametersArgument
{
  StringOrStringSequenceOrConstrainDOMStringParameters& mUnion;

  StringOrStringSequenceOrConstrainDOMStringParametersArgument(const StringOrStringSequenceOrConstrainDOMStringParametersArgument&) = delete;
  StringOrStringSequenceOrConstrainDOMStringParametersArgument& operator=(const StringOrStringSequenceOrConstrainDOMStringParametersArgument&) = delete;
public:
  explicit inline StringOrStringSequenceOrConstrainDOMStringParametersArgument(const StringOrStringSequenceOrConstrainDOMStringParameters& aUnion)
    : mUnion(const_cast<StringOrStringSequenceOrConstrainDOMStringParameters&>(aUnion))
  {
  }

  inline bool
  TrySetToString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsString().AssignLiteral(aData);
  }

  inline bool
  TrySetToStringSequence(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::AutoSequence<nsString>& memberSlot = RawSetAsStringSequence();
      JS::ForOfIterator iter(cx);
      if (!iter.init(value, JS::ForOfIterator::AllowNonIterable)) {
        return false;
      }
      if (!iter.valueIsIterable()) {
        mUnion.DestroyStringSequence();
        tryNext = true;
        return true;
      }
      binding_detail::AutoSequence<nsString> &arr = memberSlot;
      JS::Rooted<JS::Value> temp(cx);
      while (true) {
        bool done;
        if (!iter.next(&temp, &done)) {
          return false;
        }
        if (done) {
          break;
        }
        nsString* slotPtr = arr.AppendElement(mozilla::fallible);
        if (!slotPtr) {
          JS_ReportOutOfMemory(cx);
          return false;
        }
        nsString& slot = *slotPtr;
        if (!ConvertJSValueToString(cx, temp, eStringify, eStringify, slot)) {
          return false;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToStringSequence(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToStringSequence(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToConstrainDOMStringParameters(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FastConstrainDOMStringParameters& memberSlot = RawSetAsConstrainDOMStringParameters();
      if (!IsConvertibleToDictionary(value)) {
        mUnion.DestroyConstrainDOMStringParameters();
        tryNext = true;
        return true;
      }
      if (!memberSlot.Init(cx, value, "ConstrainDOMStringParameters branch of (DOMString or sequence<DOMString> or ConstrainDOMStringParameters)", passedToJSImpl)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToConstrainDOMStringParameters(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToConstrainDOMStringParameters(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline binding_detail::FakeString<char16_t>&
  RawSetAsString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eString;
    return mUnion.mValue.mString.SetValue();
  }

  inline binding_detail::AutoSequence<nsString>&
  RawSetAsStringSequence()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eStringSequence;
    return mUnion.mValue.mStringSequence.SetValue();
  }

  inline binding_detail::FastConstrainDOMStringParameters&
  RawSetAsConstrainDOMStringParameters()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eConstrainDOMStringParameters;
    return mUnion.mValue.mConstrainDOMStringParameters.SetValue();
  }
};

class StringOrWorkerOptionsArgument
{
  StringOrWorkerOptions& mUnion;

  StringOrWorkerOptionsArgument(const StringOrWorkerOptionsArgument&) = delete;
  StringOrWorkerOptionsArgument& operator=(const StringOrWorkerOptionsArgument&) = delete;
public:
  explicit inline StringOrWorkerOptionsArgument(const StringOrWorkerOptions& aUnion)
    : mUnion(const_cast<StringOrWorkerOptions&>(aUnion))
  {
  }

  inline bool
  TrySetToString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsString().AssignLiteral(aData);
  }

  inline bool
  TrySetToWorkerOptions(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FastWorkerOptions& memberSlot = RawSetAsWorkerOptions();
      if (!IsConvertibleToDictionary(value)) {
        mUnion.DestroyWorkerOptions();
        tryNext = true;
        return true;
      }
      if (!memberSlot.Init(cx, value, "WorkerOptions branch of (DOMString or WorkerOptions)", passedToJSImpl)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToWorkerOptions(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToWorkerOptions(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline binding_detail::FakeString<char16_t>&
  RawSetAsString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eString;
    return mUnion.mValue.mString.SetValue();
  }

  inline binding_detail::FastWorkerOptions&
  RawSetAsWorkerOptions()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eWorkerOptions;
    return mUnion.mValue.mWorkerOptions.SetValue();
  }
};

class SupportedTypeOrObjectArgument
{
  SupportedTypeOrObject& mUnion;

  SupportedTypeOrObjectArgument(const SupportedTypeOrObjectArgument&) = delete;
  SupportedTypeOrObjectArgument& operator=(const SupportedTypeOrObjectArgument&) = delete;
public:
  explicit inline SupportedTypeOrObjectArgument(const SupportedTypeOrObject& aUnion)
    : mUnion(const_cast<SupportedTypeOrObject&>(aUnion))
  {
  }

  inline bool
  TrySetToSupportedType(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      SupportedType& memberSlot = RawSetAsSupportedType();
      {
        int index;
        if (!FindEnumStringIndex<true>(cx, value, SupportedTypeValues::strings, "SupportedType", "SupportedType branch of (SupportedType or object)", &index)) {
          return false;
        }
        MOZ_ASSERT(index >= 0);
        memberSlot = static_cast<SupportedType>(index);
      }
    }
    return true;
  }

  inline bool
  TrySetToSupportedType(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToSupportedType(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  SetToObject(BindingCallContext& cx, JSObject* obj, bool passedToJSImpl = false)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mValue.mObject.SetValue(cx, obj);
    mUnion.mType = mUnion.eObject;
    if (passedToJSImpl && !CallerSubsumes(obj)) {
      cx.ThrowErrorMessage<MSG_PERMISSION_DENIED_TO_PASS_ARG>("object branch of (SupportedType or object)");
      return false;
    }
    return true;
  }

private:
  inline SupportedType&
  RawSetAsSupportedType()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eSupportedType;
    return mUnion.mValue.mSupportedType.SetValue();
  }
};

class TextOrElementOrDocumentArgument
{
  TextOrElementOrDocument& mUnion;

  TextOrElementOrDocumentArgument(const TextOrElementOrDocumentArgument&) = delete;
  TextOrElementOrDocumentArgument& operator=(const TextOrElementOrDocumentArgument&) = delete;
public:
  explicit inline TextOrElementOrDocumentArgument(const TextOrElementOrDocument& aUnion)
    : mUnion(const_cast<TextOrElementOrDocument&>(aUnion))
  {
  }

  inline bool
  TrySetToText(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::Text>& memberSlot = RawSetAsText();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::Text, mozilla::dom::Text>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyText();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToText(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToText(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToElement(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::Element>& memberSlot = RawSetAsElement();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::Element, mozilla::dom::Element>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyElement();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToElement(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToElement(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToDocument(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::Document>& memberSlot = RawSetAsDocument();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::Document, mozilla::dom::Document>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyDocument();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToDocument(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToDocument(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline NonNull<mozilla::dom::Text>&
  RawSetAsText()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eText;
    return mUnion.mValue.mText.SetValue();
  }

  inline NonNull<mozilla::dom::Element>&
  RawSetAsElement()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eElement;
    return mUnion.mValue.mElement.SetValue();
  }

  inline NonNull<mozilla::dom::Document>&
  RawSetAsDocument()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eDocument;
    return mUnion.mValue.mDocument.SetValue();
  }
};

class USVStringSequenceSequenceOrUSVStringUSVStringRecordOrUSVStringArgument
{
  USVStringSequenceSequenceOrUSVStringUSVStringRecordOrUSVString& mUnion;

  USVStringSequenceSequenceOrUSVStringUSVStringRecordOrUSVStringArgument(const USVStringSequenceSequenceOrUSVStringUSVStringRecordOrUSVStringArgument&) = delete;
  USVStringSequenceSequenceOrUSVStringUSVStringRecordOrUSVStringArgument& operator=(const USVStringSequenceSequenceOrUSVStringUSVStringRecordOrUSVStringArgument&) = delete;
public:
  explicit inline USVStringSequenceSequenceOrUSVStringUSVStringRecordOrUSVStringArgument(const USVStringSequenceSequenceOrUSVStringUSVStringRecordOrUSVString& aUnion)
    : mUnion(const_cast<USVStringSequenceSequenceOrUSVStringUSVStringRecordOrUSVString&>(aUnion))
  {
  }

  inline bool
  TrySetToUSVStringSequenceSequence(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::AutoSequence<Sequence<nsString>>& memberSlot = RawSetAsUSVStringSequenceSequence();
      JS::ForOfIterator iter(cx);
      if (!iter.init(value, JS::ForOfIterator::AllowNonIterable)) {
        return false;
      }
      if (!iter.valueIsIterable()) {
        mUnion.DestroyUSVStringSequenceSequence();
        tryNext = true;
        return true;
      }
      binding_detail::AutoSequence<Sequence<nsString>> &arr = memberSlot;
      JS::Rooted<JS::Value> temp(cx);
      while (true) {
        bool done;
        if (!iter.next(&temp, &done)) {
          return false;
        }
        if (done) {
          break;
        }
        Sequence<nsString>* slotPtr = arr.AppendElement(mozilla::fallible);
        if (!slotPtr) {
          JS_ReportOutOfMemory(cx);
          return false;
        }
        Sequence<nsString>& slot = *slotPtr;
        if (temp.isObject()) {
          JS::ForOfIterator iter1(cx);
          if (!iter1.init(temp, JS::ForOfIterator::AllowNonIterable)) {
            return false;
          }
          if (!iter1.valueIsIterable()) {
            cx.ThrowErrorMessage<MSG_NOT_SEQUENCE>("Element of sequence<sequence<USVString>> branch of (sequence<sequence<USVString>> or record<USVString, USVString> or USVString)");
            return false;
          }
          Sequence<nsString> &arr1 = slot;
          JS::Rooted<JS::Value> temp1(cx);
          while (true) {
            bool done1;
            if (!iter1.next(&temp1, &done1)) {
              return false;
            }
            if (done1) {
              break;
            }
            nsString* slotPtr1 = arr1.AppendElement(mozilla::fallible);
            if (!slotPtr1) {
              JS_ReportOutOfMemory(cx);
              return false;
            }
            nsString& slot1 = *slotPtr1;
            if (!ConvertJSValueToString(cx, temp1, eStringify, eStringify, slot1)) {
              return false;
            }
            if (!NormalizeUSVString(slot1)) {
              JS_ReportOutOfMemory(cx);
              return false;
            }
          }
        } else {
          cx.ThrowErrorMessage<MSG_NOT_SEQUENCE>("Element of sequence<sequence<USVString>> branch of (sequence<sequence<USVString>> or record<USVString, USVString> or USVString)");
          return false;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToUSVStringSequenceSequence(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToUSVStringSequenceSequence(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToUSVStringUSVStringRecord(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      Record<nsString, nsString>& memberSlot = RawSetAsUSVStringUSVStringRecord();
      auto& recordEntries = memberSlot.Entries();

      JS::Rooted<JSObject*> recordObj(cx, &value.toObject());
      JS::RootedVector<jsid> ids(cx);
      if (!js::GetPropertyKeys(cx, recordObj,
                               JSITER_OWNONLY | JSITER_HIDDEN | JSITER_SYMBOLS, &ids)) {
        return false;
      }
      if (!recordEntries.SetCapacity(ids.length(), mozilla::fallible)) {
        JS_ReportOutOfMemory(cx);
        return false;
      }
      JS::Rooted<JS::Value> propNameValue(cx);
      JS::Rooted<JS::Value> temp(cx);
      JS::Rooted<jsid> curId(cx);
      JS::Rooted<JS::Value> idVal(cx);
      // Use a hashset to keep track of ids seen, to avoid
      // introducing nasty O(N^2) behavior scanning for them all the
      // time.  Ideally we'd use a data structure with O(1) lookup
      // _and_ ordering for the MozMap, but we don't have one lying
      // around.
      nsTHashtable<nsStringHashKey> idsSeen;
      for (size_t i = 0; i < ids.length(); ++i) {
        curId = ids[i];

        JS::Rooted<JS::PropertyDescriptor> desc(cx);
        if (!JS_GetOwnPropertyDescriptorById(cx, recordObj, curId,
                                             &desc)) {
          return false;
        }

        if (!desc.object() /* == undefined in spec terms */ ||
            !desc.enumerable()) {
          continue;
        }

        idVal = js::IdToValue(curId);
        nsString propName;
        // This will just throw if idVal is a Symbol, like the spec says
        // to do.
        if (!ConvertJSValueToUSVString(cx, idVal, "key of record<USVString, USVString> branch of (sequence<sequence<USVString>> or record<USVString, USVString> or USVString)", propName)) {
          return false;
        }

        if (!JS_GetPropertyById(cx, recordObj, curId, &temp)) {
          return false;
        }

        Record<nsString, nsString>::EntryType* entry;
        if (!idsSeen.EnsureInserted(propName)) {
          // Find the existing entry.
          auto idx = recordEntries.IndexOf(propName);
          MOZ_ASSERT(idx != recordEntries.NoIndex,
                     "Why is it not found?");
          // Now blow it away to make it look like it was just added
          // to the array, because it's not obvious that it's
          // safe to write to its already-initialized mValue via our
          // normal codegen conversions.  For example, the value
          // could be a union and this would change its type, but
          // codegen assumes we won't do that.
          entry = recordEntries.ReconstructElementAt(idx);
        } else {
          // Safe to do an infallible append here, because we did a
          // SetCapacity above to the right capacity.
          entry = recordEntries.AppendElement();
        }
        entry->mKey = propName;
        nsString& slot = entry->mValue;
        if (!ConvertJSValueToString(cx, temp, eStringify, eStringify, slot)) {
          return false;
        }
        if (!NormalizeUSVString(slot)) {
          JS_ReportOutOfMemory(cx);
          return false;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToUSVStringUSVStringRecord(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToUSVStringUSVStringRecord(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToUSVString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsUSVString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
      if (!NormalizeUSVString(memberSlot)) {
        JS_ReportOutOfMemory(cx);
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsUSVString().AssignLiteral(aData);
  }

private:
  inline binding_detail::AutoSequence<Sequence<nsString>>&
  RawSetAsUSVStringSequenceSequence()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUSVStringSequenceSequence;
    return mUnion.mValue.mUSVStringSequenceSequence.SetValue();
  }

  inline Record<nsString, nsString>&
  RawSetAsUSVStringUSVStringRecord()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUSVStringUSVStringRecord;
    return mUnion.mValue.mUSVStringUSVStringRecord.SetValue();
  }

  inline binding_detail::FakeString<char16_t>&
  RawSetAsUSVString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUSVString;
    return mUnion.mValue.mUSVString.SetValue();
  }
};

class UTF8StringOrArrayBufferOrArrayBufferViewArgument
{
  UTF8StringOrArrayBufferOrArrayBufferView& mUnion;

  UTF8StringOrArrayBufferOrArrayBufferViewArgument(const UTF8StringOrArrayBufferOrArrayBufferViewArgument&) = delete;
  UTF8StringOrArrayBufferOrArrayBufferViewArgument& operator=(const UTF8StringOrArrayBufferOrArrayBufferViewArgument&) = delete;
public:
  explicit inline UTF8StringOrArrayBufferOrArrayBufferViewArgument(const UTF8StringOrArrayBufferOrArrayBufferView& aUnion)
    : mUnion(const_cast<UTF8StringOrArrayBufferOrArrayBufferView&>(aUnion))
  {
  }

  inline bool
  TrySetToUTF8String(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char>& memberSlot = RawSetAsUTF8String();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsCString::char_type (&aData)[N])
  {
    RawSetAsUTF8String().AssignLiteral(aData);
  }

  inline bool
  TrySetToArrayBuffer(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      RootedSpiderMonkeyInterface<ArrayBuffer>& memberSlot = RawSetAsArrayBuffer(cx);
      if (!memberSlot.Init(&value.toObject())) {
        mUnion.DestroyArrayBuffer();
        tryNext = true;
        return true;
      }
      if (JS::IsSharedArrayBufferObject(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_SHARED>("ArrayBuffer branch of (USVString or (ArrayBuffer or ArrayBufferView))");
        return false;
      }
      if (JS::IsLargeArrayBufferMaybeShared(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_LARGE>("ArrayBuffer branch of (USVString or (ArrayBuffer or ArrayBufferView))");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToArrayBuffer(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToArrayBuffer(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToArrayBufferView(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      RootedSpiderMonkeyInterface<ArrayBufferView>& memberSlot = RawSetAsArrayBufferView(cx);
      if (!memberSlot.Init(&value.toObject())) {
        mUnion.DestroyArrayBufferView();
        tryNext = true;
        return true;
      }
      if (JS::IsArrayBufferViewShared(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_SHARED>("ArrayBufferView branch of (USVString or (ArrayBuffer or ArrayBufferView))");
        return false;
      }
      if (JS::IsLargeArrayBufferView(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_LARGE>("ArrayBufferView branch of (USVString or (ArrayBuffer or ArrayBufferView))");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToArrayBufferView(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToArrayBufferView(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline binding_detail::FakeString<char>&
  RawSetAsUTF8String()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUTF8String;
    return mUnion.mValue.mUTF8String.SetValue();
  }

  inline RootedSpiderMonkeyInterface<ArrayBuffer>&
  RawSetAsArrayBuffer(JSContext* cx)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eArrayBuffer;
    return mUnion.mValue.mArrayBuffer.SetValue(cx);
  }

  inline RootedSpiderMonkeyInterface<ArrayBufferView>&
  RawSetAsArrayBufferView(JSContext* cx)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eArrayBufferView;
    return mUnion.mValue.mArrayBufferView.SetValue(cx);
  }
};

class UTF8StringOrCanvasGradientOrCanvasPatternArgument
{
  UTF8StringOrCanvasGradientOrCanvasPattern& mUnion;

  UTF8StringOrCanvasGradientOrCanvasPatternArgument(const UTF8StringOrCanvasGradientOrCanvasPatternArgument&) = delete;
  UTF8StringOrCanvasGradientOrCanvasPatternArgument& operator=(const UTF8StringOrCanvasGradientOrCanvasPatternArgument&) = delete;
public:
  explicit inline UTF8StringOrCanvasGradientOrCanvasPatternArgument(const UTF8StringOrCanvasGradientOrCanvasPattern& aUnion)
    : mUnion(const_cast<UTF8StringOrCanvasGradientOrCanvasPattern&>(aUnion))
  {
  }

  inline bool
  TrySetToUTF8String(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char>& memberSlot = RawSetAsUTF8String();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsCString::char_type (&aData)[N])
  {
    RawSetAsUTF8String().AssignLiteral(aData);
  }

  inline bool
  TrySetToCanvasGradient(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::CanvasGradient>& memberSlot = RawSetAsCanvasGradient();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::CanvasGradient, mozilla::dom::CanvasGradient>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyCanvasGradient();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToCanvasGradient(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToCanvasGradient(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToCanvasPattern(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::CanvasPattern>& memberSlot = RawSetAsCanvasPattern();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::CanvasPattern, mozilla::dom::CanvasPattern>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyCanvasPattern();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToCanvasPattern(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToCanvasPattern(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline binding_detail::FakeString<char>&
  RawSetAsUTF8String()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUTF8String;
    return mUnion.mValue.mUTF8String.SetValue();
  }

  inline NonNull<mozilla::dom::CanvasGradient>&
  RawSetAsCanvasGradient()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eCanvasGradient;
    return mUnion.mValue.mCanvasGradient.SetValue();
  }

  inline NonNull<mozilla::dom::CanvasPattern>&
  RawSetAsCanvasPattern()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eCanvasPattern;
    return mUnion.mValue.mCanvasPattern.SetValue();
  }
};

class UTF8StringOrDoubleArgument
{
  UTF8StringOrDouble& mUnion;

  UTF8StringOrDoubleArgument(const UTF8StringOrDoubleArgument&) = delete;
  UTF8StringOrDoubleArgument& operator=(const UTF8StringOrDoubleArgument&) = delete;
public:
  explicit inline UTF8StringOrDoubleArgument(const UTF8StringOrDouble& aUnion)
    : mUnion(const_cast<UTF8StringOrDouble&>(aUnion))
  {
  }

  inline bool
  TrySetToUTF8String(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char>& memberSlot = RawSetAsUTF8String();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsCString::char_type (&aData)[N])
  {
    RawSetAsUTF8String().AssignLiteral(aData);
  }

  inline bool
  TrySetToDouble(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      double& memberSlot = RawSetAsDouble();
      if (!ValueToPrimitive<double, eDefault>(cx, value, "Double branch of (USVString or double)", &memberSlot)) {
        return false;
      } else if (!mozilla::IsFinite(memberSlot)) {
        cx.ThrowErrorMessage<MSG_NOT_FINITE>("Double branch of (USVString or double)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToDouble(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToDouble(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline binding_detail::FakeString<char>&
  RawSetAsUTF8String()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUTF8String;
    return mUnion.mValue.mUTF8String.SetValue();
  }

  inline double&
  RawSetAsDouble()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eDouble;
    return mUnion.mValue.mDouble.SetValue();
  }
};

class UTF8StringOrL10nIdArgsArgument
{
  UTF8StringOrL10nIdArgs& mUnion;

  UTF8StringOrL10nIdArgsArgument(const UTF8StringOrL10nIdArgsArgument&) = delete;
  UTF8StringOrL10nIdArgsArgument& operator=(const UTF8StringOrL10nIdArgsArgument&) = delete;
public:
  explicit inline UTF8StringOrL10nIdArgsArgument(const UTF8StringOrL10nIdArgs& aUnion)
    : mUnion(const_cast<UTF8StringOrL10nIdArgs&>(aUnion))
  {
  }

  inline bool
  TrySetToUTF8String(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char>& memberSlot = RawSetAsUTF8String();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsCString::char_type (&aData)[N])
  {
    RawSetAsUTF8String().AssignLiteral(aData);
  }

  inline bool
  TrySetToL10nIdArgs(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FastL10nIdArgs& memberSlot = RawSetAsL10nIdArgs();
      if (!IsConvertibleToDictionary(value)) {
        mUnion.DestroyL10nIdArgs();
        tryNext = true;
        return true;
      }
      if (!memberSlot.Init(cx, value, "L10nIdArgs branch of (USVString or L10nIdArgs)", passedToJSImpl)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToL10nIdArgs(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToL10nIdArgs(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline binding_detail::FakeString<char>&
  RawSetAsUTF8String()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUTF8String;
    return mUnion.mValue.mUTF8String.SetValue();
  }

  inline binding_detail::FastL10nIdArgs&
  RawSetAsL10nIdArgs()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eL10nIdArgs;
    return mUnion.mValue.mL10nIdArgs.SetValue();
  }
};

class UTF8StringOrLongArgument
{
  UTF8StringOrLong& mUnion;

  UTF8StringOrLongArgument(const UTF8StringOrLongArgument&) = delete;
  UTF8StringOrLongArgument& operator=(const UTF8StringOrLongArgument&) = delete;
public:
  explicit inline UTF8StringOrLongArgument(const UTF8StringOrLong& aUnion)
    : mUnion(const_cast<UTF8StringOrLong&>(aUnion))
  {
  }

  inline bool
  TrySetToUTF8String(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char>& memberSlot = RawSetAsUTF8String();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsCString::char_type (&aData)[N])
  {
    RawSetAsUTF8String().AssignLiteral(aData);
  }

  inline bool
  TrySetToLong(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      int32_t& memberSlot = RawSetAsLong();
      if (!ValueToPrimitive<int32_t, eDefault>(cx, value, "Long branch of (USVString or long)", &memberSlot)) {
        return false;
      }
    }
    return true;
  }

private:
  inline binding_detail::FakeString<char>&
  RawSetAsUTF8String()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUTF8String;
    return mUnion.mValue.mUTF8String.SetValue();
  }

  inline int32_t&
  RawSetAsLong()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eLong;
    return mUnion.mValue.mLong.SetValue();
  }
};

class UTF8StringOrUTF8StringSequenceArgument
{
  UTF8StringOrUTF8StringSequence& mUnion;

  UTF8StringOrUTF8StringSequenceArgument(const UTF8StringOrUTF8StringSequenceArgument&) = delete;
  UTF8StringOrUTF8StringSequenceArgument& operator=(const UTF8StringOrUTF8StringSequenceArgument&) = delete;
public:
  explicit inline UTF8StringOrUTF8StringSequenceArgument(const UTF8StringOrUTF8StringSequence& aUnion)
    : mUnion(const_cast<UTF8StringOrUTF8StringSequence&>(aUnion))
  {
  }

  inline bool
  TrySetToUTF8String(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char>& memberSlot = RawSetAsUTF8String();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsCString::char_type (&aData)[N])
  {
    RawSetAsUTF8String().AssignLiteral(aData);
  }

  inline bool
  TrySetToUTF8StringSequence(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::AutoSequence<nsCString>& memberSlot = RawSetAsUTF8StringSequence();
      JS::ForOfIterator iter(cx);
      if (!iter.init(value, JS::ForOfIterator::AllowNonIterable)) {
        return false;
      }
      if (!iter.valueIsIterable()) {
        mUnion.DestroyUTF8StringSequence();
        tryNext = true;
        return true;
      }
      binding_detail::AutoSequence<nsCString> &arr = memberSlot;
      JS::Rooted<JS::Value> temp(cx);
      while (true) {
        bool done;
        if (!iter.next(&temp, &done)) {
          return false;
        }
        if (done) {
          break;
        }
        nsCString* slotPtr = arr.AppendElement(mozilla::fallible);
        if (!slotPtr) {
          JS_ReportOutOfMemory(cx);
          return false;
        }
        nsCString& slot = *slotPtr;
        if (!ConvertJSValueToString(cx, temp, eStringify, eStringify, slot)) {
          return false;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToUTF8StringSequence(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToUTF8StringSequence(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline binding_detail::FakeString<char>&
  RawSetAsUTF8String()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUTF8String;
    return mUnion.mValue.mUTF8String.SetValue();
  }

  inline binding_detail::AutoSequence<nsCString>&
  RawSetAsUTF8StringSequence()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUTF8StringSequence;
    return mUnion.mValue.mUTF8StringSequence.SetValue();
  }
};

class UTF8StringOrUnrestrictedDoubleSequenceOrDOMMatrixReadOnlyArgument
{
  UTF8StringOrUnrestrictedDoubleSequenceOrDOMMatrixReadOnly& mUnion;

  UTF8StringOrUnrestrictedDoubleSequenceOrDOMMatrixReadOnlyArgument(const UTF8StringOrUnrestrictedDoubleSequenceOrDOMMatrixReadOnlyArgument&) = delete;
  UTF8StringOrUnrestrictedDoubleSequenceOrDOMMatrixReadOnlyArgument& operator=(const UTF8StringOrUnrestrictedDoubleSequenceOrDOMMatrixReadOnlyArgument&) = delete;
public:
  explicit inline UTF8StringOrUnrestrictedDoubleSequenceOrDOMMatrixReadOnlyArgument(const UTF8StringOrUnrestrictedDoubleSequenceOrDOMMatrixReadOnly& aUnion)
    : mUnion(const_cast<UTF8StringOrUnrestrictedDoubleSequenceOrDOMMatrixReadOnly&>(aUnion))
  {
  }

  inline bool
  TrySetToUTF8String(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char>& memberSlot = RawSetAsUTF8String();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsCString::char_type (&aData)[N])
  {
    RawSetAsUTF8String().AssignLiteral(aData);
  }

  inline bool
  TrySetToUnrestrictedDoubleSequence(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::AutoSequence<double>& memberSlot = RawSetAsUnrestrictedDoubleSequence();
      JS::ForOfIterator iter(cx);
      if (!iter.init(value, JS::ForOfIterator::AllowNonIterable)) {
        return false;
      }
      if (!iter.valueIsIterable()) {
        mUnion.DestroyUnrestrictedDoubleSequence();
        tryNext = true;
        return true;
      }
      binding_detail::AutoSequence<double> &arr = memberSlot;
      JS::Rooted<JS::Value> temp(cx);
      while (true) {
        bool done;
        if (!iter.next(&temp, &done)) {
          return false;
        }
        if (done) {
          break;
        }
        double* slotPtr = arr.AppendElement(mozilla::fallible);
        if (!slotPtr) {
          JS_ReportOutOfMemory(cx);
          return false;
        }
        double& slot = *slotPtr;
        if (!ValueToPrimitive<double, eDefault>(cx, temp, "Element of sequence<unrestricted double> branch of (USVString or sequence<unrestricted double> or DOMMatrixReadOnly)", &slot)) {
          return false;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToUnrestrictedDoubleSequence(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToUnrestrictedDoubleSequence(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToDOMMatrixReadOnly(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::DOMMatrixReadOnly>& memberSlot = RawSetAsDOMMatrixReadOnly();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::DOMMatrixReadOnly, mozilla::dom::DOMMatrixReadOnly>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyDOMMatrixReadOnly();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToDOMMatrixReadOnly(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToDOMMatrixReadOnly(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline binding_detail::FakeString<char>&
  RawSetAsUTF8String()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUTF8String;
    return mUnion.mValue.mUTF8String.SetValue();
  }

  inline binding_detail::AutoSequence<double>&
  RawSetAsUnrestrictedDoubleSequence()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUnrestrictedDoubleSequence;
    return mUnion.mValue.mUnrestrictedDoubleSequence.SetValue();
  }

  inline NonNull<mozilla::dom::DOMMatrixReadOnly>&
  RawSetAsDOMMatrixReadOnly()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eDOMMatrixReadOnly;
    return mUnion.mValue.mDOMMatrixReadOnly.SetValue();
  }
};

class Uint32ArrayOrStringArgument
{
  Uint32ArrayOrString& mUnion;

  Uint32ArrayOrStringArgument(const Uint32ArrayOrStringArgument&) = delete;
  Uint32ArrayOrStringArgument& operator=(const Uint32ArrayOrStringArgument&) = delete;
public:
  explicit inline Uint32ArrayOrStringArgument(const Uint32ArrayOrString& aUnion)
    : mUnion(const_cast<Uint32ArrayOrString&>(aUnion))
  {
  }

  inline bool
  TrySetToUint32Array(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      RootedSpiderMonkeyInterface<Uint32Array>& memberSlot = RawSetAsUint32Array(cx);
      if (!memberSlot.Init(&value.toObject())) {
        mUnion.DestroyUint32Array();
        tryNext = true;
        return true;
      }
      if (JS::IsArrayBufferViewShared(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_SHARED>("Uint32Array branch of (Uint32Array or DOMString)");
        return false;
      }
      if (JS::IsLargeArrayBufferView(memberSlot.Obj())) {
        cx.ThrowErrorMessage<MSG_TYPEDARRAY_IS_LARGE>("Uint32Array branch of (Uint32Array or DOMString)");
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToUint32Array(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToUint32Array(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsString().AssignLiteral(aData);
  }

private:
  inline RootedSpiderMonkeyInterface<Uint32Array>&
  RawSetAsUint32Array(JSContext* cx)
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUint32Array;
    return mUnion.mValue.mUint32Array.SetValue(cx);
  }

  inline binding_detail::FakeString<char16_t>&
  RawSetAsString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eString;
    return mUnion.mValue.mString.SetValue();
  }
};

class UnrestrictedDoubleOrKeyframeAnimationOptionsArgument
{
  UnrestrictedDoubleOrKeyframeAnimationOptions& mUnion;

  UnrestrictedDoubleOrKeyframeAnimationOptionsArgument(const UnrestrictedDoubleOrKeyframeAnimationOptionsArgument&) = delete;
  UnrestrictedDoubleOrKeyframeAnimationOptionsArgument& operator=(const UnrestrictedDoubleOrKeyframeAnimationOptionsArgument&) = delete;
public:
  explicit inline UnrestrictedDoubleOrKeyframeAnimationOptionsArgument(const UnrestrictedDoubleOrKeyframeAnimationOptions& aUnion)
    : mUnion(const_cast<UnrestrictedDoubleOrKeyframeAnimationOptions&>(aUnion))
  {
  }

  inline bool
  TrySetToUnrestrictedDouble(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      double& memberSlot = RawSetAsUnrestrictedDouble();
      if (!ValueToPrimitive<double, eDefault>(cx, value, "Unrestricted double branch of (unrestricted double or KeyframeAnimationOptions)", &memberSlot)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToKeyframeAnimationOptions(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FastKeyframeAnimationOptions& memberSlot = RawSetAsKeyframeAnimationOptions();
      if (!IsConvertibleToDictionary(value)) {
        mUnion.DestroyKeyframeAnimationOptions();
        tryNext = true;
        return true;
      }
      if (!memberSlot.Init(cx, value, "KeyframeAnimationOptions branch of (unrestricted double or KeyframeAnimationOptions)", passedToJSImpl)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToKeyframeAnimationOptions(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToKeyframeAnimationOptions(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline double&
  RawSetAsUnrestrictedDouble()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUnrestrictedDouble;
    return mUnion.mValue.mUnrestrictedDouble.SetValue();
  }

  inline binding_detail::FastKeyframeAnimationOptions&
  RawSetAsKeyframeAnimationOptions()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eKeyframeAnimationOptions;
    return mUnion.mValue.mKeyframeAnimationOptions.SetValue();
  }
};

class UnrestrictedDoubleOrKeyframeEffectOptionsArgument
{
  UnrestrictedDoubleOrKeyframeEffectOptions& mUnion;

  UnrestrictedDoubleOrKeyframeEffectOptionsArgument(const UnrestrictedDoubleOrKeyframeEffectOptionsArgument&) = delete;
  UnrestrictedDoubleOrKeyframeEffectOptionsArgument& operator=(const UnrestrictedDoubleOrKeyframeEffectOptionsArgument&) = delete;
public:
  explicit inline UnrestrictedDoubleOrKeyframeEffectOptionsArgument(const UnrestrictedDoubleOrKeyframeEffectOptions& aUnion)
    : mUnion(const_cast<UnrestrictedDoubleOrKeyframeEffectOptions&>(aUnion))
  {
  }

  inline bool
  TrySetToUnrestrictedDouble(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      double& memberSlot = RawSetAsUnrestrictedDouble();
      if (!ValueToPrimitive<double, eDefault>(cx, value, "Unrestricted double branch of (unrestricted double or KeyframeEffectOptions)", &memberSlot)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToKeyframeEffectOptions(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FastKeyframeEffectOptions& memberSlot = RawSetAsKeyframeEffectOptions();
      if (!IsConvertibleToDictionary(value)) {
        mUnion.DestroyKeyframeEffectOptions();
        tryNext = true;
        return true;
      }
      if (!memberSlot.Init(cx, value, "KeyframeEffectOptions branch of (unrestricted double or KeyframeEffectOptions)", passedToJSImpl)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToKeyframeEffectOptions(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToKeyframeEffectOptions(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline double&
  RawSetAsUnrestrictedDouble()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUnrestrictedDouble;
    return mUnion.mValue.mUnrestrictedDouble.SetValue();
  }

  inline binding_detail::FastKeyframeEffectOptions&
  RawSetAsKeyframeEffectOptions()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eKeyframeEffectOptions;
    return mUnion.mValue.mKeyframeEffectOptions.SetValue();
  }
};

class UnrestrictedDoubleOrStringArgument
{
  UnrestrictedDoubleOrString& mUnion;

  UnrestrictedDoubleOrStringArgument(const UnrestrictedDoubleOrStringArgument&) = delete;
  UnrestrictedDoubleOrStringArgument& operator=(const UnrestrictedDoubleOrStringArgument&) = delete;
public:
  explicit inline UnrestrictedDoubleOrStringArgument(const UnrestrictedDoubleOrString& aUnion)
    : mUnion(const_cast<UnrestrictedDoubleOrString&>(aUnion))
  {
  }

  inline bool
  TrySetToUnrestrictedDouble(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      double& memberSlot = RawSetAsUnrestrictedDouble();
      if (!ValueToPrimitive<double, eDefault>(cx, value, "Unrestricted double branch of (unrestricted double or DOMString)", &memberSlot)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsString().AssignLiteral(aData);
  }

private:
  inline double&
  RawSetAsUnrestrictedDouble()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUnrestrictedDouble;
    return mUnion.mValue.mUnrestrictedDouble.SetValue();
  }

  inline binding_detail::FakeString<char16_t>&
  RawSetAsString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eString;
    return mUnion.mValue.mString.SetValue();
  }
};

class UnrestrictedFloatOrStringArgument
{
  UnrestrictedFloatOrString& mUnion;

  UnrestrictedFloatOrStringArgument(const UnrestrictedFloatOrStringArgument&) = delete;
  UnrestrictedFloatOrStringArgument& operator=(const UnrestrictedFloatOrStringArgument&) = delete;
public:
  explicit inline UnrestrictedFloatOrStringArgument(const UnrestrictedFloatOrString& aUnion)
    : mUnion(const_cast<UnrestrictedFloatOrString&>(aUnion))
  {
  }

  inline bool
  TrySetToUnrestrictedFloat(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      float& memberSlot = RawSetAsUnrestrictedFloat();
      if (!ValueToPrimitive<float, eDefault>(cx, value, "Unrestricted float branch of (unrestricted float or DOMString)", &memberSlot)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsString().AssignLiteral(aData);
  }

private:
  inline float&
  RawSetAsUnrestrictedFloat()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUnrestrictedFloat;
    return mUnion.mValue.mUnrestrictedFloat.SetValue();
  }

  inline binding_detail::FakeString<char16_t>&
  RawSetAsString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eString;
    return mUnion.mValue.mString.SetValue();
  }
};

class UnsignedLongLongOrStringArgument
{
  UnsignedLongLongOrString& mUnion;

  UnsignedLongLongOrStringArgument(const UnsignedLongLongOrStringArgument&) = delete;
  UnsignedLongLongOrStringArgument& operator=(const UnsignedLongLongOrStringArgument&) = delete;
public:
  explicit inline UnsignedLongLongOrStringArgument(const UnsignedLongLongOrString& aUnion)
    : mUnion(const_cast<UnsignedLongLongOrString&>(aUnion))
  {
  }

  inline bool
  TrySetToUnsignedLongLong(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      uint64_t& memberSlot = RawSetAsUnsignedLongLong();
      if (!ValueToPrimitive<uint64_t, eDefault>(cx, value, "Unsigned long long branch of (unsigned long long or DOMString)", &memberSlot)) {
        return false;
      }
    }
    return true;
  }

  inline bool
  TrySetToString(JSContext* cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      binding_detail::FakeString<char16_t>& memberSlot = RawSetAsString();
      if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
        return false;
      }
    }
    return true;
  }

  template <int N>
  inline void
  SetStringLiteral(const nsString::char_type (&aData)[N])
  {
    RawSetAsString().AssignLiteral(aData);
  }

private:
  inline uint64_t&
  RawSetAsUnsignedLongLong()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eUnsignedLongLong;
    return mUnion.mValue.mUnsignedLongLong.SetValue();
  }

  inline binding_detail::FakeString<char16_t>&
  RawSetAsString()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eString;
    return mUnion.mValue.mString.SetValue();
  }
};

class VideoTrackOrAudioTrackOrTextTrackArgument
{
  VideoTrackOrAudioTrackOrTextTrack& mUnion;

  VideoTrackOrAudioTrackOrTextTrackArgument(const VideoTrackOrAudioTrackOrTextTrackArgument&) = delete;
  VideoTrackOrAudioTrackOrTextTrackArgument& operator=(const VideoTrackOrAudioTrackOrTextTrackArgument&) = delete;
public:
  explicit inline VideoTrackOrAudioTrackOrTextTrackArgument(const VideoTrackOrAudioTrackOrTextTrack& aUnion)
    : mUnion(const_cast<VideoTrackOrAudioTrackOrTextTrack&>(aUnion))
  {
  }

  inline bool
  TrySetToVideoTrack(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::VideoTrack>& memberSlot = RawSetAsVideoTrack();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::VideoTrack, mozilla::dom::VideoTrack>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyVideoTrack();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToVideoTrack(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToVideoTrack(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToAudioTrack(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::AudioTrack>& memberSlot = RawSetAsAudioTrack();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::AudioTrack, mozilla::dom::AudioTrack>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyAudioTrack();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToAudioTrack(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToAudioTrack(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToTextTrack(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::TextTrack>& memberSlot = RawSetAsTextTrack();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::TextTrack, mozilla::dom::TextTrack>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyTextTrack();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToTextTrack(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToTextTrack(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline NonNull<mozilla::dom::VideoTrack>&
  RawSetAsVideoTrack()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eVideoTrack;
    return mUnion.mValue.mVideoTrack.SetValue();
  }

  inline NonNull<mozilla::dom::AudioTrack>&
  RawSetAsAudioTrack()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eAudioTrack;
    return mUnion.mValue.mAudioTrack.SetValue();
  }

  inline NonNull<mozilla::dom::TextTrack>&
  RawSetAsTextTrack()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eTextTrack;
    return mUnion.mValue.mTextTrack.SetValue();
  }
};

class WebGLRenderingContextOrWebGL2RenderingContextArgument
{
  WebGLRenderingContextOrWebGL2RenderingContext& mUnion;

  WebGLRenderingContextOrWebGL2RenderingContextArgument(const WebGLRenderingContextOrWebGL2RenderingContextArgument&) = delete;
  WebGLRenderingContextOrWebGL2RenderingContextArgument& operator=(const WebGLRenderingContextOrWebGL2RenderingContextArgument&) = delete;
public:
  explicit inline WebGLRenderingContextOrWebGL2RenderingContextArgument(const WebGLRenderingContextOrWebGL2RenderingContext& aUnion)
    : mUnion(const_cast<WebGLRenderingContextOrWebGL2RenderingContext&>(aUnion))
  {
  }

  inline bool
  TrySetToWebGLRenderingContext(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::ClientWebGLContext>& memberSlot = RawSetAsWebGLRenderingContext();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::WebGLRenderingContext, mozilla::ClientWebGLContext>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyWebGLRenderingContext();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToWebGLRenderingContext(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToWebGLRenderingContext(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToWebGL2RenderingContext(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::ClientWebGLContext>& memberSlot = RawSetAsWebGL2RenderingContext();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::WebGL2RenderingContext, mozilla::ClientWebGLContext>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyWebGL2RenderingContext();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToWebGL2RenderingContext(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToWebGL2RenderingContext(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline NonNull<mozilla::ClientWebGLContext>&
  RawSetAsWebGLRenderingContext()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eWebGLRenderingContext;
    return mUnion.mValue.mWebGLRenderingContext.SetValue();
  }

  inline NonNull<mozilla::ClientWebGLContext>&
  RawSetAsWebGL2RenderingContext()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eWebGL2RenderingContext;
    return mUnion.mValue.mWebGL2RenderingContext.SetValue();
  }
};

class WindowProxyOrMessagePortOrServiceWorkerArgument
{
  WindowProxyOrMessagePortOrServiceWorker& mUnion;

  WindowProxyOrMessagePortOrServiceWorkerArgument(const WindowProxyOrMessagePortOrServiceWorkerArgument&) = delete;
  WindowProxyOrMessagePortOrServiceWorkerArgument& operator=(const WindowProxyOrMessagePortOrServiceWorkerArgument&) = delete;
public:
  explicit inline WindowProxyOrMessagePortOrServiceWorkerArgument(const WindowProxyOrMessagePortOrServiceWorker& aUnion)
    : mUnion(const_cast<WindowProxyOrMessagePortOrServiceWorker&>(aUnion))
  {
  }

  inline bool
  TrySetToWindowProxy(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      mozilla::dom::WindowProxyHolder& memberSlot = RawSetAsWindowProxy();
      JS::Rooted<JSObject*> source(cx, &value.toObject());
      if (NS_FAILED(UnwrapWindowProxyArg(cx, source, memberSlot))) {
          mUnion.DestroyWindowProxy();
          tryNext = true;
          return true;
      }
    }
    return true;
  }

  inline bool
  TrySetToWindowProxy(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToWindowProxy(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToMessagePort(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::MessagePort>& memberSlot = RawSetAsMessagePort();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::MessagePort, mozilla::dom::MessagePort>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyMessagePort();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToMessagePort(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToMessagePort(cx, value, tryNext, passedToJSImpl);
  }

  inline bool
  TrySetToServiceWorker(BindingCallContext& cx, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    tryNext = false;
    { // scope for memberSlot
      NonNull<mozilla::dom::ServiceWorker>& memberSlot = RawSetAsServiceWorker();
      {
        // Our JSContext should be in the right global to do unwrapping in.
        nsresult rv = UnwrapObject<prototypes::id::ServiceWorker, mozilla::dom::ServiceWorker>(value, memberSlot, cx);
        if (NS_FAILED(rv)) {
          mUnion.DestroyServiceWorker();
          tryNext = true;
          return true;
        }
      }
    }
    return true;
  }

  inline bool
  TrySetToServiceWorker(JSContext* cx_, JS::MutableHandle<JS::Value> value, bool& tryNext, bool passedToJSImpl = false)
  {
    BindingCallContext cx(cx_, nullptr);
    return TrySetToServiceWorker(cx, value, tryNext, passedToJSImpl);
  }

private:
  inline mozilla::dom::WindowProxyHolder&
  RawSetAsWindowProxy()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eWindowProxy;
    return mUnion.mValue.mWindowProxy.SetValue();
  }

  inline NonNull<mozilla::dom::MessagePort>&
  RawSetAsMessagePort()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eMessagePort;
    return mUnion.mValue.mMessagePort.SetValue();
  }

  inline NonNull<mozilla::dom::ServiceWorker>&
  RawSetAsServiceWorker()
  {
    MOZ_ASSERT(mUnion.mType == mUnion.eUninitialized);
    mUnion.mType = mUnion.eServiceWorker;
    return mUnion.mValue.mServiceWorker.SetValue();
  }
};


} // namespace dom
} // namespace mozilla


#endif // mozilla_dom_UnionConversions_h
