/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/image/imgITools.idl
 */

#ifndef __gen_imgITools_h__
#define __gen_imgITools_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIChannel; /* forward declaration */

class nsIEventTarget; /* forward declaration */

class nsIInputStream; /* forward declaration */

class nsIURI; /* forward declaration */

class imgIContainer; /* forward declaration */

class imgILoader; /* forward declaration */

class imgICache; /* forward declaration */

class imgIScriptedNotificationObserver; /* forward declaration */

class imgINotificationObserver; /* forward declaration */

class imgIContainerCallback; /* forward declaration */

namespace mozilla {
namespace dom {
class Document; /* webidl Document */
} // namespace dom
} // namespace mozilla


/* starting interface:    imgITools */
#define IMGITOOLS_IID_STR "4c2383a4-931c-484d-8c4a-973590f66e3f"

#define IMGITOOLS_IID \
  {0x4c2383a4, 0x931c, 0x484d, \
    { 0x8c, 0x4a, 0x97, 0x35, 0x90, 0xf6, 0x6e, 0x3f }}

class NS_NO_VTABLE imgITools : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(IMGITOOLS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = imgITools;

  /* imgIContainer decodeImageFromBuffer (in string aBuffer, in unsigned long aSize, in ACString aMimeType); */
  NS_IMETHOD DecodeImageFromBuffer(const char * aBuffer, uint32_t aSize, const nsACString& aMimeType, imgIContainer **_retval) = 0;

  /* [implicit_jscontext] imgIContainer decodeImageFromArrayBuffer (in jsval aArrayBuffer, in ACString aMimeType); */
  NS_IMETHOD DecodeImageFromArrayBuffer(JS::HandleValue aArrayBuffer, const nsACString& aMimeType, JSContext* cx, imgIContainer **_retval) = 0;

  /* void decodeImageFromChannelAsync (in nsIURI aURI, in nsIChannel aChannel, in imgIContainerCallback aCallback, in imgINotificationObserver aObserver); */
  NS_IMETHOD DecodeImageFromChannelAsync(nsIURI *aURI, nsIChannel *aChannel, imgIContainerCallback *aCallback, imgINotificationObserver *aObserver) = 0;

  /* void decodeImageAsync (in nsIInputStream aStream, in ACString aMimeType, in imgIContainerCallback aCallback, in nsIEventTarget aEventTarget); */
  NS_IMETHOD DecodeImageAsync(nsIInputStream *aStream, const nsACString& aMimeType, imgIContainerCallback *aCallback, nsIEventTarget *aEventTarget) = 0;

  /* nsIInputStream encodeImage (in imgIContainer aContainer, in ACString aMimeType, [optional] in AString outputOptions); */
  NS_IMETHOD EncodeImage(imgIContainer *aContainer, const nsACString& aMimeType, const nsAString& outputOptions, nsIInputStream **_retval) = 0;

  /* nsIInputStream encodeScaledImage (in imgIContainer aContainer, in ACString aMimeType, in long aWidth, in long aHeight, [optional] in AString outputOptions); */
  NS_IMETHOD EncodeScaledImage(imgIContainer *aContainer, const nsACString& aMimeType, int32_t aWidth, int32_t aHeight, const nsAString& outputOptions, nsIInputStream **_retval) = 0;

  /* imgILoader getImgLoaderForDocument (in Document doc); */
  NS_IMETHOD GetImgLoaderForDocument(mozilla::dom::Document *doc, imgILoader **_retval) = 0;

  /* imgICache getImgCacheForDocument (in Document doc); */
  NS_IMETHOD GetImgCacheForDocument(mozilla::dom::Document *doc, imgICache **_retval) = 0;

  /* nsIInputStream encodeCroppedImage (in imgIContainer aContainer, in ACString aMimeType, in long aOffsetX, in long aOffsetY, in long aWidth, in long aHeight, [optional] in AString outputOptions); */
  NS_IMETHOD EncodeCroppedImage(imgIContainer *aContainer, const nsACString& aMimeType, int32_t aOffsetX, int32_t aOffsetY, int32_t aWidth, int32_t aHeight, const nsAString& outputOptions, nsIInputStream **_retval) = 0;

  /* imgINotificationObserver createScriptedObserver (in imgIScriptedNotificationObserver aObserver); */
  NS_IMETHOD CreateScriptedObserver(imgIScriptedNotificationObserver *aObserver, imgINotificationObserver **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(imgITools, IMGITOOLS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_IMGITOOLS \
  NS_IMETHOD DecodeImageFromBuffer(const char * aBuffer, uint32_t aSize, const nsACString& aMimeType, imgIContainer **_retval) override; \
  NS_IMETHOD DecodeImageFromArrayBuffer(JS::HandleValue aArrayBuffer, const nsACString& aMimeType, JSContext* cx, imgIContainer **_retval) override; \
  NS_IMETHOD DecodeImageFromChannelAsync(nsIURI *aURI, nsIChannel *aChannel, imgIContainerCallback *aCallback, imgINotificationObserver *aObserver) override; \
  NS_IMETHOD DecodeImageAsync(nsIInputStream *aStream, const nsACString& aMimeType, imgIContainerCallback *aCallback, nsIEventTarget *aEventTarget) override; \
  NS_IMETHOD EncodeImage(imgIContainer *aContainer, const nsACString& aMimeType, const nsAString& outputOptions, nsIInputStream **_retval) override; \
  NS_IMETHOD EncodeScaledImage(imgIContainer *aContainer, const nsACString& aMimeType, int32_t aWidth, int32_t aHeight, const nsAString& outputOptions, nsIInputStream **_retval) override; \
  NS_IMETHOD GetImgLoaderForDocument(mozilla::dom::Document *doc, imgILoader **_retval) override; \
  NS_IMETHOD GetImgCacheForDocument(mozilla::dom::Document *doc, imgICache **_retval) override; \
  NS_IMETHOD EncodeCroppedImage(imgIContainer *aContainer, const nsACString& aMimeType, int32_t aOffsetX, int32_t aOffsetY, int32_t aWidth, int32_t aHeight, const nsAString& outputOptions, nsIInputStream **_retval) override; \
  NS_IMETHOD CreateScriptedObserver(imgIScriptedNotificationObserver *aObserver, imgINotificationObserver **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_IMGITOOLS \
  nsresult DecodeImageFromBuffer(const char * aBuffer, uint32_t aSize, const nsACString& aMimeType, imgIContainer **_retval); \
  nsresult DecodeImageFromArrayBuffer(JS::HandleValue aArrayBuffer, const nsACString& aMimeType, JSContext* cx, imgIContainer **_retval); \
  nsresult DecodeImageFromChannelAsync(nsIURI *aURI, nsIChannel *aChannel, imgIContainerCallback *aCallback, imgINotificationObserver *aObserver); \
  nsresult DecodeImageAsync(nsIInputStream *aStream, const nsACString& aMimeType, imgIContainerCallback *aCallback, nsIEventTarget *aEventTarget); \
  nsresult EncodeImage(imgIContainer *aContainer, const nsACString& aMimeType, const nsAString& outputOptions, nsIInputStream **_retval); \
  nsresult EncodeScaledImage(imgIContainer *aContainer, const nsACString& aMimeType, int32_t aWidth, int32_t aHeight, const nsAString& outputOptions, nsIInputStream **_retval); \
  nsresult GetImgLoaderForDocument(mozilla::dom::Document *doc, imgILoader **_retval); \
  nsresult GetImgCacheForDocument(mozilla::dom::Document *doc, imgICache **_retval); \
  nsresult EncodeCroppedImage(imgIContainer *aContainer, const nsACString& aMimeType, int32_t aOffsetX, int32_t aOffsetY, int32_t aWidth, int32_t aHeight, const nsAString& outputOptions, nsIInputStream **_retval); \
  nsresult CreateScriptedObserver(imgIScriptedNotificationObserver *aObserver, imgINotificationObserver **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_IMGITOOLS(_to) \
  NS_IMETHOD DecodeImageFromBuffer(const char * aBuffer, uint32_t aSize, const nsACString& aMimeType, imgIContainer **_retval) override { return _to DecodeImageFromBuffer(aBuffer, aSize, aMimeType, _retval); } \
  NS_IMETHOD DecodeImageFromArrayBuffer(JS::HandleValue aArrayBuffer, const nsACString& aMimeType, JSContext* cx, imgIContainer **_retval) override { return _to DecodeImageFromArrayBuffer(aArrayBuffer, aMimeType, cx, _retval); } \
  NS_IMETHOD DecodeImageFromChannelAsync(nsIURI *aURI, nsIChannel *aChannel, imgIContainerCallback *aCallback, imgINotificationObserver *aObserver) override { return _to DecodeImageFromChannelAsync(aURI, aChannel, aCallback, aObserver); } \
  NS_IMETHOD DecodeImageAsync(nsIInputStream *aStream, const nsACString& aMimeType, imgIContainerCallback *aCallback, nsIEventTarget *aEventTarget) override { return _to DecodeImageAsync(aStream, aMimeType, aCallback, aEventTarget); } \
  NS_IMETHOD EncodeImage(imgIContainer *aContainer, const nsACString& aMimeType, const nsAString& outputOptions, nsIInputStream **_retval) override { return _to EncodeImage(aContainer, aMimeType, outputOptions, _retval); } \
  NS_IMETHOD EncodeScaledImage(imgIContainer *aContainer, const nsACString& aMimeType, int32_t aWidth, int32_t aHeight, const nsAString& outputOptions, nsIInputStream **_retval) override { return _to EncodeScaledImage(aContainer, aMimeType, aWidth, aHeight, outputOptions, _retval); } \
  NS_IMETHOD GetImgLoaderForDocument(mozilla::dom::Document *doc, imgILoader **_retval) override { return _to GetImgLoaderForDocument(doc, _retval); } \
  NS_IMETHOD GetImgCacheForDocument(mozilla::dom::Document *doc, imgICache **_retval) override { return _to GetImgCacheForDocument(doc, _retval); } \
  NS_IMETHOD EncodeCroppedImage(imgIContainer *aContainer, const nsACString& aMimeType, int32_t aOffsetX, int32_t aOffsetY, int32_t aWidth, int32_t aHeight, const nsAString& outputOptions, nsIInputStream **_retval) override { return _to EncodeCroppedImage(aContainer, aMimeType, aOffsetX, aOffsetY, aWidth, aHeight, outputOptions, _retval); } \
  NS_IMETHOD CreateScriptedObserver(imgIScriptedNotificationObserver *aObserver, imgINotificationObserver **_retval) override { return _to CreateScriptedObserver(aObserver, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_IMGITOOLS(_to) \
  NS_IMETHOD DecodeImageFromBuffer(const char * aBuffer, uint32_t aSize, const nsACString& aMimeType, imgIContainer **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DecodeImageFromBuffer(aBuffer, aSize, aMimeType, _retval); } \
  NS_IMETHOD DecodeImageFromArrayBuffer(JS::HandleValue aArrayBuffer, const nsACString& aMimeType, JSContext* cx, imgIContainer **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DecodeImageFromArrayBuffer(aArrayBuffer, aMimeType, cx, _retval); } \
  NS_IMETHOD DecodeImageFromChannelAsync(nsIURI *aURI, nsIChannel *aChannel, imgIContainerCallback *aCallback, imgINotificationObserver *aObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DecodeImageFromChannelAsync(aURI, aChannel, aCallback, aObserver); } \
  NS_IMETHOD DecodeImageAsync(nsIInputStream *aStream, const nsACString& aMimeType, imgIContainerCallback *aCallback, nsIEventTarget *aEventTarget) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DecodeImageAsync(aStream, aMimeType, aCallback, aEventTarget); } \
  NS_IMETHOD EncodeImage(imgIContainer *aContainer, const nsACString& aMimeType, const nsAString& outputOptions, nsIInputStream **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EncodeImage(aContainer, aMimeType, outputOptions, _retval); } \
  NS_IMETHOD EncodeScaledImage(imgIContainer *aContainer, const nsACString& aMimeType, int32_t aWidth, int32_t aHeight, const nsAString& outputOptions, nsIInputStream **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EncodeScaledImage(aContainer, aMimeType, aWidth, aHeight, outputOptions, _retval); } \
  NS_IMETHOD GetImgLoaderForDocument(mozilla::dom::Document *doc, imgILoader **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetImgLoaderForDocument(doc, _retval); } \
  NS_IMETHOD GetImgCacheForDocument(mozilla::dom::Document *doc, imgICache **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetImgCacheForDocument(doc, _retval); } \
  NS_IMETHOD EncodeCroppedImage(imgIContainer *aContainer, const nsACString& aMimeType, int32_t aOffsetX, int32_t aOffsetY, int32_t aWidth, int32_t aHeight, const nsAString& outputOptions, nsIInputStream **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EncodeCroppedImage(aContainer, aMimeType, aOffsetX, aOffsetY, aWidth, aHeight, outputOptions, _retval); } \
  NS_IMETHOD CreateScriptedObserver(imgIScriptedNotificationObserver *aObserver, imgINotificationObserver **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateScriptedObserver(aObserver, _retval); } 


/* starting interface:    imgIContainerCallback */
#define IMGICONTAINERCALLBACK_IID_STR "f195772c-a4c0-47ae-80ca-211e001c67be"

#define IMGICONTAINERCALLBACK_IID \
  {0xf195772c, 0xa4c0, 0x47ae, \
    { 0x80, 0xca, 0x21, 0x1e, 0x00, 0x1c, 0x67, 0xbe }}

class NS_NO_VTABLE imgIContainerCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(IMGICONTAINERCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = imgIContainerCallback;

  /* void onImageReady (in imgIContainer aImage, in nsresult aStatus); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnImageReady(imgIContainer *aImage, nsresult aStatus) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(imgIContainerCallback, IMGICONTAINERCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_IMGICONTAINERCALLBACK \
  NS_IMETHOD OnImageReady(imgIContainer *aImage, nsresult aStatus) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_IMGICONTAINERCALLBACK \
  nsresult OnImageReady(imgIContainer *aImage, nsresult aStatus); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_IMGICONTAINERCALLBACK(_to) \
  NS_IMETHOD OnImageReady(imgIContainer *aImage, nsresult aStatus) override { return _to OnImageReady(aImage, aStatus); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_IMGICONTAINERCALLBACK(_to) \
  NS_IMETHOD OnImageReady(imgIContainer *aImage, nsresult aStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnImageReady(aImage, aStatus); } 


#endif /* __gen_imgITools_h__ */
