/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsINetAddr.idl
 */

#ifndef __gen_nsINetAddr_h__
#define __gen_nsINetAddr_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
namespace net {
union NetAddr;
}
}

/* starting interface:    nsINetAddr */
#define NS_INETADDR_IID_STR "652b9ec5-d159-45d7-9127-50bb559486cd"

#define NS_INETADDR_IID \
  {0x652b9ec5, 0xd159, 0x45d7, \
    { 0x91, 0x27, 0x50, 0xbb, 0x55, 0x94, 0x86, 0xcd }}

class NS_NO_VTABLE nsINetAddr : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INETADDR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINetAddr;

  /* readonly attribute unsigned short family; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFamily(uint16_t *aFamily) = 0;

  /* readonly attribute AUTF8String address; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAddress(nsACString& aAddress) = 0;

  /* readonly attribute unsigned short port; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPort(uint16_t *aPort) = 0;

  /* readonly attribute unsigned long flow; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFlow(uint32_t *aFlow) = 0;

  /* readonly attribute unsigned long scope; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetScope(uint32_t *aScope) = 0;

  /* readonly attribute boolean isV4Mapped; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsV4Mapped(bool *aIsV4Mapped) = 0;

  enum {
    FAMILY_INET = 1U,
    FAMILY_INET6 = 2U,
    FAMILY_LOCAL = 3U
  };

  /* [noscript] NetAddr getNetAddr (); */
  NS_IMETHOD GetNetAddr(mozilla::net::NetAddr * _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINetAddr, NS_INETADDR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINETADDR \
  NS_IMETHOD GetFamily(uint16_t *aFamily) override; \
  NS_IMETHOD GetAddress(nsACString& aAddress) override; \
  NS_IMETHOD GetPort(uint16_t *aPort) override; \
  NS_IMETHOD GetFlow(uint32_t *aFlow) override; \
  NS_IMETHOD GetScope(uint32_t *aScope) override; \
  NS_IMETHOD GetIsV4Mapped(bool *aIsV4Mapped) override; \
  NS_IMETHOD GetNetAddr(mozilla::net::NetAddr * _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINETADDR \
  nsresult GetFamily(uint16_t *aFamily); \
  nsresult GetAddress(nsACString& aAddress); \
  nsresult GetPort(uint16_t *aPort); \
  nsresult GetFlow(uint32_t *aFlow); \
  nsresult GetScope(uint32_t *aScope); \
  nsresult GetIsV4Mapped(bool *aIsV4Mapped); \
  nsresult GetNetAddr(mozilla::net::NetAddr * _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINETADDR(_to) \
  NS_IMETHOD GetFamily(uint16_t *aFamily) override { return _to GetFamily(aFamily); } \
  NS_IMETHOD GetAddress(nsACString& aAddress) override { return _to GetAddress(aAddress); } \
  NS_IMETHOD GetPort(uint16_t *aPort) override { return _to GetPort(aPort); } \
  NS_IMETHOD GetFlow(uint32_t *aFlow) override { return _to GetFlow(aFlow); } \
  NS_IMETHOD GetScope(uint32_t *aScope) override { return _to GetScope(aScope); } \
  NS_IMETHOD GetIsV4Mapped(bool *aIsV4Mapped) override { return _to GetIsV4Mapped(aIsV4Mapped); } \
  NS_IMETHOD GetNetAddr(mozilla::net::NetAddr * _retval) override { return _to GetNetAddr(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINETADDR(_to) \
  NS_IMETHOD GetFamily(uint16_t *aFamily) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFamily(aFamily); } \
  NS_IMETHOD GetAddress(nsACString& aAddress) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAddress(aAddress); } \
  NS_IMETHOD GetPort(uint16_t *aPort) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPort(aPort); } \
  NS_IMETHOD GetFlow(uint32_t *aFlow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFlow(aFlow); } \
  NS_IMETHOD GetScope(uint32_t *aScope) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScope(aScope); } \
  NS_IMETHOD GetIsV4Mapped(bool *aIsV4Mapped) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsV4Mapped(aIsV4Mapped); } \
  NS_IMETHOD GetNetAddr(mozilla::net::NetAddr * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNetAddr(_retval); } 


#endif /* __gen_nsINetAddr_h__ */
