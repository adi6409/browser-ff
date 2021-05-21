/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIAsyncInputStream.idl
 */

#ifndef __gen_nsIAsyncInputStream_h__
#define __gen_nsIAsyncInputStream_h__


#ifndef __gen_nsIInputStream_h__
#include "nsIInputStream.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIInputStreamCallback; /* forward declaration */

class nsIEventTarget; /* forward declaration */


/* starting interface:    nsIAsyncInputStream */
#define NS_IASYNCINPUTSTREAM_IID_STR "a5f255ab-4801-4161-8816-277ac92f6ad1"

#define NS_IASYNCINPUTSTREAM_IID \
  {0xa5f255ab, 0x4801, 0x4161, \
    { 0x88, 0x16, 0x27, 0x7a, 0xc9, 0x2f, 0x6a, 0xd1 }}

class NS_NO_VTABLE nsIAsyncInputStream : public nsIInputStream {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IASYNCINPUTSTREAM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAsyncInputStream;

  /* void closeWithStatus (in nsresult aStatus); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CloseWithStatus(nsresult aStatus) = 0;

  /* void asyncWait (in nsIInputStreamCallback aCallback, in unsigned long aFlags, in unsigned long aRequestedCount, in nsIEventTarget aEventTarget); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncWait(nsIInputStreamCallback *aCallback, uint32_t aFlags, uint32_t aRequestedCount, nsIEventTarget *aEventTarget) = 0;

  enum {
    WAIT_CLOSURE_ONLY = 1U
  };

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAsyncInputStream, NS_IASYNCINPUTSTREAM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIASYNCINPUTSTREAM \
  NS_IMETHOD CloseWithStatus(nsresult aStatus) override; \
  NS_IMETHOD AsyncWait(nsIInputStreamCallback *aCallback, uint32_t aFlags, uint32_t aRequestedCount, nsIEventTarget *aEventTarget) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIASYNCINPUTSTREAM \
  nsresult CloseWithStatus(nsresult aStatus); \
  nsresult AsyncWait(nsIInputStreamCallback *aCallback, uint32_t aFlags, uint32_t aRequestedCount, nsIEventTarget *aEventTarget); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIASYNCINPUTSTREAM(_to) \
  NS_IMETHOD CloseWithStatus(nsresult aStatus) override { return _to CloseWithStatus(aStatus); } \
  NS_IMETHOD AsyncWait(nsIInputStreamCallback *aCallback, uint32_t aFlags, uint32_t aRequestedCount, nsIEventTarget *aEventTarget) override { return _to AsyncWait(aCallback, aFlags, aRequestedCount, aEventTarget); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIASYNCINPUTSTREAM(_to) \
  NS_IMETHOD CloseWithStatus(nsresult aStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CloseWithStatus(aStatus); } \
  NS_IMETHOD AsyncWait(nsIInputStreamCallback *aCallback, uint32_t aFlags, uint32_t aRequestedCount, nsIEventTarget *aEventTarget) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncWait(aCallback, aFlags, aRequestedCount, aEventTarget); } \


/* starting interface:    nsIInputStreamCallback */
#define NS_IINPUTSTREAMCALLBACK_IID_STR "d1f28e94-3a6e-4050-a5f5-2e81b1fc2a43"

#define NS_IINPUTSTREAMCALLBACK_IID \
  {0xd1f28e94, 0x3a6e, 0x4050, \
    { 0xa5, 0xf5, 0x2e, 0x81, 0xb1, 0xfc, 0x2a, 0x43 }}

class NS_NO_VTABLE nsIInputStreamCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IINPUTSTREAMCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIInputStreamCallback;

  /* void onInputStreamReady (in nsIAsyncInputStream aStream); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnInputStreamReady(nsIAsyncInputStream *aStream) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIInputStreamCallback, NS_IINPUTSTREAMCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIINPUTSTREAMCALLBACK \
  NS_IMETHOD OnInputStreamReady(nsIAsyncInputStream *aStream) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIINPUTSTREAMCALLBACK \
  nsresult OnInputStreamReady(nsIAsyncInputStream *aStream); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIINPUTSTREAMCALLBACK(_to) \
  NS_IMETHOD OnInputStreamReady(nsIAsyncInputStream *aStream) override { return _to OnInputStreamReady(aStream); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIINPUTSTREAMCALLBACK(_to) \
  NS_IMETHOD OnInputStreamReady(nsIAsyncInputStream *aStream) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnInputStreamReady(aStream); } 


#endif /* __gen_nsIAsyncInputStream_h__ */
