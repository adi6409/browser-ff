/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIMultiplexInputStream.idl
 */

#ifndef __gen_nsIMultiplexInputStream_h__
#define __gen_nsIMultiplexInputStream_h__


#ifndef __gen_nsIInputStream_h__
#include "nsIInputStream.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIMultiplexInputStream */
#define NS_IMULTIPLEXINPUTSTREAM_IID_STR "a076fd12-1dd1-11b2-b19a-d53b5dffaade"

#define NS_IMULTIPLEXINPUTSTREAM_IID \
  {0xa076fd12, 0x1dd1, 0x11b2, \
    { 0xb1, 0x9a, 0xd5, 0x3b, 0x5d, 0xff, 0xaa, 0xde }}

class NS_NO_VTABLE nsIMultiplexInputStream : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IMULTIPLEXINPUTSTREAM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIMultiplexInputStream;

  /* readonly attribute unsigned long count; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCount(uint32_t *aCount) = 0;

  /* void appendStream (in nsIInputStream stream); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AppendStream(nsIInputStream *stream) = 0;

  /* nsIInputStream getStream (in unsigned long index); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetStream(uint32_t index, nsIInputStream **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIMultiplexInputStream, NS_IMULTIPLEXINPUTSTREAM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIMULTIPLEXINPUTSTREAM \
  NS_IMETHOD GetCount(uint32_t *aCount) override; \
  NS_IMETHOD AppendStream(nsIInputStream *stream) override; \
  NS_IMETHOD GetStream(uint32_t index, nsIInputStream **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIMULTIPLEXINPUTSTREAM \
  nsresult GetCount(uint32_t *aCount); \
  nsresult AppendStream(nsIInputStream *stream); \
  nsresult GetStream(uint32_t index, nsIInputStream **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIMULTIPLEXINPUTSTREAM(_to) \
  NS_IMETHOD GetCount(uint32_t *aCount) override { return _to GetCount(aCount); } \
  NS_IMETHOD AppendStream(nsIInputStream *stream) override { return _to AppendStream(stream); } \
  NS_IMETHOD GetStream(uint32_t index, nsIInputStream **_retval) override { return _to GetStream(index, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIMULTIPLEXINPUTSTREAM(_to) \
  NS_IMETHOD GetCount(uint32_t *aCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCount(aCount); } \
  NS_IMETHOD AppendStream(nsIInputStream *stream) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AppendStream(stream); } \
  NS_IMETHOD GetStream(uint32_t index, nsIInputStream **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStream(index, _retval); } 


#endif /* __gen_nsIMultiplexInputStream_h__ */
