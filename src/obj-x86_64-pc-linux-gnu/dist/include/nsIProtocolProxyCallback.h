/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIProtocolProxyCallback.idl
 */

#ifndef __gen_nsIProtocolProxyCallback_h__
#define __gen_nsIProtocolProxyCallback_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIChannel; /* forward declaration */

class nsIProxyInfo; /* forward declaration */

class nsICancelable; /* forward declaration */


/* starting interface:    nsIProtocolProxyCallback */
#define NS_IPROTOCOLPROXYCALLBACK_IID_STR "fbb6eff6-0cc2-4d99-8d6f-0a12b462bdeb"

#define NS_IPROTOCOLPROXYCALLBACK_IID \
  {0xfbb6eff6, 0x0cc2, 0x4d99, \
    { 0x8d, 0x6f, 0x0a, 0x12, 0xb4, 0x62, 0xbd, 0xeb }}

class NS_NO_VTABLE nsIProtocolProxyCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPROTOCOLPROXYCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIProtocolProxyCallback;

  /* void onProxyAvailable (in nsICancelable aRequest, in nsIChannel aChannel, in nsIProxyInfo aProxyInfo, in nsresult aStatus); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnProxyAvailable(nsICancelable *aRequest, nsIChannel *aChannel, nsIProxyInfo *aProxyInfo, nsresult aStatus) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIProtocolProxyCallback, NS_IPROTOCOLPROXYCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPROTOCOLPROXYCALLBACK \
  NS_IMETHOD OnProxyAvailable(nsICancelable *aRequest, nsIChannel *aChannel, nsIProxyInfo *aProxyInfo, nsresult aStatus) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPROTOCOLPROXYCALLBACK \
  nsresult OnProxyAvailable(nsICancelable *aRequest, nsIChannel *aChannel, nsIProxyInfo *aProxyInfo, nsresult aStatus); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPROTOCOLPROXYCALLBACK(_to) \
  NS_IMETHOD OnProxyAvailable(nsICancelable *aRequest, nsIChannel *aChannel, nsIProxyInfo *aProxyInfo, nsresult aStatus) override { return _to OnProxyAvailable(aRequest, aChannel, aProxyInfo, aStatus); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPROTOCOLPROXYCALLBACK(_to) \
  NS_IMETHOD OnProxyAvailable(nsICancelable *aRequest, nsIChannel *aChannel, nsIProxyInfo *aProxyInfo, nsresult aStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnProxyAvailable(aRequest, aChannel, aProxyInfo, aStatus); } 


#endif /* __gen_nsIProtocolProxyCallback_h__ */
