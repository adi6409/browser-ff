/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIProtocolProxyService2.idl
 */

#ifndef __gen_nsIProtocolProxyService2_h__
#define __gen_nsIProtocolProxyService2_h__


#ifndef __gen_nsIProtocolProxyService_h__
#include "nsIProtocolProxyService.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIProtocolProxyService2 */
#define NS_IPROTOCOLPROXYSERVICE2_IID_STR "b2e5b2c0-e21e-4845-b336-be6d60a38951"

#define NS_IPROTOCOLPROXYSERVICE2_IID \
  {0xb2e5b2c0, 0xe21e, 0x4845, \
    { 0xb3, 0x36, 0xbe, 0x6d, 0x60, 0xa3, 0x89, 0x51 }}

class NS_NO_VTABLE nsIProtocolProxyService2 : public nsIProtocolProxyService {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPROTOCOLPROXYSERVICE2_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIProtocolProxyService2;

  /* void reloadPAC (); */
  NS_IMETHOD ReloadPAC(void) = 0;

  /* nsICancelable asyncResolve2 (in nsIChannel aChannel, in unsigned long aFlags, in nsIProtocolProxyCallback aCallback, [optional] in nsISerialEventTarget aMainThreadTarget); */
  NS_IMETHOD AsyncResolve2(nsIChannel *aChannel, uint32_t aFlags, nsIProtocolProxyCallback *aCallback, nsISerialEventTarget *aMainThreadTarget, nsICancelable **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIProtocolProxyService2, NS_IPROTOCOLPROXYSERVICE2_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPROTOCOLPROXYSERVICE2 \
  NS_IMETHOD ReloadPAC(void) override; \
  NS_IMETHOD AsyncResolve2(nsIChannel *aChannel, uint32_t aFlags, nsIProtocolProxyCallback *aCallback, nsISerialEventTarget *aMainThreadTarget, nsICancelable **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPROTOCOLPROXYSERVICE2 \
  nsresult ReloadPAC(void); \
  nsresult AsyncResolve2(nsIChannel *aChannel, uint32_t aFlags, nsIProtocolProxyCallback *aCallback, nsISerialEventTarget *aMainThreadTarget, nsICancelable **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPROTOCOLPROXYSERVICE2(_to) \
  NS_IMETHOD ReloadPAC(void) override { return _to ReloadPAC(); } \
  NS_IMETHOD AsyncResolve2(nsIChannel *aChannel, uint32_t aFlags, nsIProtocolProxyCallback *aCallback, nsISerialEventTarget *aMainThreadTarget, nsICancelable **_retval) override { return _to AsyncResolve2(aChannel, aFlags, aCallback, aMainThreadTarget, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPROTOCOLPROXYSERVICE2(_to) \
  NS_IMETHOD ReloadPAC(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReloadPAC(); } \
  NS_IMETHOD AsyncResolve2(nsIChannel *aChannel, uint32_t aFlags, nsIProtocolProxyCallback *aCallback, nsISerialEventTarget *aMainThreadTarget, nsICancelable **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncResolve2(aChannel, aFlags, aCallback, aMainThreadTarget, _retval); } 


#endif /* __gen_nsIProtocolProxyService2_h__ */
