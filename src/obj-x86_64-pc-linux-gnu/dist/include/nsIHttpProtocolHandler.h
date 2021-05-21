/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIHttpProtocolHandler.idl
 */

#ifndef __gen_nsIHttpProtocolHandler_h__
#define __gen_nsIHttpProtocolHandler_h__


#ifndef __gen_nsIProxiedProtocolHandler_h__
#include "nsIProxiedProtocolHandler.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/Value.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
namespace net {
class HSTSDataCallbackWrapper;
}
}

/* starting interface:    nsIHttpProtocolHandler */
#define NS_IHTTPPROTOCOLHANDLER_IID_STR "c48126d9-2ddd-485b-a51a-378e917e75f8"

#define NS_IHTTPPROTOCOLHANDLER_IID \
  {0xc48126d9, 0x2ddd, 0x485b, \
    { 0xa5, 0x1a, 0x37, 0x8e, 0x91, 0x7e, 0x75, 0xf8 }}

class NS_NO_VTABLE nsIHttpProtocolHandler : public nsIProxiedProtocolHandler {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IHTTPPROTOCOLHANDLER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIHttpProtocolHandler;

  /* [must_use] readonly attribute ACString userAgent; */
  [[nodiscard]] NS_IMETHOD GetUserAgent(nsACString& aUserAgent) = 0;

  /* [must_use] readonly attribute ACString appName; */
  [[nodiscard]] NS_IMETHOD GetAppName(nsACString& aAppName) = 0;

  /* [must_use] readonly attribute ACString appVersion; */
  [[nodiscard]] NS_IMETHOD GetAppVersion(nsACString& aAppVersion) = 0;

  /* [must_use] readonly attribute ACString platform; */
  [[nodiscard]] NS_IMETHOD GetPlatform(nsACString& aPlatform) = 0;

  /* [must_use] readonly attribute ACString oscpu; */
  [[nodiscard]] NS_IMETHOD GetOscpu(nsACString& aOscpu) = 0;

  /* [must_use] readonly attribute ACString misc; */
  [[nodiscard]] NS_IMETHOD GetMisc(nsACString& aMisc) = 0;

  /* [must_use] readonly attribute Array<ACString> altSvcCacheKeys; */
  [[nodiscard]] NS_IMETHOD GetAltSvcCacheKeys(nsTArray<nsCString >& aAltSvcCacheKeys) = 0;

  /* [must_use] readonly attribute Array<ACString> authCacheKeys; */
  [[nodiscard]] NS_IMETHOD GetAuthCacheKeys(nsTArray<nsCString >& aAuthCacheKeys) = 0;

  /* [implicit_jscontext] Promise EnsureHSTSDataReady (); */
  NS_IMETHOD EnsureHSTSDataReady(JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

  /* [noscript] void EnsureHSTSDataReadyNative (in HSTSDataCallbackWrapperAlreadyAddRefed aCallback); */
  NS_IMETHOD EnsureHSTSDataReadyNative(RefPtr<mozilla::net::HSTSDataCallbackWrapper> aCallback) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIHttpProtocolHandler, NS_IHTTPPROTOCOLHANDLER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIHTTPPROTOCOLHANDLER \
  [[nodiscard]] NS_IMETHOD GetUserAgent(nsACString& aUserAgent) override; \
  [[nodiscard]] NS_IMETHOD GetAppName(nsACString& aAppName) override; \
  [[nodiscard]] NS_IMETHOD GetAppVersion(nsACString& aAppVersion) override; \
  [[nodiscard]] NS_IMETHOD GetPlatform(nsACString& aPlatform) override; \
  [[nodiscard]] NS_IMETHOD GetOscpu(nsACString& aOscpu) override; \
  [[nodiscard]] NS_IMETHOD GetMisc(nsACString& aMisc) override; \
  [[nodiscard]] NS_IMETHOD GetAltSvcCacheKeys(nsTArray<nsCString >& aAltSvcCacheKeys) override; \
  [[nodiscard]] NS_IMETHOD GetAuthCacheKeys(nsTArray<nsCString >& aAuthCacheKeys) override; \
  NS_IMETHOD EnsureHSTSDataReady(JSContext* cx, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD EnsureHSTSDataReadyNative(RefPtr<mozilla::net::HSTSDataCallbackWrapper> aCallback) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIHTTPPROTOCOLHANDLER \
  [[nodiscard]] nsresult GetUserAgent(nsACString& aUserAgent); \
  [[nodiscard]] nsresult GetAppName(nsACString& aAppName); \
  [[nodiscard]] nsresult GetAppVersion(nsACString& aAppVersion); \
  [[nodiscard]] nsresult GetPlatform(nsACString& aPlatform); \
  [[nodiscard]] nsresult GetOscpu(nsACString& aOscpu); \
  [[nodiscard]] nsresult GetMisc(nsACString& aMisc); \
  [[nodiscard]] nsresult GetAltSvcCacheKeys(nsTArray<nsCString >& aAltSvcCacheKeys); \
  [[nodiscard]] nsresult GetAuthCacheKeys(nsTArray<nsCString >& aAuthCacheKeys); \
  nsresult EnsureHSTSDataReady(JSContext* cx, ::mozilla::dom::Promise * * _retval); \
  nsresult EnsureHSTSDataReadyNative(RefPtr<mozilla::net::HSTSDataCallbackWrapper> aCallback); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIHTTPPROTOCOLHANDLER(_to) \
  [[nodiscard]] NS_IMETHOD GetUserAgent(nsACString& aUserAgent) override { return _to GetUserAgent(aUserAgent); } \
  [[nodiscard]] NS_IMETHOD GetAppName(nsACString& aAppName) override { return _to GetAppName(aAppName); } \
  [[nodiscard]] NS_IMETHOD GetAppVersion(nsACString& aAppVersion) override { return _to GetAppVersion(aAppVersion); } \
  [[nodiscard]] NS_IMETHOD GetPlatform(nsACString& aPlatform) override { return _to GetPlatform(aPlatform); } \
  [[nodiscard]] NS_IMETHOD GetOscpu(nsACString& aOscpu) override { return _to GetOscpu(aOscpu); } \
  [[nodiscard]] NS_IMETHOD GetMisc(nsACString& aMisc) override { return _to GetMisc(aMisc); } \
  [[nodiscard]] NS_IMETHOD GetAltSvcCacheKeys(nsTArray<nsCString >& aAltSvcCacheKeys) override { return _to GetAltSvcCacheKeys(aAltSvcCacheKeys); } \
  [[nodiscard]] NS_IMETHOD GetAuthCacheKeys(nsTArray<nsCString >& aAuthCacheKeys) override { return _to GetAuthCacheKeys(aAuthCacheKeys); } \
  NS_IMETHOD EnsureHSTSDataReady(JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to EnsureHSTSDataReady(cx, _retval); } \
  NS_IMETHOD EnsureHSTSDataReadyNative(RefPtr<mozilla::net::HSTSDataCallbackWrapper> aCallback) override { return _to EnsureHSTSDataReadyNative(aCallback); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIHTTPPROTOCOLHANDLER(_to) \
  [[nodiscard]] NS_IMETHOD GetUserAgent(nsACString& aUserAgent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUserAgent(aUserAgent); } \
  [[nodiscard]] NS_IMETHOD GetAppName(nsACString& aAppName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAppName(aAppName); } \
  [[nodiscard]] NS_IMETHOD GetAppVersion(nsACString& aAppVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAppVersion(aAppVersion); } \
  [[nodiscard]] NS_IMETHOD GetPlatform(nsACString& aPlatform) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPlatform(aPlatform); } \
  [[nodiscard]] NS_IMETHOD GetOscpu(nsACString& aOscpu) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOscpu(aOscpu); } \
  [[nodiscard]] NS_IMETHOD GetMisc(nsACString& aMisc) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMisc(aMisc); } \
  [[nodiscard]] NS_IMETHOD GetAltSvcCacheKeys(nsTArray<nsCString >& aAltSvcCacheKeys) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAltSvcCacheKeys(aAltSvcCacheKeys); } \
  [[nodiscard]] NS_IMETHOD GetAuthCacheKeys(nsTArray<nsCString >& aAuthCacheKeys) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAuthCacheKeys(aAuthCacheKeys); } \
  NS_IMETHOD EnsureHSTSDataReady(JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EnsureHSTSDataReady(cx, _retval); } \
  NS_IMETHOD EnsureHSTSDataReadyNative(RefPtr<mozilla::net::HSTSDataCallbackWrapper> aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EnsureHSTSDataReadyNative(aCallback); } 

// ----------- Categories -----------
/**
 * At initialization time, the HTTP handler will initialize each service
 * registered under this category:
 */
#define NS_HTTP_STARTUP_CATEGORY "http-startup-category"
// ----------- Observer topics -----------
/**
 * nsIObserver notification corresponding to startup category.  Services
 * registered under the startup category will receive this observer topic at
 * startup if they implement nsIObserver.  The "subject" of the notification
 * is the nsIHttpProtocolHandler instance.
 */
#define NS_HTTP_STARTUP_TOPIC "http-startup"
/**
 * Called when asyncOpen synchronously failes e.g. because of any synchronously
 * performed security checks.  This only fires on the child process, but if
 * needed can be implemented also on the parent process.
 */
#define NS_HTTP_ON_FAILED_OPENING_REQUEST_TOPIC "http-on-failed-opening-request"
 /**
 * This observer topic is notified when an HTTP channel is opened.
 * It is similar to http-on-modify-request, except that
 * 1) The notification is guaranteed to occur before on-modify-request, during
 *    the AsyncOpen call itself.
 * 2) It only occurs for the initial open of a channel, not for internal
 *    asyncOpens that happen during redirects, etc.
 * 3) Some information (most notably nsIProxiedChannel.proxyInfo) may not be set
 *    on the channel object yet.
 *
 * The "subject" of the notification is the nsIHttpChannel instance.
 *
 * Generally the 'http-on-modify-request' notification is preferred unless the
 * synchronous, during-asyncOpen behavior that this notification provides is
 * required.
 */
#define NS_HTTP_ON_OPENING_REQUEST_TOPIC "http-on-opening-request"
 /**
 * This observer topic is notified when a document channel is opened.
 * It is similar to http-on-opening-request.
 */
#define NS_DOCUMENT_ON_OPENING_REQUEST_TOPIC "document-on-opening-request"
/**
 * Before an HTTP request is sent to the server, this observer topic is
 * notified.  The observer of this topic can then choose to set any additional
 * headers for this request before the request is actually sent to the server.
 * The "subject" of the notification is the nsIHttpChannel instance.
 */
#define NS_HTTP_ON_MODIFY_REQUEST_TOPIC "http-on-modify-request"
/**
  * Before an HTTP request is sent to the server via a document channel this
  * observer topic is notified.
  * It is similar to http-on-modify-request.
*/
#define NS_DOCUMENT_ON_MODIFY_REQUEST_TOPIC "document-on-modify-request"
/**
 * Before an HTTP connection to the server is created, this observer topic is
 * notified.  This observer happens after HSTS upgrades, etc. are set, providing
 * access to the full set of request headers. The observer of this topic can
 * choose to set any additional headers for this request before the request is
 * actually sent to the server. The "subject" of the notification is the
 * nsIHttpChannel instance.
 */
#define NS_HTTP_ON_BEFORE_CONNECT_TOPIC "http-on-before-connect"
/**
 * After an HTTP server response is received, this observer topic is notified.
 * The observer of this topic can interrogate the response.  The "subject" of
 * the notification is the nsIHttpChannel instance.
 */
#define NS_HTTP_ON_EXAMINE_RESPONSE_TOPIC "http-on-examine-response"
/**
 * The observer of this topic is notified after the received HTTP response
 * is merged with data from the browser cache.  This means that, among other
 * things, the Content-Type header will be set correctly.
 */
#define NS_HTTP_ON_EXAMINE_MERGED_RESPONSE_TOPIC "http-on-examine-merged-response"
/**
 * The observer of this topic is notified about a background revalidation that
 * started by hitting a request that fell into stale-while-revalidate window.
 * This notification points to the channel that performed the revalidation and
 * after this notification the cache entry has been validated or updated.
 */
#define NS_HTTP_ON_BACKGROUND_REVALIDATION "http-on-background-revalidation"
/**
 * The observer of this topic is notified before data is read from the cache.
 * The notification is sent if and only if there is no network communication
 * at all.
 */
#define NS_HTTP_ON_EXAMINE_CACHED_RESPONSE_TOPIC "http-on-examine-cached-response"
/**
 * This topic is notified for every http channel right after it called
 * OnStopRequest on its listener, regardless whether it was finished
 * successfully, failed or has been canceled.
 */
#define NS_HTTP_ON_STOP_REQUEST_TOPIC "http-on-stop-request"

#endif /* __gen_nsIHttpProtocolHandler_h__ */
