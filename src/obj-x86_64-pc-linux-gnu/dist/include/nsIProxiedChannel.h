/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIProxiedChannel.idl
 */

#ifndef __gen_nsIProxiedChannel_h__
#define __gen_nsIProxiedChannel_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIProxyInfo; /* forward declaration */


/* starting interface:    nsIProxiedChannel */
#define NS_IPROXIEDCHANNEL_IID_STR "6238f134-8c3f-4354-958f-dfd9d54a4446"

#define NS_IPROXIEDCHANNEL_IID \
  {0x6238f134, 0x8c3f, 0x4354, \
    { 0x95, 0x8f, 0xdf, 0xd9, 0xd5, 0x4a, 0x44, 0x46 }}

class NS_NO_VTABLE nsIProxiedChannel : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPROXIEDCHANNEL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIProxiedChannel;

  /* readonly attribute nsIProxyInfo proxyInfo; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetProxyInfo(nsIProxyInfo **aProxyInfo) = 0;

  /* readonly attribute int32_t httpProxyConnectResponseCode; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHttpProxyConnectResponseCode(int32_t *aHttpProxyConnectResponseCode) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIProxiedChannel, NS_IPROXIEDCHANNEL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPROXIEDCHANNEL \
  NS_IMETHOD GetProxyInfo(nsIProxyInfo **aProxyInfo) override; \
  NS_IMETHOD GetHttpProxyConnectResponseCode(int32_t *aHttpProxyConnectResponseCode) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPROXIEDCHANNEL \
  nsresult GetProxyInfo(nsIProxyInfo **aProxyInfo); \
  nsresult GetHttpProxyConnectResponseCode(int32_t *aHttpProxyConnectResponseCode); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPROXIEDCHANNEL(_to) \
  NS_IMETHOD GetProxyInfo(nsIProxyInfo **aProxyInfo) override { return _to GetProxyInfo(aProxyInfo); } \
  NS_IMETHOD GetHttpProxyConnectResponseCode(int32_t *aHttpProxyConnectResponseCode) override { return _to GetHttpProxyConnectResponseCode(aHttpProxyConnectResponseCode); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPROXIEDCHANNEL(_to) \
  NS_IMETHOD GetProxyInfo(nsIProxyInfo **aProxyInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProxyInfo(aProxyInfo); } \
  NS_IMETHOD GetHttpProxyConnectResponseCode(int32_t *aHttpProxyConnectResponseCode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHttpProxyConnectResponseCode(aHttpProxyConnectResponseCode); } 


#endif /* __gen_nsIProxiedChannel_h__ */
