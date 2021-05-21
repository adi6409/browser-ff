/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIAsyncStreamCopier2.idl
 */

#ifndef __gen_nsIAsyncStreamCopier2_h__
#define __gen_nsIAsyncStreamCopier2_h__


#ifndef __gen_nsIRequest_h__
#include "nsIRequest.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIInputStream; /* forward declaration */

class nsIOutputStream; /* forward declaration */

class nsIRequestObserver; /* forward declaration */

class nsIEventTarget; /* forward declaration */


/* starting interface:    nsIAsyncStreamCopier2 */
#define NS_IASYNCSTREAMCOPIER2_IID_STR "a5b2decf-4ede-4801-8b38-e5fe5db46bf2"

#define NS_IASYNCSTREAMCOPIER2_IID \
  {0xa5b2decf, 0x4ede, 0x4801, \
    { 0x8b, 0x38, 0xe5, 0xfe, 0x5d, 0xb4, 0x6b, 0xf2 }}

class NS_NO_VTABLE nsIAsyncStreamCopier2 : public nsIRequest {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IASYNCSTREAMCOPIER2_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAsyncStreamCopier2;

  /* void init (in nsIInputStream aSource, in nsIOutputStream aSink, in nsIEventTarget aTarget, in unsigned long aChunkSize, in boolean aCloseSource, in boolean aCloseSink); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(nsIInputStream *aSource, nsIOutputStream *aSink, nsIEventTarget *aTarget, uint32_t aChunkSize, bool aCloseSource, bool aCloseSink) = 0;

  /* void asyncCopy (in nsIRequestObserver aObserver, in nsISupports aObserverContext); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncCopy(nsIRequestObserver *aObserver, nsISupports *aObserverContext) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAsyncStreamCopier2, NS_IASYNCSTREAMCOPIER2_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIASYNCSTREAMCOPIER2 \
  NS_IMETHOD Init(nsIInputStream *aSource, nsIOutputStream *aSink, nsIEventTarget *aTarget, uint32_t aChunkSize, bool aCloseSource, bool aCloseSink) override; \
  NS_IMETHOD AsyncCopy(nsIRequestObserver *aObserver, nsISupports *aObserverContext) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIASYNCSTREAMCOPIER2 \
  nsresult Init(nsIInputStream *aSource, nsIOutputStream *aSink, nsIEventTarget *aTarget, uint32_t aChunkSize, bool aCloseSource, bool aCloseSink); \
  nsresult AsyncCopy(nsIRequestObserver *aObserver, nsISupports *aObserverContext); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIASYNCSTREAMCOPIER2(_to) \
  NS_IMETHOD Init(nsIInputStream *aSource, nsIOutputStream *aSink, nsIEventTarget *aTarget, uint32_t aChunkSize, bool aCloseSource, bool aCloseSink) override { return _to Init(aSource, aSink, aTarget, aChunkSize, aCloseSource, aCloseSink); } \
  NS_IMETHOD AsyncCopy(nsIRequestObserver *aObserver, nsISupports *aObserverContext) override { return _to AsyncCopy(aObserver, aObserverContext); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIASYNCSTREAMCOPIER2(_to) \
  NS_IMETHOD Init(nsIInputStream *aSource, nsIOutputStream *aSink, nsIEventTarget *aTarget, uint32_t aChunkSize, bool aCloseSource, bool aCloseSink) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aSource, aSink, aTarget, aChunkSize, aCloseSource, aCloseSink); } \
  NS_IMETHOD AsyncCopy(nsIRequestObserver *aObserver, nsISupports *aObserverContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncCopy(aObserver, aObserverContext); } 


#endif /* __gen_nsIAsyncStreamCopier2_h__ */
