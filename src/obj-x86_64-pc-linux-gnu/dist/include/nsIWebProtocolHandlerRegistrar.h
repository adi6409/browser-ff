/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/sidebar/nsIWebProtocolHandlerRegistrar.idl
 */

#ifndef __gen_nsIWebProtocolHandlerRegistrar_h__
#define __gen_nsIWebProtocolHandlerRegistrar_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */


/* starting interface:    nsIWebProtocolHandlerRegistrar */
#define NS_IWEBPROTOCOLHANDLERREGISTRAR_IID_STR "1ce9ef8d-f462-49ca-b8e9-c946c4f37d6e"

#define NS_IWEBPROTOCOLHANDLERREGISTRAR_IID \
  {0x1ce9ef8d, 0xf462, 0x49ca, \
    { 0xb8, 0xe9, 0xc9, 0x46, 0xc4, 0xf3, 0x7d, 0x6e }}

class NS_NO_VTABLE nsIWebProtocolHandlerRegistrar : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWEBPROTOCOLHANDLERREGISTRAR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWebProtocolHandlerRegistrar;

  /* void registerProtocolHandler (in AString protocol, in nsIURI uri, in AString title, in nsIURI documentURI, in nsISupports windowOrBrowser); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RegisterProtocolHandler(const nsAString& protocol, nsIURI *uri, const nsAString& title, nsIURI *documentURI, nsISupports *windowOrBrowser) = 0;

  /* void removeProtocolHandler (in AString protocol, in AString uri); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveProtocolHandler(const nsAString& protocol, const nsAString& uri) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWebProtocolHandlerRegistrar, NS_IWEBPROTOCOLHANDLERREGISTRAR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWEBPROTOCOLHANDLERREGISTRAR \
  NS_IMETHOD RegisterProtocolHandler(const nsAString& protocol, nsIURI *uri, const nsAString& title, nsIURI *documentURI, nsISupports *windowOrBrowser) override; \
  NS_IMETHOD RemoveProtocolHandler(const nsAString& protocol, const nsAString& uri) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWEBPROTOCOLHANDLERREGISTRAR \
  nsresult RegisterProtocolHandler(const nsAString& protocol, nsIURI *uri, const nsAString& title, nsIURI *documentURI, nsISupports *windowOrBrowser); \
  nsresult RemoveProtocolHandler(const nsAString& protocol, const nsAString& uri); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWEBPROTOCOLHANDLERREGISTRAR(_to) \
  NS_IMETHOD RegisterProtocolHandler(const nsAString& protocol, nsIURI *uri, const nsAString& title, nsIURI *documentURI, nsISupports *windowOrBrowser) override { return _to RegisterProtocolHandler(protocol, uri, title, documentURI, windowOrBrowser); } \
  NS_IMETHOD RemoveProtocolHandler(const nsAString& protocol, const nsAString& uri) override { return _to RemoveProtocolHandler(protocol, uri); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWEBPROTOCOLHANDLERREGISTRAR(_to) \
  NS_IMETHOD RegisterProtocolHandler(const nsAString& protocol, nsIURI *uri, const nsAString& title, nsIURI *documentURI, nsISupports *windowOrBrowser) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterProtocolHandler(protocol, uri, title, documentURI, windowOrBrowser); } \
  NS_IMETHOD RemoveProtocolHandler(const nsAString& protocol, const nsAString& uri) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveProtocolHandler(protocol, uri); } 


#define NS_WEBPROTOCOLHANDLERREGISTRAR_CONTRACTID "@mozilla.org/embeddor.implemented/web-protocol-handler-registrar;1"

#endif /* __gen_nsIWebProtocolHandlerRegistrar_h__ */
