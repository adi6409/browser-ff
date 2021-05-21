/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIExternalProtocolHandler.idl
 */

#ifndef __gen_nsIExternalProtocolHandler_h__
#define __gen_nsIExternalProtocolHandler_h__


#ifndef __gen_nsIProtocolHandler_h__
#include "nsIProtocolHandler.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIExternalProtocolHandler */
#define NS_IEXTERNALPROTOCOLHANDLER_IID_STR "0e61f3b2-34d7-4c79-bfdc-4860bc7341b7"

#define NS_IEXTERNALPROTOCOLHANDLER_IID \
  {0x0e61f3b2, 0x34d7, 0x4c79, \
    { 0xbf, 0xdc, 0x48, 0x60, 0xbc, 0x73, 0x41, 0xb7 }}

class NS_NO_VTABLE nsIExternalProtocolHandler : public nsIProtocolHandler {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IEXTERNALPROTOCOLHANDLER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIExternalProtocolHandler;

  /* boolean externalAppExistsForScheme (in ACString scheme); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ExternalAppExistsForScheme(const nsACString& scheme, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIExternalProtocolHandler, NS_IEXTERNALPROTOCOLHANDLER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIEXTERNALPROTOCOLHANDLER \
  NS_IMETHOD ExternalAppExistsForScheme(const nsACString& scheme, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIEXTERNALPROTOCOLHANDLER \
  nsresult ExternalAppExistsForScheme(const nsACString& scheme, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIEXTERNALPROTOCOLHANDLER(_to) \
  NS_IMETHOD ExternalAppExistsForScheme(const nsACString& scheme, bool *_retval) override { return _to ExternalAppExistsForScheme(scheme, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIEXTERNALPROTOCOLHANDLER(_to) \
  NS_IMETHOD ExternalAppExistsForScheme(const nsACString& scheme, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ExternalAppExistsForScheme(scheme, _retval); } 


#endif /* __gen_nsIExternalProtocolHandler_h__ */
