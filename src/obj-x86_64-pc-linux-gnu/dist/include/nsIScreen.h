/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIScreen.idl
 */

#ifndef __gen_nsIScreen_h__
#define __gen_nsIScreen_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
/**
 * The display type of nsIScreen belongs to.
 */
enum class DisplayType: int32_t {
  DISPLAY_PRIMARY,  // primary screen
  DISPLAY_EXTERNAL, // wired displays, such as HDMI, DisplayPort, etc.
  DISPLAY_VIRTUAL   // wireless displays, such as Chromecast, WiFi-Display, etc.
};

/* starting interface:    nsIScreen */
#define NS_ISCREEN_IID_STR "826e80c8-d70f-42e2-8aa9-82c05f2a370a"

#define NS_ISCREEN_IID \
  {0x826e80c8, 0xd70f, 0x42e2, \
    { 0x8a, 0xa9, 0x82, 0xc0, 0x5f, 0x2a, 0x37, 0x0a }}

class NS_NO_VTABLE nsIScreen : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISCREEN_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIScreen;

  /* void GetRect (out long left, out long top, out long width, out long height); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRect(int32_t *left, int32_t *top, int32_t *width, int32_t *height) = 0;

  /* void GetAvailRect (out long left, out long top, out long width, out long height); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAvailRect(int32_t *left, int32_t *top, int32_t *width, int32_t *height) = 0;

  /* void GetRectDisplayPix (out long left, out long top, out long width, out long height); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRectDisplayPix(int32_t *left, int32_t *top, int32_t *width, int32_t *height) = 0;

  /* void GetAvailRectDisplayPix (out long left, out long top, out long width, out long height); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAvailRectDisplayPix(int32_t *left, int32_t *top, int32_t *width, int32_t *height) = 0;

  /* readonly attribute long pixelDepth; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPixelDepth(int32_t *aPixelDepth) = 0;

  /* readonly attribute long colorDepth; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetColorDepth(int32_t *aColorDepth) = 0;

  /* readonly attribute double contentsScaleFactor; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetContentsScaleFactor(double *aContentsScaleFactor) = 0;

  /* readonly attribute double defaultCSSScaleFactor; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDefaultCSSScaleFactor(double *aDefaultCSSScaleFactor) = 0;

  /* readonly attribute float dpi; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDpi(float *aDpi) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIScreen, NS_ISCREEN_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISCREEN \
  NS_IMETHOD GetRect(int32_t *left, int32_t *top, int32_t *width, int32_t *height) override; \
  NS_IMETHOD GetAvailRect(int32_t *left, int32_t *top, int32_t *width, int32_t *height) override; \
  NS_IMETHOD GetRectDisplayPix(int32_t *left, int32_t *top, int32_t *width, int32_t *height) override; \
  NS_IMETHOD GetAvailRectDisplayPix(int32_t *left, int32_t *top, int32_t *width, int32_t *height) override; \
  NS_IMETHOD GetPixelDepth(int32_t *aPixelDepth) override; \
  NS_IMETHOD GetColorDepth(int32_t *aColorDepth) override; \
  NS_IMETHOD GetContentsScaleFactor(double *aContentsScaleFactor) override; \
  NS_IMETHOD GetDefaultCSSScaleFactor(double *aDefaultCSSScaleFactor) override; \
  NS_IMETHOD GetDpi(float *aDpi) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISCREEN \
  nsresult GetRect(int32_t *left, int32_t *top, int32_t *width, int32_t *height); \
  nsresult GetAvailRect(int32_t *left, int32_t *top, int32_t *width, int32_t *height); \
  nsresult GetRectDisplayPix(int32_t *left, int32_t *top, int32_t *width, int32_t *height); \
  nsresult GetAvailRectDisplayPix(int32_t *left, int32_t *top, int32_t *width, int32_t *height); \
  nsresult GetPixelDepth(int32_t *aPixelDepth); \
  nsresult GetColorDepth(int32_t *aColorDepth); \
  nsresult GetContentsScaleFactor(double *aContentsScaleFactor); \
  nsresult GetDefaultCSSScaleFactor(double *aDefaultCSSScaleFactor); \
  nsresult GetDpi(float *aDpi); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISCREEN(_to) \
  NS_IMETHOD GetRect(int32_t *left, int32_t *top, int32_t *width, int32_t *height) override { return _to GetRect(left, top, width, height); } \
  NS_IMETHOD GetAvailRect(int32_t *left, int32_t *top, int32_t *width, int32_t *height) override { return _to GetAvailRect(left, top, width, height); } \
  NS_IMETHOD GetRectDisplayPix(int32_t *left, int32_t *top, int32_t *width, int32_t *height) override { return _to GetRectDisplayPix(left, top, width, height); } \
  NS_IMETHOD GetAvailRectDisplayPix(int32_t *left, int32_t *top, int32_t *width, int32_t *height) override { return _to GetAvailRectDisplayPix(left, top, width, height); } \
  NS_IMETHOD GetPixelDepth(int32_t *aPixelDepth) override { return _to GetPixelDepth(aPixelDepth); } \
  NS_IMETHOD GetColorDepth(int32_t *aColorDepth) override { return _to GetColorDepth(aColorDepth); } \
  NS_IMETHOD GetContentsScaleFactor(double *aContentsScaleFactor) override { return _to GetContentsScaleFactor(aContentsScaleFactor); } \
  NS_IMETHOD GetDefaultCSSScaleFactor(double *aDefaultCSSScaleFactor) override { return _to GetDefaultCSSScaleFactor(aDefaultCSSScaleFactor); } \
  NS_IMETHOD GetDpi(float *aDpi) override { return _to GetDpi(aDpi); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISCREEN(_to) \
  NS_IMETHOD GetRect(int32_t *left, int32_t *top, int32_t *width, int32_t *height) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRect(left, top, width, height); } \
  NS_IMETHOD GetAvailRect(int32_t *left, int32_t *top, int32_t *width, int32_t *height) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAvailRect(left, top, width, height); } \
  NS_IMETHOD GetRectDisplayPix(int32_t *left, int32_t *top, int32_t *width, int32_t *height) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRectDisplayPix(left, top, width, height); } \
  NS_IMETHOD GetAvailRectDisplayPix(int32_t *left, int32_t *top, int32_t *width, int32_t *height) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAvailRectDisplayPix(left, top, width, height); } \
  NS_IMETHOD GetPixelDepth(int32_t *aPixelDepth) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPixelDepth(aPixelDepth); } \
  NS_IMETHOD GetColorDepth(int32_t *aColorDepth) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetColorDepth(aColorDepth); } \
  NS_IMETHOD GetContentsScaleFactor(double *aContentsScaleFactor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContentsScaleFactor(aContentsScaleFactor); } \
  NS_IMETHOD GetDefaultCSSScaleFactor(double *aDefaultCSSScaleFactor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDefaultCSSScaleFactor(aDefaultCSSScaleFactor); } \
  NS_IMETHOD GetDpi(float *aDpi) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDpi(aDpi); } 


#endif /* __gen_nsIScreen_h__ */
