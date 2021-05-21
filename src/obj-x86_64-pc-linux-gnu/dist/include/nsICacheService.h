/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache/nsICacheService.idl
 */

#ifndef __gen_nsICacheService_h__
#define __gen_nsICacheService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsICache_h__
#include "nsICache.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsISimpleEnumerator; /* forward declaration */

class nsICacheListener; /* forward declaration */

class nsICacheSession; /* forward declaration */

class nsICacheVisitor; /* forward declaration */

class nsIEventTarget; /* forward declaration */


/* starting interface:    nsICacheService */
#define NS_ICACHESERVICE_IID_STR "14dbe1e9-f3bc-45af-92f4-2c574fcd4e39"

#define NS_ICACHESERVICE_IID \
  {0x14dbe1e9, 0xf3bc, 0x45af, \
    { 0x92, 0xf4, 0x2c, 0x57, 0x4f, 0xcd, 0x4e, 0x39 }}

class NS_NO_VTABLE nsICacheService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICACHESERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICacheService;

  /* nsICacheSession createSession (in string clientID, in nsCacheStoragePolicy storagePolicy, in boolean streamBased); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CreateSession(const char * clientID, nsCacheStoragePolicy storagePolicy, bool streamBased, nsICacheSession **_retval) = 0;

  /* void visitEntries (in nsICacheVisitor visitor); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD VisitEntries(nsICacheVisitor *visitor) = 0;

  /* void evictEntries (in nsCacheStoragePolicy storagePolicy); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD EvictEntries(nsCacheStoragePolicy storagePolicy) = 0;

  /* readonly attribute nsIEventTarget cacheIOTarget; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCacheIOTarget(nsIEventTarget **aCacheIOTarget) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICacheService, NS_ICACHESERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICACHESERVICE \
  NS_IMETHOD CreateSession(const char * clientID, nsCacheStoragePolicy storagePolicy, bool streamBased, nsICacheSession **_retval) override; \
  NS_IMETHOD VisitEntries(nsICacheVisitor *visitor) override; \
  NS_IMETHOD EvictEntries(nsCacheStoragePolicy storagePolicy) override; \
  NS_IMETHOD GetCacheIOTarget(nsIEventTarget **aCacheIOTarget) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICACHESERVICE \
  nsresult CreateSession(const char * clientID, nsCacheStoragePolicy storagePolicy, bool streamBased, nsICacheSession **_retval); \
  nsresult VisitEntries(nsICacheVisitor *visitor); \
  nsresult EvictEntries(nsCacheStoragePolicy storagePolicy); \
  nsresult GetCacheIOTarget(nsIEventTarget **aCacheIOTarget); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICACHESERVICE(_to) \
  NS_IMETHOD CreateSession(const char * clientID, nsCacheStoragePolicy storagePolicy, bool streamBased, nsICacheSession **_retval) override { return _to CreateSession(clientID, storagePolicy, streamBased, _retval); } \
  NS_IMETHOD VisitEntries(nsICacheVisitor *visitor) override { return _to VisitEntries(visitor); } \
  NS_IMETHOD EvictEntries(nsCacheStoragePolicy storagePolicy) override { return _to EvictEntries(storagePolicy); } \
  NS_IMETHOD GetCacheIOTarget(nsIEventTarget **aCacheIOTarget) override { return _to GetCacheIOTarget(aCacheIOTarget); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICACHESERVICE(_to) \
  NS_IMETHOD CreateSession(const char * clientID, nsCacheStoragePolicy storagePolicy, bool streamBased, nsICacheSession **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateSession(clientID, storagePolicy, streamBased, _retval); } \
  NS_IMETHOD VisitEntries(nsICacheVisitor *visitor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->VisitEntries(visitor); } \
  NS_IMETHOD EvictEntries(nsCacheStoragePolicy storagePolicy) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EvictEntries(storagePolicy); } \
  NS_IMETHOD GetCacheIOTarget(nsIEventTarget **aCacheIOTarget) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCacheIOTarget(aCacheIOTarget); } 

/**
 * Observer service notification that is sent when
 * nsICacheService::evictEntries() or nsICacheSession::evictEntries()
 * is called.
 */
#define NS_CACHESERVICE_EMPTYCACHE_TOPIC_ID "cacheservice:empty-cache"

/* starting interface:    nsICacheServiceInternal */
#define NS_ICACHESERVICEINTERNAL_IID_STR "d0fc8d38-db80-4928-bf1c-b0085ddfa9dc"

#define NS_ICACHESERVICEINTERNAL_IID \
  {0xd0fc8d38, 0xdb80, 0x4928, \
    { 0xbf, 0x1c, 0xb0, 0x08, 0x5d, 0xdf, 0xa9, 0xdc }}

class NS_NO_VTABLE nsICacheServiceInternal : public nsICacheService {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICACHESERVICEINTERNAL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICacheServiceInternal;

  /* readonly attribute double lockHeldTime; */
  NS_IMETHOD GetLockHeldTime(double *aLockHeldTime) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICacheServiceInternal, NS_ICACHESERVICEINTERNAL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICACHESERVICEINTERNAL \
  NS_IMETHOD GetLockHeldTime(double *aLockHeldTime) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICACHESERVICEINTERNAL \
  nsresult GetLockHeldTime(double *aLockHeldTime); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICACHESERVICEINTERNAL(_to) \
  NS_IMETHOD GetLockHeldTime(double *aLockHeldTime) override { return _to GetLockHeldTime(aLockHeldTime); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICACHESERVICEINTERNAL(_to) \
  NS_IMETHOD GetLockHeldTime(double *aLockHeldTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLockHeldTime(aLockHeldTime); } 


#endif /* __gen_nsICacheService_h__ */
