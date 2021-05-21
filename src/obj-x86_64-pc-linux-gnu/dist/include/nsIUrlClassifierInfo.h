/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/url-classifier/nsIUrlClassifierInfo.idl
 */

#ifndef __gen_nsIUrlClassifierInfo_h__
#define __gen_nsIUrlClassifierInfo_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIArray_h__
#include "nsIArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIUrlClassifierPositiveCacheEntry */
#define NS_IURLCLASSIFIERPOSITIVECACHEENTRY_IID_STR "b3c27f8c-7db8-4f3f-97a5-5a94d781e565"

#define NS_IURLCLASSIFIERPOSITIVECACHEENTRY_IID \
  {0xb3c27f8c, 0x7db8, 0x4f3f, \
    { 0x97, 0xa5, 0x5a, 0x94, 0xd7, 0x81, 0xe5, 0x65 }}

class NS_NO_VTABLE nsIUrlClassifierPositiveCacheEntry : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURLCLASSIFIERPOSITIVECACHEENTRY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUrlClassifierPositiveCacheEntry;

  /* readonly attribute ACString fullhash; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFullhash(nsACString& aFullhash) = 0;

  /* readonly attribute long long expiry; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetExpiry(int64_t *aExpiry) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUrlClassifierPositiveCacheEntry, NS_IURLCLASSIFIERPOSITIVECACHEENTRY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURLCLASSIFIERPOSITIVECACHEENTRY \
  NS_IMETHOD GetFullhash(nsACString& aFullhash) override; \
  NS_IMETHOD GetExpiry(int64_t *aExpiry) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURLCLASSIFIERPOSITIVECACHEENTRY \
  nsresult GetFullhash(nsACString& aFullhash); \
  nsresult GetExpiry(int64_t *aExpiry); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURLCLASSIFIERPOSITIVECACHEENTRY(_to) \
  NS_IMETHOD GetFullhash(nsACString& aFullhash) override { return _to GetFullhash(aFullhash); } \
  NS_IMETHOD GetExpiry(int64_t *aExpiry) override { return _to GetExpiry(aExpiry); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURLCLASSIFIERPOSITIVECACHEENTRY(_to) \
  NS_IMETHOD GetFullhash(nsACString& aFullhash) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFullhash(aFullhash); } \
  NS_IMETHOD GetExpiry(int64_t *aExpiry) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetExpiry(aExpiry); } 


/* starting interface:    nsIUrlClassifierCacheEntry */
#define NS_IURLCLASSIFIERCACHEENTRY_IID_STR "d6297907-8236-4126-adaf-c3aa239a0d40"

#define NS_IURLCLASSIFIERCACHEENTRY_IID \
  {0xd6297907, 0x8236, 0x4126, \
    { 0xad, 0xaf, 0xc3, 0xaa, 0x23, 0x9a, 0x0d, 0x40 }}

class NS_NO_VTABLE nsIUrlClassifierCacheEntry : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURLCLASSIFIERCACHEENTRY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUrlClassifierCacheEntry;

  /* readonly attribute ACString prefix; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPrefix(nsACString& aPrefix) = 0;

  /* readonly attribute long long expiry; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetExpiry(int64_t *aExpiry) = 0;

  /* readonly attribute nsIArray matches; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMatches(nsIArray **aMatches) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUrlClassifierCacheEntry, NS_IURLCLASSIFIERCACHEENTRY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURLCLASSIFIERCACHEENTRY \
  NS_IMETHOD GetPrefix(nsACString& aPrefix) override; \
  NS_IMETHOD GetExpiry(int64_t *aExpiry) override; \
  NS_IMETHOD GetMatches(nsIArray **aMatches) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURLCLASSIFIERCACHEENTRY \
  nsresult GetPrefix(nsACString& aPrefix); \
  nsresult GetExpiry(int64_t *aExpiry); \
  nsresult GetMatches(nsIArray **aMatches); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURLCLASSIFIERCACHEENTRY(_to) \
  NS_IMETHOD GetPrefix(nsACString& aPrefix) override { return _to GetPrefix(aPrefix); } \
  NS_IMETHOD GetExpiry(int64_t *aExpiry) override { return _to GetExpiry(aExpiry); } \
  NS_IMETHOD GetMatches(nsIArray **aMatches) override { return _to GetMatches(aMatches); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURLCLASSIFIERCACHEENTRY(_to) \
  NS_IMETHOD GetPrefix(nsACString& aPrefix) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrefix(aPrefix); } \
  NS_IMETHOD GetExpiry(int64_t *aExpiry) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetExpiry(aExpiry); } \
  NS_IMETHOD GetMatches(nsIArray **aMatches) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMatches(aMatches); } 


/* starting interface:    nsIUrlClassifierCacheInfo */
#define NS_IURLCLASSIFIERCACHEINFO_IID_STR "69384f24-d9c5-4462-b24e-351c69e3b46a"

#define NS_IURLCLASSIFIERCACHEINFO_IID \
  {0x69384f24, 0xd9c5, 0x4462, \
    { 0xb2, 0x4e, 0x35, 0x1c, 0x69, 0xe3, 0xb4, 0x6a }}

class NS_NO_VTABLE nsIUrlClassifierCacheInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURLCLASSIFIERCACHEINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUrlClassifierCacheInfo;

  /* readonly attribute ACString table; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTable(nsACString& aTable) = 0;

  /* readonly attribute nsIArray entries; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEntries(nsIArray **aEntries) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUrlClassifierCacheInfo, NS_IURLCLASSIFIERCACHEINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURLCLASSIFIERCACHEINFO \
  NS_IMETHOD GetTable(nsACString& aTable) override; \
  NS_IMETHOD GetEntries(nsIArray **aEntries) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURLCLASSIFIERCACHEINFO \
  nsresult GetTable(nsACString& aTable); \
  nsresult GetEntries(nsIArray **aEntries); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURLCLASSIFIERCACHEINFO(_to) \
  NS_IMETHOD GetTable(nsACString& aTable) override { return _to GetTable(aTable); } \
  NS_IMETHOD GetEntries(nsIArray **aEntries) override { return _to GetEntries(aEntries); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURLCLASSIFIERCACHEINFO(_to) \
  NS_IMETHOD GetTable(nsACString& aTable) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTable(aTable); } \
  NS_IMETHOD GetEntries(nsIArray **aEntries) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEntries(aEntries); } 


/* starting interface:    nsIUrlClassifierGetCacheCallback */
#define NS_IURLCLASSIFIERGETCACHECALLBACK_IID_STR "26e12ea4-14ff-4c77-858f-6745998b7659"

#define NS_IURLCLASSIFIERGETCACHECALLBACK_IID \
  {0x26e12ea4, 0x14ff, 0x4c77, \
    { 0x85, 0x8f, 0x67, 0x45, 0x99, 0x8b, 0x76, 0x59 }}

class NS_NO_VTABLE nsIUrlClassifierGetCacheCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURLCLASSIFIERGETCACHECALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUrlClassifierGetCacheCallback;

  /* void onGetCacheComplete (in nsIUrlClassifierCacheInfo info); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnGetCacheComplete(nsIUrlClassifierCacheInfo *info) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUrlClassifierGetCacheCallback, NS_IURLCLASSIFIERGETCACHECALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURLCLASSIFIERGETCACHECALLBACK \
  NS_IMETHOD OnGetCacheComplete(nsIUrlClassifierCacheInfo *info) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURLCLASSIFIERGETCACHECALLBACK \
  nsresult OnGetCacheComplete(nsIUrlClassifierCacheInfo *info); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURLCLASSIFIERGETCACHECALLBACK(_to) \
  NS_IMETHOD OnGetCacheComplete(nsIUrlClassifierCacheInfo *info) override { return _to OnGetCacheComplete(info); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURLCLASSIFIERGETCACHECALLBACK(_to) \
  NS_IMETHOD OnGetCacheComplete(nsIUrlClassifierCacheInfo *info) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnGetCacheComplete(info); } 


/* starting interface:    nsIUrlClassifierInfo */
#define NS_IURLCLASSIFIERINFO_IID_STR "411bbff4-1b88-4687-aa36-e2bbdd93f6e8"

#define NS_IURLCLASSIFIERINFO_IID \
  {0x411bbff4, 0x1b88, 0x4687, \
    { 0xaa, 0x36, 0xe2, 0xbb, 0xdd, 0x93, 0xf6, 0xe8 }}

class NS_NO_VTABLE nsIUrlClassifierInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURLCLASSIFIERINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUrlClassifierInfo;

  /* void getCacheInfo (in ACString table, in nsIUrlClassifierGetCacheCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCacheInfo(const nsACString& table, nsIUrlClassifierGetCacheCallback *callback) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUrlClassifierInfo, NS_IURLCLASSIFIERINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURLCLASSIFIERINFO \
  NS_IMETHOD GetCacheInfo(const nsACString& table, nsIUrlClassifierGetCacheCallback *callback) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURLCLASSIFIERINFO \
  nsresult GetCacheInfo(const nsACString& table, nsIUrlClassifierGetCacheCallback *callback); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURLCLASSIFIERINFO(_to) \
  NS_IMETHOD GetCacheInfo(const nsACString& table, nsIUrlClassifierGetCacheCallback *callback) override { return _to GetCacheInfo(table, callback); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURLCLASSIFIERINFO(_to) \
  NS_IMETHOD GetCacheInfo(const nsACString& table, nsIUrlClassifierGetCacheCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCacheInfo(table, callback); } 


#endif /* __gen_nsIUrlClassifierInfo_h__ */
