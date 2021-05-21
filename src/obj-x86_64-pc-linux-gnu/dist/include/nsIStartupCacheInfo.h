/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/startupcache/nsIStartupCacheInfo.idl
 */

#ifndef __gen_nsIStartupCacheInfo_h__
#define __gen_nsIStartupCacheInfo_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIStartupCacheInfo */
#define NS_ISTARTUPCACHEINFO_IID_STR "a6b2f8b0-7438-11ea-bc55-0242ac130003"

#define NS_ISTARTUPCACHEINFO_IID \
  {0xa6b2f8b0, 0x7438, 0x11ea, \
    { 0xbc, 0x55, 0x02, 0x42, 0xac, 0x13, 0x00, 0x03 }}

class NS_NO_VTABLE nsIStartupCacheInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISTARTUPCACHEINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIStartupCacheInfo;

  /* readonly attribute boolean IgnoreDiskCache; */
  NS_IMETHOD GetIgnoreDiskCache(bool *aIgnoreDiskCache) = 0;

  /* readonly attribute boolean FoundDiskCacheOnInit; */
  NS_IMETHOD GetFoundDiskCacheOnInit(bool *aFoundDiskCacheOnInit) = 0;

  /* readonly attribute boolean WroteToDiskCache; */
  NS_IMETHOD GetWroteToDiskCache(bool *aWroteToDiskCache) = 0;

  /* readonly attribute AString DiskCachePath; */
  NS_IMETHOD GetDiskCachePath(nsAString& aDiskCachePath) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIStartupCacheInfo, NS_ISTARTUPCACHEINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISTARTUPCACHEINFO \
  NS_IMETHOD GetIgnoreDiskCache(bool *aIgnoreDiskCache) override; \
  NS_IMETHOD GetFoundDiskCacheOnInit(bool *aFoundDiskCacheOnInit) override; \
  NS_IMETHOD GetWroteToDiskCache(bool *aWroteToDiskCache) override; \
  NS_IMETHOD GetDiskCachePath(nsAString& aDiskCachePath) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISTARTUPCACHEINFO \
  nsresult GetIgnoreDiskCache(bool *aIgnoreDiskCache); \
  nsresult GetFoundDiskCacheOnInit(bool *aFoundDiskCacheOnInit); \
  nsresult GetWroteToDiskCache(bool *aWroteToDiskCache); \
  nsresult GetDiskCachePath(nsAString& aDiskCachePath); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISTARTUPCACHEINFO(_to) \
  NS_IMETHOD GetIgnoreDiskCache(bool *aIgnoreDiskCache) override { return _to GetIgnoreDiskCache(aIgnoreDiskCache); } \
  NS_IMETHOD GetFoundDiskCacheOnInit(bool *aFoundDiskCacheOnInit) override { return _to GetFoundDiskCacheOnInit(aFoundDiskCacheOnInit); } \
  NS_IMETHOD GetWroteToDiskCache(bool *aWroteToDiskCache) override { return _to GetWroteToDiskCache(aWroteToDiskCache); } \
  NS_IMETHOD GetDiskCachePath(nsAString& aDiskCachePath) override { return _to GetDiskCachePath(aDiskCachePath); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISTARTUPCACHEINFO(_to) \
  NS_IMETHOD GetIgnoreDiskCache(bool *aIgnoreDiskCache) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIgnoreDiskCache(aIgnoreDiskCache); } \
  NS_IMETHOD GetFoundDiskCacheOnInit(bool *aFoundDiskCacheOnInit) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFoundDiskCacheOnInit(aFoundDiskCacheOnInit); } \
  NS_IMETHOD GetWroteToDiskCache(bool *aWroteToDiskCache) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWroteToDiskCache(aWroteToDiskCache); } \
  NS_IMETHOD GetDiskCachePath(nsAString& aDiskCachePath) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDiskCachePath(aDiskCachePath); } 


#endif /* __gen_nsIStartupCacheInfo_h__ */
