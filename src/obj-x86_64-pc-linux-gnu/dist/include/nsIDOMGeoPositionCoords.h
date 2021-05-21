/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/geolocation/nsIDOMGeoPositionCoords.idl
 */

#ifndef __gen_nsIDOMGeoPositionCoords_h__
#define __gen_nsIDOMGeoPositionCoords_h__


#ifndef __gen_domstubs_h__
#include "domstubs.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIDOMGeoPositionCoords */
#define NS_IDOMGEOPOSITIONCOORDS_IID_STR "b31702d0-6dac-4fa0-b93b-f043e71c8f9a"

#define NS_IDOMGEOPOSITIONCOORDS_IID \
  {0xb31702d0, 0x6dac, 0x4fa0, \
    { 0xb9, 0x3b, 0xf0, 0x43, 0xe7, 0x1c, 0x8f, 0x9a }}

class NS_NO_VTABLE nsIDOMGeoPositionCoords : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOMGEOPOSITIONCOORDS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDOMGeoPositionCoords;

  /* readonly attribute double latitude; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLatitude(double *aLatitude) = 0;

  /* readonly attribute double longitude; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLongitude(double *aLongitude) = 0;

  /* readonly attribute double altitude; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAltitude(double *aAltitude) = 0;

  /* readonly attribute double accuracy; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAccuracy(double *aAccuracy) = 0;

  /* readonly attribute double altitudeAccuracy; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAltitudeAccuracy(double *aAltitudeAccuracy) = 0;

  /* readonly attribute double heading; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHeading(double *aHeading) = 0;

  /* readonly attribute double speed; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSpeed(double *aSpeed) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDOMGeoPositionCoords, NS_IDOMGEOPOSITIONCOORDS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOMGEOPOSITIONCOORDS \
  NS_IMETHOD GetLatitude(double *aLatitude) override; \
  NS_IMETHOD GetLongitude(double *aLongitude) override; \
  NS_IMETHOD GetAltitude(double *aAltitude) override; \
  NS_IMETHOD GetAccuracy(double *aAccuracy) override; \
  NS_IMETHOD GetAltitudeAccuracy(double *aAltitudeAccuracy) override; \
  NS_IMETHOD GetHeading(double *aHeading) override; \
  NS_IMETHOD GetSpeed(double *aSpeed) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOMGEOPOSITIONCOORDS \
  nsresult GetLatitude(double *aLatitude); \
  nsresult GetLongitude(double *aLongitude); \
  nsresult GetAltitude(double *aAltitude); \
  nsresult GetAccuracy(double *aAccuracy); \
  nsresult GetAltitudeAccuracy(double *aAltitudeAccuracy); \
  nsresult GetHeading(double *aHeading); \
  nsresult GetSpeed(double *aSpeed); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOMGEOPOSITIONCOORDS(_to) \
  NS_IMETHOD GetLatitude(double *aLatitude) override { return _to GetLatitude(aLatitude); } \
  NS_IMETHOD GetLongitude(double *aLongitude) override { return _to GetLongitude(aLongitude); } \
  NS_IMETHOD GetAltitude(double *aAltitude) override { return _to GetAltitude(aAltitude); } \
  NS_IMETHOD GetAccuracy(double *aAccuracy) override { return _to GetAccuracy(aAccuracy); } \
  NS_IMETHOD GetAltitudeAccuracy(double *aAltitudeAccuracy) override { return _to GetAltitudeAccuracy(aAltitudeAccuracy); } \
  NS_IMETHOD GetHeading(double *aHeading) override { return _to GetHeading(aHeading); } \
  NS_IMETHOD GetSpeed(double *aSpeed) override { return _to GetSpeed(aSpeed); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOMGEOPOSITIONCOORDS(_to) \
  NS_IMETHOD GetLatitude(double *aLatitude) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLatitude(aLatitude); } \
  NS_IMETHOD GetLongitude(double *aLongitude) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLongitude(aLongitude); } \
  NS_IMETHOD GetAltitude(double *aAltitude) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAltitude(aAltitude); } \
  NS_IMETHOD GetAccuracy(double *aAccuracy) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAccuracy(aAccuracy); } \
  NS_IMETHOD GetAltitudeAccuracy(double *aAltitudeAccuracy) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAltitudeAccuracy(aAltitudeAccuracy); } \
  NS_IMETHOD GetHeading(double *aHeading) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHeading(aHeading); } \
  NS_IMETHOD GetSpeed(double *aSpeed) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSpeed(aSpeed); } 


#endif /* __gen_nsIDOMGeoPositionCoords_h__ */
