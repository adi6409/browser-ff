/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIAsyncStreamCopier.idl
 */

#ifndef __gen_nsIAsyncStreamCopier_h__
#define __gen_nsIAsyncStreamCopier_h__


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


/* starting interface:    nsIAsyncStreamCopier */
#define NS_IASYNCSTREAMCOPIER_IID_STR "5a19ca27-e041-4aca-8287-eb248d4c50c0"

#define NS_IASYNCSTREAMCOPIER_IID \
  {0x5a19ca27, 0xe041, 0x4aca, \
    { 0x82, 0x87, 0xeb, 0x24, 0x8d, 0x4c, 0x50, 0xc0 }}

class NS_NO_VTABLE nsIAsyncStreamCopier : public nsIRequest {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IASYNCSTREAMCOPIER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAsyncStreamCopier;

  /* void init (in nsIInputStream aSource, in nsIOutputStream aSink, in nsIEventTarget aTarget, in boolean aSourceBuffered, in boolean aSinkBuffered, in unsigned long aChunkSize, in boolean aCloseSource, in boolean aCloseSink); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(nsIInputStream *aSource, nsIOutputStream *aSink, nsIEventTarget *aTarget, bool aSourceBuffered, bool aSinkBuffered, uint32_t aChunkSize, bool aCloseSource, bool aCloseSink) = 0;

  /* void asyncCopy (in nsIRequestObserver aObserver, in nsISupports aObserverContext); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncCopy(nsIRequestObserver *aObserver, nsISupports *aObserverContext) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAsyncStreamCopier, NS_IASYNCSTREAMCOPIER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIASYNCSTREAMCOPIER \
  NS_IMETHOD Init(nsIInputStream *aSource, nsIOutputStream *aSink, nsIEventTarget *aTarget, bool aSourceBuffered, bool aSinkBuffered, uint32_t aChunkSize, bool aCloseSource, bool aCloseSink) override; \
  NS_IMETHOD AsyncCopy(nsIRequestObserver *aObserver, nsISupports *aObserverContext) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIASYNCSTREAMCOPIER \
  nsresult Init(nsIInputStream *aSource, nsIOutputStream *aSink, nsIEventTarget *aTarget, bool aSourceBuffered, bool aSinkBuffered, uint32_t aChunkSize, bool aCloseSource, bool aCloseSink); \
  nsresult AsyncCopy(nsIRequestObserver *aObserver, nsISupports *aObserverContext); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIASYNCSTREAMCOPIER(_to) \
  NS_IMETHOD Init(nsIInputStream *aSource, nsIOutputStream *aSink, nsIEventTarget *aTarget, bool aSourceBuffered, bool aSinkBuffered, uint32_t aChunkSize, bool aCloseSource, bool aCloseSink) override { return _to Init(aSource, aSink, aTarget, aSourceBuffered, aSinkBuffered, aChunkSize, aCloseSource, aCloseSink); } \
  NS_IMETHOD AsyncCopy(nsIRequestObserver *aObserver, nsISupports *aObserverContext) override { return _to AsyncCopy(aObserver, aObserverContext); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIASYNCSTREAMCOPIER(_to) \
  NS_IMETHOD Init(nsIInputStream *aSource, nsIOutputStream *aSink, nsIEventTarget *aTarget, bool aSourceBuffered, bool aSinkBuffered, uint32_t aChunkSize, bool aCloseSource, bool aCloseSink) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aSource, aSink, aTarget, aSourceBuffered, aSinkBuffered, aChunkSize, aCloseSource, aCloseSink); } \
  NS_IMETHOD AsyncCopy(nsIRequestObserver *aObserver, nsISupports *aObserverContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncCopy(aObserver, aObserverContext); } 


#endif /* __gen_nsIAsyncStreamCopier_h__ */
