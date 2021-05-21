/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIStreamLoader.idl
 */

#ifndef __gen_nsIStreamLoader_h__
#define __gen_nsIStreamLoader_h__


#ifndef __gen_nsIStreamListener_h__
#include "nsIStreamListener.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIRequest; /* forward declaration */

class nsIStreamLoader; /* forward declaration */


/* starting interface:    nsIStreamLoaderObserver */
#define NS_ISTREAMLOADEROBSERVER_IID_STR "359f7990-d4e9-11d3-a1a5-0050041caf44"

#define NS_ISTREAMLOADEROBSERVER_IID \
  {0x359f7990, 0xd4e9, 0x11d3, \
    { 0xa1, 0xa5, 0x00, 0x50, 0x04, 0x1c, 0xaf, 0x44 }}

class NS_NO_VTABLE nsIStreamLoaderObserver : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISTREAMLOADEROBSERVER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIStreamLoaderObserver;

  /* void onStreamComplete (in nsIStreamLoader loader, in nsISupports ctxt, in nsresult status, in unsigned long resultLength, [array, size_is (resultLength), const] in octet result); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnStreamComplete(nsIStreamLoader *loader, nsISupports *ctxt, nsresult status, uint32_t resultLength, const uint8_t *result) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIStreamLoaderObserver, NS_ISTREAMLOADEROBSERVER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISTREAMLOADEROBSERVER \
  NS_IMETHOD OnStreamComplete(nsIStreamLoader *loader, nsISupports *ctxt, nsresult status, uint32_t resultLength, const uint8_t *result) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISTREAMLOADEROBSERVER \
  nsresult OnStreamComplete(nsIStreamLoader *loader, nsISupports *ctxt, nsresult status, uint32_t resultLength, const uint8_t *result); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISTREAMLOADEROBSERVER(_to) \
  NS_IMETHOD OnStreamComplete(nsIStreamLoader *loader, nsISupports *ctxt, nsresult status, uint32_t resultLength, const uint8_t *result) override { return _to OnStreamComplete(loader, ctxt, status, resultLength, result); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISTREAMLOADEROBSERVER(_to) \
  NS_IMETHOD OnStreamComplete(nsIStreamLoader *loader, nsISupports *ctxt, nsresult status, uint32_t resultLength, const uint8_t *result) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnStreamComplete(loader, ctxt, status, resultLength, result); } 


/* starting interface:    nsIStreamLoader */
#define NS_ISTREAMLOADER_IID_STR "323bcff1-7513-4e1f-a541-1c9213c2ed1b"

#define NS_ISTREAMLOADER_IID \
  {0x323bcff1, 0x7513, 0x4e1f, \
    { 0xa5, 0x41, 0x1c, 0x92, 0x13, 0xc2, 0xed, 0x1b }}

class NS_NO_VTABLE nsIStreamLoader : public nsIStreamListener {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISTREAMLOADER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIStreamLoader;

  /* void init (in nsIStreamLoaderObserver aStreamObserver, [optional] in nsIRequestObserver aRequestObserver); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(nsIStreamLoaderObserver *aStreamObserver, nsIRequestObserver *aRequestObserver) = 0;

  /* readonly attribute unsigned long numBytesRead; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNumBytesRead(uint32_t *aNumBytesRead) = 0;

  /* readonly attribute nsIRequest request; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRequest(nsIRequest **aRequest) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIStreamLoader, NS_ISTREAMLOADER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISTREAMLOADER \
  NS_IMETHOD Init(nsIStreamLoaderObserver *aStreamObserver, nsIRequestObserver *aRequestObserver) override; \
  NS_IMETHOD GetNumBytesRead(uint32_t *aNumBytesRead) override; \
  NS_IMETHOD GetRequest(nsIRequest **aRequest) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISTREAMLOADER \
  nsresult Init(nsIStreamLoaderObserver *aStreamObserver, nsIRequestObserver *aRequestObserver); \
  nsresult GetNumBytesRead(uint32_t *aNumBytesRead); \
  nsresult GetRequest(nsIRequest **aRequest); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISTREAMLOADER(_to) \
  NS_IMETHOD Init(nsIStreamLoaderObserver *aStreamObserver, nsIRequestObserver *aRequestObserver) override { return _to Init(aStreamObserver, aRequestObserver); } \
  NS_IMETHOD GetNumBytesRead(uint32_t *aNumBytesRead) override { return _to GetNumBytesRead(aNumBytesRead); } \
  NS_IMETHOD GetRequest(nsIRequest **aRequest) override { return _to GetRequest(aRequest); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISTREAMLOADER(_to) \
  NS_IMETHOD Init(nsIStreamLoaderObserver *aStreamObserver, nsIRequestObserver *aRequestObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aStreamObserver, aRequestObserver); } \
  NS_IMETHOD GetNumBytesRead(uint32_t *aNumBytesRead) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNumBytesRead(aNumBytesRead); } \
  NS_IMETHOD GetRequest(nsIRequest **aRequest) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRequest(aRequest); } 


#endif /* __gen_nsIStreamLoader_h__ */
