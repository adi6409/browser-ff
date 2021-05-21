/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/dns/mdns/nsIDNSServiceDiscovery.idl
 */

#ifndef __gen_nsIDNSServiceDiscovery_h__
#define __gen_nsIDNSServiceDiscovery_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsICancelable; /* forward declaration */

class nsIPropertyBag2; /* forward declaration */


/* starting interface:    nsIDNSServiceInfo */
#define NS_IDNSSERVICEINFO_IID_STR "670ed0f9-2fa5-4544-bf1e-ea58ac179374"

#define NS_IDNSSERVICEINFO_IID \
  {0x670ed0f9, 0x2fa5, 0x4544, \
    { 0xbf, 0x1e, 0xea, 0x58, 0xac, 0x17, 0x93, 0x74 }}

class NS_NO_VTABLE nsIDNSServiceInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDNSSERVICEINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDNSServiceInfo;

  /* attribute AUTF8String host; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHost(nsACString& aHost) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetHost(const nsACString& aHost) = 0;

  /* attribute AUTF8String address; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAddress(nsACString& aAddress) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetAddress(const nsACString& aAddress) = 0;

  /* attribute unsigned short port; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPort(uint16_t *aPort) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPort(uint16_t aPort) = 0;

  /* attribute AUTF8String serviceName; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetServiceName(nsACString& aServiceName) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetServiceName(const nsACString& aServiceName) = 0;

  /* attribute AUTF8String serviceType; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetServiceType(nsACString& aServiceType) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetServiceType(const nsACString& aServiceType) = 0;

  /* attribute AUTF8String domainName; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDomainName(nsACString& aDomainName) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDomainName(const nsACString& aDomainName) = 0;

  /* attribute nsIPropertyBag2 attributes; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAttributes(nsIPropertyBag2 **aAttributes) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetAttributes(nsIPropertyBag2 *aAttributes) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDNSServiceInfo, NS_IDNSSERVICEINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDNSSERVICEINFO \
  NS_IMETHOD GetHost(nsACString& aHost) override; \
  NS_IMETHOD SetHost(const nsACString& aHost) override; \
  NS_IMETHOD GetAddress(nsACString& aAddress) override; \
  NS_IMETHOD SetAddress(const nsACString& aAddress) override; \
  NS_IMETHOD GetPort(uint16_t *aPort) override; \
  NS_IMETHOD SetPort(uint16_t aPort) override; \
  NS_IMETHOD GetServiceName(nsACString& aServiceName) override; \
  NS_IMETHOD SetServiceName(const nsACString& aServiceName) override; \
  NS_IMETHOD GetServiceType(nsACString& aServiceType) override; \
  NS_IMETHOD SetServiceType(const nsACString& aServiceType) override; \
  NS_IMETHOD GetDomainName(nsACString& aDomainName) override; \
  NS_IMETHOD SetDomainName(const nsACString& aDomainName) override; \
  NS_IMETHOD GetAttributes(nsIPropertyBag2 **aAttributes) override; \
  NS_IMETHOD SetAttributes(nsIPropertyBag2 *aAttributes) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDNSSERVICEINFO \
  nsresult GetHost(nsACString& aHost); \
  nsresult SetHost(const nsACString& aHost); \
  nsresult GetAddress(nsACString& aAddress); \
  nsresult SetAddress(const nsACString& aAddress); \
  nsresult GetPort(uint16_t *aPort); \
  nsresult SetPort(uint16_t aPort); \
  nsresult GetServiceName(nsACString& aServiceName); \
  nsresult SetServiceName(const nsACString& aServiceName); \
  nsresult GetServiceType(nsACString& aServiceType); \
  nsresult SetServiceType(const nsACString& aServiceType); \
  nsresult GetDomainName(nsACString& aDomainName); \
  nsresult SetDomainName(const nsACString& aDomainName); \
  nsresult GetAttributes(nsIPropertyBag2 **aAttributes); \
  nsresult SetAttributes(nsIPropertyBag2 *aAttributes); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDNSSERVICEINFO(_to) \
  NS_IMETHOD GetHost(nsACString& aHost) override { return _to GetHost(aHost); } \
  NS_IMETHOD SetHost(const nsACString& aHost) override { return _to SetHost(aHost); } \
  NS_IMETHOD GetAddress(nsACString& aAddress) override { return _to GetAddress(aAddress); } \
  NS_IMETHOD SetAddress(const nsACString& aAddress) override { return _to SetAddress(aAddress); } \
  NS_IMETHOD GetPort(uint16_t *aPort) override { return _to GetPort(aPort); } \
  NS_IMETHOD SetPort(uint16_t aPort) override { return _to SetPort(aPort); } \
  NS_IMETHOD GetServiceName(nsACString& aServiceName) override { return _to GetServiceName(aServiceName); } \
  NS_IMETHOD SetServiceName(const nsACString& aServiceName) override { return _to SetServiceName(aServiceName); } \
  NS_IMETHOD GetServiceType(nsACString& aServiceType) override { return _to GetServiceType(aServiceType); } \
  NS_IMETHOD SetServiceType(const nsACString& aServiceType) override { return _to SetServiceType(aServiceType); } \
  NS_IMETHOD GetDomainName(nsACString& aDomainName) override { return _to GetDomainName(aDomainName); } \
  NS_IMETHOD SetDomainName(const nsACString& aDomainName) override { return _to SetDomainName(aDomainName); } \
  NS_IMETHOD GetAttributes(nsIPropertyBag2 **aAttributes) override { return _to GetAttributes(aAttributes); } \
  NS_IMETHOD SetAttributes(nsIPropertyBag2 *aAttributes) override { return _to SetAttributes(aAttributes); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDNSSERVICEINFO(_to) \
  NS_IMETHOD GetHost(nsACString& aHost) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHost(aHost); } \
  NS_IMETHOD SetHost(const nsACString& aHost) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetHost(aHost); } \
  NS_IMETHOD GetAddress(nsACString& aAddress) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAddress(aAddress); } \
  NS_IMETHOD SetAddress(const nsACString& aAddress) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAddress(aAddress); } \
  NS_IMETHOD GetPort(uint16_t *aPort) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPort(aPort); } \
  NS_IMETHOD SetPort(uint16_t aPort) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPort(aPort); } \
  NS_IMETHOD GetServiceName(nsACString& aServiceName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetServiceName(aServiceName); } \
  NS_IMETHOD SetServiceName(const nsACString& aServiceName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetServiceName(aServiceName); } \
  NS_IMETHOD GetServiceType(nsACString& aServiceType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetServiceType(aServiceType); } \
  NS_IMETHOD SetServiceType(const nsACString& aServiceType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetServiceType(aServiceType); } \
  NS_IMETHOD GetDomainName(nsACString& aDomainName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDomainName(aDomainName); } \
  NS_IMETHOD SetDomainName(const nsACString& aDomainName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDomainName(aDomainName); } \
  NS_IMETHOD GetAttributes(nsIPropertyBag2 **aAttributes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAttributes(aAttributes); } \
  NS_IMETHOD SetAttributes(nsIPropertyBag2 *aAttributes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAttributes(aAttributes); } 


/* starting interface:    nsIDNSServiceDiscoveryListener */
#define NS_IDNSSERVICEDISCOVERYLISTENER_IID_STR "3025b7f2-97bb-435b-b43d-26731b3f5fc4"

#define NS_IDNSSERVICEDISCOVERYLISTENER_IID \
  {0x3025b7f2, 0x97bb, 0x435b, \
    { 0xb4, 0x3d, 0x26, 0x73, 0x1b, 0x3f, 0x5f, 0xc4 }}

class NS_NO_VTABLE nsIDNSServiceDiscoveryListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDNSSERVICEDISCOVERYLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDNSServiceDiscoveryListener;

  /* void onDiscoveryStarted (in AUTF8String aServiceType); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnDiscoveryStarted(const nsACString& aServiceType) = 0;

  /* void onDiscoveryStopped (in AUTF8String aServiceType); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnDiscoveryStopped(const nsACString& aServiceType) = 0;

  /* void onServiceFound (in nsIDNSServiceInfo aServiceInfo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnServiceFound(nsIDNSServiceInfo *aServiceInfo) = 0;

  /* void onServiceLost (in nsIDNSServiceInfo aServiceInfo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnServiceLost(nsIDNSServiceInfo *aServiceInfo) = 0;

  /* void onStartDiscoveryFailed (in AUTF8String aServiceType, in long aErrorCode); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnStartDiscoveryFailed(const nsACString& aServiceType, int32_t aErrorCode) = 0;

  /* void onStopDiscoveryFailed (in AUTF8String aServiceType, in long aErrorCode); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnStopDiscoveryFailed(const nsACString& aServiceType, int32_t aErrorCode) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDNSServiceDiscoveryListener, NS_IDNSSERVICEDISCOVERYLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDNSSERVICEDISCOVERYLISTENER \
  NS_IMETHOD OnDiscoveryStarted(const nsACString& aServiceType) override; \
  NS_IMETHOD OnDiscoveryStopped(const nsACString& aServiceType) override; \
  NS_IMETHOD OnServiceFound(nsIDNSServiceInfo *aServiceInfo) override; \
  NS_IMETHOD OnServiceLost(nsIDNSServiceInfo *aServiceInfo) override; \
  NS_IMETHOD OnStartDiscoveryFailed(const nsACString& aServiceType, int32_t aErrorCode) override; \
  NS_IMETHOD OnStopDiscoveryFailed(const nsACString& aServiceType, int32_t aErrorCode) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDNSSERVICEDISCOVERYLISTENER \
  nsresult OnDiscoveryStarted(const nsACString& aServiceType); \
  nsresult OnDiscoveryStopped(const nsACString& aServiceType); \
  nsresult OnServiceFound(nsIDNSServiceInfo *aServiceInfo); \
  nsresult OnServiceLost(nsIDNSServiceInfo *aServiceInfo); \
  nsresult OnStartDiscoveryFailed(const nsACString& aServiceType, int32_t aErrorCode); \
  nsresult OnStopDiscoveryFailed(const nsACString& aServiceType, int32_t aErrorCode); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDNSSERVICEDISCOVERYLISTENER(_to) \
  NS_IMETHOD OnDiscoveryStarted(const nsACString& aServiceType) override { return _to OnDiscoveryStarted(aServiceType); } \
  NS_IMETHOD OnDiscoveryStopped(const nsACString& aServiceType) override { return _to OnDiscoveryStopped(aServiceType); } \
  NS_IMETHOD OnServiceFound(nsIDNSServiceInfo *aServiceInfo) override { return _to OnServiceFound(aServiceInfo); } \
  NS_IMETHOD OnServiceLost(nsIDNSServiceInfo *aServiceInfo) override { return _to OnServiceLost(aServiceInfo); } \
  NS_IMETHOD OnStartDiscoveryFailed(const nsACString& aServiceType, int32_t aErrorCode) override { return _to OnStartDiscoveryFailed(aServiceType, aErrorCode); } \
  NS_IMETHOD OnStopDiscoveryFailed(const nsACString& aServiceType, int32_t aErrorCode) override { return _to OnStopDiscoveryFailed(aServiceType, aErrorCode); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDNSSERVICEDISCOVERYLISTENER(_to) \
  NS_IMETHOD OnDiscoveryStarted(const nsACString& aServiceType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnDiscoveryStarted(aServiceType); } \
  NS_IMETHOD OnDiscoveryStopped(const nsACString& aServiceType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnDiscoveryStopped(aServiceType); } \
  NS_IMETHOD OnServiceFound(nsIDNSServiceInfo *aServiceInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnServiceFound(aServiceInfo); } \
  NS_IMETHOD OnServiceLost(nsIDNSServiceInfo *aServiceInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnServiceLost(aServiceInfo); } \
  NS_IMETHOD OnStartDiscoveryFailed(const nsACString& aServiceType, int32_t aErrorCode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnStartDiscoveryFailed(aServiceType, aErrorCode); } \
  NS_IMETHOD OnStopDiscoveryFailed(const nsACString& aServiceType, int32_t aErrorCode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnStopDiscoveryFailed(aServiceType, aErrorCode); } 


/* starting interface:    nsIDNSRegistrationListener */
#define NS_IDNSREGISTRATIONLISTENER_IID_STR "e165e4be-abf4-4963-a66d-ed3ca116e5e4"

#define NS_IDNSREGISTRATIONLISTENER_IID \
  {0xe165e4be, 0xabf4, 0x4963, \
    { 0xa6, 0x6d, 0xed, 0x3c, 0xa1, 0x16, 0xe5, 0xe4 }}

class NS_NO_VTABLE nsIDNSRegistrationListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDNSREGISTRATIONLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDNSRegistrationListener;

  enum {
    ERROR_SERVICE_NOT_RUNNING = -65563
  };

  /* void onServiceRegistered (in nsIDNSServiceInfo aServiceInfo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnServiceRegistered(nsIDNSServiceInfo *aServiceInfo) = 0;

  /* void onServiceUnregistered (in nsIDNSServiceInfo aServiceInfo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnServiceUnregistered(nsIDNSServiceInfo *aServiceInfo) = 0;

  /* void onRegistrationFailed (in nsIDNSServiceInfo aServiceInfo, in long aErrorCode); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnRegistrationFailed(nsIDNSServiceInfo *aServiceInfo, int32_t aErrorCode) = 0;

  /* void onUnregistrationFailed (in nsIDNSServiceInfo aServiceInfo, in long aErrorCode); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnUnregistrationFailed(nsIDNSServiceInfo *aServiceInfo, int32_t aErrorCode) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDNSRegistrationListener, NS_IDNSREGISTRATIONLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDNSREGISTRATIONLISTENER \
  NS_IMETHOD OnServiceRegistered(nsIDNSServiceInfo *aServiceInfo) override; \
  NS_IMETHOD OnServiceUnregistered(nsIDNSServiceInfo *aServiceInfo) override; \
  NS_IMETHOD OnRegistrationFailed(nsIDNSServiceInfo *aServiceInfo, int32_t aErrorCode) override; \
  NS_IMETHOD OnUnregistrationFailed(nsIDNSServiceInfo *aServiceInfo, int32_t aErrorCode) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDNSREGISTRATIONLISTENER \
  nsresult OnServiceRegistered(nsIDNSServiceInfo *aServiceInfo); \
  nsresult OnServiceUnregistered(nsIDNSServiceInfo *aServiceInfo); \
  nsresult OnRegistrationFailed(nsIDNSServiceInfo *aServiceInfo, int32_t aErrorCode); \
  nsresult OnUnregistrationFailed(nsIDNSServiceInfo *aServiceInfo, int32_t aErrorCode); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDNSREGISTRATIONLISTENER(_to) \
  NS_IMETHOD OnServiceRegistered(nsIDNSServiceInfo *aServiceInfo) override { return _to OnServiceRegistered(aServiceInfo); } \
  NS_IMETHOD OnServiceUnregistered(nsIDNSServiceInfo *aServiceInfo) override { return _to OnServiceUnregistered(aServiceInfo); } \
  NS_IMETHOD OnRegistrationFailed(nsIDNSServiceInfo *aServiceInfo, int32_t aErrorCode) override { return _to OnRegistrationFailed(aServiceInfo, aErrorCode); } \
  NS_IMETHOD OnUnregistrationFailed(nsIDNSServiceInfo *aServiceInfo, int32_t aErrorCode) override { return _to OnUnregistrationFailed(aServiceInfo, aErrorCode); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDNSREGISTRATIONLISTENER(_to) \
  NS_IMETHOD OnServiceRegistered(nsIDNSServiceInfo *aServiceInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnServiceRegistered(aServiceInfo); } \
  NS_IMETHOD OnServiceUnregistered(nsIDNSServiceInfo *aServiceInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnServiceUnregistered(aServiceInfo); } \
  NS_IMETHOD OnRegistrationFailed(nsIDNSServiceInfo *aServiceInfo, int32_t aErrorCode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnRegistrationFailed(aServiceInfo, aErrorCode); } \
  NS_IMETHOD OnUnregistrationFailed(nsIDNSServiceInfo *aServiceInfo, int32_t aErrorCode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnUnregistrationFailed(aServiceInfo, aErrorCode); } 


/* starting interface:    nsIDNSServiceResolveListener */
#define NS_IDNSSERVICERESOLVELISTENER_IID_STR "24ee6408-648e-421d-accf-c6e5adeccf97"

#define NS_IDNSSERVICERESOLVELISTENER_IID \
  {0x24ee6408, 0x648e, 0x421d, \
    { 0xac, 0xcf, 0xc6, 0xe5, 0xad, 0xec, 0xcf, 0x97 }}

class NS_NO_VTABLE nsIDNSServiceResolveListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDNSSERVICERESOLVELISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDNSServiceResolveListener;

  /* void onServiceResolved (in nsIDNSServiceInfo aServiceInfo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnServiceResolved(nsIDNSServiceInfo *aServiceInfo) = 0;

  /* void onResolveFailed (in nsIDNSServiceInfo aServiceInfo, in long aErrorCode); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnResolveFailed(nsIDNSServiceInfo *aServiceInfo, int32_t aErrorCode) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDNSServiceResolveListener, NS_IDNSSERVICERESOLVELISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDNSSERVICERESOLVELISTENER \
  NS_IMETHOD OnServiceResolved(nsIDNSServiceInfo *aServiceInfo) override; \
  NS_IMETHOD OnResolveFailed(nsIDNSServiceInfo *aServiceInfo, int32_t aErrorCode) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDNSSERVICERESOLVELISTENER \
  nsresult OnServiceResolved(nsIDNSServiceInfo *aServiceInfo); \
  nsresult OnResolveFailed(nsIDNSServiceInfo *aServiceInfo, int32_t aErrorCode); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDNSSERVICERESOLVELISTENER(_to) \
  NS_IMETHOD OnServiceResolved(nsIDNSServiceInfo *aServiceInfo) override { return _to OnServiceResolved(aServiceInfo); } \
  NS_IMETHOD OnResolveFailed(nsIDNSServiceInfo *aServiceInfo, int32_t aErrorCode) override { return _to OnResolveFailed(aServiceInfo, aErrorCode); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDNSSERVICERESOLVELISTENER(_to) \
  NS_IMETHOD OnServiceResolved(nsIDNSServiceInfo *aServiceInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnServiceResolved(aServiceInfo); } \
  NS_IMETHOD OnResolveFailed(nsIDNSServiceInfo *aServiceInfo, int32_t aErrorCode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnResolveFailed(aServiceInfo, aErrorCode); } 


/* starting interface:    nsIDNSServiceDiscovery */
#define NS_IDNSSERVICEDISCOVERY_IID_STR "6487899b-beb1-455a-ba65-e4fd465066d7"

#define NS_IDNSSERVICEDISCOVERY_IID \
  {0x6487899b, 0xbeb1, 0x455a, \
    { 0xba, 0x65, 0xe4, 0xfd, 0x46, 0x50, 0x66, 0xd7 }}

class NS_NO_VTABLE nsIDNSServiceDiscovery : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDNSSERVICEDISCOVERY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDNSServiceDiscovery;

  /* nsICancelable startDiscovery (in AUTF8String aServiceType, in nsIDNSServiceDiscoveryListener aListener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD StartDiscovery(const nsACString& aServiceType, nsIDNSServiceDiscoveryListener *aListener, nsICancelable **_retval) = 0;

  /* nsICancelable registerService (in nsIDNSServiceInfo aServiceInfo, in nsIDNSRegistrationListener aListener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RegisterService(nsIDNSServiceInfo *aServiceInfo, nsIDNSRegistrationListener *aListener, nsICancelable **_retval) = 0;

  /* void resolveService (in nsIDNSServiceInfo aServiceInfo, in nsIDNSServiceResolveListener aListener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ResolveService(nsIDNSServiceInfo *aServiceInfo, nsIDNSServiceResolveListener *aListener) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDNSServiceDiscovery, NS_IDNSSERVICEDISCOVERY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDNSSERVICEDISCOVERY \
  NS_IMETHOD StartDiscovery(const nsACString& aServiceType, nsIDNSServiceDiscoveryListener *aListener, nsICancelable **_retval) override; \
  NS_IMETHOD RegisterService(nsIDNSServiceInfo *aServiceInfo, nsIDNSRegistrationListener *aListener, nsICancelable **_retval) override; \
  NS_IMETHOD ResolveService(nsIDNSServiceInfo *aServiceInfo, nsIDNSServiceResolveListener *aListener) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDNSSERVICEDISCOVERY \
  nsresult StartDiscovery(const nsACString& aServiceType, nsIDNSServiceDiscoveryListener *aListener, nsICancelable **_retval); \
  nsresult RegisterService(nsIDNSServiceInfo *aServiceInfo, nsIDNSRegistrationListener *aListener, nsICancelable **_retval); \
  nsresult ResolveService(nsIDNSServiceInfo *aServiceInfo, nsIDNSServiceResolveListener *aListener); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDNSSERVICEDISCOVERY(_to) \
  NS_IMETHOD StartDiscovery(const nsACString& aServiceType, nsIDNSServiceDiscoveryListener *aListener, nsICancelable **_retval) override { return _to StartDiscovery(aServiceType, aListener, _retval); } \
  NS_IMETHOD RegisterService(nsIDNSServiceInfo *aServiceInfo, nsIDNSRegistrationListener *aListener, nsICancelable **_retval) override { return _to RegisterService(aServiceInfo, aListener, _retval); } \
  NS_IMETHOD ResolveService(nsIDNSServiceInfo *aServiceInfo, nsIDNSServiceResolveListener *aListener) override { return _to ResolveService(aServiceInfo, aListener); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDNSSERVICEDISCOVERY(_to) \
  NS_IMETHOD StartDiscovery(const nsACString& aServiceType, nsIDNSServiceDiscoveryListener *aListener, nsICancelable **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StartDiscovery(aServiceType, aListener, _retval); } \
  NS_IMETHOD RegisterService(nsIDNSServiceInfo *aServiceInfo, nsIDNSRegistrationListener *aListener, nsICancelable **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterService(aServiceInfo, aListener, _retval); } \
  NS_IMETHOD ResolveService(nsIDNSServiceInfo *aServiceInfo, nsIDNSServiceResolveListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ResolveService(aServiceInfo, aListener); } 

#define DNSSERVICEDISCOVERY_CONTRACT_ID \
  "@mozilla.org/toolkit/components/mdnsresponder/dns-sd;1"
#define DNSSERVICEINFO_CONTRACT_ID \
  "@mozilla.org/toolkit/components/mdnsresponder/dns-info;1"

#endif /* __gen_nsIDNSServiceDiscovery_h__ */
