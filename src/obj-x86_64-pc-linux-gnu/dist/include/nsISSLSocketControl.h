/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/socket/nsISSLSocketControl.idl
 */

#ifndef __gen_nsISSLSocketControl_h__
#define __gen_nsISSLSocketControl_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "mozilla/AlreadyAddRefed.h"
#include "mozilla/Assertions.h"
#include "mozilla/DebugOnly.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIInterfaceRequestor; /* forward declaration */

class nsIX509Cert; /* forward declaration */

#include "nsStringFwd.h"
#include "nsTArrayForwardDeclare.h"

/* starting interface:    nsISSLSocketControl */
#define NS_ISSLSOCKETCONTROL_IID_STR "418265c8-654e-4fbb-ba62-4eed27de1f03"

#define NS_ISSLSOCKETCONTROL_IID \
  {0x418265c8, 0x654e, 0x4fbb, \
    { 0xba, 0x62, 0x4e, 0xed, 0x27, 0xde, 0x1f, 0x03 }}

class NS_NO_VTABLE nsISSLSocketControl : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISSLSOCKETCONTROL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISSLSocketControl;

  /* attribute nsIInterfaceRequestor notificationCallbacks; */
  NS_IMETHOD GetNotificationCallbacks(nsIInterfaceRequestor **aNotificationCallbacks) = 0;
  NS_IMETHOD SetNotificationCallbacks(nsIInterfaceRequestor *aNotificationCallbacks) = 0;

  /* void proxyStartSSL (); */
  NS_IMETHOD ProxyStartSSL(void) = 0;

  /* void StartTLS (); */
  NS_IMETHOD StartTLS(void) = 0;

  /* [noscript] void setNPNList (in nsCStringTArrayRef aNPNList); */
  NS_IMETHOD SetNPNList(nsTArray<nsCString> & aNPNList) = 0;

  /* ACString getAlpnEarlySelection (); */
  NS_IMETHOD GetAlpnEarlySelection(nsACString& _retval) = 0;

  /* readonly attribute bool earlyDataAccepted; */
  NS_IMETHOD GetEarlyDataAccepted(bool *aEarlyDataAccepted) = 0;

  /* void driveHandshake (); */
  NS_IMETHOD DriveHandshake(void) = 0;

  /* boolean joinConnection (in ACString npnProtocol, in ACString hostname, in long port); */
  NS_IMETHOD JoinConnection(const nsACString& npnProtocol, const nsACString& hostname, int32_t port, bool *_retval) = 0;

  /* boolean testJoinConnection (in ACString npnProtocol, in ACString hostname, in long port); */
  NS_IMETHOD TestJoinConnection(const nsACString& npnProtocol, const nsACString& hostname, int32_t port, bool *_retval) = 0;

  /* boolean isAcceptableForHost (in ACString hostname); */
  NS_IMETHOD IsAcceptableForHost(const nsACString& hostname, bool *_retval) = 0;

  /* [infallible] readonly attribute short KEAUsed; */
  NS_IMETHOD GetKEAUsed(int16_t *aKEAUsed) = 0;
  inline int16_t  GetKEAUsed()
  {
    int16_t result;
    mozilla::DebugOnly<nsresult> rv = GetKEAUsed(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute unsigned long KEAKeyBits; */
  NS_IMETHOD GetKEAKeyBits(uint32_t *aKEAKeyBits) = 0;
  inline uint32_t  GetKEAKeyBits()
  {
    uint32_t result;
    mozilla::DebugOnly<nsresult> rv = GetKEAKeyBits(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  enum {
    KEY_EXCHANGE_UNKNOWN = -1
  };

  /* readonly attribute uint32_t providerFlags; */
  NS_IMETHOD GetProviderFlags(uint32_t *aProviderFlags) = 0;

  /* readonly attribute uint32_t providerTlsFlags; */
  NS_IMETHOD GetProviderTlsFlags(uint32_t *aProviderTlsFlags) = 0;

  enum {
    SSL_VERSION_3 = 768,
    TLS_VERSION_1 = 769,
    TLS_VERSION_1_1 = 770,
    TLS_VERSION_1_2 = 771,
    TLS_VERSION_1_3 = 772,
    SSL_VERSION_UNKNOWN = -1
  };

  /* [infallible] readonly attribute short SSLVersionUsed; */
  NS_IMETHOD GetSSLVersionUsed(int16_t *aSSLVersionUsed) = 0;
  inline int16_t  GetSSLVersionUsed()
  {
    int16_t result;
    mozilla::DebugOnly<nsresult> rv = GetSSLVersionUsed(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute short SSLVersionOffered; */
  NS_IMETHOD GetSSLVersionOffered(int16_t *aSSLVersionOffered) = 0;
  inline int16_t  GetSSLVersionOffered()
  {
    int16_t result;
    mozilla::DebugOnly<nsresult> rv = GetSSLVersionOffered(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  enum {
    SSL_MAC_UNKNOWN = -1,
    SSL_MAC_NULL = 0,
    SSL_MAC_MD5 = 1,
    SSL_MAC_SHA = 2,
    SSL_HMAC_MD5 = 3,
    SSL_HMAC_SHA = 4,
    SSL_HMAC_SHA256 = 5,
    SSL_MAC_AEAD = 6
  };

  /* [infallible] readonly attribute short MACAlgorithmUsed; */
  NS_IMETHOD GetMACAlgorithmUsed(int16_t *aMACAlgorithmUsed) = 0;
  inline int16_t  GetMACAlgorithmUsed()
  {
    int16_t result;
    mozilla::DebugOnly<nsresult> rv = GetMACAlgorithmUsed(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [nostdcall,notxpcom] attribute boolean denyClientCert; */
  virtual bool GetDenyClientCert() = 0;
  virtual void SetDenyClientCert(bool aDenyClientCert) = 0;

  /* attribute nsIX509Cert clientCert; */
  NS_IMETHOD GetClientCert(nsIX509Cert **aClientCert) = 0;
  NS_IMETHOD SetClientCert(nsIX509Cert *aClientCert) = 0;

  /* [infallible] readonly attribute boolean clientCertSent; */
  NS_IMETHOD GetClientCertSent(bool *aClientCertSent) = 0;
  inline bool  GetClientCertSent()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetClientCertSent(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute boolean failedVerification; */
  NS_IMETHOD GetFailedVerification(bool *aFailedVerification) = 0;
  inline bool  GetFailedVerification()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetFailedVerification(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* attribute ACString esniTxt; */
  NS_IMETHOD GetEsniTxt(nsACString& aEsniTxt) = 0;
  NS_IMETHOD SetEsniTxt(const nsACString& aEsniTxt) = 0;

  /* attribute ACString echConfig; */
  NS_IMETHOD GetEchConfig(nsACString& aEchConfig) = 0;
  NS_IMETHOD SetEchConfig(const nsACString& aEchConfig) = 0;

  /* readonly attribute ACString peerId; */
  NS_IMETHOD GetPeerId(nsACString& aPeerId) = 0;

  /* readonly attribute ACString retryEchConfig; */
  NS_IMETHOD GetRetryEchConfig(nsACString& aRetryEchConfig) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISSLSocketControl, NS_ISSLSOCKETCONTROL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISSLSOCKETCONTROL \
  NS_IMETHOD GetNotificationCallbacks(nsIInterfaceRequestor **aNotificationCallbacks) override; \
  NS_IMETHOD SetNotificationCallbacks(nsIInterfaceRequestor *aNotificationCallbacks) override; \
  NS_IMETHOD ProxyStartSSL(void) override; \
  NS_IMETHOD StartTLS(void) override; \
  NS_IMETHOD SetNPNList(nsTArray<nsCString> & aNPNList) override; \
  NS_IMETHOD GetAlpnEarlySelection(nsACString& _retval) override; \
  NS_IMETHOD GetEarlyDataAccepted(bool *aEarlyDataAccepted) override; \
  NS_IMETHOD DriveHandshake(void) override; \
  NS_IMETHOD JoinConnection(const nsACString& npnProtocol, const nsACString& hostname, int32_t port, bool *_retval) override; \
  NS_IMETHOD TestJoinConnection(const nsACString& npnProtocol, const nsACString& hostname, int32_t port, bool *_retval) override; \
  NS_IMETHOD IsAcceptableForHost(const nsACString& hostname, bool *_retval) override; \
  using nsISSLSocketControl::GetKEAUsed; \
  NS_IMETHOD GetKEAUsed(int16_t *aKEAUsed) override; \
  using nsISSLSocketControl::GetKEAKeyBits; \
  NS_IMETHOD GetKEAKeyBits(uint32_t *aKEAKeyBits) override; \
  NS_IMETHOD GetProviderFlags(uint32_t *aProviderFlags) override; \
  NS_IMETHOD GetProviderTlsFlags(uint32_t *aProviderTlsFlags) override; \
  using nsISSLSocketControl::GetSSLVersionUsed; \
  NS_IMETHOD GetSSLVersionUsed(int16_t *aSSLVersionUsed) override; \
  using nsISSLSocketControl::GetSSLVersionOffered; \
  NS_IMETHOD GetSSLVersionOffered(int16_t *aSSLVersionOffered) override; \
  using nsISSLSocketControl::GetMACAlgorithmUsed; \
  NS_IMETHOD GetMACAlgorithmUsed(int16_t *aMACAlgorithmUsed) override; \
  virtual bool GetDenyClientCert() override; \
  virtual void SetDenyClientCert(bool aDenyClientCert) override; \
  NS_IMETHOD GetClientCert(nsIX509Cert **aClientCert) override; \
  NS_IMETHOD SetClientCert(nsIX509Cert *aClientCert) override; \
  using nsISSLSocketControl::GetClientCertSent; \
  NS_IMETHOD GetClientCertSent(bool *aClientCertSent) override; \
  using nsISSLSocketControl::GetFailedVerification; \
  NS_IMETHOD GetFailedVerification(bool *aFailedVerification) override; \
  NS_IMETHOD GetEsniTxt(nsACString& aEsniTxt) override; \
  NS_IMETHOD SetEsniTxt(const nsACString& aEsniTxt) override; \
  NS_IMETHOD GetEchConfig(nsACString& aEchConfig) override; \
  NS_IMETHOD SetEchConfig(const nsACString& aEchConfig) override; \
  NS_IMETHOD GetPeerId(nsACString& aPeerId) override; \
  NS_IMETHOD GetRetryEchConfig(nsACString& aRetryEchConfig) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISSLSOCKETCONTROL \
  nsresult GetNotificationCallbacks(nsIInterfaceRequestor **aNotificationCallbacks); \
  nsresult SetNotificationCallbacks(nsIInterfaceRequestor *aNotificationCallbacks); \
  nsresult ProxyStartSSL(void); \
  nsresult StartTLS(void); \
  nsresult SetNPNList(nsTArray<nsCString> & aNPNList); \
  nsresult GetAlpnEarlySelection(nsACString& _retval); \
  nsresult GetEarlyDataAccepted(bool *aEarlyDataAccepted); \
  nsresult DriveHandshake(void); \
  nsresult JoinConnection(const nsACString& npnProtocol, const nsACString& hostname, int32_t port, bool *_retval); \
  nsresult TestJoinConnection(const nsACString& npnProtocol, const nsACString& hostname, int32_t port, bool *_retval); \
  nsresult IsAcceptableForHost(const nsACString& hostname, bool *_retval); \
  using nsISSLSocketControl::GetKEAUsed; \
  nsresult GetKEAUsed(int16_t *aKEAUsed); \
  using nsISSLSocketControl::GetKEAKeyBits; \
  nsresult GetKEAKeyBits(uint32_t *aKEAKeyBits); \
  nsresult GetProviderFlags(uint32_t *aProviderFlags); \
  nsresult GetProviderTlsFlags(uint32_t *aProviderTlsFlags); \
  using nsISSLSocketControl::GetSSLVersionUsed; \
  nsresult GetSSLVersionUsed(int16_t *aSSLVersionUsed); \
  using nsISSLSocketControl::GetSSLVersionOffered; \
  nsresult GetSSLVersionOffered(int16_t *aSSLVersionOffered); \
  using nsISSLSocketControl::GetMACAlgorithmUsed; \
  nsresult GetMACAlgorithmUsed(int16_t *aMACAlgorithmUsed); \
  bool GetDenyClientCert(); \
  void SetDenyClientCert(bool aDenyClientCert); \
  nsresult GetClientCert(nsIX509Cert **aClientCert); \
  nsresult SetClientCert(nsIX509Cert *aClientCert); \
  using nsISSLSocketControl::GetClientCertSent; \
  nsresult GetClientCertSent(bool *aClientCertSent); \
  using nsISSLSocketControl::GetFailedVerification; \
  nsresult GetFailedVerification(bool *aFailedVerification); \
  nsresult GetEsniTxt(nsACString& aEsniTxt); \
  nsresult SetEsniTxt(const nsACString& aEsniTxt); \
  nsresult GetEchConfig(nsACString& aEchConfig); \
  nsresult SetEchConfig(const nsACString& aEchConfig); \
  nsresult GetPeerId(nsACString& aPeerId); \
  nsresult GetRetryEchConfig(nsACString& aRetryEchConfig); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISSLSOCKETCONTROL(_to) \
  NS_IMETHOD GetNotificationCallbacks(nsIInterfaceRequestor **aNotificationCallbacks) override { return _to GetNotificationCallbacks(aNotificationCallbacks); } \
  NS_IMETHOD SetNotificationCallbacks(nsIInterfaceRequestor *aNotificationCallbacks) override { return _to SetNotificationCallbacks(aNotificationCallbacks); } \
  NS_IMETHOD ProxyStartSSL(void) override { return _to ProxyStartSSL(); } \
  NS_IMETHOD StartTLS(void) override { return _to StartTLS(); } \
  NS_IMETHOD SetNPNList(nsTArray<nsCString> & aNPNList) override { return _to SetNPNList(aNPNList); } \
  NS_IMETHOD GetAlpnEarlySelection(nsACString& _retval) override { return _to GetAlpnEarlySelection(_retval); } \
  NS_IMETHOD GetEarlyDataAccepted(bool *aEarlyDataAccepted) override { return _to GetEarlyDataAccepted(aEarlyDataAccepted); } \
  NS_IMETHOD DriveHandshake(void) override { return _to DriveHandshake(); } \
  NS_IMETHOD JoinConnection(const nsACString& npnProtocol, const nsACString& hostname, int32_t port, bool *_retval) override { return _to JoinConnection(npnProtocol, hostname, port, _retval); } \
  NS_IMETHOD TestJoinConnection(const nsACString& npnProtocol, const nsACString& hostname, int32_t port, bool *_retval) override { return _to TestJoinConnection(npnProtocol, hostname, port, _retval); } \
  NS_IMETHOD IsAcceptableForHost(const nsACString& hostname, bool *_retval) override { return _to IsAcceptableForHost(hostname, _retval); } \
  using nsISSLSocketControl::GetKEAUsed; \
  NS_IMETHOD GetKEAUsed(int16_t *aKEAUsed) override { return _to GetKEAUsed(aKEAUsed); } \
  using nsISSLSocketControl::GetKEAKeyBits; \
  NS_IMETHOD GetKEAKeyBits(uint32_t *aKEAKeyBits) override { return _to GetKEAKeyBits(aKEAKeyBits); } \
  NS_IMETHOD GetProviderFlags(uint32_t *aProviderFlags) override { return _to GetProviderFlags(aProviderFlags); } \
  NS_IMETHOD GetProviderTlsFlags(uint32_t *aProviderTlsFlags) override { return _to GetProviderTlsFlags(aProviderTlsFlags); } \
  using nsISSLSocketControl::GetSSLVersionUsed; \
  NS_IMETHOD GetSSLVersionUsed(int16_t *aSSLVersionUsed) override { return _to GetSSLVersionUsed(aSSLVersionUsed); } \
  using nsISSLSocketControl::GetSSLVersionOffered; \
  NS_IMETHOD GetSSLVersionOffered(int16_t *aSSLVersionOffered) override { return _to GetSSLVersionOffered(aSSLVersionOffered); } \
  using nsISSLSocketControl::GetMACAlgorithmUsed; \
  NS_IMETHOD GetMACAlgorithmUsed(int16_t *aMACAlgorithmUsed) override { return _to GetMACAlgorithmUsed(aMACAlgorithmUsed); } \
  virtual bool GetDenyClientCert() override { return _to GetDenyClientCert(); } \
  virtual void SetDenyClientCert(bool aDenyClientCert) override { return _to SetDenyClientCert(aDenyClientCert); } \
  NS_IMETHOD GetClientCert(nsIX509Cert **aClientCert) override { return _to GetClientCert(aClientCert); } \
  NS_IMETHOD SetClientCert(nsIX509Cert *aClientCert) override { return _to SetClientCert(aClientCert); } \
  using nsISSLSocketControl::GetClientCertSent; \
  NS_IMETHOD GetClientCertSent(bool *aClientCertSent) override { return _to GetClientCertSent(aClientCertSent); } \
  using nsISSLSocketControl::GetFailedVerification; \
  NS_IMETHOD GetFailedVerification(bool *aFailedVerification) override { return _to GetFailedVerification(aFailedVerification); } \
  NS_IMETHOD GetEsniTxt(nsACString& aEsniTxt) override { return _to GetEsniTxt(aEsniTxt); } \
  NS_IMETHOD SetEsniTxt(const nsACString& aEsniTxt) override { return _to SetEsniTxt(aEsniTxt); } \
  NS_IMETHOD GetEchConfig(nsACString& aEchConfig) override { return _to GetEchConfig(aEchConfig); } \
  NS_IMETHOD SetEchConfig(const nsACString& aEchConfig) override { return _to SetEchConfig(aEchConfig); } \
  NS_IMETHOD GetPeerId(nsACString& aPeerId) override { return _to GetPeerId(aPeerId); } \
  NS_IMETHOD GetRetryEchConfig(nsACString& aRetryEchConfig) override { return _to GetRetryEchConfig(aRetryEchConfig); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISSLSOCKETCONTROL(_to) \
  NS_IMETHOD GetNotificationCallbacks(nsIInterfaceRequestor **aNotificationCallbacks) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNotificationCallbacks(aNotificationCallbacks); } \
  NS_IMETHOD SetNotificationCallbacks(nsIInterfaceRequestor *aNotificationCallbacks) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetNotificationCallbacks(aNotificationCallbacks); } \
  NS_IMETHOD ProxyStartSSL(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ProxyStartSSL(); } \
  NS_IMETHOD StartTLS(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StartTLS(); } \
  NS_IMETHOD SetNPNList(nsTArray<nsCString> & aNPNList) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetNPNList(aNPNList); } \
  NS_IMETHOD GetAlpnEarlySelection(nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAlpnEarlySelection(_retval); } \
  NS_IMETHOD GetEarlyDataAccepted(bool *aEarlyDataAccepted) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEarlyDataAccepted(aEarlyDataAccepted); } \
  NS_IMETHOD DriveHandshake(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DriveHandshake(); } \
  NS_IMETHOD JoinConnection(const nsACString& npnProtocol, const nsACString& hostname, int32_t port, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->JoinConnection(npnProtocol, hostname, port, _retval); } \
  NS_IMETHOD TestJoinConnection(const nsACString& npnProtocol, const nsACString& hostname, int32_t port, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TestJoinConnection(npnProtocol, hostname, port, _retval); } \
  NS_IMETHOD IsAcceptableForHost(const nsACString& hostname, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsAcceptableForHost(hostname, _retval); } \
  NS_IMETHOD GetKEAUsed(int16_t *aKEAUsed) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetKEAUsed(aKEAUsed); } \
  NS_IMETHOD GetKEAKeyBits(uint32_t *aKEAKeyBits) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetKEAKeyBits(aKEAKeyBits); } \
  NS_IMETHOD GetProviderFlags(uint32_t *aProviderFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProviderFlags(aProviderFlags); } \
  NS_IMETHOD GetProviderTlsFlags(uint32_t *aProviderTlsFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProviderTlsFlags(aProviderTlsFlags); } \
  NS_IMETHOD GetSSLVersionUsed(int16_t *aSSLVersionUsed) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSSLVersionUsed(aSSLVersionUsed); } \
  NS_IMETHOD GetSSLVersionOffered(int16_t *aSSLVersionOffered) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSSLVersionOffered(aSSLVersionOffered); } \
  NS_IMETHOD GetMACAlgorithmUsed(int16_t *aMACAlgorithmUsed) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMACAlgorithmUsed(aMACAlgorithmUsed); } \
  virtual bool GetDenyClientCert() override; \
  virtual void SetDenyClientCert(bool aDenyClientCert) override; \
  NS_IMETHOD GetClientCert(nsIX509Cert **aClientCert) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetClientCert(aClientCert); } \
  NS_IMETHOD SetClientCert(nsIX509Cert *aClientCert) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetClientCert(aClientCert); } \
  NS_IMETHOD GetClientCertSent(bool *aClientCertSent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetClientCertSent(aClientCertSent); } \
  NS_IMETHOD GetFailedVerification(bool *aFailedVerification) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFailedVerification(aFailedVerification); } \
  NS_IMETHOD GetEsniTxt(nsACString& aEsniTxt) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEsniTxt(aEsniTxt); } \
  NS_IMETHOD SetEsniTxt(const nsACString& aEsniTxt) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetEsniTxt(aEsniTxt); } \
  NS_IMETHOD GetEchConfig(nsACString& aEchConfig) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEchConfig(aEchConfig); } \
  NS_IMETHOD SetEchConfig(const nsACString& aEchConfig) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetEchConfig(aEchConfig); } \
  NS_IMETHOD GetPeerId(nsACString& aPeerId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPeerId(aPeerId); } \
  NS_IMETHOD GetRetryEchConfig(nsACString& aRetryEchConfig) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRetryEchConfig(aRetryEchConfig); } 


#endif /* __gen_nsISSLSocketControl_h__ */
