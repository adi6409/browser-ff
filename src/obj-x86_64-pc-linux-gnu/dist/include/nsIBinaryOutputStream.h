/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIBinaryOutputStream.idl
 */

#ifndef __gen_nsIBinaryOutputStream_h__
#define __gen_nsIBinaryOutputStream_h__


#ifndef __gen_nsIOutputStream_h__
#include "nsIOutputStream.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
template<class ElementType, size_t Extent> class Span;
}

/* starting interface:    nsIBinaryOutputStream */
#define NS_IBINARYOUTPUTSTREAM_IID_STR "204ee610-8765-11d3-90cf-0040056a906e"

#define NS_IBINARYOUTPUTSTREAM_IID \
  {0x204ee610, 0x8765, 0x11d3, \
    { 0x90, 0xcf, 0x00, 0x40, 0x05, 0x6a, 0x90, 0x6e }}

class NS_NO_VTABLE nsIBinaryOutputStream : public nsIOutputStream {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IBINARYOUTPUTSTREAM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIBinaryOutputStream;

  /* void setOutputStream (in nsIOutputStream aOutputStream); */
  NS_IMETHOD SetOutputStream(nsIOutputStream *aOutputStream) = 0;

  /* void writeBoolean (in boolean aBoolean); */
  NS_IMETHOD WriteBoolean(bool aBoolean) = 0;

  /* void write8 (in uint8_t aByte); */
  NS_IMETHOD Write8(uint8_t aByte) = 0;

  /* void write16 (in uint16_t a16); */
  NS_IMETHOD Write16(uint16_t a16) = 0;

  /* void write32 (in uint32_t a32); */
  NS_IMETHOD Write32(uint32_t a32) = 0;

  /* void write64 (in uint64_t a64); */
  NS_IMETHOD Write64(uint64_t a64) = 0;

  /* void writeFloat (in float aFloat); */
  NS_IMETHOD WriteFloat(float aFloat) = 0;

  /* void writeDouble (in double aDouble); */
  NS_IMETHOD WriteDouble(double aDouble) = 0;

  /* void writeStringZ (in string aString); */
  NS_IMETHOD WriteStringZ(const char * aString) = 0;

  /* void writeWStringZ (in wstring aString); */
  NS_IMETHOD WriteWStringZ(const char16_t * aString) = 0;

  /* void writeUtf8Z (in wstring aString); */
  NS_IMETHOD WriteUtf8Z(const char16_t * aString) = 0;

  /* [binaryname(WriteBytesFromJS)] void writeBytes ([size_is (aLength)] in string aString, [optional] in uint32_t aLength); */
  NS_IMETHOD WriteBytesFromJS(const char * aString, uint32_t aLength) = 0;

  /* [binaryname(WriteBytes),noscript,nostdcall] void writeBytesNative (in Bytes aBytes); */
  virtual nsresult WriteBytes(mozilla::Span<const uint8_t> aBytes) = 0;

  /* void writeByteArray (in Array<uint8_t> aBytes); */
  NS_IMETHOD WriteByteArray(const nsTArray<uint8_t >& aBytes) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIBinaryOutputStream, NS_IBINARYOUTPUTSTREAM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIBINARYOUTPUTSTREAM \
  NS_IMETHOD SetOutputStream(nsIOutputStream *aOutputStream) override; \
  NS_IMETHOD WriteBoolean(bool aBoolean) override; \
  NS_IMETHOD Write8(uint8_t aByte) override; \
  NS_IMETHOD Write16(uint16_t a16) override; \
  NS_IMETHOD Write32(uint32_t a32) override; \
  NS_IMETHOD Write64(uint64_t a64) override; \
  NS_IMETHOD WriteFloat(float aFloat) override; \
  NS_IMETHOD WriteDouble(double aDouble) override; \
  NS_IMETHOD WriteStringZ(const char * aString) override; \
  NS_IMETHOD WriteWStringZ(const char16_t * aString) override; \
  NS_IMETHOD WriteUtf8Z(const char16_t * aString) override; \
  NS_IMETHOD WriteBytesFromJS(const char * aString, uint32_t aLength) override; \
  virtual nsresult WriteBytes(mozilla::Span<const uint8_t> aBytes) override; \
  NS_IMETHOD WriteByteArray(const nsTArray<uint8_t >& aBytes) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIBINARYOUTPUTSTREAM \
  nsresult SetOutputStream(nsIOutputStream *aOutputStream); \
  nsresult WriteBoolean(bool aBoolean); \
  nsresult Write8(uint8_t aByte); \
  nsresult Write16(uint16_t a16); \
  nsresult Write32(uint32_t a32); \
  nsresult Write64(uint64_t a64); \
  nsresult WriteFloat(float aFloat); \
  nsresult WriteDouble(double aDouble); \
  nsresult WriteStringZ(const char * aString); \
  nsresult WriteWStringZ(const char16_t * aString); \
  nsresult WriteUtf8Z(const char16_t * aString); \
  nsresult WriteBytesFromJS(const char * aString, uint32_t aLength); \
  nsresult WriteBytes(mozilla::Span<const uint8_t> aBytes); \
  nsresult WriteByteArray(const nsTArray<uint8_t >& aBytes); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIBINARYOUTPUTSTREAM(_to) \
  NS_IMETHOD SetOutputStream(nsIOutputStream *aOutputStream) override { return _to SetOutputStream(aOutputStream); } \
  NS_IMETHOD WriteBoolean(bool aBoolean) override { return _to WriteBoolean(aBoolean); } \
  NS_IMETHOD Write8(uint8_t aByte) override { return _to Write8(aByte); } \
  NS_IMETHOD Write16(uint16_t a16) override { return _to Write16(a16); } \
  NS_IMETHOD Write32(uint32_t a32) override { return _to Write32(a32); } \
  NS_IMETHOD Write64(uint64_t a64) override { return _to Write64(a64); } \
  NS_IMETHOD WriteFloat(float aFloat) override { return _to WriteFloat(aFloat); } \
  NS_IMETHOD WriteDouble(double aDouble) override { return _to WriteDouble(aDouble); } \
  NS_IMETHOD WriteStringZ(const char * aString) override { return _to WriteStringZ(aString); } \
  NS_IMETHOD WriteWStringZ(const char16_t * aString) override { return _to WriteWStringZ(aString); } \
  NS_IMETHOD WriteUtf8Z(const char16_t * aString) override { return _to WriteUtf8Z(aString); } \
  NS_IMETHOD WriteBytesFromJS(const char * aString, uint32_t aLength) override { return _to WriteBytesFromJS(aString, aLength); } \
  virtual nsresult WriteBytes(mozilla::Span<const uint8_t> aBytes) override { return _to WriteBytes(aBytes); } \
  NS_IMETHOD WriteByteArray(const nsTArray<uint8_t >& aBytes) override { return _to WriteByteArray(aBytes); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIBINARYOUTPUTSTREAM(_to) \
  NS_IMETHOD SetOutputStream(nsIOutputStream *aOutputStream) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetOutputStream(aOutputStream); } \
  NS_IMETHOD WriteBoolean(bool aBoolean) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WriteBoolean(aBoolean); } \
  NS_IMETHOD Write8(uint8_t aByte) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Write8(aByte); } \
  NS_IMETHOD Write16(uint16_t a16) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Write16(a16); } \
  NS_IMETHOD Write32(uint32_t a32) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Write32(a32); } \
  NS_IMETHOD Write64(uint64_t a64) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Write64(a64); } \
  NS_IMETHOD WriteFloat(float aFloat) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WriteFloat(aFloat); } \
  NS_IMETHOD WriteDouble(double aDouble) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WriteDouble(aDouble); } \
  NS_IMETHOD WriteStringZ(const char * aString) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WriteStringZ(aString); } \
  NS_IMETHOD WriteWStringZ(const char16_t * aString) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WriteWStringZ(aString); } \
  NS_IMETHOD WriteUtf8Z(const char16_t * aString) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WriteUtf8Z(aString); } \
  NS_IMETHOD WriteBytesFromJS(const char * aString, uint32_t aLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WriteBytesFromJS(aString, aLength); } \
  virtual nsresult WriteBytes(mozilla::Span<const uint8_t> aBytes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WriteBytes(aBytes); } \
  NS_IMETHOD WriteByteArray(const nsTArray<uint8_t >& aBytes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WriteByteArray(aBytes); } 


inline nsresult
NS_WriteOptionalStringZ(nsIBinaryOutputStream* aStream, const char* aString)
{
    bool nonnull = (aString != nullptr);
    nsresult rv = aStream->WriteBoolean(nonnull);
    if (NS_SUCCEEDED(rv) && nonnull)
        rv = aStream->WriteStringZ(aString);
    return rv;
}
inline nsresult
NS_WriteOptionalWStringZ(nsIBinaryOutputStream* aStream, const char16_t* aString)
{
    bool nonnull = (aString != nullptr);
    nsresult rv = aStream->WriteBoolean(nonnull);
    if (NS_SUCCEEDED(rv) && nonnull)
        rv = aStream->WriteWStringZ(aString);
    return rv;
}

#endif /* __gen_nsIBinaryOutputStream_h__ */
