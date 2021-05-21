/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIParentChannel.idl
 */

#ifndef __gen_nsIParentChannel_h__
#define __gen_nsIParentChannel_h__


#ifndef __gen_nsIStreamListener_h__
#include "nsIStreamListener.h"
#endif

#ifndef __gen_nsIHttpChannel_h__
#include "nsIHttpChannel.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIRemoteTab; /* forward declaration */

namespace mozilla {
namespace net {
class ParentChannelListener;
}
}

/* starting interface:    nsIParentChannel */
#define NS_IPARENTCHANNEL_IID_STR "e0fc4801-6030-4653-a59f-1fb282bd1a04"

#define NS_IPARENTCHANNEL_IID \
  {0xe0fc4801, 0x6030, 0x4653, \
    { 0xa5, 0x9f, 0x1f, 0xb2, 0x82, 0xbd, 0x1a, 0x04 }}

class NS_NO_VTABLE nsIParentChannel : public nsIStreamListener {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPARENTCHANNEL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIParentChannel;

  /* [noscript] void setParentListener (in ParentChannelListener listener); */
  NS_IMETHOD SetParentListener(mozilla::net::ParentChannelListener * listener) = 0;

  /* [noscript] void notifyFlashPluginStateChanged (in nsIHttpChannel_FlashPluginState aState); */
  NS_IMETHOD NotifyFlashPluginStateChanged(nsIHttpChannel::FlashPluginState aState) = 0;

  /* [noscript] void setClassifierMatchedInfo (in ACString aList, in ACString aProvider, in ACString aFullHash); */
  NS_IMETHOD SetClassifierMatchedInfo(const nsACString& aList, const nsACString& aProvider, const nsACString& aFullHash) = 0;

  /* [noscript] void setClassifierMatchedTrackingInfo (in ACString aLists, in ACString aFullHashes); */
  NS_IMETHOD SetClassifierMatchedTrackingInfo(const nsACString& aLists, const nsACString& aFullHashes) = 0;

  /* [noscript] void notifyClassificationFlags (in uint32_t aClassificationFlags, in bool aIsThirdParty); */
  NS_IMETHOD NotifyClassificationFlags(uint32_t aClassificationFlags, bool aIsThirdParty) = 0;

  /* void delete (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Delete(void) = 0;

  /* readonly attribute AUTF8String remoteType; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRemoteType(nsACString& aRemoteType) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIParentChannel, NS_IPARENTCHANNEL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPARENTCHANNEL \
  NS_IMETHOD SetParentListener(mozilla::net::ParentChannelListener * listener) override; \
  NS_IMETHOD NotifyFlashPluginStateChanged(nsIHttpChannel::FlashPluginState aState) override; \
  NS_IMETHOD SetClassifierMatchedInfo(const nsACString& aList, const nsACString& aProvider, const nsACString& aFullHash) override; \
  NS_IMETHOD SetClassifierMatchedTrackingInfo(const nsACString& aLists, const nsACString& aFullHashes) override; \
  NS_IMETHOD NotifyClassificationFlags(uint32_t aClassificationFlags, bool aIsThirdParty) override; \
  NS_IMETHOD Delete(void) override; \
  NS_IMETHOD GetRemoteType(nsACString& aRemoteType) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPARENTCHANNEL \
  nsresult SetParentListener(mozilla::net::ParentChannelListener * listener); \
  nsresult NotifyFlashPluginStateChanged(nsIHttpChannel::FlashPluginState aState); \
  nsresult SetClassifierMatchedInfo(const nsACString& aList, const nsACString& aProvider, const nsACString& aFullHash); \
  nsresult SetClassifierMatchedTrackingInfo(const nsACString& aLists, const nsACString& aFullHashes); \
  nsresult NotifyClassificationFlags(uint32_t aClassificationFlags, bool aIsThirdParty); \
  nsresult Delete(void); \
  nsresult GetRemoteType(nsACString& aRemoteType); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPARENTCHANNEL(_to) \
  NS_IMETHOD SetParentListener(mozilla::net::ParentChannelListener * listener) override { return _to SetParentListener(listener); } \
  NS_IMETHOD NotifyFlashPluginStateChanged(nsIHttpChannel::FlashPluginState aState) override { return _to NotifyFlashPluginStateChanged(aState); } \
  NS_IMETHOD SetClassifierMatchedInfo(const nsACString& aList, const nsACString& aProvider, const nsACString& aFullHash) override { return _to SetClassifierMatchedInfo(aList, aProvider, aFullHash); } \
  NS_IMETHOD SetClassifierMatchedTrackingInfo(const nsACString& aLists, const nsACString& aFullHashes) override { return _to SetClassifierMatchedTrackingInfo(aLists, aFullHashes); } \
  NS_IMETHOD NotifyClassificationFlags(uint32_t aClassificationFlags, bool aIsThirdParty) override { return _to NotifyClassificationFlags(aClassificationFlags, aIsThirdParty); } \
  NS_IMETHOD Delete(void) override { return _to Delete(); } \
  NS_IMETHOD GetRemoteType(nsACString& aRemoteType) override { return _to GetRemoteType(aRemoteType); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPARENTCHANNEL(_to) \
  NS_IMETHOD SetParentListener(mozilla::net::ParentChannelListener * listener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetParentListener(listener); } \
  NS_IMETHOD NotifyFlashPluginStateChanged(nsIHttpChannel::FlashPluginState aState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyFlashPluginStateChanged(aState); } \
  NS_IMETHOD SetClassifierMatchedInfo(const nsACString& aList, const nsACString& aProvider, const nsACString& aFullHash) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetClassifierMatchedInfo(aList, aProvider, aFullHash); } \
  NS_IMETHOD SetClassifierMatchedTrackingInfo(const nsACString& aLists, const nsACString& aFullHashes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetClassifierMatchedTrackingInfo(aLists, aFullHashes); } \
  NS_IMETHOD NotifyClassificationFlags(uint32_t aClassificationFlags, bool aIsThirdParty) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyClassificationFlags(aClassificationFlags, aIsThirdParty); } \
  NS_IMETHOD Delete(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Delete(); } \
  NS_IMETHOD GetRemoteType(nsACString& aRemoteType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRemoteType(aRemoteType); } 


#endif /* __gen_nsIParentChannel_h__ */
