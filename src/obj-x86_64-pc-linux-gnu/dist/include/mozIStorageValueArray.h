/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageValueArray.idl
 */

#ifndef __gen_mozIStorageValueArray_h__
#define __gen_mozIStorageValueArray_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#include "mozilla/DebugOnly.h"

/* starting interface:    mozIStorageValueArray */
#define MOZISTORAGEVALUEARRAY_IID_STR "6e6306f4-ffa7-40f5-96ca-36159ce8f431"

#define MOZISTORAGEVALUEARRAY_IID \
  {0x6e6306f4, 0xffa7, 0x40f5, \
    { 0x96, 0xca, 0x36, 0x15, 0x9c, 0xe8, 0xf4, 0x31 }}

class mozIStorageValueArray : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZISTORAGEVALUEARRAY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIStorageValueArray;

  enum {
    VALUE_TYPE_NULL = 0,
    VALUE_TYPE_INTEGER = 1,
    VALUE_TYPE_FLOAT = 2,
    VALUE_TYPE_TEXT = 3,
    VALUE_TYPE_BLOB = 4
  };

  /* readonly attribute unsigned long numEntries; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNumEntries(uint32_t *aNumEntries) = 0;

  /* long getTypeOfIndex (in unsigned long aIndex); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTypeOfIndex(uint32_t aIndex, int32_t *_retval) = 0;

  /* long getInt32 (in unsigned long aIndex); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetInt32(uint32_t aIndex, int32_t *_retval) = 0;

  /* long long getInt64 (in unsigned long aIndex); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetInt64(uint32_t aIndex, int64_t *_retval) = 0;

  /* double getDouble (in unsigned long aIndex); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDouble(uint32_t aIndex, double *_retval) = 0;

  /* AUTF8String getUTF8String (in unsigned long aIndex); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUTF8String(uint32_t aIndex, nsACString& _retval) = 0;

  /* AString getString (in unsigned long aIndex); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetString(uint32_t aIndex, nsAString& _retval) = 0;

  /* void getBlob (in unsigned long aIndex, out unsigned long aDataSize, [array, size_is (aDataSize)] out octet aData); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetBlob(uint32_t aIndex, uint32_t *aDataSize, uint8_t **aData) = 0;

  /* AString getBlobAsString (in unsigned long aIndex); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetBlobAsString(uint32_t aIndex, nsAString& _retval) = 0;

  /* AUTF8String getBlobAsUTF8String (in unsigned long aIndex); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetBlobAsUTF8String(uint32_t aIndex, nsACString& _retval) = 0;

  /* boolean getIsNull (in unsigned long aIndex); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsNull(uint32_t aIndex, bool *_retval) = 0;

  /* [noscript] void getSharedUTF8String (in unsigned long aIndex, out unsigned long aByteLength, [shared, retval] out string aResult); */
  NS_IMETHOD GetSharedUTF8String(uint32_t aIndex, uint32_t *aByteLength, const char * *aResult) = 0;

  /* [noscript] void getSharedString (in unsigned long aIndex, out unsigned long aByteLength, [shared, retval] out wstring aResult); */
  NS_IMETHOD GetSharedString(uint32_t aIndex, uint32_t *aByteLength, const char16_t * *aResult) = 0;

  /* [noscript] void getSharedBlob (in unsigned long aIndex, out unsigned long aByteLength, [shared, retval] out octetPtr aResult); */
  NS_IMETHOD GetSharedBlob(uint32_t aIndex, uint32_t *aByteLength, const uint8_t * * aResult) = 0;

   /**
   * Getters for native code that return their values as
   * the return type, for convenience and sanity.
   *
   * Not virtual; no vtable bloat.
   */
  inline int32_t AsInt32(uint32_t idx) {
    int32_t v = 0;
    mozilla::DebugOnly<nsresult> rv = GetInt32(idx, &v);
    MOZ_ASSERT(NS_SUCCEEDED(rv) || IsNull(idx),
               "Getting value failed, wrong column index?");
    return v;
  }
  inline int64_t AsInt64(uint32_t idx) {
    int64_t v = 0;
    mozilla::DebugOnly<nsresult> rv = GetInt64(idx, &v);
    MOZ_ASSERT(NS_SUCCEEDED(rv) || IsNull(idx),
               "Getting value failed, wrong column index?");
    return v;
  }
  inline double AsDouble(uint32_t idx) {
    double v = 0.0;
    mozilla::DebugOnly<nsresult> rv = GetDouble(idx, &v);
    MOZ_ASSERT(NS_SUCCEEDED(rv) || IsNull(idx),
               "Getting value failed, wrong column index?");
    return v;
  }
  inline const char* AsSharedUTF8String(uint32_t idx, uint32_t *len) {
    const char *str = nullptr;
    *len = 0;
    mozilla::DebugOnly<nsresult> rv = GetSharedUTF8String(idx, len, &str);
    MOZ_ASSERT(NS_SUCCEEDED(rv) || IsNull(idx),
               "Getting value failed, wrong column index?");
    return str;
  }
  inline const char16_t* AsSharedWString(uint32_t idx, uint32_t *len) {
    const char16_t *str = nullptr;
    *len = 0;
    mozilla::DebugOnly<nsresult> rv = GetSharedString(idx, len, &str);
    MOZ_ASSERT(NS_SUCCEEDED(rv) || IsNull(idx),
               "Getting value failed, wrong column index?");
    return str;
  }
  inline const uint8_t* AsSharedBlob(uint32_t idx, uint32_t *len) {
    const uint8_t *blob = nullptr;
    *len = 0;
    mozilla::DebugOnly<nsresult> rv = GetSharedBlob(idx, len, &blob);
    MOZ_ASSERT(NS_SUCCEEDED(rv) || IsNull(idx),
               "Getting value failed, wrong column index?");
    return blob;
  }
  inline bool IsNull(uint32_t idx) {
    bool b = false;
    mozilla::DebugOnly<nsresult> rv = GetIsNull(idx, &b);
    MOZ_ASSERT(NS_SUCCEEDED(rv),
               "Getting value failed, wrong column index?");
    return b;
  }
};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIStorageValueArray, MOZISTORAGEVALUEARRAY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZISTORAGEVALUEARRAY \
  NS_IMETHOD GetNumEntries(uint32_t *aNumEntries) override; \
  NS_IMETHOD GetTypeOfIndex(uint32_t aIndex, int32_t *_retval) override; \
  NS_IMETHOD GetInt32(uint32_t aIndex, int32_t *_retval) override; \
  NS_IMETHOD GetInt64(uint32_t aIndex, int64_t *_retval) override; \
  NS_IMETHOD GetDouble(uint32_t aIndex, double *_retval) override; \
  NS_IMETHOD GetUTF8String(uint32_t aIndex, nsACString& _retval) override; \
  NS_IMETHOD GetString(uint32_t aIndex, nsAString& _retval) override; \
  NS_IMETHOD GetBlob(uint32_t aIndex, uint32_t *aDataSize, uint8_t **aData) override; \
  NS_IMETHOD GetBlobAsString(uint32_t aIndex, nsAString& _retval) override; \
  NS_IMETHOD GetBlobAsUTF8String(uint32_t aIndex, nsACString& _retval) override; \
  NS_IMETHOD GetIsNull(uint32_t aIndex, bool *_retval) override; \
  NS_IMETHOD GetSharedUTF8String(uint32_t aIndex, uint32_t *aByteLength, const char * *aResult) override; \
  NS_IMETHOD GetSharedString(uint32_t aIndex, uint32_t *aByteLength, const char16_t * *aResult) override; \
  NS_IMETHOD GetSharedBlob(uint32_t aIndex, uint32_t *aByteLength, const uint8_t * * aResult) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZISTORAGEVALUEARRAY \
  nsresult GetNumEntries(uint32_t *aNumEntries); \
  nsresult GetTypeOfIndex(uint32_t aIndex, int32_t *_retval); \
  nsresult GetInt32(uint32_t aIndex, int32_t *_retval); \
  nsresult GetInt64(uint32_t aIndex, int64_t *_retval); \
  nsresult GetDouble(uint32_t aIndex, double *_retval); \
  nsresult GetUTF8String(uint32_t aIndex, nsACString& _retval); \
  nsresult GetString(uint32_t aIndex, nsAString& _retval); \
  nsresult GetBlob(uint32_t aIndex, uint32_t *aDataSize, uint8_t **aData); \
  nsresult GetBlobAsString(uint32_t aIndex, nsAString& _retval); \
  nsresult GetBlobAsUTF8String(uint32_t aIndex, nsACString& _retval); \
  nsresult GetIsNull(uint32_t aIndex, bool *_retval); \
  nsresult GetSharedUTF8String(uint32_t aIndex, uint32_t *aByteLength, const char * *aResult); \
  nsresult GetSharedString(uint32_t aIndex, uint32_t *aByteLength, const char16_t * *aResult); \
  nsresult GetSharedBlob(uint32_t aIndex, uint32_t *aByteLength, const uint8_t * * aResult); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZISTORAGEVALUEARRAY(_to) \
  NS_IMETHOD GetNumEntries(uint32_t *aNumEntries) override { return _to GetNumEntries(aNumEntries); } \
  NS_IMETHOD GetTypeOfIndex(uint32_t aIndex, int32_t *_retval) override { return _to GetTypeOfIndex(aIndex, _retval); } \
  NS_IMETHOD GetInt32(uint32_t aIndex, int32_t *_retval) override { return _to GetInt32(aIndex, _retval); } \
  NS_IMETHOD GetInt64(uint32_t aIndex, int64_t *_retval) override { return _to GetInt64(aIndex, _retval); } \
  NS_IMETHOD GetDouble(uint32_t aIndex, double *_retval) override { return _to GetDouble(aIndex, _retval); } \
  NS_IMETHOD GetUTF8String(uint32_t aIndex, nsACString& _retval) override { return _to GetUTF8String(aIndex, _retval); } \
  NS_IMETHOD GetString(uint32_t aIndex, nsAString& _retval) override { return _to GetString(aIndex, _retval); } \
  NS_IMETHOD GetBlob(uint32_t aIndex, uint32_t *aDataSize, uint8_t **aData) override { return _to GetBlob(aIndex, aDataSize, aData); } \
  NS_IMETHOD GetBlobAsString(uint32_t aIndex, nsAString& _retval) override { return _to GetBlobAsString(aIndex, _retval); } \
  NS_IMETHOD GetBlobAsUTF8String(uint32_t aIndex, nsACString& _retval) override { return _to GetBlobAsUTF8String(aIndex, _retval); } \
  NS_IMETHOD GetIsNull(uint32_t aIndex, bool *_retval) override { return _to GetIsNull(aIndex, _retval); } \
  NS_IMETHOD GetSharedUTF8String(uint32_t aIndex, uint32_t *aByteLength, const char * *aResult) override { return _to GetSharedUTF8String(aIndex, aByteLength, aResult); } \
  NS_IMETHOD GetSharedString(uint32_t aIndex, uint32_t *aByteLength, const char16_t * *aResult) override { return _to GetSharedString(aIndex, aByteLength, aResult); } \
  NS_IMETHOD GetSharedBlob(uint32_t aIndex, uint32_t *aByteLength, const uint8_t * * aResult) override { return _to GetSharedBlob(aIndex, aByteLength, aResult); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZISTORAGEVALUEARRAY(_to) \
  NS_IMETHOD GetNumEntries(uint32_t *aNumEntries) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNumEntries(aNumEntries); } \
  NS_IMETHOD GetTypeOfIndex(uint32_t aIndex, int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTypeOfIndex(aIndex, _retval); } \
  NS_IMETHOD GetInt32(uint32_t aIndex, int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInt32(aIndex, _retval); } \
  NS_IMETHOD GetInt64(uint32_t aIndex, int64_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInt64(aIndex, _retval); } \
  NS_IMETHOD GetDouble(uint32_t aIndex, double *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDouble(aIndex, _retval); } \
  NS_IMETHOD GetUTF8String(uint32_t aIndex, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUTF8String(aIndex, _retval); } \
  NS_IMETHOD GetString(uint32_t aIndex, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetString(aIndex, _retval); } \
  NS_IMETHOD GetBlob(uint32_t aIndex, uint32_t *aDataSize, uint8_t **aData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBlob(aIndex, aDataSize, aData); } \
  NS_IMETHOD GetBlobAsString(uint32_t aIndex, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBlobAsString(aIndex, _retval); } \
  NS_IMETHOD GetBlobAsUTF8String(uint32_t aIndex, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBlobAsUTF8String(aIndex, _retval); } \
  NS_IMETHOD GetIsNull(uint32_t aIndex, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsNull(aIndex, _retval); } \
  NS_IMETHOD GetSharedUTF8String(uint32_t aIndex, uint32_t *aByteLength, const char * *aResult) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSharedUTF8String(aIndex, aByteLength, aResult); } \
  NS_IMETHOD GetSharedString(uint32_t aIndex, uint32_t *aByteLength, const char16_t * *aResult) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSharedString(aIndex, aByteLength, aResult); } \
  NS_IMETHOD GetSharedBlob(uint32_t aIndex, uint32_t *aByteLength, const uint8_t * * aResult) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSharedBlob(aIndex, aByteLength, aResult); } \


#endif /* __gen_mozIStorageValueArray_h__ */
