/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/geolocation/nsIDOMGeoPositionCallback.idl
 */

#ifndef __gen_nsIDOMGeoPositionCallback_h__
#define __gen_nsIDOMGeoPositionCallback_h__


#ifndef __gen_domstubs_h__
#include "domstubs.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIDOMGeoPosition; /* forward declaration */


/* starting interface:    nsIDOMGeoPositionCallback */
#define NS_IDOMGEOPOSITIONCALLBACK_IID_STR "527e8b53-6f29-4b6a-8d04-5c1666a4c4c1"

#define NS_IDOMGEOPOSITIONCALLBACK_IID \
  {0x527e8b53, 0x6f29, 0x4b6a, \
    { 0x8d, 0x04, 0x5c, 0x16, 0x66, 0xa4, 0xc4, 0xc1 }}

class NS_NO_VTABLE nsIDOMGeoPositionCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOMGEOPOSITIONCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDOMGeoPositionCallback;

  /* void handleEvent (in nsIDOMGeoPosition position); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleEvent(nsIDOMGeoPosition *position) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDOMGeoPositionCallback, NS_IDOMGEOPOSITIONCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOMGEOPOSITIONCALLBACK \
  NS_IMETHOD HandleEvent(nsIDOMGeoPosition *position) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOMGEOPOSITIONCALLBACK \
  nsresult HandleEvent(nsIDOMGeoPosition *position); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOMGEOPOSITIONCALLBACK(_to) \
  NS_IMETHOD HandleEvent(nsIDOMGeoPosition *position) override { return _to HandleEvent(position); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOMGEOPOSITIONCALLBACK(_to) \
  NS_IMETHOD HandleEvent(nsIDOMGeoPosition *position) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleEvent(position); } 


#endif /* __gen_nsIDOMGeoPositionCallback_h__ */
