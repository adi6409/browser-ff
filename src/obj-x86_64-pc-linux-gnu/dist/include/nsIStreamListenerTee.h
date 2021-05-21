/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIStreamListenerTee.idl
 */

#ifndef __gen_nsIStreamListenerTee_h__
#define __gen_nsIStreamListenerTee_h__


#ifndef __gen_nsIStreamListener_h__
#include "nsIStreamListener.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIOutputStream; /* forward declaration */

class nsIRequestObserver; /* forward declaration */

class nsIEventTarget; /* forward declaration */


/* starting interface:    nsIStreamListenerTee */
#define NS_ISTREAMLISTENERTEE_IID_STR "62b27fc1-6e8c-4225-8ad0-b9d44252973a"

#define NS_ISTREAMLISTENERTEE_IID \
  {0x62b27fc1, 0x6e8c, 0x4225, \
    { 0x8a, 0xd0, 0xb9, 0xd4, 0x42, 0x52, 0x97, 0x3a }}

class NS_NO_VTABLE nsIStreamListenerTee : public nsIStreamListener {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISTREAMLISTENERTEE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIStreamListenerTee;

  /* void init (in nsIStreamListener listener, in nsIOutputStream sink, [optional] in nsIRequestObserver requestObserver); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(nsIStreamListener *listener, nsIOutputStream *sink, nsIRequestObserver *requestObserver) = 0;

  /* void initAsync (in nsIStreamListener listener, in nsIEventTarget eventTarget, in nsIOutputStream sink, [optional] in nsIRequestObserver requestObserver); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD InitAsync(nsIStreamListener *listener, nsIEventTarget *eventTarget, nsIOutputStream *sink, nsIRequestObserver *requestObserver) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIStreamListenerTee, NS_ISTREAMLISTENERTEE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISTREAMLISTENERTEE \
  NS_IMETHOD Init(nsIStreamListener *listener, nsIOutputStream *sink, nsIRequestObserver *requestObserver) override; \
  NS_IMETHOD InitAsync(nsIStreamListener *listener, nsIEventTarget *eventTarget, nsIOutputStream *sink, nsIRequestObserver *requestObserver) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISTREAMLISTENERTEE \
  nsresult Init(nsIStreamListener *listener, nsIOutputStream *sink, nsIRequestObserver *requestObserver); \
  nsresult InitAsync(nsIStreamListener *listener, nsIEventTarget *eventTarget, nsIOutputStream *sink, nsIRequestObserver *requestObserver); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISTREAMLISTENERTEE(_to) \
  NS_IMETHOD Init(nsIStreamListener *listener, nsIOutputStream *sink, nsIRequestObserver *requestObserver) override { return _to Init(listener, sink, requestObserver); } \
  NS_IMETHOD InitAsync(nsIStreamListener *listener, nsIEventTarget *eventTarget, nsIOutputStream *sink, nsIRequestObserver *requestObserver) override { return _to InitAsync(listener, eventTarget, sink, requestObserver); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISTREAMLISTENERTEE(_to) \
  NS_IMETHOD Init(nsIStreamListener *listener, nsIOutputStream *sink, nsIRequestObserver *requestObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(listener, sink, requestObserver); } \
  NS_IMETHOD InitAsync(nsIStreamListener *listener, nsIEventTarget *eventTarget, nsIOutputStream *sink, nsIRequestObserver *requestObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitAsync(listener, eventTarget, sink, requestObserver); } 


#endif /* __gen_nsIStreamListenerTee_h__ */
