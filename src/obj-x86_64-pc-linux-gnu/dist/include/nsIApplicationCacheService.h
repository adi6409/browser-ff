/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIApplicationCacheService.idl
 */

#ifndef __gen_nsIApplicationCacheService_h__
#define __gen_nsIApplicationCacheService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIApplicationCache; /* forward declaration */

class nsIFile; /* forward declaration */

class nsIURI; /* forward declaration */

class nsILoadContextInfo; /* forward declaration */


/* starting interface:    nsIApplicationCacheService */
#define NS_IAPPLICATIONCACHESERVICE_IID_STR "b8b6546c-6cec-4bda-82df-08e006a97b56"

#define NS_IAPPLICATIONCACHESERVICE_IID \
  {0xb8b6546c, 0x6cec, 0x4bda, \
    { 0x82, 0xdf, 0x08, 0xe0, 0x06, 0xa9, 0x7b, 0x56 }}

class NS_NO_VTABLE nsIApplicationCacheService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IAPPLICATIONCACHESERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIApplicationCacheService;

  /* ACString buildGroupIDForInfo (in nsIURI aManifestURL, in nsILoadContextInfo aLoadContextInfo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD BuildGroupIDForInfo(nsIURI *aManifestURL, nsILoadContextInfo *aLoadContextInfo, nsACString& _retval) = 0;

  /* ACString buildGroupIDForSuffix (in nsIURI aManifestURL, in ACString aOriginSuffix); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD BuildGroupIDForSuffix(nsIURI *aManifestURL, const nsACString& aOriginSuffix, nsACString& _retval) = 0;

  /* nsIApplicationCache createApplicationCache (in ACString group); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CreateApplicationCache(const nsACString& group, nsIApplicationCache **_retval) = 0;

  /* nsIApplicationCache createCustomApplicationCache (in ACString group, in nsIFile profileDir, in int32_t quota); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CreateCustomApplicationCache(const nsACString& group, nsIFile *profileDir, int32_t quota, nsIApplicationCache **_retval) = 0;

  /* nsIApplicationCache getApplicationCache (in ACString clientID); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetApplicationCache(const nsACString& clientID, nsIApplicationCache **_retval) = 0;

  /* nsIApplicationCache getActiveCache (in ACString group); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetActiveCache(const nsACString& group, nsIApplicationCache **_retval) = 0;

  /* void deactivateGroup (in ACString group); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DeactivateGroup(const nsACString& group) = 0;

  /* void evict (in nsILoadContextInfo aLoadContextInfo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Evict(nsILoadContextInfo *aLoadContextInfo) = 0;

  /* void evictMatchingOriginAttributes (in AString aPattern); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD EvictMatchingOriginAttributes(const nsAString& aPattern) = 0;

  /* nsIApplicationCache chooseApplicationCache (in ACString key, [optional] in nsILoadContextInfo aLoadContextInfo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ChooseApplicationCache(const nsACString& key, nsILoadContextInfo *aLoadContextInfo, nsIApplicationCache **_retval) = 0;

  /* void cacheOpportunistically (in nsIApplicationCache cache, in ACString key); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CacheOpportunistically(nsIApplicationCache *cache, const nsACString& key) = 0;

  /* Array<ACString> getGroups (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetGroups(nsTArray<nsCString >& _retval) = 0;

  /* Array<ACString> getGroupsTimeOrdered (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetGroupsTimeOrdered(nsTArray<nsCString >& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIApplicationCacheService, NS_IAPPLICATIONCACHESERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIAPPLICATIONCACHESERVICE \
  NS_IMETHOD BuildGroupIDForInfo(nsIURI *aManifestURL, nsILoadContextInfo *aLoadContextInfo, nsACString& _retval) override; \
  NS_IMETHOD BuildGroupIDForSuffix(nsIURI *aManifestURL, const nsACString& aOriginSuffix, nsACString& _retval) override; \
  NS_IMETHOD CreateApplicationCache(const nsACString& group, nsIApplicationCache **_retval) override; \
  NS_IMETHOD CreateCustomApplicationCache(const nsACString& group, nsIFile *profileDir, int32_t quota, nsIApplicationCache **_retval) override; \
  NS_IMETHOD GetApplicationCache(const nsACString& clientID, nsIApplicationCache **_retval) override; \
  NS_IMETHOD GetActiveCache(const nsACString& group, nsIApplicationCache **_retval) override; \
  NS_IMETHOD DeactivateGroup(const nsACString& group) override; \
  NS_IMETHOD Evict(nsILoadContextInfo *aLoadContextInfo) override; \
  NS_IMETHOD EvictMatchingOriginAttributes(const nsAString& aPattern) override; \
  NS_IMETHOD ChooseApplicationCache(const nsACString& key, nsILoadContextInfo *aLoadContextInfo, nsIApplicationCache **_retval) override; \
  NS_IMETHOD CacheOpportunistically(nsIApplicationCache *cache, const nsACString& key) override; \
  NS_IMETHOD GetGroups(nsTArray<nsCString >& _retval) override; \
  NS_IMETHOD GetGroupsTimeOrdered(nsTArray<nsCString >& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIAPPLICATIONCACHESERVICE \
  nsresult BuildGroupIDForInfo(nsIURI *aManifestURL, nsILoadContextInfo *aLoadContextInfo, nsACString& _retval); \
  nsresult BuildGroupIDForSuffix(nsIURI *aManifestURL, const nsACString& aOriginSuffix, nsACString& _retval); \
  nsresult CreateApplicationCache(const nsACString& group, nsIApplicationCache **_retval); \
  nsresult CreateCustomApplicationCache(const nsACString& group, nsIFile *profileDir, int32_t quota, nsIApplicationCache **_retval); \
  nsresult GetApplicationCache(const nsACString& clientID, nsIApplicationCache **_retval); \
  nsresult GetActiveCache(const nsACString& group, nsIApplicationCache **_retval); \
  nsresult DeactivateGroup(const nsACString& group); \
  nsresult Evict(nsILoadContextInfo *aLoadContextInfo); \
  nsresult EvictMatchingOriginAttributes(const nsAString& aPattern); \
  nsresult ChooseApplicationCache(const nsACString& key, nsILoadContextInfo *aLoadContextInfo, nsIApplicationCache **_retval); \
  nsresult CacheOpportunistically(nsIApplicationCache *cache, const nsACString& key); \
  nsresult GetGroups(nsTArray<nsCString >& _retval); \
  nsresult GetGroupsTimeOrdered(nsTArray<nsCString >& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIAPPLICATIONCACHESERVICE(_to) \
  NS_IMETHOD BuildGroupIDForInfo(nsIURI *aManifestURL, nsILoadContextInfo *aLoadContextInfo, nsACString& _retval) override { return _to BuildGroupIDForInfo(aManifestURL, aLoadContextInfo, _retval); } \
  NS_IMETHOD BuildGroupIDForSuffix(nsIURI *aManifestURL, const nsACString& aOriginSuffix, nsACString& _retval) override { return _to BuildGroupIDForSuffix(aManifestURL, aOriginSuffix, _retval); } \
  NS_IMETHOD CreateApplicationCache(const nsACString& group, nsIApplicationCache **_retval) override { return _to CreateApplicationCache(group, _retval); } \
  NS_IMETHOD CreateCustomApplicationCache(const nsACString& group, nsIFile *profileDir, int32_t quota, nsIApplicationCache **_retval) override { return _to CreateCustomApplicationCache(group, profileDir, quota, _retval); } \
  NS_IMETHOD GetApplicationCache(const nsACString& clientID, nsIApplicationCache **_retval) override { return _to GetApplicationCache(clientID, _retval); } \
  NS_IMETHOD GetActiveCache(const nsACString& group, nsIApplicationCache **_retval) override { return _to GetActiveCache(group, _retval); } \
  NS_IMETHOD DeactivateGroup(const nsACString& group) override { return _to DeactivateGroup(group); } \
  NS_IMETHOD Evict(nsILoadContextInfo *aLoadContextInfo) override { return _to Evict(aLoadContextInfo); } \
  NS_IMETHOD EvictMatchingOriginAttributes(const nsAString& aPattern) override { return _to EvictMatchingOriginAttributes(aPattern); } \
  NS_IMETHOD ChooseApplicationCache(const nsACString& key, nsILoadContextInfo *aLoadContextInfo, nsIApplicationCache **_retval) override { return _to ChooseApplicationCache(key, aLoadContextInfo, _retval); } \
  NS_IMETHOD CacheOpportunistically(nsIApplicationCache *cache, const nsACString& key) override { return _to CacheOpportunistically(cache, key); } \
  NS_IMETHOD GetGroups(nsTArray<nsCString >& _retval) override { return _to GetGroups(_retval); } \
  NS_IMETHOD GetGroupsTimeOrdered(nsTArray<nsCString >& _retval) override { return _to GetGroupsTimeOrdered(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIAPPLICATIONCACHESERVICE(_to) \
  NS_IMETHOD BuildGroupIDForInfo(nsIURI *aManifestURL, nsILoadContextInfo *aLoadContextInfo, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->BuildGroupIDForInfo(aManifestURL, aLoadContextInfo, _retval); } \
  NS_IMETHOD BuildGroupIDForSuffix(nsIURI *aManifestURL, const nsACString& aOriginSuffix, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->BuildGroupIDForSuffix(aManifestURL, aOriginSuffix, _retval); } \
  NS_IMETHOD CreateApplicationCache(const nsACString& group, nsIApplicationCache **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateApplicationCache(group, _retval); } \
  NS_IMETHOD CreateCustomApplicationCache(const nsACString& group, nsIFile *profileDir, int32_t quota, nsIApplicationCache **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateCustomApplicationCache(group, profileDir, quota, _retval); } \
  NS_IMETHOD GetApplicationCache(const nsACString& clientID, nsIApplicationCache **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetApplicationCache(clientID, _retval); } \
  NS_IMETHOD GetActiveCache(const nsACString& group, nsIApplicationCache **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetActiveCache(group, _retval); } \
  NS_IMETHOD DeactivateGroup(const nsACString& group) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DeactivateGroup(group); } \
  NS_IMETHOD Evict(nsILoadContextInfo *aLoadContextInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Evict(aLoadContextInfo); } \
  NS_IMETHOD EvictMatchingOriginAttributes(const nsAString& aPattern) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EvictMatchingOriginAttributes(aPattern); } \
  NS_IMETHOD ChooseApplicationCache(const nsACString& key, nsILoadContextInfo *aLoadContextInfo, nsIApplicationCache **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ChooseApplicationCache(key, aLoadContextInfo, _retval); } \
  NS_IMETHOD CacheOpportunistically(nsIApplicationCache *cache, const nsACString& key) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CacheOpportunistically(cache, key); } \
  NS_IMETHOD GetGroups(nsTArray<nsCString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetGroups(_retval); } \
  NS_IMETHOD GetGroupsTimeOrdered(nsTArray<nsCString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetGroupsTimeOrdered(_retval); } 


#endif /* __gen_nsIApplicationCacheService_h__ */
