/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache2/nsICacheStorageService.idl
 */

#ifndef __gen_nsICacheStorageService_h__
#define __gen_nsICacheStorageService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsICacheStorage; /* forward declaration */

class nsILoadContextInfo; /* forward declaration */

class nsIApplicationCache; /* forward declaration */

class nsIEventTarget; /* forward declaration */

class nsICacheStorageConsumptionObserver; /* forward declaration */

class nsICacheStorageVisitor; /* forward declaration */

class nsIPrincipal; /* forward declaration */


/* starting interface:    nsICacheStorageService */
#define NS_ICACHESTORAGESERVICE_IID_STR "ae29c44b-fbc3-4552-afaf-0a157ce771e7"

#define NS_ICACHESTORAGESERVICE_IID \
  {0xae29c44b, 0xfbc3, 0x4552, \
    { 0xaf, 0xaf, 0x0a, 0x15, 0x7c, 0xe7, 0x71, 0xe7 }}

class NS_NO_VTABLE nsICacheStorageService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICACHESTORAGESERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICacheStorageService;

  /* nsICacheStorage memoryCacheStorage (in nsILoadContextInfo aLoadContextInfo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MemoryCacheStorage(nsILoadContextInfo *aLoadContextInfo, nsICacheStorage **_retval) = 0;

  /* nsICacheStorage diskCacheStorage (in nsILoadContextInfo aLoadContextInfo, in bool aLookupAppCache); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DiskCacheStorage(nsILoadContextInfo *aLoadContextInfo, bool aLookupAppCache, nsICacheStorage **_retval) = 0;

  /* nsICacheStorage pinningCacheStorage (in nsILoadContextInfo aLoadContextInfo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PinningCacheStorage(nsILoadContextInfo *aLoadContextInfo, nsICacheStorage **_retval) = 0;

  /* nsICacheStorage appCacheStorage (in nsILoadContextInfo aLoadContextInfo, in nsIApplicationCache aApplicationCache); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AppCacheStorage(nsILoadContextInfo *aLoadContextInfo, nsIApplicationCache *aApplicationCache, nsICacheStorage **_retval) = 0;

  /* nsICacheStorage synthesizedCacheStorage (in nsILoadContextInfo aLoadContextInfo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SynthesizedCacheStorage(nsILoadContextInfo *aLoadContextInfo, nsICacheStorage **_retval) = 0;

  /* void clearOrigin (in nsIPrincipal aPrincipal); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ClearOrigin(nsIPrincipal *aPrincipal) = 0;

  /* void clearOriginAttributes (in AString aOriginAttributes); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ClearOriginAttributes(const nsAString& aOriginAttributes) = 0;

  /* void clear (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Clear(void) = 0;

  enum {
    PURGE_DISK_DATA_ONLY = 1U,
    PURGE_DISK_ALL = 2U,
    PURGE_EVERYTHING = 3U
  };

  /* void purgeFromMemory (in uint32_t aWhat); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PurgeFromMemory(uint32_t aWhat) = 0;

  /* readonly attribute nsIEventTarget ioTarget; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIoTarget(nsIEventTarget **aIoTarget) = 0;

  /* void asyncGetDiskConsumption (in nsICacheStorageConsumptionObserver aObserver); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncGetDiskConsumption(nsICacheStorageConsumptionObserver *aObserver) = 0;

  /* void asyncVisitAllStorages (in nsICacheStorageVisitor aVisitor, in boolean aVisitEntries); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncVisitAllStorages(nsICacheStorageVisitor *aVisitor, bool aVisitEntries) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICacheStorageService, NS_ICACHESTORAGESERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICACHESTORAGESERVICE \
  NS_IMETHOD MemoryCacheStorage(nsILoadContextInfo *aLoadContextInfo, nsICacheStorage **_retval) override; \
  NS_IMETHOD DiskCacheStorage(nsILoadContextInfo *aLoadContextInfo, bool aLookupAppCache, nsICacheStorage **_retval) override; \
  NS_IMETHOD PinningCacheStorage(nsILoadContextInfo *aLoadContextInfo, nsICacheStorage **_retval) override; \
  NS_IMETHOD AppCacheStorage(nsILoadContextInfo *aLoadContextInfo, nsIApplicationCache *aApplicationCache, nsICacheStorage **_retval) override; \
  NS_IMETHOD SynthesizedCacheStorage(nsILoadContextInfo *aLoadContextInfo, nsICacheStorage **_retval) override; \
  NS_IMETHOD ClearOrigin(nsIPrincipal *aPrincipal) override; \
  NS_IMETHOD ClearOriginAttributes(const nsAString& aOriginAttributes) override; \
  NS_IMETHOD Clear(void) override; \
  NS_IMETHOD PurgeFromMemory(uint32_t aWhat) override; \
  NS_IMETHOD GetIoTarget(nsIEventTarget **aIoTarget) override; \
  NS_IMETHOD AsyncGetDiskConsumption(nsICacheStorageConsumptionObserver *aObserver) override; \
  NS_IMETHOD AsyncVisitAllStorages(nsICacheStorageVisitor *aVisitor, bool aVisitEntries) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICACHESTORAGESERVICE \
  nsresult MemoryCacheStorage(nsILoadContextInfo *aLoadContextInfo, nsICacheStorage **_retval); \
  nsresult DiskCacheStorage(nsILoadContextInfo *aLoadContextInfo, bool aLookupAppCache, nsICacheStorage **_retval); \
  nsresult PinningCacheStorage(nsILoadContextInfo *aLoadContextInfo, nsICacheStorage **_retval); \
  nsresult AppCacheStorage(nsILoadContextInfo *aLoadContextInfo, nsIApplicationCache *aApplicationCache, nsICacheStorage **_retval); \
  nsresult SynthesizedCacheStorage(nsILoadContextInfo *aLoadContextInfo, nsICacheStorage **_retval); \
  nsresult ClearOrigin(nsIPrincipal *aPrincipal); \
  nsresult ClearOriginAttributes(const nsAString& aOriginAttributes); \
  nsresult Clear(void); \
  nsresult PurgeFromMemory(uint32_t aWhat); \
  nsresult GetIoTarget(nsIEventTarget **aIoTarget); \
  nsresult AsyncGetDiskConsumption(nsICacheStorageConsumptionObserver *aObserver); \
  nsresult AsyncVisitAllStorages(nsICacheStorageVisitor *aVisitor, bool aVisitEntries); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICACHESTORAGESERVICE(_to) \
  NS_IMETHOD MemoryCacheStorage(nsILoadContextInfo *aLoadContextInfo, nsICacheStorage **_retval) override { return _to MemoryCacheStorage(aLoadContextInfo, _retval); } \
  NS_IMETHOD DiskCacheStorage(nsILoadContextInfo *aLoadContextInfo, bool aLookupAppCache, nsICacheStorage **_retval) override { return _to DiskCacheStorage(aLoadContextInfo, aLookupAppCache, _retval); } \
  NS_IMETHOD PinningCacheStorage(nsILoadContextInfo *aLoadContextInfo, nsICacheStorage **_retval) override { return _to PinningCacheStorage(aLoadContextInfo, _retval); } \
  NS_IMETHOD AppCacheStorage(nsILoadContextInfo *aLoadContextInfo, nsIApplicationCache *aApplicationCache, nsICacheStorage **_retval) override { return _to AppCacheStorage(aLoadContextInfo, aApplicationCache, _retval); } \
  NS_IMETHOD SynthesizedCacheStorage(nsILoadContextInfo *aLoadContextInfo, nsICacheStorage **_retval) override { return _to SynthesizedCacheStorage(aLoadContextInfo, _retval); } \
  NS_IMETHOD ClearOrigin(nsIPrincipal *aPrincipal) override { return _to ClearOrigin(aPrincipal); } \
  NS_IMETHOD ClearOriginAttributes(const nsAString& aOriginAttributes) override { return _to ClearOriginAttributes(aOriginAttributes); } \
  NS_IMETHOD Clear(void) override { return _to Clear(); } \
  NS_IMETHOD PurgeFromMemory(uint32_t aWhat) override { return _to PurgeFromMemory(aWhat); } \
  NS_IMETHOD GetIoTarget(nsIEventTarget **aIoTarget) override { return _to GetIoTarget(aIoTarget); } \
  NS_IMETHOD AsyncGetDiskConsumption(nsICacheStorageConsumptionObserver *aObserver) override { return _to AsyncGetDiskConsumption(aObserver); } \
  NS_IMETHOD AsyncVisitAllStorages(nsICacheStorageVisitor *aVisitor, bool aVisitEntries) override { return _to AsyncVisitAllStorages(aVisitor, aVisitEntries); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICACHESTORAGESERVICE(_to) \
  NS_IMETHOD MemoryCacheStorage(nsILoadContextInfo *aLoadContextInfo, nsICacheStorage **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MemoryCacheStorage(aLoadContextInfo, _retval); } \
  NS_IMETHOD DiskCacheStorage(nsILoadContextInfo *aLoadContextInfo, bool aLookupAppCache, nsICacheStorage **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DiskCacheStorage(aLoadContextInfo, aLookupAppCache, _retval); } \
  NS_IMETHOD PinningCacheStorage(nsILoadContextInfo *aLoadContextInfo, nsICacheStorage **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PinningCacheStorage(aLoadContextInfo, _retval); } \
  NS_IMETHOD AppCacheStorage(nsILoadContextInfo *aLoadContextInfo, nsIApplicationCache *aApplicationCache, nsICacheStorage **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AppCacheStorage(aLoadContextInfo, aApplicationCache, _retval); } \
  NS_IMETHOD SynthesizedCacheStorage(nsILoadContextInfo *aLoadContextInfo, nsICacheStorage **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SynthesizedCacheStorage(aLoadContextInfo, _retval); } \
  NS_IMETHOD ClearOrigin(nsIPrincipal *aPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearOrigin(aPrincipal); } \
  NS_IMETHOD ClearOriginAttributes(const nsAString& aOriginAttributes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearOriginAttributes(aOriginAttributes); } \
  NS_IMETHOD Clear(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Clear(); } \
  NS_IMETHOD PurgeFromMemory(uint32_t aWhat) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PurgeFromMemory(aWhat); } \
  NS_IMETHOD GetIoTarget(nsIEventTarget **aIoTarget) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIoTarget(aIoTarget); } \
  NS_IMETHOD AsyncGetDiskConsumption(nsICacheStorageConsumptionObserver *aObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncGetDiskConsumption(aObserver); } \
  NS_IMETHOD AsyncVisitAllStorages(nsICacheStorageVisitor *aVisitor, bool aVisitEntries) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncVisitAllStorages(aVisitor, aVisitEntries); } 


/* starting interface:    nsICacheStorageConsumptionObserver */
#define NS_ICACHESTORAGECONSUMPTIONOBSERVER_IID_STR "7728ab5b-4c01-4483-a606-32bf5b8136cb"

#define NS_ICACHESTORAGECONSUMPTIONOBSERVER_IID \
  {0x7728ab5b, 0x4c01, 0x4483, \
    { 0xa6, 0x06, 0x32, 0xbf, 0x5b, 0x81, 0x36, 0xcb }}

class NS_NO_VTABLE nsICacheStorageConsumptionObserver : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICACHESTORAGECONSUMPTIONOBSERVER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICacheStorageConsumptionObserver;

  /* void onNetworkCacheDiskConsumption (in int64_t aDiskSize); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnNetworkCacheDiskConsumption(int64_t aDiskSize) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICacheStorageConsumptionObserver, NS_ICACHESTORAGECONSUMPTIONOBSERVER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICACHESTORAGECONSUMPTIONOBSERVER \
  NS_IMETHOD OnNetworkCacheDiskConsumption(int64_t aDiskSize) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICACHESTORAGECONSUMPTIONOBSERVER \
  nsresult OnNetworkCacheDiskConsumption(int64_t aDiskSize); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICACHESTORAGECONSUMPTIONOBSERVER(_to) \
  NS_IMETHOD OnNetworkCacheDiskConsumption(int64_t aDiskSize) override { return _to OnNetworkCacheDiskConsumption(aDiskSize); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICACHESTORAGECONSUMPTIONOBSERVER(_to) \
  NS_IMETHOD OnNetworkCacheDiskConsumption(int64_t aDiskSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnNetworkCacheDiskConsumption(aDiskSize); } 


#endif /* __gen_nsICacheStorageService_h__ */
