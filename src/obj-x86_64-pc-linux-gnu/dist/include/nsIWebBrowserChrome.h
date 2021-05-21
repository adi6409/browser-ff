/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/browser/nsIWebBrowserChrome.idl
 */

#ifndef __gen_nsIWebBrowserChrome_h__
#define __gen_nsIWebBrowserChrome_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIWebBrowser; /* forward declaration */

class nsIDocShellTreeItem; /* forward declaration */


/* starting interface:    nsIWebBrowserChrome */
#define NS_IWEBBROWSERCHROME_IID_STR "e8c414c4-dc38-4ba3-ab4e-ec4cbbe22907"

#define NS_IWEBBROWSERCHROME_IID \
  {0xe8c414c4, 0xdc38, 0x4ba3, \
    { 0xab, 0x4e, 0xec, 0x4c, 0xbb, 0xe2, 0x29, 0x07 }}

class NS_NO_VTABLE nsIWebBrowserChrome : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWEBBROWSERCHROME_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWebBrowserChrome;

  /* void setLinkStatus (in AString status); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetLinkStatus(const nsAString& status) = 0;

  enum {
    CHROME_DEFAULT = 1U,
    CHROME_WINDOW_BORDERS = 2U,
    CHROME_WINDOW_CLOSE = 4U,
    CHROME_WINDOW_RESIZE = 8U,
    CHROME_MENUBAR = 16U,
    CHROME_TOOLBAR = 32U,
    CHROME_LOCATIONBAR = 64U,
    CHROME_STATUSBAR = 128U,
    CHROME_PERSONAL_TOOLBAR = 256U,
    CHROME_SCROLLBARS = 512U,
    CHROME_TITLEBAR = 1024U,
    CHROME_EXTRA = 2048U,
    CHROME_WITH_SIZE = 4096U,
    CHROME_WITH_POSITION = 8192U,
    CHROME_WINDOW_MIN = 16384U,
    CHROME_WINDOW_POPUP = 32768U,
    CHROME_PRIVATE_WINDOW = 65536U,
    CHROME_NON_PRIVATE_WINDOW = 131072U,
    CHROME_PRIVATE_LIFETIME = 262144U,
    CHROME_ALWAYS_ON_TOP = 524288U,
    CHROME_REMOTE_WINDOW = 1048576U,
    CHROME_FISSION_WINDOW = 2097152U,
    CHROME_SUPPRESS_ANIMATION = 16777216U,
    CHROME_WINDOW_RAISED = 33554432U,
    CHROME_WINDOW_LOWERED = 67108864U,
    CHROME_CENTER_SCREEN = 134217728U,
    CHROME_DEPENDENT = 268435456U,
    CHROME_MODAL = 536870912U,
    CHROME_OPENAS_DIALOG = 1073741824U,
    CHROME_OPENAS_CHROME = 2147483648U,
    CHROME_ALL = 4094U
  };

  /* attribute unsigned long chromeFlags; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetChromeFlags(uint32_t *aChromeFlags) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetChromeFlags(uint32_t aChromeFlags) = 0;

  /* void showAsModal (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ShowAsModal(void) = 0;

  /* boolean isWindowModal (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsWindowModal(bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWebBrowserChrome, NS_IWEBBROWSERCHROME_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWEBBROWSERCHROME \
  NS_IMETHOD SetLinkStatus(const nsAString& status) override; \
  NS_IMETHOD GetChromeFlags(uint32_t *aChromeFlags) override; \
  NS_IMETHOD SetChromeFlags(uint32_t aChromeFlags) override; \
  NS_IMETHOD ShowAsModal(void) override; \
  NS_IMETHOD IsWindowModal(bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWEBBROWSERCHROME \
  nsresult SetLinkStatus(const nsAString& status); \
  nsresult GetChromeFlags(uint32_t *aChromeFlags); \
  nsresult SetChromeFlags(uint32_t aChromeFlags); \
  nsresult ShowAsModal(void); \
  nsresult IsWindowModal(bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWEBBROWSERCHROME(_to) \
  NS_IMETHOD SetLinkStatus(const nsAString& status) override { return _to SetLinkStatus(status); } \
  NS_IMETHOD GetChromeFlags(uint32_t *aChromeFlags) override { return _to GetChromeFlags(aChromeFlags); } \
  NS_IMETHOD SetChromeFlags(uint32_t aChromeFlags) override { return _to SetChromeFlags(aChromeFlags); } \
  NS_IMETHOD ShowAsModal(void) override { return _to ShowAsModal(); } \
  NS_IMETHOD IsWindowModal(bool *_retval) override { return _to IsWindowModal(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWEBBROWSERCHROME(_to) \
  NS_IMETHOD SetLinkStatus(const nsAString& status) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLinkStatus(status); } \
  NS_IMETHOD GetChromeFlags(uint32_t *aChromeFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChromeFlags(aChromeFlags); } \
  NS_IMETHOD SetChromeFlags(uint32_t aChromeFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetChromeFlags(aChromeFlags); } \
  NS_IMETHOD ShowAsModal(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ShowAsModal(); } \
  NS_IMETHOD IsWindowModal(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsWindowModal(_retval); } 


#endif /* __gen_nsIWebBrowserChrome_h__ */
