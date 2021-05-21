/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/system/nsIOSPermissionRequest.idl
 */

#ifndef __gen_nsIOSPermissionRequest_h__
#define __gen_nsIOSPermissionRequest_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIOSPermissionRequest */
#define NS_IOSPERMISSIONREQUEST_IID_STR "95790842-75a0-430d-98bf-f5ce3788ea6d"

#define NS_IOSPERMISSIONREQUEST_IID \
  {0x95790842, 0x75a0, 0x430d, \
    { 0x98, 0xbf, 0xf5, 0xce, 0x37, 0x88, 0xea, 0x6d }}

class NS_NO_VTABLE nsIOSPermissionRequest : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IOSPERMISSIONREQUEST_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIOSPermissionRequest;

  enum {
    PERMISSION_STATE_NOTDETERMINED = 0U,
    PERMISSION_STATE_RESTRICTED = 1U,
    PERMISSION_STATE_DENIED = 2U,
    PERMISSION_STATE_AUTHORIZED = 3U
  };

  /* void getMediaCapturePermissionState (out uint16_t aVideo, out uint16_t aAudio); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMediaCapturePermissionState(uint16_t *aVideo, uint16_t *aAudio) = 0;

  /* void getAudioCapturePermissionState (out uint16_t aAudio); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAudioCapturePermissionState(uint16_t *aAudio) = 0;

  /* void getVideoCapturePermissionState (out uint16_t aVideo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetVideoCapturePermissionState(uint16_t *aVideo) = 0;

  /* void getScreenCapturePermissionState (out uint16_t aScreen); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetScreenCapturePermissionState(uint16_t *aScreen) = 0;

  /* [implicit_jscontext,must_use] Promise requestVideoCapturePermission (); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD RequestVideoCapturePermission(JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

  /* [implicit_jscontext,must_use] Promise requestAudioCapturePermission (); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD RequestAudioCapturePermission(JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

  /* void maybeRequestScreenCapturePermission (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MaybeRequestScreenCapturePermission(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIOSPermissionRequest, NS_IOSPERMISSIONREQUEST_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIOSPERMISSIONREQUEST \
  NS_IMETHOD GetMediaCapturePermissionState(uint16_t *aVideo, uint16_t *aAudio) override; \
  NS_IMETHOD GetAudioCapturePermissionState(uint16_t *aAudio) override; \
  NS_IMETHOD GetVideoCapturePermissionState(uint16_t *aVideo) override; \
  NS_IMETHOD GetScreenCapturePermissionState(uint16_t *aScreen) override; \
  [[nodiscard]] NS_IMETHOD RequestVideoCapturePermission(JSContext* cx, ::mozilla::dom::Promise * * _retval) override; \
  [[nodiscard]] NS_IMETHOD RequestAudioCapturePermission(JSContext* cx, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD MaybeRequestScreenCapturePermission(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIOSPERMISSIONREQUEST \
  nsresult GetMediaCapturePermissionState(uint16_t *aVideo, uint16_t *aAudio); \
  nsresult GetAudioCapturePermissionState(uint16_t *aAudio); \
  nsresult GetVideoCapturePermissionState(uint16_t *aVideo); \
  nsresult GetScreenCapturePermissionState(uint16_t *aScreen); \
  [[nodiscard]] nsresult RequestVideoCapturePermission(JSContext* cx, ::mozilla::dom::Promise * * _retval); \
  [[nodiscard]] nsresult RequestAudioCapturePermission(JSContext* cx, ::mozilla::dom::Promise * * _retval); \
  nsresult MaybeRequestScreenCapturePermission(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIOSPERMISSIONREQUEST(_to) \
  NS_IMETHOD GetMediaCapturePermissionState(uint16_t *aVideo, uint16_t *aAudio) override { return _to GetMediaCapturePermissionState(aVideo, aAudio); } \
  NS_IMETHOD GetAudioCapturePermissionState(uint16_t *aAudio) override { return _to GetAudioCapturePermissionState(aAudio); } \
  NS_IMETHOD GetVideoCapturePermissionState(uint16_t *aVideo) override { return _to GetVideoCapturePermissionState(aVideo); } \
  NS_IMETHOD GetScreenCapturePermissionState(uint16_t *aScreen) override { return _to GetScreenCapturePermissionState(aScreen); } \
  [[nodiscard]] NS_IMETHOD RequestVideoCapturePermission(JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to RequestVideoCapturePermission(cx, _retval); } \
  [[nodiscard]] NS_IMETHOD RequestAudioCapturePermission(JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to RequestAudioCapturePermission(cx, _retval); } \
  NS_IMETHOD MaybeRequestScreenCapturePermission(void) override { return _to MaybeRequestScreenCapturePermission(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIOSPERMISSIONREQUEST(_to) \
  NS_IMETHOD GetMediaCapturePermissionState(uint16_t *aVideo, uint16_t *aAudio) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMediaCapturePermissionState(aVideo, aAudio); } \
  NS_IMETHOD GetAudioCapturePermissionState(uint16_t *aAudio) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAudioCapturePermissionState(aAudio); } \
  NS_IMETHOD GetVideoCapturePermissionState(uint16_t *aVideo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetVideoCapturePermissionState(aVideo); } \
  NS_IMETHOD GetScreenCapturePermissionState(uint16_t *aScreen) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScreenCapturePermissionState(aScreen); } \
  [[nodiscard]] NS_IMETHOD RequestVideoCapturePermission(JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RequestVideoCapturePermission(cx, _retval); } \
  [[nodiscard]] NS_IMETHOD RequestAudioCapturePermission(JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RequestAudioCapturePermission(cx, _retval); } \
  NS_IMETHOD MaybeRequestScreenCapturePermission(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MaybeRequestScreenCapturePermission(); } 


#endif /* __gen_nsIOSPermissionRequest_h__ */
