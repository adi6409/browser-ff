/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/wifi/nsIWifiAccessPoint.idl
 */

#ifndef __gen_nsIWifiAccessPoint_h__
#define __gen_nsIWifiAccessPoint_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIWifiAccessPoint */
#define NS_IWIFIACCESSPOINT_IID_STR "e28e614f-8f86-44ff-bcf5-5f18225834a0"

#define NS_IWIFIACCESSPOINT_IID \
  {0xe28e614f, 0x8f86, 0x44ff, \
    { 0xbc, 0xf5, 0x5f, 0x18, 0x22, 0x58, 0x34, 0xa0 }}

class NS_NO_VTABLE nsIWifiAccessPoint : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWIFIACCESSPOINT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWifiAccessPoint;

  /* readonly attribute ACString mac; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMac(nsACString& aMac) = 0;

  /* readonly attribute AString ssid; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSsid(nsAString& aSsid) = 0;

  /* readonly attribute ACString rawSSID; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRawSSID(nsACString& aRawSSID) = 0;

  /* readonly attribute long signal; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSignal(int32_t *aSignal) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWifiAccessPoint, NS_IWIFIACCESSPOINT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWIFIACCESSPOINT \
  NS_IMETHOD GetMac(nsACString& aMac) override; \
  NS_IMETHOD GetSsid(nsAString& aSsid) override; \
  NS_IMETHOD GetRawSSID(nsACString& aRawSSID) override; \
  NS_IMETHOD GetSignal(int32_t *aSignal) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWIFIACCESSPOINT \
  nsresult GetMac(nsACString& aMac); \
  nsresult GetSsid(nsAString& aSsid); \
  nsresult GetRawSSID(nsACString& aRawSSID); \
  nsresult GetSignal(int32_t *aSignal); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWIFIACCESSPOINT(_to) \
  NS_IMETHOD GetMac(nsACString& aMac) override { return _to GetMac(aMac); } \
  NS_IMETHOD GetSsid(nsAString& aSsid) override { return _to GetSsid(aSsid); } \
  NS_IMETHOD GetRawSSID(nsACString& aRawSSID) override { return _to GetRawSSID(aRawSSID); } \
  NS_IMETHOD GetSignal(int32_t *aSignal) override { return _to GetSignal(aSignal); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWIFIACCESSPOINT(_to) \
  NS_IMETHOD GetMac(nsACString& aMac) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMac(aMac); } \
  NS_IMETHOD GetSsid(nsAString& aSsid) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSsid(aSsid); } \
  NS_IMETHOD GetRawSSID(nsACString& aRawSSID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRawSSID(aRawSSID); } \
  NS_IMETHOD GetSignal(int32_t *aSignal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSignal(aSignal); } 


#endif /* __gen_nsIWifiAccessPoint_h__ */
