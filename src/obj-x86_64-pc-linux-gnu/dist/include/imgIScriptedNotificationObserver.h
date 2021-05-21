/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/image/imgIScriptedNotificationObserver.idl
 */

#ifndef __gen_imgIScriptedNotificationObserver_h__
#define __gen_imgIScriptedNotificationObserver_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class imgIRequest; /* forward declaration */


/* starting interface:    imgIScriptedNotificationObserver */
#define IMGISCRIPTEDNOTIFICATIONOBSERVER_IID_STR "10be55b3-2029-41a7-a975-538efed250ed"

#define IMGISCRIPTEDNOTIFICATIONOBSERVER_IID \
  {0x10be55b3, 0x2029, 0x41a7, \
    { 0xa9, 0x75, 0x53, 0x8e, 0xfe, 0xd2, 0x50, 0xed }}

class NS_NO_VTABLE imgIScriptedNotificationObserver : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(IMGISCRIPTEDNOTIFICATIONOBSERVER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = imgIScriptedNotificationObserver;

  /* void sizeAvailable (in imgIRequest aRequest); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SizeAvailable(imgIRequest *aRequest) = 0;

  /* void frameUpdate (in imgIRequest aRequest); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FrameUpdate(imgIRequest *aRequest) = 0;

  /* void frameComplete (in imgIRequest aRequest); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FrameComplete(imgIRequest *aRequest) = 0;

  /* void loadComplete (in imgIRequest aRequest); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD LoadComplete(imgIRequest *aRequest) = 0;

  /* void decodeComplete (in imgIRequest aRequest); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DecodeComplete(imgIRequest *aRequest) = 0;

  /* void discard (in imgIRequest aRequest); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Discard(imgIRequest *aRequest) = 0;

  /* void isAnimated (in imgIRequest aRequest); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsAnimated(imgIRequest *aRequest) = 0;

  /* void hasTransparency (in imgIRequest aRequest); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HasTransparency(imgIRequest *aRequest) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(imgIScriptedNotificationObserver, IMGISCRIPTEDNOTIFICATIONOBSERVER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_IMGISCRIPTEDNOTIFICATIONOBSERVER \
  NS_IMETHOD SizeAvailable(imgIRequest *aRequest) override; \
  NS_IMETHOD FrameUpdate(imgIRequest *aRequest) override; \
  NS_IMETHOD FrameComplete(imgIRequest *aRequest) override; \
  NS_IMETHOD LoadComplete(imgIRequest *aRequest) override; \
  NS_IMETHOD DecodeComplete(imgIRequest *aRequest) override; \
  NS_IMETHOD Discard(imgIRequest *aRequest) override; \
  NS_IMETHOD IsAnimated(imgIRequest *aRequest) override; \
  NS_IMETHOD HasTransparency(imgIRequest *aRequest) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_IMGISCRIPTEDNOTIFICATIONOBSERVER \
  nsresult SizeAvailable(imgIRequest *aRequest); \
  nsresult FrameUpdate(imgIRequest *aRequest); \
  nsresult FrameComplete(imgIRequest *aRequest); \
  nsresult LoadComplete(imgIRequest *aRequest); \
  nsresult DecodeComplete(imgIRequest *aRequest); \
  nsresult Discard(imgIRequest *aRequest); \
  nsresult IsAnimated(imgIRequest *aRequest); \
  nsresult HasTransparency(imgIRequest *aRequest); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_IMGISCRIPTEDNOTIFICATIONOBSERVER(_to) \
  NS_IMETHOD SizeAvailable(imgIRequest *aRequest) override { return _to SizeAvailable(aRequest); } \
  NS_IMETHOD FrameUpdate(imgIRequest *aRequest) override { return _to FrameUpdate(aRequest); } \
  NS_IMETHOD FrameComplete(imgIRequest *aRequest) override { return _to FrameComplete(aRequest); } \
  NS_IMETHOD LoadComplete(imgIRequest *aRequest) override { return _to LoadComplete(aRequest); } \
  NS_IMETHOD DecodeComplete(imgIRequest *aRequest) override { return _to DecodeComplete(aRequest); } \
  NS_IMETHOD Discard(imgIRequest *aRequest) override { return _to Discard(aRequest); } \
  NS_IMETHOD IsAnimated(imgIRequest *aRequest) override { return _to IsAnimated(aRequest); } \
  NS_IMETHOD HasTransparency(imgIRequest *aRequest) override { return _to HasTransparency(aRequest); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_IMGISCRIPTEDNOTIFICATIONOBSERVER(_to) \
  NS_IMETHOD SizeAvailable(imgIRequest *aRequest) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SizeAvailable(aRequest); } \
  NS_IMETHOD FrameUpdate(imgIRequest *aRequest) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FrameUpdate(aRequest); } \
  NS_IMETHOD FrameComplete(imgIRequest *aRequest) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FrameComplete(aRequest); } \
  NS_IMETHOD LoadComplete(imgIRequest *aRequest) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LoadComplete(aRequest); } \
  NS_IMETHOD DecodeComplete(imgIRequest *aRequest) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DecodeComplete(aRequest); } \
  NS_IMETHOD Discard(imgIRequest *aRequest) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Discard(aRequest); } \
  NS_IMETHOD IsAnimated(imgIRequest *aRequest) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsAnimated(aRequest); } \
  NS_IMETHOD HasTransparency(imgIRequest *aRequest) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasTransparency(aRequest); } 


#endif /* __gen_imgIScriptedNotificationObserver_h__ */
