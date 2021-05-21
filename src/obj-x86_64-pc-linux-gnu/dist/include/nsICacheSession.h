/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache/nsICacheSession.idl
 */

#ifndef __gen_nsICacheSession_h__
#define __gen_nsICacheSession_h__


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
class nsICacheEntryDescriptor; /* forward declaration */

class nsICacheListener; /* forward declaration */

class nsIFile; /* forward declaration */


/* starting interface:    nsICacheSession */
#define NS_ICACHESESSION_IID_STR "1dd7708c-de48-4ffe-b5aa-cd218c762887"

#define NS_ICACHESESSION_IID \
  {0x1dd7708c, 0xde48, 0x4ffe, \
    { 0xb5, 0xaa, 0xcd, 0x21, 0x8c, 0x76, 0x28, 0x87 }}

class NS_NO_VTABLE nsICacheSession : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICACHESESSION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICacheSession;

  /* attribute boolean doomEntriesIfExpired; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDoomEntriesIfExpired(bool *aDoomEntriesIfExpired) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDoomEntriesIfExpired(bool aDoomEntriesIfExpired) = 0;

  /* attribute nsIFile profileDirectory; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetProfileDirectory(nsIFile **aProfileDirectory) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetProfileDirectory(nsIFile *aProfileDirectory) = 0;

  /* nsICacheEntryDescriptor openCacheEntry (in ACString key, in nsCacheAccessMode accessRequested, in boolean blockingMode); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OpenCacheEntry(const nsACString& key, nsCacheAccessMode accessRequested, bool blockingMode, nsICacheEntryDescriptor **_retval) = 0;

  /* void asyncOpenCacheEntry (in ACString key, in nsCacheAccessMode accessRequested, in nsICacheListener listener, [optional] in boolean noWait); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncOpenCacheEntry(const nsACString& key, nsCacheAccessMode accessRequested, nsICacheListener *listener, bool noWait) = 0;

  /* void evictEntries (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD EvictEntries(void) = 0;

  /* boolean isStorageEnabled (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsStorageEnabled(bool *_retval) = 0;

  /* void doomEntry (in ACString key, in nsICacheListener listener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DoomEntry(const nsACString& key, nsICacheListener *listener) = 0;

  /* attribute boolean isPrivate; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsPrivate(bool *aIsPrivate) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetIsPrivate(bool aIsPrivate) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICacheSession, NS_ICACHESESSION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICACHESESSION \
  NS_IMETHOD GetDoomEntriesIfExpired(bool *aDoomEntriesIfExpired) override; \
  NS_IMETHOD SetDoomEntriesIfExpired(bool aDoomEntriesIfExpired) override; \
  NS_IMETHOD GetProfileDirectory(nsIFile **aProfileDirectory) override; \
  NS_IMETHOD SetProfileDirectory(nsIFile *aProfileDirectory) override; \
  NS_IMETHOD OpenCacheEntry(const nsACString& key, nsCacheAccessMode accessRequested, bool blockingMode, nsICacheEntryDescriptor **_retval) override; \
  NS_IMETHOD AsyncOpenCacheEntry(const nsACString& key, nsCacheAccessMode accessRequested, nsICacheListener *listener, bool noWait) override; \
  NS_IMETHOD EvictEntries(void) override; \
  NS_IMETHOD IsStorageEnabled(bool *_retval) override; \
  NS_IMETHOD DoomEntry(const nsACString& key, nsICacheListener *listener) override; \
  NS_IMETHOD GetIsPrivate(bool *aIsPrivate) override; \
  NS_IMETHOD SetIsPrivate(bool aIsPrivate) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICACHESESSION \
  nsresult GetDoomEntriesIfExpired(bool *aDoomEntriesIfExpired); \
  nsresult SetDoomEntriesIfExpired(bool aDoomEntriesIfExpired); \
  nsresult GetProfileDirectory(nsIFile **aProfileDirectory); \
  nsresult SetProfileDirectory(nsIFile *aProfileDirectory); \
  nsresult OpenCacheEntry(const nsACString& key, nsCacheAccessMode accessRequested, bool blockingMode, nsICacheEntryDescriptor **_retval); \
  nsresult AsyncOpenCacheEntry(const nsACString& key, nsCacheAccessMode accessRequested, nsICacheListener *listener, bool noWait); \
  nsresult EvictEntries(void); \
  nsresult IsStorageEnabled(bool *_retval); \
  nsresult DoomEntry(const nsACString& key, nsICacheListener *listener); \
  nsresult GetIsPrivate(bool *aIsPrivate); \
  nsresult SetIsPrivate(bool aIsPrivate); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICACHESESSION(_to) \
  NS_IMETHOD GetDoomEntriesIfExpired(bool *aDoomEntriesIfExpired) override { return _to GetDoomEntriesIfExpired(aDoomEntriesIfExpired); } \
  NS_IMETHOD SetDoomEntriesIfExpired(bool aDoomEntriesIfExpired) override { return _to SetDoomEntriesIfExpired(aDoomEntriesIfExpired); } \
  NS_IMETHOD GetProfileDirectory(nsIFile **aProfileDirectory) override { return _to GetProfileDirectory(aProfileDirectory); } \
  NS_IMETHOD SetProfileDirectory(nsIFile *aProfileDirectory) override { return _to SetProfileDirectory(aProfileDirectory); } \
  NS_IMETHOD OpenCacheEntry(const nsACString& key, nsCacheAccessMode accessRequested, bool blockingMode, nsICacheEntryDescriptor **_retval) override { return _to OpenCacheEntry(key, accessRequested, blockingMode, _retval); } \
  NS_IMETHOD AsyncOpenCacheEntry(const nsACString& key, nsCacheAccessMode accessRequested, nsICacheListener *listener, bool noWait) override { return _to AsyncOpenCacheEntry(key, accessRequested, listener, noWait); } \
  NS_IMETHOD EvictEntries(void) override { return _to EvictEntries(); } \
  NS_IMETHOD IsStorageEnabled(bool *_retval) override { return _to IsStorageEnabled(_retval); } \
  NS_IMETHOD DoomEntry(const nsACString& key, nsICacheListener *listener) override { return _to DoomEntry(key, listener); } \
  NS_IMETHOD GetIsPrivate(bool *aIsPrivate) override { return _to GetIsPrivate(aIsPrivate); } \
  NS_IMETHOD SetIsPrivate(bool aIsPrivate) override { return _to SetIsPrivate(aIsPrivate); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICACHESESSION(_to) \
  NS_IMETHOD GetDoomEntriesIfExpired(bool *aDoomEntriesIfExpired) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDoomEntriesIfExpired(aDoomEntriesIfExpired); } \
  NS_IMETHOD SetDoomEntriesIfExpired(bool aDoomEntriesIfExpired) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDoomEntriesIfExpired(aDoomEntriesIfExpired); } \
  NS_IMETHOD GetProfileDirectory(nsIFile **aProfileDirectory) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProfileDirectory(aProfileDirectory); } \
  NS_IMETHOD SetProfileDirectory(nsIFile *aProfileDirectory) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetProfileDirectory(aProfileDirectory); } \
  NS_IMETHOD OpenCacheEntry(const nsACString& key, nsCacheAccessMode accessRequested, bool blockingMode, nsICacheEntryDescriptor **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OpenCacheEntry(key, accessRequested, blockingMode, _retval); } \
  NS_IMETHOD AsyncOpenCacheEntry(const nsACString& key, nsCacheAccessMode accessRequested, nsICacheListener *listener, bool noWait) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncOpenCacheEntry(key, accessRequested, listener, noWait); } \
  NS_IMETHOD EvictEntries(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EvictEntries(); } \
  NS_IMETHOD IsStorageEnabled(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsStorageEnabled(_retval); } \
  NS_IMETHOD DoomEntry(const nsACString& key, nsICacheListener *listener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DoomEntry(key, listener); } \
  NS_IMETHOD GetIsPrivate(bool *aIsPrivate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsPrivate(aIsPrivate); } \
  NS_IMETHOD SetIsPrivate(bool aIsPrivate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetIsPrivate(aIsPrivate); } 


#endif /* __gen_nsICacheSession_h__ */
