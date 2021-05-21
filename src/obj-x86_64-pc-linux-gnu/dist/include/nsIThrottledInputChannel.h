/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIThrottledInputChannel.idl
 */

#ifndef __gen_nsIThrottledInputChannel_h__
#define __gen_nsIThrottledInputChannel_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIInputStream; /* forward declaration */

class nsIAsyncInputStream; /* forward declaration */


/* starting interface:    nsIInputChannelThrottleQueue */
#define NS_IINPUTCHANNELTHROTTLEQUEUE_IID_STR "6b4b96fe-3c67-4587-af7b-58b6b17da411"

#define NS_IINPUTCHANNELTHROTTLEQUEUE_IID \
  {0x6b4b96fe, 0x3c67, 0x4587, \
    { 0xaf, 0x7b, 0x58, 0xb6, 0xb1, 0x7d, 0xa4, 0x11 }}

class NS_NO_VTABLE nsIInputChannelThrottleQueue : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IINPUTCHANNELTHROTTLEQUEUE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIInputChannelThrottleQueue;

  /* void init (in unsigned long aMeanBytesPerSecond, in unsigned long aMaxBytesPerSecond); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(uint32_t aMeanBytesPerSecond, uint32_t aMaxBytesPerSecond) = 0;

  /* [noscript] readonly attribute unsigned long meanBytesPerSecond; */
  NS_IMETHOD GetMeanBytesPerSecond(uint32_t *aMeanBytesPerSecond) = 0;

  /* [noscript] readonly attribute unsigned long maxBytesPerSecond; */
  NS_IMETHOD GetMaxBytesPerSecond(uint32_t *aMaxBytesPerSecond) = 0;

  /* unsigned long available (in unsigned long aRemaining); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Available(uint32_t aRemaining, uint32_t *_retval) = 0;

  /* void recordRead (in unsigned long aBytesRead); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RecordRead(uint32_t aBytesRead) = 0;

  /* unsigned long long bytesProcessed (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD BytesProcessed(uint64_t *_retval) = 0;

  /* nsIAsyncInputStream wrapStream (in nsIInputStream aInputStream); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD WrapStream(nsIInputStream *aInputStream, nsIAsyncInputStream **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIInputChannelThrottleQueue, NS_IINPUTCHANNELTHROTTLEQUEUE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIINPUTCHANNELTHROTTLEQUEUE \
  NS_IMETHOD Init(uint32_t aMeanBytesPerSecond, uint32_t aMaxBytesPerSecond) override; \
  NS_IMETHOD GetMeanBytesPerSecond(uint32_t *aMeanBytesPerSecond) override; \
  NS_IMETHOD GetMaxBytesPerSecond(uint32_t *aMaxBytesPerSecond) override; \
  NS_IMETHOD Available(uint32_t aRemaining, uint32_t *_retval) override; \
  NS_IMETHOD RecordRead(uint32_t aBytesRead) override; \
  NS_IMETHOD BytesProcessed(uint64_t *_retval) override; \
  NS_IMETHOD WrapStream(nsIInputStream *aInputStream, nsIAsyncInputStream **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIINPUTCHANNELTHROTTLEQUEUE \
  nsresult Init(uint32_t aMeanBytesPerSecond, uint32_t aMaxBytesPerSecond); \
  nsresult GetMeanBytesPerSecond(uint32_t *aMeanBytesPerSecond); \
  nsresult GetMaxBytesPerSecond(uint32_t *aMaxBytesPerSecond); \
  nsresult Available(uint32_t aRemaining, uint32_t *_retval); \
  nsresult RecordRead(uint32_t aBytesRead); \
  nsresult BytesProcessed(uint64_t *_retval); \
  nsresult WrapStream(nsIInputStream *aInputStream, nsIAsyncInputStream **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIINPUTCHANNELTHROTTLEQUEUE(_to) \
  NS_IMETHOD Init(uint32_t aMeanBytesPerSecond, uint32_t aMaxBytesPerSecond) override { return _to Init(aMeanBytesPerSecond, aMaxBytesPerSecond); } \
  NS_IMETHOD GetMeanBytesPerSecond(uint32_t *aMeanBytesPerSecond) override { return _to GetMeanBytesPerSecond(aMeanBytesPerSecond); } \
  NS_IMETHOD GetMaxBytesPerSecond(uint32_t *aMaxBytesPerSecond) override { return _to GetMaxBytesPerSecond(aMaxBytesPerSecond); } \
  NS_IMETHOD Available(uint32_t aRemaining, uint32_t *_retval) override { return _to Available(aRemaining, _retval); } \
  NS_IMETHOD RecordRead(uint32_t aBytesRead) override { return _to RecordRead(aBytesRead); } \
  NS_IMETHOD BytesProcessed(uint64_t *_retval) override { return _to BytesProcessed(_retval); } \
  NS_IMETHOD WrapStream(nsIInputStream *aInputStream, nsIAsyncInputStream **_retval) override { return _to WrapStream(aInputStream, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIINPUTCHANNELTHROTTLEQUEUE(_to) \
  NS_IMETHOD Init(uint32_t aMeanBytesPerSecond, uint32_t aMaxBytesPerSecond) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aMeanBytesPerSecond, aMaxBytesPerSecond); } \
  NS_IMETHOD GetMeanBytesPerSecond(uint32_t *aMeanBytesPerSecond) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMeanBytesPerSecond(aMeanBytesPerSecond); } \
  NS_IMETHOD GetMaxBytesPerSecond(uint32_t *aMaxBytesPerSecond) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMaxBytesPerSecond(aMaxBytesPerSecond); } \
  NS_IMETHOD Available(uint32_t aRemaining, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Available(aRemaining, _retval); } \
  NS_IMETHOD RecordRead(uint32_t aBytesRead) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RecordRead(aBytesRead); } \
  NS_IMETHOD BytesProcessed(uint64_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->BytesProcessed(_retval); } \
  NS_IMETHOD WrapStream(nsIInputStream *aInputStream, nsIAsyncInputStream **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WrapStream(aInputStream, _retval); } 


/* starting interface:    nsIThrottledInputChannel */
#define NS_ITHROTTLEDINPUTCHANNEL_IID_STR "0a32a100-c031-45b6-9e8b-0444c7d4a143"

#define NS_ITHROTTLEDINPUTCHANNEL_IID \
  {0x0a32a100, 0xc031, 0x45b6, \
    { 0x9e, 0x8b, 0x04, 0x44, 0xc7, 0xd4, 0xa1, 0x43 }}

class NS_NO_VTABLE nsIThrottledInputChannel : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITHROTTLEDINPUTCHANNEL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIThrottledInputChannel;

  /* attribute nsIInputChannelThrottleQueue throttleQueue; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetThrottleQueue(nsIInputChannelThrottleQueue **aThrottleQueue) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetThrottleQueue(nsIInputChannelThrottleQueue *aThrottleQueue) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIThrottledInputChannel, NS_ITHROTTLEDINPUTCHANNEL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITHROTTLEDINPUTCHANNEL \
  NS_IMETHOD GetThrottleQueue(nsIInputChannelThrottleQueue **aThrottleQueue) override; \
  NS_IMETHOD SetThrottleQueue(nsIInputChannelThrottleQueue *aThrottleQueue) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITHROTTLEDINPUTCHANNEL \
  nsresult GetThrottleQueue(nsIInputChannelThrottleQueue **aThrottleQueue); \
  nsresult SetThrottleQueue(nsIInputChannelThrottleQueue *aThrottleQueue); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITHROTTLEDINPUTCHANNEL(_to) \
  NS_IMETHOD GetThrottleQueue(nsIInputChannelThrottleQueue **aThrottleQueue) override { return _to GetThrottleQueue(aThrottleQueue); } \
  NS_IMETHOD SetThrottleQueue(nsIInputChannelThrottleQueue *aThrottleQueue) override { return _to SetThrottleQueue(aThrottleQueue); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITHROTTLEDINPUTCHANNEL(_to) \
  NS_IMETHOD GetThrottleQueue(nsIInputChannelThrottleQueue **aThrottleQueue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetThrottleQueue(aThrottleQueue); } \
  NS_IMETHOD SetThrottleQueue(nsIInputChannelThrottleQueue *aThrottleQueue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetThrottleQueue(aThrottleQueue); } 


#endif /* __gen_nsIThrottledInputChannel_h__ */
