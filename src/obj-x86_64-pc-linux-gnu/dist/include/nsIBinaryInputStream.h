/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIBinaryInputStream.idl
 */

#ifndef __gen_nsIBinaryInputStream_h__
#define __gen_nsIBinaryInputStream_h__


#ifndef __gen_nsIInputStream_h__
#include "nsIInputStream.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIBinaryInputStream */
#define NS_IBINARYINPUTSTREAM_IID_STR "899b826b-2eb3-469c-8b31-4c29f5d341a6"

#define NS_IBINARYINPUTSTREAM_IID \
  {0x899b826b, 0x2eb3, 0x469c, \
    { 0x8b, 0x31, 0x4c, 0x29, 0xf5, 0xd3, 0x41, 0xa6 }}

class NS_NO_VTABLE nsIBinaryInputStream : public nsIInputStream {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IBINARYINPUTSTREAM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIBinaryInputStream;

  /* void setInputStream (in nsIInputStream aInputStream); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetInputStream(nsIInputStream *aInputStream) = 0;

  /* boolean readBoolean (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ReadBoolean(bool *_retval) = 0;

  /* uint8_t read8 (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Read8(uint8_t *_retval) = 0;

  /* uint16_t read16 (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Read16(uint16_t *_retval) = 0;

  /* uint32_t read32 (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Read32(uint32_t *_retval) = 0;

  /* uint64_t read64 (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Read64(uint64_t *_retval) = 0;

  /* float readFloat (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ReadFloat(float *_retval) = 0;

  /* double readDouble (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ReadDouble(double *_retval) = 0;

  /* ACString readCString (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ReadCString(nsACString& _retval) = 0;

  /* AString readString (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ReadString(nsAString& _retval) = 0;

  /* void readBytes (in uint32_t aLength, [size_is (aLength), retval] out string aString); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ReadBytes(uint32_t aLength, char * *aString) = 0;

  /* Array<uint8_t> readByteArray (in uint32_t aLength); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ReadByteArray(uint32_t aLength, nsTArray<uint8_t >& _retval) = 0;

  /* [implicit_jscontext] uint64_t readArrayBuffer (in uint64_t aLength, in jsval aArrayBuffer); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ReadArrayBuffer(uint64_t aLength, JS::HandleValue aArrayBuffer, JSContext* cx, uint64_t *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIBinaryInputStream, NS_IBINARYINPUTSTREAM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIBINARYINPUTSTREAM \
  NS_IMETHOD SetInputStream(nsIInputStream *aInputStream) override; \
  NS_IMETHOD ReadBoolean(bool *_retval) override; \
  NS_IMETHOD Read8(uint8_t *_retval) override; \
  NS_IMETHOD Read16(uint16_t *_retval) override; \
  NS_IMETHOD Read32(uint32_t *_retval) override; \
  NS_IMETHOD Read64(uint64_t *_retval) override; \
  NS_IMETHOD ReadFloat(float *_retval) override; \
  NS_IMETHOD ReadDouble(double *_retval) override; \
  NS_IMETHOD ReadCString(nsACString& _retval) override; \
  NS_IMETHOD ReadString(nsAString& _retval) override; \
  NS_IMETHOD ReadBytes(uint32_t aLength, char * *aString) override; \
  NS_IMETHOD ReadByteArray(uint32_t aLength, nsTArray<uint8_t >& _retval) override; \
  NS_IMETHOD ReadArrayBuffer(uint64_t aLength, JS::HandleValue aArrayBuffer, JSContext* cx, uint64_t *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIBINARYINPUTSTREAM \
  nsresult SetInputStream(nsIInputStream *aInputStream); \
  nsresult ReadBoolean(bool *_retval); \
  nsresult Read8(uint8_t *_retval); \
  nsresult Read16(uint16_t *_retval); \
  nsresult Read32(uint32_t *_retval); \
  nsresult Read64(uint64_t *_retval); \
  nsresult ReadFloat(float *_retval); \
  nsresult ReadDouble(double *_retval); \
  nsresult ReadCString(nsACString& _retval); \
  nsresult ReadString(nsAString& _retval); \
  nsresult ReadBytes(uint32_t aLength, char * *aString); \
  nsresult ReadByteArray(uint32_t aLength, nsTArray<uint8_t >& _retval); \
  nsresult ReadArrayBuffer(uint64_t aLength, JS::HandleValue aArrayBuffer, JSContext* cx, uint64_t *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIBINARYINPUTSTREAM(_to) \
  NS_IMETHOD SetInputStream(nsIInputStream *aInputStream) override { return _to SetInputStream(aInputStream); } \
  NS_IMETHOD ReadBoolean(bool *_retval) override { return _to ReadBoolean(_retval); } \
  NS_IMETHOD Read8(uint8_t *_retval) override { return _to Read8(_retval); } \
  NS_IMETHOD Read16(uint16_t *_retval) override { return _to Read16(_retval); } \
  NS_IMETHOD Read32(uint32_t *_retval) override { return _to Read32(_retval); } \
  NS_IMETHOD Read64(uint64_t *_retval) override { return _to Read64(_retval); } \
  NS_IMETHOD ReadFloat(float *_retval) override { return _to ReadFloat(_retval); } \
  NS_IMETHOD ReadDouble(double *_retval) override { return _to ReadDouble(_retval); } \
  NS_IMETHOD ReadCString(nsACString& _retval) override { return _to ReadCString(_retval); } \
  NS_IMETHOD ReadString(nsAString& _retval) override { return _to ReadString(_retval); } \
  NS_IMETHOD ReadBytes(uint32_t aLength, char * *aString) override { return _to ReadBytes(aLength, aString); } \
  NS_IMETHOD ReadByteArray(uint32_t aLength, nsTArray<uint8_t >& _retval) override { return _to ReadByteArray(aLength, _retval); } \
  NS_IMETHOD ReadArrayBuffer(uint64_t aLength, JS::HandleValue aArrayBuffer, JSContext* cx, uint64_t *_retval) override { return _to ReadArrayBuffer(aLength, aArrayBuffer, cx, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIBINARYINPUTSTREAM(_to) \
  NS_IMETHOD SetInputStream(nsIInputStream *aInputStream) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetInputStream(aInputStream); } \
  NS_IMETHOD ReadBoolean(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReadBoolean(_retval); } \
  NS_IMETHOD Read8(uint8_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Read8(_retval); } \
  NS_IMETHOD Read16(uint16_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Read16(_retval); } \
  NS_IMETHOD Read32(uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Read32(_retval); } \
  NS_IMETHOD Read64(uint64_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Read64(_retval); } \
  NS_IMETHOD ReadFloat(float *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReadFloat(_retval); } \
  NS_IMETHOD ReadDouble(double *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReadDouble(_retval); } \
  NS_IMETHOD ReadCString(nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReadCString(_retval); } \
  NS_IMETHOD ReadString(nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReadString(_retval); } \
  NS_IMETHOD ReadBytes(uint32_t aLength, char * *aString) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReadBytes(aLength, aString); } \
  NS_IMETHOD ReadByteArray(uint32_t aLength, nsTArray<uint8_t >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReadByteArray(aLength, _retval); } \
  NS_IMETHOD ReadArrayBuffer(uint64_t aLength, JS::HandleValue aArrayBuffer, JSContext* cx, uint64_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReadArrayBuffer(aLength, aArrayBuffer, cx, _retval); } 


#ifdef MOZILLA_INTERNAL_API
#include "nsString.h"
inline nsresult
NS_ReadOptionalCString(nsIBinaryInputStream* aStream, nsACString& aResult)
{
    bool nonnull;
    nsresult rv = aStream->ReadBoolean(&nonnull);
    if (NS_SUCCEEDED(rv)) {
        if (nonnull)
            rv = aStream->ReadCString(aResult);
        else
            aResult.Truncate();
    }
    return rv;
}
inline nsresult
NS_ReadOptionalString(nsIBinaryInputStream* aStream, nsAString& aResult)
{
    bool nonnull;
    nsresult rv = aStream->ReadBoolean(&nonnull);
    if (NS_SUCCEEDED(rv)) {
        if (nonnull)
            rv = aStream->ReadString(aResult);
        else
            aResult.Truncate();
    }
    return rv;
}
#endif

#endif /* __gen_nsIBinaryInputStream_h__ */
