/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIDOMChromeWindow.idl
 */

#ifndef __gen_nsIDOMChromeWindow_h__
#define __gen_nsIDOMChromeWindow_h__


#ifndef __gen_domstubs_h__
#include "domstubs.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIBrowserDOMWindow; /* forward declaration */

class mozIDOMWindowProxy; /* forward declaration */


/* starting interface:    nsIDOMChromeWindow */
#define NS_IDOMCHROMEWINDOW_IID_STR "78bdcb41-1efa-409f-aaba-70842213f80f"

#define NS_IDOMCHROMEWINDOW_IID \
  {0x78bdcb41, 0x1efa, 0x409f, \
    { 0xaa, 0xba, 0x70, 0x84, 0x22, 0x13, 0xf8, 0x0f }}

class NS_NO_VTABLE nsIDOMChromeWindow : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOMCHROMEWINDOW_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDOMChromeWindow;

  /* [noscript] readonly attribute nsIBrowserDOMWindow browserDOMWindow; */
  NS_IMETHOD GetBrowserDOMWindow(nsIBrowserDOMWindow **aBrowserDOMWindow) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDOMChromeWindow, NS_IDOMCHROMEWINDOW_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOMCHROMEWINDOW \
  NS_IMETHOD GetBrowserDOMWindow(nsIBrowserDOMWindow **aBrowserDOMWindow) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOMCHROMEWINDOW \
  nsresult GetBrowserDOMWindow(nsIBrowserDOMWindow **aBrowserDOMWindow); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOMCHROMEWINDOW(_to) \
  NS_IMETHOD GetBrowserDOMWindow(nsIBrowserDOMWindow **aBrowserDOMWindow) override { return _to GetBrowserDOMWindow(aBrowserDOMWindow); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOMCHROMEWINDOW(_to) \
  NS_IMETHOD GetBrowserDOMWindow(nsIBrowserDOMWindow **aBrowserDOMWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBrowserDOMWindow(aBrowserDOMWindow); } 


#endif /* __gen_nsIDOMChromeWindow_h__ */
