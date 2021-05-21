/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIIncrementalDownload.idl
 */

#ifndef __gen_nsIIncrementalDownload_h__
#define __gen_nsIIncrementalDownload_h__


#ifndef __gen_nsIRequest_h__
#include "nsIRequest.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class nsIFile; /* forward declaration */

class nsIRequestObserver; /* forward declaration */


/* starting interface:    nsIIncrementalDownload */
#define NS_IINCREMENTALDOWNLOAD_IID_STR "6687823f-56c4-461d-93a1-7f6cb7dfbfba"

#define NS_IINCREMENTALDOWNLOAD_IID \
  {0x6687823f, 0x56c4, 0x461d, \
    { 0x93, 0xa1, 0x7f, 0x6c, 0xb7, 0xdf, 0xbf, 0xba }}

class NS_NO_VTABLE nsIIncrementalDownload : public nsIRequest {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IINCREMENTALDOWNLOAD_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIIncrementalDownload;

  /* void init (in nsIURI uri, in nsIFile destination, in long chunkSize, in long intervalInSeconds); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(nsIURI *uri, nsIFile *destination, int32_t chunkSize, int32_t intervalInSeconds) = 0;

  /* readonly attribute nsIURI URI; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetURI(nsIURI **aURI) = 0;

  /* readonly attribute nsIURI finalURI; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFinalURI(nsIURI **aFinalURI) = 0;

  /* readonly attribute nsIFile destination; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDestination(nsIFile **aDestination) = 0;

  /* readonly attribute long long totalSize; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTotalSize(int64_t *aTotalSize) = 0;

  /* readonly attribute long long currentSize; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCurrentSize(int64_t *aCurrentSize) = 0;

  /* void start (in nsIRequestObserver observer, in nsISupports ctxt); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Start(nsIRequestObserver *observer, nsISupports *ctxt) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIIncrementalDownload, NS_IINCREMENTALDOWNLOAD_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIINCREMENTALDOWNLOAD \
  NS_IMETHOD Init(nsIURI *uri, nsIFile *destination, int32_t chunkSize, int32_t intervalInSeconds) override; \
  NS_IMETHOD GetURI(nsIURI **aURI) override; \
  NS_IMETHOD GetFinalURI(nsIURI **aFinalURI) override; \
  NS_IMETHOD GetDestination(nsIFile **aDestination) override; \
  NS_IMETHOD GetTotalSize(int64_t *aTotalSize) override; \
  NS_IMETHOD GetCurrentSize(int64_t *aCurrentSize) override; \
  NS_IMETHOD Start(nsIRequestObserver *observer, nsISupports *ctxt) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIINCREMENTALDOWNLOAD \
  nsresult Init(nsIURI *uri, nsIFile *destination, int32_t chunkSize, int32_t intervalInSeconds); \
  nsresult GetURI(nsIURI **aURI); \
  nsresult GetFinalURI(nsIURI **aFinalURI); \
  nsresult GetDestination(nsIFile **aDestination); \
  nsresult GetTotalSize(int64_t *aTotalSize); \
  nsresult GetCurrentSize(int64_t *aCurrentSize); \
  nsresult Start(nsIRequestObserver *observer, nsISupports *ctxt); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIINCREMENTALDOWNLOAD(_to) \
  NS_IMETHOD Init(nsIURI *uri, nsIFile *destination, int32_t chunkSize, int32_t intervalInSeconds) override { return _to Init(uri, destination, chunkSize, intervalInSeconds); } \
  NS_IMETHOD GetURI(nsIURI **aURI) override { return _to GetURI(aURI); } \
  NS_IMETHOD GetFinalURI(nsIURI **aFinalURI) override { return _to GetFinalURI(aFinalURI); } \
  NS_IMETHOD GetDestination(nsIFile **aDestination) override { return _to GetDestination(aDestination); } \
  NS_IMETHOD GetTotalSize(int64_t *aTotalSize) override { return _to GetTotalSize(aTotalSize); } \
  NS_IMETHOD GetCurrentSize(int64_t *aCurrentSize) override { return _to GetCurrentSize(aCurrentSize); } \
  NS_IMETHOD Start(nsIRequestObserver *observer, nsISupports *ctxt) override { return _to Start(observer, ctxt); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIINCREMENTALDOWNLOAD(_to) \
  NS_IMETHOD Init(nsIURI *uri, nsIFile *destination, int32_t chunkSize, int32_t intervalInSeconds) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(uri, destination, chunkSize, intervalInSeconds); } \
  NS_IMETHOD GetURI(nsIURI **aURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetURI(aURI); } \
  NS_IMETHOD GetFinalURI(nsIURI **aFinalURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFinalURI(aFinalURI); } \
  NS_IMETHOD GetDestination(nsIFile **aDestination) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDestination(aDestination); } \
  NS_IMETHOD GetTotalSize(int64_t *aTotalSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTotalSize(aTotalSize); } \
  NS_IMETHOD GetCurrentSize(int64_t *aCurrentSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCurrentSize(aCurrentSize); } \
  NS_IMETHOD Start(nsIRequestObserver *observer, nsISupports *ctxt) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Start(observer, ctxt); } 


#endif /* __gen_nsIIncrementalDownload_h__ */
