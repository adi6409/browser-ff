/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/base/nsIWebProgressListener2.idl
 */

#ifndef __gen_nsIWebProgressListener2_h__
#define __gen_nsIWebProgressListener2_h__


#ifndef __gen_nsIWebProgressListener_h__
#include "nsIWebProgressListener.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIWebProgressListener2 */
#define NS_IWEBPROGRESSLISTENER2_IID_STR "dde39de0-e4e0-11da-8ad9-0800200c9a66"

#define NS_IWEBPROGRESSLISTENER2_IID \
  {0xdde39de0, 0xe4e0, 0x11da, \
    { 0x8a, 0xd9, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66 }}

class NS_NO_VTABLE nsIWebProgressListener2 : public nsIWebProgressListener {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWEBPROGRESSLISTENER2_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWebProgressListener2;

  /* void onProgressChange64 (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in long long aCurSelfProgress, in long long aMaxSelfProgress, in long long aCurTotalProgress, in long long aMaxTotalProgress); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnProgressChange64(nsIWebProgress *aWebProgress, nsIRequest *aRequest, int64_t aCurSelfProgress, int64_t aMaxSelfProgress, int64_t aCurTotalProgress, int64_t aMaxTotalProgress) = 0;

  /* boolean onRefreshAttempted (in nsIWebProgress aWebProgress, in nsIURI aRefreshURI, in long aMillis, in boolean aSameURI); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnRefreshAttempted(nsIWebProgress *aWebProgress, nsIURI *aRefreshURI, int32_t aMillis, bool aSameURI, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWebProgressListener2, NS_IWEBPROGRESSLISTENER2_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWEBPROGRESSLISTENER2 \
  NS_IMETHOD OnProgressChange64(nsIWebProgress *aWebProgress, nsIRequest *aRequest, int64_t aCurSelfProgress, int64_t aMaxSelfProgress, int64_t aCurTotalProgress, int64_t aMaxTotalProgress) override; \
  NS_IMETHOD OnRefreshAttempted(nsIWebProgress *aWebProgress, nsIURI *aRefreshURI, int32_t aMillis, bool aSameURI, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWEBPROGRESSLISTENER2 \
  nsresult OnProgressChange64(nsIWebProgress *aWebProgress, nsIRequest *aRequest, int64_t aCurSelfProgress, int64_t aMaxSelfProgress, int64_t aCurTotalProgress, int64_t aMaxTotalProgress); \
  nsresult OnRefreshAttempted(nsIWebProgress *aWebProgress, nsIURI *aRefreshURI, int32_t aMillis, bool aSameURI, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWEBPROGRESSLISTENER2(_to) \
  NS_IMETHOD OnProgressChange64(nsIWebProgress *aWebProgress, nsIRequest *aRequest, int64_t aCurSelfProgress, int64_t aMaxSelfProgress, int64_t aCurTotalProgress, int64_t aMaxTotalProgress) override { return _to OnProgressChange64(aWebProgress, aRequest, aCurSelfProgress, aMaxSelfProgress, aCurTotalProgress, aMaxTotalProgress); } \
  NS_IMETHOD OnRefreshAttempted(nsIWebProgress *aWebProgress, nsIURI *aRefreshURI, int32_t aMillis, bool aSameURI, bool *_retval) override { return _to OnRefreshAttempted(aWebProgress, aRefreshURI, aMillis, aSameURI, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWEBPROGRESSLISTENER2(_to) \
  NS_IMETHOD OnProgressChange64(nsIWebProgress *aWebProgress, nsIRequest *aRequest, int64_t aCurSelfProgress, int64_t aMaxSelfProgress, int64_t aCurTotalProgress, int64_t aMaxTotalProgress) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnProgressChange64(aWebProgress, aRequest, aCurSelfProgress, aMaxSelfProgress, aCurTotalProgress, aMaxTotalProgress); } \
  NS_IMETHOD OnRefreshAttempted(nsIWebProgress *aWebProgress, nsIURI *aRefreshURI, int32_t aMillis, bool aSameURI, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnRefreshAttempted(aWebProgress, aRefreshURI, aMillis, aSameURI, _retval); } 


#endif /* __gen_nsIWebProgressListener2_h__ */
