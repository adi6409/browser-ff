/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIProxyInfo.idl
 */

#ifndef __gen_nsIProxyInfo_h__
#define __gen_nsIProxyInfo_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIProxyInfo */
#define NS_IPROXYINFO_IID_STR "63fff172-2564-4138-96c6-3ae7d245fbed"

#define NS_IPROXYINFO_IID \
  {0x63fff172, 0x2564, 0x4138, \
    { 0x96, 0xc6, 0x3a, 0xe7, 0xd2, 0x45, 0xfb, 0xed }}

class NS_NO_VTABLE nsIProxyInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPROXYINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIProxyInfo;

  /* readonly attribute AUTF8String host; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHost(nsACString& aHost) = 0;

  /* readonly attribute long port; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPort(int32_t *aPort) = 0;

  /* readonly attribute ACString type; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetType(nsACString& aType) = 0;

  /* readonly attribute unsigned long flags; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFlags(uint32_t *aFlags) = 0;

  /* attribute unsigned long resolveFlags; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetResolveFlags(uint32_t *aResolveFlags) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetResolveFlags(uint32_t aResolveFlags) = 0;

  /* readonly attribute ACString username; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUsername(nsACString& aUsername) = 0;

  /* readonly attribute ACString password; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPassword(nsACString& aPassword) = 0;

  /* readonly attribute unsigned long failoverTimeout; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFailoverTimeout(uint32_t *aFailoverTimeout) = 0;

  /* attribute nsIProxyInfo failoverProxy; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFailoverProxy(nsIProxyInfo **aFailoverProxy) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetFailoverProxy(nsIProxyInfo *aFailoverProxy) = 0;

  /* readonly attribute ACString proxyAuthorizationHeader; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetProxyAuthorizationHeader(nsACString& aProxyAuthorizationHeader) = 0;

  /* readonly attribute ACString connectionIsolationKey; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetConnectionIsolationKey(nsACString& aConnectionIsolationKey) = 0;

  enum {
    TRANSPARENT_PROXY_RESOLVES_HOST = 1U
  };

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIProxyInfo, NS_IPROXYINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPROXYINFO \
  NS_IMETHOD GetHost(nsACString& aHost) override; \
  NS_IMETHOD GetPort(int32_t *aPort) override; \
  NS_IMETHOD GetType(nsACString& aType) override; \
  NS_IMETHOD GetFlags(uint32_t *aFlags) override; \
  NS_IMETHOD GetResolveFlags(uint32_t *aResolveFlags) override; \
  NS_IMETHOD SetResolveFlags(uint32_t aResolveFlags) override; \
  NS_IMETHOD GetUsername(nsACString& aUsername) override; \
  NS_IMETHOD GetPassword(nsACString& aPassword) override; \
  NS_IMETHOD GetFailoverTimeout(uint32_t *aFailoverTimeout) override; \
  NS_IMETHOD GetFailoverProxy(nsIProxyInfo **aFailoverProxy) override; \
  NS_IMETHOD SetFailoverProxy(nsIProxyInfo *aFailoverProxy) override; \
  NS_IMETHOD GetProxyAuthorizationHeader(nsACString& aProxyAuthorizationHeader) override; \
  NS_IMETHOD GetConnectionIsolationKey(nsACString& aConnectionIsolationKey) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPROXYINFO \
  nsresult GetHost(nsACString& aHost); \
  nsresult GetPort(int32_t *aPort); \
  nsresult GetType(nsACString& aType); \
  nsresult GetFlags(uint32_t *aFlags); \
  nsresult GetResolveFlags(uint32_t *aResolveFlags); \
  nsresult SetResolveFlags(uint32_t aResolveFlags); \
  nsresult GetUsername(nsACString& aUsername); \
  nsresult GetPassword(nsACString& aPassword); \
  nsresult GetFailoverTimeout(uint32_t *aFailoverTimeout); \
  nsresult GetFailoverProxy(nsIProxyInfo **aFailoverProxy); \
  nsresult SetFailoverProxy(nsIProxyInfo *aFailoverProxy); \
  nsresult GetProxyAuthorizationHeader(nsACString& aProxyAuthorizationHeader); \
  nsresult GetConnectionIsolationKey(nsACString& aConnectionIsolationKey); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPROXYINFO(_to) \
  NS_IMETHOD GetHost(nsACString& aHost) override { return _to GetHost(aHost); } \
  NS_IMETHOD GetPort(int32_t *aPort) override { return _to GetPort(aPort); } \
  NS_IMETHOD GetType(nsACString& aType) override { return _to GetType(aType); } \
  NS_IMETHOD GetFlags(uint32_t *aFlags) override { return _to GetFlags(aFlags); } \
  NS_IMETHOD GetResolveFlags(uint32_t *aResolveFlags) override { return _to GetResolveFlags(aResolveFlags); } \
  NS_IMETHOD SetResolveFlags(uint32_t aResolveFlags) override { return _to SetResolveFlags(aResolveFlags); } \
  NS_IMETHOD GetUsername(nsACString& aUsername) override { return _to GetUsername(aUsername); } \
  NS_IMETHOD GetPassword(nsACString& aPassword) override { return _to GetPassword(aPassword); } \
  NS_IMETHOD GetFailoverTimeout(uint32_t *aFailoverTimeout) override { return _to GetFailoverTimeout(aFailoverTimeout); } \
  NS_IMETHOD GetFailoverProxy(nsIProxyInfo **aFailoverProxy) override { return _to GetFailoverProxy(aFailoverProxy); } \
  NS_IMETHOD SetFailoverProxy(nsIProxyInfo *aFailoverProxy) override { return _to SetFailoverProxy(aFailoverProxy); } \
  NS_IMETHOD GetProxyAuthorizationHeader(nsACString& aProxyAuthorizationHeader) override { return _to GetProxyAuthorizationHeader(aProxyAuthorizationHeader); } \
  NS_IMETHOD GetConnectionIsolationKey(nsACString& aConnectionIsolationKey) override { return _to GetConnectionIsolationKey(aConnectionIsolationKey); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPROXYINFO(_to) \
  NS_IMETHOD GetHost(nsACString& aHost) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHost(aHost); } \
  NS_IMETHOD GetPort(int32_t *aPort) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPort(aPort); } \
  NS_IMETHOD GetType(nsACString& aType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetType(aType); } \
  NS_IMETHOD GetFlags(uint32_t *aFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFlags(aFlags); } \
  NS_IMETHOD GetResolveFlags(uint32_t *aResolveFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResolveFlags(aResolveFlags); } \
  NS_IMETHOD SetResolveFlags(uint32_t aResolveFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetResolveFlags(aResolveFlags); } \
  NS_IMETHOD GetUsername(nsACString& aUsername) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUsername(aUsername); } \
  NS_IMETHOD GetPassword(nsACString& aPassword) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPassword(aPassword); } \
  NS_IMETHOD GetFailoverTimeout(uint32_t *aFailoverTimeout) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFailoverTimeout(aFailoverTimeout); } \
  NS_IMETHOD GetFailoverProxy(nsIProxyInfo **aFailoverProxy) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFailoverProxy(aFailoverProxy); } \
  NS_IMETHOD SetFailoverProxy(nsIProxyInfo *aFailoverProxy) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFailoverProxy(aFailoverProxy); } \
  NS_IMETHOD GetProxyAuthorizationHeader(nsACString& aProxyAuthorizationHeader) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProxyAuthorizationHeader(aProxyAuthorizationHeader); } \
  NS_IMETHOD GetConnectionIsolationKey(nsACString& aConnectionIsolationKey) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetConnectionIsolationKey(aConnectionIsolationKey); } \


#endif /* __gen_nsIProxyInfo_h__ */
