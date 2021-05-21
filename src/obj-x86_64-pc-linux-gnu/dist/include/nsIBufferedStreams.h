/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIBufferedStreams.idl
 */

#ifndef __gen_nsIBufferedStreams_h__
#define __gen_nsIBufferedStreams_h__


#ifndef __gen_nsIInputStream_h__
#include "nsIInputStream.h"
#endif

#ifndef __gen_nsIOutputStream_h__
#include "nsIOutputStream.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIBufferedInputStream */
#define NS_IBUFFEREDINPUTSTREAM_IID_STR "616f5b48-da09-11d3-8cda-0060b0fc14a3"

#define NS_IBUFFEREDINPUTSTREAM_IID \
  {0x616f5b48, 0xda09, 0x11d3, \
    { 0x8c, 0xda, 0x00, 0x60, 0xb0, 0xfc, 0x14, 0xa3 }}

class NS_NO_VTABLE nsIBufferedInputStream : public nsIInputStream {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IBUFFEREDINPUTSTREAM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIBufferedInputStream;

  /* void init (in nsIInputStream fillFromStream, in unsigned long bufferSize); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(nsIInputStream *fillFromStream, uint32_t bufferSize) = 0;

  /* readonly attribute nsIInputStream data; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetData(nsIInputStream **aData) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIBufferedInputStream, NS_IBUFFEREDINPUTSTREAM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIBUFFEREDINPUTSTREAM \
  NS_IMETHOD Init(nsIInputStream *fillFromStream, uint32_t bufferSize) override; \
  NS_IMETHOD GetData(nsIInputStream **aData) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIBUFFEREDINPUTSTREAM \
  nsresult Init(nsIInputStream *fillFromStream, uint32_t bufferSize); \
  nsresult GetData(nsIInputStream **aData); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIBUFFEREDINPUTSTREAM(_to) \
  NS_IMETHOD Init(nsIInputStream *fillFromStream, uint32_t bufferSize) override { return _to Init(fillFromStream, bufferSize); } \
  NS_IMETHOD GetData(nsIInputStream **aData) override { return _to GetData(aData); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIBUFFEREDINPUTSTREAM(_to) \
  NS_IMETHOD Init(nsIInputStream *fillFromStream, uint32_t bufferSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(fillFromStream, bufferSize); } \
  NS_IMETHOD GetData(nsIInputStream **aData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetData(aData); } 


/* starting interface:    nsIBufferedOutputStream */
#define NS_IBUFFEREDOUTPUTSTREAM_IID_STR "6476378a-da09-11d3-8cda-0060b0fc14a3"

#define NS_IBUFFEREDOUTPUTSTREAM_IID \
  {0x6476378a, 0xda09, 0x11d3, \
    { 0x8c, 0xda, 0x00, 0x60, 0xb0, 0xfc, 0x14, 0xa3 }}

class NS_NO_VTABLE nsIBufferedOutputStream : public nsIOutputStream {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IBUFFEREDOUTPUTSTREAM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIBufferedOutputStream;

  /* void init (in nsIOutputStream sinkToStream, in unsigned long bufferSize); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(nsIOutputStream *sinkToStream, uint32_t bufferSize) = 0;

  /* readonly attribute nsIOutputStream data; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetData(nsIOutputStream **aData) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIBufferedOutputStream, NS_IBUFFEREDOUTPUTSTREAM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIBUFFEREDOUTPUTSTREAM \
  NS_IMETHOD Init(nsIOutputStream *sinkToStream, uint32_t bufferSize) override; \
  NS_IMETHOD GetData(nsIOutputStream **aData) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIBUFFEREDOUTPUTSTREAM \
  nsresult Init(nsIOutputStream *sinkToStream, uint32_t bufferSize); \
  nsresult GetData(nsIOutputStream **aData); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIBUFFEREDOUTPUTSTREAM(_to) \
  NS_IMETHOD Init(nsIOutputStream *sinkToStream, uint32_t bufferSize) override { return _to Init(sinkToStream, bufferSize); } \
  NS_IMETHOD GetData(nsIOutputStream **aData) override { return _to GetData(aData); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIBUFFEREDOUTPUTSTREAM(_to) \
  NS_IMETHOD Init(nsIOutputStream *sinkToStream, uint32_t bufferSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(sinkToStream, bufferSize); } \
  NS_IMETHOD GetData(nsIOutputStream **aData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetData(aData); } 


#endif /* __gen_nsIBufferedStreams_h__ */
