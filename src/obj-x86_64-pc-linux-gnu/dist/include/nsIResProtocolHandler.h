/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/res/nsIResProtocolHandler.idl
 */

#ifndef __gen_nsIResProtocolHandler_h__
#define __gen_nsIResProtocolHandler_h__


#ifndef __gen_nsISubstitutingProtocolHandler_h__
#include "nsISubstitutingProtocolHandler.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIResProtocolHandler */
#define NS_IRESPROTOCOLHANDLER_IID_STR "241d34ac-9ed5-46d7-910c-7a9d914aa0c5"

#define NS_IRESPROTOCOLHANDLER_IID \
  {0x241d34ac, 0x9ed5, 0x46d7, \
    { 0x91, 0x0c, 0x7a, 0x9d, 0x91, 0x4a, 0xa0, 0xc5 }}

class NS_NO_VTABLE nsIResProtocolHandler : public nsISubstitutingProtocolHandler {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IRESPROTOCOLHANDLER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIResProtocolHandler;

  /* boolean allowContentToAccess (in nsIURI url); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AllowContentToAccess(nsIURI *url, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIResProtocolHandler, NS_IRESPROTOCOLHANDLER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIRESPROTOCOLHANDLER \
  NS_IMETHOD AllowContentToAccess(nsIURI *url, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIRESPROTOCOLHANDLER \
  nsresult AllowContentToAccess(nsIURI *url, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIRESPROTOCOLHANDLER(_to) \
  NS_IMETHOD AllowContentToAccess(nsIURI *url, bool *_retval) override { return _to AllowContentToAccess(url, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIRESPROTOCOLHANDLER(_to) \
  NS_IMETHOD AllowContentToAccess(nsIURI *url, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AllowContentToAccess(url, _retval); } 


#endif /* __gen_nsIResProtocolHandler_h__ */
