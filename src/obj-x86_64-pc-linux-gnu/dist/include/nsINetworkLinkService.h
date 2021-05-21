/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsINetworkLinkService.idl
 */

#ifndef __gen_nsINetworkLinkService_h__
#define __gen_nsINetworkLinkService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsINetworkLinkService */
#define NS_INETWORKLINKSERVICE_IID_STR "103e5293-77b3-4b70-af59-6e9e4a1f994a"

#define NS_INETWORKLINKSERVICE_IID \
  {0x103e5293, 0x77b3, 0x4b70, \
    { 0xaf, 0x59, 0x6e, 0x9e, 0x4a, 0x1f, 0x99, 0x4a }}

class NS_NO_VTABLE nsINetworkLinkService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INETWORKLINKSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINetworkLinkService;

  enum {
    LINK_TYPE_UNKNOWN = 0U,
    LINK_TYPE_ETHERNET = 1U,
    LINK_TYPE_USB = 2U,
    LINK_TYPE_WIFI = 3U,
    LINK_TYPE_WIMAX = 4U,
    LINK_TYPE_2G = 5U,
    LINK_TYPE_3G = 6U,
    LINK_TYPE_4G = 7U,
    LINK_TYPE_5G = 8U
  };

  /* readonly attribute boolean isLinkUp; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsLinkUp(bool *aIsLinkUp) = 0;

  /* readonly attribute boolean linkStatusKnown; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLinkStatusKnown(bool *aLinkStatusKnown) = 0;

  /* readonly attribute unsigned long linkType; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLinkType(uint32_t *aLinkType) = 0;

  /* readonly attribute ACString networkID; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNetworkID(nsACString& aNetworkID) = 0;

  /* readonly attribute Array<ACString> dnsSuffixList; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDnsSuffixList(nsTArray<nsCString >& aDnsSuffixList) = 0;

  enum {
    NONE_DETECTED = 0U,
    VPN_DETECTED = 1U,
    PROXY_DETECTED = 2U,
    NRPT_DETECTED = 4U
  };

  /* readonly attribute unsigned long platformDNSIndications; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPlatformDNSIndications(uint32_t *aPlatformDNSIndications) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINetworkLinkService, NS_INETWORKLINKSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINETWORKLINKSERVICE \
  NS_IMETHOD GetIsLinkUp(bool *aIsLinkUp) override; \
  NS_IMETHOD GetLinkStatusKnown(bool *aLinkStatusKnown) override; \
  NS_IMETHOD GetLinkType(uint32_t *aLinkType) override; \
  NS_IMETHOD GetNetworkID(nsACString& aNetworkID) override; \
  NS_IMETHOD GetDnsSuffixList(nsTArray<nsCString >& aDnsSuffixList) override; \
  NS_IMETHOD GetPlatformDNSIndications(uint32_t *aPlatformDNSIndications) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINETWORKLINKSERVICE \
  nsresult GetIsLinkUp(bool *aIsLinkUp); \
  nsresult GetLinkStatusKnown(bool *aLinkStatusKnown); \
  nsresult GetLinkType(uint32_t *aLinkType); \
  nsresult GetNetworkID(nsACString& aNetworkID); \
  nsresult GetDnsSuffixList(nsTArray<nsCString >& aDnsSuffixList); \
  nsresult GetPlatformDNSIndications(uint32_t *aPlatformDNSIndications); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINETWORKLINKSERVICE(_to) \
  NS_IMETHOD GetIsLinkUp(bool *aIsLinkUp) override { return _to GetIsLinkUp(aIsLinkUp); } \
  NS_IMETHOD GetLinkStatusKnown(bool *aLinkStatusKnown) override { return _to GetLinkStatusKnown(aLinkStatusKnown); } \
  NS_IMETHOD GetLinkType(uint32_t *aLinkType) override { return _to GetLinkType(aLinkType); } \
  NS_IMETHOD GetNetworkID(nsACString& aNetworkID) override { return _to GetNetworkID(aNetworkID); } \
  NS_IMETHOD GetDnsSuffixList(nsTArray<nsCString >& aDnsSuffixList) override { return _to GetDnsSuffixList(aDnsSuffixList); } \
  NS_IMETHOD GetPlatformDNSIndications(uint32_t *aPlatformDNSIndications) override { return _to GetPlatformDNSIndications(aPlatformDNSIndications); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINETWORKLINKSERVICE(_to) \
  NS_IMETHOD GetIsLinkUp(bool *aIsLinkUp) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsLinkUp(aIsLinkUp); } \
  NS_IMETHOD GetLinkStatusKnown(bool *aLinkStatusKnown) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLinkStatusKnown(aLinkStatusKnown); } \
  NS_IMETHOD GetLinkType(uint32_t *aLinkType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLinkType(aLinkType); } \
  NS_IMETHOD GetNetworkID(nsACString& aNetworkID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNetworkID(aNetworkID); } \
  NS_IMETHOD GetDnsSuffixList(nsTArray<nsCString >& aDnsSuffixList) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDnsSuffixList(aDnsSuffixList); } \
  NS_IMETHOD GetPlatformDNSIndications(uint32_t *aPlatformDNSIndications) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPlatformDNSIndications(aPlatformDNSIndications); } 

/**
 * We send notifications through nsIObserverService with topic
 * NS_NETWORK_LINK_TOPIC whenever one of isLinkUp or linkStatusKnown
 * changes. We pass one of the NS_NETWORK_LINK_DATA_ constants below
 * as the aData parameter of the notification.
 */
#define NS_NETWORK_LINK_TOPIC "network:link-status-changed"
/**
 * isLinkUp is now true, linkStatusKnown is true.
 */
#define NS_NETWORK_LINK_DATA_UP      "up"
/**
 * isLinkUp is now false, linkStatusKnown is true.
 */
#define NS_NETWORK_LINK_DATA_DOWN    "down"
/**
 * isLinkUp is still true, but the network setup is modified.
 * linkStatusKnown is true.
 */
#define NS_NETWORK_LINK_DATA_CHANGED "changed"
/**
 * linkStatusKnown is now false.
 */
#define NS_NETWORK_LINK_DATA_UNKNOWN "unknown"
/**
 * network ID has changed.
 */
#define NS_NETWORK_ID_CHANGED_TOPIC "network:networkid-changed"
/**
 * DNS suffix list has updated.
 */
#define NS_DNS_SUFFIX_LIST_UPDATED_TOPIC "network:dns-suffix-list-updated"
/**
 * We send notifications through nsIObserverService with topic
 * NS_NETWORK_LINK_TYPE_TOPIC whenever the network connection type
 * changes. We pass one of the valid connection type constants
 * below as the aData parameter of the notification.
 */
#define NS_NETWORK_LINK_TYPE_TOPIC "network:link-type-changed"
/** We were unable to determine the network connection type */
#define NS_NETWORK_LINK_TYPE_UNKNOWN  "unknown"
/** A standard wired ethernet connection */
#define NS_NETWORK_LINK_TYPE_ETHERNET    "ethernet"
/** A connection via a USB port */
#define NS_NETWORK_LINK_TYPE_USB    "usb"
/** A connection via a WiFi access point (IEEE802.11) */
#define NS_NETWORK_LINK_TYPE_WIFI "wifi"
/** A connection via WiMax (IEEE802.16) */
#define NS_NETWORK_LINK_TYPE_WIMAX "wimax"
/** A '2G' mobile connection (e.g. GSM, GPRS, EDGE) */
#define NS_NETWORK_LINK_TYPE_2G "2g"
/** A '3G' mobile connection (e.g. UMTS, CDMA) */
#define NS_NETWORK_LINK_TYPE_3G "3g"
/** A '4G' mobile connection (e.g. LTE, UMB) */
#define NS_NETWORK_LINK_TYPE_4G "4g"
/** A '5G' mobile connection */
#define NS_NETWORK_LINK_TYPE_5G "5g"

#endif /* __gen_nsINetworkLinkService_h__ */
