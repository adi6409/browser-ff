/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIIOService.idl
 */

#ifndef __gen_nsIIOService_h__
#define __gen_nsIIOService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIContentPolicy_h__
#include "nsIContentPolicy.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIProtocolHandler; /* forward declaration */

class nsIChannel; /* forward declaration */

class nsIURI; /* forward declaration */

class nsIFile; /* forward declaration */

class nsIPrincipal; /* forward declaration */

class nsILoadInfo; /* forward declaration */

class nsINode; /* webidl Node */

#include "mozilla/Maybe.h"
namespace mozilla {
namespace dom {
class ClientInfo;
class ServiceWorkerDescriptor;
} // namespace dom
} // namespace mozilla

/* starting interface:    nsIIOService */
#define NS_IIOSERVICE_IID_STR "4286de5a-b2ea-446f-8f70-e2a461f42694"

#define NS_IIOSERVICE_IID \
  {0x4286de5a, 0xb2ea, 0x446f, \
    { 0x8f, 0x70, 0xe2, 0xa4, 0x61, 0xf4, 0x26, 0x94 }}

class NS_NO_VTABLE nsIIOService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IIOSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIIOService;

  /* nsIProtocolHandler getProtocolHandler (in string aScheme); */
  NS_IMETHOD GetProtocolHandler(const char * aScheme, nsIProtocolHandler **_retval) = 0;

  /* unsigned long getProtocolFlags (in string aScheme); */
  NS_IMETHOD GetProtocolFlags(const char * aScheme, uint32_t *_retval) = 0;

  /* nsIURI newURI (in AUTF8String aSpec, [optional] in string aOriginCharset, [optional] in nsIURI aBaseURI); */
  NS_IMETHOD NewURI(const nsACString& aSpec, const char * aOriginCharset, nsIURI *aBaseURI, nsIURI **_retval) = 0;

  /* nsIURI newFileURI (in nsIFile aFile); */
  NS_IMETHOD NewFileURI(nsIFile *aFile, nsIURI **_retval) = 0;

  /* nsIURI createExposableURI (in nsIURI aURI); */
  NS_IMETHOD CreateExposableURI(nsIURI *aURI, nsIURI **_retval) = 0;

  /* nsIChannel newChannelFromURI (in nsIURI aURI, in Node aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in unsigned long aSecurityFlags, in nsContentPolicyType aContentPolicyType); */
  NS_IMETHOD NewChannelFromURI(nsIURI *aURI, nsINode *aLoadingNode, nsIPrincipal *aLoadingPrincipal, nsIPrincipal *aTriggeringPrincipal, uint32_t aSecurityFlags, nsContentPolicyType aContentPolicyType, nsIChannel **_retval) = 0;

  /* [noscript,nostdcall,notxpcom] nsresult NewChannelFromURIWithClientAndController (in nsIURI aURI, in Node aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in const_MaybeClientInfoRef aLoadingClientInfo, in const_MaybeServiceWorkerDescriptorRef aController, in unsigned long aSecurityFlags, in nsContentPolicyType aContentPolicyType, in unsigned long aSandboxFlags, out nsIChannel aResult); */
  virtual nsresult NewChannelFromURIWithClientAndController(nsIURI *aURI, nsINode *aLoadingNode, nsIPrincipal *aLoadingPrincipal, nsIPrincipal *aTriggeringPrincipal, const mozilla::Maybe<mozilla::dom::ClientInfo> & aLoadingClientInfo, const mozilla::Maybe<mozilla::dom::ServiceWorkerDescriptor> & aController, uint32_t aSecurityFlags, nsContentPolicyType aContentPolicyType, uint32_t aSandboxFlags, nsIChannel **aResult) = 0;

  /* nsIChannel newChannelFromURIWithLoadInfo (in nsIURI aURI, in nsILoadInfo aLoadInfo); */
  NS_IMETHOD NewChannelFromURIWithLoadInfo(nsIURI *aURI, nsILoadInfo *aLoadInfo, nsIChannel **_retval) = 0;

  /* nsIChannel newChannel (in AUTF8String aSpec, in string aOriginCharset, in nsIURI aBaseURI, in Node aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in unsigned long aSecurityFlags, in nsContentPolicyType aContentPolicyType); */
  NS_IMETHOD NewChannel(const nsACString& aSpec, const char * aOriginCharset, nsIURI *aBaseURI, nsINode *aLoadingNode, nsIPrincipal *aLoadingPrincipal, nsIPrincipal *aTriggeringPrincipal, uint32_t aSecurityFlags, nsContentPolicyType aContentPolicyType, nsIChannel **_retval) = 0;

  /* attribute boolean offline; */
  NS_IMETHOD GetOffline(bool *aOffline) = 0;
  NS_IMETHOD SetOffline(bool aOffline) = 0;

  /* readonly attribute boolean connectivity; */
  NS_IMETHOD GetConnectivity(bool *aConnectivity) = 0;

  /* boolean allowPort (in long aPort, in string aScheme); */
  NS_IMETHOD AllowPort(int32_t aPort, const char * aScheme, bool *_retval) = 0;

  /* ACString extractScheme (in AUTF8String urlString); */
  NS_IMETHOD ExtractScheme(const nsACString& urlString, nsACString& _retval) = 0;

  /* boolean hostnameIsLocalIPAddress (in nsIURI aURI); */
  NS_IMETHOD HostnameIsLocalIPAddress(nsIURI *aURI, bool *_retval) = 0;

  /* boolean hostnameIsSharedIPAddress (in nsIURI aURI); */
  NS_IMETHOD HostnameIsSharedIPAddress(nsIURI *aURI, bool *_retval) = 0;

  /* attribute boolean manageOfflineStatus; */
  NS_IMETHOD GetManageOfflineStatus(bool *aManageOfflineStatus) = 0;
  NS_IMETHOD SetManageOfflineStatus(bool aManageOfflineStatus) = 0;

  /* nsIChannel newChannelFromURIWithProxyFlags (in nsIURI aURI, in nsIURI aProxyURI, in unsigned long aProxyFlags, in Node aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in unsigned long aSecurityFlags, in nsContentPolicyType aContentPolicyType); */
  NS_IMETHOD NewChannelFromURIWithProxyFlags(nsIURI *aURI, nsIURI *aProxyURI, uint32_t aProxyFlags, nsINode *aLoadingNode, nsIPrincipal *aLoadingPrincipal, nsIPrincipal *aTriggeringPrincipal, uint32_t aSecurityFlags, nsContentPolicyType aContentPolicyType, nsIChannel **_retval) = 0;

  /* readonly attribute boolean socketProcessLaunched; */
  NS_IMETHOD GetSocketProcessLaunched(bool *aSocketProcessLaunched) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIIOService, NS_IIOSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIIOSERVICE \
  NS_IMETHOD GetProtocolHandler(const char * aScheme, nsIProtocolHandler **_retval) override; \
  NS_IMETHOD GetProtocolFlags(const char * aScheme, uint32_t *_retval) override; \
  NS_IMETHOD NewURI(const nsACString& aSpec, const char * aOriginCharset, nsIURI *aBaseURI, nsIURI **_retval) override; \
  NS_IMETHOD NewFileURI(nsIFile *aFile, nsIURI **_retval) override; \
  NS_IMETHOD CreateExposableURI(nsIURI *aURI, nsIURI **_retval) override; \
  NS_IMETHOD NewChannelFromURI(nsIURI *aURI, nsINode *aLoadingNode, nsIPrincipal *aLoadingPrincipal, nsIPrincipal *aTriggeringPrincipal, uint32_t aSecurityFlags, nsContentPolicyType aContentPolicyType, nsIChannel **_retval) override; \
  virtual nsresult NewChannelFromURIWithClientAndController(nsIURI *aURI, nsINode *aLoadingNode, nsIPrincipal *aLoadingPrincipal, nsIPrincipal *aTriggeringPrincipal, const mozilla::Maybe<mozilla::dom::ClientInfo> & aLoadingClientInfo, const mozilla::Maybe<mozilla::dom::ServiceWorkerDescriptor> & aController, uint32_t aSecurityFlags, nsContentPolicyType aContentPolicyType, uint32_t aSandboxFlags, nsIChannel **aResult) override; \
  NS_IMETHOD NewChannelFromURIWithLoadInfo(nsIURI *aURI, nsILoadInfo *aLoadInfo, nsIChannel **_retval) override; \
  NS_IMETHOD NewChannel(const nsACString& aSpec, const char * aOriginCharset, nsIURI *aBaseURI, nsINode *aLoadingNode, nsIPrincipal *aLoadingPrincipal, nsIPrincipal *aTriggeringPrincipal, uint32_t aSecurityFlags, nsContentPolicyType aContentPolicyType, nsIChannel **_retval) override; \
  NS_IMETHOD GetOffline(bool *aOffline) override; \
  NS_IMETHOD SetOffline(bool aOffline) override; \
  NS_IMETHOD GetConnectivity(bool *aConnectivity) override; \
  NS_IMETHOD AllowPort(int32_t aPort, const char * aScheme, bool *_retval) override; \
  NS_IMETHOD ExtractScheme(const nsACString& urlString, nsACString& _retval) override; \
  NS_IMETHOD HostnameIsLocalIPAddress(nsIURI *aURI, bool *_retval) override; \
  NS_IMETHOD HostnameIsSharedIPAddress(nsIURI *aURI, bool *_retval) override; \
  NS_IMETHOD GetManageOfflineStatus(bool *aManageOfflineStatus) override; \
  NS_IMETHOD SetManageOfflineStatus(bool aManageOfflineStatus) override; \
  NS_IMETHOD NewChannelFromURIWithProxyFlags(nsIURI *aURI, nsIURI *aProxyURI, uint32_t aProxyFlags, nsINode *aLoadingNode, nsIPrincipal *aLoadingPrincipal, nsIPrincipal *aTriggeringPrincipal, uint32_t aSecurityFlags, nsContentPolicyType aContentPolicyType, nsIChannel **_retval) override; \
  NS_IMETHOD GetSocketProcessLaunched(bool *aSocketProcessLaunched) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIIOSERVICE \
  nsresult GetProtocolHandler(const char * aScheme, nsIProtocolHandler **_retval); \
  nsresult GetProtocolFlags(const char * aScheme, uint32_t *_retval); \
  nsresult NewURI(const nsACString& aSpec, const char * aOriginCharset, nsIURI *aBaseURI, nsIURI **_retval); \
  nsresult NewFileURI(nsIFile *aFile, nsIURI **_retval); \
  nsresult CreateExposableURI(nsIURI *aURI, nsIURI **_retval); \
  nsresult NewChannelFromURI(nsIURI *aURI, nsINode *aLoadingNode, nsIPrincipal *aLoadingPrincipal, nsIPrincipal *aTriggeringPrincipal, uint32_t aSecurityFlags, nsContentPolicyType aContentPolicyType, nsIChannel **_retval); \
  nsresult NewChannelFromURIWithClientAndController(nsIURI *aURI, nsINode *aLoadingNode, nsIPrincipal *aLoadingPrincipal, nsIPrincipal *aTriggeringPrincipal, const mozilla::Maybe<mozilla::dom::ClientInfo> & aLoadingClientInfo, const mozilla::Maybe<mozilla::dom::ServiceWorkerDescriptor> & aController, uint32_t aSecurityFlags, nsContentPolicyType aContentPolicyType, uint32_t aSandboxFlags, nsIChannel **aResult); \
  nsresult NewChannelFromURIWithLoadInfo(nsIURI *aURI, nsILoadInfo *aLoadInfo, nsIChannel **_retval); \
  nsresult NewChannel(const nsACString& aSpec, const char * aOriginCharset, nsIURI *aBaseURI, nsINode *aLoadingNode, nsIPrincipal *aLoadingPrincipal, nsIPrincipal *aTriggeringPrincipal, uint32_t aSecurityFlags, nsContentPolicyType aContentPolicyType, nsIChannel **_retval); \
  nsresult GetOffline(bool *aOffline); \
  nsresult SetOffline(bool aOffline); \
  nsresult GetConnectivity(bool *aConnectivity); \
  nsresult AllowPort(int32_t aPort, const char * aScheme, bool *_retval); \
  nsresult ExtractScheme(const nsACString& urlString, nsACString& _retval); \
  nsresult HostnameIsLocalIPAddress(nsIURI *aURI, bool *_retval); \
  nsresult HostnameIsSharedIPAddress(nsIURI *aURI, bool *_retval); \
  nsresult GetManageOfflineStatus(bool *aManageOfflineStatus); \
  nsresult SetManageOfflineStatus(bool aManageOfflineStatus); \
  nsresult NewChannelFromURIWithProxyFlags(nsIURI *aURI, nsIURI *aProxyURI, uint32_t aProxyFlags, nsINode *aLoadingNode, nsIPrincipal *aLoadingPrincipal, nsIPrincipal *aTriggeringPrincipal, uint32_t aSecurityFlags, nsContentPolicyType aContentPolicyType, nsIChannel **_retval); \
  nsresult GetSocketProcessLaunched(bool *aSocketProcessLaunched); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIIOSERVICE(_to) \
  NS_IMETHOD GetProtocolHandler(const char * aScheme, nsIProtocolHandler **_retval) override { return _to GetProtocolHandler(aScheme, _retval); } \
  NS_IMETHOD GetProtocolFlags(const char * aScheme, uint32_t *_retval) override { return _to GetProtocolFlags(aScheme, _retval); } \
  NS_IMETHOD NewURI(const nsACString& aSpec, const char * aOriginCharset, nsIURI *aBaseURI, nsIURI **_retval) override { return _to NewURI(aSpec, aOriginCharset, aBaseURI, _retval); } \
  NS_IMETHOD NewFileURI(nsIFile *aFile, nsIURI **_retval) override { return _to NewFileURI(aFile, _retval); } \
  NS_IMETHOD CreateExposableURI(nsIURI *aURI, nsIURI **_retval) override { return _to CreateExposableURI(aURI, _retval); } \
  NS_IMETHOD NewChannelFromURI(nsIURI *aURI, nsINode *aLoadingNode, nsIPrincipal *aLoadingPrincipal, nsIPrincipal *aTriggeringPrincipal, uint32_t aSecurityFlags, nsContentPolicyType aContentPolicyType, nsIChannel **_retval) override { return _to NewChannelFromURI(aURI, aLoadingNode, aLoadingPrincipal, aTriggeringPrincipal, aSecurityFlags, aContentPolicyType, _retval); } \
  virtual nsresult NewChannelFromURIWithClientAndController(nsIURI *aURI, nsINode *aLoadingNode, nsIPrincipal *aLoadingPrincipal, nsIPrincipal *aTriggeringPrincipal, const mozilla::Maybe<mozilla::dom::ClientInfo> & aLoadingClientInfo, const mozilla::Maybe<mozilla::dom::ServiceWorkerDescriptor> & aController, uint32_t aSecurityFlags, nsContentPolicyType aContentPolicyType, uint32_t aSandboxFlags, nsIChannel **aResult) override { return _to NewChannelFromURIWithClientAndController(aURI, aLoadingNode, aLoadingPrincipal, aTriggeringPrincipal, aLoadingClientInfo, aController, aSecurityFlags, aContentPolicyType, aSandboxFlags, aResult); } \
  NS_IMETHOD NewChannelFromURIWithLoadInfo(nsIURI *aURI, nsILoadInfo *aLoadInfo, nsIChannel **_retval) override { return _to NewChannelFromURIWithLoadInfo(aURI, aLoadInfo, _retval); } \
  NS_IMETHOD NewChannel(const nsACString& aSpec, const char * aOriginCharset, nsIURI *aBaseURI, nsINode *aLoadingNode, nsIPrincipal *aLoadingPrincipal, nsIPrincipal *aTriggeringPrincipal, uint32_t aSecurityFlags, nsContentPolicyType aContentPolicyType, nsIChannel **_retval) override { return _to NewChannel(aSpec, aOriginCharset, aBaseURI, aLoadingNode, aLoadingPrincipal, aTriggeringPrincipal, aSecurityFlags, aContentPolicyType, _retval); } \
  NS_IMETHOD GetOffline(bool *aOffline) override { return _to GetOffline(aOffline); } \
  NS_IMETHOD SetOffline(bool aOffline) override { return _to SetOffline(aOffline); } \
  NS_IMETHOD GetConnectivity(bool *aConnectivity) override { return _to GetConnectivity(aConnectivity); } \
  NS_IMETHOD AllowPort(int32_t aPort, const char * aScheme, bool *_retval) override { return _to AllowPort(aPort, aScheme, _retval); } \
  NS_IMETHOD ExtractScheme(const nsACString& urlString, nsACString& _retval) override { return _to ExtractScheme(urlString, _retval); } \
  NS_IMETHOD HostnameIsLocalIPAddress(nsIURI *aURI, bool *_retval) override { return _to HostnameIsLocalIPAddress(aURI, _retval); } \
  NS_IMETHOD HostnameIsSharedIPAddress(nsIURI *aURI, bool *_retval) override { return _to HostnameIsSharedIPAddress(aURI, _retval); } \
  NS_IMETHOD GetManageOfflineStatus(bool *aManageOfflineStatus) override { return _to GetManageOfflineStatus(aManageOfflineStatus); } \
  NS_IMETHOD SetManageOfflineStatus(bool aManageOfflineStatus) override { return _to SetManageOfflineStatus(aManageOfflineStatus); } \
  NS_IMETHOD NewChannelFromURIWithProxyFlags(nsIURI *aURI, nsIURI *aProxyURI, uint32_t aProxyFlags, nsINode *aLoadingNode, nsIPrincipal *aLoadingPrincipal, nsIPrincipal *aTriggeringPrincipal, uint32_t aSecurityFlags, nsContentPolicyType aContentPolicyType, nsIChannel **_retval) override { return _to NewChannelFromURIWithProxyFlags(aURI, aProxyURI, aProxyFlags, aLoadingNode, aLoadingPrincipal, aTriggeringPrincipal, aSecurityFlags, aContentPolicyType, _retval); } \
  NS_IMETHOD GetSocketProcessLaunched(bool *aSocketProcessLaunched) override { return _to GetSocketProcessLaunched(aSocketProcessLaunched); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIIOSERVICE(_to) \
  NS_IMETHOD GetProtocolHandler(const char * aScheme, nsIProtocolHandler **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProtocolHandler(aScheme, _retval); } \
  NS_IMETHOD GetProtocolFlags(const char * aScheme, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProtocolFlags(aScheme, _retval); } \
  NS_IMETHOD NewURI(const nsACString& aSpec, const char * aOriginCharset, nsIURI *aBaseURI, nsIURI **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewURI(aSpec, aOriginCharset, aBaseURI, _retval); } \
  NS_IMETHOD NewFileURI(nsIFile *aFile, nsIURI **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewFileURI(aFile, _retval); } \
  NS_IMETHOD CreateExposableURI(nsIURI *aURI, nsIURI **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateExposableURI(aURI, _retval); } \
  NS_IMETHOD NewChannelFromURI(nsIURI *aURI, nsINode *aLoadingNode, nsIPrincipal *aLoadingPrincipal, nsIPrincipal *aTriggeringPrincipal, uint32_t aSecurityFlags, nsContentPolicyType aContentPolicyType, nsIChannel **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewChannelFromURI(aURI, aLoadingNode, aLoadingPrincipal, aTriggeringPrincipal, aSecurityFlags, aContentPolicyType, _retval); } \
  virtual nsresult NewChannelFromURIWithClientAndController(nsIURI *aURI, nsINode *aLoadingNode, nsIPrincipal *aLoadingPrincipal, nsIPrincipal *aTriggeringPrincipal, const mozilla::Maybe<mozilla::dom::ClientInfo> & aLoadingClientInfo, const mozilla::Maybe<mozilla::dom::ServiceWorkerDescriptor> & aController, uint32_t aSecurityFlags, nsContentPolicyType aContentPolicyType, uint32_t aSandboxFlags, nsIChannel **aResult) override; \
  NS_IMETHOD NewChannelFromURIWithLoadInfo(nsIURI *aURI, nsILoadInfo *aLoadInfo, nsIChannel **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewChannelFromURIWithLoadInfo(aURI, aLoadInfo, _retval); } \
  NS_IMETHOD NewChannel(const nsACString& aSpec, const char * aOriginCharset, nsIURI *aBaseURI, nsINode *aLoadingNode, nsIPrincipal *aLoadingPrincipal, nsIPrincipal *aTriggeringPrincipal, uint32_t aSecurityFlags, nsContentPolicyType aContentPolicyType, nsIChannel **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewChannel(aSpec, aOriginCharset, aBaseURI, aLoadingNode, aLoadingPrincipal, aTriggeringPrincipal, aSecurityFlags, aContentPolicyType, _retval); } \
  NS_IMETHOD GetOffline(bool *aOffline) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOffline(aOffline); } \
  NS_IMETHOD SetOffline(bool aOffline) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetOffline(aOffline); } \
  NS_IMETHOD GetConnectivity(bool *aConnectivity) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetConnectivity(aConnectivity); } \
  NS_IMETHOD AllowPort(int32_t aPort, const char * aScheme, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AllowPort(aPort, aScheme, _retval); } \
  NS_IMETHOD ExtractScheme(const nsACString& urlString, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ExtractScheme(urlString, _retval); } \
  NS_IMETHOD HostnameIsLocalIPAddress(nsIURI *aURI, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HostnameIsLocalIPAddress(aURI, _retval); } \
  NS_IMETHOD HostnameIsSharedIPAddress(nsIURI *aURI, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HostnameIsSharedIPAddress(aURI, _retval); } \
  NS_IMETHOD GetManageOfflineStatus(bool *aManageOfflineStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetManageOfflineStatus(aManageOfflineStatus); } \
  NS_IMETHOD SetManageOfflineStatus(bool aManageOfflineStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetManageOfflineStatus(aManageOfflineStatus); } \
  NS_IMETHOD NewChannelFromURIWithProxyFlags(nsIURI *aURI, nsIURI *aProxyURI, uint32_t aProxyFlags, nsINode *aLoadingNode, nsIPrincipal *aLoadingPrincipal, nsIPrincipal *aTriggeringPrincipal, uint32_t aSecurityFlags, nsContentPolicyType aContentPolicyType, nsIChannel **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewChannelFromURIWithProxyFlags(aURI, aProxyURI, aProxyFlags, aLoadingNode, aLoadingPrincipal, aTriggeringPrincipal, aSecurityFlags, aContentPolicyType, _retval); } \
  NS_IMETHOD GetSocketProcessLaunched(bool *aSocketProcessLaunched) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSocketProcessLaunched(aSocketProcessLaunched); } 

/**
 * We send notifications through nsIObserverService with topic
 * NS_IOSERVICE_GOING_OFFLINE_TOPIC and data NS_IOSERVICE_OFFLINE
 * when 'offline' has changed from false to true, and we are about
 * to shut down network services such as DNS. When those
 * services have been shut down, we send a notification with
 * topic NS_IOSERVICE_OFFLINE_STATUS_TOPIC and data
 * NS_IOSERVICE_OFFLINE.
 *
 * When 'offline' changes from true to false, then after
 * network services have been restarted, we send a notification
 * with topic NS_IOSERVICE_OFFLINE_STATUS_TOPIC and data
 * NS_IOSERVICE_ONLINE.
 */
#define NS_IOSERVICE_GOING_OFFLINE_TOPIC  "network:offline-about-to-go-offline"
#define NS_IOSERVICE_OFFLINE_STATUS_TOPIC "network:offline-status-changed"
#define NS_IOSERVICE_OFFLINE              "offline"
#define NS_IOSERVICE_ONLINE               "online"

/* starting interface:    nsIIOServiceInternal */
#define NS_IIOSERVICEINTERNAL_IID_STR "6633c0bf-d97a-428f-8ece-cb6a655fb95a"

#define NS_IIOSERVICEINTERNAL_IID \
  {0x6633c0bf, 0xd97a, 0x428f, \
    { 0x8e, 0xce, 0xcb, 0x6a, 0x65, 0x5f, 0xb9, 0x5a }}

class NS_NO_VTABLE nsIIOServiceInternal : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IIOSERVICEINTERNAL_IID)

  /* void SetConnectivity (in boolean connectivity); */
  NS_IMETHOD SetConnectivity(bool connectivity) = 0;

  /* void NotifyWakeup (); */
  NS_IMETHOD NotifyWakeup(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIIOServiceInternal, NS_IIOSERVICEINTERNAL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIIOSERVICEINTERNAL \
  NS_IMETHOD SetConnectivity(bool connectivity) override; \
  NS_IMETHOD NotifyWakeup(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIIOSERVICEINTERNAL \
  nsresult SetConnectivity(bool connectivity); \
  nsresult NotifyWakeup(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIIOSERVICEINTERNAL(_to) \
  NS_IMETHOD SetConnectivity(bool connectivity) override { return _to SetConnectivity(connectivity); } \
  NS_IMETHOD NotifyWakeup(void) override { return _to NotifyWakeup(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIIOSERVICEINTERNAL(_to) \
  NS_IMETHOD SetConnectivity(bool connectivity) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetConnectivity(connectivity); } \
  NS_IMETHOD NotifyWakeup(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyWakeup(); } 


#endif /* __gen_nsIIOService_h__ */
