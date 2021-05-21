/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationDeviceProvider.idl
 */

#ifndef __gen_nsIPresentationDeviceProvider_h__
#define __gen_nsIPresentationDeviceProvider_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIPresentationDevice; /* forward declaration */

class nsIPresentationControlChannel; /* forward declaration */

#define PRESENTATION_DEVICE_PROVIDER_CATEGORY "presentation-device-provider"

/* starting interface:    nsIPresentationDeviceListener */
#define NS_IPRESENTATIONDEVICELISTENER_IID_STR "46fd372b-2e40-4179-9b36-0478d141e440"

#define NS_IPRESENTATIONDEVICELISTENER_IID \
  {0x46fd372b, 0x2e40, 0x4179, \
    { 0x9b, 0x36, 0x04, 0x78, 0xd1, 0x41, 0xe4, 0x40 }}

class NS_NO_VTABLE nsIPresentationDeviceListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPRESENTATIONDEVICELISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPresentationDeviceListener;

  /* void addDevice (in nsIPresentationDevice device); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddDevice(nsIPresentationDevice *device) = 0;

  /* void removeDevice (in nsIPresentationDevice device); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveDevice(nsIPresentationDevice *device) = 0;

  /* void updateDevice (in nsIPresentationDevice device); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UpdateDevice(nsIPresentationDevice *device) = 0;

  /* void onSessionRequest (in nsIPresentationDevice device, in AString url, in AString presentationId, in nsIPresentationControlChannel controlChannel); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnSessionRequest(nsIPresentationDevice *device, const nsAString& url, const nsAString& presentationId, nsIPresentationControlChannel *controlChannel) = 0;

  /* void onTerminateRequest (in nsIPresentationDevice device, in AString presentationId, in nsIPresentationControlChannel controlChannel, in boolean aIsFromReceiver); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnTerminateRequest(nsIPresentationDevice *device, const nsAString& presentationId, nsIPresentationControlChannel *controlChannel, bool aIsFromReceiver) = 0;

  /* void onReconnectRequest (in nsIPresentationDevice device, in AString url, in AString presentationId, in nsIPresentationControlChannel controlChannel); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnReconnectRequest(nsIPresentationDevice *device, const nsAString& url, const nsAString& presentationId, nsIPresentationControlChannel *controlChannel) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPresentationDeviceListener, NS_IPRESENTATIONDEVICELISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPRESENTATIONDEVICELISTENER \
  NS_IMETHOD AddDevice(nsIPresentationDevice *device) override; \
  NS_IMETHOD RemoveDevice(nsIPresentationDevice *device) override; \
  NS_IMETHOD UpdateDevice(nsIPresentationDevice *device) override; \
  NS_IMETHOD OnSessionRequest(nsIPresentationDevice *device, const nsAString& url, const nsAString& presentationId, nsIPresentationControlChannel *controlChannel) override; \
  NS_IMETHOD OnTerminateRequest(nsIPresentationDevice *device, const nsAString& presentationId, nsIPresentationControlChannel *controlChannel, bool aIsFromReceiver) override; \
  NS_IMETHOD OnReconnectRequest(nsIPresentationDevice *device, const nsAString& url, const nsAString& presentationId, nsIPresentationControlChannel *controlChannel) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPRESENTATIONDEVICELISTENER \
  nsresult AddDevice(nsIPresentationDevice *device); \
  nsresult RemoveDevice(nsIPresentationDevice *device); \
  nsresult UpdateDevice(nsIPresentationDevice *device); \
  nsresult OnSessionRequest(nsIPresentationDevice *device, const nsAString& url, const nsAString& presentationId, nsIPresentationControlChannel *controlChannel); \
  nsresult OnTerminateRequest(nsIPresentationDevice *device, const nsAString& presentationId, nsIPresentationControlChannel *controlChannel, bool aIsFromReceiver); \
  nsresult OnReconnectRequest(nsIPresentationDevice *device, const nsAString& url, const nsAString& presentationId, nsIPresentationControlChannel *controlChannel); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPRESENTATIONDEVICELISTENER(_to) \
  NS_IMETHOD AddDevice(nsIPresentationDevice *device) override { return _to AddDevice(device); } \
  NS_IMETHOD RemoveDevice(nsIPresentationDevice *device) override { return _to RemoveDevice(device); } \
  NS_IMETHOD UpdateDevice(nsIPresentationDevice *device) override { return _to UpdateDevice(device); } \
  NS_IMETHOD OnSessionRequest(nsIPresentationDevice *device, const nsAString& url, const nsAString& presentationId, nsIPresentationControlChannel *controlChannel) override { return _to OnSessionRequest(device, url, presentationId, controlChannel); } \
  NS_IMETHOD OnTerminateRequest(nsIPresentationDevice *device, const nsAString& presentationId, nsIPresentationControlChannel *controlChannel, bool aIsFromReceiver) override { return _to OnTerminateRequest(device, presentationId, controlChannel, aIsFromReceiver); } \
  NS_IMETHOD OnReconnectRequest(nsIPresentationDevice *device, const nsAString& url, const nsAString& presentationId, nsIPresentationControlChannel *controlChannel) override { return _to OnReconnectRequest(device, url, presentationId, controlChannel); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPRESENTATIONDEVICELISTENER(_to) \
  NS_IMETHOD AddDevice(nsIPresentationDevice *device) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddDevice(device); } \
  NS_IMETHOD RemoveDevice(nsIPresentationDevice *device) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveDevice(device); } \
  NS_IMETHOD UpdateDevice(nsIPresentationDevice *device) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UpdateDevice(device); } \
  NS_IMETHOD OnSessionRequest(nsIPresentationDevice *device, const nsAString& url, const nsAString& presentationId, nsIPresentationControlChannel *controlChannel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnSessionRequest(device, url, presentationId, controlChannel); } \
  NS_IMETHOD OnTerminateRequest(nsIPresentationDevice *device, const nsAString& presentationId, nsIPresentationControlChannel *controlChannel, bool aIsFromReceiver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnTerminateRequest(device, presentationId, controlChannel, aIsFromReceiver); } \
  NS_IMETHOD OnReconnectRequest(nsIPresentationDevice *device, const nsAString& url, const nsAString& presentationId, nsIPresentationControlChannel *controlChannel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnReconnectRequest(device, url, presentationId, controlChannel); } 


/* starting interface:    nsIPresentationDeviceProvider */
#define NS_IPRESENTATIONDEVICEPROVIDER_IID_STR "3db2578a-0f50-44ad-b01b-28427b71b7bf"

#define NS_IPRESENTATIONDEVICEPROVIDER_IID \
  {0x3db2578a, 0x0f50, 0x44ad, \
    { 0xb0, 0x1b, 0x28, 0x42, 0x7b, 0x71, 0xb7, 0xbf }}

class NS_NO_VTABLE nsIPresentationDeviceProvider : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPRESENTATIONDEVICEPROVIDER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPresentationDeviceProvider;

  /* attribute nsIPresentationDeviceListener listener; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetListener(nsIPresentationDeviceListener **aListener) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetListener(nsIPresentationDeviceListener *aListener) = 0;

  /* void forceDiscovery (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ForceDiscovery(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPresentationDeviceProvider, NS_IPRESENTATIONDEVICEPROVIDER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPRESENTATIONDEVICEPROVIDER \
  NS_IMETHOD GetListener(nsIPresentationDeviceListener **aListener) override; \
  NS_IMETHOD SetListener(nsIPresentationDeviceListener *aListener) override; \
  NS_IMETHOD ForceDiscovery(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPRESENTATIONDEVICEPROVIDER \
  nsresult GetListener(nsIPresentationDeviceListener **aListener); \
  nsresult SetListener(nsIPresentationDeviceListener *aListener); \
  nsresult ForceDiscovery(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPRESENTATIONDEVICEPROVIDER(_to) \
  NS_IMETHOD GetListener(nsIPresentationDeviceListener **aListener) override { return _to GetListener(aListener); } \
  NS_IMETHOD SetListener(nsIPresentationDeviceListener *aListener) override { return _to SetListener(aListener); } \
  NS_IMETHOD ForceDiscovery(void) override { return _to ForceDiscovery(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPRESENTATIONDEVICEPROVIDER(_to) \
  NS_IMETHOD GetListener(nsIPresentationDeviceListener **aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetListener(aListener); } \
  NS_IMETHOD SetListener(nsIPresentationDeviceListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetListener(aListener); } \
  NS_IMETHOD ForceDiscovery(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ForceDiscovery(); } 


#endif /* __gen_nsIPresentationDeviceProvider_h__ */
