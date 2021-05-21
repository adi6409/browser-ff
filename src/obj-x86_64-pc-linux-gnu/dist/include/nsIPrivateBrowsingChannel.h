/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIPrivateBrowsingChannel.idl
 */

#ifndef __gen_nsIPrivateBrowsingChannel_h__
#define __gen_nsIPrivateBrowsingChannel_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIPrivateBrowsingChannel */
#define NS_IPRIVATEBROWSINGCHANNEL_IID_STR "df702bb0-55b8-11e2-bcfd-0800200c9a66"

#define NS_IPRIVATEBROWSINGCHANNEL_IID \
  {0xdf702bb0, 0x55b8, 0x11e2, \
    { 0xbc, 0xfd, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66 }}

class NS_NO_VTABLE nsIPrivateBrowsingChannel : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPRIVATEBROWSINGCHANNEL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPrivateBrowsingChannel;

  /* void setPrivate (in boolean aPrivate); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPrivate(bool aPrivate) = 0;

  /* readonly attribute boolean isChannelPrivate; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsChannelPrivate(bool *aIsChannelPrivate) = 0;

  /* [noscript] boolean isPrivateModeOverriden (out boolean aValue); */
  NS_IMETHOD IsPrivateModeOverriden(bool *aValue, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPrivateBrowsingChannel, NS_IPRIVATEBROWSINGCHANNEL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPRIVATEBROWSINGCHANNEL \
  NS_IMETHOD SetPrivate(bool aPrivate) override; \
  NS_IMETHOD GetIsChannelPrivate(bool *aIsChannelPrivate) override; \
  NS_IMETHOD IsPrivateModeOverriden(bool *aValue, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPRIVATEBROWSINGCHANNEL \
  nsresult SetPrivate(bool aPrivate); \
  nsresult GetIsChannelPrivate(bool *aIsChannelPrivate); \
  nsresult IsPrivateModeOverriden(bool *aValue, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPRIVATEBROWSINGCHANNEL(_to) \
  NS_IMETHOD SetPrivate(bool aPrivate) override { return _to SetPrivate(aPrivate); } \
  NS_IMETHOD GetIsChannelPrivate(bool *aIsChannelPrivate) override { return _to GetIsChannelPrivate(aIsChannelPrivate); } \
  NS_IMETHOD IsPrivateModeOverriden(bool *aValue, bool *_retval) override { return _to IsPrivateModeOverriden(aValue, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPRIVATEBROWSINGCHANNEL(_to) \
  NS_IMETHOD SetPrivate(bool aPrivate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPrivate(aPrivate); } \
  NS_IMETHOD GetIsChannelPrivate(bool *aIsChannelPrivate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsChannelPrivate(aIsChannelPrivate); } \
  NS_IMETHOD IsPrivateModeOverriden(bool *aValue, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsPrivateModeOverriden(aValue, _retval); } 


#endif /* __gen_nsIPrivateBrowsingChannel_h__ */
