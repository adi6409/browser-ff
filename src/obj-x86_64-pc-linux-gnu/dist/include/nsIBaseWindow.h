/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIBaseWindow.idl
 */

#ifndef __gen_nsIBaseWindow_h__
#define __gen_nsIBaseWindow_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsrootidl_h__
#include "nsrootidl.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIWidget;
typedef void *  nativeWindow;


/* starting interface:    nsIBaseWindow */
#define NS_IBASEWINDOW_IID_STR "ca635529-a977-4552-9b8a-66187e54d882"

#define NS_IBASEWINDOW_IID \
  {0xca635529, 0xa977, 0x4552, \
    { 0x9b, 0x8a, 0x66, 0x18, 0x7e, 0x54, 0xd8, 0x82 }}

class NS_NO_VTABLE nsIBaseWindow : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IBASEWINDOW_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIBaseWindow;

  /* [noscript] void initWindow (in nativeWindow parentNativeWindow, in nsIWidget parentWidget, in long x, in long y, in long cx, in long cy); */
  NS_IMETHOD InitWindow(nativeWindow parentNativeWindow, nsIWidget * parentWidget, int32_t x, int32_t y, int32_t cx, int32_t cy) = 0;

  /* void destroy (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Destroy(void) = 0;

  /* void setPosition (in long x, in long y); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPosition(int32_t x, int32_t y) = 0;

  /* void setPositionDesktopPix (in long x, in long y); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPositionDesktopPix(int32_t x, int32_t y) = 0;

  /* void getPosition (out long x, out long y); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPosition(int32_t *x, int32_t *y) = 0;

  /* void setSize (in long cx, in long cy, in boolean fRepaint); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSize(int32_t cx, int32_t cy, bool fRepaint) = 0;

  /* void getSize (out long cx, out long cy); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSize(int32_t *cx, int32_t *cy) = 0;

  enum {
    eRepaint = 1U,
    eDelayResize = 2U
  };

  /* void setPositionAndSize (in long x, in long y, in long cx, in long cy, in unsigned long flags); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPositionAndSize(int32_t x, int32_t y, int32_t cx, int32_t cy, uint32_t flags) = 0;

  /* void getPositionAndSize (out long x, out long y, out long cx, out long cy); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPositionAndSize(int32_t *x, int32_t *y, int32_t *cx, int32_t *cy) = 0;

  /* void repaint (in boolean force); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Repaint(bool force) = 0;

  /* [noscript] attribute nsIWidget parentWidget; */
  NS_IMETHOD GetParentWidget(nsIWidget * * aParentWidget) = 0;
  NS_IMETHOD SetParentWidget(nsIWidget * aParentWidget) = 0;

  /* attribute nativeWindow parentNativeWindow; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetParentNativeWindow(nativeWindow *aParentNativeWindow) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetParentNativeWindow(nativeWindow aParentNativeWindow) = 0;

  /* readonly attribute AString nativeHandle; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNativeHandle(nsAString& aNativeHandle) = 0;

  /* attribute boolean visibility; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetVisibility(bool *aVisibility) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetVisibility(bool aVisibility) = 0;

  /* attribute boolean enabled; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEnabled(bool *aEnabled) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetEnabled(bool aEnabled) = 0;

  /* [noscript] readonly attribute nsIWidget mainWidget; */
  NS_IMETHOD GetMainWidget(nsIWidget * * aMainWidget) = 0;

  /* readonly attribute double unscaledDevicePixelsPerCSSPixel; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUnscaledDevicePixelsPerCSSPixel(double *aUnscaledDevicePixelsPerCSSPixel) = 0;

  /* readonly attribute double devicePixelsPerDesktopPixel; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDevicePixelsPerDesktopPixel(double *aDevicePixelsPerDesktopPixel) = 0;

  /* void setFocus (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetFocus(void) = 0;

  /* attribute AString title; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTitle(nsAString& aTitle) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetTitle(const nsAString& aTitle) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIBaseWindow, NS_IBASEWINDOW_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIBASEWINDOW \
  NS_IMETHOD InitWindow(nativeWindow parentNativeWindow, nsIWidget * parentWidget, int32_t x, int32_t y, int32_t cx, int32_t cy) override; \
  NS_IMETHOD Destroy(void) override; \
  NS_IMETHOD SetPosition(int32_t x, int32_t y) override; \
  NS_IMETHOD SetPositionDesktopPix(int32_t x, int32_t y) override; \
  NS_IMETHOD GetPosition(int32_t *x, int32_t *y) override; \
  NS_IMETHOD SetSize(int32_t cx, int32_t cy, bool fRepaint) override; \
  NS_IMETHOD GetSize(int32_t *cx, int32_t *cy) override; \
  NS_IMETHOD SetPositionAndSize(int32_t x, int32_t y, int32_t cx, int32_t cy, uint32_t flags) override; \
  NS_IMETHOD GetPositionAndSize(int32_t *x, int32_t *y, int32_t *cx, int32_t *cy) override; \
  NS_IMETHOD Repaint(bool force) override; \
  NS_IMETHOD GetParentWidget(nsIWidget * * aParentWidget) override; \
  NS_IMETHOD SetParentWidget(nsIWidget * aParentWidget) override; \
  NS_IMETHOD GetParentNativeWindow(nativeWindow *aParentNativeWindow) override; \
  NS_IMETHOD SetParentNativeWindow(nativeWindow aParentNativeWindow) override; \
  NS_IMETHOD GetNativeHandle(nsAString& aNativeHandle) override; \
  NS_IMETHOD GetVisibility(bool *aVisibility) override; \
  NS_IMETHOD SetVisibility(bool aVisibility) override; \
  NS_IMETHOD GetEnabled(bool *aEnabled) override; \
  NS_IMETHOD SetEnabled(bool aEnabled) override; \
  NS_IMETHOD GetMainWidget(nsIWidget * * aMainWidget) override; \
  NS_IMETHOD GetUnscaledDevicePixelsPerCSSPixel(double *aUnscaledDevicePixelsPerCSSPixel) override; \
  NS_IMETHOD GetDevicePixelsPerDesktopPixel(double *aDevicePixelsPerDesktopPixel) override; \
  NS_IMETHOD SetFocus(void) override; \
  NS_IMETHOD GetTitle(nsAString& aTitle) override; \
  NS_IMETHOD SetTitle(const nsAString& aTitle) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIBASEWINDOW \
  nsresult InitWindow(nativeWindow parentNativeWindow, nsIWidget * parentWidget, int32_t x, int32_t y, int32_t cx, int32_t cy); \
  nsresult Destroy(void); \
  nsresult SetPosition(int32_t x, int32_t y); \
  nsresult SetPositionDesktopPix(int32_t x, int32_t y); \
  nsresult GetPosition(int32_t *x, int32_t *y); \
  nsresult SetSize(int32_t cx, int32_t cy, bool fRepaint); \
  nsresult GetSize(int32_t *cx, int32_t *cy); \
  nsresult SetPositionAndSize(int32_t x, int32_t y, int32_t cx, int32_t cy, uint32_t flags); \
  nsresult GetPositionAndSize(int32_t *x, int32_t *y, int32_t *cx, int32_t *cy); \
  nsresult Repaint(bool force); \
  nsresult GetParentWidget(nsIWidget * * aParentWidget); \
  nsresult SetParentWidget(nsIWidget * aParentWidget); \
  nsresult GetParentNativeWindow(nativeWindow *aParentNativeWindow); \
  nsresult SetParentNativeWindow(nativeWindow aParentNativeWindow); \
  nsresult GetNativeHandle(nsAString& aNativeHandle); \
  nsresult GetVisibility(bool *aVisibility); \
  nsresult SetVisibility(bool aVisibility); \
  nsresult GetEnabled(bool *aEnabled); \
  nsresult SetEnabled(bool aEnabled); \
  nsresult GetMainWidget(nsIWidget * * aMainWidget); \
  nsresult GetUnscaledDevicePixelsPerCSSPixel(double *aUnscaledDevicePixelsPerCSSPixel); \
  nsresult GetDevicePixelsPerDesktopPixel(double *aDevicePixelsPerDesktopPixel); \
  nsresult SetFocus(void); \
  nsresult GetTitle(nsAString& aTitle); \
  nsresult SetTitle(const nsAString& aTitle); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIBASEWINDOW(_to) \
  NS_IMETHOD InitWindow(nativeWindow parentNativeWindow, nsIWidget * parentWidget, int32_t x, int32_t y, int32_t cx, int32_t cy) override { return _to InitWindow(parentNativeWindow, parentWidget, x, y, cx, cy); } \
  NS_IMETHOD Destroy(void) override { return _to Destroy(); } \
  NS_IMETHOD SetPosition(int32_t x, int32_t y) override { return _to SetPosition(x, y); } \
  NS_IMETHOD SetPositionDesktopPix(int32_t x, int32_t y) override { return _to SetPositionDesktopPix(x, y); } \
  NS_IMETHOD GetPosition(int32_t *x, int32_t *y) override { return _to GetPosition(x, y); } \
  NS_IMETHOD SetSize(int32_t cx, int32_t cy, bool fRepaint) override { return _to SetSize(cx, cy, fRepaint); } \
  NS_IMETHOD GetSize(int32_t *cx, int32_t *cy) override { return _to GetSize(cx, cy); } \
  NS_IMETHOD SetPositionAndSize(int32_t x, int32_t y, int32_t cx, int32_t cy, uint32_t flags) override { return _to SetPositionAndSize(x, y, cx, cy, flags); } \
  NS_IMETHOD GetPositionAndSize(int32_t *x, int32_t *y, int32_t *cx, int32_t *cy) override { return _to GetPositionAndSize(x, y, cx, cy); } \
  NS_IMETHOD Repaint(bool force) override { return _to Repaint(force); } \
  NS_IMETHOD GetParentWidget(nsIWidget * * aParentWidget) override { return _to GetParentWidget(aParentWidget); } \
  NS_IMETHOD SetParentWidget(nsIWidget * aParentWidget) override { return _to SetParentWidget(aParentWidget); } \
  NS_IMETHOD GetParentNativeWindow(nativeWindow *aParentNativeWindow) override { return _to GetParentNativeWindow(aParentNativeWindow); } \
  NS_IMETHOD SetParentNativeWindow(nativeWindow aParentNativeWindow) override { return _to SetParentNativeWindow(aParentNativeWindow); } \
  NS_IMETHOD GetNativeHandle(nsAString& aNativeHandle) override { return _to GetNativeHandle(aNativeHandle); } \
  NS_IMETHOD GetVisibility(bool *aVisibility) override { return _to GetVisibility(aVisibility); } \
  NS_IMETHOD SetVisibility(bool aVisibility) override { return _to SetVisibility(aVisibility); } \
  NS_IMETHOD GetEnabled(bool *aEnabled) override { return _to GetEnabled(aEnabled); } \
  NS_IMETHOD SetEnabled(bool aEnabled) override { return _to SetEnabled(aEnabled); } \
  NS_IMETHOD GetMainWidget(nsIWidget * * aMainWidget) override { return _to GetMainWidget(aMainWidget); } \
  NS_IMETHOD GetUnscaledDevicePixelsPerCSSPixel(double *aUnscaledDevicePixelsPerCSSPixel) override { return _to GetUnscaledDevicePixelsPerCSSPixel(aUnscaledDevicePixelsPerCSSPixel); } \
  NS_IMETHOD GetDevicePixelsPerDesktopPixel(double *aDevicePixelsPerDesktopPixel) override { return _to GetDevicePixelsPerDesktopPixel(aDevicePixelsPerDesktopPixel); } \
  NS_IMETHOD SetFocus(void) override { return _to SetFocus(); } \
  NS_IMETHOD GetTitle(nsAString& aTitle) override { return _to GetTitle(aTitle); } \
  NS_IMETHOD SetTitle(const nsAString& aTitle) override { return _to SetTitle(aTitle); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIBASEWINDOW(_to) \
  NS_IMETHOD InitWindow(nativeWindow parentNativeWindow, nsIWidget * parentWidget, int32_t x, int32_t y, int32_t cx, int32_t cy) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitWindow(parentNativeWindow, parentWidget, x, y, cx, cy); } \
  NS_IMETHOD Destroy(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Destroy(); } \
  NS_IMETHOD SetPosition(int32_t x, int32_t y) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPosition(x, y); } \
  NS_IMETHOD SetPositionDesktopPix(int32_t x, int32_t y) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPositionDesktopPix(x, y); } \
  NS_IMETHOD GetPosition(int32_t *x, int32_t *y) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPosition(x, y); } \
  NS_IMETHOD SetSize(int32_t cx, int32_t cy, bool fRepaint) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSize(cx, cy, fRepaint); } \
  NS_IMETHOD GetSize(int32_t *cx, int32_t *cy) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSize(cx, cy); } \
  NS_IMETHOD SetPositionAndSize(int32_t x, int32_t y, int32_t cx, int32_t cy, uint32_t flags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPositionAndSize(x, y, cx, cy, flags); } \
  NS_IMETHOD GetPositionAndSize(int32_t *x, int32_t *y, int32_t *cx, int32_t *cy) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPositionAndSize(x, y, cx, cy); } \
  NS_IMETHOD Repaint(bool force) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Repaint(force); } \
  NS_IMETHOD GetParentWidget(nsIWidget * * aParentWidget) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetParentWidget(aParentWidget); } \
  NS_IMETHOD SetParentWidget(nsIWidget * aParentWidget) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetParentWidget(aParentWidget); } \
  NS_IMETHOD GetParentNativeWindow(nativeWindow *aParentNativeWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetParentNativeWindow(aParentNativeWindow); } \
  NS_IMETHOD SetParentNativeWindow(nativeWindow aParentNativeWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetParentNativeWindow(aParentNativeWindow); } \
  NS_IMETHOD GetNativeHandle(nsAString& aNativeHandle) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNativeHandle(aNativeHandle); } \
  NS_IMETHOD GetVisibility(bool *aVisibility) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetVisibility(aVisibility); } \
  NS_IMETHOD SetVisibility(bool aVisibility) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetVisibility(aVisibility); } \
  NS_IMETHOD GetEnabled(bool *aEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEnabled(aEnabled); } \
  NS_IMETHOD SetEnabled(bool aEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetEnabled(aEnabled); } \
  NS_IMETHOD GetMainWidget(nsIWidget * * aMainWidget) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMainWidget(aMainWidget); } \
  NS_IMETHOD GetUnscaledDevicePixelsPerCSSPixel(double *aUnscaledDevicePixelsPerCSSPixel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUnscaledDevicePixelsPerCSSPixel(aUnscaledDevicePixelsPerCSSPixel); } \
  NS_IMETHOD GetDevicePixelsPerDesktopPixel(double *aDevicePixelsPerDesktopPixel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDevicePixelsPerDesktopPixel(aDevicePixelsPerDesktopPixel); } \
  NS_IMETHOD SetFocus(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFocus(); } \
  NS_IMETHOD GetTitle(nsAString& aTitle) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTitle(aTitle); } \
  NS_IMETHOD SetTitle(const nsAString& aTitle) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTitle(aTitle); } 


#endif /* __gen_nsIBaseWindow_h__ */
