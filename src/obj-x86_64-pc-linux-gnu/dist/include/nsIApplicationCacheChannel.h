/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIApplicationCacheChannel.idl
 */

#ifndef __gen_nsIApplicationCacheChannel_h__
#define __gen_nsIApplicationCacheChannel_h__


#ifndef __gen_nsIApplicationCacheContainer_h__
#include "nsIApplicationCacheContainer.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIApplicationCacheChannel */
#define NS_IAPPLICATIONCACHECHANNEL_IID_STR "6fa816b1-6d5f-4380-9704-054d0908cfa3"

#define NS_IAPPLICATIONCACHECHANNEL_IID \
  {0x6fa816b1, 0x6d5f, 0x4380, \
    { 0x97, 0x04, 0x05, 0x4d, 0x09, 0x08, 0xcf, 0xa3 }}

class NS_NO_VTABLE nsIApplicationCacheChannel : public nsIApplicationCacheContainer {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IAPPLICATIONCACHECHANNEL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIApplicationCacheChannel;

  /* readonly attribute boolean loadedFromApplicationCache; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLoadedFromApplicationCache(bool *aLoadedFromApplicationCache) = 0;

  /* attribute boolean inheritApplicationCache; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetInheritApplicationCache(bool *aInheritApplicationCache) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetInheritApplicationCache(bool aInheritApplicationCache) = 0;

  /* attribute boolean chooseApplicationCache; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetChooseApplicationCache(bool *aChooseApplicationCache) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetChooseApplicationCache(bool aChooseApplicationCache) = 0;

  /* void markOfflineCacheEntryAsForeign (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MarkOfflineCacheEntryAsForeign(void) = 0;

  /* attribute nsIApplicationCache applicationCacheForWrite; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetApplicationCacheForWrite(nsIApplicationCache **aApplicationCacheForWrite) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetApplicationCacheForWrite(nsIApplicationCache *aApplicationCacheForWrite) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIApplicationCacheChannel, NS_IAPPLICATIONCACHECHANNEL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIAPPLICATIONCACHECHANNEL \
  NS_IMETHOD GetLoadedFromApplicationCache(bool *aLoadedFromApplicationCache) override; \
  NS_IMETHOD GetInheritApplicationCache(bool *aInheritApplicationCache) override; \
  NS_IMETHOD SetInheritApplicationCache(bool aInheritApplicationCache) override; \
  NS_IMETHOD GetChooseApplicationCache(bool *aChooseApplicationCache) override; \
  NS_IMETHOD SetChooseApplicationCache(bool aChooseApplicationCache) override; \
  NS_IMETHOD MarkOfflineCacheEntryAsForeign(void) override; \
  NS_IMETHOD GetApplicationCacheForWrite(nsIApplicationCache **aApplicationCacheForWrite) override; \
  NS_IMETHOD SetApplicationCacheForWrite(nsIApplicationCache *aApplicationCacheForWrite) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIAPPLICATIONCACHECHANNEL \
  nsresult GetLoadedFromApplicationCache(bool *aLoadedFromApplicationCache); \
  nsresult GetInheritApplicationCache(bool *aInheritApplicationCache); \
  nsresult SetInheritApplicationCache(bool aInheritApplicationCache); \
  nsresult GetChooseApplicationCache(bool *aChooseApplicationCache); \
  nsresult SetChooseApplicationCache(bool aChooseApplicationCache); \
  nsresult MarkOfflineCacheEntryAsForeign(void); \
  nsresult GetApplicationCacheForWrite(nsIApplicationCache **aApplicationCacheForWrite); \
  nsresult SetApplicationCacheForWrite(nsIApplicationCache *aApplicationCacheForWrite); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIAPPLICATIONCACHECHANNEL(_to) \
  NS_IMETHOD GetLoadedFromApplicationCache(bool *aLoadedFromApplicationCache) override { return _to GetLoadedFromApplicationCache(aLoadedFromApplicationCache); } \
  NS_IMETHOD GetInheritApplicationCache(bool *aInheritApplicationCache) override { return _to GetInheritApplicationCache(aInheritApplicationCache); } \
  NS_IMETHOD SetInheritApplicationCache(bool aInheritApplicationCache) override { return _to SetInheritApplicationCache(aInheritApplicationCache); } \
  NS_IMETHOD GetChooseApplicationCache(bool *aChooseApplicationCache) override { return _to GetChooseApplicationCache(aChooseApplicationCache); } \
  NS_IMETHOD SetChooseApplicationCache(bool aChooseApplicationCache) override { return _to SetChooseApplicationCache(aChooseApplicationCache); } \
  NS_IMETHOD MarkOfflineCacheEntryAsForeign(void) override { return _to MarkOfflineCacheEntryAsForeign(); } \
  NS_IMETHOD GetApplicationCacheForWrite(nsIApplicationCache **aApplicationCacheForWrite) override { return _to GetApplicationCacheForWrite(aApplicationCacheForWrite); } \
  NS_IMETHOD SetApplicationCacheForWrite(nsIApplicationCache *aApplicationCacheForWrite) override { return _to SetApplicationCacheForWrite(aApplicationCacheForWrite); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIAPPLICATIONCACHECHANNEL(_to) \
  NS_IMETHOD GetLoadedFromApplicationCache(bool *aLoadedFromApplicationCache) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoadedFromApplicationCache(aLoadedFromApplicationCache); } \
  NS_IMETHOD GetInheritApplicationCache(bool *aInheritApplicationCache) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInheritApplicationCache(aInheritApplicationCache); } \
  NS_IMETHOD SetInheritApplicationCache(bool aInheritApplicationCache) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetInheritApplicationCache(aInheritApplicationCache); } \
  NS_IMETHOD GetChooseApplicationCache(bool *aChooseApplicationCache) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChooseApplicationCache(aChooseApplicationCache); } \
  NS_IMETHOD SetChooseApplicationCache(bool aChooseApplicationCache) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetChooseApplicationCache(aChooseApplicationCache); } \
  NS_IMETHOD MarkOfflineCacheEntryAsForeign(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MarkOfflineCacheEntryAsForeign(); } \
  NS_IMETHOD GetApplicationCacheForWrite(nsIApplicationCache **aApplicationCacheForWrite) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetApplicationCacheForWrite(aApplicationCacheForWrite); } \
  NS_IMETHOD SetApplicationCacheForWrite(nsIApplicationCache *aApplicationCacheForWrite) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetApplicationCacheForWrite(aApplicationCacheForWrite); } 


#endif /* __gen_nsIApplicationCacheChannel_h__ */
