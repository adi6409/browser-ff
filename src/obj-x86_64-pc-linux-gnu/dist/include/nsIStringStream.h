/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIStringStream.idl
 */

#ifndef __gen_nsIStringStream_h__
#define __gen_nsIStringStream_h__


#ifndef __gen_nsIInputStream_h__
#include "nsIInputStream.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#include "mozilla/MemoryReporting.h"

/* starting interface:    nsIStringInputStream */
#define NS_ISTRINGINPUTSTREAM_IID_STR "450cd2d4-f0fd-424d-b365-b1251f80fd53"

#define NS_ISTRINGINPUTSTREAM_IID \
  {0x450cd2d4, 0xf0fd, 0x424d, \
    { 0xb3, 0x65, 0xb1, 0x25, 0x1f, 0x80, 0xfd, 0x53 }}

class NS_NO_VTABLE nsIStringInputStream : public nsIInputStream {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISTRINGINPUTSTREAM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIStringInputStream;

  /* void setData (in string data, in long dataLen); */
  NS_IMETHOD SetData(const char * data, int32_t dataLen) = 0;

  /* void setUTF8Data (in AUTF8String data); */
  NS_IMETHOD SetUTF8Data(const nsACString& data) = 0;

  /* [noscript] void adoptData (in charPtr data, in long dataLen); */
  NS_IMETHOD AdoptData(char * data, int32_t dataLen) = 0;

  /* [noscript] void shareData (in string data, in long dataLen); */
  NS_IMETHOD ShareData(const char * data, int32_t dataLen) = 0;

  /* [noscript,notxpcom] size_t SizeOfIncludingThisIfUnshared (in MallocSizeOf aMallocSizeOf); */
  NS_IMETHOD_(size_t) SizeOfIncludingThisIfUnshared(mozilla::MallocSizeOf aMallocSizeOf) = 0;

  /* [noscript,notxpcom] size_t SizeOfIncludingThisEvenIfShared (in MallocSizeOf aMallocSizeOf); */
  NS_IMETHOD_(size_t) SizeOfIncludingThisEvenIfShared(mozilla::MallocSizeOf aMallocSizeOf) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIStringInputStream, NS_ISTRINGINPUTSTREAM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISTRINGINPUTSTREAM \
  NS_IMETHOD SetData(const char * data, int32_t dataLen) override; \
  NS_IMETHOD SetUTF8Data(const nsACString& data) override; \
  NS_IMETHOD AdoptData(char * data, int32_t dataLen) override; \
  NS_IMETHOD ShareData(const char * data, int32_t dataLen) override; \
  NS_IMETHOD_(size_t) SizeOfIncludingThisIfUnshared(mozilla::MallocSizeOf aMallocSizeOf) override; \
  NS_IMETHOD_(size_t) SizeOfIncludingThisEvenIfShared(mozilla::MallocSizeOf aMallocSizeOf) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISTRINGINPUTSTREAM \
  nsresult SetData(const char * data, int32_t dataLen); \
  nsresult SetUTF8Data(const nsACString& data); \
  nsresult AdoptData(char * data, int32_t dataLen); \
  nsresult ShareData(const char * data, int32_t dataLen); \
  nsresult_(size_t) SizeOfIncludingThisIfUnshared(mozilla::MallocSizeOf aMallocSizeOf); \
  nsresult_(size_t) SizeOfIncludingThisEvenIfShared(mozilla::MallocSizeOf aMallocSizeOf); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISTRINGINPUTSTREAM(_to) \
  NS_IMETHOD SetData(const char * data, int32_t dataLen) override { return _to SetData(data, dataLen); } \
  NS_IMETHOD SetUTF8Data(const nsACString& data) override { return _to SetUTF8Data(data); } \
  NS_IMETHOD AdoptData(char * data, int32_t dataLen) override { return _to AdoptData(data, dataLen); } \
  NS_IMETHOD ShareData(const char * data, int32_t dataLen) override { return _to ShareData(data, dataLen); } \
  NS_IMETHOD_(size_t) SizeOfIncludingThisIfUnshared(mozilla::MallocSizeOf aMallocSizeOf) override { return _to SizeOfIncludingThisIfUnshared(aMallocSizeOf); } \
  NS_IMETHOD_(size_t) SizeOfIncludingThisEvenIfShared(mozilla::MallocSizeOf aMallocSizeOf) override { return _to SizeOfIncludingThisEvenIfShared(aMallocSizeOf); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISTRINGINPUTSTREAM(_to) \
  NS_IMETHOD SetData(const char * data, int32_t dataLen) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetData(data, dataLen); } \
  NS_IMETHOD SetUTF8Data(const nsACString& data) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetUTF8Data(data); } \
  NS_IMETHOD AdoptData(char * data, int32_t dataLen) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AdoptData(data, dataLen); } \
  NS_IMETHOD ShareData(const char * data, int32_t dataLen) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ShareData(data, dataLen); } \
  NS_IMETHOD_(size_t) SizeOfIncludingThisIfUnshared(mozilla::MallocSizeOf aMallocSizeOf) override; \
  NS_IMETHOD_(size_t) SizeOfIncludingThisEvenIfShared(mozilla::MallocSizeOf aMallocSizeOf) override; 


#endif /* __gen_nsIStringStream_h__ */
