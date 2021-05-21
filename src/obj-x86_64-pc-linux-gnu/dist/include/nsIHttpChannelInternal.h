/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIHttpChannelInternal.idl
 */

#ifndef __gen_nsIHttpChannelInternal_h__
#define __gen_nsIHttpChannelInternal_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsILoadInfo_h__
#include "nsILoadInfo.h"
#endif

#include "js/GCAnnotations.h"

#include "mozilla/AlreadyAddRefed.h"
#include "mozilla/Assertions.h"
#include "mozilla/DebugOnly.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#include "nsStringFwd.h"
#include "nsTArrayForwardDeclare.h"
template<class T> class nsCOMArray;
namespace mozilla {
class TimeStamp;
}
class nsIAsyncInputStream; /* forward declaration */

class nsIAsyncOutputStream; /* forward declaration */

class nsIPrincipal; /* forward declaration */

class nsIProxyInfo; /* forward declaration */

class nsISecurityConsoleMessage; /* forward declaration */

class nsISocketTransport; /* forward declaration */

class nsIURI; /* forward declaration */


/* starting interface:    nsIHttpUpgradeListener */
#define NS_IHTTPUPGRADELISTENER_IID_STR "5b515449-ab64-4dba-b3cd-da8fc2f83064"

#define NS_IHTTPUPGRADELISTENER_IID \
  {0x5b515449, 0xab64, 0x4dba, \
    { 0xb3, 0xcd, 0xda, 0x8f, 0xc2, 0xf8, 0x30, 0x64 }}

class NS_NO_VTABLE nsIHttpUpgradeListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IHTTPUPGRADELISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIHttpUpgradeListener;

  /* [must_use] void onTransportAvailable (in nsISocketTransport aTransport, in nsIAsyncInputStream aSocketIn, in nsIAsyncOutputStream aSocketOut); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD OnTransportAvailable(nsISocketTransport *aTransport, nsIAsyncInputStream *aSocketIn, nsIAsyncOutputStream *aSocketOut) = 0;

  /* [must_use] void onUpgradeFailed (in nsresult aErrorCode); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD OnUpgradeFailed(nsresult aErrorCode) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIHttpUpgradeListener, NS_IHTTPUPGRADELISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIHTTPUPGRADELISTENER \
  [[nodiscard]] NS_IMETHOD OnTransportAvailable(nsISocketTransport *aTransport, nsIAsyncInputStream *aSocketIn, nsIAsyncOutputStream *aSocketOut) override; \
  [[nodiscard]] NS_IMETHOD OnUpgradeFailed(nsresult aErrorCode) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIHTTPUPGRADELISTENER \
  [[nodiscard]] nsresult OnTransportAvailable(nsISocketTransport *aTransport, nsIAsyncInputStream *aSocketIn, nsIAsyncOutputStream *aSocketOut); \
  [[nodiscard]] nsresult OnUpgradeFailed(nsresult aErrorCode); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIHTTPUPGRADELISTENER(_to) \
  [[nodiscard]] NS_IMETHOD OnTransportAvailable(nsISocketTransport *aTransport, nsIAsyncInputStream *aSocketIn, nsIAsyncOutputStream *aSocketOut) override { return _to OnTransportAvailable(aTransport, aSocketIn, aSocketOut); } \
  [[nodiscard]] NS_IMETHOD OnUpgradeFailed(nsresult aErrorCode) override { return _to OnUpgradeFailed(aErrorCode); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIHTTPUPGRADELISTENER(_to) \
  [[nodiscard]] NS_IMETHOD OnTransportAvailable(nsISocketTransport *aTransport, nsIAsyncInputStream *aSocketIn, nsIAsyncOutputStream *aSocketOut) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnTransportAvailable(aTransport, aSocketIn, aSocketOut); } \
  [[nodiscard]] NS_IMETHOD OnUpgradeFailed(nsresult aErrorCode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnUpgradeFailed(aErrorCode); } 


/* starting interface:    nsIHttpChannelInternal */
#define NS_IHTTPCHANNELINTERNAL_IID_STR "4e28263d-1e03-46f4-aa5c-9512f91957f9"

#define NS_IHTTPCHANNELINTERNAL_IID \
  {0x4e28263d, 0x1e03, 0x46f4, \
    { 0xaa, 0x5c, 0x95, 0x12, 0xf9, 0x19, 0x57, 0xf9 }}

class NS_NO_VTABLE nsIHttpChannelInternal : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IHTTPCHANNELINTERNAL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIHttpChannelInternal;

  /* [must_use] attribute nsIURI documentURI; */
  [[nodiscard]] NS_IMETHOD GetDocumentURI(nsIURI **aDocumentURI) = 0;
  [[nodiscard]] NS_IMETHOD SetDocumentURI(nsIURI *aDocumentURI) = 0;

  /* [must_use] void getRequestVersion (out unsigned long major, out unsigned long minor); */
  [[nodiscard]] NS_IMETHOD GetRequestVersion(uint32_t *major, uint32_t *minor) = 0;

  /* [must_use] void getResponseVersion (out unsigned long major, out unsigned long minor); */
  [[nodiscard]] NS_IMETHOD GetResponseVersion(uint32_t *major, uint32_t *minor) = 0;

  /* [must_use,noscript] void takeAllSecurityMessages (in securityMessagesArray aMessages); */
  [[nodiscard]] NS_IMETHOD TakeAllSecurityMessages(nsCOMArray<nsISecurityConsoleMessage> & aMessages) = 0;

  /* [must_use] void setCookie (in ACString aCookieHeader); */
  [[nodiscard]] NS_IMETHOD SetCookie(const nsACString& aCookieHeader) = 0;

  /* [must_use] void setupFallbackChannel (in string aFallbackKey); */
  [[nodiscard]] NS_IMETHOD SetupFallbackChannel(const char * aFallbackKey) = 0;

  /* [must_use,noscript] readonly attribute bool isAuthChannel; */
  [[nodiscard]] NS_IMETHOD GetIsAuthChannel(bool *aIsAuthChannel) = 0;

  enum {
    THIRD_PARTY_FORCE_ALLOW = 1U
  };

  /* [must_use] attribute unsigned long thirdPartyFlags; */
  [[nodiscard]] NS_IMETHOD GetThirdPartyFlags(uint32_t *aThirdPartyFlags) = 0;
  [[nodiscard]] NS_IMETHOD SetThirdPartyFlags(uint32_t aThirdPartyFlags) = 0;

  /* [must_use] attribute boolean forceAllowThirdPartyCookie; */
  [[nodiscard]] NS_IMETHOD GetForceAllowThirdPartyCookie(bool *aForceAllowThirdPartyCookie) = 0;
  [[nodiscard]] NS_IMETHOD SetForceAllowThirdPartyCookie(bool aForceAllowThirdPartyCookie) = 0;

  /* [must_use] attribute boolean channelIsForDownload; */
  [[nodiscard]] NS_IMETHOD GetChannelIsForDownload(bool *aChannelIsForDownload) = 0;
  [[nodiscard]] NS_IMETHOD SetChannelIsForDownload(bool aChannelIsForDownload) = 0;

  /* [must_use] readonly attribute AUTF8String localAddress; */
  [[nodiscard]] NS_IMETHOD GetLocalAddress(nsACString& aLocalAddress) = 0;

  /* [must_use] readonly attribute int32_t localPort; */
  [[nodiscard]] NS_IMETHOD GetLocalPort(int32_t *aLocalPort) = 0;

  /* [must_use] readonly attribute AUTF8String remoteAddress; */
  [[nodiscard]] NS_IMETHOD GetRemoteAddress(nsACString& aRemoteAddress) = 0;

  /* [must_use] readonly attribute int32_t remotePort; */
  [[nodiscard]] NS_IMETHOD GetRemotePort(int32_t *aRemotePort) = 0;

  /* [must_use,noscript] void setCacheKeysRedirectChain (in StringArray cacheKeys); */
  [[nodiscard]] NS_IMETHOD SetCacheKeysRedirectChain(nsTArray<nsCString> * cacheKeys) = 0;

  /* [must_use] void HTTPUpgrade (in ACString aProtocolName, in nsIHttpUpgradeListener aListener); */
  [[nodiscard]] NS_IMETHOD HTTPUpgrade(const nsACString& aProtocolName, nsIHttpUpgradeListener *aListener) = 0;

  /* [must_use] void setConnectOnly (); */
  [[nodiscard]] NS_IMETHOD SetConnectOnly(void) = 0;

  /* [must_use] readonly attribute boolean onlyConnect; */
  [[nodiscard]] NS_IMETHOD GetOnlyConnect(bool *aOnlyConnect) = 0;

  /* [must_use] attribute boolean allowSpdy; */
  [[nodiscard]] NS_IMETHOD GetAllowSpdy(bool *aAllowSpdy) = 0;
  [[nodiscard]] NS_IMETHOD SetAllowSpdy(bool aAllowSpdy) = 0;

  /* [must_use] attribute boolean allowHttp3; */
  [[nodiscard]] NS_IMETHOD GetAllowHttp3(bool *aAllowHttp3) = 0;
  [[nodiscard]] NS_IMETHOD SetAllowHttp3(bool aAllowHttp3) = 0;

  /* [must_use] attribute boolean responseTimeoutEnabled; */
  [[nodiscard]] NS_IMETHOD GetResponseTimeoutEnabled(bool *aResponseTimeoutEnabled) = 0;
  [[nodiscard]] NS_IMETHOD SetResponseTimeoutEnabled(bool aResponseTimeoutEnabled) = 0;

  /* [must_use] attribute unsigned long initialRwin; */
  [[nodiscard]] NS_IMETHOD GetInitialRwin(uint32_t *aInitialRwin) = 0;
  [[nodiscard]] NS_IMETHOD SetInitialRwin(uint32_t aInitialRwin) = 0;

  /* [must_use] readonly attribute nsIURI apiRedirectToURI; */
  [[nodiscard]] NS_IMETHOD GetApiRedirectToURI(nsIURI **aApiRedirectToURI) = 0;

  /* [must_use] attribute boolean allowAltSvc; */
  [[nodiscard]] NS_IMETHOD GetAllowAltSvc(bool *aAllowAltSvc) = 0;
  [[nodiscard]] NS_IMETHOD SetAllowAltSvc(bool aAllowAltSvc) = 0;

  /* [must_use] attribute boolean beConservative; */
  [[nodiscard]] NS_IMETHOD GetBeConservative(bool *aBeConservative) = 0;
  [[nodiscard]] NS_IMETHOD SetBeConservative(bool aBeConservative) = 0;

  /* [must_use,noscript] attribute boolean isTRRServiceChannel; */
  [[nodiscard]] NS_IMETHOD GetIsTRRServiceChannel(bool *aIsTRRServiceChannel) = 0;
  [[nodiscard]] NS_IMETHOD SetIsTRRServiceChannel(bool aIsTRRServiceChannel) = 0;

  /* [must_use,noscript] readonly attribute boolean isResolvedByTRR; */
  [[nodiscard]] NS_IMETHOD GetIsResolvedByTRR(bool *aIsResolvedByTRR) = 0;

  /* [must_use] attribute unsigned long tlsFlags; */
  [[nodiscard]] NS_IMETHOD GetTlsFlags(uint32_t *aTlsFlags) = 0;
  [[nodiscard]] NS_IMETHOD SetTlsFlags(uint32_t aTlsFlags) = 0;

  /* [must_use] readonly attribute PRTime lastModifiedTime; */
  [[nodiscard]] NS_IMETHOD GetLastModifiedTime(PRTime *aLastModifiedTime) = 0;

  /* [must_use] attribute boolean corsIncludeCredentials; */
  [[nodiscard]] NS_IMETHOD GetCorsIncludeCredentials(bool *aCorsIncludeCredentials) = 0;
  [[nodiscard]] NS_IMETHOD SetCorsIncludeCredentials(bool aCorsIncludeCredentials) = 0;

  enum {
    CORS_MODE_SAME_ORIGIN = 0U,
    CORS_MODE_NO_CORS = 1U,
    CORS_MODE_CORS = 2U,
    CORS_MODE_NAVIGATE = 3U
  };

  /* [must_use] attribute unsigned long corsMode; */
  [[nodiscard]] NS_IMETHOD GetCorsMode(uint32_t *aCorsMode) = 0;
  [[nodiscard]] NS_IMETHOD SetCorsMode(uint32_t aCorsMode) = 0;

  enum {
    REDIRECT_MODE_FOLLOW = 0U,
    REDIRECT_MODE_ERROR = 1U,
    REDIRECT_MODE_MANUAL = 2U
  };

  /* [must_use] attribute unsigned long redirectMode; */
  [[nodiscard]] NS_IMETHOD GetRedirectMode(uint32_t *aRedirectMode) = 0;
  [[nodiscard]] NS_IMETHOD SetRedirectMode(uint32_t aRedirectMode) = 0;

  enum {
    FETCH_CACHE_MODE_DEFAULT = 0U,
    FETCH_CACHE_MODE_NO_STORE = 1U,
    FETCH_CACHE_MODE_RELOAD = 2U,
    FETCH_CACHE_MODE_NO_CACHE = 3U,
    FETCH_CACHE_MODE_FORCE_CACHE = 4U,
    FETCH_CACHE_MODE_ONLY_IF_CACHED = 5U
  };

  /* [must_use] attribute unsigned long fetchCacheMode; */
  [[nodiscard]] NS_IMETHOD GetFetchCacheMode(uint32_t *aFetchCacheMode) = 0;
  [[nodiscard]] NS_IMETHOD SetFetchCacheMode(uint32_t aFetchCacheMode) = 0;

  /* [must_use] readonly attribute nsIURI topWindowURI; */
  [[nodiscard]] NS_IMETHOD GetTopWindowURI(nsIURI **aTopWindowURI) = 0;

  /* [must_use] void setTopWindowURIIfUnknown (in nsIURI topWindowURI); */
  [[nodiscard]] NS_IMETHOD SetTopWindowURIIfUnknown(nsIURI *topWindowURI) = 0;

  /* [must_use] readonly attribute nsIURI proxyURI; */
  [[nodiscard]] NS_IMETHOD GetProxyURI(nsIURI **aProxyURI) = 0;

  /* [noscript,nostdcall,notxpcom] void setCorsPreflightParameters (in CStringArrayRef unsafeHeaders, in boolean shouldStripRequestBodyHeader); */
  virtual void SetCorsPreflightParameters(const nsTArray<nsCString> & unsafeHeaders, bool shouldStripRequestBodyHeader) = 0;

  /* [noscript,nostdcall,notxpcom] void setAltDataForChild (in boolean aIsForChild); */
  virtual void SetAltDataForChild(bool aIsForChild) = 0;

  /* [noscript,nostdcall,notxpcom] void disableAltDataCache (); */
  virtual void DisableAltDataCache(void) = 0;

  /* [infallible] attribute boolean blockAuthPrompt; */
  NS_IMETHOD GetBlockAuthPrompt(bool *aBlockAuthPrompt) = 0;
  inline bool  GetBlockAuthPrompt()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetBlockAuthPrompt(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetBlockAuthPrompt(bool aBlockAuthPrompt) = 0;

  /* [must_use] attribute AString integrityMetadata; */
  [[nodiscard]] NS_IMETHOD GetIntegrityMetadata(nsAString& aIntegrityMetadata) = 0;
  [[nodiscard]] NS_IMETHOD SetIntegrityMetadata(const nsAString& aIntegrityMetadata) = 0;

  /* [must_use] readonly attribute ACString connectionInfoHashKey; */
  [[nodiscard]] NS_IMETHOD GetConnectionInfoHashKey(nsACString& aConnectionInfoHashKey) = 0;

  /* [infallible,noscript] attribute unsigned long lastRedirectFlags; */
  NS_IMETHOD GetLastRedirectFlags(uint32_t *aLastRedirectFlags) = 0;
  inline uint32_t  GetLastRedirectFlags()
  {
    uint32_t result;
    mozilla::DebugOnly<nsresult> rv = GetLastRedirectFlags(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetLastRedirectFlags(uint32_t aLastRedirectFlags) = 0;

  /* [noscript] attribute TimeStamp navigationStartTimeStamp; */
  NS_IMETHOD GetNavigationStartTimeStamp(mozilla::TimeStamp * aNavigationStartTimeStamp) = 0;
  NS_IMETHOD SetNavigationStartTimeStamp(mozilla::TimeStamp aNavigationStartTimeStamp) = 0;

  /* [noscript] void cancelByURLClassifier (in nsresult aErrorCode); */
  NS_IMETHOD CancelByURLClassifier(nsresult aErrorCode) = 0;

  /* [noscript,nostdcall,notxpcom] void setIPv4Disabled (); */
  virtual void SetIPv4Disabled(void) = 0;

  /* [noscript,nostdcall,notxpcom] void setIPv6Disabled (); */
  virtual void SetIPv6Disabled(void) = 0;

  /* readonly attribute nsILoadInfo_CrossOriginOpenerPolicy crossOriginOpenerPolicy; */
  NS_IMETHOD GetCrossOriginOpenerPolicy(nsILoadInfo::CrossOriginOpenerPolicy *aCrossOriginOpenerPolicy) = 0;

  /* [noscript] nsILoadInfo_CrossOriginOpenerPolicy computeCrossOriginOpenerPolicy (in nsILoadInfo_CrossOriginOpenerPolicy aInitiatorPolicy); */
  NS_IMETHOD ComputeCrossOriginOpenerPolicy(nsILoadInfo::CrossOriginOpenerPolicy aInitiatorPolicy, nsILoadInfo::CrossOriginOpenerPolicy *_retval) = 0;

  /* [noscript] bool hasCrossOriginOpenerPolicyMismatch (); */
  NS_IMETHOD HasCrossOriginOpenerPolicyMismatch(bool *_retval) = 0;

  /* [noscript] nsILoadInfo_CrossOriginEmbedderPolicy getResponseEmbedderPolicy (); */
  NS_IMETHOD GetResponseEmbedderPolicy(nsILoadInfo::CrossOriginEmbedderPolicy *_retval) = 0;

  /* [nostdcall,notxpcom] attribute boolean hasNonEmptySandboxingFlag; */
  virtual bool GetHasNonEmptySandboxingFlag() = 0;
  virtual void SetHasNonEmptySandboxingFlag(bool aHasNonEmptySandboxingFlag) = 0;

  /* [noscript,nostdcall,notxpcom] void DoDiagnosticAssertWhenOnStopNotCalledOnDestroy (); */
  virtual void DoDiagnosticAssertWhenOnStopNotCalledOnDestroy(void) = 0;

  /* [must_use] void setWaitForHTTPSSVCRecord (); */
  [[nodiscard]] NS_IMETHOD SetWaitForHTTPSSVCRecord(void) = 0;

  /* [must_use] readonly attribute boolean supportsHTTP3; */
  [[nodiscard]] NS_IMETHOD GetSupportsHTTP3(bool *aSupportsHTTP3) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIHttpChannelInternal, NS_IHTTPCHANNELINTERNAL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIHTTPCHANNELINTERNAL \
  [[nodiscard]] NS_IMETHOD GetDocumentURI(nsIURI **aDocumentURI) override; \
  [[nodiscard]] NS_IMETHOD SetDocumentURI(nsIURI *aDocumentURI) override; \
  [[nodiscard]] NS_IMETHOD GetRequestVersion(uint32_t *major, uint32_t *minor) override; \
  [[nodiscard]] NS_IMETHOD GetResponseVersion(uint32_t *major, uint32_t *minor) override; \
  [[nodiscard]] NS_IMETHOD TakeAllSecurityMessages(nsCOMArray<nsISecurityConsoleMessage> & aMessages) override; \
  [[nodiscard]] NS_IMETHOD SetCookie(const nsACString& aCookieHeader) override; \
  [[nodiscard]] NS_IMETHOD SetupFallbackChannel(const char * aFallbackKey) override; \
  [[nodiscard]] NS_IMETHOD GetIsAuthChannel(bool *aIsAuthChannel) override; \
  [[nodiscard]] NS_IMETHOD GetThirdPartyFlags(uint32_t *aThirdPartyFlags) override; \
  [[nodiscard]] NS_IMETHOD SetThirdPartyFlags(uint32_t aThirdPartyFlags) override; \
  [[nodiscard]] NS_IMETHOD GetForceAllowThirdPartyCookie(bool *aForceAllowThirdPartyCookie) override; \
  [[nodiscard]] NS_IMETHOD SetForceAllowThirdPartyCookie(bool aForceAllowThirdPartyCookie) override; \
  [[nodiscard]] NS_IMETHOD GetChannelIsForDownload(bool *aChannelIsForDownload) override; \
  [[nodiscard]] NS_IMETHOD SetChannelIsForDownload(bool aChannelIsForDownload) override; \
  [[nodiscard]] NS_IMETHOD GetLocalAddress(nsACString& aLocalAddress) override; \
  [[nodiscard]] NS_IMETHOD GetLocalPort(int32_t *aLocalPort) override; \
  [[nodiscard]] NS_IMETHOD GetRemoteAddress(nsACString& aRemoteAddress) override; \
  [[nodiscard]] NS_IMETHOD GetRemotePort(int32_t *aRemotePort) override; \
  [[nodiscard]] NS_IMETHOD SetCacheKeysRedirectChain(nsTArray<nsCString> * cacheKeys) override; \
  [[nodiscard]] NS_IMETHOD HTTPUpgrade(const nsACString& aProtocolName, nsIHttpUpgradeListener *aListener) override; \
  [[nodiscard]] NS_IMETHOD SetConnectOnly(void) override; \
  [[nodiscard]] NS_IMETHOD GetOnlyConnect(bool *aOnlyConnect) override; \
  [[nodiscard]] NS_IMETHOD GetAllowSpdy(bool *aAllowSpdy) override; \
  [[nodiscard]] NS_IMETHOD SetAllowSpdy(bool aAllowSpdy) override; \
  [[nodiscard]] NS_IMETHOD GetAllowHttp3(bool *aAllowHttp3) override; \
  [[nodiscard]] NS_IMETHOD SetAllowHttp3(bool aAllowHttp3) override; \
  [[nodiscard]] NS_IMETHOD GetResponseTimeoutEnabled(bool *aResponseTimeoutEnabled) override; \
  [[nodiscard]] NS_IMETHOD SetResponseTimeoutEnabled(bool aResponseTimeoutEnabled) override; \
  [[nodiscard]] NS_IMETHOD GetInitialRwin(uint32_t *aInitialRwin) override; \
  [[nodiscard]] NS_IMETHOD SetInitialRwin(uint32_t aInitialRwin) override; \
  [[nodiscard]] NS_IMETHOD GetApiRedirectToURI(nsIURI **aApiRedirectToURI) override; \
  [[nodiscard]] NS_IMETHOD GetAllowAltSvc(bool *aAllowAltSvc) override; \
  [[nodiscard]] NS_IMETHOD SetAllowAltSvc(bool aAllowAltSvc) override; \
  [[nodiscard]] NS_IMETHOD GetBeConservative(bool *aBeConservative) override; \
  [[nodiscard]] NS_IMETHOD SetBeConservative(bool aBeConservative) override; \
  [[nodiscard]] NS_IMETHOD GetIsTRRServiceChannel(bool *aIsTRRServiceChannel) override; \
  [[nodiscard]] NS_IMETHOD SetIsTRRServiceChannel(bool aIsTRRServiceChannel) override; \
  [[nodiscard]] NS_IMETHOD GetIsResolvedByTRR(bool *aIsResolvedByTRR) override; \
  [[nodiscard]] NS_IMETHOD GetTlsFlags(uint32_t *aTlsFlags) override; \
  [[nodiscard]] NS_IMETHOD SetTlsFlags(uint32_t aTlsFlags) override; \
  [[nodiscard]] NS_IMETHOD GetLastModifiedTime(PRTime *aLastModifiedTime) override; \
  [[nodiscard]] NS_IMETHOD GetCorsIncludeCredentials(bool *aCorsIncludeCredentials) override; \
  [[nodiscard]] NS_IMETHOD SetCorsIncludeCredentials(bool aCorsIncludeCredentials) override; \
  [[nodiscard]] NS_IMETHOD GetCorsMode(uint32_t *aCorsMode) override; \
  [[nodiscard]] NS_IMETHOD SetCorsMode(uint32_t aCorsMode) override; \
  [[nodiscard]] NS_IMETHOD GetRedirectMode(uint32_t *aRedirectMode) override; \
  [[nodiscard]] NS_IMETHOD SetRedirectMode(uint32_t aRedirectMode) override; \
  [[nodiscard]] NS_IMETHOD GetFetchCacheMode(uint32_t *aFetchCacheMode) override; \
  [[nodiscard]] NS_IMETHOD SetFetchCacheMode(uint32_t aFetchCacheMode) override; \
  [[nodiscard]] NS_IMETHOD GetTopWindowURI(nsIURI **aTopWindowURI) override; \
  [[nodiscard]] NS_IMETHOD SetTopWindowURIIfUnknown(nsIURI *topWindowURI) override; \
  [[nodiscard]] NS_IMETHOD GetProxyURI(nsIURI **aProxyURI) override; \
  virtual void SetCorsPreflightParameters(const nsTArray<nsCString> & unsafeHeaders, bool shouldStripRequestBodyHeader) override; \
  virtual void SetAltDataForChild(bool aIsForChild) override; \
  virtual void DisableAltDataCache(void) override; \
  using nsIHttpChannelInternal::GetBlockAuthPrompt; \
  NS_IMETHOD GetBlockAuthPrompt(bool *aBlockAuthPrompt) override; \
  NS_IMETHOD SetBlockAuthPrompt(bool aBlockAuthPrompt) override; \
  [[nodiscard]] NS_IMETHOD GetIntegrityMetadata(nsAString& aIntegrityMetadata) override; \
  [[nodiscard]] NS_IMETHOD SetIntegrityMetadata(const nsAString& aIntegrityMetadata) override; \
  [[nodiscard]] NS_IMETHOD GetConnectionInfoHashKey(nsACString& aConnectionInfoHashKey) override; \
  using nsIHttpChannelInternal::GetLastRedirectFlags; \
  NS_IMETHOD GetLastRedirectFlags(uint32_t *aLastRedirectFlags) override; \
  NS_IMETHOD SetLastRedirectFlags(uint32_t aLastRedirectFlags) override; \
  NS_IMETHOD GetNavigationStartTimeStamp(mozilla::TimeStamp * aNavigationStartTimeStamp) override; \
  NS_IMETHOD SetNavigationStartTimeStamp(mozilla::TimeStamp aNavigationStartTimeStamp) override; \
  NS_IMETHOD CancelByURLClassifier(nsresult aErrorCode) override; \
  virtual void SetIPv4Disabled(void) override; \
  virtual void SetIPv6Disabled(void) override; \
  NS_IMETHOD GetCrossOriginOpenerPolicy(nsILoadInfo::CrossOriginOpenerPolicy *aCrossOriginOpenerPolicy) override; \
  NS_IMETHOD ComputeCrossOriginOpenerPolicy(nsILoadInfo::CrossOriginOpenerPolicy aInitiatorPolicy, nsILoadInfo::CrossOriginOpenerPolicy *_retval) override; \
  NS_IMETHOD HasCrossOriginOpenerPolicyMismatch(bool *_retval) override; \
  NS_IMETHOD GetResponseEmbedderPolicy(nsILoadInfo::CrossOriginEmbedderPolicy *_retval) override; \
  virtual bool GetHasNonEmptySandboxingFlag() override; \
  virtual void SetHasNonEmptySandboxingFlag(bool aHasNonEmptySandboxingFlag) override; \
  virtual void DoDiagnosticAssertWhenOnStopNotCalledOnDestroy(void) override; \
  [[nodiscard]] NS_IMETHOD SetWaitForHTTPSSVCRecord(void) override; \
  [[nodiscard]] NS_IMETHOD GetSupportsHTTP3(bool *aSupportsHTTP3) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIHTTPCHANNELINTERNAL \
  [[nodiscard]] nsresult GetDocumentURI(nsIURI **aDocumentURI); \
  [[nodiscard]] nsresult SetDocumentURI(nsIURI *aDocumentURI); \
  [[nodiscard]] nsresult GetRequestVersion(uint32_t *major, uint32_t *minor); \
  [[nodiscard]] nsresult GetResponseVersion(uint32_t *major, uint32_t *minor); \
  [[nodiscard]] nsresult TakeAllSecurityMessages(nsCOMArray<nsISecurityConsoleMessage> & aMessages); \
  [[nodiscard]] nsresult SetCookie(const nsACString& aCookieHeader); \
  [[nodiscard]] nsresult SetupFallbackChannel(const char * aFallbackKey); \
  [[nodiscard]] nsresult GetIsAuthChannel(bool *aIsAuthChannel); \
  [[nodiscard]] nsresult GetThirdPartyFlags(uint32_t *aThirdPartyFlags); \
  [[nodiscard]] nsresult SetThirdPartyFlags(uint32_t aThirdPartyFlags); \
  [[nodiscard]] nsresult GetForceAllowThirdPartyCookie(bool *aForceAllowThirdPartyCookie); \
  [[nodiscard]] nsresult SetForceAllowThirdPartyCookie(bool aForceAllowThirdPartyCookie); \
  [[nodiscard]] nsresult GetChannelIsForDownload(bool *aChannelIsForDownload); \
  [[nodiscard]] nsresult SetChannelIsForDownload(bool aChannelIsForDownload); \
  [[nodiscard]] nsresult GetLocalAddress(nsACString& aLocalAddress); \
  [[nodiscard]] nsresult GetLocalPort(int32_t *aLocalPort); \
  [[nodiscard]] nsresult GetRemoteAddress(nsACString& aRemoteAddress); \
  [[nodiscard]] nsresult GetRemotePort(int32_t *aRemotePort); \
  [[nodiscard]] nsresult SetCacheKeysRedirectChain(nsTArray<nsCString> * cacheKeys); \
  [[nodiscard]] nsresult HTTPUpgrade(const nsACString& aProtocolName, nsIHttpUpgradeListener *aListener); \
  [[nodiscard]] nsresult SetConnectOnly(void); \
  [[nodiscard]] nsresult GetOnlyConnect(bool *aOnlyConnect); \
  [[nodiscard]] nsresult GetAllowSpdy(bool *aAllowSpdy); \
  [[nodiscard]] nsresult SetAllowSpdy(bool aAllowSpdy); \
  [[nodiscard]] nsresult GetAllowHttp3(bool *aAllowHttp3); \
  [[nodiscard]] nsresult SetAllowHttp3(bool aAllowHttp3); \
  [[nodiscard]] nsresult GetResponseTimeoutEnabled(bool *aResponseTimeoutEnabled); \
  [[nodiscard]] nsresult SetResponseTimeoutEnabled(bool aResponseTimeoutEnabled); \
  [[nodiscard]] nsresult GetInitialRwin(uint32_t *aInitialRwin); \
  [[nodiscard]] nsresult SetInitialRwin(uint32_t aInitialRwin); \
  [[nodiscard]] nsresult GetApiRedirectToURI(nsIURI **aApiRedirectToURI); \
  [[nodiscard]] nsresult GetAllowAltSvc(bool *aAllowAltSvc); \
  [[nodiscard]] nsresult SetAllowAltSvc(bool aAllowAltSvc); \
  [[nodiscard]] nsresult GetBeConservative(bool *aBeConservative); \
  [[nodiscard]] nsresult SetBeConservative(bool aBeConservative); \
  [[nodiscard]] nsresult GetIsTRRServiceChannel(bool *aIsTRRServiceChannel); \
  [[nodiscard]] nsresult SetIsTRRServiceChannel(bool aIsTRRServiceChannel); \
  [[nodiscard]] nsresult GetIsResolvedByTRR(bool *aIsResolvedByTRR); \
  [[nodiscard]] nsresult GetTlsFlags(uint32_t *aTlsFlags); \
  [[nodiscard]] nsresult SetTlsFlags(uint32_t aTlsFlags); \
  [[nodiscard]] nsresult GetLastModifiedTime(PRTime *aLastModifiedTime); \
  [[nodiscard]] nsresult GetCorsIncludeCredentials(bool *aCorsIncludeCredentials); \
  [[nodiscard]] nsresult SetCorsIncludeCredentials(bool aCorsIncludeCredentials); \
  [[nodiscard]] nsresult GetCorsMode(uint32_t *aCorsMode); \
  [[nodiscard]] nsresult SetCorsMode(uint32_t aCorsMode); \
  [[nodiscard]] nsresult GetRedirectMode(uint32_t *aRedirectMode); \
  [[nodiscard]] nsresult SetRedirectMode(uint32_t aRedirectMode); \
  [[nodiscard]] nsresult GetFetchCacheMode(uint32_t *aFetchCacheMode); \
  [[nodiscard]] nsresult SetFetchCacheMode(uint32_t aFetchCacheMode); \
  [[nodiscard]] nsresult GetTopWindowURI(nsIURI **aTopWindowURI); \
  [[nodiscard]] nsresult SetTopWindowURIIfUnknown(nsIURI *topWindowURI); \
  [[nodiscard]] nsresult GetProxyURI(nsIURI **aProxyURI); \
  void SetCorsPreflightParameters(const nsTArray<nsCString> & unsafeHeaders, bool shouldStripRequestBodyHeader); \
  void SetAltDataForChild(bool aIsForChild); \
  void DisableAltDataCache(void); \
  using nsIHttpChannelInternal::GetBlockAuthPrompt; \
  nsresult GetBlockAuthPrompt(bool *aBlockAuthPrompt); \
  nsresult SetBlockAuthPrompt(bool aBlockAuthPrompt); \
  [[nodiscard]] nsresult GetIntegrityMetadata(nsAString& aIntegrityMetadata); \
  [[nodiscard]] nsresult SetIntegrityMetadata(const nsAString& aIntegrityMetadata); \
  [[nodiscard]] nsresult GetConnectionInfoHashKey(nsACString& aConnectionInfoHashKey); \
  using nsIHttpChannelInternal::GetLastRedirectFlags; \
  nsresult GetLastRedirectFlags(uint32_t *aLastRedirectFlags); \
  nsresult SetLastRedirectFlags(uint32_t aLastRedirectFlags); \
  nsresult GetNavigationStartTimeStamp(mozilla::TimeStamp * aNavigationStartTimeStamp); \
  nsresult SetNavigationStartTimeStamp(mozilla::TimeStamp aNavigationStartTimeStamp); \
  nsresult CancelByURLClassifier(nsresult aErrorCode); \
  void SetIPv4Disabled(void); \
  void SetIPv6Disabled(void); \
  nsresult GetCrossOriginOpenerPolicy(nsILoadInfo::CrossOriginOpenerPolicy *aCrossOriginOpenerPolicy); \
  nsresult ComputeCrossOriginOpenerPolicy(nsILoadInfo::CrossOriginOpenerPolicy aInitiatorPolicy, nsILoadInfo::CrossOriginOpenerPolicy *_retval); \
  nsresult HasCrossOriginOpenerPolicyMismatch(bool *_retval); \
  nsresult GetResponseEmbedderPolicy(nsILoadInfo::CrossOriginEmbedderPolicy *_retval); \
  bool GetHasNonEmptySandboxingFlag(); \
  void SetHasNonEmptySandboxingFlag(bool aHasNonEmptySandboxingFlag); \
  void DoDiagnosticAssertWhenOnStopNotCalledOnDestroy(void); \
  [[nodiscard]] nsresult SetWaitForHTTPSSVCRecord(void); \
  [[nodiscard]] nsresult GetSupportsHTTP3(bool *aSupportsHTTP3); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIHTTPCHANNELINTERNAL(_to) \
  [[nodiscard]] NS_IMETHOD GetDocumentURI(nsIURI **aDocumentURI) override { return _to GetDocumentURI(aDocumentURI); } \
  [[nodiscard]] NS_IMETHOD SetDocumentURI(nsIURI *aDocumentURI) override { return _to SetDocumentURI(aDocumentURI); } \
  [[nodiscard]] NS_IMETHOD GetRequestVersion(uint32_t *major, uint32_t *minor) override { return _to GetRequestVersion(major, minor); } \
  [[nodiscard]] NS_IMETHOD GetResponseVersion(uint32_t *major, uint32_t *minor) override { return _to GetResponseVersion(major, minor); } \
  [[nodiscard]] NS_IMETHOD TakeAllSecurityMessages(nsCOMArray<nsISecurityConsoleMessage> & aMessages) override { return _to TakeAllSecurityMessages(aMessages); } \
  [[nodiscard]] NS_IMETHOD SetCookie(const nsACString& aCookieHeader) override { return _to SetCookie(aCookieHeader); } \
  [[nodiscard]] NS_IMETHOD SetupFallbackChannel(const char * aFallbackKey) override { return _to SetupFallbackChannel(aFallbackKey); } \
  [[nodiscard]] NS_IMETHOD GetIsAuthChannel(bool *aIsAuthChannel) override { return _to GetIsAuthChannel(aIsAuthChannel); } \
  [[nodiscard]] NS_IMETHOD GetThirdPartyFlags(uint32_t *aThirdPartyFlags) override { return _to GetThirdPartyFlags(aThirdPartyFlags); } \
  [[nodiscard]] NS_IMETHOD SetThirdPartyFlags(uint32_t aThirdPartyFlags) override { return _to SetThirdPartyFlags(aThirdPartyFlags); } \
  [[nodiscard]] NS_IMETHOD GetForceAllowThirdPartyCookie(bool *aForceAllowThirdPartyCookie) override { return _to GetForceAllowThirdPartyCookie(aForceAllowThirdPartyCookie); } \
  [[nodiscard]] NS_IMETHOD SetForceAllowThirdPartyCookie(bool aForceAllowThirdPartyCookie) override { return _to SetForceAllowThirdPartyCookie(aForceAllowThirdPartyCookie); } \
  [[nodiscard]] NS_IMETHOD GetChannelIsForDownload(bool *aChannelIsForDownload) override { return _to GetChannelIsForDownload(aChannelIsForDownload); } \
  [[nodiscard]] NS_IMETHOD SetChannelIsForDownload(bool aChannelIsForDownload) override { return _to SetChannelIsForDownload(aChannelIsForDownload); } \
  [[nodiscard]] NS_IMETHOD GetLocalAddress(nsACString& aLocalAddress) override { return _to GetLocalAddress(aLocalAddress); } \
  [[nodiscard]] NS_IMETHOD GetLocalPort(int32_t *aLocalPort) override { return _to GetLocalPort(aLocalPort); } \
  [[nodiscard]] NS_IMETHOD GetRemoteAddress(nsACString& aRemoteAddress) override { return _to GetRemoteAddress(aRemoteAddress); } \
  [[nodiscard]] NS_IMETHOD GetRemotePort(int32_t *aRemotePort) override { return _to GetRemotePort(aRemotePort); } \
  [[nodiscard]] NS_IMETHOD SetCacheKeysRedirectChain(nsTArray<nsCString> * cacheKeys) override { return _to SetCacheKeysRedirectChain(cacheKeys); } \
  [[nodiscard]] NS_IMETHOD HTTPUpgrade(const nsACString& aProtocolName, nsIHttpUpgradeListener *aListener) override { return _to HTTPUpgrade(aProtocolName, aListener); } \
  [[nodiscard]] NS_IMETHOD SetConnectOnly(void) override { return _to SetConnectOnly(); } \
  [[nodiscard]] NS_IMETHOD GetOnlyConnect(bool *aOnlyConnect) override { return _to GetOnlyConnect(aOnlyConnect); } \
  [[nodiscard]] NS_IMETHOD GetAllowSpdy(bool *aAllowSpdy) override { return _to GetAllowSpdy(aAllowSpdy); } \
  [[nodiscard]] NS_IMETHOD SetAllowSpdy(bool aAllowSpdy) override { return _to SetAllowSpdy(aAllowSpdy); } \
  [[nodiscard]] NS_IMETHOD GetAllowHttp3(bool *aAllowHttp3) override { return _to GetAllowHttp3(aAllowHttp3); } \
  [[nodiscard]] NS_IMETHOD SetAllowHttp3(bool aAllowHttp3) override { return _to SetAllowHttp3(aAllowHttp3); } \
  [[nodiscard]] NS_IMETHOD GetResponseTimeoutEnabled(bool *aResponseTimeoutEnabled) override { return _to GetResponseTimeoutEnabled(aResponseTimeoutEnabled); } \
  [[nodiscard]] NS_IMETHOD SetResponseTimeoutEnabled(bool aResponseTimeoutEnabled) override { return _to SetResponseTimeoutEnabled(aResponseTimeoutEnabled); } \
  [[nodiscard]] NS_IMETHOD GetInitialRwin(uint32_t *aInitialRwin) override { return _to GetInitialRwin(aInitialRwin); } \
  [[nodiscard]] NS_IMETHOD SetInitialRwin(uint32_t aInitialRwin) override { return _to SetInitialRwin(aInitialRwin); } \
  [[nodiscard]] NS_IMETHOD GetApiRedirectToURI(nsIURI **aApiRedirectToURI) override { return _to GetApiRedirectToURI(aApiRedirectToURI); } \
  [[nodiscard]] NS_IMETHOD GetAllowAltSvc(bool *aAllowAltSvc) override { return _to GetAllowAltSvc(aAllowAltSvc); } \
  [[nodiscard]] NS_IMETHOD SetAllowAltSvc(bool aAllowAltSvc) override { return _to SetAllowAltSvc(aAllowAltSvc); } \
  [[nodiscard]] NS_IMETHOD GetBeConservative(bool *aBeConservative) override { return _to GetBeConservative(aBeConservative); } \
  [[nodiscard]] NS_IMETHOD SetBeConservative(bool aBeConservative) override { return _to SetBeConservative(aBeConservative); } \
  [[nodiscard]] NS_IMETHOD GetIsTRRServiceChannel(bool *aIsTRRServiceChannel) override { return _to GetIsTRRServiceChannel(aIsTRRServiceChannel); } \
  [[nodiscard]] NS_IMETHOD SetIsTRRServiceChannel(bool aIsTRRServiceChannel) override { return _to SetIsTRRServiceChannel(aIsTRRServiceChannel); } \
  [[nodiscard]] NS_IMETHOD GetIsResolvedByTRR(bool *aIsResolvedByTRR) override { return _to GetIsResolvedByTRR(aIsResolvedByTRR); } \
  [[nodiscard]] NS_IMETHOD GetTlsFlags(uint32_t *aTlsFlags) override { return _to GetTlsFlags(aTlsFlags); } \
  [[nodiscard]] NS_IMETHOD SetTlsFlags(uint32_t aTlsFlags) override { return _to SetTlsFlags(aTlsFlags); } \
  [[nodiscard]] NS_IMETHOD GetLastModifiedTime(PRTime *aLastModifiedTime) override { return _to GetLastModifiedTime(aLastModifiedTime); } \
  [[nodiscard]] NS_IMETHOD GetCorsIncludeCredentials(bool *aCorsIncludeCredentials) override { return _to GetCorsIncludeCredentials(aCorsIncludeCredentials); } \
  [[nodiscard]] NS_IMETHOD SetCorsIncludeCredentials(bool aCorsIncludeCredentials) override { return _to SetCorsIncludeCredentials(aCorsIncludeCredentials); } \
  [[nodiscard]] NS_IMETHOD GetCorsMode(uint32_t *aCorsMode) override { return _to GetCorsMode(aCorsMode); } \
  [[nodiscard]] NS_IMETHOD SetCorsMode(uint32_t aCorsMode) override { return _to SetCorsMode(aCorsMode); } \
  [[nodiscard]] NS_IMETHOD GetRedirectMode(uint32_t *aRedirectMode) override { return _to GetRedirectMode(aRedirectMode); } \
  [[nodiscard]] NS_IMETHOD SetRedirectMode(uint32_t aRedirectMode) override { return _to SetRedirectMode(aRedirectMode); } \
  [[nodiscard]] NS_IMETHOD GetFetchCacheMode(uint32_t *aFetchCacheMode) override { return _to GetFetchCacheMode(aFetchCacheMode); } \
  [[nodiscard]] NS_IMETHOD SetFetchCacheMode(uint32_t aFetchCacheMode) override { return _to SetFetchCacheMode(aFetchCacheMode); } \
  [[nodiscard]] NS_IMETHOD GetTopWindowURI(nsIURI **aTopWindowURI) override { return _to GetTopWindowURI(aTopWindowURI); } \
  [[nodiscard]] NS_IMETHOD SetTopWindowURIIfUnknown(nsIURI *topWindowURI) override { return _to SetTopWindowURIIfUnknown(topWindowURI); } \
  [[nodiscard]] NS_IMETHOD GetProxyURI(nsIURI **aProxyURI) override { return _to GetProxyURI(aProxyURI); } \
  virtual void SetCorsPreflightParameters(const nsTArray<nsCString> & unsafeHeaders, bool shouldStripRequestBodyHeader) override { return _to SetCorsPreflightParameters(unsafeHeaders, shouldStripRequestBodyHeader); } \
  virtual void SetAltDataForChild(bool aIsForChild) override { return _to SetAltDataForChild(aIsForChild); } \
  virtual void DisableAltDataCache(void) override { return _to DisableAltDataCache(); } \
  using nsIHttpChannelInternal::GetBlockAuthPrompt; \
  NS_IMETHOD GetBlockAuthPrompt(bool *aBlockAuthPrompt) override { return _to GetBlockAuthPrompt(aBlockAuthPrompt); } \
  NS_IMETHOD SetBlockAuthPrompt(bool aBlockAuthPrompt) override { return _to SetBlockAuthPrompt(aBlockAuthPrompt); } \
  [[nodiscard]] NS_IMETHOD GetIntegrityMetadata(nsAString& aIntegrityMetadata) override { return _to GetIntegrityMetadata(aIntegrityMetadata); } \
  [[nodiscard]] NS_IMETHOD SetIntegrityMetadata(const nsAString& aIntegrityMetadata) override { return _to SetIntegrityMetadata(aIntegrityMetadata); } \
  [[nodiscard]] NS_IMETHOD GetConnectionInfoHashKey(nsACString& aConnectionInfoHashKey) override { return _to GetConnectionInfoHashKey(aConnectionInfoHashKey); } \
  using nsIHttpChannelInternal::GetLastRedirectFlags; \
  NS_IMETHOD GetLastRedirectFlags(uint32_t *aLastRedirectFlags) override { return _to GetLastRedirectFlags(aLastRedirectFlags); } \
  NS_IMETHOD SetLastRedirectFlags(uint32_t aLastRedirectFlags) override { return _to SetLastRedirectFlags(aLastRedirectFlags); } \
  NS_IMETHOD GetNavigationStartTimeStamp(mozilla::TimeStamp * aNavigationStartTimeStamp) override { return _to GetNavigationStartTimeStamp(aNavigationStartTimeStamp); } \
  NS_IMETHOD SetNavigationStartTimeStamp(mozilla::TimeStamp aNavigationStartTimeStamp) override { return _to SetNavigationStartTimeStamp(aNavigationStartTimeStamp); } \
  NS_IMETHOD CancelByURLClassifier(nsresult aErrorCode) override { return _to CancelByURLClassifier(aErrorCode); } \
  virtual void SetIPv4Disabled(void) override { return _to SetIPv4Disabled(); } \
  virtual void SetIPv6Disabled(void) override { return _to SetIPv6Disabled(); } \
  NS_IMETHOD GetCrossOriginOpenerPolicy(nsILoadInfo::CrossOriginOpenerPolicy *aCrossOriginOpenerPolicy) override { return _to GetCrossOriginOpenerPolicy(aCrossOriginOpenerPolicy); } \
  NS_IMETHOD ComputeCrossOriginOpenerPolicy(nsILoadInfo::CrossOriginOpenerPolicy aInitiatorPolicy, nsILoadInfo::CrossOriginOpenerPolicy *_retval) override { return _to ComputeCrossOriginOpenerPolicy(aInitiatorPolicy, _retval); } \
  NS_IMETHOD HasCrossOriginOpenerPolicyMismatch(bool *_retval) override { return _to HasCrossOriginOpenerPolicyMismatch(_retval); } \
  NS_IMETHOD GetResponseEmbedderPolicy(nsILoadInfo::CrossOriginEmbedderPolicy *_retval) override { return _to GetResponseEmbedderPolicy(_retval); } \
  virtual bool GetHasNonEmptySandboxingFlag() override { return _to GetHasNonEmptySandboxingFlag(); } \
  virtual void SetHasNonEmptySandboxingFlag(bool aHasNonEmptySandboxingFlag) override { return _to SetHasNonEmptySandboxingFlag(aHasNonEmptySandboxingFlag); } \
  virtual void DoDiagnosticAssertWhenOnStopNotCalledOnDestroy(void) override { return _to DoDiagnosticAssertWhenOnStopNotCalledOnDestroy(); } \
  [[nodiscard]] NS_IMETHOD SetWaitForHTTPSSVCRecord(void) override { return _to SetWaitForHTTPSSVCRecord(); } \
  [[nodiscard]] NS_IMETHOD GetSupportsHTTP3(bool *aSupportsHTTP3) override { return _to GetSupportsHTTP3(aSupportsHTTP3); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIHTTPCHANNELINTERNAL(_to) \
  [[nodiscard]] NS_IMETHOD GetDocumentURI(nsIURI **aDocumentURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDocumentURI(aDocumentURI); } \
  [[nodiscard]] NS_IMETHOD SetDocumentURI(nsIURI *aDocumentURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDocumentURI(aDocumentURI); } \
  [[nodiscard]] NS_IMETHOD GetRequestVersion(uint32_t *major, uint32_t *minor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRequestVersion(major, minor); } \
  [[nodiscard]] NS_IMETHOD GetResponseVersion(uint32_t *major, uint32_t *minor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResponseVersion(major, minor); } \
  [[nodiscard]] NS_IMETHOD TakeAllSecurityMessages(nsCOMArray<nsISecurityConsoleMessage> & aMessages) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TakeAllSecurityMessages(aMessages); } \
  [[nodiscard]] NS_IMETHOD SetCookie(const nsACString& aCookieHeader) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCookie(aCookieHeader); } \
  [[nodiscard]] NS_IMETHOD SetupFallbackChannel(const char * aFallbackKey) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetupFallbackChannel(aFallbackKey); } \
  [[nodiscard]] NS_IMETHOD GetIsAuthChannel(bool *aIsAuthChannel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsAuthChannel(aIsAuthChannel); } \
  [[nodiscard]] NS_IMETHOD GetThirdPartyFlags(uint32_t *aThirdPartyFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetThirdPartyFlags(aThirdPartyFlags); } \
  [[nodiscard]] NS_IMETHOD SetThirdPartyFlags(uint32_t aThirdPartyFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetThirdPartyFlags(aThirdPartyFlags); } \
  [[nodiscard]] NS_IMETHOD GetForceAllowThirdPartyCookie(bool *aForceAllowThirdPartyCookie) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetForceAllowThirdPartyCookie(aForceAllowThirdPartyCookie); } \
  [[nodiscard]] NS_IMETHOD SetForceAllowThirdPartyCookie(bool aForceAllowThirdPartyCookie) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetForceAllowThirdPartyCookie(aForceAllowThirdPartyCookie); } \
  [[nodiscard]] NS_IMETHOD GetChannelIsForDownload(bool *aChannelIsForDownload) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChannelIsForDownload(aChannelIsForDownload); } \
  [[nodiscard]] NS_IMETHOD SetChannelIsForDownload(bool aChannelIsForDownload) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetChannelIsForDownload(aChannelIsForDownload); } \
  [[nodiscard]] NS_IMETHOD GetLocalAddress(nsACString& aLocalAddress) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLocalAddress(aLocalAddress); } \
  [[nodiscard]] NS_IMETHOD GetLocalPort(int32_t *aLocalPort) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLocalPort(aLocalPort); } \
  [[nodiscard]] NS_IMETHOD GetRemoteAddress(nsACString& aRemoteAddress) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRemoteAddress(aRemoteAddress); } \
  [[nodiscard]] NS_IMETHOD GetRemotePort(int32_t *aRemotePort) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRemotePort(aRemotePort); } \
  [[nodiscard]] NS_IMETHOD SetCacheKeysRedirectChain(nsTArray<nsCString> * cacheKeys) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCacheKeysRedirectChain(cacheKeys); } \
  [[nodiscard]] NS_IMETHOD HTTPUpgrade(const nsACString& aProtocolName, nsIHttpUpgradeListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HTTPUpgrade(aProtocolName, aListener); } \
  [[nodiscard]] NS_IMETHOD SetConnectOnly(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetConnectOnly(); } \
  [[nodiscard]] NS_IMETHOD GetOnlyConnect(bool *aOnlyConnect) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOnlyConnect(aOnlyConnect); } \
  [[nodiscard]] NS_IMETHOD GetAllowSpdy(bool *aAllowSpdy) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllowSpdy(aAllowSpdy); } \
  [[nodiscard]] NS_IMETHOD SetAllowSpdy(bool aAllowSpdy) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAllowSpdy(aAllowSpdy); } \
  [[nodiscard]] NS_IMETHOD GetAllowHttp3(bool *aAllowHttp3) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllowHttp3(aAllowHttp3); } \
  [[nodiscard]] NS_IMETHOD SetAllowHttp3(bool aAllowHttp3) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAllowHttp3(aAllowHttp3); } \
  [[nodiscard]] NS_IMETHOD GetResponseTimeoutEnabled(bool *aResponseTimeoutEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResponseTimeoutEnabled(aResponseTimeoutEnabled); } \
  [[nodiscard]] NS_IMETHOD SetResponseTimeoutEnabled(bool aResponseTimeoutEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetResponseTimeoutEnabled(aResponseTimeoutEnabled); } \
  [[nodiscard]] NS_IMETHOD GetInitialRwin(uint32_t *aInitialRwin) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInitialRwin(aInitialRwin); } \
  [[nodiscard]] NS_IMETHOD SetInitialRwin(uint32_t aInitialRwin) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetInitialRwin(aInitialRwin); } \
  [[nodiscard]] NS_IMETHOD GetApiRedirectToURI(nsIURI **aApiRedirectToURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetApiRedirectToURI(aApiRedirectToURI); } \
  [[nodiscard]] NS_IMETHOD GetAllowAltSvc(bool *aAllowAltSvc) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllowAltSvc(aAllowAltSvc); } \
  [[nodiscard]] NS_IMETHOD SetAllowAltSvc(bool aAllowAltSvc) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAllowAltSvc(aAllowAltSvc); } \
  [[nodiscard]] NS_IMETHOD GetBeConservative(bool *aBeConservative) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBeConservative(aBeConservative); } \
  [[nodiscard]] NS_IMETHOD SetBeConservative(bool aBeConservative) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetBeConservative(aBeConservative); } \
  [[nodiscard]] NS_IMETHOD GetIsTRRServiceChannel(bool *aIsTRRServiceChannel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsTRRServiceChannel(aIsTRRServiceChannel); } \
  [[nodiscard]] NS_IMETHOD SetIsTRRServiceChannel(bool aIsTRRServiceChannel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetIsTRRServiceChannel(aIsTRRServiceChannel); } \
  [[nodiscard]] NS_IMETHOD GetIsResolvedByTRR(bool *aIsResolvedByTRR) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsResolvedByTRR(aIsResolvedByTRR); } \
  [[nodiscard]] NS_IMETHOD GetTlsFlags(uint32_t *aTlsFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTlsFlags(aTlsFlags); } \
  [[nodiscard]] NS_IMETHOD SetTlsFlags(uint32_t aTlsFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTlsFlags(aTlsFlags); } \
  [[nodiscard]] NS_IMETHOD GetLastModifiedTime(PRTime *aLastModifiedTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastModifiedTime(aLastModifiedTime); } \
  [[nodiscard]] NS_IMETHOD GetCorsIncludeCredentials(bool *aCorsIncludeCredentials) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCorsIncludeCredentials(aCorsIncludeCredentials); } \
  [[nodiscard]] NS_IMETHOD SetCorsIncludeCredentials(bool aCorsIncludeCredentials) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCorsIncludeCredentials(aCorsIncludeCredentials); } \
  [[nodiscard]] NS_IMETHOD GetCorsMode(uint32_t *aCorsMode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCorsMode(aCorsMode); } \
  [[nodiscard]] NS_IMETHOD SetCorsMode(uint32_t aCorsMode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCorsMode(aCorsMode); } \
  [[nodiscard]] NS_IMETHOD GetRedirectMode(uint32_t *aRedirectMode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRedirectMode(aRedirectMode); } \
  [[nodiscard]] NS_IMETHOD SetRedirectMode(uint32_t aRedirectMode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetRedirectMode(aRedirectMode); } \
  [[nodiscard]] NS_IMETHOD GetFetchCacheMode(uint32_t *aFetchCacheMode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFetchCacheMode(aFetchCacheMode); } \
  [[nodiscard]] NS_IMETHOD SetFetchCacheMode(uint32_t aFetchCacheMode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFetchCacheMode(aFetchCacheMode); } \
  [[nodiscard]] NS_IMETHOD GetTopWindowURI(nsIURI **aTopWindowURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTopWindowURI(aTopWindowURI); } \
  [[nodiscard]] NS_IMETHOD SetTopWindowURIIfUnknown(nsIURI *topWindowURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTopWindowURIIfUnknown(topWindowURI); } \
  [[nodiscard]] NS_IMETHOD GetProxyURI(nsIURI **aProxyURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProxyURI(aProxyURI); } \
  virtual void SetCorsPreflightParameters(const nsTArray<nsCString> & unsafeHeaders, bool shouldStripRequestBodyHeader) override; \
  virtual void SetAltDataForChild(bool aIsForChild) override; \
  virtual void DisableAltDataCache(void) override; \
  NS_IMETHOD GetBlockAuthPrompt(bool *aBlockAuthPrompt) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBlockAuthPrompt(aBlockAuthPrompt); } \
  NS_IMETHOD SetBlockAuthPrompt(bool aBlockAuthPrompt) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetBlockAuthPrompt(aBlockAuthPrompt); } \
  [[nodiscard]] NS_IMETHOD GetIntegrityMetadata(nsAString& aIntegrityMetadata) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIntegrityMetadata(aIntegrityMetadata); } \
  [[nodiscard]] NS_IMETHOD SetIntegrityMetadata(const nsAString& aIntegrityMetadata) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetIntegrityMetadata(aIntegrityMetadata); } \
  [[nodiscard]] NS_IMETHOD GetConnectionInfoHashKey(nsACString& aConnectionInfoHashKey) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetConnectionInfoHashKey(aConnectionInfoHashKey); } \
  NS_IMETHOD GetLastRedirectFlags(uint32_t *aLastRedirectFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastRedirectFlags(aLastRedirectFlags); } \
  NS_IMETHOD SetLastRedirectFlags(uint32_t aLastRedirectFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLastRedirectFlags(aLastRedirectFlags); } \
  NS_IMETHOD GetNavigationStartTimeStamp(mozilla::TimeStamp * aNavigationStartTimeStamp) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNavigationStartTimeStamp(aNavigationStartTimeStamp); } \
  NS_IMETHOD SetNavigationStartTimeStamp(mozilla::TimeStamp aNavigationStartTimeStamp) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetNavigationStartTimeStamp(aNavigationStartTimeStamp); } \
  NS_IMETHOD CancelByURLClassifier(nsresult aErrorCode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CancelByURLClassifier(aErrorCode); } \
  virtual void SetIPv4Disabled(void) override; \
  virtual void SetIPv6Disabled(void) override; \
  NS_IMETHOD GetCrossOriginOpenerPolicy(nsILoadInfo::CrossOriginOpenerPolicy *aCrossOriginOpenerPolicy) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCrossOriginOpenerPolicy(aCrossOriginOpenerPolicy); } \
  NS_IMETHOD ComputeCrossOriginOpenerPolicy(nsILoadInfo::CrossOriginOpenerPolicy aInitiatorPolicy, nsILoadInfo::CrossOriginOpenerPolicy *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ComputeCrossOriginOpenerPolicy(aInitiatorPolicy, _retval); } \
  NS_IMETHOD HasCrossOriginOpenerPolicyMismatch(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasCrossOriginOpenerPolicyMismatch(_retval); } \
  NS_IMETHOD GetResponseEmbedderPolicy(nsILoadInfo::CrossOriginEmbedderPolicy *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResponseEmbedderPolicy(_retval); } \
  virtual bool GetHasNonEmptySandboxingFlag() override; \
  virtual void SetHasNonEmptySandboxingFlag(bool aHasNonEmptySandboxingFlag) override; \
  virtual void DoDiagnosticAssertWhenOnStopNotCalledOnDestroy(void) override; \
  [[nodiscard]] NS_IMETHOD SetWaitForHTTPSSVCRecord(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetWaitForHTTPSSVCRecord(); } \
  [[nodiscard]] NS_IMETHOD GetSupportsHTTP3(bool *aSupportsHTTP3) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSupportsHTTP3(aSupportsHTTP3); } 


#endif /* __gen_nsIHttpChannelInternal_h__ */
