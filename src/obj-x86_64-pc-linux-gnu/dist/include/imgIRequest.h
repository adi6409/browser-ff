/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/image/imgIRequest.idl
 */

#ifndef __gen_imgIRequest_h__
#define __gen_imgIRequest_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIRequest_h__
#include "nsIRequest.h"
#endif

#ifndef __gen_imgIContainer_h__
#include "imgIContainer.h"
#endif

#include "mozilla/AlreadyAddRefed.h"
#include "mozilla/Assertions.h"
#include "mozilla/DebugOnly.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class imgINotificationObserver; /* forward declaration */

class nsIURI; /* forward declaration */

class nsIPrincipal; /* forward declaration */


/* starting interface:    imgIRequest */
#define IMGIREQUEST_IID_STR "db0a945c-3883-424a-98d0-2ee0523b0255"

#define IMGIREQUEST_IID \
  {0xdb0a945c, 0x3883, 0x424a, \
    { 0x98, 0xd0, 0x2e, 0xe0, 0x52, 0x3b, 0x02, 0x55 }}

class NS_NO_VTABLE imgIRequest : public nsIRequest {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(IMGIREQUEST_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = imgIRequest;

  /* readonly attribute imgIContainer image; */
  NS_IMETHOD GetImage(imgIContainer **aImage) = 0;

  /* [infallible] readonly attribute unsigned long producerId; */
  NS_IMETHOD GetProducerId(uint32_t *aProducerId) = 0;
  inline uint32_t  GetProducerId()
  {
    uint32_t result;
    mozilla::DebugOnly<nsresult> rv = GetProducerId(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  enum {
    STATUS_NONE = 0,
    STATUS_SIZE_AVAILABLE = 1,
    STATUS_LOAD_COMPLETE = 2,
    STATUS_ERROR = 4,
    STATUS_FRAME_COMPLETE = 8,
    STATUS_DECODE_COMPLETE = 16,
    STATUS_IS_ANIMATED = 32,
    STATUS_HAS_TRANSPARENCY = 64
  };

  /* readonly attribute unsigned long imageStatus; */
  NS_IMETHOD GetImageStatus(uint32_t *aImageStatus) = 0;

  /* [noscript] readonly attribute nsresult imageErrorCode; */
  NS_IMETHOD GetImageErrorCode(nsresult *aImageErrorCode) = 0;

  /* readonly attribute nsIURI URI; */
  NS_IMETHOD GetURI(nsIURI **aURI) = 0;

  /* readonly attribute nsIURI finalURI; */
  NS_IMETHOD GetFinalURI(nsIURI **aFinalURI) = 0;

  /* readonly attribute imgINotificationObserver notificationObserver; */
  NS_IMETHOD GetNotificationObserver(imgINotificationObserver **aNotificationObserver) = 0;

  /* readonly attribute string mimeType; */
  NS_IMETHOD GetMimeType(char * *aMimeType) = 0;

  /* imgIRequest clone (in imgINotificationObserver aObserver); */
  NS_IMETHOD Clone(imgINotificationObserver *aObserver, imgIRequest **_retval) = 0;

  /* readonly attribute nsIPrincipal imagePrincipal; */
  NS_IMETHOD GetImagePrincipal(nsIPrincipal **aImagePrincipal) = 0;

  /* readonly attribute bool hadCrossOriginRedirects; */
  NS_IMETHOD GetHadCrossOriginRedirects(bool *aHadCrossOriginRedirects) = 0;

  /* readonly attribute bool multipart; */
  NS_IMETHOD GetMultipart(bool *aMultipart) = 0;

  enum {
    CORS_NONE = 1,
    CORS_ANONYMOUS = 2,
    CORS_USE_CREDENTIALS = 3
  };

  /* readonly attribute long CORSMode; */
  NS_IMETHOD GetCORSMode(int32_t *aCORSMode) = 0;

  /* void cancelAndForgetObserver (in nsresult aStatus); */
  NS_IMETHOD CancelAndForgetObserver(nsresult aStatus) = 0;

  /* void startDecoding (in uint32_t aFlags); */
  NS_IMETHOD StartDecoding(uint32_t aFlags) = 0;

  /* [noscript,notxpcom] boolean startDecodingWithResult (in uint32_t aFlags); */
  NS_IMETHOD_(bool) StartDecodingWithResult(uint32_t aFlags) = 0;

  /* [noscript,notxpcom] imgIContainer_DecodeResult requestDecodeWithResult (in uint32_t aFlags); */
  NS_IMETHOD_(imgIContainer::DecodeResult) RequestDecodeWithResult(uint32_t aFlags) = 0;

  /* void lockImage (); */
  NS_IMETHOD LockImage(void) = 0;

  /* void unlockImage (); */
  NS_IMETHOD UnlockImage(void) = 0;

  /* void requestDiscard (); */
  NS_IMETHOD RequestDiscard(void) = 0;

  /* imgIRequest getStaticRequest (); */
  NS_IMETHOD GetStaticRequest(imgIRequest **_retval) = 0;

  /* void incrementAnimationConsumers (); */
  NS_IMETHOD IncrementAnimationConsumers(void) = 0;

  /* void decrementAnimationConsumers (); */
  NS_IMETHOD DecrementAnimationConsumers(void) = 0;

  enum {
    CATEGORY_FRAME_INIT = 1U,
    CATEGORY_FRAME_STYLE = 2U,
    CATEGORY_SIZE_QUERY = 4U,
    CATEGORY_DISPLAY = 8U
  };

  /* void boostPriority (in uint32_t aCategory); */
  NS_IMETHOD BoostPriority(uint32_t aCategory) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(imgIRequest, IMGIREQUEST_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_IMGIREQUEST \
  NS_IMETHOD GetImage(imgIContainer **aImage) override; \
  using imgIRequest::GetProducerId; \
  NS_IMETHOD GetProducerId(uint32_t *aProducerId) override; \
  NS_IMETHOD GetImageStatus(uint32_t *aImageStatus) override; \
  NS_IMETHOD GetImageErrorCode(nsresult *aImageErrorCode) override; \
  NS_IMETHOD GetURI(nsIURI **aURI) override; \
  NS_IMETHOD GetFinalURI(nsIURI **aFinalURI) override; \
  NS_IMETHOD GetNotificationObserver(imgINotificationObserver **aNotificationObserver) override; \
  NS_IMETHOD GetMimeType(char * *aMimeType) override; \
  NS_IMETHOD Clone(imgINotificationObserver *aObserver, imgIRequest **_retval) override; \
  NS_IMETHOD GetImagePrincipal(nsIPrincipal **aImagePrincipal) override; \
  NS_IMETHOD GetHadCrossOriginRedirects(bool *aHadCrossOriginRedirects) override; \
  NS_IMETHOD GetMultipart(bool *aMultipart) override; \
  NS_IMETHOD GetCORSMode(int32_t *aCORSMode) override; \
  NS_IMETHOD CancelAndForgetObserver(nsresult aStatus) override; \
  NS_IMETHOD StartDecoding(uint32_t aFlags) override; \
  NS_IMETHOD_(bool) StartDecodingWithResult(uint32_t aFlags) override; \
  NS_IMETHOD_(imgIContainer::DecodeResult) RequestDecodeWithResult(uint32_t aFlags) override; \
  NS_IMETHOD LockImage(void) override; \
  NS_IMETHOD UnlockImage(void) override; \
  NS_IMETHOD RequestDiscard(void) override; \
  NS_IMETHOD GetStaticRequest(imgIRequest **_retval) override; \
  NS_IMETHOD IncrementAnimationConsumers(void) override; \
  NS_IMETHOD DecrementAnimationConsumers(void) override; \
  NS_IMETHOD BoostPriority(uint32_t aCategory) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_IMGIREQUEST \
  nsresult GetImage(imgIContainer **aImage); \
  using imgIRequest::GetProducerId; \
  nsresult GetProducerId(uint32_t *aProducerId); \
  nsresult GetImageStatus(uint32_t *aImageStatus); \
  nsresult GetImageErrorCode(nsresult *aImageErrorCode); \
  nsresult GetURI(nsIURI **aURI); \
  nsresult GetFinalURI(nsIURI **aFinalURI); \
  nsresult GetNotificationObserver(imgINotificationObserver **aNotificationObserver); \
  nsresult GetMimeType(char * *aMimeType); \
  nsresult Clone(imgINotificationObserver *aObserver, imgIRequest **_retval); \
  nsresult GetImagePrincipal(nsIPrincipal **aImagePrincipal); \
  nsresult GetHadCrossOriginRedirects(bool *aHadCrossOriginRedirects); \
  nsresult GetMultipart(bool *aMultipart); \
  nsresult GetCORSMode(int32_t *aCORSMode); \
  nsresult CancelAndForgetObserver(nsresult aStatus); \
  nsresult StartDecoding(uint32_t aFlags); \
  nsresult_(bool) StartDecodingWithResult(uint32_t aFlags); \
  nsresult_(imgIContainer::DecodeResult) RequestDecodeWithResult(uint32_t aFlags); \
  nsresult LockImage(void); \
  nsresult UnlockImage(void); \
  nsresult RequestDiscard(void); \
  nsresult GetStaticRequest(imgIRequest **_retval); \
  nsresult IncrementAnimationConsumers(void); \
  nsresult DecrementAnimationConsumers(void); \
  nsresult BoostPriority(uint32_t aCategory); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_IMGIREQUEST(_to) \
  NS_IMETHOD GetImage(imgIContainer **aImage) override { return _to GetImage(aImage); } \
  using imgIRequest::GetProducerId; \
  NS_IMETHOD GetProducerId(uint32_t *aProducerId) override { return _to GetProducerId(aProducerId); } \
  NS_IMETHOD GetImageStatus(uint32_t *aImageStatus) override { return _to GetImageStatus(aImageStatus); } \
  NS_IMETHOD GetImageErrorCode(nsresult *aImageErrorCode) override { return _to GetImageErrorCode(aImageErrorCode); } \
  NS_IMETHOD GetURI(nsIURI **aURI) override { return _to GetURI(aURI); } \
  NS_IMETHOD GetFinalURI(nsIURI **aFinalURI) override { return _to GetFinalURI(aFinalURI); } \
  NS_IMETHOD GetNotificationObserver(imgINotificationObserver **aNotificationObserver) override { return _to GetNotificationObserver(aNotificationObserver); } \
  NS_IMETHOD GetMimeType(char * *aMimeType) override { return _to GetMimeType(aMimeType); } \
  NS_IMETHOD Clone(imgINotificationObserver *aObserver, imgIRequest **_retval) override { return _to Clone(aObserver, _retval); } \
  NS_IMETHOD GetImagePrincipal(nsIPrincipal **aImagePrincipal) override { return _to GetImagePrincipal(aImagePrincipal); } \
  NS_IMETHOD GetHadCrossOriginRedirects(bool *aHadCrossOriginRedirects) override { return _to GetHadCrossOriginRedirects(aHadCrossOriginRedirects); } \
  NS_IMETHOD GetMultipart(bool *aMultipart) override { return _to GetMultipart(aMultipart); } \
  NS_IMETHOD GetCORSMode(int32_t *aCORSMode) override { return _to GetCORSMode(aCORSMode); } \
  NS_IMETHOD CancelAndForgetObserver(nsresult aStatus) override { return _to CancelAndForgetObserver(aStatus); } \
  NS_IMETHOD StartDecoding(uint32_t aFlags) override { return _to StartDecoding(aFlags); } \
  NS_IMETHOD_(bool) StartDecodingWithResult(uint32_t aFlags) override { return _to StartDecodingWithResult(aFlags); } \
  NS_IMETHOD_(imgIContainer::DecodeResult) RequestDecodeWithResult(uint32_t aFlags) override { return _to RequestDecodeWithResult(aFlags); } \
  NS_IMETHOD LockImage(void) override { return _to LockImage(); } \
  NS_IMETHOD UnlockImage(void) override { return _to UnlockImage(); } \
  NS_IMETHOD RequestDiscard(void) override { return _to RequestDiscard(); } \
  NS_IMETHOD GetStaticRequest(imgIRequest **_retval) override { return _to GetStaticRequest(_retval); } \
  NS_IMETHOD IncrementAnimationConsumers(void) override { return _to IncrementAnimationConsumers(); } \
  NS_IMETHOD DecrementAnimationConsumers(void) override { return _to DecrementAnimationConsumers(); } \
  NS_IMETHOD BoostPriority(uint32_t aCategory) override { return _to BoostPriority(aCategory); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_IMGIREQUEST(_to) \
  NS_IMETHOD GetImage(imgIContainer **aImage) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetImage(aImage); } \
  NS_IMETHOD GetProducerId(uint32_t *aProducerId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProducerId(aProducerId); } \
  NS_IMETHOD GetImageStatus(uint32_t *aImageStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetImageStatus(aImageStatus); } \
  NS_IMETHOD GetImageErrorCode(nsresult *aImageErrorCode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetImageErrorCode(aImageErrorCode); } \
  NS_IMETHOD GetURI(nsIURI **aURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetURI(aURI); } \
  NS_IMETHOD GetFinalURI(nsIURI **aFinalURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFinalURI(aFinalURI); } \
  NS_IMETHOD GetNotificationObserver(imgINotificationObserver **aNotificationObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNotificationObserver(aNotificationObserver); } \
  NS_IMETHOD GetMimeType(char * *aMimeType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMimeType(aMimeType); } \
  NS_IMETHOD Clone(imgINotificationObserver *aObserver, imgIRequest **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Clone(aObserver, _retval); } \
  NS_IMETHOD GetImagePrincipal(nsIPrincipal **aImagePrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetImagePrincipal(aImagePrincipal); } \
  NS_IMETHOD GetHadCrossOriginRedirects(bool *aHadCrossOriginRedirects) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHadCrossOriginRedirects(aHadCrossOriginRedirects); } \
  NS_IMETHOD GetMultipart(bool *aMultipart) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMultipart(aMultipart); } \
  NS_IMETHOD GetCORSMode(int32_t *aCORSMode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCORSMode(aCORSMode); } \
  NS_IMETHOD CancelAndForgetObserver(nsresult aStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CancelAndForgetObserver(aStatus); } \
  NS_IMETHOD StartDecoding(uint32_t aFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StartDecoding(aFlags); } \
  NS_IMETHOD_(bool) StartDecodingWithResult(uint32_t aFlags) override; \
  NS_IMETHOD_(imgIContainer::DecodeResult) RequestDecodeWithResult(uint32_t aFlags) override; \
  NS_IMETHOD LockImage(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LockImage(); } \
  NS_IMETHOD UnlockImage(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UnlockImage(); } \
  NS_IMETHOD RequestDiscard(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RequestDiscard(); } \
  NS_IMETHOD GetStaticRequest(imgIRequest **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStaticRequest(_retval); } \
  NS_IMETHOD IncrementAnimationConsumers(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IncrementAnimationConsumers(); } \
  NS_IMETHOD DecrementAnimationConsumers(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DecrementAnimationConsumers(); } \
  NS_IMETHOD BoostPriority(uint32_t aCategory) override { return !_to ? NS_ERROR_NULL_POINTER : _to->BoostPriority(aCategory); } 


#endif /* __gen_imgIRequest_h__ */
