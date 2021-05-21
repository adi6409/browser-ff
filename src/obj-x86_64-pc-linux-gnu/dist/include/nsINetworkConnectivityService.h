/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsINetworkConnectivityService.idl
 */

#ifndef __gen_nsINetworkConnectivityService_h__
#define __gen_nsINetworkConnectivityService_h__


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

/* starting interface:    nsINetworkConnectivityService */
#define NS_INETWORKCONNECTIVITYSERVICE_IID_STR "2693457e-3ba5-4455-991f-5350946adb12"

#define NS_INETWORKCONNECTIVITYSERVICE_IID \
  {0x2693457e, 0x3ba5, 0x4455, \
    { 0x99, 0x1f, 0x53, 0x50, 0x94, 0x6a, 0xdb, 0x12 }}

class NS_NO_VTABLE nsINetworkConnectivityService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INETWORKCONNECTIVITYSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINetworkConnectivityService;

  enum ConnectivityState : uint32_t {
    UNKNOWN = 0,
    OK = 1,
    NOT_AVAILABLE = 2,
  };

  /* [infallible] readonly attribute nsINetworkConnectivityService_ConnectivityState DNSv4; */
  NS_IMETHOD GetDNSv4(nsINetworkConnectivityService::ConnectivityState *aDNSv4) = 0;
  inline nsINetworkConnectivityService::ConnectivityState  GetDNSv4()
  {
    nsINetworkConnectivityService::ConnectivityState result;
    mozilla::DebugOnly<nsresult> rv = GetDNSv4(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute nsINetworkConnectivityService_ConnectivityState DNSv6; */
  NS_IMETHOD GetDNSv6(nsINetworkConnectivityService::ConnectivityState *aDNSv6) = 0;
  inline nsINetworkConnectivityService::ConnectivityState  GetDNSv6()
  {
    nsINetworkConnectivityService::ConnectivityState result;
    mozilla::DebugOnly<nsresult> rv = GetDNSv6(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute nsINetworkConnectivityService_ConnectivityState IPv4; */
  NS_IMETHOD GetIPv4(nsINetworkConnectivityService::ConnectivityState *aIPv4) = 0;
  inline nsINetworkConnectivityService::ConnectivityState  GetIPv4()
  {
    nsINetworkConnectivityService::ConnectivityState result;
    mozilla::DebugOnly<nsresult> rv = GetIPv4(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute nsINetworkConnectivityService_ConnectivityState IPv6; */
  NS_IMETHOD GetIPv6(nsINetworkConnectivityService::ConnectivityState *aIPv6) = 0;
  inline nsINetworkConnectivityService::ConnectivityState  GetIPv6()
  {
    nsINetworkConnectivityService::ConnectivityState result;
    mozilla::DebugOnly<nsresult> rv = GetIPv6(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute nsINetworkConnectivityService_ConnectivityState NAT64; */
  NS_IMETHOD GetNAT64(nsINetworkConnectivityService::ConnectivityState *aNAT64) = 0;
  inline nsINetworkConnectivityService::ConnectivityState  GetNAT64()
  {
    nsINetworkConnectivityService::ConnectivityState result;
    mozilla::DebugOnly<nsresult> rv = GetNAT64(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* void recheckDNS (); */
  NS_IMETHOD RecheckDNS(void) = 0;

  /* void recheckIPConnectivity (); */
  NS_IMETHOD RecheckIPConnectivity(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINetworkConnectivityService, NS_INETWORKCONNECTIVITYSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINETWORKCONNECTIVITYSERVICE \
  using nsINetworkConnectivityService::GetDNSv4; \
  NS_IMETHOD GetDNSv4(nsINetworkConnectivityService::ConnectivityState *aDNSv4) override; \
  using nsINetworkConnectivityService::GetDNSv6; \
  NS_IMETHOD GetDNSv6(nsINetworkConnectivityService::ConnectivityState *aDNSv6) override; \
  using nsINetworkConnectivityService::GetIPv4; \
  NS_IMETHOD GetIPv4(nsINetworkConnectivityService::ConnectivityState *aIPv4) override; \
  using nsINetworkConnectivityService::GetIPv6; \
  NS_IMETHOD GetIPv6(nsINetworkConnectivityService::ConnectivityState *aIPv6) override; \
  using nsINetworkConnectivityService::GetNAT64; \
  NS_IMETHOD GetNAT64(nsINetworkConnectivityService::ConnectivityState *aNAT64) override; \
  NS_IMETHOD RecheckDNS(void) override; \
  NS_IMETHOD RecheckIPConnectivity(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINETWORKCONNECTIVITYSERVICE \
  using nsINetworkConnectivityService::GetDNSv4; \
  nsresult GetDNSv4(nsINetworkConnectivityService::ConnectivityState *aDNSv4); \
  using nsINetworkConnectivityService::GetDNSv6; \
  nsresult GetDNSv6(nsINetworkConnectivityService::ConnectivityState *aDNSv6); \
  using nsINetworkConnectivityService::GetIPv4; \
  nsresult GetIPv4(nsINetworkConnectivityService::ConnectivityState *aIPv4); \
  using nsINetworkConnectivityService::GetIPv6; \
  nsresult GetIPv6(nsINetworkConnectivityService::ConnectivityState *aIPv6); \
  using nsINetworkConnectivityService::GetNAT64; \
  nsresult GetNAT64(nsINetworkConnectivityService::ConnectivityState *aNAT64); \
  nsresult RecheckDNS(void); \
  nsresult RecheckIPConnectivity(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINETWORKCONNECTIVITYSERVICE(_to) \
  using nsINetworkConnectivityService::GetDNSv4; \
  NS_IMETHOD GetDNSv4(nsINetworkConnectivityService::ConnectivityState *aDNSv4) override { return _to GetDNSv4(aDNSv4); } \
  using nsINetworkConnectivityService::GetDNSv6; \
  NS_IMETHOD GetDNSv6(nsINetworkConnectivityService::ConnectivityState *aDNSv6) override { return _to GetDNSv6(aDNSv6); } \
  using nsINetworkConnectivityService::GetIPv4; \
  NS_IMETHOD GetIPv4(nsINetworkConnectivityService::ConnectivityState *aIPv4) override { return _to GetIPv4(aIPv4); } \
  using nsINetworkConnectivityService::GetIPv6; \
  NS_IMETHOD GetIPv6(nsINetworkConnectivityService::ConnectivityState *aIPv6) override { return _to GetIPv6(aIPv6); } \
  using nsINetworkConnectivityService::GetNAT64; \
  NS_IMETHOD GetNAT64(nsINetworkConnectivityService::ConnectivityState *aNAT64) override { return _to GetNAT64(aNAT64); } \
  NS_IMETHOD RecheckDNS(void) override { return _to RecheckDNS(); } \
  NS_IMETHOD RecheckIPConnectivity(void) override { return _to RecheckIPConnectivity(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINETWORKCONNECTIVITYSERVICE(_to) \
  NS_IMETHOD GetDNSv4(nsINetworkConnectivityService::ConnectivityState *aDNSv4) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDNSv4(aDNSv4); } \
  NS_IMETHOD GetDNSv6(nsINetworkConnectivityService::ConnectivityState *aDNSv6) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDNSv6(aDNSv6); } \
  NS_IMETHOD GetIPv4(nsINetworkConnectivityService::ConnectivityState *aIPv4) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIPv4(aIPv4); } \
  NS_IMETHOD GetIPv6(nsINetworkConnectivityService::ConnectivityState *aIPv6) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIPv6(aIPv6); } \
  NS_IMETHOD GetNAT64(nsINetworkConnectivityService::ConnectivityState *aNAT64) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNAT64(aNAT64); } \
  NS_IMETHOD RecheckDNS(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RecheckDNS(); } \
  NS_IMETHOD RecheckIPConnectivity(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RecheckIPConnectivity(); } 


#endif /* __gen_nsINetworkConnectivityService_h__ */
