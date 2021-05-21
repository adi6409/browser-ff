/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIProtocolProxyService.idl
 */

#ifndef __gen_nsIProtocolProxyService_h__
#define __gen_nsIProtocolProxyService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsICancelable; /* forward declaration */

class nsIProtocolProxyCallback; /* forward declaration */

class nsIProtocolProxyFilter; /* forward declaration */

class nsIProtocolProxyChannelFilter; /* forward declaration */

class nsIProxyInfo; /* forward declaration */

class nsIChannel; /* forward declaration */

class nsIURI; /* forward declaration */

class nsISerialEventTarget; /* forward declaration */


/* starting interface:    nsIProtocolProxyService */
#define NS_IPROTOCOLPROXYSERVICE_IID_STR "ef57c8b6-e09d-4cd4-9222-2a5d2402e15d"

#define NS_IPROTOCOLPROXYSERVICE_IID \
  {0xef57c8b6, 0xe09d, 0x4cd4, \
    { 0x92, 0x22, 0x2a, 0x5d, 0x24, 0x02, 0xe1, 0x5d }}

class NS_NO_VTABLE nsIProtocolProxyService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPROTOCOLPROXYSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIProtocolProxyService;

  enum {
    RESOLVE_PREFER_SOCKS_PROXY = 2U,
    RESOLVE_IGNORE_URI_SCHEME = 4U,
    RESOLVE_PREFER_HTTPS_PROXY = 12U,
    RESOLVE_ALWAYS_TUNNEL = 16U
  };

  /* nsICancelable asyncResolve (in nsISupports aChannelOrURI, in unsigned long aFlags, in nsIProtocolProxyCallback aCallback, [optional] in nsISerialEventTarget aMainThreadTarget); */
  NS_IMETHOD AsyncResolve(nsISupports *aChannelOrURI, uint32_t aFlags, nsIProtocolProxyCallback *aCallback, nsISerialEventTarget *aMainThreadTarget, nsICancelable **_retval) = 0;

  /* nsIProxyInfo newProxyInfo (in ACString aType, in AUTF8String aHost, in long aPort, in ACString aProxyAuthorizationHeader, in ACString aConnectionIsolationKey, in unsigned long aFlags, in unsigned long aFailoverTimeout, in nsIProxyInfo aFailoverProxy); */
  NS_IMETHOD NewProxyInfo(const nsACString& aType, const nsACString& aHost, int32_t aPort, const nsACString& aProxyAuthorizationHeader, const nsACString& aConnectionIsolationKey, uint32_t aFlags, uint32_t aFailoverTimeout, nsIProxyInfo *aFailoverProxy, nsIProxyInfo **_retval) = 0;

  /* nsIProxyInfo newProxyInfoWithAuth (in ACString aType, in AUTF8String aHost, in long aPort, in ACString aUsername, in ACString aPassword, in ACString aProxyAuthorizationHeader, in ACString aConnectionIsolationKey, in unsigned long aFlags, in unsigned long aFailoverTimeout, in nsIProxyInfo aFailoverProxy); */
  NS_IMETHOD NewProxyInfoWithAuth(const nsACString& aType, const nsACString& aHost, int32_t aPort, const nsACString& aUsername, const nsACString& aPassword, const nsACString& aProxyAuthorizationHeader, const nsACString& aConnectionIsolationKey, uint32_t aFlags, uint32_t aFailoverTimeout, nsIProxyInfo *aFailoverProxy, nsIProxyInfo **_retval) = 0;

  /* nsIProxyInfo getFailoverForProxy (in nsIProxyInfo aProxyInfo, in nsIURI aURI, in nsresult aReason); */
  NS_IMETHOD GetFailoverForProxy(nsIProxyInfo *aProxyInfo, nsIURI *aURI, nsresult aReason, nsIProxyInfo **_retval) = 0;

  /* void registerFilter (in nsIProtocolProxyFilter aFilter, in unsigned long aPosition); */
  NS_IMETHOD RegisterFilter(nsIProtocolProxyFilter *aFilter, uint32_t aPosition) = 0;

  /* void registerChannelFilter (in nsIProtocolProxyChannelFilter aFilter, in unsigned long aPosition); */
  NS_IMETHOD RegisterChannelFilter(nsIProtocolProxyChannelFilter *aFilter, uint32_t aPosition) = 0;

  /* void unregisterFilter (in nsIProtocolProxyFilter aFilter); */
  NS_IMETHOD UnregisterFilter(nsIProtocolProxyFilter *aFilter) = 0;

  /* void unregisterChannelFilter (in nsIProtocolProxyChannelFilter aFilter); */
  NS_IMETHOD UnregisterChannelFilter(nsIProtocolProxyChannelFilter *aFilter) = 0;

  enum {
    PROXYCONFIG_DIRECT = 0U,
    PROXYCONFIG_MANUAL = 1U,
    PROXYCONFIG_PAC = 2U,
    PROXYCONFIG_WPAD = 4U,
    PROXYCONFIG_SYSTEM = 5U
  };

  /* readonly attribute unsigned long proxyConfigType; */
  NS_IMETHOD GetProxyConfigType(uint32_t *aProxyConfigType) = 0;

  /* [nostdcall,notxpcom] readonly attribute boolean isPACLoading; */
  virtual bool GetIsPACLoading() = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIProtocolProxyService, NS_IPROTOCOLPROXYSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPROTOCOLPROXYSERVICE \
  NS_IMETHOD AsyncResolve(nsISupports *aChannelOrURI, uint32_t aFlags, nsIProtocolProxyCallback *aCallback, nsISerialEventTarget *aMainThreadTarget, nsICancelable **_retval) override; \
  NS_IMETHOD NewProxyInfo(const nsACString& aType, const nsACString& aHost, int32_t aPort, const nsACString& aProxyAuthorizationHeader, const nsACString& aConnectionIsolationKey, uint32_t aFlags, uint32_t aFailoverTimeout, nsIProxyInfo *aFailoverProxy, nsIProxyInfo **_retval) override; \
  NS_IMETHOD NewProxyInfoWithAuth(const nsACString& aType, const nsACString& aHost, int32_t aPort, const nsACString& aUsername, const nsACString& aPassword, const nsACString& aProxyAuthorizationHeader, const nsACString& aConnectionIsolationKey, uint32_t aFlags, uint32_t aFailoverTimeout, nsIProxyInfo *aFailoverProxy, nsIProxyInfo **_retval) override; \
  NS_IMETHOD GetFailoverForProxy(nsIProxyInfo *aProxyInfo, nsIURI *aURI, nsresult aReason, nsIProxyInfo **_retval) override; \
  NS_IMETHOD RegisterFilter(nsIProtocolProxyFilter *aFilter, uint32_t aPosition) override; \
  NS_IMETHOD RegisterChannelFilter(nsIProtocolProxyChannelFilter *aFilter, uint32_t aPosition) override; \
  NS_IMETHOD UnregisterFilter(nsIProtocolProxyFilter *aFilter) override; \
  NS_IMETHOD UnregisterChannelFilter(nsIProtocolProxyChannelFilter *aFilter) override; \
  NS_IMETHOD GetProxyConfigType(uint32_t *aProxyConfigType) override; \
  virtual bool GetIsPACLoading() override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPROTOCOLPROXYSERVICE \
  nsresult AsyncResolve(nsISupports *aChannelOrURI, uint32_t aFlags, nsIProtocolProxyCallback *aCallback, nsISerialEventTarget *aMainThreadTarget, nsICancelable **_retval); \
  nsresult NewProxyInfo(const nsACString& aType, const nsACString& aHost, int32_t aPort, const nsACString& aProxyAuthorizationHeader, const nsACString& aConnectionIsolationKey, uint32_t aFlags, uint32_t aFailoverTimeout, nsIProxyInfo *aFailoverProxy, nsIProxyInfo **_retval); \
  nsresult NewProxyInfoWithAuth(const nsACString& aType, const nsACString& aHost, int32_t aPort, const nsACString& aUsername, const nsACString& aPassword, const nsACString& aProxyAuthorizationHeader, const nsACString& aConnectionIsolationKey, uint32_t aFlags, uint32_t aFailoverTimeout, nsIProxyInfo *aFailoverProxy, nsIProxyInfo **_retval); \
  nsresult GetFailoverForProxy(nsIProxyInfo *aProxyInfo, nsIURI *aURI, nsresult aReason, nsIProxyInfo **_retval); \
  nsresult RegisterFilter(nsIProtocolProxyFilter *aFilter, uint32_t aPosition); \
  nsresult RegisterChannelFilter(nsIProtocolProxyChannelFilter *aFilter, uint32_t aPosition); \
  nsresult UnregisterFilter(nsIProtocolProxyFilter *aFilter); \
  nsresult UnregisterChannelFilter(nsIProtocolProxyChannelFilter *aFilter); \
  nsresult GetProxyConfigType(uint32_t *aProxyConfigType); \
  bool GetIsPACLoading(); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPROTOCOLPROXYSERVICE(_to) \
  NS_IMETHOD AsyncResolve(nsISupports *aChannelOrURI, uint32_t aFlags, nsIProtocolProxyCallback *aCallback, nsISerialEventTarget *aMainThreadTarget, nsICancelable **_retval) override { return _to AsyncResolve(aChannelOrURI, aFlags, aCallback, aMainThreadTarget, _retval); } \
  NS_IMETHOD NewProxyInfo(const nsACString& aType, const nsACString& aHost, int32_t aPort, const nsACString& aProxyAuthorizationHeader, const nsACString& aConnectionIsolationKey, uint32_t aFlags, uint32_t aFailoverTimeout, nsIProxyInfo *aFailoverProxy, nsIProxyInfo **_retval) override { return _to NewProxyInfo(aType, aHost, aPort, aProxyAuthorizationHeader, aConnectionIsolationKey, aFlags, aFailoverTimeout, aFailoverProxy, _retval); } \
  NS_IMETHOD NewProxyInfoWithAuth(const nsACString& aType, const nsACString& aHost, int32_t aPort, const nsACString& aUsername, const nsACString& aPassword, const nsACString& aProxyAuthorizationHeader, const nsACString& aConnectionIsolationKey, uint32_t aFlags, uint32_t aFailoverTimeout, nsIProxyInfo *aFailoverProxy, nsIProxyInfo **_retval) override { return _to NewProxyInfoWithAuth(aType, aHost, aPort, aUsername, aPassword, aProxyAuthorizationHeader, aConnectionIsolationKey, aFlags, aFailoverTimeout, aFailoverProxy, _retval); } \
  NS_IMETHOD GetFailoverForProxy(nsIProxyInfo *aProxyInfo, nsIURI *aURI, nsresult aReason, nsIProxyInfo **_retval) override { return _to GetFailoverForProxy(aProxyInfo, aURI, aReason, _retval); } \
  NS_IMETHOD RegisterFilter(nsIProtocolProxyFilter *aFilter, uint32_t aPosition) override { return _to RegisterFilter(aFilter, aPosition); } \
  NS_IMETHOD RegisterChannelFilter(nsIProtocolProxyChannelFilter *aFilter, uint32_t aPosition) override { return _to RegisterChannelFilter(aFilter, aPosition); } \
  NS_IMETHOD UnregisterFilter(nsIProtocolProxyFilter *aFilter) override { return _to UnregisterFilter(aFilter); } \
  NS_IMETHOD UnregisterChannelFilter(nsIProtocolProxyChannelFilter *aFilter) override { return _to UnregisterChannelFilter(aFilter); } \
  NS_IMETHOD GetProxyConfigType(uint32_t *aProxyConfigType) override { return _to GetProxyConfigType(aProxyConfigType); } \
  virtual bool GetIsPACLoading() override { return _to GetIsPACLoading(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPROTOCOLPROXYSERVICE(_to) \
  NS_IMETHOD AsyncResolve(nsISupports *aChannelOrURI, uint32_t aFlags, nsIProtocolProxyCallback *aCallback, nsISerialEventTarget *aMainThreadTarget, nsICancelable **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncResolve(aChannelOrURI, aFlags, aCallback, aMainThreadTarget, _retval); } \
  NS_IMETHOD NewProxyInfo(const nsACString& aType, const nsACString& aHost, int32_t aPort, const nsACString& aProxyAuthorizationHeader, const nsACString& aConnectionIsolationKey, uint32_t aFlags, uint32_t aFailoverTimeout, nsIProxyInfo *aFailoverProxy, nsIProxyInfo **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewProxyInfo(aType, aHost, aPort, aProxyAuthorizationHeader, aConnectionIsolationKey, aFlags, aFailoverTimeout, aFailoverProxy, _retval); } \
  NS_IMETHOD NewProxyInfoWithAuth(const nsACString& aType, const nsACString& aHost, int32_t aPort, const nsACString& aUsername, const nsACString& aPassword, const nsACString& aProxyAuthorizationHeader, const nsACString& aConnectionIsolationKey, uint32_t aFlags, uint32_t aFailoverTimeout, nsIProxyInfo *aFailoverProxy, nsIProxyInfo **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewProxyInfoWithAuth(aType, aHost, aPort, aUsername, aPassword, aProxyAuthorizationHeader, aConnectionIsolationKey, aFlags, aFailoverTimeout, aFailoverProxy, _retval); } \
  NS_IMETHOD GetFailoverForProxy(nsIProxyInfo *aProxyInfo, nsIURI *aURI, nsresult aReason, nsIProxyInfo **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFailoverForProxy(aProxyInfo, aURI, aReason, _retval); } \
  NS_IMETHOD RegisterFilter(nsIProtocolProxyFilter *aFilter, uint32_t aPosition) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterFilter(aFilter, aPosition); } \
  NS_IMETHOD RegisterChannelFilter(nsIProtocolProxyChannelFilter *aFilter, uint32_t aPosition) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterChannelFilter(aFilter, aPosition); } \
  NS_IMETHOD UnregisterFilter(nsIProtocolProxyFilter *aFilter) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UnregisterFilter(aFilter); } \
  NS_IMETHOD UnregisterChannelFilter(nsIProtocolProxyChannelFilter *aFilter) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UnregisterChannelFilter(aFilter); } \
  NS_IMETHOD GetProxyConfigType(uint32_t *aProxyConfigType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProxyConfigType(aProxyConfigType); } \
  virtual bool GetIsPACLoading() override; 


#endif /* __gen_nsIProtocolProxyService_h__ */
