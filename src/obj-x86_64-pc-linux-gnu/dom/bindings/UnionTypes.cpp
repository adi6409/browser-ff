#include "js/ForOfIterator.h"
#include "mozilla/FloatingPoint.h"
#include "mozilla/dom/BindingCallContext.h"
#include "mozilla/dom/CanvasGradient.h"
#include "mozilla/dom/CanvasPattern.h"
#include "mozilla/dom/Directory.h"
#include "mozilla/dom/File.h"
#include "mozilla/dom/HTMLOptGroupElement.h"
#include "mozilla/dom/HTMLOptionElement.h"
#include "mozilla/dom/PrimitiveConversions.h"
#include "mozilla/dom/UnionTypes.h"
#include "nsGenericHTMLElement.h"
#include "nsINode.h"

namespace mozilla {
namespace dom {
void
ImplCycleCollectionTraverse(nsCycleCollectionTraversalCallback& aCallback, OwningCanvasPatternOrCanvasGradient& aUnion, const char* aName, uint32_t aFlags)
{
  if (aUnion.IsCanvasPattern()) {
    ImplCycleCollectionTraverse(aCallback, aUnion.GetAsCanvasPattern(), "mCanvasPattern", aFlags);
  } else if (aUnion.IsCanvasGradient()) {
    ImplCycleCollectionTraverse(aCallback, aUnion.GetAsCanvasGradient(), "mCanvasGradient", aFlags);
  }
}

void
ImplCycleCollectionTraverse(nsCycleCollectionTraversalCallback& aCallback, OwningCanvasPatternOrNullOrCanvasGradient& aUnion, const char* aName, uint32_t aFlags)
{
  if (aUnion.IsCanvasPattern()) {
    ImplCycleCollectionTraverse(aCallback, aUnion.GetAsCanvasPattern(), "mCanvasPattern", aFlags);
  } else if (aUnion.IsCanvasGradient()) {
    ImplCycleCollectionTraverse(aCallback, aUnion.GetAsCanvasGradient(), "mCanvasGradient", aFlags);
  }
}

void
ImplCycleCollectionTraverse(nsCycleCollectionTraversalCallback& aCallback, OwningFileOrDirectory& aUnion, const char* aName, uint32_t aFlags)
{
  if (aUnion.IsFile()) {
    ImplCycleCollectionTraverse(aCallback, aUnion.GetAsFile(), "mFile", aFlags);
  } else if (aUnion.IsDirectory()) {
    ImplCycleCollectionTraverse(aCallback, aUnion.GetAsDirectory(), "mDirectory", aFlags);
  }
}

void
ImplCycleCollectionTraverse(nsCycleCollectionTraversalCallback& aCallback, OwningHTMLElementOrLong& aUnion, const char* aName, uint32_t aFlags)
{
  if (aUnion.IsHTMLElement()) {
    ImplCycleCollectionTraverse(aCallback, aUnion.GetAsHTMLElement(), "mHTMLElement", aFlags);
  }
}

void
ImplCycleCollectionTraverse(nsCycleCollectionTraversalCallback& aCallback, OwningHTMLOptionElementOrHTMLOptGroupElement& aUnion, const char* aName, uint32_t aFlags)
{
  if (aUnion.IsHTMLOptionElement()) {
    ImplCycleCollectionTraverse(aCallback, aUnion.GetAsHTMLOptionElement(), "mHTMLOptionElement", aFlags);
  } else if (aUnion.IsHTMLOptGroupElement()) {
    ImplCycleCollectionTraverse(aCallback, aUnion.GetAsHTMLOptGroupElement(), "mHTMLOptGroupElement", aFlags);
  }
}

void
ImplCycleCollectionTraverse(nsCycleCollectionTraversalCallback& aCallback, OwningNodeOrString& aUnion, const char* aName, uint32_t aFlags)
{
  if (aUnion.IsNode()) {
    ImplCycleCollectionTraverse(aCallback, aUnion.GetAsNode(), "mNode", aFlags);
  }
}

void
ImplCycleCollectionUnlink(OwningCanvasPatternOrCanvasGradient& aUnion)
{
  aUnion.Uninit();
}

void
ImplCycleCollectionUnlink(OwningCanvasPatternOrNullOrCanvasGradient& aUnion)
{
  aUnion.Uninit();
}

void
ImplCycleCollectionUnlink(OwningFileOrDirectory& aUnion)
{
  aUnion.Uninit();
}

void
ImplCycleCollectionUnlink(OwningHTMLElementOrLong& aUnion)
{
  aUnion.Uninit();
}

void
ImplCycleCollectionUnlink(OwningHTMLOptionElementOrHTMLOptGroupElement& aUnion)
{
  aUnion.Uninit();
}

void
ImplCycleCollectionUnlink(OwningNodeOrString& aUnion)
{
  aUnion.Uninit();
}

bool
ArrayBufferOrLong::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eArrayBuffer: {
      rval.setObject(*mValue.mArrayBuffer.Value().Obj());
      if (!MaybeWrapNonDOMObjectValue(cx, rval)) {
        return false;
      }
      return true;
      break;
    }
    case eLong: {
      rval.setInt32(int32_t(mValue.mLong.Value()));
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

bool
ArrayBufferViewOrArrayBuffer::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eArrayBufferView: {
      rval.setObject(*mValue.mArrayBufferView.Value().Obj());
      if (!MaybeWrapNonDOMObjectValue(cx, rval)) {
        return false;
      }
      return true;
      break;
    }
    case eArrayBuffer: {
      rval.setObject(*mValue.mArrayBuffer.Value().Obj());
      if (!MaybeWrapNonDOMObjectValue(cx, rval)) {
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

bool
ByteStringOrLong::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eByteString: {
      if (!NonVoidByteStringToJsval(cx, mValue.mByteString.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    case eLong: {
      rval.setInt32(int32_t(mValue.mLong.Value()));
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

bool
CanvasPatternOrCanvasGradient::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eCanvasPattern: {
      if (!GetOrCreateDOMReflector(cx, mValue.mCanvasPattern.Value(), rval)) {
        MOZ_ASSERT(JS_IsExceptionPending(cx));
        return false;
      }
      return true;
      break;
    }
    case eCanvasGradient: {
      if (!GetOrCreateDOMReflector(cx, mValue.mCanvasGradient.Value(), rval)) {
        MOZ_ASSERT(JS_IsExceptionPending(cx));
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

bool
CanvasPatternOrNullOrCanvasGradient::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eNull: {
      rval.setNull();
      return true;
      break;
    }
    case eCanvasPattern: {
      if (!GetOrCreateDOMReflector(cx, mValue.mCanvasPattern.Value(), rval)) {
        MOZ_ASSERT(JS_IsExceptionPending(cx));
        return false;
      }
      return true;
      break;
    }
    case eCanvasGradient: {
      if (!GetOrCreateDOMReflector(cx, mValue.mCanvasGradient.Value(), rval)) {
        MOZ_ASSERT(JS_IsExceptionPending(cx));
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

bool
DoubleOrByteString::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eDouble: {
      rval.set(JS_NumberValue(double(mValue.mDouble.Value())));
      return true;
      break;
    }
    case eByteString: {
      if (!NonVoidByteStringToJsval(cx, mValue.mByteString.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

bool
DoubleOrString::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eDouble: {
      rval.set(JS_NumberValue(double(mValue.mDouble.Value())));
      return true;
      break;
    }
    case eString: {
      if (!xpc::NonVoidStringToJsval(cx, mValue.mString.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

bool
DoubleOrSupportedType::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eDouble: {
      rval.set(JS_NumberValue(double(mValue.mDouble.Value())));
      return true;
      break;
    }
    case eSupportedType: {
      if (!ToJSValue(cx, mValue.mSupportedType.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

bool
DoubleOrUSVString::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eDouble: {
      rval.set(JS_NumberValue(double(mValue.mDouble.Value())));
      return true;
      break;
    }
    case eUSVString: {
      if (!xpc::NonVoidStringToJsval(cx, mValue.mUSVString.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

bool
DoubleOrUTF8String::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eDouble: {
      rval.set(JS_NumberValue(double(mValue.mDouble.Value())));
      return true;
      break;
    }
    case eUTF8String: {
      if (!NonVoidUTF8StringToJsval(cx, mValue.mUTF8String.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

bool
FileOrDirectory::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eFile: {
      if (!GetOrCreateDOMReflector(cx, mValue.mFile.Value(), rval)) {
        MOZ_ASSERT(JS_IsExceptionPending(cx));
        return false;
      }
      return true;
      break;
    }
    case eDirectory: {
      if (!GetOrCreateDOMReflector(cx, mValue.mDirectory.Value(), rval)) {
        MOZ_ASSERT(JS_IsExceptionPending(cx));
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

bool
FloatOrString::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eFloat: {
      rval.set(JS_NumberValue(double(mValue.mFloat.Value())));
      return true;
      break;
    }
    case eString: {
      if (!xpc::NonVoidStringToJsval(cx, mValue.mString.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

bool
HTMLElementOrLong::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eHTMLElement: {
      if (!GetOrCreateDOMReflector(cx, mValue.mHTMLElement.Value(), rval)) {
        MOZ_ASSERT(JS_IsExceptionPending(cx));
        return false;
      }
      return true;
      break;
    }
    case eLong: {
      rval.setInt32(int32_t(mValue.mLong.Value()));
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

bool
HTMLOptionElementOrHTMLOptGroupElement::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eHTMLOptionElement: {
      if (!GetOrCreateDOMReflector(cx, mValue.mHTMLOptionElement.Value(), rval)) {
        MOZ_ASSERT(JS_IsExceptionPending(cx));
        return false;
      }
      return true;
      break;
    }
    case eHTMLOptGroupElement: {
      if (!GetOrCreateDOMReflector(cx, mValue.mHTMLOptGroupElement.Value(), rval)) {
        MOZ_ASSERT(JS_IsExceptionPending(cx));
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

bool
LongOrStringAnyRecord::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eLong: {
      rval.setInt32(int32_t(mValue.mLong.Value()));
      return true;
      break;
    }
    case eStringAnyRecord: {

      JS::Rooted<JSObject*> returnObj(cx, JS_NewPlainObject(cx));
      if (!returnObj) {
        return false;
      }
      // Scope for 'tmp'
      {
        JS::Rooted<JS::Value> tmp(cx);
        for (auto& entry : mValue.mStringAnyRecord.Value().Entries()) {
          auto& recordValue0 = entry.mValue;
          // Control block to let us common up the JS_DefineUCProperty calls when there
          // are different ways to succeed at wrapping the value.
          do {
            JS::ExposeValueToActiveJS(recordValue0);
            tmp.set(recordValue0);
            if (!MaybeWrapValue(cx, &tmp)) {
              return false;
            }
            break;
          } while (false);
          if (!JS_DefineUCProperty(cx, returnObj,
                                   entry.mKey.BeginReading(),
                                   entry.mKey.Length(), tmp,
                                   JSPROP_ENUMERATE)) {
            return false;
          }
        }
      }
      rval.setObject(*returnObj);
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

bool
NodeOrString::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eNode: {
      if (!GetOrCreateDOMReflector(cx, mValue.mNode.Value(), rval)) {
        MOZ_ASSERT(JS_IsExceptionPending(cx));
        return false;
      }
      return true;
      break;
    }
    case eString: {
      if (!xpc::NonVoidStringToJsval(cx, mValue.mString.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

bool
ObjectOrLong::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eObject: {
      JS::ExposeObjectToActiveJS(mValue.mObject.Value());
      rval.setObject(*mValue.mObject.Value());
      if (!MaybeWrapObjectValue(cx, rval)) {
        return false;
      }
      return true;
      break;
    }
    case eLong: {
      rval.setInt32(int32_t(mValue.mLong.Value()));
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

bool
ObjectOrNullOrLong::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eNull: {
      rval.setNull();
      return true;
      break;
    }
    case eObject: {
      JS::ExposeObjectToActiveJS(mValue.mObject.Value());
      rval.setObject(*mValue.mObject.Value());
      if (!MaybeWrapObjectValue(cx, rval)) {
        return false;
      }
      return true;
      break;
    }
    case eLong: {
      rval.setInt32(int32_t(mValue.mLong.Value()));
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

bool
StringOrArrayBuffer::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eString: {
      if (!xpc::NonVoidStringToJsval(cx, mValue.mString.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    case eArrayBuffer: {
      rval.setObject(*mValue.mArrayBuffer.Value().Obj());
      if (!MaybeWrapNonDOMObjectValue(cx, rval)) {
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

bool
StringOrMaybeSharedArrayBuffer::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eString: {
      if (!xpc::NonVoidStringToJsval(cx, mValue.mString.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    case eArrayBuffer: {
      rval.setObject(*mValue.mArrayBuffer.Value().Obj());
      if (!MaybeWrapNonDOMObjectValue(cx, rval)) {
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

bool
StringOrObject::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eString: {
      if (!xpc::NonVoidStringToJsval(cx, mValue.mString.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    case eObject: {
      JS::ExposeObjectToActiveJS(mValue.mObject.Value());
      rval.setObject(*mValue.mObject.Value());
      if (!MaybeWrapObjectValue(cx, rval)) {
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

bool
StringOrStringSequence::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eString: {
      if (!xpc::NonVoidStringToJsval(cx, mValue.mString.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    case eStringSequence: {

      uint32_t length = mValue.mStringSequence.Value().Length();
      JS::Rooted<JSObject*> returnArray(cx, JS::NewArrayObject(cx, length));
      if (!returnArray) {
        return false;
      }
      // Scope for 'tmp'
      {
        JS::Rooted<JS::Value> tmp(cx);
        for (uint32_t sequenceIdx0 = 0; sequenceIdx0 < length; ++sequenceIdx0) {
          // Control block to let us common up the JS_DefineElement calls when there
          // are different ways to succeed at wrapping the object.
          do {
            if (!xpc::NonVoidStringToJsval(cx, mValue.mStringSequence.Value()[sequenceIdx0], &tmp)) {
              return false;
            }
            break;
          } while (false);
          if (!JS_DefineElement(cx, returnArray, sequenceIdx0, tmp,
                                JSPROP_ENUMERATE)) {
            return false;
          }
        }
      }
      rval.setObject(*returnArray);
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

bool
SupportedTypeOrObject::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eSupportedType: {
      if (!ToJSValue(cx, mValue.mSupportedType.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    case eObject: {
      JS::ExposeObjectToActiveJS(mValue.mObject.Value());
      rval.setObject(*mValue.mObject.Value());
      if (!MaybeWrapObjectValue(cx, rval)) {
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

bool
UTF8StringOrLong::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eUTF8String: {
      if (!NonVoidUTF8StringToJsval(cx, mValue.mUTF8String.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    case eLong: {
      rval.setInt32(int32_t(mValue.mLong.Value()));
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

bool
UTF8StringOrUTF8StringSequence::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eUTF8String: {
      if (!NonVoidUTF8StringToJsval(cx, mValue.mUTF8String.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    case eUTF8StringSequence: {

      uint32_t length = mValue.mUTF8StringSequence.Value().Length();
      JS::Rooted<JSObject*> returnArray(cx, JS::NewArrayObject(cx, length));
      if (!returnArray) {
        return false;
      }
      // Scope for 'tmp'
      {
        JS::Rooted<JS::Value> tmp(cx);
        for (uint32_t sequenceIdx0 = 0; sequenceIdx0 < length; ++sequenceIdx0) {
          // Control block to let us common up the JS_DefineElement calls when there
          // are different ways to succeed at wrapping the object.
          do {
            if (!NonVoidUTF8StringToJsval(cx, mValue.mUTF8StringSequence.Value()[sequenceIdx0], &tmp)) {
              return false;
            }
            break;
          } while (false);
          if (!JS_DefineElement(cx, returnArray, sequenceIdx0, tmp,
                                JSPROP_ENUMERATE)) {
            return false;
          }
        }
      }
      rval.setObject(*returnArray);
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

bool
UnrestrictedDoubleOrString::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eUnrestrictedDouble: {
      rval.set(JS_NumberValue(double(mValue.mUnrestrictedDouble.Value())));
      return true;
      break;
    }
    case eString: {
      if (!xpc::NonVoidStringToJsval(cx, mValue.mString.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

bool
UnrestrictedFloatOrString::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eUnrestrictedFloat: {
      rval.set(JS_NumberValue(double(mValue.mUnrestrictedFloat.Value())));
      return true;
      break;
    }
    case eString: {
      if (!xpc::NonVoidStringToJsval(cx, mValue.mString.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

ArrayBuffer&
OwningArrayBufferOrLong::RawSetAsArrayBuffer()
{
  if (mType == eArrayBuffer) {
    return mValue.mArrayBuffer.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eArrayBuffer;
  return mValue.mArrayBuffer.SetValue();
}

ArrayBuffer&
OwningArrayBufferOrLong::SetAsArrayBuffer()
{
  if (mType == eArrayBuffer) {
    return mValue.mArrayBuffer.Value();
  }
  Uninit();
  mType = eArrayBuffer;
  return mValue.mArrayBuffer.SetValue();
}

bool
OwningArrayBufferOrLong::TrySetToArrayBuffer(BindingCallContext& cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  tryNext = false;
  { // scope for memberSlot
    ArrayBuffer& memberSlot = RawSetAsArrayBuffer();
    if (!memberSlot.Init(&value.toObject())) {
      DestroyArrayBuffer();
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

bool
OwningArrayBufferOrLong::TrySetToArrayBuffer(JSContext* cx_, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  BindingCallContext cx(cx_, nullptr);
  return TrySetToArrayBuffer(cx, value, tryNext, passedToJSImpl);
}

void
OwningArrayBufferOrLong::DestroyArrayBuffer()
{
  MOZ_ASSERT(IsArrayBuffer(), "Wrong type!");
  mValue.mArrayBuffer.Destroy();
  mType = eUninitialized;
}




int32_t&
OwningArrayBufferOrLong::RawSetAsLong()
{
  if (mType == eLong) {
    return mValue.mLong.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eLong;
  return mValue.mLong.SetValue();
}

int32_t&
OwningArrayBufferOrLong::SetAsLong()
{
  if (mType == eLong) {
    return mValue.mLong.Value();
  }
  Uninit();
  mType = eLong;
  return mValue.mLong.SetValue();
}

bool
OwningArrayBufferOrLong::TrySetToLong(JSContext* cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
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

void
OwningArrayBufferOrLong::DestroyLong()
{
  MOZ_ASSERT(IsLong(), "Wrong type!");
  mValue.mLong.Destroy();
  mType = eUninitialized;
}




void
OwningArrayBufferOrLong::Uninit()
{
  switch (mType) {
    case eUninitialized: {
      break;
    }
    case eArrayBuffer: {
      DestroyArrayBuffer();
      break;
    }
    case eLong: {
      DestroyLong();
      break;
    }
  }
}

bool
OwningArrayBufferOrLong::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eArrayBuffer: {
      rval.setObject(*mValue.mArrayBuffer.Value().Obj());
      if (!MaybeWrapNonDOMObjectValue(cx, rval)) {
        return false;
      }
      return true;
      break;
    }
    case eLong: {
      rval.setInt32(int32_t(mValue.mLong.Value()));
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

void
OwningArrayBufferOrLong::TraceUnion(JSTracer* trc)
{
  switch (mType) {
    case eArrayBuffer: {
      mValue.mArrayBuffer.Value().TraceSelf(trc);
      break;
    }
    default: {
      break;
    }
  }
}

ArrayBufferView&
OwningArrayBufferViewOrArrayBuffer::RawSetAsArrayBufferView()
{
  if (mType == eArrayBufferView) {
    return mValue.mArrayBufferView.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eArrayBufferView;
  return mValue.mArrayBufferView.SetValue();
}

ArrayBufferView&
OwningArrayBufferViewOrArrayBuffer::SetAsArrayBufferView()
{
  if (mType == eArrayBufferView) {
    return mValue.mArrayBufferView.Value();
  }
  Uninit();
  mType = eArrayBufferView;
  return mValue.mArrayBufferView.SetValue();
}

bool
OwningArrayBufferViewOrArrayBuffer::TrySetToArrayBufferView(BindingCallContext& cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  tryNext = false;
  { // scope for memberSlot
    ArrayBufferView& memberSlot = RawSetAsArrayBufferView();
    if (!memberSlot.Init(&value.toObject())) {
      DestroyArrayBufferView();
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

bool
OwningArrayBufferViewOrArrayBuffer::TrySetToArrayBufferView(JSContext* cx_, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  BindingCallContext cx(cx_, nullptr);
  return TrySetToArrayBufferView(cx, value, tryNext, passedToJSImpl);
}

void
OwningArrayBufferViewOrArrayBuffer::DestroyArrayBufferView()
{
  MOZ_ASSERT(IsArrayBufferView(), "Wrong type!");
  mValue.mArrayBufferView.Destroy();
  mType = eUninitialized;
}




ArrayBuffer&
OwningArrayBufferViewOrArrayBuffer::RawSetAsArrayBuffer()
{
  if (mType == eArrayBuffer) {
    return mValue.mArrayBuffer.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eArrayBuffer;
  return mValue.mArrayBuffer.SetValue();
}

ArrayBuffer&
OwningArrayBufferViewOrArrayBuffer::SetAsArrayBuffer()
{
  if (mType == eArrayBuffer) {
    return mValue.mArrayBuffer.Value();
  }
  Uninit();
  mType = eArrayBuffer;
  return mValue.mArrayBuffer.SetValue();
}

bool
OwningArrayBufferViewOrArrayBuffer::TrySetToArrayBuffer(BindingCallContext& cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  tryNext = false;
  { // scope for memberSlot
    ArrayBuffer& memberSlot = RawSetAsArrayBuffer();
    if (!memberSlot.Init(&value.toObject())) {
      DestroyArrayBuffer();
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

bool
OwningArrayBufferViewOrArrayBuffer::TrySetToArrayBuffer(JSContext* cx_, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  BindingCallContext cx(cx_, nullptr);
  return TrySetToArrayBuffer(cx, value, tryNext, passedToJSImpl);
}

void
OwningArrayBufferViewOrArrayBuffer::DestroyArrayBuffer()
{
  MOZ_ASSERT(IsArrayBuffer(), "Wrong type!");
  mValue.mArrayBuffer.Destroy();
  mType = eUninitialized;
}




void
OwningArrayBufferViewOrArrayBuffer::Uninit()
{
  switch (mType) {
    case eUninitialized: {
      break;
    }
    case eArrayBufferView: {
      DestroyArrayBufferView();
      break;
    }
    case eArrayBuffer: {
      DestroyArrayBuffer();
      break;
    }
  }
}

bool
OwningArrayBufferViewOrArrayBuffer::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eArrayBufferView: {
      rval.setObject(*mValue.mArrayBufferView.Value().Obj());
      if (!MaybeWrapNonDOMObjectValue(cx, rval)) {
        return false;
      }
      return true;
      break;
    }
    case eArrayBuffer: {
      rval.setObject(*mValue.mArrayBuffer.Value().Obj());
      if (!MaybeWrapNonDOMObjectValue(cx, rval)) {
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

void
OwningArrayBufferViewOrArrayBuffer::TraceUnion(JSTracer* trc)
{
  switch (mType) {
    case eArrayBufferView: {
      mValue.mArrayBufferView.Value().TraceSelf(trc);
      break;
    }
    case eArrayBuffer: {
      mValue.mArrayBuffer.Value().TraceSelf(trc);
      break;
    }
    default: {
      break;
    }
  }
}

nsCString&
OwningByteStringOrLong::RawSetAsByteString()
{
  if (mType == eByteString) {
    return mValue.mByteString.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eByteString;
  return mValue.mByteString.SetValue();
}

nsCString&
OwningByteStringOrLong::SetAsByteString()
{
  if (mType == eByteString) {
    return mValue.mByteString.Value();
  }
  Uninit();
  mType = eByteString;
  return mValue.mByteString.SetValue();
}

bool
OwningByteStringOrLong::TrySetToByteString(BindingCallContext& cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
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

bool
OwningByteStringOrLong::TrySetToByteString(JSContext* cx_, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  BindingCallContext cx(cx_, nullptr);
  return TrySetToByteString(cx, value, tryNext, passedToJSImpl);
}


void
OwningByteStringOrLong::DestroyByteString()
{
  MOZ_ASSERT(IsByteString(), "Wrong type!");
  mValue.mByteString.Destroy();
  mType = eUninitialized;
}




int32_t&
OwningByteStringOrLong::RawSetAsLong()
{
  if (mType == eLong) {
    return mValue.mLong.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eLong;
  return mValue.mLong.SetValue();
}

int32_t&
OwningByteStringOrLong::SetAsLong()
{
  if (mType == eLong) {
    return mValue.mLong.Value();
  }
  Uninit();
  mType = eLong;
  return mValue.mLong.SetValue();
}

bool
OwningByteStringOrLong::TrySetToLong(JSContext* cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
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

void
OwningByteStringOrLong::DestroyLong()
{
  MOZ_ASSERT(IsLong(), "Wrong type!");
  mValue.mLong.Destroy();
  mType = eUninitialized;
}




void
OwningByteStringOrLong::Uninit()
{
  switch (mType) {
    case eUninitialized: {
      break;
    }
    case eByteString: {
      DestroyByteString();
      break;
    }
    case eLong: {
      DestroyLong();
      break;
    }
  }
}

bool
OwningByteStringOrLong::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eByteString: {
      if (!NonVoidByteStringToJsval(cx, mValue.mByteString.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    case eLong: {
      rval.setInt32(int32_t(mValue.mLong.Value()));
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

void
OwningByteStringOrLong::TraceUnion(JSTracer* trc)
{
}

OwningByteStringOrLong&
OwningByteStringOrLong::operator=(const OwningByteStringOrLong& aOther)
{
  switch (aOther.mType) {
    case eUninitialized: {
      MOZ_ASSERT(mType == eUninitialized,
                 "We need to destroy ourselves?");
      break;
    }
    case eByteString: {
      SetAsByteString() = aOther.GetAsByteString();
      break;
    }
    case eLong: {
      SetAsLong() = aOther.GetAsLong();
      break;
    }
  }
  return *this;
}

OwningNonNull<mozilla::dom::CanvasPattern>&
OwningCanvasPatternOrCanvasGradient::RawSetAsCanvasPattern()
{
  if (mType == eCanvasPattern) {
    return mValue.mCanvasPattern.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eCanvasPattern;
  return mValue.mCanvasPattern.SetValue();
}

OwningNonNull<mozilla::dom::CanvasPattern>&
OwningCanvasPatternOrCanvasGradient::SetAsCanvasPattern()
{
  if (mType == eCanvasPattern) {
    return mValue.mCanvasPattern.Value();
  }
  Uninit();
  mType = eCanvasPattern;
  return mValue.mCanvasPattern.SetValue();
}

bool
OwningCanvasPatternOrCanvasGradient::TrySetToCanvasPattern(BindingCallContext& cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  tryNext = false;
  { // scope for memberSlot
    OwningNonNull<mozilla::dom::CanvasPattern>& memberSlot = RawSetAsCanvasPattern();
    static_assert(IsRefcounted<mozilla::dom::CanvasPattern>::value, "We can only store refcounted classes.");
    {
      // Our JSContext should be in the right global to do unwrapping in.
      nsresult rv = UnwrapObject<prototypes::id::CanvasPattern, mozilla::dom::CanvasPattern>(value, memberSlot, cx);
      if (NS_FAILED(rv)) {
        DestroyCanvasPattern();
        tryNext = true;
        return true;
      }
    }
  }
  return true;
}

bool
OwningCanvasPatternOrCanvasGradient::TrySetToCanvasPattern(JSContext* cx_, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  BindingCallContext cx(cx_, nullptr);
  return TrySetToCanvasPattern(cx, value, tryNext, passedToJSImpl);
}

void
OwningCanvasPatternOrCanvasGradient::DestroyCanvasPattern()
{
  MOZ_ASSERT(IsCanvasPattern(), "Wrong type!");
  mValue.mCanvasPattern.Destroy();
  mType = eUninitialized;
}




OwningNonNull<mozilla::dom::CanvasGradient>&
OwningCanvasPatternOrCanvasGradient::RawSetAsCanvasGradient()
{
  if (mType == eCanvasGradient) {
    return mValue.mCanvasGradient.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eCanvasGradient;
  return mValue.mCanvasGradient.SetValue();
}

OwningNonNull<mozilla::dom::CanvasGradient>&
OwningCanvasPatternOrCanvasGradient::SetAsCanvasGradient()
{
  if (mType == eCanvasGradient) {
    return mValue.mCanvasGradient.Value();
  }
  Uninit();
  mType = eCanvasGradient;
  return mValue.mCanvasGradient.SetValue();
}

bool
OwningCanvasPatternOrCanvasGradient::TrySetToCanvasGradient(BindingCallContext& cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  tryNext = false;
  { // scope for memberSlot
    OwningNonNull<mozilla::dom::CanvasGradient>& memberSlot = RawSetAsCanvasGradient();
    static_assert(IsRefcounted<mozilla::dom::CanvasGradient>::value, "We can only store refcounted classes.");
    {
      // Our JSContext should be in the right global to do unwrapping in.
      nsresult rv = UnwrapObject<prototypes::id::CanvasGradient, mozilla::dom::CanvasGradient>(value, memberSlot, cx);
      if (NS_FAILED(rv)) {
        DestroyCanvasGradient();
        tryNext = true;
        return true;
      }
    }
  }
  return true;
}

bool
OwningCanvasPatternOrCanvasGradient::TrySetToCanvasGradient(JSContext* cx_, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  BindingCallContext cx(cx_, nullptr);
  return TrySetToCanvasGradient(cx, value, tryNext, passedToJSImpl);
}

void
OwningCanvasPatternOrCanvasGradient::DestroyCanvasGradient()
{
  MOZ_ASSERT(IsCanvasGradient(), "Wrong type!");
  mValue.mCanvasGradient.Destroy();
  mType = eUninitialized;
}




void
OwningCanvasPatternOrCanvasGradient::Uninit()
{
  switch (mType) {
    case eUninitialized: {
      break;
    }
    case eCanvasPattern: {
      DestroyCanvasPattern();
      break;
    }
    case eCanvasGradient: {
      DestroyCanvasGradient();
      break;
    }
  }
}

bool
OwningCanvasPatternOrCanvasGradient::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eCanvasPattern: {
      if (!GetOrCreateDOMReflector(cx, mValue.mCanvasPattern.Value(), rval)) {
        MOZ_ASSERT(JS_IsExceptionPending(cx));
        return false;
      }
      return true;
      break;
    }
    case eCanvasGradient: {
      if (!GetOrCreateDOMReflector(cx, mValue.mCanvasGradient.Value(), rval)) {
        MOZ_ASSERT(JS_IsExceptionPending(cx));
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

void
OwningCanvasPatternOrCanvasGradient::TraceUnion(JSTracer* trc)
{
}

OwningCanvasPatternOrCanvasGradient&
OwningCanvasPatternOrCanvasGradient::operator=(const OwningCanvasPatternOrCanvasGradient& aOther)
{
  switch (aOther.mType) {
    case eUninitialized: {
      MOZ_ASSERT(mType == eUninitialized,
                 "We need to destroy ourselves?");
      break;
    }
    case eCanvasPattern: {
      SetAsCanvasPattern() = aOther.GetAsCanvasPattern();
      break;
    }
    case eCanvasGradient: {
      SetAsCanvasGradient() = aOther.GetAsCanvasGradient();
      break;
    }
  }
  return *this;
}

OwningNonNull<mozilla::dom::CanvasPattern>&
OwningCanvasPatternOrNullOrCanvasGradient::RawSetAsCanvasPattern()
{
  if (mType == eCanvasPattern) {
    return mValue.mCanvasPattern.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eCanvasPattern;
  return mValue.mCanvasPattern.SetValue();
}

OwningNonNull<mozilla::dom::CanvasPattern>&
OwningCanvasPatternOrNullOrCanvasGradient::SetAsCanvasPattern()
{
  if (mType == eCanvasPattern) {
    return mValue.mCanvasPattern.Value();
  }
  Uninit();
  mType = eCanvasPattern;
  return mValue.mCanvasPattern.SetValue();
}

bool
OwningCanvasPatternOrNullOrCanvasGradient::TrySetToCanvasPattern(BindingCallContext& cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  tryNext = false;
  { // scope for memberSlot
    OwningNonNull<mozilla::dom::CanvasPattern>& memberSlot = RawSetAsCanvasPattern();
    static_assert(IsRefcounted<mozilla::dom::CanvasPattern>::value, "We can only store refcounted classes.");
    {
      // Our JSContext should be in the right global to do unwrapping in.
      nsresult rv = UnwrapObject<prototypes::id::CanvasPattern, mozilla::dom::CanvasPattern>(value, memberSlot, cx);
      if (NS_FAILED(rv)) {
        DestroyCanvasPattern();
        tryNext = true;
        return true;
      }
    }
  }
  return true;
}

bool
OwningCanvasPatternOrNullOrCanvasGradient::TrySetToCanvasPattern(JSContext* cx_, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  BindingCallContext cx(cx_, nullptr);
  return TrySetToCanvasPattern(cx, value, tryNext, passedToJSImpl);
}

void
OwningCanvasPatternOrNullOrCanvasGradient::DestroyCanvasPattern()
{
  MOZ_ASSERT(IsCanvasPattern(), "Wrong type!");
  mValue.mCanvasPattern.Destroy();
  mType = eUninitialized;
}




OwningNonNull<mozilla::dom::CanvasGradient>&
OwningCanvasPatternOrNullOrCanvasGradient::RawSetAsCanvasGradient()
{
  if (mType == eCanvasGradient) {
    return mValue.mCanvasGradient.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eCanvasGradient;
  return mValue.mCanvasGradient.SetValue();
}

OwningNonNull<mozilla::dom::CanvasGradient>&
OwningCanvasPatternOrNullOrCanvasGradient::SetAsCanvasGradient()
{
  if (mType == eCanvasGradient) {
    return mValue.mCanvasGradient.Value();
  }
  Uninit();
  mType = eCanvasGradient;
  return mValue.mCanvasGradient.SetValue();
}

bool
OwningCanvasPatternOrNullOrCanvasGradient::TrySetToCanvasGradient(BindingCallContext& cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  tryNext = false;
  { // scope for memberSlot
    OwningNonNull<mozilla::dom::CanvasGradient>& memberSlot = RawSetAsCanvasGradient();
    static_assert(IsRefcounted<mozilla::dom::CanvasGradient>::value, "We can only store refcounted classes.");
    {
      // Our JSContext should be in the right global to do unwrapping in.
      nsresult rv = UnwrapObject<prototypes::id::CanvasGradient, mozilla::dom::CanvasGradient>(value, memberSlot, cx);
      if (NS_FAILED(rv)) {
        DestroyCanvasGradient();
        tryNext = true;
        return true;
      }
    }
  }
  return true;
}

bool
OwningCanvasPatternOrNullOrCanvasGradient::TrySetToCanvasGradient(JSContext* cx_, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  BindingCallContext cx(cx_, nullptr);
  return TrySetToCanvasGradient(cx, value, tryNext, passedToJSImpl);
}

void
OwningCanvasPatternOrNullOrCanvasGradient::DestroyCanvasGradient()
{
  MOZ_ASSERT(IsCanvasGradient(), "Wrong type!");
  mValue.mCanvasGradient.Destroy();
  mType = eUninitialized;
}




void
OwningCanvasPatternOrNullOrCanvasGradient::Uninit()
{
  switch (mType) {
    case eUninitialized: {
      break;
    }
    case eNull: {
      break;
    }
    case eCanvasPattern: {
      DestroyCanvasPattern();
      break;
    }
    case eCanvasGradient: {
      DestroyCanvasGradient();
      break;
    }
  }
}

bool
OwningCanvasPatternOrNullOrCanvasGradient::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eNull: {
      rval.setNull();
      return true;
      break;
    }
    case eCanvasPattern: {
      if (!GetOrCreateDOMReflector(cx, mValue.mCanvasPattern.Value(), rval)) {
        MOZ_ASSERT(JS_IsExceptionPending(cx));
        return false;
      }
      return true;
      break;
    }
    case eCanvasGradient: {
      if (!GetOrCreateDOMReflector(cx, mValue.mCanvasGradient.Value(), rval)) {
        MOZ_ASSERT(JS_IsExceptionPending(cx));
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

void
OwningCanvasPatternOrNullOrCanvasGradient::TraceUnion(JSTracer* trc)
{
}

OwningCanvasPatternOrNullOrCanvasGradient&
OwningCanvasPatternOrNullOrCanvasGradient::operator=(const OwningCanvasPatternOrNullOrCanvasGradient& aOther)
{
  switch (aOther.mType) {
    case eUninitialized: {
      MOZ_ASSERT(mType == eUninitialized,
                 "We need to destroy ourselves?");
      break;
    }
    case eNull: {
      MOZ_ASSERT(mType == eUninitialized);
      mType = eNull;
      break;
    }
    case eCanvasPattern: {
      SetAsCanvasPattern() = aOther.GetAsCanvasPattern();
      break;
    }
    case eCanvasGradient: {
      SetAsCanvasGradient() = aOther.GetAsCanvasGradient();
      break;
    }
  }
  return *this;
}

double&
OwningDoubleOrByteString::RawSetAsDouble()
{
  if (mType == eDouble) {
    return mValue.mDouble.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eDouble;
  return mValue.mDouble.SetValue();
}

double&
OwningDoubleOrByteString::SetAsDouble()
{
  if (mType == eDouble) {
    return mValue.mDouble.Value();
  }
  Uninit();
  mType = eDouble;
  return mValue.mDouble.SetValue();
}

bool
OwningDoubleOrByteString::TrySetToDouble(BindingCallContext& cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
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

bool
OwningDoubleOrByteString::TrySetToDouble(JSContext* cx_, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  BindingCallContext cx(cx_, nullptr);
  return TrySetToDouble(cx, value, tryNext, passedToJSImpl);
}

void
OwningDoubleOrByteString::DestroyDouble()
{
  MOZ_ASSERT(IsDouble(), "Wrong type!");
  mValue.mDouble.Destroy();
  mType = eUninitialized;
}




nsCString&
OwningDoubleOrByteString::RawSetAsByteString()
{
  if (mType == eByteString) {
    return mValue.mByteString.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eByteString;
  return mValue.mByteString.SetValue();
}

nsCString&
OwningDoubleOrByteString::SetAsByteString()
{
  if (mType == eByteString) {
    return mValue.mByteString.Value();
  }
  Uninit();
  mType = eByteString;
  return mValue.mByteString.SetValue();
}

bool
OwningDoubleOrByteString::TrySetToByteString(BindingCallContext& cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
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

bool
OwningDoubleOrByteString::TrySetToByteString(JSContext* cx_, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  BindingCallContext cx(cx_, nullptr);
  return TrySetToByteString(cx, value, tryNext, passedToJSImpl);
}


void
OwningDoubleOrByteString::DestroyByteString()
{
  MOZ_ASSERT(IsByteString(), "Wrong type!");
  mValue.mByteString.Destroy();
  mType = eUninitialized;
}




void
OwningDoubleOrByteString::Uninit()
{
  switch (mType) {
    case eUninitialized: {
      break;
    }
    case eDouble: {
      DestroyDouble();
      break;
    }
    case eByteString: {
      DestroyByteString();
      break;
    }
  }
}

bool
OwningDoubleOrByteString::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eDouble: {
      rval.set(JS_NumberValue(double(mValue.mDouble.Value())));
      return true;
      break;
    }
    case eByteString: {
      if (!NonVoidByteStringToJsval(cx, mValue.mByteString.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

void
OwningDoubleOrByteString::TraceUnion(JSTracer* trc)
{
}

OwningDoubleOrByteString&
OwningDoubleOrByteString::operator=(const OwningDoubleOrByteString& aOther)
{
  switch (aOther.mType) {
    case eUninitialized: {
      MOZ_ASSERT(mType == eUninitialized,
                 "We need to destroy ourselves?");
      break;
    }
    case eDouble: {
      SetAsDouble() = aOther.GetAsDouble();
      break;
    }
    case eByteString: {
      SetAsByteString() = aOther.GetAsByteString();
      break;
    }
  }
  return *this;
}

double&
OwningDoubleOrString::RawSetAsDouble()
{
  if (mType == eDouble) {
    return mValue.mDouble.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eDouble;
  return mValue.mDouble.SetValue();
}

double&
OwningDoubleOrString::SetAsDouble()
{
  if (mType == eDouble) {
    return mValue.mDouble.Value();
  }
  Uninit();
  mType = eDouble;
  return mValue.mDouble.SetValue();
}

bool
OwningDoubleOrString::TrySetToDouble(BindingCallContext& cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
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

bool
OwningDoubleOrString::TrySetToDouble(JSContext* cx_, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  BindingCallContext cx(cx_, nullptr);
  return TrySetToDouble(cx, value, tryNext, passedToJSImpl);
}

void
OwningDoubleOrString::DestroyDouble()
{
  MOZ_ASSERT(IsDouble(), "Wrong type!");
  mValue.mDouble.Destroy();
  mType = eUninitialized;
}




nsString&
OwningDoubleOrString::RawSetAsString()
{
  if (mType == eString) {
    return mValue.mString.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eString;
  return mValue.mString.SetValue();
}

nsString&
OwningDoubleOrString::SetAsString()
{
  if (mType == eString) {
    return mValue.mString.Value();
  }
  Uninit();
  mType = eString;
  return mValue.mString.SetValue();
}

bool
OwningDoubleOrString::TrySetToString(JSContext* cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  tryNext = false;
  { // scope for memberSlot
    nsString& memberSlot = RawSetAsString();
    if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
      return false;
    }
  }
  return true;
}


void
OwningDoubleOrString::DestroyString()
{
  MOZ_ASSERT(IsString(), "Wrong type!");
  mValue.mString.Destroy();
  mType = eUninitialized;
}




void
OwningDoubleOrString::Uninit()
{
  switch (mType) {
    case eUninitialized: {
      break;
    }
    case eDouble: {
      DestroyDouble();
      break;
    }
    case eString: {
      DestroyString();
      break;
    }
  }
}

bool
OwningDoubleOrString::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eDouble: {
      rval.set(JS_NumberValue(double(mValue.mDouble.Value())));
      return true;
      break;
    }
    case eString: {
      if (!xpc::NonVoidStringToJsval(cx, mValue.mString.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

void
OwningDoubleOrString::TraceUnion(JSTracer* trc)
{
}

OwningDoubleOrString&
OwningDoubleOrString::operator=(const OwningDoubleOrString& aOther)
{
  switch (aOther.mType) {
    case eUninitialized: {
      MOZ_ASSERT(mType == eUninitialized,
                 "We need to destroy ourselves?");
      break;
    }
    case eDouble: {
      SetAsDouble() = aOther.GetAsDouble();
      break;
    }
    case eString: {
      SetAsString() = aOther.GetAsString();
      break;
    }
  }
  return *this;
}

double&
OwningDoubleOrSupportedType::RawSetAsDouble()
{
  if (mType == eDouble) {
    return mValue.mDouble.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eDouble;
  return mValue.mDouble.SetValue();
}

double&
OwningDoubleOrSupportedType::SetAsDouble()
{
  if (mType == eDouble) {
    return mValue.mDouble.Value();
  }
  Uninit();
  mType = eDouble;
  return mValue.mDouble.SetValue();
}

bool
OwningDoubleOrSupportedType::TrySetToDouble(BindingCallContext& cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
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

bool
OwningDoubleOrSupportedType::TrySetToDouble(JSContext* cx_, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  BindingCallContext cx(cx_, nullptr);
  return TrySetToDouble(cx, value, tryNext, passedToJSImpl);
}

void
OwningDoubleOrSupportedType::DestroyDouble()
{
  MOZ_ASSERT(IsDouble(), "Wrong type!");
  mValue.mDouble.Destroy();
  mType = eUninitialized;
}




SupportedType&
OwningDoubleOrSupportedType::RawSetAsSupportedType()
{
  if (mType == eSupportedType) {
    return mValue.mSupportedType.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eSupportedType;
  return mValue.mSupportedType.SetValue();
}

SupportedType&
OwningDoubleOrSupportedType::SetAsSupportedType()
{
  if (mType == eSupportedType) {
    return mValue.mSupportedType.Value();
  }
  Uninit();
  mType = eSupportedType;
  return mValue.mSupportedType.SetValue();
}

bool
OwningDoubleOrSupportedType::TrySetToSupportedType(BindingCallContext& cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
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

bool
OwningDoubleOrSupportedType::TrySetToSupportedType(JSContext* cx_, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  BindingCallContext cx(cx_, nullptr);
  return TrySetToSupportedType(cx, value, tryNext, passedToJSImpl);
}

void
OwningDoubleOrSupportedType::DestroySupportedType()
{
  MOZ_ASSERT(IsSupportedType(), "Wrong type!");
  mValue.mSupportedType.Destroy();
  mType = eUninitialized;
}




void
OwningDoubleOrSupportedType::Uninit()
{
  switch (mType) {
    case eUninitialized: {
      break;
    }
    case eDouble: {
      DestroyDouble();
      break;
    }
    case eSupportedType: {
      DestroySupportedType();
      break;
    }
  }
}

bool
OwningDoubleOrSupportedType::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eDouble: {
      rval.set(JS_NumberValue(double(mValue.mDouble.Value())));
      return true;
      break;
    }
    case eSupportedType: {
      if (!ToJSValue(cx, mValue.mSupportedType.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

void
OwningDoubleOrSupportedType::TraceUnion(JSTracer* trc)
{
}

OwningDoubleOrSupportedType&
OwningDoubleOrSupportedType::operator=(const OwningDoubleOrSupportedType& aOther)
{
  switch (aOther.mType) {
    case eUninitialized: {
      MOZ_ASSERT(mType == eUninitialized,
                 "We need to destroy ourselves?");
      break;
    }
    case eDouble: {
      SetAsDouble() = aOther.GetAsDouble();
      break;
    }
    case eSupportedType: {
      SetAsSupportedType() = aOther.GetAsSupportedType();
      break;
    }
  }
  return *this;
}

double&
OwningDoubleOrUSVString::RawSetAsDouble()
{
  if (mType == eDouble) {
    return mValue.mDouble.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eDouble;
  return mValue.mDouble.SetValue();
}

double&
OwningDoubleOrUSVString::SetAsDouble()
{
  if (mType == eDouble) {
    return mValue.mDouble.Value();
  }
  Uninit();
  mType = eDouble;
  return mValue.mDouble.SetValue();
}

bool
OwningDoubleOrUSVString::TrySetToDouble(BindingCallContext& cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
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

bool
OwningDoubleOrUSVString::TrySetToDouble(JSContext* cx_, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  BindingCallContext cx(cx_, nullptr);
  return TrySetToDouble(cx, value, tryNext, passedToJSImpl);
}

void
OwningDoubleOrUSVString::DestroyDouble()
{
  MOZ_ASSERT(IsDouble(), "Wrong type!");
  mValue.mDouble.Destroy();
  mType = eUninitialized;
}




nsString&
OwningDoubleOrUSVString::RawSetAsUSVString()
{
  if (mType == eUSVString) {
    return mValue.mUSVString.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eUSVString;
  return mValue.mUSVString.SetValue();
}

nsString&
OwningDoubleOrUSVString::SetAsUSVString()
{
  if (mType == eUSVString) {
    return mValue.mUSVString.Value();
  }
  Uninit();
  mType = eUSVString;
  return mValue.mUSVString.SetValue();
}

bool
OwningDoubleOrUSVString::TrySetToUSVString(JSContext* cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  tryNext = false;
  { // scope for memberSlot
    nsString& memberSlot = RawSetAsUSVString();
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


void
OwningDoubleOrUSVString::DestroyUSVString()
{
  MOZ_ASSERT(IsUSVString(), "Wrong type!");
  mValue.mUSVString.Destroy();
  mType = eUninitialized;
}




void
OwningDoubleOrUSVString::Uninit()
{
  switch (mType) {
    case eUninitialized: {
      break;
    }
    case eDouble: {
      DestroyDouble();
      break;
    }
    case eUSVString: {
      DestroyUSVString();
      break;
    }
  }
}

bool
OwningDoubleOrUSVString::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eDouble: {
      rval.set(JS_NumberValue(double(mValue.mDouble.Value())));
      return true;
      break;
    }
    case eUSVString: {
      if (!xpc::NonVoidStringToJsval(cx, mValue.mUSVString.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

void
OwningDoubleOrUSVString::TraceUnion(JSTracer* trc)
{
}

OwningDoubleOrUSVString&
OwningDoubleOrUSVString::operator=(const OwningDoubleOrUSVString& aOther)
{
  switch (aOther.mType) {
    case eUninitialized: {
      MOZ_ASSERT(mType == eUninitialized,
                 "We need to destroy ourselves?");
      break;
    }
    case eDouble: {
      SetAsDouble() = aOther.GetAsDouble();
      break;
    }
    case eUSVString: {
      SetAsUSVString() = aOther.GetAsUSVString();
      break;
    }
  }
  return *this;
}

double&
OwningDoubleOrUTF8String::RawSetAsDouble()
{
  if (mType == eDouble) {
    return mValue.mDouble.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eDouble;
  return mValue.mDouble.SetValue();
}

double&
OwningDoubleOrUTF8String::SetAsDouble()
{
  if (mType == eDouble) {
    return mValue.mDouble.Value();
  }
  Uninit();
  mType = eDouble;
  return mValue.mDouble.SetValue();
}

bool
OwningDoubleOrUTF8String::TrySetToDouble(BindingCallContext& cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
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

bool
OwningDoubleOrUTF8String::TrySetToDouble(JSContext* cx_, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  BindingCallContext cx(cx_, nullptr);
  return TrySetToDouble(cx, value, tryNext, passedToJSImpl);
}

void
OwningDoubleOrUTF8String::DestroyDouble()
{
  MOZ_ASSERT(IsDouble(), "Wrong type!");
  mValue.mDouble.Destroy();
  mType = eUninitialized;
}




nsCString&
OwningDoubleOrUTF8String::RawSetAsUTF8String()
{
  if (mType == eUTF8String) {
    return mValue.mUTF8String.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eUTF8String;
  return mValue.mUTF8String.SetValue();
}

nsCString&
OwningDoubleOrUTF8String::SetAsUTF8String()
{
  if (mType == eUTF8String) {
    return mValue.mUTF8String.Value();
  }
  Uninit();
  mType = eUTF8String;
  return mValue.mUTF8String.SetValue();
}

bool
OwningDoubleOrUTF8String::TrySetToUTF8String(JSContext* cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  tryNext = false;
  { // scope for memberSlot
    nsCString& memberSlot = RawSetAsUTF8String();
    if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
      return false;
    }
  }
  return true;
}


void
OwningDoubleOrUTF8String::DestroyUTF8String()
{
  MOZ_ASSERT(IsUTF8String(), "Wrong type!");
  mValue.mUTF8String.Destroy();
  mType = eUninitialized;
}




void
OwningDoubleOrUTF8String::Uninit()
{
  switch (mType) {
    case eUninitialized: {
      break;
    }
    case eDouble: {
      DestroyDouble();
      break;
    }
    case eUTF8String: {
      DestroyUTF8String();
      break;
    }
  }
}

bool
OwningDoubleOrUTF8String::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eDouble: {
      rval.set(JS_NumberValue(double(mValue.mDouble.Value())));
      return true;
      break;
    }
    case eUTF8String: {
      if (!NonVoidUTF8StringToJsval(cx, mValue.mUTF8String.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

void
OwningDoubleOrUTF8String::TraceUnion(JSTracer* trc)
{
}

OwningDoubleOrUTF8String&
OwningDoubleOrUTF8String::operator=(const OwningDoubleOrUTF8String& aOther)
{
  switch (aOther.mType) {
    case eUninitialized: {
      MOZ_ASSERT(mType == eUninitialized,
                 "We need to destroy ourselves?");
      break;
    }
    case eDouble: {
      SetAsDouble() = aOther.GetAsDouble();
      break;
    }
    case eUTF8String: {
      SetAsUTF8String() = aOther.GetAsUTF8String();
      break;
    }
  }
  return *this;
}

OwningNonNull<mozilla::dom::File>&
OwningFileOrDirectory::RawSetAsFile()
{
  if (mType == eFile) {
    return mValue.mFile.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eFile;
  return mValue.mFile.SetValue();
}

OwningNonNull<mozilla::dom::File>&
OwningFileOrDirectory::SetAsFile()
{
  if (mType == eFile) {
    return mValue.mFile.Value();
  }
  Uninit();
  mType = eFile;
  return mValue.mFile.SetValue();
}

bool
OwningFileOrDirectory::TrySetToFile(BindingCallContext& cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  tryNext = false;
  { // scope for memberSlot
    OwningNonNull<mozilla::dom::File>& memberSlot = RawSetAsFile();
    static_assert(IsRefcounted<mozilla::dom::File>::value, "We can only store refcounted classes.");
    {
      // Our JSContext should be in the right global to do unwrapping in.
      nsresult rv = UnwrapObject<prototypes::id::File, mozilla::dom::File>(value, memberSlot, cx);
      if (NS_FAILED(rv)) {
        DestroyFile();
        tryNext = true;
        return true;
      }
    }
  }
  return true;
}

bool
OwningFileOrDirectory::TrySetToFile(JSContext* cx_, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  BindingCallContext cx(cx_, nullptr);
  return TrySetToFile(cx, value, tryNext, passedToJSImpl);
}

void
OwningFileOrDirectory::DestroyFile()
{
  MOZ_ASSERT(IsFile(), "Wrong type!");
  mValue.mFile.Destroy();
  mType = eUninitialized;
}




OwningNonNull<mozilla::dom::Directory>&
OwningFileOrDirectory::RawSetAsDirectory()
{
  if (mType == eDirectory) {
    return mValue.mDirectory.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eDirectory;
  return mValue.mDirectory.SetValue();
}

OwningNonNull<mozilla::dom::Directory>&
OwningFileOrDirectory::SetAsDirectory()
{
  if (mType == eDirectory) {
    return mValue.mDirectory.Value();
  }
  Uninit();
  mType = eDirectory;
  return mValue.mDirectory.SetValue();
}

bool
OwningFileOrDirectory::TrySetToDirectory(BindingCallContext& cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  tryNext = false;
  { // scope for memberSlot
    OwningNonNull<mozilla::dom::Directory>& memberSlot = RawSetAsDirectory();
    static_assert(IsRefcounted<mozilla::dom::Directory>::value, "We can only store refcounted classes.");
    {
      // Our JSContext should be in the right global to do unwrapping in.
      nsresult rv = UnwrapObject<prototypes::id::Directory, mozilla::dom::Directory>(value, memberSlot, cx);
      if (NS_FAILED(rv)) {
        DestroyDirectory();
        tryNext = true;
        return true;
      }
    }
  }
  return true;
}

bool
OwningFileOrDirectory::TrySetToDirectory(JSContext* cx_, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  BindingCallContext cx(cx_, nullptr);
  return TrySetToDirectory(cx, value, tryNext, passedToJSImpl);
}

void
OwningFileOrDirectory::DestroyDirectory()
{
  MOZ_ASSERT(IsDirectory(), "Wrong type!");
  mValue.mDirectory.Destroy();
  mType = eUninitialized;
}




void
OwningFileOrDirectory::Uninit()
{
  switch (mType) {
    case eUninitialized: {
      break;
    }
    case eFile: {
      DestroyFile();
      break;
    }
    case eDirectory: {
      DestroyDirectory();
      break;
    }
  }
}

bool
OwningFileOrDirectory::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eFile: {
      if (!GetOrCreateDOMReflector(cx, mValue.mFile.Value(), rval)) {
        MOZ_ASSERT(JS_IsExceptionPending(cx));
        return false;
      }
      return true;
      break;
    }
    case eDirectory: {
      if (!GetOrCreateDOMReflector(cx, mValue.mDirectory.Value(), rval)) {
        MOZ_ASSERT(JS_IsExceptionPending(cx));
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

void
OwningFileOrDirectory::TraceUnion(JSTracer* trc)
{
}

OwningFileOrDirectory&
OwningFileOrDirectory::operator=(const OwningFileOrDirectory& aOther)
{
  switch (aOther.mType) {
    case eUninitialized: {
      MOZ_ASSERT(mType == eUninitialized,
                 "We need to destroy ourselves?");
      break;
    }
    case eFile: {
      SetAsFile() = aOther.GetAsFile();
      break;
    }
    case eDirectory: {
      SetAsDirectory() = aOther.GetAsDirectory();
      break;
    }
  }
  return *this;
}

float&
OwningFloatOrString::RawSetAsFloat()
{
  if (mType == eFloat) {
    return mValue.mFloat.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eFloat;
  return mValue.mFloat.SetValue();
}

float&
OwningFloatOrString::SetAsFloat()
{
  if (mType == eFloat) {
    return mValue.mFloat.Value();
  }
  Uninit();
  mType = eFloat;
  return mValue.mFloat.SetValue();
}

bool
OwningFloatOrString::TrySetToFloat(BindingCallContext& cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
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

bool
OwningFloatOrString::TrySetToFloat(JSContext* cx_, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  BindingCallContext cx(cx_, nullptr);
  return TrySetToFloat(cx, value, tryNext, passedToJSImpl);
}

void
OwningFloatOrString::DestroyFloat()
{
  MOZ_ASSERT(IsFloat(), "Wrong type!");
  mValue.mFloat.Destroy();
  mType = eUninitialized;
}




nsString&
OwningFloatOrString::RawSetAsString()
{
  if (mType == eString) {
    return mValue.mString.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eString;
  return mValue.mString.SetValue();
}

nsString&
OwningFloatOrString::SetAsString()
{
  if (mType == eString) {
    return mValue.mString.Value();
  }
  Uninit();
  mType = eString;
  return mValue.mString.SetValue();
}

bool
OwningFloatOrString::TrySetToString(JSContext* cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  tryNext = false;
  { // scope for memberSlot
    nsString& memberSlot = RawSetAsString();
    if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
      return false;
    }
  }
  return true;
}


void
OwningFloatOrString::DestroyString()
{
  MOZ_ASSERT(IsString(), "Wrong type!");
  mValue.mString.Destroy();
  mType = eUninitialized;
}




void
OwningFloatOrString::Uninit()
{
  switch (mType) {
    case eUninitialized: {
      break;
    }
    case eFloat: {
      DestroyFloat();
      break;
    }
    case eString: {
      DestroyString();
      break;
    }
  }
}

bool
OwningFloatOrString::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eFloat: {
      rval.set(JS_NumberValue(double(mValue.mFloat.Value())));
      return true;
      break;
    }
    case eString: {
      if (!xpc::NonVoidStringToJsval(cx, mValue.mString.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

void
OwningFloatOrString::TraceUnion(JSTracer* trc)
{
}

OwningFloatOrString&
OwningFloatOrString::operator=(const OwningFloatOrString& aOther)
{
  switch (aOther.mType) {
    case eUninitialized: {
      MOZ_ASSERT(mType == eUninitialized,
                 "We need to destroy ourselves?");
      break;
    }
    case eFloat: {
      SetAsFloat() = aOther.GetAsFloat();
      break;
    }
    case eString: {
      SetAsString() = aOther.GetAsString();
      break;
    }
  }
  return *this;
}

OwningNonNull<nsGenericHTMLElement>&
OwningHTMLElementOrLong::RawSetAsHTMLElement()
{
  if (mType == eHTMLElement) {
    return mValue.mHTMLElement.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eHTMLElement;
  return mValue.mHTMLElement.SetValue();
}

OwningNonNull<nsGenericHTMLElement>&
OwningHTMLElementOrLong::SetAsHTMLElement()
{
  if (mType == eHTMLElement) {
    return mValue.mHTMLElement.Value();
  }
  Uninit();
  mType = eHTMLElement;
  return mValue.mHTMLElement.SetValue();
}

bool
OwningHTMLElementOrLong::TrySetToHTMLElement(BindingCallContext& cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  tryNext = false;
  { // scope for memberSlot
    OwningNonNull<nsGenericHTMLElement>& memberSlot = RawSetAsHTMLElement();
    static_assert(IsRefcounted<nsGenericHTMLElement>::value, "We can only store refcounted classes.");
    {
      // Our JSContext should be in the right global to do unwrapping in.
      nsresult rv = UnwrapObject<prototypes::id::HTMLElement, nsGenericHTMLElement>(value, memberSlot, cx);
      if (NS_FAILED(rv)) {
        DestroyHTMLElement();
        tryNext = true;
        return true;
      }
    }
  }
  return true;
}

bool
OwningHTMLElementOrLong::TrySetToHTMLElement(JSContext* cx_, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  BindingCallContext cx(cx_, nullptr);
  return TrySetToHTMLElement(cx, value, tryNext, passedToJSImpl);
}

void
OwningHTMLElementOrLong::DestroyHTMLElement()
{
  MOZ_ASSERT(IsHTMLElement(), "Wrong type!");
  mValue.mHTMLElement.Destroy();
  mType = eUninitialized;
}




int32_t&
OwningHTMLElementOrLong::RawSetAsLong()
{
  if (mType == eLong) {
    return mValue.mLong.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eLong;
  return mValue.mLong.SetValue();
}

int32_t&
OwningHTMLElementOrLong::SetAsLong()
{
  if (mType == eLong) {
    return mValue.mLong.Value();
  }
  Uninit();
  mType = eLong;
  return mValue.mLong.SetValue();
}

bool
OwningHTMLElementOrLong::TrySetToLong(JSContext* cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
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

void
OwningHTMLElementOrLong::DestroyLong()
{
  MOZ_ASSERT(IsLong(), "Wrong type!");
  mValue.mLong.Destroy();
  mType = eUninitialized;
}




void
OwningHTMLElementOrLong::Uninit()
{
  switch (mType) {
    case eUninitialized: {
      break;
    }
    case eHTMLElement: {
      DestroyHTMLElement();
      break;
    }
    case eLong: {
      DestroyLong();
      break;
    }
  }
}

bool
OwningHTMLElementOrLong::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eHTMLElement: {
      if (!GetOrCreateDOMReflector(cx, mValue.mHTMLElement.Value(), rval)) {
        MOZ_ASSERT(JS_IsExceptionPending(cx));
        return false;
      }
      return true;
      break;
    }
    case eLong: {
      rval.setInt32(int32_t(mValue.mLong.Value()));
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

void
OwningHTMLElementOrLong::TraceUnion(JSTracer* trc)
{
}

OwningHTMLElementOrLong&
OwningHTMLElementOrLong::operator=(const OwningHTMLElementOrLong& aOther)
{
  switch (aOther.mType) {
    case eUninitialized: {
      MOZ_ASSERT(mType == eUninitialized,
                 "We need to destroy ourselves?");
      break;
    }
    case eHTMLElement: {
      SetAsHTMLElement() = aOther.GetAsHTMLElement();
      break;
    }
    case eLong: {
      SetAsLong() = aOther.GetAsLong();
      break;
    }
  }
  return *this;
}

OwningNonNull<mozilla::dom::HTMLOptionElement>&
OwningHTMLOptionElementOrHTMLOptGroupElement::RawSetAsHTMLOptionElement()
{
  if (mType == eHTMLOptionElement) {
    return mValue.mHTMLOptionElement.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eHTMLOptionElement;
  return mValue.mHTMLOptionElement.SetValue();
}

OwningNonNull<mozilla::dom::HTMLOptionElement>&
OwningHTMLOptionElementOrHTMLOptGroupElement::SetAsHTMLOptionElement()
{
  if (mType == eHTMLOptionElement) {
    return mValue.mHTMLOptionElement.Value();
  }
  Uninit();
  mType = eHTMLOptionElement;
  return mValue.mHTMLOptionElement.SetValue();
}

bool
OwningHTMLOptionElementOrHTMLOptGroupElement::TrySetToHTMLOptionElement(BindingCallContext& cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  tryNext = false;
  { // scope for memberSlot
    OwningNonNull<mozilla::dom::HTMLOptionElement>& memberSlot = RawSetAsHTMLOptionElement();
    static_assert(IsRefcounted<mozilla::dom::HTMLOptionElement>::value, "We can only store refcounted classes.");
    {
      // Our JSContext should be in the right global to do unwrapping in.
      nsresult rv = UnwrapObject<prototypes::id::HTMLOptionElement, mozilla::dom::HTMLOptionElement>(value, memberSlot, cx);
      if (NS_FAILED(rv)) {
        DestroyHTMLOptionElement();
        tryNext = true;
        return true;
      }
    }
  }
  return true;
}

bool
OwningHTMLOptionElementOrHTMLOptGroupElement::TrySetToHTMLOptionElement(JSContext* cx_, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  BindingCallContext cx(cx_, nullptr);
  return TrySetToHTMLOptionElement(cx, value, tryNext, passedToJSImpl);
}

void
OwningHTMLOptionElementOrHTMLOptGroupElement::DestroyHTMLOptionElement()
{
  MOZ_ASSERT(IsHTMLOptionElement(), "Wrong type!");
  mValue.mHTMLOptionElement.Destroy();
  mType = eUninitialized;
}




OwningNonNull<mozilla::dom::HTMLOptGroupElement>&
OwningHTMLOptionElementOrHTMLOptGroupElement::RawSetAsHTMLOptGroupElement()
{
  if (mType == eHTMLOptGroupElement) {
    return mValue.mHTMLOptGroupElement.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eHTMLOptGroupElement;
  return mValue.mHTMLOptGroupElement.SetValue();
}

OwningNonNull<mozilla::dom::HTMLOptGroupElement>&
OwningHTMLOptionElementOrHTMLOptGroupElement::SetAsHTMLOptGroupElement()
{
  if (mType == eHTMLOptGroupElement) {
    return mValue.mHTMLOptGroupElement.Value();
  }
  Uninit();
  mType = eHTMLOptGroupElement;
  return mValue.mHTMLOptGroupElement.SetValue();
}

bool
OwningHTMLOptionElementOrHTMLOptGroupElement::TrySetToHTMLOptGroupElement(BindingCallContext& cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  tryNext = false;
  { // scope for memberSlot
    OwningNonNull<mozilla::dom::HTMLOptGroupElement>& memberSlot = RawSetAsHTMLOptGroupElement();
    static_assert(IsRefcounted<mozilla::dom::HTMLOptGroupElement>::value, "We can only store refcounted classes.");
    {
      // Our JSContext should be in the right global to do unwrapping in.
      nsresult rv = UnwrapObject<prototypes::id::HTMLOptGroupElement, mozilla::dom::HTMLOptGroupElement>(value, memberSlot, cx);
      if (NS_FAILED(rv)) {
        DestroyHTMLOptGroupElement();
        tryNext = true;
        return true;
      }
    }
  }
  return true;
}

bool
OwningHTMLOptionElementOrHTMLOptGroupElement::TrySetToHTMLOptGroupElement(JSContext* cx_, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  BindingCallContext cx(cx_, nullptr);
  return TrySetToHTMLOptGroupElement(cx, value, tryNext, passedToJSImpl);
}

void
OwningHTMLOptionElementOrHTMLOptGroupElement::DestroyHTMLOptGroupElement()
{
  MOZ_ASSERT(IsHTMLOptGroupElement(), "Wrong type!");
  mValue.mHTMLOptGroupElement.Destroy();
  mType = eUninitialized;
}




void
OwningHTMLOptionElementOrHTMLOptGroupElement::Uninit()
{
  switch (mType) {
    case eUninitialized: {
      break;
    }
    case eHTMLOptionElement: {
      DestroyHTMLOptionElement();
      break;
    }
    case eHTMLOptGroupElement: {
      DestroyHTMLOptGroupElement();
      break;
    }
  }
}

bool
OwningHTMLOptionElementOrHTMLOptGroupElement::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eHTMLOptionElement: {
      if (!GetOrCreateDOMReflector(cx, mValue.mHTMLOptionElement.Value(), rval)) {
        MOZ_ASSERT(JS_IsExceptionPending(cx));
        return false;
      }
      return true;
      break;
    }
    case eHTMLOptGroupElement: {
      if (!GetOrCreateDOMReflector(cx, mValue.mHTMLOptGroupElement.Value(), rval)) {
        MOZ_ASSERT(JS_IsExceptionPending(cx));
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

void
OwningHTMLOptionElementOrHTMLOptGroupElement::TraceUnion(JSTracer* trc)
{
}

OwningHTMLOptionElementOrHTMLOptGroupElement&
OwningHTMLOptionElementOrHTMLOptGroupElement::operator=(const OwningHTMLOptionElementOrHTMLOptGroupElement& aOther)
{
  switch (aOther.mType) {
    case eUninitialized: {
      MOZ_ASSERT(mType == eUninitialized,
                 "We need to destroy ourselves?");
      break;
    }
    case eHTMLOptionElement: {
      SetAsHTMLOptionElement() = aOther.GetAsHTMLOptionElement();
      break;
    }
    case eHTMLOptGroupElement: {
      SetAsHTMLOptGroupElement() = aOther.GetAsHTMLOptGroupElement();
      break;
    }
  }
  return *this;
}

int32_t&
OwningLongOrStringAnyRecord::RawSetAsLong()
{
  if (mType == eLong) {
    return mValue.mLong.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eLong;
  return mValue.mLong.SetValue();
}

int32_t&
OwningLongOrStringAnyRecord::SetAsLong()
{
  if (mType == eLong) {
    return mValue.mLong.Value();
  }
  Uninit();
  mType = eLong;
  return mValue.mLong.SetValue();
}

bool
OwningLongOrStringAnyRecord::TrySetToLong(JSContext* cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
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

void
OwningLongOrStringAnyRecord::DestroyLong()
{
  MOZ_ASSERT(IsLong(), "Wrong type!");
  mValue.mLong.Destroy();
  mType = eUninitialized;
}




Record<nsString, JS::Value>&
OwningLongOrStringAnyRecord::RawSetAsStringAnyRecord()
{
  if (mType == eStringAnyRecord) {
    return mValue.mStringAnyRecord.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eStringAnyRecord;
  return mValue.mStringAnyRecord.SetValue();
}

Record<nsString, JS::Value>&
OwningLongOrStringAnyRecord::SetAsStringAnyRecord()
{
  if (mType == eStringAnyRecord) {
    return mValue.mStringAnyRecord.Value();
  }
  Uninit();
  mType = eStringAnyRecord;
  return mValue.mStringAnyRecord.SetValue();
}

bool
OwningLongOrStringAnyRecord::TrySetToStringAnyRecord(BindingCallContext& cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  tryNext = false;
  { // scope for memberSlot
    Record<nsString, JS::Value>& memberSlot = RawSetAsStringAnyRecord();
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

bool
OwningLongOrStringAnyRecord::TrySetToStringAnyRecord(JSContext* cx_, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  BindingCallContext cx(cx_, nullptr);
  return TrySetToStringAnyRecord(cx, value, tryNext, passedToJSImpl);
}

void
OwningLongOrStringAnyRecord::DestroyStringAnyRecord()
{
  MOZ_ASSERT(IsStringAnyRecord(), "Wrong type!");
  mValue.mStringAnyRecord.Destroy();
  mType = eUninitialized;
}




void
OwningLongOrStringAnyRecord::Uninit()
{
  switch (mType) {
    case eUninitialized: {
      break;
    }
    case eLong: {
      DestroyLong();
      break;
    }
    case eStringAnyRecord: {
      DestroyStringAnyRecord();
      break;
    }
  }
}

bool
OwningLongOrStringAnyRecord::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eLong: {
      rval.setInt32(int32_t(mValue.mLong.Value()));
      return true;
      break;
    }
    case eStringAnyRecord: {

      JS::Rooted<JSObject*> returnObj(cx, JS_NewPlainObject(cx));
      if (!returnObj) {
        return false;
      }
      // Scope for 'tmp'
      {
        JS::Rooted<JS::Value> tmp(cx);
        for (auto& entry : mValue.mStringAnyRecord.Value().Entries()) {
          auto& recordValue0 = entry.mValue;
          // Control block to let us common up the JS_DefineUCProperty calls when there
          // are different ways to succeed at wrapping the value.
          do {
            JS::ExposeValueToActiveJS(recordValue0);
            tmp.set(recordValue0);
            if (!MaybeWrapValue(cx, &tmp)) {
              return false;
            }
            break;
          } while (false);
          if (!JS_DefineUCProperty(cx, returnObj,
                                   entry.mKey.BeginReading(),
                                   entry.mKey.Length(), tmp,
                                   JSPROP_ENUMERATE)) {
            return false;
          }
        }
      }
      rval.setObject(*returnObj);
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

void
OwningLongOrStringAnyRecord::TraceUnion(JSTracer* trc)
{
  switch (mType) {
    case eStringAnyRecord: {
      TraceRecord(trc, mValue.mStringAnyRecord.Value());
      break;
    }
    default: {
      break;
    }
  }
}

OwningNonNull<nsINode>&
OwningNodeOrString::RawSetAsNode()
{
  if (mType == eNode) {
    return mValue.mNode.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eNode;
  return mValue.mNode.SetValue();
}

OwningNonNull<nsINode>&
OwningNodeOrString::SetAsNode()
{
  if (mType == eNode) {
    return mValue.mNode.Value();
  }
  Uninit();
  mType = eNode;
  return mValue.mNode.SetValue();
}

bool
OwningNodeOrString::TrySetToNode(BindingCallContext& cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  tryNext = false;
  { // scope for memberSlot
    OwningNonNull<nsINode>& memberSlot = RawSetAsNode();
    static_assert(IsRefcounted<nsINode>::value, "We can only store refcounted classes.");
    {
      // Our JSContext should be in the right global to do unwrapping in.
      nsresult rv = UnwrapObject<prototypes::id::Node, nsINode>(value, memberSlot, cx);
      if (NS_FAILED(rv)) {
        DestroyNode();
        tryNext = true;
        return true;
      }
    }
  }
  return true;
}

bool
OwningNodeOrString::TrySetToNode(JSContext* cx_, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  BindingCallContext cx(cx_, nullptr);
  return TrySetToNode(cx, value, tryNext, passedToJSImpl);
}

void
OwningNodeOrString::DestroyNode()
{
  MOZ_ASSERT(IsNode(), "Wrong type!");
  mValue.mNode.Destroy();
  mType = eUninitialized;
}




nsString&
OwningNodeOrString::RawSetAsString()
{
  if (mType == eString) {
    return mValue.mString.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eString;
  return mValue.mString.SetValue();
}

nsString&
OwningNodeOrString::SetAsString()
{
  if (mType == eString) {
    return mValue.mString.Value();
  }
  Uninit();
  mType = eString;
  return mValue.mString.SetValue();
}

bool
OwningNodeOrString::TrySetToString(JSContext* cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  tryNext = false;
  { // scope for memberSlot
    nsString& memberSlot = RawSetAsString();
    if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
      return false;
    }
  }
  return true;
}


void
OwningNodeOrString::DestroyString()
{
  MOZ_ASSERT(IsString(), "Wrong type!");
  mValue.mString.Destroy();
  mType = eUninitialized;
}




void
OwningNodeOrString::Uninit()
{
  switch (mType) {
    case eUninitialized: {
      break;
    }
    case eNode: {
      DestroyNode();
      break;
    }
    case eString: {
      DestroyString();
      break;
    }
  }
}

bool
OwningNodeOrString::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eNode: {
      if (!GetOrCreateDOMReflector(cx, mValue.mNode.Value(), rval)) {
        MOZ_ASSERT(JS_IsExceptionPending(cx));
        return false;
      }
      return true;
      break;
    }
    case eString: {
      if (!xpc::NonVoidStringToJsval(cx, mValue.mString.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

void
OwningNodeOrString::TraceUnion(JSTracer* trc)
{
}

OwningNodeOrString&
OwningNodeOrString::operator=(const OwningNodeOrString& aOther)
{
  switch (aOther.mType) {
    case eUninitialized: {
      MOZ_ASSERT(mType == eUninitialized,
                 "We need to destroy ourselves?");
      break;
    }
    case eNode: {
      SetAsNode() = aOther.GetAsNode();
      break;
    }
    case eString: {
      SetAsString() = aOther.GetAsString();
      break;
    }
  }
  return *this;
}

JSObject*&
OwningObjectOrLong::RawSetAsObject()
{
  if (mType == eObject) {
    return mValue.mObject.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eObject;
  return mValue.mObject.SetValue();
}

JSObject*&
OwningObjectOrLong::SetAsObject()
{
  if (mType == eObject) {
    return mValue.mObject.Value();
  }
  Uninit();
  mType = eObject;
  return mValue.mObject.SetValue();
}


void
OwningObjectOrLong::DestroyObject()
{
  MOZ_ASSERT(IsObject(), "Wrong type!");
  mValue.mObject.Destroy();
  mType = eUninitialized;
}




int32_t&
OwningObjectOrLong::RawSetAsLong()
{
  if (mType == eLong) {
    return mValue.mLong.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eLong;
  return mValue.mLong.SetValue();
}

int32_t&
OwningObjectOrLong::SetAsLong()
{
  if (mType == eLong) {
    return mValue.mLong.Value();
  }
  Uninit();
  mType = eLong;
  return mValue.mLong.SetValue();
}

bool
OwningObjectOrLong::TrySetToLong(JSContext* cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
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

void
OwningObjectOrLong::DestroyLong()
{
  MOZ_ASSERT(IsLong(), "Wrong type!");
  mValue.mLong.Destroy();
  mType = eUninitialized;
}




void
OwningObjectOrLong::Uninit()
{
  switch (mType) {
    case eUninitialized: {
      break;
    }
    case eObject: {
      DestroyObject();
      break;
    }
    case eLong: {
      DestroyLong();
      break;
    }
  }
}

bool
OwningObjectOrLong::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eObject: {
      JS::ExposeObjectToActiveJS(mValue.mObject.Value());
      rval.setObject(*mValue.mObject.Value());
      if (!MaybeWrapObjectValue(cx, rval)) {
        return false;
      }
      return true;
      break;
    }
    case eLong: {
      rval.setInt32(int32_t(mValue.mLong.Value()));
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

void
OwningObjectOrLong::TraceUnion(JSTracer* trc)
{
  switch (mType) {
    case eObject: {
      JS::UnsafeTraceRoot(trc, &mValue.mObject.Value(), "mValue.mObject");
      break;
    }
    default: {
      break;
    }
  }
}

JSObject*&
OwningObjectOrNullOrLong::RawSetAsObject()
{
  if (mType == eObject) {
    return mValue.mObject.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eObject;
  return mValue.mObject.SetValue();
}

JSObject*&
OwningObjectOrNullOrLong::SetAsObject()
{
  if (mType == eObject) {
    return mValue.mObject.Value();
  }
  Uninit();
  mType = eObject;
  return mValue.mObject.SetValue();
}


void
OwningObjectOrNullOrLong::DestroyObject()
{
  MOZ_ASSERT(IsObject(), "Wrong type!");
  mValue.mObject.Destroy();
  mType = eUninitialized;
}




int32_t&
OwningObjectOrNullOrLong::RawSetAsLong()
{
  if (mType == eLong) {
    return mValue.mLong.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eLong;
  return mValue.mLong.SetValue();
}

int32_t&
OwningObjectOrNullOrLong::SetAsLong()
{
  if (mType == eLong) {
    return mValue.mLong.Value();
  }
  Uninit();
  mType = eLong;
  return mValue.mLong.SetValue();
}

bool
OwningObjectOrNullOrLong::TrySetToLong(JSContext* cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
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

void
OwningObjectOrNullOrLong::DestroyLong()
{
  MOZ_ASSERT(IsLong(), "Wrong type!");
  mValue.mLong.Destroy();
  mType = eUninitialized;
}




void
OwningObjectOrNullOrLong::Uninit()
{
  switch (mType) {
    case eUninitialized: {
      break;
    }
    case eNull: {
      break;
    }
    case eObject: {
      DestroyObject();
      break;
    }
    case eLong: {
      DestroyLong();
      break;
    }
  }
}

bool
OwningObjectOrNullOrLong::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eNull: {
      rval.setNull();
      return true;
      break;
    }
    case eObject: {
      JS::ExposeObjectToActiveJS(mValue.mObject.Value());
      rval.setObject(*mValue.mObject.Value());
      if (!MaybeWrapObjectValue(cx, rval)) {
        return false;
      }
      return true;
      break;
    }
    case eLong: {
      rval.setInt32(int32_t(mValue.mLong.Value()));
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

void
OwningObjectOrNullOrLong::TraceUnion(JSTracer* trc)
{
  switch (mType) {
    case eObject: {
      JS::UnsafeTraceRoot(trc, &mValue.mObject.Value(), "mValue.mObject");
      break;
    }
    default: {
      break;
    }
  }
}

nsString&
OwningStringOrArrayBuffer::RawSetAsString()
{
  if (mType == eString) {
    return mValue.mString.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eString;
  return mValue.mString.SetValue();
}

nsString&
OwningStringOrArrayBuffer::SetAsString()
{
  if (mType == eString) {
    return mValue.mString.Value();
  }
  Uninit();
  mType = eString;
  return mValue.mString.SetValue();
}

bool
OwningStringOrArrayBuffer::TrySetToString(JSContext* cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  tryNext = false;
  { // scope for memberSlot
    nsString& memberSlot = RawSetAsString();
    if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
      return false;
    }
  }
  return true;
}


void
OwningStringOrArrayBuffer::DestroyString()
{
  MOZ_ASSERT(IsString(), "Wrong type!");
  mValue.mString.Destroy();
  mType = eUninitialized;
}




ArrayBuffer&
OwningStringOrArrayBuffer::RawSetAsArrayBuffer()
{
  if (mType == eArrayBuffer) {
    return mValue.mArrayBuffer.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eArrayBuffer;
  return mValue.mArrayBuffer.SetValue();
}

ArrayBuffer&
OwningStringOrArrayBuffer::SetAsArrayBuffer()
{
  if (mType == eArrayBuffer) {
    return mValue.mArrayBuffer.Value();
  }
  Uninit();
  mType = eArrayBuffer;
  return mValue.mArrayBuffer.SetValue();
}

bool
OwningStringOrArrayBuffer::TrySetToArrayBuffer(BindingCallContext& cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  tryNext = false;
  { // scope for memberSlot
    ArrayBuffer& memberSlot = RawSetAsArrayBuffer();
    if (!memberSlot.Init(&value.toObject())) {
      DestroyArrayBuffer();
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

bool
OwningStringOrArrayBuffer::TrySetToArrayBuffer(JSContext* cx_, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  BindingCallContext cx(cx_, nullptr);
  return TrySetToArrayBuffer(cx, value, tryNext, passedToJSImpl);
}

void
OwningStringOrArrayBuffer::DestroyArrayBuffer()
{
  MOZ_ASSERT(IsArrayBuffer(), "Wrong type!");
  mValue.mArrayBuffer.Destroy();
  mType = eUninitialized;
}




void
OwningStringOrArrayBuffer::Uninit()
{
  switch (mType) {
    case eUninitialized: {
      break;
    }
    case eString: {
      DestroyString();
      break;
    }
    case eArrayBuffer: {
      DestroyArrayBuffer();
      break;
    }
  }
}

bool
OwningStringOrArrayBuffer::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eString: {
      if (!xpc::NonVoidStringToJsval(cx, mValue.mString.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    case eArrayBuffer: {
      rval.setObject(*mValue.mArrayBuffer.Value().Obj());
      if (!MaybeWrapNonDOMObjectValue(cx, rval)) {
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

void
OwningStringOrArrayBuffer::TraceUnion(JSTracer* trc)
{
  switch (mType) {
    case eArrayBuffer: {
      mValue.mArrayBuffer.Value().TraceSelf(trc);
      break;
    }
    default: {
      break;
    }
  }
}

nsString&
OwningStringOrMaybeSharedArrayBuffer::RawSetAsString()
{
  if (mType == eString) {
    return mValue.mString.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eString;
  return mValue.mString.SetValue();
}

nsString&
OwningStringOrMaybeSharedArrayBuffer::SetAsString()
{
  if (mType == eString) {
    return mValue.mString.Value();
  }
  Uninit();
  mType = eString;
  return mValue.mString.SetValue();
}

bool
OwningStringOrMaybeSharedArrayBuffer::TrySetToString(JSContext* cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  tryNext = false;
  { // scope for memberSlot
    nsString& memberSlot = RawSetAsString();
    if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
      return false;
    }
  }
  return true;
}


void
OwningStringOrMaybeSharedArrayBuffer::DestroyString()
{
  MOZ_ASSERT(IsString(), "Wrong type!");
  mValue.mString.Destroy();
  mType = eUninitialized;
}




ArrayBuffer&
OwningStringOrMaybeSharedArrayBuffer::RawSetAsArrayBuffer()
{
  if (mType == eArrayBuffer) {
    return mValue.mArrayBuffer.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eArrayBuffer;
  return mValue.mArrayBuffer.SetValue();
}

ArrayBuffer&
OwningStringOrMaybeSharedArrayBuffer::SetAsArrayBuffer()
{
  if (mType == eArrayBuffer) {
    return mValue.mArrayBuffer.Value();
  }
  Uninit();
  mType = eArrayBuffer;
  return mValue.mArrayBuffer.SetValue();
}

bool
OwningStringOrMaybeSharedArrayBuffer::TrySetToArrayBuffer(BindingCallContext& cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  tryNext = false;
  { // scope for memberSlot
    ArrayBuffer& memberSlot = RawSetAsArrayBuffer();
    if (!memberSlot.Init(&value.toObject())) {
      DestroyArrayBuffer();
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

bool
OwningStringOrMaybeSharedArrayBuffer::TrySetToArrayBuffer(JSContext* cx_, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  BindingCallContext cx(cx_, nullptr);
  return TrySetToArrayBuffer(cx, value, tryNext, passedToJSImpl);
}

void
OwningStringOrMaybeSharedArrayBuffer::DestroyArrayBuffer()
{
  MOZ_ASSERT(IsArrayBuffer(), "Wrong type!");
  mValue.mArrayBuffer.Destroy();
  mType = eUninitialized;
}




void
OwningStringOrMaybeSharedArrayBuffer::Uninit()
{
  switch (mType) {
    case eUninitialized: {
      break;
    }
    case eString: {
      DestroyString();
      break;
    }
    case eArrayBuffer: {
      DestroyArrayBuffer();
      break;
    }
  }
}

bool
OwningStringOrMaybeSharedArrayBuffer::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eString: {
      if (!xpc::NonVoidStringToJsval(cx, mValue.mString.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    case eArrayBuffer: {
      rval.setObject(*mValue.mArrayBuffer.Value().Obj());
      if (!MaybeWrapNonDOMObjectValue(cx, rval)) {
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

void
OwningStringOrMaybeSharedArrayBuffer::TraceUnion(JSTracer* trc)
{
  switch (mType) {
    case eArrayBuffer: {
      mValue.mArrayBuffer.Value().TraceSelf(trc);
      break;
    }
    default: {
      break;
    }
  }
}

nsString&
OwningStringOrObject::RawSetAsString()
{
  if (mType == eString) {
    return mValue.mString.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eString;
  return mValue.mString.SetValue();
}

nsString&
OwningStringOrObject::SetAsString()
{
  if (mType == eString) {
    return mValue.mString.Value();
  }
  Uninit();
  mType = eString;
  return mValue.mString.SetValue();
}

bool
OwningStringOrObject::TrySetToString(JSContext* cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  tryNext = false;
  { // scope for memberSlot
    nsString& memberSlot = RawSetAsString();
    if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
      return false;
    }
  }
  return true;
}


void
OwningStringOrObject::DestroyString()
{
  MOZ_ASSERT(IsString(), "Wrong type!");
  mValue.mString.Destroy();
  mType = eUninitialized;
}




JSObject*&
OwningStringOrObject::RawSetAsObject()
{
  if (mType == eObject) {
    return mValue.mObject.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eObject;
  return mValue.mObject.SetValue();
}

JSObject*&
OwningStringOrObject::SetAsObject()
{
  if (mType == eObject) {
    return mValue.mObject.Value();
  }
  Uninit();
  mType = eObject;
  return mValue.mObject.SetValue();
}


void
OwningStringOrObject::DestroyObject()
{
  MOZ_ASSERT(IsObject(), "Wrong type!");
  mValue.mObject.Destroy();
  mType = eUninitialized;
}




void
OwningStringOrObject::Uninit()
{
  switch (mType) {
    case eUninitialized: {
      break;
    }
    case eString: {
      DestroyString();
      break;
    }
    case eObject: {
      DestroyObject();
      break;
    }
  }
}

bool
OwningStringOrObject::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eString: {
      if (!xpc::NonVoidStringToJsval(cx, mValue.mString.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    case eObject: {
      JS::ExposeObjectToActiveJS(mValue.mObject.Value());
      rval.setObject(*mValue.mObject.Value());
      if (!MaybeWrapObjectValue(cx, rval)) {
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

void
OwningStringOrObject::TraceUnion(JSTracer* trc)
{
  switch (mType) {
    case eObject: {
      JS::UnsafeTraceRoot(trc, &mValue.mObject.Value(), "mValue.mObject");
      break;
    }
    default: {
      break;
    }
  }
}

nsString&
OwningStringOrStringSequence::RawSetAsString()
{
  if (mType == eString) {
    return mValue.mString.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eString;
  return mValue.mString.SetValue();
}

nsString&
OwningStringOrStringSequence::SetAsString()
{
  if (mType == eString) {
    return mValue.mString.Value();
  }
  Uninit();
  mType = eString;
  return mValue.mString.SetValue();
}

bool
OwningStringOrStringSequence::TrySetToString(JSContext* cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  tryNext = false;
  { // scope for memberSlot
    nsString& memberSlot = RawSetAsString();
    if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
      return false;
    }
  }
  return true;
}


void
OwningStringOrStringSequence::DestroyString()
{
  MOZ_ASSERT(IsString(), "Wrong type!");
  mValue.mString.Destroy();
  mType = eUninitialized;
}




Sequence<nsString>&
OwningStringOrStringSequence::RawSetAsStringSequence()
{
  if (mType == eStringSequence) {
    return mValue.mStringSequence.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eStringSequence;
  return mValue.mStringSequence.SetValue();
}

Sequence<nsString>&
OwningStringOrStringSequence::SetAsStringSequence()
{
  if (mType == eStringSequence) {
    return mValue.mStringSequence.Value();
  }
  Uninit();
  mType = eStringSequence;
  return mValue.mStringSequence.SetValue();
}

bool
OwningStringOrStringSequence::TrySetToStringSequence(BindingCallContext& cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  tryNext = false;
  { // scope for memberSlot
    Sequence<nsString>& memberSlot = RawSetAsStringSequence();
    JS::ForOfIterator iter(cx);
    if (!iter.init(value, JS::ForOfIterator::AllowNonIterable)) {
      return false;
    }
    if (!iter.valueIsIterable()) {
      DestroyStringSequence();
      tryNext = true;
      return true;
    }
    Sequence<nsString> &arr = memberSlot;
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

bool
OwningStringOrStringSequence::TrySetToStringSequence(JSContext* cx_, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  BindingCallContext cx(cx_, nullptr);
  return TrySetToStringSequence(cx, value, tryNext, passedToJSImpl);
}

void
OwningStringOrStringSequence::DestroyStringSequence()
{
  MOZ_ASSERT(IsStringSequence(), "Wrong type!");
  mValue.mStringSequence.Destroy();
  mType = eUninitialized;
}




void
OwningStringOrStringSequence::Uninit()
{
  switch (mType) {
    case eUninitialized: {
      break;
    }
    case eString: {
      DestroyString();
      break;
    }
    case eStringSequence: {
      DestroyStringSequence();
      break;
    }
  }
}

bool
OwningStringOrStringSequence::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eString: {
      if (!xpc::NonVoidStringToJsval(cx, mValue.mString.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    case eStringSequence: {

      uint32_t length = mValue.mStringSequence.Value().Length();
      JS::Rooted<JSObject*> returnArray(cx, JS::NewArrayObject(cx, length));
      if (!returnArray) {
        return false;
      }
      // Scope for 'tmp'
      {
        JS::Rooted<JS::Value> tmp(cx);
        for (uint32_t sequenceIdx0 = 0; sequenceIdx0 < length; ++sequenceIdx0) {
          // Control block to let us common up the JS_DefineElement calls when there
          // are different ways to succeed at wrapping the object.
          do {
            if (!xpc::NonVoidStringToJsval(cx, mValue.mStringSequence.Value()[sequenceIdx0], &tmp)) {
              return false;
            }
            break;
          } while (false);
          if (!JS_DefineElement(cx, returnArray, sequenceIdx0, tmp,
                                JSPROP_ENUMERATE)) {
            return false;
          }
        }
      }
      rval.setObject(*returnArray);
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

void
OwningStringOrStringSequence::TraceUnion(JSTracer* trc)
{
}

OwningStringOrStringSequence&
OwningStringOrStringSequence::operator=(const OwningStringOrStringSequence& aOther)
{
  switch (aOther.mType) {
    case eUninitialized: {
      MOZ_ASSERT(mType == eUninitialized,
                 "We need to destroy ourselves?");
      break;
    }
    case eString: {
      SetAsString() = aOther.GetAsString();
      break;
    }
    case eStringSequence: {
      SetAsStringSequence() = aOther.GetAsStringSequence();
      break;
    }
  }
  return *this;
}

SupportedType&
OwningSupportedTypeOrObject::RawSetAsSupportedType()
{
  if (mType == eSupportedType) {
    return mValue.mSupportedType.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eSupportedType;
  return mValue.mSupportedType.SetValue();
}

SupportedType&
OwningSupportedTypeOrObject::SetAsSupportedType()
{
  if (mType == eSupportedType) {
    return mValue.mSupportedType.Value();
  }
  Uninit();
  mType = eSupportedType;
  return mValue.mSupportedType.SetValue();
}

bool
OwningSupportedTypeOrObject::TrySetToSupportedType(BindingCallContext& cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
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

bool
OwningSupportedTypeOrObject::TrySetToSupportedType(JSContext* cx_, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  BindingCallContext cx(cx_, nullptr);
  return TrySetToSupportedType(cx, value, tryNext, passedToJSImpl);
}

void
OwningSupportedTypeOrObject::DestroySupportedType()
{
  MOZ_ASSERT(IsSupportedType(), "Wrong type!");
  mValue.mSupportedType.Destroy();
  mType = eUninitialized;
}




JSObject*&
OwningSupportedTypeOrObject::RawSetAsObject()
{
  if (mType == eObject) {
    return mValue.mObject.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eObject;
  return mValue.mObject.SetValue();
}

JSObject*&
OwningSupportedTypeOrObject::SetAsObject()
{
  if (mType == eObject) {
    return mValue.mObject.Value();
  }
  Uninit();
  mType = eObject;
  return mValue.mObject.SetValue();
}


void
OwningSupportedTypeOrObject::DestroyObject()
{
  MOZ_ASSERT(IsObject(), "Wrong type!");
  mValue.mObject.Destroy();
  mType = eUninitialized;
}




void
OwningSupportedTypeOrObject::Uninit()
{
  switch (mType) {
    case eUninitialized: {
      break;
    }
    case eSupportedType: {
      DestroySupportedType();
      break;
    }
    case eObject: {
      DestroyObject();
      break;
    }
  }
}

bool
OwningSupportedTypeOrObject::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eSupportedType: {
      if (!ToJSValue(cx, mValue.mSupportedType.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    case eObject: {
      JS::ExposeObjectToActiveJS(mValue.mObject.Value());
      rval.setObject(*mValue.mObject.Value());
      if (!MaybeWrapObjectValue(cx, rval)) {
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

void
OwningSupportedTypeOrObject::TraceUnion(JSTracer* trc)
{
  switch (mType) {
    case eObject: {
      JS::UnsafeTraceRoot(trc, &mValue.mObject.Value(), "mValue.mObject");
      break;
    }
    default: {
      break;
    }
  }
}

nsCString&
OwningUTF8StringOrLong::RawSetAsUTF8String()
{
  if (mType == eUTF8String) {
    return mValue.mUTF8String.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eUTF8String;
  return mValue.mUTF8String.SetValue();
}

nsCString&
OwningUTF8StringOrLong::SetAsUTF8String()
{
  if (mType == eUTF8String) {
    return mValue.mUTF8String.Value();
  }
  Uninit();
  mType = eUTF8String;
  return mValue.mUTF8String.SetValue();
}

bool
OwningUTF8StringOrLong::TrySetToUTF8String(JSContext* cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  tryNext = false;
  { // scope for memberSlot
    nsCString& memberSlot = RawSetAsUTF8String();
    if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
      return false;
    }
  }
  return true;
}


void
OwningUTF8StringOrLong::DestroyUTF8String()
{
  MOZ_ASSERT(IsUTF8String(), "Wrong type!");
  mValue.mUTF8String.Destroy();
  mType = eUninitialized;
}




int32_t&
OwningUTF8StringOrLong::RawSetAsLong()
{
  if (mType == eLong) {
    return mValue.mLong.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eLong;
  return mValue.mLong.SetValue();
}

int32_t&
OwningUTF8StringOrLong::SetAsLong()
{
  if (mType == eLong) {
    return mValue.mLong.Value();
  }
  Uninit();
  mType = eLong;
  return mValue.mLong.SetValue();
}

bool
OwningUTF8StringOrLong::TrySetToLong(JSContext* cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
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

void
OwningUTF8StringOrLong::DestroyLong()
{
  MOZ_ASSERT(IsLong(), "Wrong type!");
  mValue.mLong.Destroy();
  mType = eUninitialized;
}




void
OwningUTF8StringOrLong::Uninit()
{
  switch (mType) {
    case eUninitialized: {
      break;
    }
    case eUTF8String: {
      DestroyUTF8String();
      break;
    }
    case eLong: {
      DestroyLong();
      break;
    }
  }
}

bool
OwningUTF8StringOrLong::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eUTF8String: {
      if (!NonVoidUTF8StringToJsval(cx, mValue.mUTF8String.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    case eLong: {
      rval.setInt32(int32_t(mValue.mLong.Value()));
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

void
OwningUTF8StringOrLong::TraceUnion(JSTracer* trc)
{
}

OwningUTF8StringOrLong&
OwningUTF8StringOrLong::operator=(const OwningUTF8StringOrLong& aOther)
{
  switch (aOther.mType) {
    case eUninitialized: {
      MOZ_ASSERT(mType == eUninitialized,
                 "We need to destroy ourselves?");
      break;
    }
    case eUTF8String: {
      SetAsUTF8String() = aOther.GetAsUTF8String();
      break;
    }
    case eLong: {
      SetAsLong() = aOther.GetAsLong();
      break;
    }
  }
  return *this;
}

nsCString&
OwningUTF8StringOrUTF8StringSequence::RawSetAsUTF8String()
{
  if (mType == eUTF8String) {
    return mValue.mUTF8String.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eUTF8String;
  return mValue.mUTF8String.SetValue();
}

nsCString&
OwningUTF8StringOrUTF8StringSequence::SetAsUTF8String()
{
  if (mType == eUTF8String) {
    return mValue.mUTF8String.Value();
  }
  Uninit();
  mType = eUTF8String;
  return mValue.mUTF8String.SetValue();
}

bool
OwningUTF8StringOrUTF8StringSequence::TrySetToUTF8String(JSContext* cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  tryNext = false;
  { // scope for memberSlot
    nsCString& memberSlot = RawSetAsUTF8String();
    if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
      return false;
    }
  }
  return true;
}


void
OwningUTF8StringOrUTF8StringSequence::DestroyUTF8String()
{
  MOZ_ASSERT(IsUTF8String(), "Wrong type!");
  mValue.mUTF8String.Destroy();
  mType = eUninitialized;
}




Sequence<nsCString>&
OwningUTF8StringOrUTF8StringSequence::RawSetAsUTF8StringSequence()
{
  if (mType == eUTF8StringSequence) {
    return mValue.mUTF8StringSequence.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eUTF8StringSequence;
  return mValue.mUTF8StringSequence.SetValue();
}

Sequence<nsCString>&
OwningUTF8StringOrUTF8StringSequence::SetAsUTF8StringSequence()
{
  if (mType == eUTF8StringSequence) {
    return mValue.mUTF8StringSequence.Value();
  }
  Uninit();
  mType = eUTF8StringSequence;
  return mValue.mUTF8StringSequence.SetValue();
}

bool
OwningUTF8StringOrUTF8StringSequence::TrySetToUTF8StringSequence(BindingCallContext& cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  tryNext = false;
  { // scope for memberSlot
    Sequence<nsCString>& memberSlot = RawSetAsUTF8StringSequence();
    JS::ForOfIterator iter(cx);
    if (!iter.init(value, JS::ForOfIterator::AllowNonIterable)) {
      return false;
    }
    if (!iter.valueIsIterable()) {
      DestroyUTF8StringSequence();
      tryNext = true;
      return true;
    }
    Sequence<nsCString> &arr = memberSlot;
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

bool
OwningUTF8StringOrUTF8StringSequence::TrySetToUTF8StringSequence(JSContext* cx_, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  BindingCallContext cx(cx_, nullptr);
  return TrySetToUTF8StringSequence(cx, value, tryNext, passedToJSImpl);
}

void
OwningUTF8StringOrUTF8StringSequence::DestroyUTF8StringSequence()
{
  MOZ_ASSERT(IsUTF8StringSequence(), "Wrong type!");
  mValue.mUTF8StringSequence.Destroy();
  mType = eUninitialized;
}




void
OwningUTF8StringOrUTF8StringSequence::Uninit()
{
  switch (mType) {
    case eUninitialized: {
      break;
    }
    case eUTF8String: {
      DestroyUTF8String();
      break;
    }
    case eUTF8StringSequence: {
      DestroyUTF8StringSequence();
      break;
    }
  }
}

bool
OwningUTF8StringOrUTF8StringSequence::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eUTF8String: {
      if (!NonVoidUTF8StringToJsval(cx, mValue.mUTF8String.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    case eUTF8StringSequence: {

      uint32_t length = mValue.mUTF8StringSequence.Value().Length();
      JS::Rooted<JSObject*> returnArray(cx, JS::NewArrayObject(cx, length));
      if (!returnArray) {
        return false;
      }
      // Scope for 'tmp'
      {
        JS::Rooted<JS::Value> tmp(cx);
        for (uint32_t sequenceIdx0 = 0; sequenceIdx0 < length; ++sequenceIdx0) {
          // Control block to let us common up the JS_DefineElement calls when there
          // are different ways to succeed at wrapping the object.
          do {
            if (!NonVoidUTF8StringToJsval(cx, mValue.mUTF8StringSequence.Value()[sequenceIdx0], &tmp)) {
              return false;
            }
            break;
          } while (false);
          if (!JS_DefineElement(cx, returnArray, sequenceIdx0, tmp,
                                JSPROP_ENUMERATE)) {
            return false;
          }
        }
      }
      rval.setObject(*returnArray);
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

void
OwningUTF8StringOrUTF8StringSequence::TraceUnion(JSTracer* trc)
{
}

OwningUTF8StringOrUTF8StringSequence&
OwningUTF8StringOrUTF8StringSequence::operator=(const OwningUTF8StringOrUTF8StringSequence& aOther)
{
  switch (aOther.mType) {
    case eUninitialized: {
      MOZ_ASSERT(mType == eUninitialized,
                 "We need to destroy ourselves?");
      break;
    }
    case eUTF8String: {
      SetAsUTF8String() = aOther.GetAsUTF8String();
      break;
    }
    case eUTF8StringSequence: {
      SetAsUTF8StringSequence() = aOther.GetAsUTF8StringSequence();
      break;
    }
  }
  return *this;
}

double&
OwningUnrestrictedDoubleOrString::RawSetAsUnrestrictedDouble()
{
  if (mType == eUnrestrictedDouble) {
    return mValue.mUnrestrictedDouble.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eUnrestrictedDouble;
  return mValue.mUnrestrictedDouble.SetValue();
}

double&
OwningUnrestrictedDoubleOrString::SetAsUnrestrictedDouble()
{
  if (mType == eUnrestrictedDouble) {
    return mValue.mUnrestrictedDouble.Value();
  }
  Uninit();
  mType = eUnrestrictedDouble;
  return mValue.mUnrestrictedDouble.SetValue();
}

bool
OwningUnrestrictedDoubleOrString::TrySetToUnrestrictedDouble(JSContext* cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
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

void
OwningUnrestrictedDoubleOrString::DestroyUnrestrictedDouble()
{
  MOZ_ASSERT(IsUnrestrictedDouble(), "Wrong type!");
  mValue.mUnrestrictedDouble.Destroy();
  mType = eUninitialized;
}




nsString&
OwningUnrestrictedDoubleOrString::RawSetAsString()
{
  if (mType == eString) {
    return mValue.mString.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eString;
  return mValue.mString.SetValue();
}

nsString&
OwningUnrestrictedDoubleOrString::SetAsString()
{
  if (mType == eString) {
    return mValue.mString.Value();
  }
  Uninit();
  mType = eString;
  return mValue.mString.SetValue();
}

bool
OwningUnrestrictedDoubleOrString::TrySetToString(JSContext* cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  tryNext = false;
  { // scope for memberSlot
    nsString& memberSlot = RawSetAsString();
    if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
      return false;
    }
  }
  return true;
}


void
OwningUnrestrictedDoubleOrString::DestroyString()
{
  MOZ_ASSERT(IsString(), "Wrong type!");
  mValue.mString.Destroy();
  mType = eUninitialized;
}




void
OwningUnrestrictedDoubleOrString::Uninit()
{
  switch (mType) {
    case eUninitialized: {
      break;
    }
    case eUnrestrictedDouble: {
      DestroyUnrestrictedDouble();
      break;
    }
    case eString: {
      DestroyString();
      break;
    }
  }
}

bool
OwningUnrestrictedDoubleOrString::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eUnrestrictedDouble: {
      rval.set(JS_NumberValue(double(mValue.mUnrestrictedDouble.Value())));
      return true;
      break;
    }
    case eString: {
      if (!xpc::NonVoidStringToJsval(cx, mValue.mString.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

void
OwningUnrestrictedDoubleOrString::TraceUnion(JSTracer* trc)
{
}

OwningUnrestrictedDoubleOrString&
OwningUnrestrictedDoubleOrString::operator=(const OwningUnrestrictedDoubleOrString& aOther)
{
  switch (aOther.mType) {
    case eUninitialized: {
      MOZ_ASSERT(mType == eUninitialized,
                 "We need to destroy ourselves?");
      break;
    }
    case eUnrestrictedDouble: {
      SetAsUnrestrictedDouble() = aOther.GetAsUnrestrictedDouble();
      break;
    }
    case eString: {
      SetAsString() = aOther.GetAsString();
      break;
    }
  }
  return *this;
}

float&
OwningUnrestrictedFloatOrString::RawSetAsUnrestrictedFloat()
{
  if (mType == eUnrestrictedFloat) {
    return mValue.mUnrestrictedFloat.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eUnrestrictedFloat;
  return mValue.mUnrestrictedFloat.SetValue();
}

float&
OwningUnrestrictedFloatOrString::SetAsUnrestrictedFloat()
{
  if (mType == eUnrestrictedFloat) {
    return mValue.mUnrestrictedFloat.Value();
  }
  Uninit();
  mType = eUnrestrictedFloat;
  return mValue.mUnrestrictedFloat.SetValue();
}

bool
OwningUnrestrictedFloatOrString::TrySetToUnrestrictedFloat(JSContext* cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
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

void
OwningUnrestrictedFloatOrString::DestroyUnrestrictedFloat()
{
  MOZ_ASSERT(IsUnrestrictedFloat(), "Wrong type!");
  mValue.mUnrestrictedFloat.Destroy();
  mType = eUninitialized;
}




nsString&
OwningUnrestrictedFloatOrString::RawSetAsString()
{
  if (mType == eString) {
    return mValue.mString.Value();
  }
  MOZ_ASSERT(mType == eUninitialized);
  mType = eString;
  return mValue.mString.SetValue();
}

nsString&
OwningUnrestrictedFloatOrString::SetAsString()
{
  if (mType == eString) {
    return mValue.mString.Value();
  }
  Uninit();
  mType = eString;
  return mValue.mString.SetValue();
}

bool
OwningUnrestrictedFloatOrString::TrySetToString(JSContext* cx, JS::Handle<JS::Value> value, bool& tryNext, bool passedToJSImpl)
{
  tryNext = false;
  { // scope for memberSlot
    nsString& memberSlot = RawSetAsString();
    if (!ConvertJSValueToString(cx, value, eStringify, eStringify, memberSlot)) {
      return false;
    }
  }
  return true;
}


void
OwningUnrestrictedFloatOrString::DestroyString()
{
  MOZ_ASSERT(IsString(), "Wrong type!");
  mValue.mString.Destroy();
  mType = eUninitialized;
}




void
OwningUnrestrictedFloatOrString::Uninit()
{
  switch (mType) {
    case eUninitialized: {
      break;
    }
    case eUnrestrictedFloat: {
      DestroyUnrestrictedFloat();
      break;
    }
    case eString: {
      DestroyString();
      break;
    }
  }
}

bool
OwningUnrestrictedFloatOrString::ToJSVal(JSContext* cx, JS::Handle<JSObject*> scopeObj, JS::MutableHandle<JS::Value> rval) const
{
  switch (mType) {
    case eUninitialized: {
      return false;
      break;
    }
    case eUnrestrictedFloat: {
      rval.set(JS_NumberValue(double(mValue.mUnrestrictedFloat.Value())));
      return true;
      break;
    }
    case eString: {
      if (!xpc::NonVoidStringToJsval(cx, mValue.mString.Value(), rval)) {
        return false;
      }
      return true;
      break;
    }
    default: {
      return false;
      break;
    }
  }

  return false;
}

void
OwningUnrestrictedFloatOrString::TraceUnion(JSTracer* trc)
{
}

OwningUnrestrictedFloatOrString&
OwningUnrestrictedFloatOrString::operator=(const OwningUnrestrictedFloatOrString& aOther)
{
  switch (aOther.mType) {
    case eUninitialized: {
      MOZ_ASSERT(mType == eUninitialized,
                 "We need to destroy ourselves?");
      break;
    }
    case eUnrestrictedFloat: {
      SetAsUnrestrictedFloat() = aOther.GetAsUnrestrictedFloat();
      break;
    }
    case eString: {
      SetAsString() = aOther.GetAsString();
      break;
    }
  }
  return *this;
}
} // namespace dom
} // namespace mozilla

