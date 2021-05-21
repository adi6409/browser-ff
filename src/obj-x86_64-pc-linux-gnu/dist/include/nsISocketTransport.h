/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsISocketTransport.idl
 */

#ifndef __gen_nsISocketTransport_h__
#define __gen_nsISocketTransport_h__


#ifndef __gen_nsITransport_h__
#include "nsITransport.h"
#endif

#ifndef __gen_nsIRequest_h__
#include "nsIRequest.h"
#endif

#include "js/Value.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIInterfaceRequestor; /* forward declaration */

class nsINetAddr; /* forward declaration */

#include "mozilla/BasePrincipal.h"
namespace mozilla {
namespace net {
union NetAddr;
}
}

/* starting interface:    nsISocketTransport */
#define NS_ISOCKETTRANSPORT_IID_STR "79221831-85e2-43a8-8152-05d77d6fde31"

#define NS_ISOCKETTRANSPORT_IID \
  {0x79221831, 0x85e2, 0x43a8, \
    { 0x81, 0x52, 0x05, 0xd7, 0x7d, 0x6f, 0xde, 0x31 }}

class nsISocketTransport : public nsITransport {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISOCKETTRANSPORT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISocketTransport;

  /* readonly attribute AUTF8String host; */
  NS_IMETHOD GetHost(nsACString& aHost) = 0;

  /* readonly attribute long port; */
  NS_IMETHOD GetPort(int32_t *aPort) = 0;

  /* [binaryname(ScriptableOriginAttributes),implicit_jscontext] attribute jsval originAttributes; */
  NS_IMETHOD GetScriptableOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) = 0;
  NS_IMETHOD SetScriptableOriginAttributes(JSContext* cx, JS::HandleValue aOriginAttributes) = 0;

  /* [binaryname(GetOriginAttributes),noscript,nostdcall] OriginAttributes binaryGetOriginAttributes (); */
  virtual nsresult GetOriginAttributes(mozilla::OriginAttributes * _retval) = 0;

  /* [binaryname(SetOriginAttributes),noscript,nostdcall] void binarySetOriginAttributes (in const_OriginAttributesRef aOriginAttrs); */
  virtual nsresult SetOriginAttributes(const mozilla::OriginAttributes & aOriginAttrs) = 0;

  /* [noscript] NetAddr getPeerAddr (); */
  NS_IMETHOD GetPeerAddr(mozilla::net::NetAddr * _retval) = 0;

  /* [noscript] NetAddr getSelfAddr (); */
  NS_IMETHOD GetSelfAddr(mozilla::net::NetAddr * _retval) = 0;

  /* [noscript] void bind (in NetAddrPtr aLocalAddr); */
  NS_IMETHOD Bind(mozilla::net::NetAddr * aLocalAddr) = 0;

  /* nsINetAddr getScriptablePeerAddr (); */
  NS_IMETHOD GetScriptablePeerAddr(nsINetAddr **_retval) = 0;

  /* nsINetAddr getScriptableSelfAddr (); */
  NS_IMETHOD GetScriptableSelfAddr(nsINetAddr **_retval) = 0;

  /* readonly attribute nsISupports securityInfo; */
  NS_IMETHOD GetSecurityInfo(nsISupports **aSecurityInfo) = 0;

  /* attribute nsIInterfaceRequestor securityCallbacks; */
  NS_IMETHOD GetSecurityCallbacks(nsIInterfaceRequestor **aSecurityCallbacks) = 0;
  NS_IMETHOD SetSecurityCallbacks(nsIInterfaceRequestor *aSecurityCallbacks) = 0;

  /* boolean isAlive (); */
  NS_IMETHOD IsAlive(bool *_retval) = 0;

  /* unsigned long getTimeout (in unsigned long aType); */
  NS_IMETHOD GetTimeout(uint32_t aType, uint32_t *_retval) = 0;

  /* void setTimeout (in unsigned long aType, in unsigned long aValue); */
  NS_IMETHOD SetTimeout(uint32_t aType, uint32_t aValue) = 0;

  /* void setLinger (in boolean aPolarity, in short aTimeout); */
  NS_IMETHOD SetLinger(bool aPolarity, int16_t aTimeout) = 0;

  /* void setReuseAddrPort (in bool reuseAddrPort); */
  NS_IMETHOD SetReuseAddrPort(bool reuseAddrPort) = 0;

  enum {
    TIMEOUT_CONNECT = 0U,
    TIMEOUT_READ_WRITE = 1U,
    STATUS_RESOLVING = 2152398851U,
    STATUS_RESOLVED = 2152398859U,
    STATUS_CONNECTING_TO = 2152398855U,
    STATUS_CONNECTED_TO = 2152398852U,
    STATUS_SENDING_TO = 2152398853U,
    STATUS_WAITING_FOR = 2152398858U,
    STATUS_RECEIVING_FROM = 2152398854U,
    STATUS_TLS_HANDSHAKE_STARTING = 2152398860U,
    STATUS_TLS_HANDSHAKE_ENDED = 2152398861U
  };

  /* attribute unsigned long connectionFlags; */
  NS_IMETHOD GetConnectionFlags(uint32_t *aConnectionFlags) = 0;
  NS_IMETHOD SetConnectionFlags(uint32_t aConnectionFlags) = 0;

  enum {
    BYPASS_CACHE = 1U,
    ANONYMOUS_CONNECT = 2U,
    DISABLE_IPV6 = 4U,
    NO_PERMANENT_STORAGE = 8U,
    DISABLE_IPV4 = 16U,
    DISABLE_RFC1918 = 32U,
    BE_CONSERVATIVE = 64U,
    DISABLE_TRR = 128U,
    REFRESH_CACHE = 256U,
    RETRY_WITH_DIFFERENT_IP_FAMILY = 512U,
    DONT_TRY_ECH = 1024U,
    TRR_MODE_FLAGS = 6144U
  };

 
    static uint32_t GetFlagsFromTRRMode(nsIRequest::TRRMode aMode) {
        return static_cast<uint32_t>(aMode) << 11;
    }
    static nsIRequest::TRRMode GetTRRModeFromFlags(uint32_t aFlags) {
        return static_cast<nsIRequest::TRRMode>((aFlags & TRR_MODE_FLAGS) >> 11);
    }
  enum {
    USE_IP_HINT_ADDRESS = 8192U,
    ANONYMOUS_CONNECT_ALLOW_CLIENT_CERT = 16384
  };

  /* attribute unsigned long tlsFlags; */
  NS_IMETHOD GetTlsFlags(uint32_t *aTlsFlags) = 0;
  NS_IMETHOD SetTlsFlags(uint32_t aTlsFlags) = 0;

  /* attribute octet QoSBits; */
  NS_IMETHOD GetQoSBits(uint8_t *aQoSBits) = 0;
  NS_IMETHOD SetQoSBits(uint8_t aQoSBits) = 0;

  /* attribute unsigned long recvBufferSize; */
  NS_IMETHOD GetRecvBufferSize(uint32_t *aRecvBufferSize) = 0;
  NS_IMETHOD SetRecvBufferSize(uint32_t aRecvBufferSize) = 0;

  /* attribute unsigned long sendBufferSize; */
  NS_IMETHOD GetSendBufferSize(uint32_t *aSendBufferSize) = 0;
  NS_IMETHOD SetSendBufferSize(uint32_t aSendBufferSize) = 0;

  /* attribute boolean keepaliveEnabled; */
  NS_IMETHOD GetKeepaliveEnabled(bool *aKeepaliveEnabled) = 0;
  NS_IMETHOD SetKeepaliveEnabled(bool aKeepaliveEnabled) = 0;

  /* void setKeepaliveVals (in long keepaliveIdleTime, in long keepaliveRetryInterval); */
  NS_IMETHOD SetKeepaliveVals(int32_t keepaliveIdleTime, int32_t keepaliveRetryInterval) = 0;

  /* readonly attribute boolean resetIPFamilyPreference; */
  NS_IMETHOD GetResetIPFamilyPreference(bool *aResetIPFamilyPreference) = 0;

  /* readonly attribute boolean echConfigUsed; */
  NS_IMETHOD GetEchConfigUsed(bool *aEchConfigUsed) = 0;

  /* void setEchConfig (in ACString echConfig); */
  NS_IMETHOD SetEchConfig(const nsACString& echConfig) = 0;

  /* bool resolvedByTRR (); */
  NS_IMETHOD ResolvedByTRR(bool *_retval) = 0;

  /* [noscript] void setIsPrivate (in boolean isPrivate); */
  NS_IMETHOD SetIsPrivate(bool isPrivate) = 0;

  /* readonly attribute boolean retryDnsIfPossible; */
  NS_IMETHOD GetRetryDnsIfPossible(bool *aRetryDnsIfPossible) = 0;

  /* [noscript] readonly attribute nsresult status; */
  NS_IMETHOD GetStatus(nsresult *aStatus) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISocketTransport, NS_ISOCKETTRANSPORT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISOCKETTRANSPORT \
  NS_IMETHOD GetHost(nsACString& aHost) override; \
  NS_IMETHOD GetPort(int32_t *aPort) override; \
  NS_IMETHOD GetScriptableOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) override; \
  NS_IMETHOD SetScriptableOriginAttributes(JSContext* cx, JS::HandleValue aOriginAttributes) override; \
  virtual nsresult GetOriginAttributes(mozilla::OriginAttributes * _retval) override; \
  virtual nsresult SetOriginAttributes(const mozilla::OriginAttributes & aOriginAttrs) override; \
  NS_IMETHOD GetPeerAddr(mozilla::net::NetAddr * _retval) override; \
  NS_IMETHOD GetSelfAddr(mozilla::net::NetAddr * _retval) override; \
  NS_IMETHOD Bind(mozilla::net::NetAddr * aLocalAddr) override; \
  NS_IMETHOD GetScriptablePeerAddr(nsINetAddr **_retval) override; \
  NS_IMETHOD GetScriptableSelfAddr(nsINetAddr **_retval) override; \
  NS_IMETHOD GetSecurityInfo(nsISupports **aSecurityInfo) override; \
  NS_IMETHOD GetSecurityCallbacks(nsIInterfaceRequestor **aSecurityCallbacks) override; \
  NS_IMETHOD SetSecurityCallbacks(nsIInterfaceRequestor *aSecurityCallbacks) override; \
  NS_IMETHOD IsAlive(bool *_retval) override; \
  NS_IMETHOD GetTimeout(uint32_t aType, uint32_t *_retval) override; \
  NS_IMETHOD SetTimeout(uint32_t aType, uint32_t aValue) override; \
  NS_IMETHOD SetLinger(bool aPolarity, int16_t aTimeout) override; \
  NS_IMETHOD SetReuseAddrPort(bool reuseAddrPort) override; \
  NS_IMETHOD GetConnectionFlags(uint32_t *aConnectionFlags) override; \
  NS_IMETHOD SetConnectionFlags(uint32_t aConnectionFlags) override; \
  NS_IMETHOD GetTlsFlags(uint32_t *aTlsFlags) override; \
  NS_IMETHOD SetTlsFlags(uint32_t aTlsFlags) override; \
  NS_IMETHOD GetQoSBits(uint8_t *aQoSBits) override; \
  NS_IMETHOD SetQoSBits(uint8_t aQoSBits) override; \
  NS_IMETHOD GetRecvBufferSize(uint32_t *aRecvBufferSize) override; \
  NS_IMETHOD SetRecvBufferSize(uint32_t aRecvBufferSize) override; \
  NS_IMETHOD GetSendBufferSize(uint32_t *aSendBufferSize) override; \
  NS_IMETHOD SetSendBufferSize(uint32_t aSendBufferSize) override; \
  NS_IMETHOD GetKeepaliveEnabled(bool *aKeepaliveEnabled) override; \
  NS_IMETHOD SetKeepaliveEnabled(bool aKeepaliveEnabled) override; \
  NS_IMETHOD SetKeepaliveVals(int32_t keepaliveIdleTime, int32_t keepaliveRetryInterval) override; \
  NS_IMETHOD GetResetIPFamilyPreference(bool *aResetIPFamilyPreference) override; \
  NS_IMETHOD GetEchConfigUsed(bool *aEchConfigUsed) override; \
  NS_IMETHOD SetEchConfig(const nsACString& echConfig) override; \
  NS_IMETHOD ResolvedByTRR(bool *_retval) override; \
  NS_IMETHOD SetIsPrivate(bool isPrivate) override; \
  NS_IMETHOD GetRetryDnsIfPossible(bool *aRetryDnsIfPossible) override; \
  NS_IMETHOD GetStatus(nsresult *aStatus) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISOCKETTRANSPORT \
  nsresult GetHost(nsACString& aHost); \
  nsresult GetPort(int32_t *aPort); \
  nsresult GetScriptableOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes); \
  nsresult SetScriptableOriginAttributes(JSContext* cx, JS::HandleValue aOriginAttributes); \
  nsresult GetOriginAttributes(mozilla::OriginAttributes * _retval); \
  nsresult SetOriginAttributes(const mozilla::OriginAttributes & aOriginAttrs); \
  nsresult GetPeerAddr(mozilla::net::NetAddr * _retval); \
  nsresult GetSelfAddr(mozilla::net::NetAddr * _retval); \
  nsresult Bind(mozilla::net::NetAddr * aLocalAddr); \
  nsresult GetScriptablePeerAddr(nsINetAddr **_retval); \
  nsresult GetScriptableSelfAddr(nsINetAddr **_retval); \
  nsresult GetSecurityInfo(nsISupports **aSecurityInfo); \
  nsresult GetSecurityCallbacks(nsIInterfaceRequestor **aSecurityCallbacks); \
  nsresult SetSecurityCallbacks(nsIInterfaceRequestor *aSecurityCallbacks); \
  nsresult IsAlive(bool *_retval); \
  nsresult GetTimeout(uint32_t aType, uint32_t *_retval); \
  nsresult SetTimeout(uint32_t aType, uint32_t aValue); \
  nsresult SetLinger(bool aPolarity, int16_t aTimeout); \
  nsresult SetReuseAddrPort(bool reuseAddrPort); \
  nsresult GetConnectionFlags(uint32_t *aConnectionFlags); \
  nsresult SetConnectionFlags(uint32_t aConnectionFlags); \
  nsresult GetTlsFlags(uint32_t *aTlsFlags); \
  nsresult SetTlsFlags(uint32_t aTlsFlags); \
  nsresult GetQoSBits(uint8_t *aQoSBits); \
  nsresult SetQoSBits(uint8_t aQoSBits); \
  nsresult GetRecvBufferSize(uint32_t *aRecvBufferSize); \
  nsresult SetRecvBufferSize(uint32_t aRecvBufferSize); \
  nsresult GetSendBufferSize(uint32_t *aSendBufferSize); \
  nsresult SetSendBufferSize(uint32_t aSendBufferSize); \
  nsresult GetKeepaliveEnabled(bool *aKeepaliveEnabled); \
  nsresult SetKeepaliveEnabled(bool aKeepaliveEnabled); \
  nsresult SetKeepaliveVals(int32_t keepaliveIdleTime, int32_t keepaliveRetryInterval); \
  nsresult GetResetIPFamilyPreference(bool *aResetIPFamilyPreference); \
  nsresult GetEchConfigUsed(bool *aEchConfigUsed); \
  nsresult SetEchConfig(const nsACString& echConfig); \
  nsresult ResolvedByTRR(bool *_retval); \
  nsresult SetIsPrivate(bool isPrivate); \
  nsresult GetRetryDnsIfPossible(bool *aRetryDnsIfPossible); \
  nsresult GetStatus(nsresult *aStatus); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISOCKETTRANSPORT(_to) \
  NS_IMETHOD GetHost(nsACString& aHost) override { return _to GetHost(aHost); } \
  NS_IMETHOD GetPort(int32_t *aPort) override { return _to GetPort(aPort); } \
  NS_IMETHOD GetScriptableOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) override { return _to GetScriptableOriginAttributes(cx, aOriginAttributes); } \
  NS_IMETHOD SetScriptableOriginAttributes(JSContext* cx, JS::HandleValue aOriginAttributes) override { return _to SetScriptableOriginAttributes(cx, aOriginAttributes); } \
  virtual nsresult GetOriginAttributes(mozilla::OriginAttributes * _retval) override { return _to GetOriginAttributes(_retval); } \
  virtual nsresult SetOriginAttributes(const mozilla::OriginAttributes & aOriginAttrs) override { return _to SetOriginAttributes(aOriginAttrs); } \
  NS_IMETHOD GetPeerAddr(mozilla::net::NetAddr * _retval) override { return _to GetPeerAddr(_retval); } \
  NS_IMETHOD GetSelfAddr(mozilla::net::NetAddr * _retval) override { return _to GetSelfAddr(_retval); } \
  NS_IMETHOD Bind(mozilla::net::NetAddr * aLocalAddr) override { return _to Bind(aLocalAddr); } \
  NS_IMETHOD GetScriptablePeerAddr(nsINetAddr **_retval) override { return _to GetScriptablePeerAddr(_retval); } \
  NS_IMETHOD GetScriptableSelfAddr(nsINetAddr **_retval) override { return _to GetScriptableSelfAddr(_retval); } \
  NS_IMETHOD GetSecurityInfo(nsISupports **aSecurityInfo) override { return _to GetSecurityInfo(aSecurityInfo); } \
  NS_IMETHOD GetSecurityCallbacks(nsIInterfaceRequestor **aSecurityCallbacks) override { return _to GetSecurityCallbacks(aSecurityCallbacks); } \
  NS_IMETHOD SetSecurityCallbacks(nsIInterfaceRequestor *aSecurityCallbacks) override { return _to SetSecurityCallbacks(aSecurityCallbacks); } \
  NS_IMETHOD IsAlive(bool *_retval) override { return _to IsAlive(_retval); } \
  NS_IMETHOD GetTimeout(uint32_t aType, uint32_t *_retval) override { return _to GetTimeout(aType, _retval); } \
  NS_IMETHOD SetTimeout(uint32_t aType, uint32_t aValue) override { return _to SetTimeout(aType, aValue); } \
  NS_IMETHOD SetLinger(bool aPolarity, int16_t aTimeout) override { return _to SetLinger(aPolarity, aTimeout); } \
  NS_IMETHOD SetReuseAddrPort(bool reuseAddrPort) override { return _to SetReuseAddrPort(reuseAddrPort); } \
  NS_IMETHOD GetConnectionFlags(uint32_t *aConnectionFlags) override { return _to GetConnectionFlags(aConnectionFlags); } \
  NS_IMETHOD SetConnectionFlags(uint32_t aConnectionFlags) override { return _to SetConnectionFlags(aConnectionFlags); } \
  NS_IMETHOD GetTlsFlags(uint32_t *aTlsFlags) override { return _to GetTlsFlags(aTlsFlags); } \
  NS_IMETHOD SetTlsFlags(uint32_t aTlsFlags) override { return _to SetTlsFlags(aTlsFlags); } \
  NS_IMETHOD GetQoSBits(uint8_t *aQoSBits) override { return _to GetQoSBits(aQoSBits); } \
  NS_IMETHOD SetQoSBits(uint8_t aQoSBits) override { return _to SetQoSBits(aQoSBits); } \
  NS_IMETHOD GetRecvBufferSize(uint32_t *aRecvBufferSize) override { return _to GetRecvBufferSize(aRecvBufferSize); } \
  NS_IMETHOD SetRecvBufferSize(uint32_t aRecvBufferSize) override { return _to SetRecvBufferSize(aRecvBufferSize); } \
  NS_IMETHOD GetSendBufferSize(uint32_t *aSendBufferSize) override { return _to GetSendBufferSize(aSendBufferSize); } \
  NS_IMETHOD SetSendBufferSize(uint32_t aSendBufferSize) override { return _to SetSendBufferSize(aSendBufferSize); } \
  NS_IMETHOD GetKeepaliveEnabled(bool *aKeepaliveEnabled) override { return _to GetKeepaliveEnabled(aKeepaliveEnabled); } \
  NS_IMETHOD SetKeepaliveEnabled(bool aKeepaliveEnabled) override { return _to SetKeepaliveEnabled(aKeepaliveEnabled); } \
  NS_IMETHOD SetKeepaliveVals(int32_t keepaliveIdleTime, int32_t keepaliveRetryInterval) override { return _to SetKeepaliveVals(keepaliveIdleTime, keepaliveRetryInterval); } \
  NS_IMETHOD GetResetIPFamilyPreference(bool *aResetIPFamilyPreference) override { return _to GetResetIPFamilyPreference(aResetIPFamilyPreference); } \
  NS_IMETHOD GetEchConfigUsed(bool *aEchConfigUsed) override { return _to GetEchConfigUsed(aEchConfigUsed); } \
  NS_IMETHOD SetEchConfig(const nsACString& echConfig) override { return _to SetEchConfig(echConfig); } \
  NS_IMETHOD ResolvedByTRR(bool *_retval) override { return _to ResolvedByTRR(_retval); } \
  NS_IMETHOD SetIsPrivate(bool isPrivate) override { return _to SetIsPrivate(isPrivate); } \
  NS_IMETHOD GetRetryDnsIfPossible(bool *aRetryDnsIfPossible) override { return _to GetRetryDnsIfPossible(aRetryDnsIfPossible); } \
  NS_IMETHOD GetStatus(nsresult *aStatus) override { return _to GetStatus(aStatus); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISOCKETTRANSPORT(_to) \
  NS_IMETHOD GetHost(nsACString& aHost) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHost(aHost); } \
  NS_IMETHOD GetPort(int32_t *aPort) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPort(aPort); } \
  NS_IMETHOD GetScriptableOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScriptableOriginAttributes(cx, aOriginAttributes); } \
  NS_IMETHOD SetScriptableOriginAttributes(JSContext* cx, JS::HandleValue aOriginAttributes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetScriptableOriginAttributes(cx, aOriginAttributes); } \
  virtual nsresult GetOriginAttributes(mozilla::OriginAttributes * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOriginAttributes(_retval); } \
  virtual nsresult SetOriginAttributes(const mozilla::OriginAttributes & aOriginAttrs) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetOriginAttributes(aOriginAttrs); } \
  NS_IMETHOD GetPeerAddr(mozilla::net::NetAddr * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPeerAddr(_retval); } \
  NS_IMETHOD GetSelfAddr(mozilla::net::NetAddr * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelfAddr(_retval); } \
  NS_IMETHOD Bind(mozilla::net::NetAddr * aLocalAddr) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Bind(aLocalAddr); } \
  NS_IMETHOD GetScriptablePeerAddr(nsINetAddr **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScriptablePeerAddr(_retval); } \
  NS_IMETHOD GetScriptableSelfAddr(nsINetAddr **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScriptableSelfAddr(_retval); } \
  NS_IMETHOD GetSecurityInfo(nsISupports **aSecurityInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSecurityInfo(aSecurityInfo); } \
  NS_IMETHOD GetSecurityCallbacks(nsIInterfaceRequestor **aSecurityCallbacks) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSecurityCallbacks(aSecurityCallbacks); } \
  NS_IMETHOD SetSecurityCallbacks(nsIInterfaceRequestor *aSecurityCallbacks) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSecurityCallbacks(aSecurityCallbacks); } \
  NS_IMETHOD IsAlive(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsAlive(_retval); } \
  NS_IMETHOD GetTimeout(uint32_t aType, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTimeout(aType, _retval); } \
  NS_IMETHOD SetTimeout(uint32_t aType, uint32_t aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTimeout(aType, aValue); } \
  NS_IMETHOD SetLinger(bool aPolarity, int16_t aTimeout) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLinger(aPolarity, aTimeout); } \
  NS_IMETHOD SetReuseAddrPort(bool reuseAddrPort) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetReuseAddrPort(reuseAddrPort); } \
  NS_IMETHOD GetConnectionFlags(uint32_t *aConnectionFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetConnectionFlags(aConnectionFlags); } \
  NS_IMETHOD SetConnectionFlags(uint32_t aConnectionFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetConnectionFlags(aConnectionFlags); } \
  NS_IMETHOD GetTlsFlags(uint32_t *aTlsFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTlsFlags(aTlsFlags); } \
  NS_IMETHOD SetTlsFlags(uint32_t aTlsFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTlsFlags(aTlsFlags); } \
  NS_IMETHOD GetQoSBits(uint8_t *aQoSBits) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetQoSBits(aQoSBits); } \
  NS_IMETHOD SetQoSBits(uint8_t aQoSBits) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetQoSBits(aQoSBits); } \
  NS_IMETHOD GetRecvBufferSize(uint32_t *aRecvBufferSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRecvBufferSize(aRecvBufferSize); } \
  NS_IMETHOD SetRecvBufferSize(uint32_t aRecvBufferSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetRecvBufferSize(aRecvBufferSize); } \
  NS_IMETHOD GetSendBufferSize(uint32_t *aSendBufferSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSendBufferSize(aSendBufferSize); } \
  NS_IMETHOD SetSendBufferSize(uint32_t aSendBufferSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSendBufferSize(aSendBufferSize); } \
  NS_IMETHOD GetKeepaliveEnabled(bool *aKeepaliveEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetKeepaliveEnabled(aKeepaliveEnabled); } \
  NS_IMETHOD SetKeepaliveEnabled(bool aKeepaliveEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetKeepaliveEnabled(aKeepaliveEnabled); } \
  NS_IMETHOD SetKeepaliveVals(int32_t keepaliveIdleTime, int32_t keepaliveRetryInterval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetKeepaliveVals(keepaliveIdleTime, keepaliveRetryInterval); } \
  NS_IMETHOD GetResetIPFamilyPreference(bool *aResetIPFamilyPreference) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResetIPFamilyPreference(aResetIPFamilyPreference); } \
  NS_IMETHOD GetEchConfigUsed(bool *aEchConfigUsed) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEchConfigUsed(aEchConfigUsed); } \
  NS_IMETHOD SetEchConfig(const nsACString& echConfig) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetEchConfig(echConfig); } \
  NS_IMETHOD ResolvedByTRR(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ResolvedByTRR(_retval); } \
  NS_IMETHOD SetIsPrivate(bool isPrivate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetIsPrivate(isPrivate); } \
  NS_IMETHOD GetRetryDnsIfPossible(bool *aRetryDnsIfPossible) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRetryDnsIfPossible(aRetryDnsIfPossible); } \
  NS_IMETHOD GetStatus(nsresult *aStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStatus(aStatus); } 


#endif /* __gen_nsISocketTransport_h__ */
