/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/geolocation/nsIDOMGeoPositionErrorCallback.idl
 */

#ifndef __gen_nsIDOMGeoPositionErrorCallback_h__
#define __gen_nsIDOMGeoPositionErrorCallback_h__


#ifndef __gen_domstubs_h__
#include "domstubs.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
namespace dom {
class GeolocationPositionError; /* webidl GeolocationPositionError */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIDOMGeoPositionErrorCallback */
#define NS_IDOMGEOPOSITIONERRORCALLBACK_IID_STR "7d9b09d9-4843-43eb-a7a7-67f7dda6b3c4"

#define NS_IDOMGEOPOSITIONERRORCALLBACK_IID \
  {0x7d9b09d9, 0x4843, 0x43eb, \
    { 0xa7, 0xa7, 0x67, 0xf7, 0xdd, 0xa6, 0xb3, 0xc4 }}

class NS_NO_VTABLE nsIDOMGeoPositionErrorCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOMGEOPOSITIONERRORCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDOMGeoPositionErrorCallback;

  /* void handleEvent (in GeolocationPositionError positionError); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleEvent(mozilla::dom::GeolocationPositionError *positionError) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDOMGeoPositionErrorCallback, NS_IDOMGEOPOSITIONERRORCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOMGEOPOSITIONERRORCALLBACK \
  NS_IMETHOD HandleEvent(mozilla::dom::GeolocationPositionError *positionError) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOMGEOPOSITIONERRORCALLBACK \
  nsresult HandleEvent(mozilla::dom::GeolocationPositionError *positionError); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOMGEOPOSITIONERRORCALLBACK(_to) \
  NS_IMETHOD HandleEvent(mozilla::dom::GeolocationPositionError *positionError) override { return _to HandleEvent(positionError); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOMGEOPOSITIONERRORCALLBACK(_to) \
  NS_IMETHOD HandleEvent(mozilla::dom::GeolocationPositionError *positionError) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleEvent(positionError); } 


#endif /* __gen_nsIDOMGeoPositionErrorCallback_h__ */
