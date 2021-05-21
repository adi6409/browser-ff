/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/browser/nsIWebBrowser.idl
 */

#ifndef __gen_nsIWebBrowser_h__
#define __gen_nsIWebBrowser_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIInterfaceRequestor; /* forward declaration */

class nsIWebBrowserChrome; /* forward declaration */

class nsIURIContentListener; /* forward declaration */

class mozIDOMWindowProxy; /* forward declaration */

class nsIWeakReference; /* forward declaration */

namespace mozilla {
class OriginAttributes;
}

/* starting interface:    nsIWebBrowser */
#define NS_IWEBBROWSER_IID_STR "4052b6da-4faa-4646-b3a1-7e16a01c2dc2"

#define NS_IWEBBROWSER_IID \
  {0x4052b6da, 0x4faa, 0x4646, \
    { 0xb3, 0xa1, 0x7e, 0x16, 0xa0, 0x1c, 0x2d, 0xc2 }}

class NS_NO_VTABLE nsIWebBrowser : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWEBBROWSER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWebBrowser;

  /* attribute nsIWebBrowserChrome containerWindow; */
  NS_IMETHOD GetContainerWindow(nsIWebBrowserChrome **aContainerWindow) = 0;
  NS_IMETHOD SetContainerWindow(nsIWebBrowserChrome *aContainerWindow) = 0;

  /* readonly attribute mozIDOMWindowProxy contentDOMWindow; */
  NS_IMETHOD GetContentDOMWindow(mozIDOMWindowProxy **aContentDOMWindow) = 0;

  /* [binaryname(SetOriginAttributes),noscript,nostdcall,notxpcom] void binarySetOriginAttributes (in const_OriginAttributesRef aOriginAttrs); */
  virtual void SetOriginAttributes(const mozilla::OriginAttributes & aOriginAttrs) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWebBrowser, NS_IWEBBROWSER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWEBBROWSER \
  NS_IMETHOD GetContainerWindow(nsIWebBrowserChrome **aContainerWindow) override; \
  NS_IMETHOD SetContainerWindow(nsIWebBrowserChrome *aContainerWindow) override; \
  NS_IMETHOD GetContentDOMWindow(mozIDOMWindowProxy **aContentDOMWindow) override; \
  virtual void SetOriginAttributes(const mozilla::OriginAttributes & aOriginAttrs) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWEBBROWSER \
  nsresult GetContainerWindow(nsIWebBrowserChrome **aContainerWindow); \
  nsresult SetContainerWindow(nsIWebBrowserChrome *aContainerWindow); \
  nsresult GetContentDOMWindow(mozIDOMWindowProxy **aContentDOMWindow); \
  void SetOriginAttributes(const mozilla::OriginAttributes & aOriginAttrs); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWEBBROWSER(_to) \
  NS_IMETHOD GetContainerWindow(nsIWebBrowserChrome **aContainerWindow) override { return _to GetContainerWindow(aContainerWindow); } \
  NS_IMETHOD SetContainerWindow(nsIWebBrowserChrome *aContainerWindow) override { return _to SetContainerWindow(aContainerWindow); } \
  NS_IMETHOD GetContentDOMWindow(mozIDOMWindowProxy **aContentDOMWindow) override { return _to GetContentDOMWindow(aContentDOMWindow); } \
  virtual void SetOriginAttributes(const mozilla::OriginAttributes & aOriginAttrs) override { return _to SetOriginAttributes(aOriginAttrs); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWEBBROWSER(_to) \
  NS_IMETHOD GetContainerWindow(nsIWebBrowserChrome **aContainerWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContainerWindow(aContainerWindow); } \
  NS_IMETHOD SetContainerWindow(nsIWebBrowserChrome *aContainerWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetContainerWindow(aContainerWindow); } \
  NS_IMETHOD GetContentDOMWindow(mozIDOMWindowProxy **aContentDOMWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContentDOMWindow(aContentDOMWindow); } \
  virtual void SetOriginAttributes(const mozilla::OriginAttributes & aOriginAttrs) override; 


#endif /* __gen_nsIWebBrowser_h__ */
