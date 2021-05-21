/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIProgressEventSink.idl
 */

#ifndef __gen_nsIProgressEventSink_h__
#define __gen_nsIProgressEventSink_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class nsIRequest; /* forward declaration */


/* starting interface:    nsIProgressEventSink */
#define NS_IPROGRESSEVENTSINK_IID_STR "87d55fba-cb7e-4f38-84c1-5c6c2b2a55e9"

#define NS_IPROGRESSEVENTSINK_IID \
  {0x87d55fba, 0xcb7e, 0x4f38, \
    { 0x84, 0xc1, 0x5c, 0x6c, 0x2b, 0x2a, 0x55, 0xe9 }}

class NS_NO_VTABLE nsIProgressEventSink : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPROGRESSEVENTSINK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIProgressEventSink;

  /* void onProgress (in nsIRequest aRequest, in long long aProgress, in long long aProgressMax); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnProgress(nsIRequest *aRequest, int64_t aProgress, int64_t aProgressMax) = 0;

  /* void onStatus (in nsIRequest aRequest, in nsresult aStatus, in wstring aStatusArg); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnStatus(nsIRequest *aRequest, nsresult aStatus, const char16_t * aStatusArg) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIProgressEventSink, NS_IPROGRESSEVENTSINK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPROGRESSEVENTSINK \
  NS_IMETHOD OnProgress(nsIRequest *aRequest, int64_t aProgress, int64_t aProgressMax) override; \
  NS_IMETHOD OnStatus(nsIRequest *aRequest, nsresult aStatus, const char16_t * aStatusArg) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPROGRESSEVENTSINK \
  nsresult OnProgress(nsIRequest *aRequest, int64_t aProgress, int64_t aProgressMax); \
  nsresult OnStatus(nsIRequest *aRequest, nsresult aStatus, const char16_t * aStatusArg); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPROGRESSEVENTSINK(_to) \
  NS_IMETHOD OnProgress(nsIRequest *aRequest, int64_t aProgress, int64_t aProgressMax) override { return _to OnProgress(aRequest, aProgress, aProgressMax); } \
  NS_IMETHOD OnStatus(nsIRequest *aRequest, nsresult aStatus, const char16_t * aStatusArg) override { return _to OnStatus(aRequest, aStatus, aStatusArg); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPROGRESSEVENTSINK(_to) \
  NS_IMETHOD OnProgress(nsIRequest *aRequest, int64_t aProgress, int64_t aProgressMax) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnProgress(aRequest, aProgress, aProgressMax); } \
  NS_IMETHOD OnStatus(nsIRequest *aRequest, nsresult aStatus, const char16_t * aStatusArg) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnStatus(aRequest, aStatus, aStatusArg); } 


#endif /* __gen_nsIProgressEventSink_h__ */
