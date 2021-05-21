/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache2/nsICacheStorage.idl
 */

#ifndef __gen_nsICacheStorage_h__
#define __gen_nsICacheStorage_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class nsICacheEntry; /* forward declaration */

class nsICacheEntryOpenCallback; /* forward declaration */

class nsICacheEntryDoomCallback; /* forward declaration */

class nsICacheStorageVisitor; /* forward declaration */


/* starting interface:    nsICacheStorage */
#define NS_ICACHESTORAGE_IID_STR "35d104a6-d252-4fd4-8a56-3c14657cad3b"

#define NS_ICACHESTORAGE_IID \
  {0x35d104a6, 0xd252, 0x4fd4, \
    { 0x8a, 0x56, 0x3c, 0x14, 0x65, 0x7c, 0xad, 0x3b }}

class NS_NO_VTABLE nsICacheStorage : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICACHESTORAGE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICacheStorage;

  enum {
    OPEN_NORMALLY = 0U,
    OPEN_TRUNCATE = 1U,
    OPEN_READONLY = 2U,
    OPEN_PRIORITY = 4U,
    OPEN_BYPASS_IF_BUSY = 8U,
    CHECK_MULTITHREADED = 16U,
    OPEN_SECRETLY = 32U,
    OPEN_INTERCEPTED = 64U
  };

  /* void asyncOpenURI (in nsIURI aURI, in ACString aIdExtension, in uint32_t aFlags, in nsICacheEntryOpenCallback aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncOpenURI(nsIURI *aURI, const nsACString& aIdExtension, uint32_t aFlags, nsICacheEntryOpenCallback *aCallback) = 0;

  /* nsICacheEntry openTruncate (in nsIURI aURI, in ACString aIdExtension); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OpenTruncate(nsIURI *aURI, const nsACString& aIdExtension, nsICacheEntry **_retval) = 0;

  /* boolean exists (in nsIURI aURI, in ACString aIdExtension); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Exists(nsIURI *aURI, const nsACString& aIdExtension, bool *_retval) = 0;

  /* void getCacheIndexEntryAttrs (in nsIURI aURI, in ACString aIdExtension, out bool aHasAltData, out uint32_t aSizeInKB); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCacheIndexEntryAttrs(nsIURI *aURI, const nsACString& aIdExtension, bool *aHasAltData, uint32_t *aSizeInKB) = 0;

  /* void asyncDoomURI (in nsIURI aURI, in ACString aIdExtension, in nsICacheEntryDoomCallback aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncDoomURI(nsIURI *aURI, const nsACString& aIdExtension, nsICacheEntryDoomCallback *aCallback) = 0;

  /* void asyncEvictStorage (in nsICacheEntryDoomCallback aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncEvictStorage(nsICacheEntryDoomCallback *aCallback) = 0;

  /* void asyncVisitStorage (in nsICacheStorageVisitor aVisitor, in boolean aVisitEntries); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncVisitStorage(nsICacheStorageVisitor *aVisitor, bool aVisitEntries) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICacheStorage, NS_ICACHESTORAGE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICACHESTORAGE \
  NS_IMETHOD AsyncOpenURI(nsIURI *aURI, const nsACString& aIdExtension, uint32_t aFlags, nsICacheEntryOpenCallback *aCallback) override; \
  NS_IMETHOD OpenTruncate(nsIURI *aURI, const nsACString& aIdExtension, nsICacheEntry **_retval) override; \
  NS_IMETHOD Exists(nsIURI *aURI, const nsACString& aIdExtension, bool *_retval) override; \
  NS_IMETHOD GetCacheIndexEntryAttrs(nsIURI *aURI, const nsACString& aIdExtension, bool *aHasAltData, uint32_t *aSizeInKB) override; \
  NS_IMETHOD AsyncDoomURI(nsIURI *aURI, const nsACString& aIdExtension, nsICacheEntryDoomCallback *aCallback) override; \
  NS_IMETHOD AsyncEvictStorage(nsICacheEntryDoomCallback *aCallback) override; \
  NS_IMETHOD AsyncVisitStorage(nsICacheStorageVisitor *aVisitor, bool aVisitEntries) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICACHESTORAGE \
  nsresult AsyncOpenURI(nsIURI *aURI, const nsACString& aIdExtension, uint32_t aFlags, nsICacheEntryOpenCallback *aCallback); \
  nsresult OpenTruncate(nsIURI *aURI, const nsACString& aIdExtension, nsICacheEntry **_retval); \
  nsresult Exists(nsIURI *aURI, const nsACString& aIdExtension, bool *_retval); \
  nsresult GetCacheIndexEntryAttrs(nsIURI *aURI, const nsACString& aIdExtension, bool *aHasAltData, uint32_t *aSizeInKB); \
  nsresult AsyncDoomURI(nsIURI *aURI, const nsACString& aIdExtension, nsICacheEntryDoomCallback *aCallback); \
  nsresult AsyncEvictStorage(nsICacheEntryDoomCallback *aCallback); \
  nsresult AsyncVisitStorage(nsICacheStorageVisitor *aVisitor, bool aVisitEntries); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICACHESTORAGE(_to) \
  NS_IMETHOD AsyncOpenURI(nsIURI *aURI, const nsACString& aIdExtension, uint32_t aFlags, nsICacheEntryOpenCallback *aCallback) override { return _to AsyncOpenURI(aURI, aIdExtension, aFlags, aCallback); } \
  NS_IMETHOD OpenTruncate(nsIURI *aURI, const nsACString& aIdExtension, nsICacheEntry **_retval) override { return _to OpenTruncate(aURI, aIdExtension, _retval); } \
  NS_IMETHOD Exists(nsIURI *aURI, const nsACString& aIdExtension, bool *_retval) override { return _to Exists(aURI, aIdExtension, _retval); } \
  NS_IMETHOD GetCacheIndexEntryAttrs(nsIURI *aURI, const nsACString& aIdExtension, bool *aHasAltData, uint32_t *aSizeInKB) override { return _to GetCacheIndexEntryAttrs(aURI, aIdExtension, aHasAltData, aSizeInKB); } \
  NS_IMETHOD AsyncDoomURI(nsIURI *aURI, const nsACString& aIdExtension, nsICacheEntryDoomCallback *aCallback) override { return _to AsyncDoomURI(aURI, aIdExtension, aCallback); } \
  NS_IMETHOD AsyncEvictStorage(nsICacheEntryDoomCallback *aCallback) override { return _to AsyncEvictStorage(aCallback); } \
  NS_IMETHOD AsyncVisitStorage(nsICacheStorageVisitor *aVisitor, bool aVisitEntries) override { return _to AsyncVisitStorage(aVisitor, aVisitEntries); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICACHESTORAGE(_to) \
  NS_IMETHOD AsyncOpenURI(nsIURI *aURI, const nsACString& aIdExtension, uint32_t aFlags, nsICacheEntryOpenCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncOpenURI(aURI, aIdExtension, aFlags, aCallback); } \
  NS_IMETHOD OpenTruncate(nsIURI *aURI, const nsACString& aIdExtension, nsICacheEntry **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OpenTruncate(aURI, aIdExtension, _retval); } \
  NS_IMETHOD Exists(nsIURI *aURI, const nsACString& aIdExtension, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Exists(aURI, aIdExtension, _retval); } \
  NS_IMETHOD GetCacheIndexEntryAttrs(nsIURI *aURI, const nsACString& aIdExtension, bool *aHasAltData, uint32_t *aSizeInKB) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCacheIndexEntryAttrs(aURI, aIdExtension, aHasAltData, aSizeInKB); } \
  NS_IMETHOD AsyncDoomURI(nsIURI *aURI, const nsACString& aIdExtension, nsICacheEntryDoomCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncDoomURI(aURI, aIdExtension, aCallback); } \
  NS_IMETHOD AsyncEvictStorage(nsICacheEntryDoomCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncEvictStorage(aCallback); } \
  NS_IMETHOD AsyncVisitStorage(nsICacheStorageVisitor *aVisitor, bool aVisitEntries) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncVisitStorage(aVisitor, aVisitEntries); } 


#endif /* __gen_nsICacheStorage_h__ */
