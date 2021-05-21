/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIRequest.idl
 */

#ifndef __gen_nsIRequest_h__
#define __gen_nsIRequest_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsILoadGroup; /* forward declaration */

typedef uint32_t  nsLoadFlags;


/* starting interface:    nsIRequest */
#define NS_IREQUEST_IID_STR "ef6bfbd2-fd46-48d8-96b7-9f8f0fd387fe"

#define NS_IREQUEST_IID \
  {0xef6bfbd2, 0xfd46, 0x48d8, \
    { 0x96, 0xb7, 0x9f, 0x8f, 0x0f, 0xd3, 0x87, 0xfe }}

class nsIRequest : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IREQUEST_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIRequest;

  /* readonly attribute AUTF8String name; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetName(nsACString& aName) = 0;

  /* boolean isPending (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsPending(bool *_retval) = 0;

  /* readonly attribute nsresult status; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetStatus(nsresult *aStatus) = 0;

  /* void cancel (in nsresult aStatus); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Cancel(nsresult aStatus) = 0;

  /* void suspend (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Suspend(void) = 0;

  /* void resume (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Resume(void) = 0;

  /* attribute nsILoadGroup loadGroup; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLoadGroup(nsILoadGroup **aLoadGroup) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetLoadGroup(nsILoadGroup *aLoadGroup) = 0;

  /* attribute nsLoadFlags loadFlags; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLoadFlags(nsLoadFlags *aLoadFlags) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetLoadFlags(nsLoadFlags aLoadFlags) = 0;

  enum {
    LOAD_REQUESTMASK = 65535U,
    LOAD_NORMAL = 0U,
    LOAD_BACKGROUND = 1U,
    LOAD_HTML_OBJECT_DATA = 2U,
    LOAD_DOCUMENT_NEEDS_COOKIE = 4U
  };

  enum TRRMode : uint8_t {
    TRR_DEFAULT_MODE = 0,
    TRR_DISABLED_MODE = 1,
    TRR_FIRST_MODE = 2,
    TRR_ONLY_MODE = 3,
  };

  /* nsIRequest_TRRMode getTRRMode (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTRRMode(nsIRequest::TRRMode *_retval) = 0;

  /* void setTRRMode (in nsIRequest_TRRMode mode); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetTRRMode(nsIRequest::TRRMode mode) = 0;

         inline TRRMode GetTRRMode() {
            TRRMode mode = TRR_DEFAULT_MODE;
            GetTRRMode(&mode);
            return mode;
        }
        inline nsresult GetTRRModeImpl(nsIRequest::TRRMode* aTRRMode) {
          NS_ENSURE_ARG_POINTER(aTRRMode);
          nsLoadFlags flags = nsIRequest::LOAD_NORMAL;
          nsresult rv = GetLoadFlags(&flags);
          if (NS_FAILED(rv)) {
            return rv;
          }
          *aTRRMode = static_cast<nsIRequest::TRRMode>(
              (flags & nsIRequest::LOAD_TRR_MASK) >> 3);
          return NS_OK;
        }
        inline nsresult SetTRRModeImpl(nsIRequest::TRRMode aTRRMode) {
          MOZ_ASSERT(aTRRMode <= 3, "invalid value");
          nsLoadFlags flags = nsIRequest::LOAD_NORMAL;
          nsresult rv = GetLoadFlags(&flags);
          if (NS_FAILED(rv)) {
            return rv;
          }
          flags = (flags & ~nsIRequest::LOAD_TRR_MASK) | (aTRRMode << 3);
          return SetLoadFlags(flags);
        }
      enum {
    LOAD_TRR_MASK = 24U,
    LOAD_TRR_DISABLED_MODE = 8U,
    LOAD_TRR_FIRST_MODE = 16U,
    LOAD_TRR_ONLY_MODE = 24U,
    LOAD_ANONYMOUS_ALLOW_CLIENT_CERT = 32,
    INHIBIT_CACHING = 128U,
    INHIBIT_PERSISTENT_CACHING = 256U,
    LOAD_BYPASS_CACHE = 512U,
    LOAD_FROM_CACHE = 1024U,
    VALIDATE_ALWAYS = 2048U,
    VALIDATE_NEVER = 4096U,
    VALIDATE_ONCE_PER_SESSION = 8192U,
    LOAD_ANONYMOUS = 16384U,
    LOAD_FRESH_CONNECTION = 32768U
  };

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIRequest, NS_IREQUEST_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIREQUEST \
  NS_IMETHOD GetName(nsACString& aName) override; \
  NS_IMETHOD IsPending(bool *_retval) override; \
  NS_IMETHOD GetStatus(nsresult *aStatus) override; \
  NS_IMETHOD Cancel(nsresult aStatus) override; \
  NS_IMETHOD Suspend(void) override; \
  NS_IMETHOD Resume(void) override; \
  NS_IMETHOD GetLoadGroup(nsILoadGroup **aLoadGroup) override; \
  NS_IMETHOD SetLoadGroup(nsILoadGroup *aLoadGroup) override; \
  NS_IMETHOD GetLoadFlags(nsLoadFlags *aLoadFlags) override; \
  NS_IMETHOD SetLoadFlags(nsLoadFlags aLoadFlags) override; \
  NS_IMETHOD GetTRRMode(nsIRequest::TRRMode *_retval) override; \
  NS_IMETHOD SetTRRMode(nsIRequest::TRRMode mode) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIREQUEST \
  nsresult GetName(nsACString& aName); \
  nsresult IsPending(bool *_retval); \
  nsresult GetStatus(nsresult *aStatus); \
  nsresult Cancel(nsresult aStatus); \
  nsresult Suspend(void); \
  nsresult Resume(void); \
  nsresult GetLoadGroup(nsILoadGroup **aLoadGroup); \
  nsresult SetLoadGroup(nsILoadGroup *aLoadGroup); \
  nsresult GetLoadFlags(nsLoadFlags *aLoadFlags); \
  nsresult SetLoadFlags(nsLoadFlags aLoadFlags); \
  nsresult GetTRRMode(nsIRequest::TRRMode *_retval); \
  nsresult SetTRRMode(nsIRequest::TRRMode mode); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIREQUEST(_to) \
  NS_IMETHOD GetName(nsACString& aName) override { return _to GetName(aName); } \
  NS_IMETHOD IsPending(bool *_retval) override { return _to IsPending(_retval); } \
  NS_IMETHOD GetStatus(nsresult *aStatus) override { return _to GetStatus(aStatus); } \
  NS_IMETHOD Cancel(nsresult aStatus) override { return _to Cancel(aStatus); } \
  NS_IMETHOD Suspend(void) override { return _to Suspend(); } \
  NS_IMETHOD Resume(void) override { return _to Resume(); } \
  NS_IMETHOD GetLoadGroup(nsILoadGroup **aLoadGroup) override { return _to GetLoadGroup(aLoadGroup); } \
  NS_IMETHOD SetLoadGroup(nsILoadGroup *aLoadGroup) override { return _to SetLoadGroup(aLoadGroup); } \
  NS_IMETHOD GetLoadFlags(nsLoadFlags *aLoadFlags) override { return _to GetLoadFlags(aLoadFlags); } \
  NS_IMETHOD SetLoadFlags(nsLoadFlags aLoadFlags) override { return _to SetLoadFlags(aLoadFlags); } \
  NS_IMETHOD GetTRRMode(nsIRequest::TRRMode *_retval) override { return _to GetTRRMode(_retval); } \
  NS_IMETHOD SetTRRMode(nsIRequest::TRRMode mode) override { return _to SetTRRMode(mode); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIREQUEST(_to) \
  NS_IMETHOD GetName(nsACString& aName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetName(aName); } \
  NS_IMETHOD IsPending(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsPending(_retval); } \
  NS_IMETHOD GetStatus(nsresult *aStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStatus(aStatus); } \
  NS_IMETHOD Cancel(nsresult aStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Cancel(aStatus); } \
  NS_IMETHOD Suspend(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Suspend(); } \
  NS_IMETHOD Resume(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Resume(); } \
  NS_IMETHOD GetLoadGroup(nsILoadGroup **aLoadGroup) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoadGroup(aLoadGroup); } \
  NS_IMETHOD SetLoadGroup(nsILoadGroup *aLoadGroup) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLoadGroup(aLoadGroup); } \
  NS_IMETHOD GetLoadFlags(nsLoadFlags *aLoadFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoadFlags(aLoadFlags); } \
  NS_IMETHOD SetLoadFlags(nsLoadFlags aLoadFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLoadFlags(aLoadFlags); } \
  NS_IMETHOD GetTRRMode(nsIRequest::TRRMode *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTRRMode(_retval); } \
  NS_IMETHOD SetTRRMode(nsIRequest::TRRMode mode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTRRMode(mode); } \


#endif /* __gen_nsIRequest_h__ */
