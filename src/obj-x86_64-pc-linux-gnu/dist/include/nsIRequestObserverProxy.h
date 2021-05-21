/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIRequestObserverProxy.idl
 */

#ifndef __gen_nsIRequestObserverProxy_h__
#define __gen_nsIRequestObserverProxy_h__


#ifndef __gen_nsIRequestObserver_h__
#include "nsIRequestObserver.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIEventTarget; /* forward declaration */


/* starting interface:    nsIRequestObserverProxy */
#define NS_IREQUESTOBSERVERPROXY_IID_STR "c2b06151-1bf8-4eef-aea9-1532f12f5a10"

#define NS_IREQUESTOBSERVERPROXY_IID \
  {0xc2b06151, 0x1bf8, 0x4eef, \
    { 0xae, 0xa9, 0x15, 0x32, 0xf1, 0x2f, 0x5a, 0x10 }}

class NS_NO_VTABLE nsIRequestObserverProxy : public nsIRequestObserver {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IREQUESTOBSERVERPROXY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIRequestObserverProxy;

  /* void init (in nsIRequestObserver observer, in nsISupports context); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(nsIRequestObserver *observer, nsISupports *context) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIRequestObserverProxy, NS_IREQUESTOBSERVERPROXY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIREQUESTOBSERVERPROXY \
  NS_IMETHOD Init(nsIRequestObserver *observer, nsISupports *context) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIREQUESTOBSERVERPROXY \
  nsresult Init(nsIRequestObserver *observer, nsISupports *context); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIREQUESTOBSERVERPROXY(_to) \
  NS_IMETHOD Init(nsIRequestObserver *observer, nsISupports *context) override { return _to Init(observer, context); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIREQUESTOBSERVERPROXY(_to) \
  NS_IMETHOD Init(nsIRequestObserver *observer, nsISupports *context) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(observer, context); } 


#endif /* __gen_nsIRequestObserverProxy_h__ */
