/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/system/nsIDeviceSensors.idl
 */

#ifndef __gen_nsIDeviceSensors_h__
#define __gen_nsIDeviceSensors_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIDOMWindow; /* forward declaration */


/* starting interface:    nsIDeviceSensorData */
#define NS_IDEVICESENSORDATA_IID_STR "0462247e-fe8c-4aa5-b675-3752547e485f"

#define NS_IDEVICESENSORDATA_IID \
  {0x0462247e, 0xfe8c, 0x4aa5, \
    { 0xb6, 0x75, 0x37, 0x52, 0x54, 0x7e, 0x48, 0x5f }}

class NS_NO_VTABLE nsIDeviceSensorData : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDEVICESENSORDATA_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDeviceSensorData;

  enum {
    TYPE_ORIENTATION = 0U,
    TYPE_ACCELERATION = 1U,
    TYPE_PROXIMITY = 2U,
    TYPE_LINEAR_ACCELERATION = 3U,
    TYPE_GYROSCOPE = 4U,
    TYPE_LIGHT = 5U,
    TYPE_ROTATION_VECTOR = 6U,
    TYPE_GAME_ROTATION_VECTOR = 7U
  };

  /* readonly attribute unsigned long type; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetType(uint32_t *aType) = 0;

  /* readonly attribute double x; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetX(double *aX) = 0;

  /* readonly attribute double y; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetY(double *aY) = 0;

  /* readonly attribute double z; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetZ(double *aZ) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDeviceSensorData, NS_IDEVICESENSORDATA_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDEVICESENSORDATA \
  NS_IMETHOD GetType(uint32_t *aType) override; \
  NS_IMETHOD GetX(double *aX) override; \
  NS_IMETHOD GetY(double *aY) override; \
  NS_IMETHOD GetZ(double *aZ) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDEVICESENSORDATA \
  nsresult GetType(uint32_t *aType); \
  nsresult GetX(double *aX); \
  nsresult GetY(double *aY); \
  nsresult GetZ(double *aZ); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDEVICESENSORDATA(_to) \
  NS_IMETHOD GetType(uint32_t *aType) override { return _to GetType(aType); } \
  NS_IMETHOD GetX(double *aX) override { return _to GetX(aX); } \
  NS_IMETHOD GetY(double *aY) override { return _to GetY(aY); } \
  NS_IMETHOD GetZ(double *aZ) override { return _to GetZ(aZ); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDEVICESENSORDATA(_to) \
  NS_IMETHOD GetType(uint32_t *aType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetType(aType); } \
  NS_IMETHOD GetX(double *aX) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetX(aX); } \
  NS_IMETHOD GetY(double *aY) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetY(aY); } \
  NS_IMETHOD GetZ(double *aZ) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetZ(aZ); } 


/* starting interface:    nsIDeviceSensors */
#define NS_IDEVICESENSORS_IID_STR "e46e47c7-55ff-44c4-abce-21b14ba07f86"

#define NS_IDEVICESENSORS_IID \
  {0xe46e47c7, 0x55ff, 0x44c4, \
    { 0xab, 0xce, 0x21, 0xb1, 0x4b, 0xa0, 0x7f, 0x86 }}

class NS_NO_VTABLE nsIDeviceSensors : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDEVICESENSORS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDeviceSensors;

  /* bool hasWindowListener (in unsigned long aType, in nsIDOMWindow aWindow); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HasWindowListener(uint32_t aType, nsIDOMWindow *aWindow, bool *_retval) = 0;

  /* [noscript] void addWindowListener (in unsigned long aType, in nsIDOMWindow aWindow); */
  NS_IMETHOD AddWindowListener(uint32_t aType, nsIDOMWindow *aWindow) = 0;

  /* [noscript] void removeWindowListener (in unsigned long aType, in nsIDOMWindow aWindow); */
  NS_IMETHOD RemoveWindowListener(uint32_t aType, nsIDOMWindow *aWindow) = 0;

  /* [noscript] void removeWindowAsListener (in nsIDOMWindow aWindow); */
  NS_IMETHOD RemoveWindowAsListener(nsIDOMWindow *aWindow) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDeviceSensors, NS_IDEVICESENSORS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDEVICESENSORS \
  NS_IMETHOD HasWindowListener(uint32_t aType, nsIDOMWindow *aWindow, bool *_retval) override; \
  NS_IMETHOD AddWindowListener(uint32_t aType, nsIDOMWindow *aWindow) override; \
  NS_IMETHOD RemoveWindowListener(uint32_t aType, nsIDOMWindow *aWindow) override; \
  NS_IMETHOD RemoveWindowAsListener(nsIDOMWindow *aWindow) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDEVICESENSORS \
  nsresult HasWindowListener(uint32_t aType, nsIDOMWindow *aWindow, bool *_retval); \
  nsresult AddWindowListener(uint32_t aType, nsIDOMWindow *aWindow); \
  nsresult RemoveWindowListener(uint32_t aType, nsIDOMWindow *aWindow); \
  nsresult RemoveWindowAsListener(nsIDOMWindow *aWindow); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDEVICESENSORS(_to) \
  NS_IMETHOD HasWindowListener(uint32_t aType, nsIDOMWindow *aWindow, bool *_retval) override { return _to HasWindowListener(aType, aWindow, _retval); } \
  NS_IMETHOD AddWindowListener(uint32_t aType, nsIDOMWindow *aWindow) override { return _to AddWindowListener(aType, aWindow); } \
  NS_IMETHOD RemoveWindowListener(uint32_t aType, nsIDOMWindow *aWindow) override { return _to RemoveWindowListener(aType, aWindow); } \
  NS_IMETHOD RemoveWindowAsListener(nsIDOMWindow *aWindow) override { return _to RemoveWindowAsListener(aWindow); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDEVICESENSORS(_to) \
  NS_IMETHOD HasWindowListener(uint32_t aType, nsIDOMWindow *aWindow, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasWindowListener(aType, aWindow, _retval); } \
  NS_IMETHOD AddWindowListener(uint32_t aType, nsIDOMWindow *aWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddWindowListener(aType, aWindow); } \
  NS_IMETHOD RemoveWindowListener(uint32_t aType, nsIDOMWindow *aWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveWindowListener(aType, aWindow); } \
  NS_IMETHOD RemoveWindowAsListener(nsIDOMWindow *aWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveWindowAsListener(aWindow); } 


#define NS_DEVICE_SENSORS_CID \
{ 0xecba5203, 0x77da, 0x465a, \
{ 0x86, 0x5e, 0x78, 0xb7, 0xaf, 0x10, 0xd8, 0xf7 } }
#define NS_DEVICE_SENSORS_CONTRACTID "@mozilla.org/devicesensors;1"

#endif /* __gen_nsIDeviceSensors_h__ */
