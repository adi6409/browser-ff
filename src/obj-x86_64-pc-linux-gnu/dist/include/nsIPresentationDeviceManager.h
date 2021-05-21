/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationDeviceManager.idl
 */

#ifndef __gen_nsIPresentationDeviceManager_h__
#define __gen_nsIPresentationDeviceManager_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIArray; /* forward declaration */

class nsIPresentationDeviceProvider; /* forward declaration */

#define PRESENTATION_DEVICE_MANAGER_CONTRACTID "@mozilla.org/presentation-device/manager;1"
#define PRESENTATION_DEVICE_CHANGE_TOPIC "presentation-device-change"

/* starting interface:    nsIPresentationDeviceManager */
#define NS_IPRESENTATIONDEVICEMANAGER_IID_STR "beb61db5-3d5f-454f-a15a-dbfa0337c569"

#define NS_IPRESENTATIONDEVICEMANAGER_IID \
  {0xbeb61db5, 0x3d5f, 0x454f, \
    { 0xa1, 0x5a, 0xdb, 0xfa, 0x03, 0x37, 0xc5, 0x69 }}

class NS_NO_VTABLE nsIPresentationDeviceManager : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPRESENTATIONDEVICEMANAGER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPresentationDeviceManager;

  /* readonly attribute boolean deviceAvailable; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDeviceAvailable(bool *aDeviceAvailable) = 0;

  /* void addDeviceProvider (in nsIPresentationDeviceProvider provider); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddDeviceProvider(nsIPresentationDeviceProvider *provider) = 0;

  /* void removeDeviceProvider (in nsIPresentationDeviceProvider provider); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveDeviceProvider(nsIPresentationDeviceProvider *provider) = 0;

  /* void forceDiscovery (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ForceDiscovery(void) = 0;

  /* nsIArray getAvailableDevices ([optional] in nsIArray presentationUrls); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAvailableDevices(nsIArray *presentationUrls, nsIArray **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPresentationDeviceManager, NS_IPRESENTATIONDEVICEMANAGER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPRESENTATIONDEVICEMANAGER \
  NS_IMETHOD GetDeviceAvailable(bool *aDeviceAvailable) override; \
  NS_IMETHOD AddDeviceProvider(nsIPresentationDeviceProvider *provider) override; \
  NS_IMETHOD RemoveDeviceProvider(nsIPresentationDeviceProvider *provider) override; \
  NS_IMETHOD ForceDiscovery(void) override; \
  NS_IMETHOD GetAvailableDevices(nsIArray *presentationUrls, nsIArray **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPRESENTATIONDEVICEMANAGER \
  nsresult GetDeviceAvailable(bool *aDeviceAvailable); \
  nsresult AddDeviceProvider(nsIPresentationDeviceProvider *provider); \
  nsresult RemoveDeviceProvider(nsIPresentationDeviceProvider *provider); \
  nsresult ForceDiscovery(void); \
  nsresult GetAvailableDevices(nsIArray *presentationUrls, nsIArray **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPRESENTATIONDEVICEMANAGER(_to) \
  NS_IMETHOD GetDeviceAvailable(bool *aDeviceAvailable) override { return _to GetDeviceAvailable(aDeviceAvailable); } \
  NS_IMETHOD AddDeviceProvider(nsIPresentationDeviceProvider *provider) override { return _to AddDeviceProvider(provider); } \
  NS_IMETHOD RemoveDeviceProvider(nsIPresentationDeviceProvider *provider) override { return _to RemoveDeviceProvider(provider); } \
  NS_IMETHOD ForceDiscovery(void) override { return _to ForceDiscovery(); } \
  NS_IMETHOD GetAvailableDevices(nsIArray *presentationUrls, nsIArray **_retval) override { return _to GetAvailableDevices(presentationUrls, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPRESENTATIONDEVICEMANAGER(_to) \
  NS_IMETHOD GetDeviceAvailable(bool *aDeviceAvailable) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDeviceAvailable(aDeviceAvailable); } \
  NS_IMETHOD AddDeviceProvider(nsIPresentationDeviceProvider *provider) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddDeviceProvider(provider); } \
  NS_IMETHOD RemoveDeviceProvider(nsIPresentationDeviceProvider *provider) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveDeviceProvider(provider); } \
  NS_IMETHOD ForceDiscovery(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ForceDiscovery(); } \
  NS_IMETHOD GetAvailableDevices(nsIArray *presentationUrls, nsIArray **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAvailableDevices(presentationUrls, _retval); } 


#endif /* __gen_nsIPresentationDeviceManager_h__ */
