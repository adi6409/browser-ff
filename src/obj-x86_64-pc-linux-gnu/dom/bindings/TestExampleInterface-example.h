/* -*- Mode: C++; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* vim:set ts=2 sw=2 sts=2 et cindent: */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef mozilla_dom_TestExampleInterface_h
#define mozilla_dom_TestExampleInterface_h

#include "js/TypeDecls.h"
#include "mozilla/Attributes.h"
#include "mozilla/ErrorResult.h"
#include "mozilla/dom/BindingDeclarations.h"
#include "nsCycleCollectionParticipant.h"
#include "nsWrapperCache.h"

class nsRenamedInterface;

namespace mozilla {
namespace dom {

class ArrayBufferOrLong;
class ByteStringOrLong;
class CanvasPatternOrCanvasGradient;
class CanvasPatternOrNullOrCanvasGradient;
struct Dict;
struct DictContainingDict;
struct DictContainingSequence;
class DocGroup;
class DoubleOrByteString;
class DoubleOrString;
class DoubleOrSupportedType;
class DoubleOrUSVString;
class DoubleOrUTF8String;
class FloatOrString;
class GlobalObject;
struct GrandparentDict;
class ObjectOrLong;
class ObjectOrNullOrLong;
class OwningArrayBufferOrLong;
class OwningByteStringOrLong;
class OwningCanvasPatternOrCanvasGradient;
class OwningCanvasPatternOrNullOrCanvasGradient;
class OwningDoubleOrByteString;
class OwningDoubleOrString;
class OwningDoubleOrSupportedType;
class OwningDoubleOrUSVString;
class OwningDoubleOrUTF8String;
class OwningFloatOrString;
class OwningObjectOrLong;
class OwningObjectOrNullOrLong;
class OwningStringOrArrayBuffer;
class OwningStringOrMaybeSharedArrayBuffer;
class OwningStringOrObject;
class OwningSupportedTypeOrObject;
class OwningUTF8StringOrLong;
class OwningUnrestrictedDoubleOrString;
class OwningUnrestrictedFloatOrString;
class Promise;
class StringOrArrayBuffer;
class StringOrMaybeSharedArrayBuffer;
class StringOrObject;
class SupportedTypeOrObject;
class TestCallback;
class TestCallbackInterface;
class TestExampleInterface;
class TestExternalInterface;
class TestInterface;
class TestNonWrapperCacheInterface;
class TestParentInterface;
class TestTreatAsNullCallback;
class UTF8StringOrLong;
class UnrestrictedDoubleOrString;
class UnrestrictedFloatOrString;

} // namespace dom
} // namespace mozilla

namespace mozilla {
namespace dom {

class TestExampleInterface final : public nsISupports /* or NonRefcountedDOMObject if this is a non-refcounted object */,
                                   public nsWrapperCache /* Change wrapperCache in the binding configuration if you don't want this */
{
public:
  NS_DECL_CYCLE_COLLECTING_ISUPPORTS
  NS_DECL_CYCLE_COLLECTION_SCRIPT_HOLDER_CLASS(TestExampleInterface)

public:
  TestExampleInterface();

protected:
  ~TestExampleInterface();

public:
  // This should return something that eventually allows finding a
  // path to the global this object is associated with.  Most simply,
  // returning an actual global works.
  nsIGlobalObject* GetParentObject() const;

  DocGroup* GetDocGroup() const;

  JSObject* WrapObject(JSContext* aCx, JS::Handle<JSObject*> aGivenProto) override;

  static already_AddRefed<TestExampleInterface> Constructor(const GlobalObject& global);
  static already_AddRefed<TestExampleInterface> Constructor(const GlobalObject& global, const nsAString& str);
  static already_AddRefed<TestExampleInterface> Constructor(const GlobalObject& global, uint32_t num, const Nullable<bool>& boolArg);
  static already_AddRefed<TestExampleInterface> Constructor(const GlobalObject& global, TestInterface* iface);
  static already_AddRefed<TestExampleInterface> Constructor(const GlobalObject& global, uint32_t arg1, TestInterface& iface);
  static already_AddRefed<TestExampleInterface> Constructor(const GlobalObject& global, const ArrayBuffer& arrayBuf);
  static already_AddRefed<TestExampleInterface> Constructor(const GlobalObject& global, const Uint8Array& typedArr);

  static already_AddRefed<TestExampleInterface> Example(const GlobalObject& global, ErrorResult& aRv);
  static already_AddRefed<TestExampleInterface> Example(const GlobalObject& global, const nsAString& str, ErrorResult& aRv);

  static already_AddRefed<TestExampleInterface> Example2(const GlobalObject& global, const DictForConstructor& dict, JS::Handle<JS::Value> any1, JS::Handle<JSObject*> obj1, JS::Handle<JSObject*> obj2, const Sequence<Dict>& seq, JS::Handle<JS::Value> any2, const Optional<JS::Handle<JSObject*>>& obj3, const Optional<JS::Handle<JSObject*>>& obj4, ErrorResult& aRv);
  static already_AddRefed<TestExampleInterface> Example2(const GlobalObject& global, const LongOrStringAnyRecord& arg1, ErrorResult& aRv);

  int8_t ReadonlyByte() const;

  int8_t WritableByte() const;

  void SetWritableByte(int8_t arg);

  void PassByte(int8_t arg);

  int8_t ReceiveByte();

  void PassOptionalByte(const Optional<int8_t>& arg);

  void PassOptionalByteBeforeRequired(const Optional<int8_t>& arg1, int8_t arg2);

  void PassOptionalByteWithDefault(int8_t arg);

  void PassOptionalByteWithDefaultBeforeRequired(int8_t arg1, int8_t arg2);

  void PassNullableByte(const Nullable<int8_t>& arg);

  void PassOptionalNullableByte(const Optional<Nullable<int8_t>>& arg);

  void PassVariadicByte(const Sequence<int8_t>& arg);

  int8_t CachedByte() const;

  int8_t CachedConstantByte() const;

  int8_t CachedWritableByte() const;

  void SetCachedWritableByte(int8_t arg);

  int8_t SideEffectFreeByte() const;

  void SetSideEffectFreeByte(int8_t arg);

  int8_t DomDependentByte() const;

  void SetDomDependentByte(int8_t arg);

  int8_t ConstantByte() const;

  int8_t DeviceStateDependentByte() const;

  int8_t ReturnByteSideEffectFree();

  int8_t ReturnDOMDependentByte();

  int8_t ReturnConstantByte();

  int8_t ReturnDeviceStateDependentByte();

  int16_t ReadonlyShort() const;

  int16_t WritableShort() const;

  void SetWritableShort(int16_t arg);

  void PassShort(int16_t arg);

  int16_t ReceiveShort();

  void PassOptionalShort(const Optional<int16_t>& arg);

  void PassOptionalShortWithDefault(int16_t arg);

  int32_t ReadonlyLong() const;

  int32_t WritableLong() const;

  void SetWritableLong(int32_t arg);

  void PassLong(int32_t arg);

  int32_t ReceiveLong();

  void PassOptionalLong(const Optional<int32_t>& arg);

  void PassOptionalLongWithDefault(int32_t arg);

  int64_t ReadonlyLongLong() const;

  int64_t WritableLongLong() const;

  void SetWritableLongLong(int64_t arg);

  void PassLongLong(int64_t arg);

  int64_t ReceiveLongLong();

  void PassOptionalLongLong(const Optional<int64_t>& arg);

  void PassOptionalLongLongWithDefault(int64_t arg);

  uint8_t ReadonlyOctet() const;

  uint8_t WritableOctet() const;

  void SetWritableOctet(uint8_t arg);

  void PassOctet(uint8_t arg);

  uint8_t ReceiveOctet();

  void PassOptionalOctet(const Optional<uint8_t>& arg);

  void PassOptionalOctetWithDefault(uint8_t arg);

  uint16_t ReadonlyUnsignedShort() const;

  uint16_t WritableUnsignedShort() const;

  void SetWritableUnsignedShort(uint16_t arg);

  void PassUnsignedShort(uint16_t arg);

  uint16_t ReceiveUnsignedShort();

  void PassOptionalUnsignedShort(const Optional<uint16_t>& arg);

  void PassOptionalUnsignedShortWithDefault(uint16_t arg);

  uint32_t ReadonlyUnsignedLong() const;

  uint32_t WritableUnsignedLong() const;

  void SetWritableUnsignedLong(uint32_t arg);

  void PassUnsignedLong(uint32_t arg);

  uint32_t ReceiveUnsignedLong();

  void PassOptionalUnsignedLong(const Optional<uint32_t>& arg);

  void PassOptionalUnsignedLongWithDefault(uint32_t arg);

  uint64_t ReadonlyUnsignedLongLong() const;

  uint64_t WritableUnsignedLongLong() const;

  void SetWritableUnsignedLongLong(uint64_t arg);

  void PassUnsignedLongLong(uint64_t arg);

  uint64_t ReceiveUnsignedLongLong();

  void PassOptionalUnsignedLongLong(const Optional<uint64_t>& arg);

  void PassOptionalUnsignedLongLongWithDefault(uint64_t arg);

  float WritableFloat() const;

  void SetWritableFloat(float arg);

  float WritableUnrestrictedFloat() const;

  void SetWritableUnrestrictedFloat(float arg);

  Nullable<float> GetWritableNullableFloat() const;

  void SetWritableNullableFloat(const Nullable<float>& arg);

  Nullable<float> GetWritableNullableUnrestrictedFloat() const;

  void SetWritableNullableUnrestrictedFloat(const Nullable<float>& arg);

  double WritableDouble() const;

  void SetWritableDouble(double arg);

  double WritableUnrestrictedDouble() const;

  void SetWritableUnrestrictedDouble(double arg);

  Nullable<double> GetWritableNullableDouble() const;

  void SetWritableNullableDouble(const Nullable<double>& arg);

  Nullable<double> GetWritableNullableUnrestrictedDouble() const;

  void SetWritableNullableUnrestrictedDouble(const Nullable<double>& arg);

  void PassFloat(float arg1, float arg2, const Nullable<float>& arg3, const Nullable<float>& arg4, double arg5, double arg6, const Nullable<double>& arg7, const Nullable<double>& arg8, const Sequence<float>& arg9, const Sequence<float>& arg10, const Sequence<Nullable<float>>& arg11, const Sequence<Nullable<float>>& arg12, const Sequence<double>& arg13, const Sequence<double>& arg14, const Sequence<Nullable<double>>& arg15, const Sequence<Nullable<double>>& arg16);

  void PassLenientFloat(float arg1, float arg2, const Nullable<float>& arg3, const Nullable<float>& arg4, double arg5, double arg6, const Nullable<double>& arg7, const Nullable<double>& arg8, const Sequence<float>& arg9, const Sequence<float>& arg10, const Sequence<Nullable<float>>& arg11, const Sequence<Nullable<float>>& arg12, const Sequence<double>& arg13, const Sequence<double>& arg14, const Sequence<Nullable<double>>& arg15, const Sequence<Nullable<double>>& arg16);

  float LenientFloatAttr() const;

  void SetLenientFloatAttr(float arg);

  double LenientDoubleAttr() const;

  void SetLenientDoubleAttr(double arg);

  // Return a raw pointer here to avoid refcounting, but make sure it's safe (the object should be kept alive by the callee).
  already_AddRefed<TestInterface> ReceiveSelf();

  // Return a raw pointer here to avoid refcounting, but make sure it's safe (the object should be kept alive by the callee).
  already_AddRefed<TestInterface> ReceiveNullableSelf();

  // Return a raw pointer here to avoid refcounting, but make sure it's safe (the object should be kept alive by the callee).
  already_AddRefed<TestInterface> ReceiveWeakSelf();

  // Return a raw pointer here to avoid refcounting, but make sure it's safe (the object should be kept alive by the callee).
  already_AddRefed<TestInterface> ReceiveWeakNullableSelf();

  void PassSelf(TestInterface& arg);

  void PassNullableSelf(TestInterface* arg);

  // Return a raw pointer here to avoid refcounting, but make sure it's safe (the object should be kept alive by the callee).
  already_AddRefed<TestInterface> NonNullSelf() const;

  void SetNonNullSelf(TestInterface& arg);

  // Return a raw pointer here to avoid refcounting, but make sure it's safe (the object should be kept alive by the callee).
  already_AddRefed<TestInterface> GetNullableSelf() const;

  void SetNullableSelf(TestInterface* arg);

  // Return a raw pointer here to avoid refcounting, but make sure it's safe (the object should be kept alive by the callee).
  already_AddRefed<TestInterface> CachedSelf() const;

  void PassOptionalSelf(const Optional<TestInterface*>& arg);

  void PassOptionalNonNullSelf(const Optional<NonNull<TestInterface>>& arg);

  void PassOptionalSelfWithDefault(TestInterface* arg);

  already_AddRefed<TestNonWrapperCacheInterface> ReceiveNonWrapperCacheInterface();

  already_AddRefed<TestNonWrapperCacheInterface> ReceiveNullableNonWrapperCacheInterface();

  void ReceiveNonWrapperCacheInterfaceSequence(nsTArray<RefPtr<TestNonWrapperCacheInterface>>& aRetVal);

  void ReceiveNullableNonWrapperCacheInterfaceSequence(nsTArray<RefPtr<TestNonWrapperCacheInterface>>& aRetVal);

  void ReceiveNonWrapperCacheInterfaceNullableSequence(Nullable<nsTArray<RefPtr<TestNonWrapperCacheInterface>>>& aRetVal);

  void ReceiveNullableNonWrapperCacheInterfaceNullableSequence(Nullable<nsTArray<RefPtr<TestNonWrapperCacheInterface>>>& aRetVal);

  // Return a raw pointer here to avoid refcounting, but make sure it's safe (the object should be kept alive by the callee).
  already_AddRefed<TestExternalInterface> ReceiveExternal();

  // Return a raw pointer here to avoid refcounting, but make sure it's safe (the object should be kept alive by the callee).
  already_AddRefed<TestExternalInterface> ReceiveNullableExternal();

  // Return a raw pointer here to avoid refcounting, but make sure it's safe (the object should be kept alive by the callee).
  already_AddRefed<TestExternalInterface> ReceiveWeakExternal();

  // Return a raw pointer here to avoid refcounting, but make sure it's safe (the object should be kept alive by the callee).
  already_AddRefed<TestExternalInterface> ReceiveWeakNullableExternal();

  void PassExternal(TestExternalInterface* arg);

  void PassNullableExternal(TestExternalInterface* arg);

  // Return a raw pointer here to avoid refcounting, but make sure it's safe (the object should be kept alive by the callee).
  already_AddRefed<TestExternalInterface> NonNullExternal() const;

  void SetNonNullExternal(TestExternalInterface* arg);

  // Return a raw pointer here to avoid refcounting, but make sure it's safe (the object should be kept alive by the callee).
  already_AddRefed<TestExternalInterface> GetNullableExternal() const;

  void SetNullableExternal(TestExternalInterface* arg);

  void PassOptionalExternal(const Optional<TestExternalInterface*>& arg);

  void PassOptionalNonNullExternal(const Optional<TestExternalInterface*>& arg);

  void PassOptionalExternalWithDefault(TestExternalInterface* arg);

  // Return a raw pointer here to avoid refcounting, but make sure it's safe (the object should be kept alive by the callee).
  already_AddRefed<TestCallbackInterface> ReceiveCallbackInterface();

  // Return a raw pointer here to avoid refcounting, but make sure it's safe (the object should be kept alive by the callee).
  already_AddRefed<TestCallbackInterface> ReceiveNullableCallbackInterface();

  // Return a raw pointer here to avoid refcounting, but make sure it's safe (the object should be kept alive by the callee).
  already_AddRefed<TestCallbackInterface> ReceiveWeakCallbackInterface();

  // Return a raw pointer here to avoid refcounting, but make sure it's safe (the object should be kept alive by the callee).
  already_AddRefed<TestCallbackInterface> ReceiveWeakNullableCallbackInterface();

  void PassCallbackInterface(TestCallbackInterface& arg);

  void PassNullableCallbackInterface(TestCallbackInterface* arg);

  // Return a raw pointer here to avoid refcounting, but make sure it's safe (the object should be kept alive by the callee).
  already_AddRefed<TestCallbackInterface> NonNullCallbackInterface() const;

  void SetNonNullCallbackInterface(TestCallbackInterface& arg);

  // Return a raw pointer here to avoid refcounting, but make sure it's safe (the object should be kept alive by the callee).
  already_AddRefed<TestCallbackInterface> GetNullableCallbackInterface() const;

  void SetNullableCallbackInterface(TestCallbackInterface* arg);

  void PassOptionalCallbackInterface(const Optional<RefPtr<TestCallbackInterface>>& arg);

  void PassOptionalNonNullCallbackInterface(const Optional<OwningNonNull<TestCallbackInterface>>& arg);

  void PassOptionalCallbackInterfaceWithDefault(TestCallbackInterface* arg);

  void GetReadonlySequence(nsTArray<int32_t>& aRetVal) const;

  void GetReadonlySequenceOfDictionaries(JSContext* cx, nsTArray<Dict>& aRetVal) const;

  void GetReadonlyNullableSequenceOfDictionaries(JSContext* cx, Nullable<nsTArray<Dict>>& aRetVal) const;

  void GetReadonlyFrozenSequence(nsTArray<int32_t>& aRetVal) const;

  void GetReadonlyFrozenNullableSequence(Nullable<nsTArray<int32_t>>& aRetVal) const;

  void ReceiveSequence(nsTArray<int32_t>& aRetVal);

  void ReceiveNullableSequence(Nullable<nsTArray<int32_t>>& aRetVal);

  void ReceiveSequenceOfNullableInts(nsTArray<Nullable<int32_t>>& aRetVal);

  void ReceiveNullableSequenceOfNullableInts(Nullable<nsTArray<Nullable<int32_t>>>& aRetVal);

  void PassSequence(const Sequence<int32_t>& arg);

  void PassNullableSequence(const Nullable<Sequence<int32_t>>& arg);

  void PassSequenceOfNullableInts(const Sequence<Nullable<int32_t>>& arg);

  void PassOptionalSequenceOfNullableInts(const Optional<Sequence<Nullable<int32_t>>>& arg);

  void PassOptionalNullableSequenceOfNullableInts(const Optional<Nullable<Sequence<Nullable<int32_t>>>>& arg);

  void ReceiveCastableObjectSequence(nsTArray<RefPtr<TestInterface>>& aRetVal);

  void ReceiveCallbackObjectSequence(nsTArray<RefPtr<TestCallbackInterface>>& aRetVal);

  void ReceiveNullableCastableObjectSequence(nsTArray<RefPtr<TestInterface>>& aRetVal);

  void ReceiveNullableCallbackObjectSequence(nsTArray<RefPtr<TestCallbackInterface>>& aRetVal);

  void ReceiveCastableObjectNullableSequence(Nullable<nsTArray<RefPtr<TestInterface>>>& aRetVal);

  void ReceiveNullableCastableObjectNullableSequence(Nullable<nsTArray<RefPtr<TestInterface>>>& aRetVal);

  void ReceiveWeakCastableObjectSequence(nsTArray<RefPtr<TestInterface>>& aRetVal);

  void ReceiveWeakNullableCastableObjectSequence(nsTArray<RefPtr<TestInterface>>& aRetVal);

  void ReceiveWeakCastableObjectNullableSequence(Nullable<nsTArray<RefPtr<TestInterface>>>& aRetVal);

  void ReceiveWeakNullableCastableObjectNullableSequence(Nullable<nsTArray<RefPtr<TestInterface>>>& aRetVal);

  void PassCastableObjectSequence(const Sequence<OwningNonNull<TestInterface>>& arg);

  void PassNullableCastableObjectSequence(const Sequence<RefPtr<TestInterface>>& arg);

  void PassCastableObjectNullableSequence(const Nullable<Sequence<OwningNonNull<TestInterface>>>& arg);

  void PassNullableCastableObjectNullableSequence(const Nullable<Sequence<RefPtr<TestInterface>>>& arg);

  void PassOptionalSequence(const Optional<Sequence<int32_t>>& arg);

  void PassOptionalSequenceWithDefaultValue(const Sequence<int32_t>& arg);

  void PassOptionalNullableSequence(const Optional<Nullable<Sequence<int32_t>>>& arg);

  void PassOptionalNullableSequenceWithDefaultValue(const Nullable<Sequence<int32_t>>& arg);

  void PassOptionalNullableSequenceWithDefaultValue2(const Nullable<Sequence<int32_t>>& arg);

  void PassOptionalObjectSequence(const Optional<Sequence<OwningNonNull<TestInterface>>>& arg);

  void PassExternalInterfaceSequence(const Sequence<RefPtr<TestExternalInterface>>& arg);

  void PassNullableExternalInterfaceSequence(const Sequence<RefPtr<TestExternalInterface>>& arg);

  void ReceiveStringSequence(nsTArray<nsString>& aRetVal);

  void PassStringSequence(const Sequence<nsString>& arg);

  void ReceiveByteStringSequence(nsTArray<nsCString>& aRetVal);

  void PassByteStringSequence(const Sequence<nsCString>& arg);

  void ReceiveUTF8StringSequence(nsTArray<nsCString>& aRetVal);

  void PassUTF8StringSequence(const Sequence<nsCString>& arg);

  void ReceiveAnySequence(JSContext* cx, nsTArray<JS::Value>& aRetVal);

  void ReceiveNullableAnySequence(JSContext* cx, Nullable<nsTArray<JS::Value>>& aRetVal);

  void ReceiveObjectSequence(JSContext* cx, nsTArray<JSObject*>& aRetVal);

  void ReceiveNullableObjectSequence(JSContext* cx, nsTArray<JSObject*>& aRetVal);

  void PassSequenceOfSequences(const Sequence<Sequence<int32_t>>& arg);

  void PassSequenceOfSequencesOfSequences(const Sequence<Sequence<Sequence<int32_t>>>& arg);

  void PassRecord(const Record<nsString, int32_t>& arg);

  void PassNullableRecord(const Nullable<Record<nsString, int32_t>>& arg);

  void PassRecordOfNullableInts(const Record<nsString, Nullable<int32_t>>& arg);

  void PassOptionalRecordOfNullableInts(const Optional<Record<nsString, Nullable<int32_t>>>& arg);

  void PassOptionalNullableRecordOfNullableInts(const Optional<Nullable<Record<nsString, Nullable<int32_t>>>>& arg);

  void PassCastableObjectRecord(const Record<nsString, OwningNonNull<TestInterface>>& arg);

  void PassNullableCastableObjectRecord(const Record<nsString, RefPtr<TestInterface>>& arg);

  void PassCastableObjectNullableRecord(const Nullable<Record<nsString, OwningNonNull<TestInterface>>>& arg);

  void PassNullableCastableObjectNullableRecord(const Nullable<Record<nsString, RefPtr<TestInterface>>>& arg);

  void PassOptionalRecord(const Optional<Record<nsString, int32_t>>& arg);

  void PassOptionalNullableRecord(const Optional<Nullable<Record<nsString, int32_t>>>& arg);

  void PassOptionalNullableRecordWithDefaultValue(const Nullable<Record<nsString, int32_t>>& arg);

  void PassOptionalObjectRecord(const Optional<Record<nsString, OwningNonNull<TestInterface>>>& arg);

  void PassExternalInterfaceRecord(const Record<nsString, RefPtr<TestExternalInterface>>& arg);

  void PassNullableExternalInterfaceRecord(const Record<nsString, RefPtr<TestExternalInterface>>& arg);

  void PassStringRecord(const Record<nsString, nsString>& arg);

  void PassByteStringRecord(const Record<nsString, nsCString>& arg);

  void PassUTF8StringRecord(const Record<nsString, nsCString>& arg);

  void PassRecordOfRecords(const Record<nsString, Record<nsString, int32_t>>& arg);

  void ReceiveRecord(Record<nsString, int32_t>& aRetVal);

  void ReceiveNullableRecord(Nullable<Record<nsString, int32_t>>& aRetVal);

  void ReceiveRecordOfNullableInts(Record<nsString, Nullable<int32_t>>& aRetVal);

  void ReceiveNullableRecordOfNullableInts(Nullable<Record<nsString, Nullable<int32_t>>>& aRetVal);

  void ReceiveAnyRecord(JSContext* cx, Record<nsString, JS::Value>& aRetVal);

  void PassArrayBuffer(const ArrayBuffer& arg);

  void PassNullableArrayBuffer(const Nullable<ArrayBuffer>& arg);

  void PassOptionalArrayBuffer(const Optional<ArrayBuffer>& arg);

  void PassOptionalNullableArrayBuffer(const Optional<Nullable<ArrayBuffer>>& arg);

  void PassOptionalNullableArrayBufferWithDefaultValue(const Nullable<ArrayBuffer>& arg);

  void PassArrayBufferView(const ArrayBufferView& arg);

  void PassInt8Array(const Int8Array& arg);

  void PassInt16Array(const Int16Array& arg);

  void PassInt32Array(const Int32Array& arg);

  void PassUint8Array(const Uint8Array& arg);

  void PassUint16Array(const Uint16Array& arg);

  void PassUint32Array(const Uint32Array& arg);

  void PassUint8ClampedArray(const Uint8ClampedArray& arg);

  void PassFloat32Array(const Float32Array& arg);

  void PassFloat64Array(const Float64Array& arg);

  void PassSequenceOfArrayBuffers(const Sequence<ArrayBuffer>& arg);

  void PassSequenceOfNullableArrayBuffers(const Sequence<Nullable<ArrayBuffer>>& arg);

  void PassRecordOfArrayBuffers(const Record<nsString, ArrayBuffer>& arg);

  void PassRecordOfNullableArrayBuffers(const Record<nsString, Nullable<ArrayBuffer>>& arg);

  void PassVariadicTypedArray(const Sequence<Float32Array>& arg);

  void PassVariadicNullableTypedArray(const Sequence<Nullable<Float32Array>>& arg);

  void ReceiveUint8Array(JSContext* cx, JS::MutableHandle<JSObject*> aRetVal);

  void GetUint8ArrayAttr(JSContext* cx, JS::MutableHandle<JSObject*> aRetVal) const;

  void SetUint8ArrayAttr(const Uint8Array& arg);

  void PassString(const nsAString& arg);

  void PassNullableString(const nsAString& arg);

  void PassOptionalString(const Optional<nsAString>& arg);

  void PassOptionalStringWithDefaultValue(const nsAString& arg);

  void PassOptionalNullableString(const Optional<nsAString>& arg);

  void PassOptionalNullableStringWithDefaultValue(const nsAString& arg);

  void PassVariadicString(const Sequence<nsString>& arg);

  void PassByteString(const nsCString& arg);

  void PassNullableByteString(const nsCString& arg);

  void PassOptionalByteString(const Optional<nsCString>& arg);

  void PassOptionalByteStringWithDefaultValue(const nsCString& arg);

  void PassOptionalNullableByteString(const Optional<nsCString>& arg);

  void PassOptionalNullableByteStringWithDefaultValue(const nsCString& arg);

  void PassVariadicByteString(const Sequence<nsCString>& arg);

  void PassUnionByteString(const ByteStringOrLong& arg);

  void PassOptionalUnionByteString(const Optional<ByteStringOrLong>& arg);

  void PassOptionalUnionByteStringWithDefaultValue(const ByteStringOrLong& arg);

  void PassUTF8String(const nsACString& arg);

  void PassNullableUTF8String(const nsACString& arg);

  void PassOptionalUTF8String(const Optional<nsACString>& arg);

  void PassOptionalUTF8StringWithDefaultValue(const nsACString& arg);

  void PassOptionalNullableUTF8String(const Optional<nsACString>& arg);

  void PassOptionalNullableUTF8StringWithDefaultValue(const nsACString& arg);

  void PassVariadicUTF8String(const Sequence<nsCString>& arg);

  void PassUnionUTF8String(const UTF8StringOrLong& arg);

  void PassOptionalUnionUTF8String(const Optional<UTF8StringOrLong>& arg);

  void PassOptionalUnionUTF8StringWithDefaultValue(const UTF8StringOrLong& arg);

  void PassSVS(const nsAString& arg);

  void PassNullableSVS(const nsAString& arg);

  void PassOptionalSVS(const Optional<nsAString>& arg);

  void PassOptionalSVSWithDefaultValue(const nsAString& arg);

  void PassOptionalNullableSVS(const Optional<nsAString>& arg);

  void PassOptionalNullableSVSWithDefaultValue(const nsAString& arg);

  void PassVariadicSVS(const Sequence<nsString>& arg);

  void ReceiveSVS(nsString& aRetVal);

  void PassJSString(JSContext* cx, JS::Handle<JSString*> arg);

  void PassOptionalJSStringWithDefaultValue(JSContext* cx, JS::Handle<JSString*> arg);

  void ReceiveJSString(JSContext* cx, JS::MutableHandle<JSString*> aRetVal);

  void GetReadonlyJSStringAttr(JSContext* cx, JS::MutableHandle<JSString*> aRetVal) const;

  void GetJsStringAttr(JSContext* cx, JS::MutableHandle<JSString*> aRetVal) const;

  void SetJsStringAttr(JSContext* cx, JS::Handle<JSString*> arg);

  void PassEnum(TestEnum arg);

  void PassNullableEnum(const Nullable<TestEnum>& arg);

  void PassOptionalEnum(const Optional<TestEnum>& arg);

  void PassEnumWithDefault(TestEnum arg);

  void PassOptionalNullableEnum(const Optional<Nullable<TestEnum>>& arg);

  void PassOptionalNullableEnumWithDefaultValue(const Nullable<TestEnum>& arg);

  void PassOptionalNullableEnumWithDefaultValue2(const Nullable<TestEnum>& arg);

  TestEnum ReceiveEnum();

  Nullable<TestEnum> ReceiveNullableEnum();

  TestEnum EnumAttribute() const;

  void SetEnumAttribute(TestEnum arg);

  TestEnum ReadonlyEnumAttribute() const;

  void PassCallback(TestCallback& arg);

  void PassNullableCallback(TestCallback* arg);

  void PassOptionalCallback(const Optional<OwningNonNull<TestCallback>>& arg);

  void PassOptionalNullableCallback(const Optional<RefPtr<TestCallback>>& arg);

  void PassOptionalNullableCallbackWithDefaultValue(TestCallback* arg);

  already_AddRefed<TestCallback> ReceiveCallback();

  already_AddRefed<TestCallback> ReceiveNullableCallback();

  void PassNullableTreatAsNullCallback(TestTreatAsNullCallback* arg);

  void PassOptionalNullableTreatAsNullCallback(const Optional<RefPtr<TestTreatAsNullCallback>>& arg);

  void PassOptionalNullableTreatAsNullCallbackWithDefaultValue(TestTreatAsNullCallback* arg);

  void PassAny(JSContext* cx, JS::Handle<JS::Value> arg);

  void PassVariadicAny(JSContext* cx, const Sequence<JS::Value>& arg);

  void PassOptionalAny(JSContext* cx, JS::Handle<JS::Value> arg);

  void PassAnyDefaultNull(JSContext* cx, JS::Handle<JS::Value> arg);

  void PassSequenceOfAny(JSContext* cx, const Sequence<JS::Value>& arg);

  void PassNullableSequenceOfAny(JSContext* cx, const Nullable<Sequence<JS::Value>>& arg);

  void PassOptionalSequenceOfAny(JSContext* cx, const Optional<Sequence<JS::Value>>& arg);

  void PassOptionalNullableSequenceOfAny(JSContext* cx, const Optional<Nullable<Sequence<JS::Value>>>& arg);

  void PassOptionalSequenceOfAnyWithDefaultValue(JSContext* cx, const Nullable<Sequence<JS::Value>>& arg);

  void PassSequenceOfSequenceOfAny(JSContext* cx, const Sequence<Sequence<JS::Value>>& arg);

  void PassSequenceOfNullableSequenceOfAny(JSContext* cx, const Sequence<Nullable<Sequence<JS::Value>>>& arg);

  void PassNullableSequenceOfNullableSequenceOfAny(JSContext* cx, const Nullable<Sequence<Nullable<Sequence<JS::Value>>>>& arg);

  void PassOptionalNullableSequenceOfNullableSequenceOfAny(JSContext* cx, const Optional<Nullable<Sequence<Nullable<Sequence<JS::Value>>>>>& arg);

  void PassRecordOfAny(JSContext* cx, const Record<nsString, JS::Value>& arg);

  void PassNullableRecordOfAny(JSContext* cx, const Nullable<Record<nsString, JS::Value>>& arg);

  void PassOptionalRecordOfAny(JSContext* cx, const Optional<Record<nsString, JS::Value>>& arg);

  void PassOptionalNullableRecordOfAny(JSContext* cx, const Optional<Nullable<Record<nsString, JS::Value>>>& arg);

  void PassOptionalRecordOfAnyWithDefaultValue(JSContext* cx, const Nullable<Record<nsString, JS::Value>>& arg);

  void PassRecordOfRecordOfAny(JSContext* cx, const Record<nsString, Record<nsString, JS::Value>>& arg);

  void PassRecordOfNullableRecordOfAny(JSContext* cx, const Record<nsString, Nullable<Record<nsString, JS::Value>>>& arg);

  void PassNullableRecordOfNullableRecordOfAny(JSContext* cx, const Nullable<Record<nsString, Nullable<Record<nsString, JS::Value>>>>& arg);

  void PassOptionalNullableRecordOfNullableRecordOfAny(JSContext* cx, const Optional<Nullable<Record<nsString, Nullable<Record<nsString, JS::Value>>>>>& arg);

  void PassOptionalNullableRecordOfNullableSequenceOfAny(JSContext* cx, const Optional<Nullable<Record<nsString, Nullable<Sequence<JS::Value>>>>>& arg);

  void PassOptionalNullableSequenceOfNullableRecordOfAny(JSContext* cx, const Optional<Nullable<Sequence<Nullable<Record<nsString, JS::Value>>>>>& arg);

  void ReceiveAny(JSContext* cx, JS::MutableHandle<JS::Value> aRetVal);

  void PassObject(JSContext* cx, JS::Handle<JSObject*> arg);

  void PassVariadicObject(JSContext* cx, const Sequence<JSObject*>& arg);

  void PassNullableObject(JSContext* cx, JS::Handle<JSObject*> arg);

  void PassVariadicNullableObject(JSContext* cx, const Sequence<JSObject*>& arg);

  void PassOptionalObject(JSContext* cx, const Optional<JS::Handle<JSObject*>>& arg);

  void PassOptionalNullableObject(JSContext* cx, const Optional<JS::Handle<JSObject*>>& arg);

  void PassOptionalNullableObjectWithDefaultValue(JSContext* cx, JS::Handle<JSObject*> arg);

  void PassSequenceOfObject(JSContext* cx, const Sequence<JSObject*>& arg);

  void PassSequenceOfNullableObject(JSContext* cx, const Sequence<JSObject*>& arg);

  void PassNullableSequenceOfObject(JSContext* cx, const Nullable<Sequence<JSObject*>>& arg);

  void PassOptionalNullableSequenceOfNullableSequenceOfObject(JSContext* cx, const Optional<Nullable<Sequence<Nullable<Sequence<JSObject*>>>>>& arg);

  void PassOptionalNullableSequenceOfNullableSequenceOfNullableObject(JSContext* cx, const Optional<Nullable<Sequence<Nullable<Sequence<JSObject*>>>>>& arg);

  void PassRecordOfObject(JSContext* cx, const Record<nsString, JSObject*>& arg);

  void ReceiveObject(JSContext* cx, JS::MutableHandle<JSObject*> aRetVal);

  void ReceiveNullableObject(JSContext* cx, JS::MutableHandle<JSObject*> aRetVal);

  void PassUnion(JSContext* cx, const ObjectOrLong& arg);

  void PassUnionWithNullable(JSContext* cx, const ObjectOrNullOrLong& arg);

  void PassNullableUnion(JSContext* cx, const Nullable<ObjectOrLong>& arg);

  void PassOptionalUnion(JSContext* cx, const Optional<ObjectOrLong>& arg);

  void PassOptionalNullableUnion(JSContext* cx, const Optional<Nullable<ObjectOrLong>>& arg);

  void PassOptionalNullableUnionWithDefaultValue(JSContext* cx, const Nullable<ObjectOrLong>& arg);

  void PassUnionWithArrayBuffer(const ArrayBufferOrLong& arg);

  void PassUnionWithString(JSContext* cx, const StringOrObject& arg);

  void PassUnionWithEnum(JSContext* cx, const SupportedTypeOrObject& arg);

  void PassUnionWithObject(JSContext* cx, const ObjectOrLong& arg);

  void PassUnionWithDefaultValue1(const DoubleOrString& arg);

  void PassUnionWithDefaultValue2(const DoubleOrString& arg);

  void PassUnionWithDefaultValue3(const DoubleOrString& arg);

  void PassUnionWithDefaultValue4(const FloatOrString& arg);

  void PassUnionWithDefaultValue5(const FloatOrString& arg);

  void PassUnionWithDefaultValue6(const FloatOrString& arg);

  void PassUnionWithDefaultValue7(const UnrestrictedDoubleOrString& arg);

  void PassUnionWithDefaultValue8(const UnrestrictedDoubleOrString& arg);

  void PassUnionWithDefaultValue9(const UnrestrictedDoubleOrString& arg);

  void PassUnionWithDefaultValue10(const UnrestrictedDoubleOrString& arg);

  void PassUnionWithDefaultValue11(const UnrestrictedFloatOrString& arg);

  void PassUnionWithDefaultValue12(const UnrestrictedFloatOrString& arg);

  void PassUnionWithDefaultValue13(const UnrestrictedFloatOrString& arg);

  void PassUnionWithDefaultValue14(const DoubleOrByteString& arg);

  void PassUnionWithDefaultValue15(const DoubleOrByteString& arg);

  void PassUnionWithDefaultValue16(const DoubleOrByteString& arg);

  void PassUnionWithDefaultValue17(const DoubleOrSupportedType& arg);

  void PassUnionWithDefaultValue18(const DoubleOrSupportedType& arg);

  void PassUnionWithDefaultValue19(const DoubleOrSupportedType& arg);

  void PassUnionWithDefaultValue20(const DoubleOrUSVString& arg);

  void PassUnionWithDefaultValue21(const DoubleOrUSVString& arg);

  void PassUnionWithDefaultValue22(const DoubleOrUSVString& arg);

  void PassUnionWithDefaultValue23(const DoubleOrUTF8String& arg);

  void PassUnionWithDefaultValue24(const DoubleOrUTF8String& arg);

  void PassUnionWithDefaultValue25(const DoubleOrUTF8String& arg);

  void PassNullableUnionWithDefaultValue1(const Nullable<DoubleOrString>& arg);

  void PassNullableUnionWithDefaultValue2(const Nullable<DoubleOrString>& arg);

  void PassNullableUnionWithDefaultValue3(const Nullable<DoubleOrString>& arg);

  void PassNullableUnionWithDefaultValue4(const Nullable<FloatOrString>& arg);

  void PassNullableUnionWithDefaultValue5(const Nullable<FloatOrString>& arg);

  void PassNullableUnionWithDefaultValue6(const Nullable<FloatOrString>& arg);

  void PassNullableUnionWithDefaultValue7(const Nullable<UnrestrictedDoubleOrString>& arg);

  void PassNullableUnionWithDefaultValue8(const Nullable<UnrestrictedDoubleOrString>& arg);

  void PassNullableUnionWithDefaultValue9(const Nullable<UnrestrictedDoubleOrString>& arg);

  void PassNullableUnionWithDefaultValue10(const Nullable<UnrestrictedFloatOrString>& arg);

  void PassNullableUnionWithDefaultValue11(const Nullable<UnrestrictedFloatOrString>& arg);

  void PassNullableUnionWithDefaultValue12(const Nullable<UnrestrictedFloatOrString>& arg);

  void PassNullableUnionWithDefaultValue13(const Nullable<DoubleOrByteString>& arg);

  void PassNullableUnionWithDefaultValue14(const Nullable<DoubleOrByteString>& arg);

  void PassNullableUnionWithDefaultValue15(const Nullable<DoubleOrByteString>& arg);

  void PassNullableUnionWithDefaultValue16(const Nullable<DoubleOrByteString>& arg);

  void PassNullableUnionWithDefaultValue17(const Nullable<DoubleOrSupportedType>& arg);

  void PassNullableUnionWithDefaultValue18(const Nullable<DoubleOrSupportedType>& arg);

  void PassNullableUnionWithDefaultValue19(const Nullable<DoubleOrSupportedType>& arg);

  void PassNullableUnionWithDefaultValue20(const Nullable<DoubleOrSupportedType>& arg);

  void PassNullableUnionWithDefaultValue21(const Nullable<DoubleOrUSVString>& arg);

  void PassNullableUnionWithDefaultValue22(const Nullable<DoubleOrUSVString>& arg);

  void PassNullableUnionWithDefaultValue23(const Nullable<DoubleOrUSVString>& arg);

  void PassNullableUnionWithDefaultValue24(const Nullable<DoubleOrUSVString>& arg);

  void PassNullableUnionWithDefaultValue25(const Nullable<DoubleOrUTF8String>& arg);

  void PassNullableUnionWithDefaultValue26(const Nullable<DoubleOrUTF8String>& arg);

  void PassNullableUnionWithDefaultValue27(const Nullable<DoubleOrUTF8String>& arg);

  void PassNullableUnionWithDefaultValue28(const Nullable<DoubleOrUTF8String>& arg);

  void PassSequenceOfUnions(const Sequence<OwningCanvasPatternOrCanvasGradient>& arg);

  void PassSequenceOfUnions2(JSContext* cx, const Sequence<OwningObjectOrLong>& arg);

  void PassVariadicUnion(const Sequence<OwningCanvasPatternOrCanvasGradient>& arg);

  void PassSequenceOfNullableUnions(const Sequence<Nullable<OwningCanvasPatternOrCanvasGradient>>& arg);

  void PassVariadicNullableUnion(const Sequence<Nullable<OwningCanvasPatternOrCanvasGradient>>& arg);

  void PassRecordOfUnions(const Record<nsString, OwningCanvasPatternOrCanvasGradient>& arg);

  void ReceiveUnion(OwningCanvasPatternOrCanvasGradient& aRetVal);

  void ReceiveUnion2(JSContext* cx, OwningObjectOrLong& aRetVal);

  void ReceiveUnionContainingNull(OwningCanvasPatternOrNullOrCanvasGradient& aRetVal);

  void ReceiveNullableUnion(Nullable<OwningCanvasPatternOrCanvasGradient>& aRetVal);

  void ReceiveNullableUnion2(JSContext* cx, Nullable<OwningObjectOrLong>& aRetVal);

  void GetWritableUnion(OwningCanvasPatternOrCanvasGradient& aRetVal) const;

  void SetWritableUnion(const CanvasPatternOrCanvasGradient& arg);

  void GetWritableUnionContainingNull(OwningCanvasPatternOrNullOrCanvasGradient& aRetVal) const;

  void SetWritableUnionContainingNull(const CanvasPatternOrNullOrCanvasGradient& arg);

  void GetWritableNullableUnion(Nullable<OwningCanvasPatternOrCanvasGradient>& aRetVal) const;

  void SetWritableNullableUnion(const Nullable<CanvasPatternOrCanvasGradient>& arg);

  void PassPromise(Promise& arg);

  void PassOptionalPromise(const Optional<OwningNonNull<Promise>>& arg);

  void PassPromiseSequence(const Sequence<OwningNonNull<Promise>>& arg);

  // Return a raw pointer here to avoid refcounting, but make sure it's safe (the object should be kept alive by the callee).
  already_AddRefed<Promise> ReceivePromise();

  // Return a raw pointer here to avoid refcounting, but make sure it's safe (the object should be kept alive by the callee).
  already_AddRefed<Promise> ReceiveAddrefedPromise();

  void MethodRenamedTo();
  void MethodRenamedTo(int8_t argument);

  int8_t AttributeGetterRenamedTo() const;

  int8_t AttributeRenamedTo() const;

  void SetAttributeRenamedTo(int8_t arg);

  void PassDictionary(JSContext* cx, const Dict& x);

  void PassDictionary2(JSContext* cx, const Dict& x);

  void GetReadonlyDictionary(JSContext* cx, Dict& aRetVal) const;

  void GetReadonlyNullableDictionary(JSContext* cx, Nullable<Dict>& aRetVal) const;

  void GetWritableDictionary(JSContext* cx, Dict& aRetVal) const;

  void SetWritableDictionary(JSContext* cx, const Dict& arg);

  void GetReadonlyFrozenDictionary(JSContext* cx, Dict& aRetVal) const;

  void GetReadonlyFrozenNullableDictionary(JSContext* cx, Nullable<Dict>& aRetVal) const;

  void GetWritableFrozenDictionary(JSContext* cx, Dict& aRetVal) const;

  void SetWritableFrozenDictionary(JSContext* cx, const Dict& arg);

  void ReceiveDictionary(JSContext* cx, Dict& aRetVal);

  void ReceiveNullableDictionary(JSContext* cx, Nullable<Dict>& aRetVal);

  void PassOtherDictionary(const GrandparentDict& x);

  void PassSequenceOfDictionaries(JSContext* cx, const Sequence<Dict>& x);

  void PassRecordOfDictionaries(const Record<nsString, GrandparentDict>& x);

  void PassDictionaryOrLong(JSContext* cx, const Dict& x);
  void PassDictionaryOrLong(int32_t x);

  void PassDictContainingDict(JSContext* cx, const DictContainingDict& arg);

  void PassDictContainingSequence(JSContext* cx, const DictContainingSequence& arg);

  void ReceiveDictContainingSequence(JSContext* cx, DictContainingSequence& aRetVal);

  void PassVariadicDictionary(JSContext* cx, const Sequence<Dict>& arg);

  void DontEnforceRangeOrClamp(int8_t arg);

  void DoEnforceRange(int8_t arg);

  void DoEnforceRangeNullable(const Nullable<int8_t>& arg);

  void DoClamp(int8_t arg);

  void DoClampNullable(const Nullable<int8_t>& arg);

  int8_t EnforcedByte() const;

  void SetEnforcedByte(int8_t arg);

  Nullable<int8_t> GetEnforcedByteNullable() const;

  void SetEnforcedByteNullable(const Nullable<int8_t>& arg);

  int8_t ClampedByte() const;

  void SetClampedByte(int8_t arg);

  Nullable<int8_t> GetClampedByteNullable() const;

  void SetClampedByteNullable(const Nullable<int8_t>& arg);

  void ExerciseTypedefInterfaces1(TestInterface& arg);

  // Return a raw pointer here to avoid refcounting, but make sure it's safe (the object should be kept alive by the callee).
  already_AddRefed<TestInterface> ExerciseTypedefInterfaces2(TestInterface* arg);

  void ExerciseTypedefInterfaces3(TestInterface& arg);

  bool DeprecatedAttribute() const;

  void SetDeprecatedAttribute(bool arg);

  void DeprecatedMethod(bool arg);

  void DeprecatedMethodWithContext(JSContext* cx, JS::Handle<JS::Value> arg);

  static bool StaticAttribute(const GlobalObject& global);

  static void SetStaticAttribute(const GlobalObject& global, bool arg);

  static void StaticMethod(const GlobalObject& global, bool arg);

  static void StaticMethodWithContext(const GlobalObject& global, JS::Handle<JS::Value> arg);

  static bool StaticDeprecatedAttribute(const GlobalObject& global);

  static void SetStaticDeprecatedAttribute(const GlobalObject& global, bool arg);

  static void StaticDeprecatedMethod(const GlobalObject& global, bool arg);

  static void StaticDeprecatedMethodWithContext(const GlobalObject& global, JS::Handle<JS::Value> arg);

  bool Overload1(TestInterface& arg);
  // Return a raw pointer here to avoid refcounting, but make sure it's safe (the object should be kept alive by the callee).
  already_AddRefed<TestInterface> Overload1(const nsAString& strs, TestInterface& arg);

  void Overload2(TestInterface& arg);
  void Overload2(JSContext* cx, const Dict& arg);
  void Overload2(bool arg);
  void Overload2(const nsAString& arg);

  void Overload3(TestInterface& arg);
  void Overload3(TestCallback& arg);
  void Overload3(bool arg);

  void Overload4(TestInterface& arg);
  void Overload4(TestCallbackInterface& arg);
  void Overload4(const nsAString& arg);

  void Overload5(int32_t arg);
  void Overload5(TestEnum arg);

  void Overload6(int32_t arg);
  void Overload6(bool arg);

  void Overload7(int32_t arg);
  void Overload7(bool arg);
  void Overload7(const nsCString& arg);

  void Overload8(int32_t arg);
  void Overload8(TestInterface& arg);

  void Overload9(const Nullable<int32_t>& arg);
  void Overload9(const nsAString& arg);

  void Overload10(const Nullable<int32_t>& arg);
  void Overload10(JSContext* cx, JS::Handle<JSObject*> arg);

  void Overload11(int32_t arg);
  void Overload11(const nsAString& arg);

  void Overload12(int32_t arg);
  void Overload12(const Nullable<bool>& arg);

  void Overload13(const Nullable<int32_t>& arg);
  void Overload13(bool arg);

  void Overload14(const Optional<int32_t>& arg);
  void Overload14(TestInterface& arg);

  void Overload15(int32_t arg);
  void Overload15(const Optional<NonNull<TestInterface>>& arg);

  void Overload16(int32_t arg);
  void Overload16(const Optional<TestInterface*>& arg);

  void Overload17(const Sequence<int32_t>& arg);
  void Overload17(const Record<nsString, int32_t>& arg);

  void Overload18(const Record<nsString, nsString>& arg);
  void Overload18(const Sequence<nsString>& arg);

  void Overload19(const Sequence<int32_t>& arg);
  void Overload19(JSContext* cx, const Dict& arg);

  void Overload20(JSContext* cx, const Dict& arg);
  void Overload20(const Sequence<int32_t>& arg);

  void PassVariadicThirdArg(const nsAString& arg1, int32_t arg2, const Sequence<OwningNonNull<TestInterface>>& arg3);

  bool Prefable1() const;

  bool Prefable2() const;

  bool Prefable3() const;

  bool Prefable4() const;

  bool Prefable5() const;

  bool Prefable6() const;

  bool Prefable7() const;

  bool Prefable8() const;

  bool Prefable9() const;

  void Prefable10();

  void Prefable11();

  bool Prefable12() const;

  void Prefable13();

  bool Prefable14() const;

  bool Prefable15() const;

  bool Prefable16() const;

  void Prefable17();

  void Prefable18();

  void Prefable19();

  bool ConditionalOnSecureContext1() const;

  bool ConditionalOnSecureContext2() const;

  bool ConditionalOnSecureContext3() const;

  bool ConditionalOnSecureContext4() const;

  void ConditionalOnSecureContext5();

  void ConditionalOnSecureContext6();

  void ConditionalOnSecureContext7();

  void ConditionalOnSecureContext8();

  int32_t AttrWithLenientThis() const;

  void SetAttrWithLenientThis(int32_t arg);

  int32_t UnforgeableAttr() const;

  int32_t UnforgeableAttr2() const;

  int32_t UnforgeableMethod();

  int32_t UnforgeableMethod2();

  void PassRenamedInterface(nsRenamedInterface& arg);

  // Return a raw pointer here to avoid refcounting, but make sure it's safe (the object should be kept alive by the callee).
  already_AddRefed<TestExampleInterface> PutForwardsAttr() const;

  // Return a raw pointer here to avoid refcounting, but make sure it's safe (the object should be kept alive by the callee).
  already_AddRefed<TestExampleInterface> PutForwardsAttr2() const;

  // Return a raw pointer here to avoid refcounting, but make sure it's safe (the object should be kept alive by the callee).
  already_AddRefed<TestExampleInterface> PutForwardsAttr3() const;

  void ThrowingMethod(ErrorResult& aRv);

  bool GetThrowingAttr(ErrorResult& aRv) const;

  void SetThrowingAttr(bool arg, ErrorResult& aRv);

  bool GetThrowingGetterAttr(ErrorResult& aRv) const;

  void SetThrowingGetterAttr(bool arg);

  bool ThrowingSetterAttr() const;

  void SetThrowingSetterAttr(bool arg, ErrorResult& aRv);

  void CanOOMMethod(OOMReporter& aRv);

  bool GetCanOOMAttr(OOMReporter& aRv) const;

  void SetCanOOMAttr(bool arg, OOMReporter& aRv);

  bool GetCanOOMGetterAttr(OOMReporter& aRv) const;

  void SetCanOOMGetterAttr(bool arg);

  bool CanOOMSetterAttr() const;

  void SetCanOOMSetterAttr(bool arg, OOMReporter& aRv);

  void NeedsSubjectPrincipalMethod(nsIPrincipal& aPrincipal);

  bool NeedsSubjectPrincipalAttr(nsIPrincipal& aPrincipal) const;

  void SetNeedsSubjectPrincipalAttr(bool arg, nsIPrincipal& aPrincipal);

  void NeedsNonSystemSubjectPrincipalMethod(nsIPrincipal* aPrincipal);

  bool NeedsNonSystemSubjectPrincipalAttr(nsIPrincipal* aPrincipal) const;

  void SetNeedsNonSystemSubjectPrincipalAttr(bool arg, nsIPrincipal* aPrincipal);

  void NeedsCallerTypeMethod(CallerType aCallerType);

  bool NeedsCallerTypeAttr(CallerType aCallerType) const;

  void SetNeedsCallerTypeAttr(bool arg, CallerType aCallerType);

  void CeReactionsMethod();

  void CeReactionsMethodOverload();
  void CeReactionsMethodOverload(const nsAString& bar);

  bool CeReactionsAttr() const;

  void SetCeReactionsAttr(bool arg);

  void PassArgsWithDefaults(JSContext* cx, const Optional<int32_t>& arg1, TestInterface* arg2, const Dict& arg3, double arg4, const Optional<float>& arg5);

  void GetToJSONShouldSkipThis(JSContext* cx, JS::MutableHandle<JS::Value> aRetVal) const;

  void SetToJSONShouldSkipThis(JSContext* cx, JS::Handle<JS::Value> arg);

  // Return a raw pointer here to avoid refcounting, but make sure it's safe (the object should be kept alive by the callee).
  already_AddRefed<TestParentInterface> ToJSONShouldSkipThis2() const;

  void SetToJSONShouldSkipThis2(TestParentInterface& arg);

  // Return a raw pointer here to avoid refcounting, but make sure it's safe (the object should be kept alive by the callee).
  already_AddRefed<TestCallbackInterface> ToJSONShouldSkipThis3() const;

  void SetToJSONShouldSkipThis3(TestCallbackInterface& arg);

  void ToJSON(JSContext* cx, JS::MutableHandle<JSObject*> aRetVal);

  int8_t Dashed_attribute() const;

  void SetDashed_attribute(int8_t arg);

  void Dashed_method();

  bool NonEnumerableAttr() const;

  void SetNonEnumerableAttr(bool arg);

  void NonEnumerableMethod();

  void GetAllowSharedArrayBufferViewTypedef(JSContext* cx, JS::MutableHandle<JSObject*> aRetVal) const;

  void SetAllowSharedArrayBufferViewTypedef(const ArrayBufferView& arg);

  void GetAllowSharedArrayBufferView(JSContext* cx, JS::MutableHandle<JSObject*> aRetVal) const;

  void SetAllowSharedArrayBufferView(const ArrayBufferView& arg);

  void GetAllowSharedNullableArrayBufferView(JSContext* cx, JS::MutableHandle<JSObject*> aRetVal) const;

  void SetAllowSharedNullableArrayBufferView(const Nullable<ArrayBufferView>& arg);

  void GetAllowSharedArrayBuffer(JSContext* cx, JS::MutableHandle<JSObject*> aRetVal) const;

  void SetAllowSharedArrayBuffer(const ArrayBuffer& arg);

  void GetAllowSharedNullableArrayBuffer(JSContext* cx, JS::MutableHandle<JSObject*> aRetVal) const;

  void SetAllowSharedNullableArrayBuffer(const Nullable<ArrayBuffer>& arg);

  void PassAllowSharedArrayBufferViewTypedef(const ArrayBufferView& foo);

  void PassAllowSharedArrayBufferView(const ArrayBufferView& foo);

  void PassAllowSharedNullableArrayBufferView(const Nullable<ArrayBufferView>& foo);

  void PassAllowSharedArrayBuffer(const ArrayBuffer& foo);

  void PassAllowSharedNullableArrayBuffer(const Nullable<ArrayBuffer>& foo);

  void PassUnionArrayBuffer(const StringOrArrayBuffer& foo);

  void PassUnionAllowSharedArrayBuffer(const StringOrMaybeSharedArrayBuffer& foo);

  int16_t LegacyCall(const JS::Value& aThisVal, uint32_t arg1, TestInterface& arg2);

  void Stringify(nsString& aRetVal);
};

} // namespace dom
} // namespace mozilla

#endif // mozilla_dom_TestExampleInterface_h
