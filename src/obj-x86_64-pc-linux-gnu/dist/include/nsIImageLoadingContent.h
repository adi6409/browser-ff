/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsIImageLoadingContent.idl
 */

#ifndef __gen_nsIImageLoadingContent_h__
#define __gen_nsIImageLoadingContent_h__


#ifndef __gen_imgINotificationObserver_h__
#include "imgINotificationObserver.h"
#endif

#include "mozilla/AlreadyAddRefed.h"
#include "mozilla/Assertions.h"
#include "mozilla/DebugOnly.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#include "mozilla/Maybe.h"
#include "Visibility.h"
class imgIRequest; /* forward declaration */

class nsIChannel; /* forward declaration */

class nsIStreamListener; /* forward declaration */

class nsIURI; /* forward declaration */

class nsIFrame; /* forward declaration */


/* starting interface:    nsIImageLoadingContent */
#define NS_IIMAGELOADINGCONTENT_IID_STR "0357123d-9224-4d12-a47e-868c32689777"

#define NS_IIMAGELOADINGCONTENT_IID \
  {0x0357123d, 0x9224, 0x4d12, \
    { 0xa4, 0x7e, 0x86, 0x8c, 0x32, 0x68, 0x97, 0x77 }}

class NS_NO_VTABLE nsIImageLoadingContent : public imgINotificationObserver {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IIMAGELOADINGCONTENT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIImageLoadingContent;

  enum {
    UNKNOWN_REQUEST = -1,
    CURRENT_REQUEST = 0,
    PENDING_REQUEST = 1
  };

  /* [nostdcall,notxpcom] void setLoadingEnabled (in boolean aEnabled); */
  virtual void SetLoadingEnabled(bool aEnabled) = 0;

  /* [nostdcall,notxpcom] void addNativeObserver (in imgINotificationObserver aObserver); */
  virtual void AddNativeObserver(imgINotificationObserver *aObserver) = 0;

  /* [nostdcall,notxpcom] void removeNativeObserver (in imgINotificationObserver aObserver); */
  virtual void RemoveNativeObserver(imgINotificationObserver *aObserver) = 0;

  /* [noscript] imgIRequest getRequest (in long aRequestType); */
  NS_IMETHOD GetRequest(int32_t aRequestType, imgIRequest **_retval) = 0;

  /* [notxpcom] void frameCreated (in nsIFrame aFrame); */
  NS_IMETHOD_(void) FrameCreated(nsIFrame *aFrame) = 0;

  /* [notxpcom] void frameDestroyed (in nsIFrame aFrame); */
  NS_IMETHOD_(void) FrameDestroyed(nsIFrame *aFrame) = 0;

  /* [noscript] long getRequestType (in imgIRequest aRequest); */
  NS_IMETHOD GetRequestType(imgIRequest *aRequest, int32_t *_retval) = 0;

  /* [infallible,noscript] readonly attribute nsIURI currentURI; */
  NS_IMETHOD GetCurrentURI(nsIURI **aCurrentURI) = 0;
   inline already_AddRefed<nsIURI> GetCurrentURI()
  {
    nsIURI* result = nullptr;
    mozilla::DebugOnly<nsresult> rv = GetCurrentURI(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return already_AddRefed<nsIURI>(result);
  }

  /* [noscript] nsIStreamListener loadImageWithChannel (in nsIChannel aChannel); */
  NS_IMETHOD LoadImageWithChannel(nsIChannel *aChannel, nsIStreamListener **_retval) = 0;

  /* [nostdcall,notxpcom] void forceImageState (in boolean aForce, in unsigned long long aState); */
  virtual void ForceImageState(bool aForce, uint64_t aState) = 0;

  /* [noscript,notxpcom] void onVisibilityChange (in Visibility aNewVisibility, in MaybeOnNonvisible aNonvisibleAction); */
  NS_IMETHOD_(void) OnVisibilityChange(mozilla::Visibility aNewVisibility, const mozilla::Maybe<mozilla::OnNonvisible> & aNonvisibleAction) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIImageLoadingContent, NS_IIMAGELOADINGCONTENT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIIMAGELOADINGCONTENT \
  virtual void SetLoadingEnabled(bool aEnabled) override; \
  virtual void AddNativeObserver(imgINotificationObserver *aObserver) override; \
  virtual void RemoveNativeObserver(imgINotificationObserver *aObserver) override; \
  NS_IMETHOD GetRequest(int32_t aRequestType, imgIRequest **_retval) override; \
  NS_IMETHOD_(void) FrameCreated(nsIFrame *aFrame) override; \
  NS_IMETHOD_(void) FrameDestroyed(nsIFrame *aFrame) override; \
  NS_IMETHOD GetRequestType(imgIRequest *aRequest, int32_t *_retval) override; \
  using nsIImageLoadingContent::GetCurrentURI; \
  NS_IMETHOD GetCurrentURI(nsIURI **aCurrentURI) override; \
  NS_IMETHOD LoadImageWithChannel(nsIChannel *aChannel, nsIStreamListener **_retval) override; \
  virtual void ForceImageState(bool aForce, uint64_t aState) override; \
  NS_IMETHOD_(void) OnVisibilityChange(mozilla::Visibility aNewVisibility, const mozilla::Maybe<mozilla::OnNonvisible> & aNonvisibleAction) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIIMAGELOADINGCONTENT \
  void SetLoadingEnabled(bool aEnabled); \
  void AddNativeObserver(imgINotificationObserver *aObserver); \
  void RemoveNativeObserver(imgINotificationObserver *aObserver); \
  nsresult GetRequest(int32_t aRequestType, imgIRequest **_retval); \
  nsresult_(void) FrameCreated(nsIFrame *aFrame); \
  nsresult_(void) FrameDestroyed(nsIFrame *aFrame); \
  nsresult GetRequestType(imgIRequest *aRequest, int32_t *_retval); \
  using nsIImageLoadingContent::GetCurrentURI; \
  nsresult GetCurrentURI(nsIURI **aCurrentURI); \
  nsresult LoadImageWithChannel(nsIChannel *aChannel, nsIStreamListener **_retval); \
  void ForceImageState(bool aForce, uint64_t aState); \
  nsresult_(void) OnVisibilityChange(mozilla::Visibility aNewVisibility, const mozilla::Maybe<mozilla::OnNonvisible> & aNonvisibleAction); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIIMAGELOADINGCONTENT(_to) \
  virtual void SetLoadingEnabled(bool aEnabled) override { return _to SetLoadingEnabled(aEnabled); } \
  virtual void AddNativeObserver(imgINotificationObserver *aObserver) override { return _to AddNativeObserver(aObserver); } \
  virtual void RemoveNativeObserver(imgINotificationObserver *aObserver) override { return _to RemoveNativeObserver(aObserver); } \
  NS_IMETHOD GetRequest(int32_t aRequestType, imgIRequest **_retval) override { return _to GetRequest(aRequestType, _retval); } \
  NS_IMETHOD_(void) FrameCreated(nsIFrame *aFrame) override { return _to FrameCreated(aFrame); } \
  NS_IMETHOD_(void) FrameDestroyed(nsIFrame *aFrame) override { return _to FrameDestroyed(aFrame); } \
  NS_IMETHOD GetRequestType(imgIRequest *aRequest, int32_t *_retval) override { return _to GetRequestType(aRequest, _retval); } \
  using nsIImageLoadingContent::GetCurrentURI; \
  NS_IMETHOD GetCurrentURI(nsIURI **aCurrentURI) override { return _to GetCurrentURI(aCurrentURI); } \
  NS_IMETHOD LoadImageWithChannel(nsIChannel *aChannel, nsIStreamListener **_retval) override { return _to LoadImageWithChannel(aChannel, _retval); } \
  virtual void ForceImageState(bool aForce, uint64_t aState) override { return _to ForceImageState(aForce, aState); } \
  NS_IMETHOD_(void) OnVisibilityChange(mozilla::Visibility aNewVisibility, const mozilla::Maybe<mozilla::OnNonvisible> & aNonvisibleAction) override { return _to OnVisibilityChange(aNewVisibility, aNonvisibleAction); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIIMAGELOADINGCONTENT(_to) \
  virtual void SetLoadingEnabled(bool aEnabled) override; \
  virtual void AddNativeObserver(imgINotificationObserver *aObserver) override; \
  virtual void RemoveNativeObserver(imgINotificationObserver *aObserver) override; \
  NS_IMETHOD GetRequest(int32_t aRequestType, imgIRequest **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRequest(aRequestType, _retval); } \
  NS_IMETHOD_(void) FrameCreated(nsIFrame *aFrame) override; \
  NS_IMETHOD_(void) FrameDestroyed(nsIFrame *aFrame) override; \
  NS_IMETHOD GetRequestType(imgIRequest *aRequest, int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRequestType(aRequest, _retval); } \
  NS_IMETHOD GetCurrentURI(nsIURI **aCurrentURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCurrentURI(aCurrentURI); } \
  NS_IMETHOD LoadImageWithChannel(nsIChannel *aChannel, nsIStreamListener **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LoadImageWithChannel(aChannel, _retval); } \
  virtual void ForceImageState(bool aForce, uint64_t aState) override; \
  NS_IMETHOD_(void) OnVisibilityChange(mozilla::Visibility aNewVisibility, const mozilla::Maybe<mozilla::OnNonvisible> & aNonvisibleAction) override; 


#endif /* __gen_nsIImageLoadingContent_h__ */
