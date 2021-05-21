/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/geolocation/nsIDOMGeoPosition.idl
 */

#ifndef __gen_nsIDOMGeoPosition_h__
#define __gen_nsIDOMGeoPosition_h__


#ifndef __gen_domstubs_h__
#include "domstubs.h"
#endif

#ifndef __gen_nsIDOMGeoPositionCoords_h__
#include "nsIDOMGeoPositionCoords.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIDOMGeoPosition */
#define NS_IDOMGEOPOSITION_IID_STR "dd9f7e81-0f74-4fb5-b361-37019bf60c3f"

#define NS_IDOMGEOPOSITION_IID \
  {0xdd9f7e81, 0x0f74, 0x4fb5, \
    { 0xb3, 0x61, 0x37, 0x01, 0x9b, 0xf6, 0x0c, 0x3f }}

class NS_NO_VTABLE nsIDOMGeoPosition : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOMGEOPOSITION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDOMGeoPosition;

  /* readonly attribute DOMTimeStamp timestamp; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTimestamp(DOMTimeStamp *aTimestamp) = 0;

  /* readonly attribute nsIDOMGeoPositionCoords coords; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCoords(nsIDOMGeoPositionCoords **aCoords) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDOMGeoPosition, NS_IDOMGEOPOSITION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOMGEOPOSITION \
  NS_IMETHOD GetTimestamp(DOMTimeStamp *aTimestamp) override; \
  NS_IMETHOD GetCoords(nsIDOMGeoPositionCoords **aCoords) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOMGEOPOSITION \
  nsresult GetTimestamp(DOMTimeStamp *aTimestamp); \
  nsresult GetCoords(nsIDOMGeoPositionCoords **aCoords); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOMGEOPOSITION(_to) \
  NS_IMETHOD GetTimestamp(DOMTimeStamp *aTimestamp) override { return _to GetTimestamp(aTimestamp); } \
  NS_IMETHOD GetCoords(nsIDOMGeoPositionCoords **aCoords) override { return _to GetCoords(aCoords); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOMGEOPOSITION(_to) \
  NS_IMETHOD GetTimestamp(DOMTimeStamp *aTimestamp) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTimestamp(aTimestamp); } \
  NS_IMETHOD GetCoords(nsIDOMGeoPositionCoords **aCoords) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCoords(aCoords); } 


#endif /* __gen_nsIDOMGeoPosition_h__ */
