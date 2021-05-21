/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/windowcreator/nsIWindowCreator.idl
 */

#ifndef __gen_nsIWindowCreator_h__
#define __gen_nsIWindowCreator_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIOpenWindowInfo; /* forward declaration */

class nsIRemoteTab; /* forward declaration */

class nsIURI; /* forward declaration */

class nsIWebBrowserChrome; /* forward declaration */

class mozIDOMWindowProxy; /* forward declaration */


/* starting interface:    nsIWindowCreator */
#define NS_IWINDOWCREATOR_IID_STR "30465632-a777-44cc-90f9-8145475ef999"

#define NS_IWINDOWCREATOR_IID \
  {0x30465632, 0xa777, 0x44cc, \
    { 0x90, 0xf9, 0x81, 0x45, 0x47, 0x5e, 0xf9, 0x99 }}

class NS_NO_VTABLE nsIWindowCreator : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWINDOWCREATOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWindowCreator;

  /* nsIWebBrowserChrome createChromeWindow (in nsIWebBrowserChrome parent, in uint32_t chromeFlags, in nsIOpenWindowInfo aOpenWindowInfo, out boolean cancel); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CreateChromeWindow(nsIWebBrowserChrome *parent, uint32_t chromeFlags, nsIOpenWindowInfo *aOpenWindowInfo, bool *cancel, nsIWebBrowserChrome **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWindowCreator, NS_IWINDOWCREATOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWINDOWCREATOR \
  NS_IMETHOD CreateChromeWindow(nsIWebBrowserChrome *parent, uint32_t chromeFlags, nsIOpenWindowInfo *aOpenWindowInfo, bool *cancel, nsIWebBrowserChrome **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWINDOWCREATOR \
  nsresult CreateChromeWindow(nsIWebBrowserChrome *parent, uint32_t chromeFlags, nsIOpenWindowInfo *aOpenWindowInfo, bool *cancel, nsIWebBrowserChrome **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWINDOWCREATOR(_to) \
  NS_IMETHOD CreateChromeWindow(nsIWebBrowserChrome *parent, uint32_t chromeFlags, nsIOpenWindowInfo *aOpenWindowInfo, bool *cancel, nsIWebBrowserChrome **_retval) override { return _to CreateChromeWindow(parent, chromeFlags, aOpenWindowInfo, cancel, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWINDOWCREATOR(_to) \
  NS_IMETHOD CreateChromeWindow(nsIWebBrowserChrome *parent, uint32_t chromeFlags, nsIOpenWindowInfo *aOpenWindowInfo, bool *cancel, nsIWebBrowserChrome **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateChromeWindow(parent, chromeFlags, aOpenWindowInfo, cancel, _retval); } 

// {30465632-A777-44cc-90F9-8145475EF999}
#define NS_WINDOWCREATOR_IID \
 {0x30465632, 0xa777, 0x44cc, {0x90, 0xf9, 0x81, 0x45, 0x47, 0x5e, 0xf9, 0x99}}

#endif /* __gen_nsIWindowCreator_h__ */
