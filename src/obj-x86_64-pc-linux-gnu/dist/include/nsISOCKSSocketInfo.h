/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/socket/nsISOCKSSocketInfo.idl
 */

#ifndef __gen_nsISOCKSSocketInfo_h__
#define __gen_nsISOCKSSocketInfo_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
namespace net {
union NetAddr;
}
}

/* starting interface:    nsISOCKSSocketInfo */
#define NS_ISOCKSSOCKETINFO_IID_STR "d5c0d1f9-22d7-47dc-bf91-d9ac6e1251a6"

#define NS_ISOCKSSOCKETINFO_IID \
  {0xd5c0d1f9, 0x22d7, 0x47dc, \
    { 0xbf, 0x91, 0xd9, 0xac, 0x6e, 0x12, 0x51, 0xa6 }}

class NS_NO_VTABLE nsISOCKSSocketInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISOCKSSOCKETINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISOCKSSocketInfo;

  /* [noscript] attribute NetAddrPtr destinationAddr; */
  NS_IMETHOD GetDestinationAddr(mozilla::net::NetAddr * * aDestinationAddr) = 0;
  NS_IMETHOD SetDestinationAddr(mozilla::net::NetAddr * aDestinationAddr) = 0;

  /* [noscript] attribute NetAddrPtr externalProxyAddr; */
  NS_IMETHOD GetExternalProxyAddr(mozilla::net::NetAddr * * aExternalProxyAddr) = 0;
  NS_IMETHOD SetExternalProxyAddr(mozilla::net::NetAddr * aExternalProxyAddr) = 0;

  /* [noscript] attribute NetAddrPtr internalProxyAddr; */
  NS_IMETHOD GetInternalProxyAddr(mozilla::net::NetAddr * * aInternalProxyAddr) = 0;
  NS_IMETHOD SetInternalProxyAddr(mozilla::net::NetAddr * aInternalProxyAddr) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISOCKSSocketInfo, NS_ISOCKSSOCKETINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISOCKSSOCKETINFO \
  NS_IMETHOD GetDestinationAddr(mozilla::net::NetAddr * * aDestinationAddr) override; \
  NS_IMETHOD SetDestinationAddr(mozilla::net::NetAddr * aDestinationAddr) override; \
  NS_IMETHOD GetExternalProxyAddr(mozilla::net::NetAddr * * aExternalProxyAddr) override; \
  NS_IMETHOD SetExternalProxyAddr(mozilla::net::NetAddr * aExternalProxyAddr) override; \
  NS_IMETHOD GetInternalProxyAddr(mozilla::net::NetAddr * * aInternalProxyAddr) override; \
  NS_IMETHOD SetInternalProxyAddr(mozilla::net::NetAddr * aInternalProxyAddr) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISOCKSSOCKETINFO \
  nsresult GetDestinationAddr(mozilla::net::NetAddr * * aDestinationAddr); \
  nsresult SetDestinationAddr(mozilla::net::NetAddr * aDestinationAddr); \
  nsresult GetExternalProxyAddr(mozilla::net::NetAddr * * aExternalProxyAddr); \
  nsresult SetExternalProxyAddr(mozilla::net::NetAddr * aExternalProxyAddr); \
  nsresult GetInternalProxyAddr(mozilla::net::NetAddr * * aInternalProxyAddr); \
  nsresult SetInternalProxyAddr(mozilla::net::NetAddr * aInternalProxyAddr); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISOCKSSOCKETINFO(_to) \
  NS_IMETHOD GetDestinationAddr(mozilla::net::NetAddr * * aDestinationAddr) override { return _to GetDestinationAddr(aDestinationAddr); } \
  NS_IMETHOD SetDestinationAddr(mozilla::net::NetAddr * aDestinationAddr) override { return _to SetDestinationAddr(aDestinationAddr); } \
  NS_IMETHOD GetExternalProxyAddr(mozilla::net::NetAddr * * aExternalProxyAddr) override { return _to GetExternalProxyAddr(aExternalProxyAddr); } \
  NS_IMETHOD SetExternalProxyAddr(mozilla::net::NetAddr * aExternalProxyAddr) override { return _to SetExternalProxyAddr(aExternalProxyAddr); } \
  NS_IMETHOD GetInternalProxyAddr(mozilla::net::NetAddr * * aInternalProxyAddr) override { return _to GetInternalProxyAddr(aInternalProxyAddr); } \
  NS_IMETHOD SetInternalProxyAddr(mozilla::net::NetAddr * aInternalProxyAddr) override { return _to SetInternalProxyAddr(aInternalProxyAddr); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISOCKSSOCKETINFO(_to) \
  NS_IMETHOD GetDestinationAddr(mozilla::net::NetAddr * * aDestinationAddr) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDestinationAddr(aDestinationAddr); } \
  NS_IMETHOD SetDestinationAddr(mozilla::net::NetAddr * aDestinationAddr) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDestinationAddr(aDestinationAddr); } \
  NS_IMETHOD GetExternalProxyAddr(mozilla::net::NetAddr * * aExternalProxyAddr) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetExternalProxyAddr(aExternalProxyAddr); } \
  NS_IMETHOD SetExternalProxyAddr(mozilla::net::NetAddr * aExternalProxyAddr) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetExternalProxyAddr(aExternalProxyAddr); } \
  NS_IMETHOD GetInternalProxyAddr(mozilla::net::NetAddr * * aInternalProxyAddr) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInternalProxyAddr(aInternalProxyAddr); } \
  NS_IMETHOD SetInternalProxyAddr(mozilla::net::NetAddr * aInternalProxyAddr) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetInternalProxyAddr(aInternalProxyAddr); } 


#endif /* __gen_nsISOCKSSocketInfo_h__ */
