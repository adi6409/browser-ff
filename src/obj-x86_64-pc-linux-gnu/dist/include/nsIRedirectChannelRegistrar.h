/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIRedirectChannelRegistrar.idl
 */

#ifndef __gen_nsIRedirectChannelRegistrar_h__
#define __gen_nsIRedirectChannelRegistrar_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIChannel; /* forward declaration */

class nsIParentChannel; /* forward declaration */


/* starting interface:    nsIRedirectChannelRegistrar */
#define NS_IREDIRECTCHANNELREGISTRAR_IID_STR "efa36ea2-5b07-46fc-9534-a5acb8b77b72"

#define NS_IREDIRECTCHANNELREGISTRAR_IID \
  {0xefa36ea2, 0x5b07, 0x46fc, \
    { 0x95, 0x34, 0xa5, 0xac, 0xb8, 0xb7, 0x7b, 0x72 }}

class NS_NO_VTABLE nsIRedirectChannelRegistrar : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IREDIRECTCHANNELREGISTRAR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIRedirectChannelRegistrar;

  /* void registerChannel (in nsIChannel channel, in uint64_t id); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RegisterChannel(nsIChannel *channel, uint64_t id) = 0;

  /* nsIChannel linkChannels (in uint64_t id, in nsIParentChannel channel); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD LinkChannels(uint64_t id, nsIParentChannel *channel, nsIChannel **_retval) = 0;

  /* nsIChannel getRegisteredChannel (in uint64_t id); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRegisteredChannel(uint64_t id, nsIChannel **_retval) = 0;

  /* nsIParentChannel getParentChannel (in uint64_t id); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetParentChannel(uint64_t id, nsIParentChannel **_retval) = 0;

  /* void deregisterChannels (in uint64_t id); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DeregisterChannels(uint64_t id) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIRedirectChannelRegistrar, NS_IREDIRECTCHANNELREGISTRAR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIREDIRECTCHANNELREGISTRAR \
  NS_IMETHOD RegisterChannel(nsIChannel *channel, uint64_t id) override; \
  NS_IMETHOD LinkChannels(uint64_t id, nsIParentChannel *channel, nsIChannel **_retval) override; \
  NS_IMETHOD GetRegisteredChannel(uint64_t id, nsIChannel **_retval) override; \
  NS_IMETHOD GetParentChannel(uint64_t id, nsIParentChannel **_retval) override; \
  NS_IMETHOD DeregisterChannels(uint64_t id) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIREDIRECTCHANNELREGISTRAR \
  nsresult RegisterChannel(nsIChannel *channel, uint64_t id); \
  nsresult LinkChannels(uint64_t id, nsIParentChannel *channel, nsIChannel **_retval); \
  nsresult GetRegisteredChannel(uint64_t id, nsIChannel **_retval); \
  nsresult GetParentChannel(uint64_t id, nsIParentChannel **_retval); \
  nsresult DeregisterChannels(uint64_t id); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIREDIRECTCHANNELREGISTRAR(_to) \
  NS_IMETHOD RegisterChannel(nsIChannel *channel, uint64_t id) override { return _to RegisterChannel(channel, id); } \
  NS_IMETHOD LinkChannels(uint64_t id, nsIParentChannel *channel, nsIChannel **_retval) override { return _to LinkChannels(id, channel, _retval); } \
  NS_IMETHOD GetRegisteredChannel(uint64_t id, nsIChannel **_retval) override { return _to GetRegisteredChannel(id, _retval); } \
  NS_IMETHOD GetParentChannel(uint64_t id, nsIParentChannel **_retval) override { return _to GetParentChannel(id, _retval); } \
  NS_IMETHOD DeregisterChannels(uint64_t id) override { return _to DeregisterChannels(id); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIREDIRECTCHANNELREGISTRAR(_to) \
  NS_IMETHOD RegisterChannel(nsIChannel *channel, uint64_t id) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterChannel(channel, id); } \
  NS_IMETHOD LinkChannels(uint64_t id, nsIParentChannel *channel, nsIChannel **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LinkChannels(id, channel, _retval); } \
  NS_IMETHOD GetRegisteredChannel(uint64_t id, nsIChannel **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRegisteredChannel(id, _retval); } \
  NS_IMETHOD GetParentChannel(uint64_t id, nsIParentChannel **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetParentChannel(id, _retval); } \
  NS_IMETHOD DeregisterChannels(uint64_t id) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DeregisterChannels(id); } 


#endif /* __gen_nsIRedirectChannelRegistrar_h__ */
