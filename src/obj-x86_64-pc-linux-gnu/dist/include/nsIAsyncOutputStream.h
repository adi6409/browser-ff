/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIAsyncOutputStream.idl
 */

#ifndef __gen_nsIAsyncOutputStream_h__
#define __gen_nsIAsyncOutputStream_h__


#ifndef __gen_nsIOutputStream_h__
#include "nsIOutputStream.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIOutputStreamCallback; /* forward declaration */

class nsIEventTarget; /* forward declaration */


/* starting interface:    nsIAsyncOutputStream */
#define NS_IASYNCOUTPUTSTREAM_IID_STR "beb632d3-d77a-4e90-9134-f9ece69e8200"

#define NS_IASYNCOUTPUTSTREAM_IID \
  {0xbeb632d3, 0xd77a, 0x4e90, \
    { 0x91, 0x34, 0xf9, 0xec, 0xe6, 0x9e, 0x82, 0x00 }}

class NS_NO_VTABLE nsIAsyncOutputStream : public nsIOutputStream {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IASYNCOUTPUTSTREAM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAsyncOutputStream;

  /* void closeWithStatus (in nsresult reason); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CloseWithStatus(nsresult reason) = 0;

  /* void asyncWait (in nsIOutputStreamCallback aCallback, in unsigned long aFlags, in unsigned long aRequestedCount, in nsIEventTarget aEventTarget); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncWait(nsIOutputStreamCallback *aCallback, uint32_t aFlags, uint32_t aRequestedCount, nsIEventTarget *aEventTarget) = 0;

  enum {
    WAIT_CLOSURE_ONLY = 1U
  };

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAsyncOutputStream, NS_IASYNCOUTPUTSTREAM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIASYNCOUTPUTSTREAM \
  NS_IMETHOD CloseWithStatus(nsresult reason) override; \
  NS_IMETHOD AsyncWait(nsIOutputStreamCallback *aCallback, uint32_t aFlags, uint32_t aRequestedCount, nsIEventTarget *aEventTarget) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIASYNCOUTPUTSTREAM \
  nsresult CloseWithStatus(nsresult reason); \
  nsresult AsyncWait(nsIOutputStreamCallback *aCallback, uint32_t aFlags, uint32_t aRequestedCount, nsIEventTarget *aEventTarget); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIASYNCOUTPUTSTREAM(_to) \
  NS_IMETHOD CloseWithStatus(nsresult reason) override { return _to CloseWithStatus(reason); } \
  NS_IMETHOD AsyncWait(nsIOutputStreamCallback *aCallback, uint32_t aFlags, uint32_t aRequestedCount, nsIEventTarget *aEventTarget) override { return _to AsyncWait(aCallback, aFlags, aRequestedCount, aEventTarget); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIASYNCOUTPUTSTREAM(_to) \
  NS_IMETHOD CloseWithStatus(nsresult reason) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CloseWithStatus(reason); } \
  NS_IMETHOD AsyncWait(nsIOutputStreamCallback *aCallback, uint32_t aFlags, uint32_t aRequestedCount, nsIEventTarget *aEventTarget) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncWait(aCallback, aFlags, aRequestedCount, aEventTarget); } \


/* starting interface:    nsIOutputStreamCallback */
#define NS_IOUTPUTSTREAMCALLBACK_IID_STR "40dbcdff-9053-42c5-a57c-3ec910d0f148"

#define NS_IOUTPUTSTREAMCALLBACK_IID \
  {0x40dbcdff, 0x9053, 0x42c5, \
    { 0xa5, 0x7c, 0x3e, 0xc9, 0x10, 0xd0, 0xf1, 0x48 }}

class NS_NO_VTABLE nsIOutputStreamCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IOUTPUTSTREAMCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIOutputStreamCallback;

  /* void onOutputStreamReady (in nsIAsyncOutputStream aStream); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnOutputStreamReady(nsIAsyncOutputStream *aStream) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIOutputStreamCallback, NS_IOUTPUTSTREAMCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIOUTPUTSTREAMCALLBACK \
  NS_IMETHOD OnOutputStreamReady(nsIAsyncOutputStream *aStream) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIOUTPUTSTREAMCALLBACK \
  nsresult OnOutputStreamReady(nsIAsyncOutputStream *aStream); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIOUTPUTSTREAMCALLBACK(_to) \
  NS_IMETHOD OnOutputStreamReady(nsIAsyncOutputStream *aStream) override { return _to OnOutputStreamReady(aStream); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIOUTPUTSTREAMCALLBACK(_to) \
  NS_IMETHOD OnOutputStreamReady(nsIAsyncOutputStream *aStream) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnOutputStreamReady(aStream); } 


#endif /* __gen_nsIAsyncOutputStream_h__ */
