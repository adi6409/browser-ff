/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/browser-element/nsIBrowserElementAPI.idl
 */

#ifndef __gen_nsIBrowserElementAPI_h__
#define __gen_nsIBrowserElementAPI_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsFrameLoader; /* webidl FrameLoader */

#define BROWSER_ELEMENT_API_CONTRACTID "@mozilla.org/dom/browser-element-api;1"
#define BROWSER_ELEMENT_API_CID                                 \
    { 0x651db7e3, 0x1734, 0x4536,                               \
      { 0xb1, 0x5a, 0x5b, 0x3a, 0xe6, 0x44, 0x13, 0x4c } }

/* starting interface:    nsIBrowserElementAPI */
#define NS_IBROWSERELEMENTAPI_IID_STR "57758c10-6036-11e5-a837-0800200c9a66"

#define NS_IBROWSERELEMENTAPI_IID \
  {0x57758c10, 0x6036, 0x11e5, \
    { 0xa8, 0x37, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66 }}

class NS_NO_VTABLE nsIBrowserElementAPI : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IBROWSERELEMENTAPI_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIBrowserElementAPI;

  /* void destroyFrameScripts (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DestroyFrameScripts(void) = 0;

  /* void setFrameLoader (in FrameLoader frameLoader); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetFrameLoader(nsFrameLoader *frameLoader) = 0;

  /* void sendMouseEvent (in AString type, in uint32_t x, in uint32_t y, in uint32_t button, in uint32_t clickCount, in uint32_t mifiers); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SendMouseEvent(const nsAString& type, uint32_t x, uint32_t y, uint32_t button, uint32_t clickCount, uint32_t mifiers) = 0;

  /* void goBack (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GoBack(void) = 0;

  /* void goForward (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GoForward(void) = 0;

  /* void reload (in boolean hardReload); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Reload(bool hardReload) = 0;

  /* void stop (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Stop(void) = 0;

  /* Promise getCanGoBack (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCanGoBack(::mozilla::dom::Promise * * _retval) = 0;

  /* Promise getCanGoForward (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCanGoForward(::mozilla::dom::Promise * * _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIBrowserElementAPI, NS_IBROWSERELEMENTAPI_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIBROWSERELEMENTAPI \
  NS_IMETHOD DestroyFrameScripts(void) override; \
  NS_IMETHOD SetFrameLoader(nsFrameLoader *frameLoader) override; \
  NS_IMETHOD SendMouseEvent(const nsAString& type, uint32_t x, uint32_t y, uint32_t button, uint32_t clickCount, uint32_t mifiers) override; \
  NS_IMETHOD GoBack(void) override; \
  NS_IMETHOD GoForward(void) override; \
  NS_IMETHOD Reload(bool hardReload) override; \
  NS_IMETHOD Stop(void) override; \
  NS_IMETHOD GetCanGoBack(::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD GetCanGoForward(::mozilla::dom::Promise * * _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIBROWSERELEMENTAPI \
  nsresult DestroyFrameScripts(void); \
  nsresult SetFrameLoader(nsFrameLoader *frameLoader); \
  nsresult SendMouseEvent(const nsAString& type, uint32_t x, uint32_t y, uint32_t button, uint32_t clickCount, uint32_t mifiers); \
  nsresult GoBack(void); \
  nsresult GoForward(void); \
  nsresult Reload(bool hardReload); \
  nsresult Stop(void); \
  nsresult GetCanGoBack(::mozilla::dom::Promise * * _retval); \
  nsresult GetCanGoForward(::mozilla::dom::Promise * * _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIBROWSERELEMENTAPI(_to) \
  NS_IMETHOD DestroyFrameScripts(void) override { return _to DestroyFrameScripts(); } \
  NS_IMETHOD SetFrameLoader(nsFrameLoader *frameLoader) override { return _to SetFrameLoader(frameLoader); } \
  NS_IMETHOD SendMouseEvent(const nsAString& type, uint32_t x, uint32_t y, uint32_t button, uint32_t clickCount, uint32_t mifiers) override { return _to SendMouseEvent(type, x, y, button, clickCount, mifiers); } \
  NS_IMETHOD GoBack(void) override { return _to GoBack(); } \
  NS_IMETHOD GoForward(void) override { return _to GoForward(); } \
  NS_IMETHOD Reload(bool hardReload) override { return _to Reload(hardReload); } \
  NS_IMETHOD Stop(void) override { return _to Stop(); } \
  NS_IMETHOD GetCanGoBack(::mozilla::dom::Promise * * _retval) override { return _to GetCanGoBack(_retval); } \
  NS_IMETHOD GetCanGoForward(::mozilla::dom::Promise * * _retval) override { return _to GetCanGoForward(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIBROWSERELEMENTAPI(_to) \
  NS_IMETHOD DestroyFrameScripts(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DestroyFrameScripts(); } \
  NS_IMETHOD SetFrameLoader(nsFrameLoader *frameLoader) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFrameLoader(frameLoader); } \
  NS_IMETHOD SendMouseEvent(const nsAString& type, uint32_t x, uint32_t y, uint32_t button, uint32_t clickCount, uint32_t mifiers) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendMouseEvent(type, x, y, button, clickCount, mifiers); } \
  NS_IMETHOD GoBack(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GoBack(); } \
  NS_IMETHOD GoForward(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GoForward(); } \
  NS_IMETHOD Reload(bool hardReload) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Reload(hardReload); } \
  NS_IMETHOD Stop(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Stop(); } \
  NS_IMETHOD GetCanGoBack(::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCanGoBack(_retval); } \
  NS_IMETHOD GetCanGoForward(::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCanGoForward(_retval); } 


#endif /* __gen_nsIBrowserElementAPI_h__ */
