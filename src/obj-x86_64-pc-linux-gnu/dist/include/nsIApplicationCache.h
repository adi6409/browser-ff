/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIApplicationCache.idl
 */

#ifndef __gen_nsIApplicationCache_h__
#define __gen_nsIApplicationCache_h__


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
class nsIArray; /* forward declaration */

class nsIFile; /* forward declaration */

class nsIURI; /* forward declaration */


/* starting interface:    nsIApplicationCacheNamespace */
#define NS_IAPPLICATIONCACHENAMESPACE_IID_STR "96e4c264-2065-4ce9-93bb-43734c62c4eb"

#define NS_IAPPLICATIONCACHENAMESPACE_IID \
  {0x96e4c264, 0x2065, 0x4ce9, \
    { 0x93, 0xbb, 0x43, 0x73, 0x4c, 0x62, 0xc4, 0xeb }}

class NS_NO_VTABLE nsIApplicationCacheNamespace : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IAPPLICATIONCACHENAMESPACE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIApplicationCacheNamespace;

  enum {
    NAMESPACE_BYPASS = 1U,
    NAMESPACE_FALLBACK = 2U,
    NAMESPACE_OPPORTUNISTIC = 4U
  };

  /* void init (in unsigned long itemType, in ACString namespaceSpec, in ACString data); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(uint32_t itemType, const nsACString& namespaceSpec, const nsACString& data) = 0;

  /* readonly attribute unsigned long itemType; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetItemType(uint32_t *aItemType) = 0;

  /* readonly attribute ACString namespaceSpec; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNamespaceSpec(nsACString& aNamespaceSpec) = 0;

  /* readonly attribute ACString data; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetData(nsACString& aData) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIApplicationCacheNamespace, NS_IAPPLICATIONCACHENAMESPACE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIAPPLICATIONCACHENAMESPACE \
  NS_IMETHOD Init(uint32_t itemType, const nsACString& namespaceSpec, const nsACString& data) override; \
  NS_IMETHOD GetItemType(uint32_t *aItemType) override; \
  NS_IMETHOD GetNamespaceSpec(nsACString& aNamespaceSpec) override; \
  NS_IMETHOD GetData(nsACString& aData) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIAPPLICATIONCACHENAMESPACE \
  nsresult Init(uint32_t itemType, const nsACString& namespaceSpec, const nsACString& data); \
  nsresult GetItemType(uint32_t *aItemType); \
  nsresult GetNamespaceSpec(nsACString& aNamespaceSpec); \
  nsresult GetData(nsACString& aData); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIAPPLICATIONCACHENAMESPACE(_to) \
  NS_IMETHOD Init(uint32_t itemType, const nsACString& namespaceSpec, const nsACString& data) override { return _to Init(itemType, namespaceSpec, data); } \
  NS_IMETHOD GetItemType(uint32_t *aItemType) override { return _to GetItemType(aItemType); } \
  NS_IMETHOD GetNamespaceSpec(nsACString& aNamespaceSpec) override { return _to GetNamespaceSpec(aNamespaceSpec); } \
  NS_IMETHOD GetData(nsACString& aData) override { return _to GetData(aData); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIAPPLICATIONCACHENAMESPACE(_to) \
  NS_IMETHOD Init(uint32_t itemType, const nsACString& namespaceSpec, const nsACString& data) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(itemType, namespaceSpec, data); } \
  NS_IMETHOD GetItemType(uint32_t *aItemType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetItemType(aItemType); } \
  NS_IMETHOD GetNamespaceSpec(nsACString& aNamespaceSpec) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNamespaceSpec(aNamespaceSpec); } \
  NS_IMETHOD GetData(nsACString& aData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetData(aData); } 


/* starting interface:    nsIApplicationCache */
#define NS_IAPPLICATIONCACHE_IID_STR "06568dae-c374-4383-a122-0cc96c7177f2"

#define NS_IAPPLICATIONCACHE_IID \
  {0x06568dae, 0xc374, 0x4383, \
    { 0xa1, 0x22, 0x0c, 0xc9, 0x6c, 0x71, 0x77, 0xf2 }}

class NS_NO_VTABLE nsIApplicationCache : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IAPPLICATIONCACHE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIApplicationCache;

  /* void initAsHandle (in ACString groupId, in ACString clientId); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD InitAsHandle(const nsACString& groupId, const nsACString& clientId) = 0;

  enum {
    ITEM_MANIFEST = 1U,
    ITEM_EXPLICIT = 2U,
    ITEM_IMPLICIT = 4U,
    ITEM_DYNAMIC = 8U,
    ITEM_FOREIGN = 16U,
    ITEM_FALLBACK = 32U,
    ITEM_OPPORTUNISTIC = 64U
  };

  /* readonly attribute nsIURI manifestURI; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetManifestURI(nsIURI **aManifestURI) = 0;

  /* readonly attribute ACString groupID; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetGroupID(nsACString& aGroupID) = 0;

  /* readonly attribute ACString clientID; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetClientID(nsACString& aClientID) = 0;

  /* readonly attribute boolean active; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetActive(bool *aActive) = 0;

  /* readonly attribute unsigned long usage; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUsage(uint32_t *aUsage) = 0;

  /* void activate (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Activate(void) = 0;

  /* void discard (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Discard(void) = 0;

  /* void markEntry (in ACString key, in unsigned long typeBits); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MarkEntry(const nsACString& key, uint32_t typeBits) = 0;

  /* void unmarkEntry (in ACString key, in unsigned long typeBits); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UnmarkEntry(const nsACString& key, uint32_t typeBits) = 0;

  /* unsigned long getTypes (in ACString key); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTypes(const nsACString& key, uint32_t *_retval) = 0;

  /* Array<ACString> gatherEntries (in uint32_t typeBits); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GatherEntries(uint32_t typeBits, nsTArray<nsCString >& _retval) = 0;

  /* void addNamespaces (in nsIArray namespaces); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddNamespaces(nsIArray *namespaces) = 0;

  /* nsIApplicationCacheNamespace getMatchingNamespace (in ACString key); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMatchingNamespace(const nsACString& key, nsIApplicationCacheNamespace **_retval) = 0;

  /* readonly attribute nsIFile profileDirectory; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetProfileDirectory(nsIFile **aProfileDirectory) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIApplicationCache, NS_IAPPLICATIONCACHE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIAPPLICATIONCACHE \
  NS_IMETHOD InitAsHandle(const nsACString& groupId, const nsACString& clientId) override; \
  NS_IMETHOD GetManifestURI(nsIURI **aManifestURI) override; \
  NS_IMETHOD GetGroupID(nsACString& aGroupID) override; \
  NS_IMETHOD GetClientID(nsACString& aClientID) override; \
  NS_IMETHOD GetActive(bool *aActive) override; \
  NS_IMETHOD GetUsage(uint32_t *aUsage) override; \
  NS_IMETHOD Activate(void) override; \
  NS_IMETHOD Discard(void) override; \
  NS_IMETHOD MarkEntry(const nsACString& key, uint32_t typeBits) override; \
  NS_IMETHOD UnmarkEntry(const nsACString& key, uint32_t typeBits) override; \
  NS_IMETHOD GetTypes(const nsACString& key, uint32_t *_retval) override; \
  NS_IMETHOD GatherEntries(uint32_t typeBits, nsTArray<nsCString >& _retval) override; \
  NS_IMETHOD AddNamespaces(nsIArray *namespaces) override; \
  NS_IMETHOD GetMatchingNamespace(const nsACString& key, nsIApplicationCacheNamespace **_retval) override; \
  NS_IMETHOD GetProfileDirectory(nsIFile **aProfileDirectory) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIAPPLICATIONCACHE \
  nsresult InitAsHandle(const nsACString& groupId, const nsACString& clientId); \
  nsresult GetManifestURI(nsIURI **aManifestURI); \
  nsresult GetGroupID(nsACString& aGroupID); \
  nsresult GetClientID(nsACString& aClientID); \
  nsresult GetActive(bool *aActive); \
  nsresult GetUsage(uint32_t *aUsage); \
  nsresult Activate(void); \
  nsresult Discard(void); \
  nsresult MarkEntry(const nsACString& key, uint32_t typeBits); \
  nsresult UnmarkEntry(const nsACString& key, uint32_t typeBits); \
  nsresult GetTypes(const nsACString& key, uint32_t *_retval); \
  nsresult GatherEntries(uint32_t typeBits, nsTArray<nsCString >& _retval); \
  nsresult AddNamespaces(nsIArray *namespaces); \
  nsresult GetMatchingNamespace(const nsACString& key, nsIApplicationCacheNamespace **_retval); \
  nsresult GetProfileDirectory(nsIFile **aProfileDirectory); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIAPPLICATIONCACHE(_to) \
  NS_IMETHOD InitAsHandle(const nsACString& groupId, const nsACString& clientId) override { return _to InitAsHandle(groupId, clientId); } \
  NS_IMETHOD GetManifestURI(nsIURI **aManifestURI) override { return _to GetManifestURI(aManifestURI); } \
  NS_IMETHOD GetGroupID(nsACString& aGroupID) override { return _to GetGroupID(aGroupID); } \
  NS_IMETHOD GetClientID(nsACString& aClientID) override { return _to GetClientID(aClientID); } \
  NS_IMETHOD GetActive(bool *aActive) override { return _to GetActive(aActive); } \
  NS_IMETHOD GetUsage(uint32_t *aUsage) override { return _to GetUsage(aUsage); } \
  NS_IMETHOD Activate(void) override { return _to Activate(); } \
  NS_IMETHOD Discard(void) override { return _to Discard(); } \
  NS_IMETHOD MarkEntry(const nsACString& key, uint32_t typeBits) override { return _to MarkEntry(key, typeBits); } \
  NS_IMETHOD UnmarkEntry(const nsACString& key, uint32_t typeBits) override { return _to UnmarkEntry(key, typeBits); } \
  NS_IMETHOD GetTypes(const nsACString& key, uint32_t *_retval) override { return _to GetTypes(key, _retval); } \
  NS_IMETHOD GatherEntries(uint32_t typeBits, nsTArray<nsCString >& _retval) override { return _to GatherEntries(typeBits, _retval); } \
  NS_IMETHOD AddNamespaces(nsIArray *namespaces) override { return _to AddNamespaces(namespaces); } \
  NS_IMETHOD GetMatchingNamespace(const nsACString& key, nsIApplicationCacheNamespace **_retval) override { return _to GetMatchingNamespace(key, _retval); } \
  NS_IMETHOD GetProfileDirectory(nsIFile **aProfileDirectory) override { return _to GetProfileDirectory(aProfileDirectory); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIAPPLICATIONCACHE(_to) \
  NS_IMETHOD InitAsHandle(const nsACString& groupId, const nsACString& clientId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitAsHandle(groupId, clientId); } \
  NS_IMETHOD GetManifestURI(nsIURI **aManifestURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetManifestURI(aManifestURI); } \
  NS_IMETHOD GetGroupID(nsACString& aGroupID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetGroupID(aGroupID); } \
  NS_IMETHOD GetClientID(nsACString& aClientID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetClientID(aClientID); } \
  NS_IMETHOD GetActive(bool *aActive) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetActive(aActive); } \
  NS_IMETHOD GetUsage(uint32_t *aUsage) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUsage(aUsage); } \
  NS_IMETHOD Activate(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Activate(); } \
  NS_IMETHOD Discard(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Discard(); } \
  NS_IMETHOD MarkEntry(const nsACString& key, uint32_t typeBits) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MarkEntry(key, typeBits); } \
  NS_IMETHOD UnmarkEntry(const nsACString& key, uint32_t typeBits) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UnmarkEntry(key, typeBits); } \
  NS_IMETHOD GetTypes(const nsACString& key, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTypes(key, _retval); } \
  NS_IMETHOD GatherEntries(uint32_t typeBits, nsTArray<nsCString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GatherEntries(typeBits, _retval); } \
  NS_IMETHOD AddNamespaces(nsIArray *namespaces) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddNamespaces(namespaces); } \
  NS_IMETHOD GetMatchingNamespace(const nsACString& key, nsIApplicationCacheNamespace **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMatchingNamespace(key, _retval); } \
  NS_IMETHOD GetProfileDirectory(nsIFile **aProfileDirectory) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProfileDirectory(aProfileDirectory); } 


#endif /* __gen_nsIApplicationCache_h__ */
