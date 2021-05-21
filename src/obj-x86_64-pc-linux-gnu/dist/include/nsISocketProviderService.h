/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/socket/nsISocketProviderService.idl
 */

#ifndef __gen_nsISocketProviderService_h__
#define __gen_nsISocketProviderService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsISocketProvider; /* forward declaration */


/* starting interface:    nsISocketProviderService */
#define NS_ISOCKETPROVIDERSERVICE_IID_STR "8f8a23d0-5472-11d3-bbc8-0000861d1237"

#define NS_ISOCKETPROVIDERSERVICE_IID \
  {0x8f8a23d0, 0x5472, 0x11d3, \
    { 0xbb, 0xc8, 0x00, 0x00, 0x86, 0x1d, 0x12, 0x37 }}

class NS_NO_VTABLE nsISocketProviderService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISOCKETPROVIDERSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISocketProviderService;

  /* nsISocketProvider getSocketProvider (in string socketType); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSocketProvider(const char * socketType, nsISocketProvider **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISocketProviderService, NS_ISOCKETPROVIDERSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISOCKETPROVIDERSERVICE \
  NS_IMETHOD GetSocketProvider(const char * socketType, nsISocketProvider **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISOCKETPROVIDERSERVICE \
  nsresult GetSocketProvider(const char * socketType, nsISocketProvider **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISOCKETPROVIDERSERVICE(_to) \
  NS_IMETHOD GetSocketProvider(const char * socketType, nsISocketProvider **_retval) override { return _to GetSocketProvider(socketType, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISOCKETPROVIDERSERVICE(_to) \
  NS_IMETHOD GetSocketProvider(const char * socketType, nsISocketProvider **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSocketProvider(socketType, _retval); } 


#endif /* __gen_nsISocketProviderService_h__ */
