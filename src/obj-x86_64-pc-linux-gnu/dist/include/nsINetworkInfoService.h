/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsINetworkInfoService.idl
 */

#ifndef __gen_nsINetworkInfoService_h__
#define __gen_nsINetworkInfoService_h__


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

/* starting interface:    nsIListNetworkAddressesListener */
#define NS_ILISTNETWORKADDRESSESLISTENER_IID_STR "c4bdaac1-3ab1-4fdb-9a16-17cbed794603"

#define NS_ILISTNETWORKADDRESSESLISTENER_IID \
  {0xc4bdaac1, 0x3ab1, 0x4fdb, \
    { 0x9a, 0x16, 0x17, 0xcb, 0xed, 0x79, 0x46, 0x03 }}

class NS_NO_VTABLE nsIListNetworkAddressesListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ILISTNETWORKADDRESSESLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIListNetworkAddressesListener;

  /* void onListedNetworkAddresses (in Array<ACString> aAddressArray); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnListedNetworkAddresses(const nsTArray<nsCString >& aAddressArray) = 0;

  /* void onListNetworkAddressesFailed (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnListNetworkAddressesFailed(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIListNetworkAddressesListener, NS_ILISTNETWORKADDRESSESLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSILISTNETWORKADDRESSESLISTENER \
  NS_IMETHOD OnListedNetworkAddresses(const nsTArray<nsCString >& aAddressArray) override; \
  NS_IMETHOD OnListNetworkAddressesFailed(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSILISTNETWORKADDRESSESLISTENER \
  nsresult OnListedNetworkAddresses(const nsTArray<nsCString >& aAddressArray); \
  nsresult OnListNetworkAddressesFailed(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSILISTNETWORKADDRESSESLISTENER(_to) \
  NS_IMETHOD OnListedNetworkAddresses(const nsTArray<nsCString >& aAddressArray) override { return _to OnListedNetworkAddresses(aAddressArray); } \
  NS_IMETHOD OnListNetworkAddressesFailed(void) override { return _to OnListNetworkAddressesFailed(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSILISTNETWORKADDRESSESLISTENER(_to) \
  NS_IMETHOD OnListedNetworkAddresses(const nsTArray<nsCString >& aAddressArray) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnListedNetworkAddresses(aAddressArray); } \
  NS_IMETHOD OnListNetworkAddressesFailed(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnListNetworkAddressesFailed(); } 


/* starting interface:    nsIGetHostnameListener */
#define NS_IGETHOSTNAMELISTENER_IID_STR "3ebdcb62-2df4-4042-8864-3fa81abd4693"

#define NS_IGETHOSTNAMELISTENER_IID \
  {0x3ebdcb62, 0x2df4, 0x4042, \
    { 0x88, 0x64, 0x3f, 0xa8, 0x1a, 0xbd, 0x46, 0x93 }}

class NS_NO_VTABLE nsIGetHostnameListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IGETHOSTNAMELISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIGetHostnameListener;

  /* void onGotHostname (in AUTF8String aHostname); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnGotHostname(const nsACString& aHostname) = 0;

  /* void onGetHostnameFailed (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnGetHostnameFailed(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIGetHostnameListener, NS_IGETHOSTNAMELISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIGETHOSTNAMELISTENER \
  NS_IMETHOD OnGotHostname(const nsACString& aHostname) override; \
  NS_IMETHOD OnGetHostnameFailed(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIGETHOSTNAMELISTENER \
  nsresult OnGotHostname(const nsACString& aHostname); \
  nsresult OnGetHostnameFailed(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIGETHOSTNAMELISTENER(_to) \
  NS_IMETHOD OnGotHostname(const nsACString& aHostname) override { return _to OnGotHostname(aHostname); } \
  NS_IMETHOD OnGetHostnameFailed(void) override { return _to OnGetHostnameFailed(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIGETHOSTNAMELISTENER(_to) \
  NS_IMETHOD OnGotHostname(const nsACString& aHostname) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnGotHostname(aHostname); } \
  NS_IMETHOD OnGetHostnameFailed(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnGetHostnameFailed(); } 


/* starting interface:    nsINetworkInfoService */
#define NS_INETWORKINFOSERVICE_IID_STR "55fc8dae-4a58-4e0f-a49b-901cbabae809"

#define NS_INETWORKINFOSERVICE_IID \
  {0x55fc8dae, 0x4a58, 0x4e0f, \
    { 0xa4, 0x9b, 0x90, 0x1c, 0xba, 0xba, 0xe8, 0x09 }}

class NS_NO_VTABLE nsINetworkInfoService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INETWORKINFOSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINetworkInfoService;

  /* void listNetworkAddresses (in nsIListNetworkAddressesListener aListener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ListNetworkAddresses(nsIListNetworkAddressesListener *aListener) = 0;

  /* void getHostname (in nsIGetHostnameListener aListener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHostname(nsIGetHostnameListener *aListener) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINetworkInfoService, NS_INETWORKINFOSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINETWORKINFOSERVICE \
  NS_IMETHOD ListNetworkAddresses(nsIListNetworkAddressesListener *aListener) override; \
  NS_IMETHOD GetHostname(nsIGetHostnameListener *aListener) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINETWORKINFOSERVICE \
  nsresult ListNetworkAddresses(nsIListNetworkAddressesListener *aListener); \
  nsresult GetHostname(nsIGetHostnameListener *aListener); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINETWORKINFOSERVICE(_to) \
  NS_IMETHOD ListNetworkAddresses(nsIListNetworkAddressesListener *aListener) override { return _to ListNetworkAddresses(aListener); } \
  NS_IMETHOD GetHostname(nsIGetHostnameListener *aListener) override { return _to GetHostname(aListener); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINETWORKINFOSERVICE(_to) \
  NS_IMETHOD ListNetworkAddresses(nsIListNetworkAddressesListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ListNetworkAddresses(aListener); } \
  NS_IMETHOD GetHostname(nsIGetHostnameListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHostname(aListener); } 

#define NETWORKINFOSERVICE_CONTRACT_ID \
  "@mozilla.org/network-info-service;1"

#endif /* __gen_nsINetworkInfoService_h__ */
