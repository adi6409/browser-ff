/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/websocket/nsIWebSocketChannel.idl
 */

#ifndef __gen_nsIWebSocketChannel_h__
#define __gen_nsIWebSocketChannel_h__


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
class nsICookieJarSettings; /* forward declaration */

class nsIURI; /* forward declaration */

class nsIInterfaceRequestor; /* forward declaration */

class nsILoadGroup; /* forward declaration */

class nsIWebSocketListener; /* forward declaration */

class nsIInputStream; /* forward declaration */

class nsILoadInfo; /* forward declaration */

class nsIPrincipal; /* forward declaration */

class nsITransportProvider; /* forward declaration */

class nsINode; /* webidl Node */


/* starting interface:    nsIWebSocketChannel */
#define NS_IWEBSOCKETCHANNEL_IID_STR "ce71d028-322a-4105-a947-a894689b52bf"

#define NS_IWEBSOCKETCHANNEL_IID \
  {0xce71d028, 0x322a, 0x4105, \
    { 0xa9, 0x47, 0xa8, 0x94, 0x68, 0x9b, 0x52, 0xbf }}

class nsIWebSocketChannel : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWEBSOCKETCHANNEL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWebSocketChannel;

  /* [must_use] readonly attribute nsIURI originalURI; */
  [[nodiscard]] NS_IMETHOD GetOriginalURI(nsIURI **aOriginalURI) = 0;

  /* [must_use] readonly attribute nsIURI URI; */
  [[nodiscard]] NS_IMETHOD GetURI(nsIURI **aURI) = 0;

  /* [must_use] attribute nsIInterfaceRequestor notificationCallbacks; */
  [[nodiscard]] NS_IMETHOD GetNotificationCallbacks(nsIInterfaceRequestor **aNotificationCallbacks) = 0;
  [[nodiscard]] NS_IMETHOD SetNotificationCallbacks(nsIInterfaceRequestor *aNotificationCallbacks) = 0;

  /* [must_use] readonly attribute nsISupports securityInfo; */
  [[nodiscard]] NS_IMETHOD GetSecurityInfo(nsISupports **aSecurityInfo) = 0;

  /* [must_use] attribute nsILoadGroup loadGroup; */
  [[nodiscard]] NS_IMETHOD GetLoadGroup(nsILoadGroup **aLoadGroup) = 0;
  [[nodiscard]] NS_IMETHOD SetLoadGroup(nsILoadGroup *aLoadGroup) = 0;

  /* [must_use] attribute nsILoadInfo loadInfo; */
  [[nodiscard]] NS_IMETHOD GetLoadInfo(nsILoadInfo **aLoadInfo) = 0;
  [[nodiscard]] NS_IMETHOD SetLoadInfo(nsILoadInfo *aLoadInfo) = 0;

  /* [must_use] attribute ACString protocol; */
  [[nodiscard]] NS_IMETHOD GetProtocol(nsACString& aProtocol) = 0;
  [[nodiscard]] NS_IMETHOD SetProtocol(const nsACString& aProtocol) = 0;

  /* [must_use] readonly attribute ACString extensions; */
  [[nodiscard]] NS_IMETHOD GetExtensions(nsACString& aExtensions) = 0;

  /* [must_use] readonly attribute uint64_t httpChannelId; */
  [[nodiscard]] NS_IMETHOD GetHttpChannelId(uint64_t *aHttpChannelId) = 0;

  /* [notxpcom] nsresult initLoadInfoNative (in Node aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in nsICookieJarSettings aCookieJarSettings, in unsigned long aSecurityFlags, in nsContentPolicyType aContentPolicyType, in unsigned long aSandboxFlags); */
  NS_IMETHOD InitLoadInfoNative(nsINode *aLoadingNode, nsIPrincipal *aLoadingPrincipal, nsIPrincipal *aTriggeringPrincipal, nsICookieJarSettings *aCookieJarSettings, uint32_t aSecurityFlags, nsContentPolicyType aContentPolicyType, uint32_t aSandboxFlags) = 0;

  /* [must_use] void initLoadInfo (in Node aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in unsigned long aSecurityFlags, in nsContentPolicyType aContentPolicyType); */
  [[nodiscard]] NS_IMETHOD InitLoadInfo(nsINode *aLoadingNode, nsIPrincipal *aLoadingPrincipal, nsIPrincipal *aTriggeringPrincipal, uint32_t aSecurityFlags, nsContentPolicyType aContentPolicyType) = 0;

  /* [must_use] void asyncOpen (in nsIURI aURI, in ACString aOrigin, in unsigned long long aInnerWindowID, in nsIWebSocketListener aListener, in nsISupports aContext); */
  [[nodiscard]] NS_IMETHOD AsyncOpen(nsIURI *aURI, const nsACString& aOrigin, uint64_t aInnerWindowID, nsIWebSocketListener *aListener, nsISupports *aContext) = 0;

  /* [must_use] void close (in unsigned short aCode, in AUTF8String aReason); */
  [[nodiscard]] NS_IMETHOD Close(uint16_t aCode, const nsACString& aReason) = 0;

  enum {
    CLOSE_NORMAL = 1000U,
    CLOSE_GOING_AWAY = 1001U,
    CLOSE_PROTOCOL_ERROR = 1002U,
    CLOSE_UNSUPPORTED_DATATYPE = 1003U,
    CLOSE_NO_STATUS = 1005U,
    CLOSE_ABNORMAL = 1006U,
    CLOSE_INVALID_PAYLOAD = 1007U,
    CLOSE_POLICY_VIOLATION = 1008U,
    CLOSE_TOO_LARGE = 1009U,
    CLOSE_EXTENSION_MISSING = 1010U,
    CLOSE_INTERNAL_ERROR = 1011U,
    CLOSE_TLS_FAILED = 1015U
  };

  /* [must_use] void sendMsg (in AUTF8String aMsg); */
  [[nodiscard]] NS_IMETHOD SendMsg(const nsACString& aMsg) = 0;

  /* [must_use] void sendBinaryMsg (in ACString aMsg); */
  [[nodiscard]] NS_IMETHOD SendBinaryMsg(const nsACString& aMsg) = 0;

  /* [must_use] void sendBinaryStream (in nsIInputStream aStream, in unsigned long length); */
  [[nodiscard]] NS_IMETHOD SendBinaryStream(nsIInputStream *aStream, uint32_t length) = 0;

  /* [must_use] attribute unsigned long pingInterval; */
  [[nodiscard]] NS_IMETHOD GetPingInterval(uint32_t *aPingInterval) = 0;
  [[nodiscard]] NS_IMETHOD SetPingInterval(uint32_t aPingInterval) = 0;

  /* [must_use] attribute unsigned long pingTimeout; */
  [[nodiscard]] NS_IMETHOD GetPingTimeout(uint32_t *aPingTimeout) = 0;
  [[nodiscard]] NS_IMETHOD SetPingTimeout(uint32_t aPingTimeout) = 0;

  /* [must_use] attribute unsigned long serial; */
  [[nodiscard]] NS_IMETHOD GetSerial(uint32_t *aSerial) = 0;
  [[nodiscard]] NS_IMETHOD SetSerial(uint32_t aSerial) = 0;

  /* [must_use] void setServerParameters (in nsITransportProvider aProvider, in ACString aNegotiatedExtensions); */
  [[nodiscard]] NS_IMETHOD SetServerParameters(nsITransportProvider *aProvider, const nsACString& aNegotiatedExtensions) = 0;

     inline uint32_t Serial()
    {
      uint32_t serial;
      nsresult rv = GetSerial(&serial);
      if (NS_WARN_IF(NS_FAILED(rv))) {
        return 0;
      }
      return serial;
    }
    inline uint64_t HttpChannelId()
    {
      uint64_t httpChannelId;
      nsresult rv = GetHttpChannelId(&httpChannelId);
      if (NS_WARN_IF(NS_FAILED(rv))) {
        return 0;
      }
      return httpChannelId;
    }
};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWebSocketChannel, NS_IWEBSOCKETCHANNEL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWEBSOCKETCHANNEL \
  [[nodiscard]] NS_IMETHOD GetOriginalURI(nsIURI **aOriginalURI) override; \
  [[nodiscard]] NS_IMETHOD GetURI(nsIURI **aURI) override; \
  [[nodiscard]] NS_IMETHOD GetNotificationCallbacks(nsIInterfaceRequestor **aNotificationCallbacks) override; \
  [[nodiscard]] NS_IMETHOD SetNotificationCallbacks(nsIInterfaceRequestor *aNotificationCallbacks) override; \
  [[nodiscard]] NS_IMETHOD GetSecurityInfo(nsISupports **aSecurityInfo) override; \
  [[nodiscard]] NS_IMETHOD GetLoadGroup(nsILoadGroup **aLoadGroup) override; \
  [[nodiscard]] NS_IMETHOD SetLoadGroup(nsILoadGroup *aLoadGroup) override; \
  [[nodiscard]] NS_IMETHOD GetLoadInfo(nsILoadInfo **aLoadInfo) override; \
  [[nodiscard]] NS_IMETHOD SetLoadInfo(nsILoadInfo *aLoadInfo) override; \
  [[nodiscard]] NS_IMETHOD GetProtocol(nsACString& aProtocol) override; \
  [[nodiscard]] NS_IMETHOD SetProtocol(const nsACString& aProtocol) override; \
  [[nodiscard]] NS_IMETHOD GetExtensions(nsACString& aExtensions) override; \
  [[nodiscard]] NS_IMETHOD GetHttpChannelId(uint64_t *aHttpChannelId) override; \
  NS_IMETHOD InitLoadInfoNative(nsINode *aLoadingNode, nsIPrincipal *aLoadingPrincipal, nsIPrincipal *aTriggeringPrincipal, nsICookieJarSettings *aCookieJarSettings, uint32_t aSecurityFlags, nsContentPolicyType aContentPolicyType, uint32_t aSandboxFlags) override; \
  [[nodiscard]] NS_IMETHOD InitLoadInfo(nsINode *aLoadingNode, nsIPrincipal *aLoadingPrincipal, nsIPrincipal *aTriggeringPrincipal, uint32_t aSecurityFlags, nsContentPolicyType aContentPolicyType) override; \
  [[nodiscard]] NS_IMETHOD AsyncOpen(nsIURI *aURI, const nsACString& aOrigin, uint64_t aInnerWindowID, nsIWebSocketListener *aListener, nsISupports *aContext) override; \
  [[nodiscard]] NS_IMETHOD Close(uint16_t aCode, const nsACString& aReason) override; \
  [[nodiscard]] NS_IMETHOD SendMsg(const nsACString& aMsg) override; \
  [[nodiscard]] NS_IMETHOD SendBinaryMsg(const nsACString& aMsg) override; \
  [[nodiscard]] NS_IMETHOD SendBinaryStream(nsIInputStream *aStream, uint32_t length) override; \
  [[nodiscard]] NS_IMETHOD GetPingInterval(uint32_t *aPingInterval) override; \
  [[nodiscard]] NS_IMETHOD SetPingInterval(uint32_t aPingInterval) override; \
  [[nodiscard]] NS_IMETHOD GetPingTimeout(uint32_t *aPingTimeout) override; \
  [[nodiscard]] NS_IMETHOD SetPingTimeout(uint32_t aPingTimeout) override; \
  [[nodiscard]] NS_IMETHOD GetSerial(uint32_t *aSerial) override; \
  [[nodiscard]] NS_IMETHOD SetSerial(uint32_t aSerial) override; \
  [[nodiscard]] NS_IMETHOD SetServerParameters(nsITransportProvider *aProvider, const nsACString& aNegotiatedExtensions) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWEBSOCKETCHANNEL \
  [[nodiscard]] nsresult GetOriginalURI(nsIURI **aOriginalURI); \
  [[nodiscard]] nsresult GetURI(nsIURI **aURI); \
  [[nodiscard]] nsresult GetNotificationCallbacks(nsIInterfaceRequestor **aNotificationCallbacks); \
  [[nodiscard]] nsresult SetNotificationCallbacks(nsIInterfaceRequestor *aNotificationCallbacks); \
  [[nodiscard]] nsresult GetSecurityInfo(nsISupports **aSecurityInfo); \
  [[nodiscard]] nsresult GetLoadGroup(nsILoadGroup **aLoadGroup); \
  [[nodiscard]] nsresult SetLoadGroup(nsILoadGroup *aLoadGroup); \
  [[nodiscard]] nsresult GetLoadInfo(nsILoadInfo **aLoadInfo); \
  [[nodiscard]] nsresult SetLoadInfo(nsILoadInfo *aLoadInfo); \
  [[nodiscard]] nsresult GetProtocol(nsACString& aProtocol); \
  [[nodiscard]] nsresult SetProtocol(const nsACString& aProtocol); \
  [[nodiscard]] nsresult GetExtensions(nsACString& aExtensions); \
  [[nodiscard]] nsresult GetHttpChannelId(uint64_t *aHttpChannelId); \
  nsresult InitLoadInfoNative(nsINode *aLoadingNode, nsIPrincipal *aLoadingPrincipal, nsIPrincipal *aTriggeringPrincipal, nsICookieJarSettings *aCookieJarSettings, uint32_t aSecurityFlags, nsContentPolicyType aContentPolicyType, uint32_t aSandboxFlags); \
  [[nodiscard]] nsresult InitLoadInfo(nsINode *aLoadingNode, nsIPrincipal *aLoadingPrincipal, nsIPrincipal *aTriggeringPrincipal, uint32_t aSecurityFlags, nsContentPolicyType aContentPolicyType); \
  [[nodiscard]] nsresult AsyncOpen(nsIURI *aURI, const nsACString& aOrigin, uint64_t aInnerWindowID, nsIWebSocketListener *aListener, nsISupports *aContext); \
  [[nodiscard]] nsresult Close(uint16_t aCode, const nsACString& aReason); \
  [[nodiscard]] nsresult SendMsg(const nsACString& aMsg); \
  [[nodiscard]] nsresult SendBinaryMsg(const nsACString& aMsg); \
  [[nodiscard]] nsresult SendBinaryStream(nsIInputStream *aStream, uint32_t length); \
  [[nodiscard]] nsresult GetPingInterval(uint32_t *aPingInterval); \
  [[nodiscard]] nsresult SetPingInterval(uint32_t aPingInterval); \
  [[nodiscard]] nsresult GetPingTimeout(uint32_t *aPingTimeout); \
  [[nodiscard]] nsresult SetPingTimeout(uint32_t aPingTimeout); \
  [[nodiscard]] nsresult GetSerial(uint32_t *aSerial); \
  [[nodiscard]] nsresult SetSerial(uint32_t aSerial); \
  [[nodiscard]] nsresult SetServerParameters(nsITransportProvider *aProvider, const nsACString& aNegotiatedExtensions); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWEBSOCKETCHANNEL(_to) \
  [[nodiscard]] NS_IMETHOD GetOriginalURI(nsIURI **aOriginalURI) override { return _to GetOriginalURI(aOriginalURI); } \
  [[nodiscard]] NS_IMETHOD GetURI(nsIURI **aURI) override { return _to GetURI(aURI); } \
  [[nodiscard]] NS_IMETHOD GetNotificationCallbacks(nsIInterfaceRequestor **aNotificationCallbacks) override { return _to GetNotificationCallbacks(aNotificationCallbacks); } \
  [[nodiscard]] NS_IMETHOD SetNotificationCallbacks(nsIInterfaceRequestor *aNotificationCallbacks) override { return _to SetNotificationCallbacks(aNotificationCallbacks); } \
  [[nodiscard]] NS_IMETHOD GetSecurityInfo(nsISupports **aSecurityInfo) override { return _to GetSecurityInfo(aSecurityInfo); } \
  [[nodiscard]] NS_IMETHOD GetLoadGroup(nsILoadGroup **aLoadGroup) override { return _to GetLoadGroup(aLoadGroup); } \
  [[nodiscard]] NS_IMETHOD SetLoadGroup(nsILoadGroup *aLoadGroup) override { return _to SetLoadGroup(aLoadGroup); } \
  [[nodiscard]] NS_IMETHOD GetLoadInfo(nsILoadInfo **aLoadInfo) override { return _to GetLoadInfo(aLoadInfo); } \
  [[nodiscard]] NS_IMETHOD SetLoadInfo(nsILoadInfo *aLoadInfo) override { return _to SetLoadInfo(aLoadInfo); } \
  [[nodiscard]] NS_IMETHOD GetProtocol(nsACString& aProtocol) override { return _to GetProtocol(aProtocol); } \
  [[nodiscard]] NS_IMETHOD SetProtocol(const nsACString& aProtocol) override { return _to SetProtocol(aProtocol); } \
  [[nodiscard]] NS_IMETHOD GetExtensions(nsACString& aExtensions) override { return _to GetExtensions(aExtensions); } \
  [[nodiscard]] NS_IMETHOD GetHttpChannelId(uint64_t *aHttpChannelId) override { return _to GetHttpChannelId(aHttpChannelId); } \
  NS_IMETHOD InitLoadInfoNative(nsINode *aLoadingNode, nsIPrincipal *aLoadingPrincipal, nsIPrincipal *aTriggeringPrincipal, nsICookieJarSettings *aCookieJarSettings, uint32_t aSecurityFlags, nsContentPolicyType aContentPolicyType, uint32_t aSandboxFlags) override { return _to InitLoadInfoNative(aLoadingNode, aLoadingPrincipal, aTriggeringPrincipal, aCookieJarSettings, aSecurityFlags, aContentPolicyType, aSandboxFlags); } \
  [[nodiscard]] NS_IMETHOD InitLoadInfo(nsINode *aLoadingNode, nsIPrincipal *aLoadingPrincipal, nsIPrincipal *aTriggeringPrincipal, uint32_t aSecurityFlags, nsContentPolicyType aContentPolicyType) override { return _to InitLoadInfo(aLoadingNode, aLoadingPrincipal, aTriggeringPrincipal, aSecurityFlags, aContentPolicyType); } \
  [[nodiscard]] NS_IMETHOD AsyncOpen(nsIURI *aURI, const nsACString& aOrigin, uint64_t aInnerWindowID, nsIWebSocketListener *aListener, nsISupports *aContext) override { return _to AsyncOpen(aURI, aOrigin, aInnerWindowID, aListener, aContext); } \
  [[nodiscard]] NS_IMETHOD Close(uint16_t aCode, const nsACString& aReason) override { return _to Close(aCode, aReason); } \
  [[nodiscard]] NS_IMETHOD SendMsg(const nsACString& aMsg) override { return _to SendMsg(aMsg); } \
  [[nodiscard]] NS_IMETHOD SendBinaryMsg(const nsACString& aMsg) override { return _to SendBinaryMsg(aMsg); } \
  [[nodiscard]] NS_IMETHOD SendBinaryStream(nsIInputStream *aStream, uint32_t length) override { return _to SendBinaryStream(aStream, length); } \
  [[nodiscard]] NS_IMETHOD GetPingInterval(uint32_t *aPingInterval) override { return _to GetPingInterval(aPingInterval); } \
  [[nodiscard]] NS_IMETHOD SetPingInterval(uint32_t aPingInterval) override { return _to SetPingInterval(aPingInterval); } \
  [[nodiscard]] NS_IMETHOD GetPingTimeout(uint32_t *aPingTimeout) override { return _to GetPingTimeout(aPingTimeout); } \
  [[nodiscard]] NS_IMETHOD SetPingTimeout(uint32_t aPingTimeout) override { return _to SetPingTimeout(aPingTimeout); } \
  [[nodiscard]] NS_IMETHOD GetSerial(uint32_t *aSerial) override { return _to GetSerial(aSerial); } \
  [[nodiscard]] NS_IMETHOD SetSerial(uint32_t aSerial) override { return _to SetSerial(aSerial); } \
  [[nodiscard]] NS_IMETHOD SetServerParameters(nsITransportProvider *aProvider, const nsACString& aNegotiatedExtensions) override { return _to SetServerParameters(aProvider, aNegotiatedExtensions); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWEBSOCKETCHANNEL(_to) \
  [[nodiscard]] NS_IMETHOD GetOriginalURI(nsIURI **aOriginalURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOriginalURI(aOriginalURI); } \
  [[nodiscard]] NS_IMETHOD GetURI(nsIURI **aURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetURI(aURI); } \
  [[nodiscard]] NS_IMETHOD GetNotificationCallbacks(nsIInterfaceRequestor **aNotificationCallbacks) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNotificationCallbacks(aNotificationCallbacks); } \
  [[nodiscard]] NS_IMETHOD SetNotificationCallbacks(nsIInterfaceRequestor *aNotificationCallbacks) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetNotificationCallbacks(aNotificationCallbacks); } \
  [[nodiscard]] NS_IMETHOD GetSecurityInfo(nsISupports **aSecurityInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSecurityInfo(aSecurityInfo); } \
  [[nodiscard]] NS_IMETHOD GetLoadGroup(nsILoadGroup **aLoadGroup) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoadGroup(aLoadGroup); } \
  [[nodiscard]] NS_IMETHOD SetLoadGroup(nsILoadGroup *aLoadGroup) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLoadGroup(aLoadGroup); } \
  [[nodiscard]] NS_IMETHOD GetLoadInfo(nsILoadInfo **aLoadInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoadInfo(aLoadInfo); } \
  [[nodiscard]] NS_IMETHOD SetLoadInfo(nsILoadInfo *aLoadInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLoadInfo(aLoadInfo); } \
  [[nodiscard]] NS_IMETHOD GetProtocol(nsACString& aProtocol) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProtocol(aProtocol); } \
  [[nodiscard]] NS_IMETHOD SetProtocol(const nsACString& aProtocol) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetProtocol(aProtocol); } \
  [[nodiscard]] NS_IMETHOD GetExtensions(nsACString& aExtensions) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetExtensions(aExtensions); } \
  [[nodiscard]] NS_IMETHOD GetHttpChannelId(uint64_t *aHttpChannelId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHttpChannelId(aHttpChannelId); } \
  NS_IMETHOD InitLoadInfoNative(nsINode *aLoadingNode, nsIPrincipal *aLoadingPrincipal, nsIPrincipal *aTriggeringPrincipal, nsICookieJarSettings *aCookieJarSettings, uint32_t aSecurityFlags, nsContentPolicyType aContentPolicyType, uint32_t aSandboxFlags) override; \
  [[nodiscard]] NS_IMETHOD InitLoadInfo(nsINode *aLoadingNode, nsIPrincipal *aLoadingPrincipal, nsIPrincipal *aTriggeringPrincipal, uint32_t aSecurityFlags, nsContentPolicyType aContentPolicyType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitLoadInfo(aLoadingNode, aLoadingPrincipal, aTriggeringPrincipal, aSecurityFlags, aContentPolicyType); } \
  [[nodiscard]] NS_IMETHOD AsyncOpen(nsIURI *aURI, const nsACString& aOrigin, uint64_t aInnerWindowID, nsIWebSocketListener *aListener, nsISupports *aContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncOpen(aURI, aOrigin, aInnerWindowID, aListener, aContext); } \
  [[nodiscard]] NS_IMETHOD Close(uint16_t aCode, const nsACString& aReason) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Close(aCode, aReason); } \
  [[nodiscard]] NS_IMETHOD SendMsg(const nsACString& aMsg) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendMsg(aMsg); } \
  [[nodiscard]] NS_IMETHOD SendBinaryMsg(const nsACString& aMsg) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendBinaryMsg(aMsg); } \
  [[nodiscard]] NS_IMETHOD SendBinaryStream(nsIInputStream *aStream, uint32_t length) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendBinaryStream(aStream, length); } \
  [[nodiscard]] NS_IMETHOD GetPingInterval(uint32_t *aPingInterval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPingInterval(aPingInterval); } \
  [[nodiscard]] NS_IMETHOD SetPingInterval(uint32_t aPingInterval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPingInterval(aPingInterval); } \
  [[nodiscard]] NS_IMETHOD GetPingTimeout(uint32_t *aPingTimeout) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPingTimeout(aPingTimeout); } \
  [[nodiscard]] NS_IMETHOD SetPingTimeout(uint32_t aPingTimeout) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPingTimeout(aPingTimeout); } \
  [[nodiscard]] NS_IMETHOD GetSerial(uint32_t *aSerial) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSerial(aSerial); } \
  [[nodiscard]] NS_IMETHOD SetSerial(uint32_t aSerial) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSerial(aSerial); } \
  [[nodiscard]] NS_IMETHOD SetServerParameters(nsITransportProvider *aProvider, const nsACString& aNegotiatedExtensions) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetServerParameters(aProvider, aNegotiatedExtensions); } \


#endif /* __gen_nsIWebSocketChannel_h__ */
