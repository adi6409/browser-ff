/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIDHCPClient.idl
 */

#ifndef __gen_nsIDHCPClient_h__
#define __gen_nsIDHCPClient_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIDHCPClient */
#define NS_IDHCPCLIENT_IID_STR "aee75dc0-be1a-46b9-9e0c-31a6899c175c"

#define NS_IDHCPCLIENT_IID \
  {0xaee75dc0, 0xbe1a, 0x46b9, \
    { 0x9e, 0x0c, 0x31, 0xa6, 0x89, 0x9c, 0x17, 0x5c }}

class NS_NO_VTABLE nsIDHCPClient : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDHCPCLIENT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDHCPClient;

  /* ACString getOption (in uint8_t option); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetOption(uint8_t option, nsACString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDHCPClient, NS_IDHCPCLIENT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDHCPCLIENT \
  NS_IMETHOD GetOption(uint8_t option, nsACString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDHCPCLIENT \
  nsresult GetOption(uint8_t option, nsACString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDHCPCLIENT(_to) \
  NS_IMETHOD GetOption(uint8_t option, nsACString& _retval) override { return _to GetOption(option, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDHCPCLIENT(_to) \
  NS_IMETHOD GetOption(uint8_t option, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOption(option, _retval); } 


#endif /* __gen_nsIDHCPClient_h__ */
