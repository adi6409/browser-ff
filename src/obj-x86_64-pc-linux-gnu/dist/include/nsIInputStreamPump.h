/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIInputStreamPump.idl
 */

#ifndef __gen_nsIInputStreamPump_h__
#define __gen_nsIInputStreamPump_h__


#ifndef __gen_nsIRequest_h__
#include "nsIRequest.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIEventTarget; /* forward declaration */

class nsIInputStream; /* forward declaration */

class nsIStreamListener; /* forward declaration */


/* starting interface:    nsIInputStreamPump */
#define NS_IINPUTSTREAMPUMP_IID_STR "400f5468-97e7-4d2b-9c65-a82aecc7ae82"

#define NS_IINPUTSTREAMPUMP_IID \
  {0x400f5468, 0x97e7, 0x4d2b, \
    { 0x9c, 0x65, 0xa8, 0x2a, 0xec, 0xc7, 0xae, 0x82 }}

class NS_NO_VTABLE nsIInputStreamPump : public nsIRequest {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IINPUTSTREAMPUMP_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIInputStreamPump;

  /* void init (in nsIInputStream aStream, in unsigned long aSegmentSize, in unsigned long aSegmentCount, in boolean aCloseWhenDone, [optional] in nsIEventTarget aMainThreadTarget); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(nsIInputStream *aStream, uint32_t aSegmentSize, uint32_t aSegmentCount, bool aCloseWhenDone, nsIEventTarget *aMainThreadTarget) = 0;

  /* void asyncRead (in nsIStreamListener aListener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncRead(nsIStreamListener *aListener) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIInputStreamPump, NS_IINPUTSTREAMPUMP_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIINPUTSTREAMPUMP \
  NS_IMETHOD Init(nsIInputStream *aStream, uint32_t aSegmentSize, uint32_t aSegmentCount, bool aCloseWhenDone, nsIEventTarget *aMainThreadTarget) override; \
  NS_IMETHOD AsyncRead(nsIStreamListener *aListener) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIINPUTSTREAMPUMP \
  nsresult Init(nsIInputStream *aStream, uint32_t aSegmentSize, uint32_t aSegmentCount, bool aCloseWhenDone, nsIEventTarget *aMainThreadTarget); \
  nsresult AsyncRead(nsIStreamListener *aListener); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIINPUTSTREAMPUMP(_to) \
  NS_IMETHOD Init(nsIInputStream *aStream, uint32_t aSegmentSize, uint32_t aSegmentCount, bool aCloseWhenDone, nsIEventTarget *aMainThreadTarget) override { return _to Init(aStream, aSegmentSize, aSegmentCount, aCloseWhenDone, aMainThreadTarget); } \
  NS_IMETHOD AsyncRead(nsIStreamListener *aListener) override { return _to AsyncRead(aListener); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIINPUTSTREAMPUMP(_to) \
  NS_IMETHOD Init(nsIInputStream *aStream, uint32_t aSegmentSize, uint32_t aSegmentCount, bool aCloseWhenDone, nsIEventTarget *aMainThreadTarget) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aStream, aSegmentSize, aSegmentCount, aCloseWhenDone, aMainThreadTarget); } \
  NS_IMETHOD AsyncRead(nsIStreamListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncRead(aListener); } 


#endif /* __gen_nsIInputStreamPump_h__ */
