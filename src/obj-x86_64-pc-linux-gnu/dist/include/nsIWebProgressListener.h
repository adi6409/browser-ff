/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/base/nsIWebProgressListener.idl
 */

#ifndef __gen_nsIWebProgressListener_h__
#define __gen_nsIWebProgressListener_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIWebProgress; /* forward declaration */

class nsIRequest; /* forward declaration */

class nsIURI; /* forward declaration */


/* starting interface:    nsIWebProgressListener */
#define NS_IWEBPROGRESSLISTENER_IID_STR "a9df523b-efe2-421e-9d8e-3d7f807dda4c"

#define NS_IWEBPROGRESSLISTENER_IID \
  {0xa9df523b, 0xefe2, 0x421e, \
    { 0x9d, 0x8e, 0x3d, 0x7f, 0x80, 0x7d, 0xda, 0x4c }}

class NS_NO_VTABLE nsIWebProgressListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWEBPROGRESSLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWebProgressListener;

  enum {
    STATE_START = 1U,
    STATE_REDIRECTING = 2U,
    STATE_TRANSFERRING = 4U,
    STATE_NEGOTIATING = 8U,
    STATE_STOP = 16U,
    STATE_IS_REQUEST = 65536U,
    STATE_IS_DOCUMENT = 131072U,
    STATE_IS_NETWORK = 262144U,
    STATE_IS_WINDOW = 524288U,
    STATE_IS_REDIRECTED_DOCUMENT = 1048576U,
    STATE_RESTORING = 16777216U,
    STATE_IS_INSECURE = 4U,
    STATE_IS_BROKEN = 1U,
    STATE_IS_SECURE = 2U,
    STATE_BLOCKED_MIXED_ACTIVE_CONTENT = 16U,
    STATE_LOADED_MIXED_ACTIVE_CONTENT = 32U,
    STATE_BLOCKED_MIXED_DISPLAY_CONTENT = 256U,
    STATE_LOADED_MIXED_DISPLAY_CONTENT = 512U,
    STATE_IDENTITY_EV_TOPLEVEL = 1048576U,
    STATE_USES_SSL_3 = 16777216U,
    STATE_USES_WEAK_CRYPTO = 33554432U,
    STATE_CERT_USER_OVERRIDDEN = 67108864U,
    STATE_BLOCKED_TRACKING_CONTENT = 4096U,
    STATE_LOADED_LEVEL_1_TRACKING_CONTENT = 8192U,
    STATE_LOADED_LEVEL_2_TRACKING_CONTENT = 1048576U,
    STATE_BLOCKED_FINGERPRINTING_CONTENT = 64U,
    STATE_LOADED_FINGERPRINTING_CONTENT = 1024U,
    STATE_BLOCKED_CRYPTOMINING_CONTENT = 2048U,
    STATE_LOADED_CRYPTOMINING_CONTENT = 2097152U,
    STATE_BLOCKED_UNSAFE_CONTENT = 16384U,
    STATE_COOKIES_LOADED = 32768U,
    STATE_COOKIES_LOADED_TRACKER = 262144U,
    STATE_COOKIES_LOADED_SOCIALTRACKER = 524288U,
    STATE_COOKIES_BLOCKED_BY_PERMISSION = 268435456U,
    STATE_COOKIES_BLOCKED_TRACKER = 536870912U,
    STATE_COOKIES_BLOCKED_SOCIALTRACKER = 16777216U,
    STATE_COOKIES_BLOCKED_ALL = 1073741824U,
    STATE_COOKIES_PARTITIONED_FOREIGN = 2147483648U,
    STATE_COOKIES_BLOCKED_FOREIGN = 128U,
    STATE_BLOCKED_SOCIALTRACKING_CONTENT = 65536U,
    STATE_LOADED_SOCIALTRACKING_CONTENT = 131072U,
    STATE_UNBLOCKED_TRACKING_CONTENT = 16U,
    STATE_HTTPS_ONLY_MODE_UPGRADED = 4194304U,
    STATE_HTTPS_ONLY_MODE_UPGRADE_FAILED = 8388608U
  };

  /* void onStateChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in unsigned long aStateFlags, in nsresult aStatus); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnStateChange(nsIWebProgress *aWebProgress, nsIRequest *aRequest, uint32_t aStateFlags, nsresult aStatus) = 0;

  /* void onProgressChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in long aCurSelfProgress, in long aMaxSelfProgress, in long aCurTotalProgress, in long aMaxTotalProgress); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnProgressChange(nsIWebProgress *aWebProgress, nsIRequest *aRequest, int32_t aCurSelfProgress, int32_t aMaxSelfProgress, int32_t aCurTotalProgress, int32_t aMaxTotalProgress) = 0;

  enum {
    LOCATION_CHANGE_SAME_DOCUMENT = 1U,
    LOCATION_CHANGE_ERROR_PAGE = 2U,
    LOCATION_CHANGE_RELOAD = 4U
  };

  /* void onLocationChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in nsIURI aLocation, [optional] in unsigned long aFlags); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnLocationChange(nsIWebProgress *aWebProgress, nsIRequest *aRequest, nsIURI *aLocation, uint32_t aFlags) = 0;

  /* void onStatusChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in nsresult aStatus, in wstring aMessage); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnStatusChange(nsIWebProgress *aWebProgress, nsIRequest *aRequest, nsresult aStatus, const char16_t * aMessage) = 0;

  /* void onSecurityChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in unsigned long aState); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnSecurityChange(nsIWebProgress *aWebProgress, nsIRequest *aRequest, uint32_t aState) = 0;

  /* void onContentBlockingEvent (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in unsigned long aEvent); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnContentBlockingEvent(nsIWebProgress *aWebProgress, nsIRequest *aRequest, uint32_t aEvent) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWebProgressListener, NS_IWEBPROGRESSLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWEBPROGRESSLISTENER \
  NS_IMETHOD OnStateChange(nsIWebProgress *aWebProgress, nsIRequest *aRequest, uint32_t aStateFlags, nsresult aStatus) override; \
  NS_IMETHOD OnProgressChange(nsIWebProgress *aWebProgress, nsIRequest *aRequest, int32_t aCurSelfProgress, int32_t aMaxSelfProgress, int32_t aCurTotalProgress, int32_t aMaxTotalProgress) override; \
  NS_IMETHOD OnLocationChange(nsIWebProgress *aWebProgress, nsIRequest *aRequest, nsIURI *aLocation, uint32_t aFlags) override; \
  NS_IMETHOD OnStatusChange(nsIWebProgress *aWebProgress, nsIRequest *aRequest, nsresult aStatus, const char16_t * aMessage) override; \
  NS_IMETHOD OnSecurityChange(nsIWebProgress *aWebProgress, nsIRequest *aRequest, uint32_t aState) override; \
  NS_IMETHOD OnContentBlockingEvent(nsIWebProgress *aWebProgress, nsIRequest *aRequest, uint32_t aEvent) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWEBPROGRESSLISTENER \
  nsresult OnStateChange(nsIWebProgress *aWebProgress, nsIRequest *aRequest, uint32_t aStateFlags, nsresult aStatus); \
  nsresult OnProgressChange(nsIWebProgress *aWebProgress, nsIRequest *aRequest, int32_t aCurSelfProgress, int32_t aMaxSelfProgress, int32_t aCurTotalProgress, int32_t aMaxTotalProgress); \
  nsresult OnLocationChange(nsIWebProgress *aWebProgress, nsIRequest *aRequest, nsIURI *aLocation, uint32_t aFlags); \
  nsresult OnStatusChange(nsIWebProgress *aWebProgress, nsIRequest *aRequest, nsresult aStatus, const char16_t * aMessage); \
  nsresult OnSecurityChange(nsIWebProgress *aWebProgress, nsIRequest *aRequest, uint32_t aState); \
  nsresult OnContentBlockingEvent(nsIWebProgress *aWebProgress, nsIRequest *aRequest, uint32_t aEvent); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWEBPROGRESSLISTENER(_to) \
  NS_IMETHOD OnStateChange(nsIWebProgress *aWebProgress, nsIRequest *aRequest, uint32_t aStateFlags, nsresult aStatus) override { return _to OnStateChange(aWebProgress, aRequest, aStateFlags, aStatus); } \
  NS_IMETHOD OnProgressChange(nsIWebProgress *aWebProgress, nsIRequest *aRequest, int32_t aCurSelfProgress, int32_t aMaxSelfProgress, int32_t aCurTotalProgress, int32_t aMaxTotalProgress) override { return _to OnProgressChange(aWebProgress, aRequest, aCurSelfProgress, aMaxSelfProgress, aCurTotalProgress, aMaxTotalProgress); } \
  NS_IMETHOD OnLocationChange(nsIWebProgress *aWebProgress, nsIRequest *aRequest, nsIURI *aLocation, uint32_t aFlags) override { return _to OnLocationChange(aWebProgress, aRequest, aLocation, aFlags); } \
  NS_IMETHOD OnStatusChange(nsIWebProgress *aWebProgress, nsIRequest *aRequest, nsresult aStatus, const char16_t * aMessage) override { return _to OnStatusChange(aWebProgress, aRequest, aStatus, aMessage); } \
  NS_IMETHOD OnSecurityChange(nsIWebProgress *aWebProgress, nsIRequest *aRequest, uint32_t aState) override { return _to OnSecurityChange(aWebProgress, aRequest, aState); } \
  NS_IMETHOD OnContentBlockingEvent(nsIWebProgress *aWebProgress, nsIRequest *aRequest, uint32_t aEvent) override { return _to OnContentBlockingEvent(aWebProgress, aRequest, aEvent); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWEBPROGRESSLISTENER(_to) \
  NS_IMETHOD OnStateChange(nsIWebProgress *aWebProgress, nsIRequest *aRequest, uint32_t aStateFlags, nsresult aStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnStateChange(aWebProgress, aRequest, aStateFlags, aStatus); } \
  NS_IMETHOD OnProgressChange(nsIWebProgress *aWebProgress, nsIRequest *aRequest, int32_t aCurSelfProgress, int32_t aMaxSelfProgress, int32_t aCurTotalProgress, int32_t aMaxTotalProgress) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnProgressChange(aWebProgress, aRequest, aCurSelfProgress, aMaxSelfProgress, aCurTotalProgress, aMaxTotalProgress); } \
  NS_IMETHOD OnLocationChange(nsIWebProgress *aWebProgress, nsIRequest *aRequest, nsIURI *aLocation, uint32_t aFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnLocationChange(aWebProgress, aRequest, aLocation, aFlags); } \
  NS_IMETHOD OnStatusChange(nsIWebProgress *aWebProgress, nsIRequest *aRequest, nsresult aStatus, const char16_t * aMessage) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnStatusChange(aWebProgress, aRequest, aStatus, aMessage); } \
  NS_IMETHOD OnSecurityChange(nsIWebProgress *aWebProgress, nsIRequest *aRequest, uint32_t aState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnSecurityChange(aWebProgress, aRequest, aState); } \
  NS_IMETHOD OnContentBlockingEvent(nsIWebProgress *aWebProgress, nsIRequest *aRequest, uint32_t aEvent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnContentBlockingEvent(aWebProgress, aRequest, aEvent); } 


#endif /* __gen_nsIWebProgressListener_h__ */
