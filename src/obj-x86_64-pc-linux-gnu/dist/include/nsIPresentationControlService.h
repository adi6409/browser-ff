/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationControlService.idl
 */

#ifndef __gen_nsIPresentationControlService_h__
#define __gen_nsIPresentationControlService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIPresentationControlChannel; /* forward declaration */

#define PRESENTATION_CONTROL_SERVICE_CONTACT_ID \
  "@mozilla.org/presentation/control-service;1"

/* starting interface:    nsITCPDeviceInfo */
#define NS_ITCPDEVICEINFO_IID_STR "296fd171-e4d0-4de0-99ff-ad8ed52ddef3"

#define NS_ITCPDEVICEINFO_IID \
  {0x296fd171, 0xe4d0, 0x4de0, \
    { 0x99, 0xff, 0xad, 0x8e, 0xd5, 0x2d, 0xde, 0xf3 }}

class NS_NO_VTABLE nsITCPDeviceInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITCPDEVICEINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITCPDeviceInfo;

  /* readonly attribute AUTF8String id; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetId(nsACString& aId) = 0;

  /* readonly attribute AUTF8String address; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAddress(nsACString& aAddress) = 0;

  /* readonly attribute uint16_t port; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPort(uint16_t *aPort) = 0;

  /* readonly attribute AUTF8String certFingerprint; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCertFingerprint(nsACString& aCertFingerprint) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITCPDeviceInfo, NS_ITCPDEVICEINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITCPDEVICEINFO \
  NS_IMETHOD GetId(nsACString& aId) override; \
  NS_IMETHOD GetAddress(nsACString& aAddress) override; \
  NS_IMETHOD GetPort(uint16_t *aPort) override; \
  NS_IMETHOD GetCertFingerprint(nsACString& aCertFingerprint) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITCPDEVICEINFO \
  nsresult GetId(nsACString& aId); \
  nsresult GetAddress(nsACString& aAddress); \
  nsresult GetPort(uint16_t *aPort); \
  nsresult GetCertFingerprint(nsACString& aCertFingerprint); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITCPDEVICEINFO(_to) \
  NS_IMETHOD GetId(nsACString& aId) override { return _to GetId(aId); } \
  NS_IMETHOD GetAddress(nsACString& aAddress) override { return _to GetAddress(aAddress); } \
  NS_IMETHOD GetPort(uint16_t *aPort) override { return _to GetPort(aPort); } \
  NS_IMETHOD GetCertFingerprint(nsACString& aCertFingerprint) override { return _to GetCertFingerprint(aCertFingerprint); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITCPDEVICEINFO(_to) \
  NS_IMETHOD GetId(nsACString& aId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetId(aId); } \
  NS_IMETHOD GetAddress(nsACString& aAddress) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAddress(aAddress); } \
  NS_IMETHOD GetPort(uint16_t *aPort) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPort(aPort); } \
  NS_IMETHOD GetCertFingerprint(nsACString& aCertFingerprint) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCertFingerprint(aCertFingerprint); } 


/* starting interface:    nsIPresentationControlServerListener */
#define NS_IPRESENTATIONCONTROLSERVERLISTENER_IID_STR "09bddfaf-fcc2-4dc9-b33e-a509a1c2fb6d"

#define NS_IPRESENTATIONCONTROLSERVERLISTENER_IID \
  {0x09bddfaf, 0xfcc2, 0x4dc9, \
    { 0xb3, 0x3e, 0xa5, 0x09, 0xa1, 0xc2, 0xfb, 0x6d }}

class NS_NO_VTABLE nsIPresentationControlServerListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPRESENTATIONCONTROLSERVERLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPresentationControlServerListener;

  /* void onServerReady (in uint16_t aPort, in AUTF8String aCertFingerprint); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnServerReady(uint16_t aPort, const nsACString& aCertFingerprint) = 0;

  /* void onServerStopped (in nsresult aResult); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnServerStopped(nsresult aResult) = 0;

  /* void onSessionRequest (in nsITCPDeviceInfo aDeviceInfo, in AString aUrl, in AString aPresentationId, in nsIPresentationControlChannel aControlChannel); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnSessionRequest(nsITCPDeviceInfo *aDeviceInfo, const nsAString& aUrl, const nsAString& aPresentationId, nsIPresentationControlChannel *aControlChannel) = 0;

  /* void onTerminateRequest (in nsITCPDeviceInfo aDeviceInfo, in AString aPresentationId, in nsIPresentationControlChannel aControlChannel, in boolean aIsFromReceiver); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnTerminateRequest(nsITCPDeviceInfo *aDeviceInfo, const nsAString& aPresentationId, nsIPresentationControlChannel *aControlChannel, bool aIsFromReceiver) = 0;

  /* void onReconnectRequest (in nsITCPDeviceInfo aDeviceInfo, in AString url, in AString aPresentationId, in nsIPresentationControlChannel aControlChannel); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnReconnectRequest(nsITCPDeviceInfo *aDeviceInfo, const nsAString& url, const nsAString& aPresentationId, nsIPresentationControlChannel *aControlChannel) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPresentationControlServerListener, NS_IPRESENTATIONCONTROLSERVERLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPRESENTATIONCONTROLSERVERLISTENER \
  NS_IMETHOD OnServerReady(uint16_t aPort, const nsACString& aCertFingerprint) override; \
  NS_IMETHOD OnServerStopped(nsresult aResult) override; \
  NS_IMETHOD OnSessionRequest(nsITCPDeviceInfo *aDeviceInfo, const nsAString& aUrl, const nsAString& aPresentationId, nsIPresentationControlChannel *aControlChannel) override; \
  NS_IMETHOD OnTerminateRequest(nsITCPDeviceInfo *aDeviceInfo, const nsAString& aPresentationId, nsIPresentationControlChannel *aControlChannel, bool aIsFromReceiver) override; \
  NS_IMETHOD OnReconnectRequest(nsITCPDeviceInfo *aDeviceInfo, const nsAString& url, const nsAString& aPresentationId, nsIPresentationControlChannel *aControlChannel) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPRESENTATIONCONTROLSERVERLISTENER \
  nsresult OnServerReady(uint16_t aPort, const nsACString& aCertFingerprint); \
  nsresult OnServerStopped(nsresult aResult); \
  nsresult OnSessionRequest(nsITCPDeviceInfo *aDeviceInfo, const nsAString& aUrl, const nsAString& aPresentationId, nsIPresentationControlChannel *aControlChannel); \
  nsresult OnTerminateRequest(nsITCPDeviceInfo *aDeviceInfo, const nsAString& aPresentationId, nsIPresentationControlChannel *aControlChannel, bool aIsFromReceiver); \
  nsresult OnReconnectRequest(nsITCPDeviceInfo *aDeviceInfo, const nsAString& url, const nsAString& aPresentationId, nsIPresentationControlChannel *aControlChannel); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPRESENTATIONCONTROLSERVERLISTENER(_to) \
  NS_IMETHOD OnServerReady(uint16_t aPort, const nsACString& aCertFingerprint) override { return _to OnServerReady(aPort, aCertFingerprint); } \
  NS_IMETHOD OnServerStopped(nsresult aResult) override { return _to OnServerStopped(aResult); } \
  NS_IMETHOD OnSessionRequest(nsITCPDeviceInfo *aDeviceInfo, const nsAString& aUrl, const nsAString& aPresentationId, nsIPresentationControlChannel *aControlChannel) override { return _to OnSessionRequest(aDeviceInfo, aUrl, aPresentationId, aControlChannel); } \
  NS_IMETHOD OnTerminateRequest(nsITCPDeviceInfo *aDeviceInfo, const nsAString& aPresentationId, nsIPresentationControlChannel *aControlChannel, bool aIsFromReceiver) override { return _to OnTerminateRequest(aDeviceInfo, aPresentationId, aControlChannel, aIsFromReceiver); } \
  NS_IMETHOD OnReconnectRequest(nsITCPDeviceInfo *aDeviceInfo, const nsAString& url, const nsAString& aPresentationId, nsIPresentationControlChannel *aControlChannel) override { return _to OnReconnectRequest(aDeviceInfo, url, aPresentationId, aControlChannel); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPRESENTATIONCONTROLSERVERLISTENER(_to) \
  NS_IMETHOD OnServerReady(uint16_t aPort, const nsACString& aCertFingerprint) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnServerReady(aPort, aCertFingerprint); } \
  NS_IMETHOD OnServerStopped(nsresult aResult) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnServerStopped(aResult); } \
  NS_IMETHOD OnSessionRequest(nsITCPDeviceInfo *aDeviceInfo, const nsAString& aUrl, const nsAString& aPresentationId, nsIPresentationControlChannel *aControlChannel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnSessionRequest(aDeviceInfo, aUrl, aPresentationId, aControlChannel); } \
  NS_IMETHOD OnTerminateRequest(nsITCPDeviceInfo *aDeviceInfo, const nsAString& aPresentationId, nsIPresentationControlChannel *aControlChannel, bool aIsFromReceiver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnTerminateRequest(aDeviceInfo, aPresentationId, aControlChannel, aIsFromReceiver); } \
  NS_IMETHOD OnReconnectRequest(nsITCPDeviceInfo *aDeviceInfo, const nsAString& url, const nsAString& aPresentationId, nsIPresentationControlChannel *aControlChannel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnReconnectRequest(aDeviceInfo, url, aPresentationId, aControlChannel); } 


/* starting interface:    nsIPresentationControlService */
#define NS_IPRESENTATIONCONTROLSERVICE_IID_STR "55d6b605-2389-4aae-a8fe-60d4440540ea"

#define NS_IPRESENTATIONCONTROLSERVICE_IID \
  {0x55d6b605, 0x2389, 0x4aae, \
    { 0xa8, 0xfe, 0x60, 0xd4, 0x44, 0x05, 0x40, 0xea }}

class NS_NO_VTABLE nsIPresentationControlService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPRESENTATIONCONTROLSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPresentationControlService;

  /* void startServer (in boolean aEncrypted, [optional] in uint16_t aPort); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD StartServer(bool aEncrypted, uint16_t aPort) = 0;

  /* nsIPresentationControlChannel connect (in nsITCPDeviceInfo aDeviceInfo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Connect(nsITCPDeviceInfo *aDeviceInfo, nsIPresentationControlChannel **_retval) = 0;

  /* boolean isCompatibleServer (in uint32_t aVersion); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsCompatibleServer(uint32_t aVersion, bool *_retval) = 0;

  /* void close (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Close(void) = 0;

  /* readonly attribute uint16_t port; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPort(uint16_t *aPort) = 0;

  /* readonly attribute uint32_t version; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetVersion(uint32_t *aVersion) = 0;

  /* attribute AUTF8String id; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetId(nsACString& aId) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetId(const nsACString& aId) = 0;

  /* attribute AUTF8String certFingerprint; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCertFingerprint(nsACString& aCertFingerprint) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCertFingerprint(const nsACString& aCertFingerprint) = 0;

  /* attribute nsIPresentationControlServerListener listener; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetListener(nsIPresentationControlServerListener **aListener) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetListener(nsIPresentationControlServerListener *aListener) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPresentationControlService, NS_IPRESENTATIONCONTROLSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPRESENTATIONCONTROLSERVICE \
  NS_IMETHOD StartServer(bool aEncrypted, uint16_t aPort) override; \
  NS_IMETHOD Connect(nsITCPDeviceInfo *aDeviceInfo, nsIPresentationControlChannel **_retval) override; \
  NS_IMETHOD IsCompatibleServer(uint32_t aVersion, bool *_retval) override; \
  NS_IMETHOD Close(void) override; \
  NS_IMETHOD GetPort(uint16_t *aPort) override; \
  NS_IMETHOD GetVersion(uint32_t *aVersion) override; \
  NS_IMETHOD GetId(nsACString& aId) override; \
  NS_IMETHOD SetId(const nsACString& aId) override; \
  NS_IMETHOD GetCertFingerprint(nsACString& aCertFingerprint) override; \
  NS_IMETHOD SetCertFingerprint(const nsACString& aCertFingerprint) override; \
  NS_IMETHOD GetListener(nsIPresentationControlServerListener **aListener) override; \
  NS_IMETHOD SetListener(nsIPresentationControlServerListener *aListener) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPRESENTATIONCONTROLSERVICE \
  nsresult StartServer(bool aEncrypted, uint16_t aPort); \
  nsresult Connect(nsITCPDeviceInfo *aDeviceInfo, nsIPresentationControlChannel **_retval); \
  nsresult IsCompatibleServer(uint32_t aVersion, bool *_retval); \
  nsresult Close(void); \
  nsresult GetPort(uint16_t *aPort); \
  nsresult GetVersion(uint32_t *aVersion); \
  nsresult GetId(nsACString& aId); \
  nsresult SetId(const nsACString& aId); \
  nsresult GetCertFingerprint(nsACString& aCertFingerprint); \
  nsresult SetCertFingerprint(const nsACString& aCertFingerprint); \
  nsresult GetListener(nsIPresentationControlServerListener **aListener); \
  nsresult SetListener(nsIPresentationControlServerListener *aListener); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPRESENTATIONCONTROLSERVICE(_to) \
  NS_IMETHOD StartServer(bool aEncrypted, uint16_t aPort) override { return _to StartServer(aEncrypted, aPort); } \
  NS_IMETHOD Connect(nsITCPDeviceInfo *aDeviceInfo, nsIPresentationControlChannel **_retval) override { return _to Connect(aDeviceInfo, _retval); } \
  NS_IMETHOD IsCompatibleServer(uint32_t aVersion, bool *_retval) override { return _to IsCompatibleServer(aVersion, _retval); } \
  NS_IMETHOD Close(void) override { return _to Close(); } \
  NS_IMETHOD GetPort(uint16_t *aPort) override { return _to GetPort(aPort); } \
  NS_IMETHOD GetVersion(uint32_t *aVersion) override { return _to GetVersion(aVersion); } \
  NS_IMETHOD GetId(nsACString& aId) override { return _to GetId(aId); } \
  NS_IMETHOD SetId(const nsACString& aId) override { return _to SetId(aId); } \
  NS_IMETHOD GetCertFingerprint(nsACString& aCertFingerprint) override { return _to GetCertFingerprint(aCertFingerprint); } \
  NS_IMETHOD SetCertFingerprint(const nsACString& aCertFingerprint) override { return _to SetCertFingerprint(aCertFingerprint); } \
  NS_IMETHOD GetListener(nsIPresentationControlServerListener **aListener) override { return _to GetListener(aListener); } \
  NS_IMETHOD SetListener(nsIPresentationControlServerListener *aListener) override { return _to SetListener(aListener); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPRESENTATIONCONTROLSERVICE(_to) \
  NS_IMETHOD StartServer(bool aEncrypted, uint16_t aPort) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StartServer(aEncrypted, aPort); } \
  NS_IMETHOD Connect(nsITCPDeviceInfo *aDeviceInfo, nsIPresentationControlChannel **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Connect(aDeviceInfo, _retval); } \
  NS_IMETHOD IsCompatibleServer(uint32_t aVersion, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsCompatibleServer(aVersion, _retval); } \
  NS_IMETHOD Close(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Close(); } \
  NS_IMETHOD GetPort(uint16_t *aPort) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPort(aPort); } \
  NS_IMETHOD GetVersion(uint32_t *aVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetVersion(aVersion); } \
  NS_IMETHOD GetId(nsACString& aId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetId(aId); } \
  NS_IMETHOD SetId(const nsACString& aId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetId(aId); } \
  NS_IMETHOD GetCertFingerprint(nsACString& aCertFingerprint) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCertFingerprint(aCertFingerprint); } \
  NS_IMETHOD SetCertFingerprint(const nsACString& aCertFingerprint) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCertFingerprint(aCertFingerprint); } \
  NS_IMETHOD GetListener(nsIPresentationControlServerListener **aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetListener(aListener); } \
  NS_IMETHOD SetListener(nsIPresentationControlServerListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetListener(aListener); } 


#endif /* __gen_nsIPresentationControlService_h__ */
