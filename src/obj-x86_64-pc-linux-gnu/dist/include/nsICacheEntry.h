/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache2/nsICacheEntry.idl
 */

#ifndef __gen_nsICacheEntry_h__
#define __gen_nsICacheEntry_h__


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
class nsIInputStream; /* forward declaration */

class nsIOutputStream; /* forward declaration */

class nsIAsyncOutputStream; /* forward declaration */

class nsICacheEntryDoomCallback; /* forward declaration */

class nsICacheListener; /* forward declaration */

class nsIFile; /* forward declaration */

class nsICacheEntryMetaDataVisitor; /* forward declaration */

class nsILoadContextInfo; /* forward declaration */


/* starting interface:    nsICacheEntry */
#define NS_ICACHEENTRY_IID_STR "607c2a2c-0a48-40b9-a956-8cf2bb9857cf"

#define NS_ICACHEENTRY_IID \
  {0x607c2a2c, 0x0a48, 0x40b9, \
    { 0xa9, 0x56, 0x8c, 0xf2, 0xbb, 0x98, 0x57, 0xcf }}

class NS_NO_VTABLE nsICacheEntry : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICACHEENTRY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICacheEntry;

  enum {
    CONTENT_TYPE_UNKNOWN = 0U,
    CONTENT_TYPE_OTHER = 1U,
    CONTENT_TYPE_JAVASCRIPT = 2U,
    CONTENT_TYPE_IMAGE = 3U,
    CONTENT_TYPE_MEDIA = 4U,
    CONTENT_TYPE_STYLESHEET = 5U,
    CONTENT_TYPE_WASM = 6U,
    CONTENT_TYPE_LAST = 7U,
    NO_EXPIRATION_TIME = 4294967295U
  };

  /* readonly attribute ACString key; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetKey(nsACString& aKey) = 0;

  /* readonly attribute uint64_t cacheEntryId; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCacheEntryId(uint64_t *aCacheEntryId) = 0;

  /* readonly attribute boolean persistent; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPersistent(bool *aPersistent) = 0;

  /* readonly attribute long fetchCount; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFetchCount(int32_t *aFetchCount) = 0;

  /* readonly attribute uint32_t lastFetched; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLastFetched(uint32_t *aLastFetched) = 0;

  /* readonly attribute uint32_t lastModified; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLastModified(uint32_t *aLastModified) = 0;

  /* readonly attribute uint32_t expirationTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetExpirationTime(uint32_t *aExpirationTime) = 0;

  /* void setExpirationTime (in uint32_t expirationTime); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetExpirationTime(uint32_t expirationTime) = 0;

  /* readonly attribute uint64_t onStartTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetOnStartTime(uint64_t *aOnStartTime) = 0;

  /* readonly attribute uint64_t onStopTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetOnStopTime(uint64_t *aOnStopTime) = 0;

  /* void setNetworkTimes (in uint64_t onStartTime, in uint64_t onStopTime); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetNetworkTimes(uint64_t onStartTime, uint64_t onStopTime) = 0;

  /* void setContentType (in uint8_t contentType); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetContentType(uint8_t contentType) = 0;

  /* void forceValidFor (in unsigned long aSecondsToTheFuture); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ForceValidFor(uint32_t aSecondsToTheFuture) = 0;

  /* readonly attribute boolean isForcedValid; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsForcedValid(bool *aIsForcedValid) = 0;

  /* nsIInputStream openInputStream (in long long offset); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OpenInputStream(int64_t offset, nsIInputStream **_retval) = 0;

  /* nsIOutputStream openOutputStream (in long long offset, in long long predictedSize); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OpenOutputStream(int64_t offset, int64_t predictedSize, nsIOutputStream **_retval) = 0;

  /* attribute nsISupports securityInfo; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSecurityInfo(nsISupports **aSecurityInfo) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSecurityInfo(nsISupports *aSecurityInfo) = 0;

  /* readonly attribute unsigned long storageDataSize; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetStorageDataSize(uint32_t *aStorageDataSize) = 0;

  /* void asyncDoom (in nsICacheEntryDoomCallback listener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncDoom(nsICacheEntryDoomCallback *listener) = 0;

  /* string getMetaDataElement (in string key); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMetaDataElement(const char * key, char * *_retval) = 0;

  /* void setMetaDataElement (in string key, in string value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetMetaDataElement(const char * key, const char * value) = 0;

  /* void visitMetaData (in nsICacheEntryMetaDataVisitor visitor); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD VisitMetaData(nsICacheEntryMetaDataVisitor *visitor) = 0;

  /* void metaDataReady (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MetaDataReady(void) = 0;

  /* void setValid (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetValid(void) = 0;

  /* void dismiss (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Dismiss(void) = 0;

  /* readonly attribute uint32_t diskStorageSizeInKB; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDiskStorageSizeInKB(uint32_t *aDiskStorageSizeInKB) = 0;

  /* nsICacheEntry recreate ([optional] in boolean aMemoryOnly); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Recreate(bool aMemoryOnly, nsICacheEntry **_retval) = 0;

  /* readonly attribute long long dataSize; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDataSize(int64_t *aDataSize) = 0;

  /* readonly attribute long long altDataSize; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAltDataSize(int64_t *aAltDataSize) = 0;

  /* readonly attribute ACString altDataType; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAltDataType(nsACString& aAltDataType) = 0;

  /* nsIAsyncOutputStream openAlternativeOutputStream (in ACString type, in long long predictedSize); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OpenAlternativeOutputStream(const nsACString& type, int64_t predictedSize, nsIAsyncOutputStream **_retval) = 0;

  /* nsIInputStream openAlternativeInputStream (in ACString type); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OpenAlternativeInputStream(const nsACString& type, nsIInputStream **_retval) = 0;

  /* readonly attribute nsILoadContextInfo loadContextInfo; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLoadContextInfo(nsILoadContextInfo **aLoadContextInfo) = 0;

  /* void close (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Close(void) = 0;

  /* void markValid (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MarkValid(void) = 0;

  /* void maybeMarkValid (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MaybeMarkValid(void) = 0;

  /* boolean hasWriteAccess (in boolean aWriteAllowed); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HasWriteAccess(bool aWriteAllowed, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICacheEntry, NS_ICACHEENTRY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICACHEENTRY \
  NS_IMETHOD GetKey(nsACString& aKey) override; \
  NS_IMETHOD GetCacheEntryId(uint64_t *aCacheEntryId) override; \
  NS_IMETHOD GetPersistent(bool *aPersistent) override; \
  NS_IMETHOD GetFetchCount(int32_t *aFetchCount) override; \
  NS_IMETHOD GetLastFetched(uint32_t *aLastFetched) override; \
  NS_IMETHOD GetLastModified(uint32_t *aLastModified) override; \
  NS_IMETHOD GetExpirationTime(uint32_t *aExpirationTime) override; \
  NS_IMETHOD SetExpirationTime(uint32_t expirationTime) override; \
  NS_IMETHOD GetOnStartTime(uint64_t *aOnStartTime) override; \
  NS_IMETHOD GetOnStopTime(uint64_t *aOnStopTime) override; \
  NS_IMETHOD SetNetworkTimes(uint64_t onStartTime, uint64_t onStopTime) override; \
  NS_IMETHOD SetContentType(uint8_t contentType) override; \
  NS_IMETHOD ForceValidFor(uint32_t aSecondsToTheFuture) override; \
  NS_IMETHOD GetIsForcedValid(bool *aIsForcedValid) override; \
  NS_IMETHOD OpenInputStream(int64_t offset, nsIInputStream **_retval) override; \
  NS_IMETHOD OpenOutputStream(int64_t offset, int64_t predictedSize, nsIOutputStream **_retval) override; \
  NS_IMETHOD GetSecurityInfo(nsISupports **aSecurityInfo) override; \
  NS_IMETHOD SetSecurityInfo(nsISupports *aSecurityInfo) override; \
  NS_IMETHOD GetStorageDataSize(uint32_t *aStorageDataSize) override; \
  NS_IMETHOD AsyncDoom(nsICacheEntryDoomCallback *listener) override; \
  NS_IMETHOD GetMetaDataElement(const char * key, char * *_retval) override; \
  NS_IMETHOD SetMetaDataElement(const char * key, const char * value) override; \
  NS_IMETHOD VisitMetaData(nsICacheEntryMetaDataVisitor *visitor) override; \
  NS_IMETHOD MetaDataReady(void) override; \
  NS_IMETHOD SetValid(void) override; \
  NS_IMETHOD Dismiss(void) override; \
  NS_IMETHOD GetDiskStorageSizeInKB(uint32_t *aDiskStorageSizeInKB) override; \
  NS_IMETHOD Recreate(bool aMemoryOnly, nsICacheEntry **_retval) override; \
  NS_IMETHOD GetDataSize(int64_t *aDataSize) override; \
  NS_IMETHOD GetAltDataSize(int64_t *aAltDataSize) override; \
  NS_IMETHOD GetAltDataType(nsACString& aAltDataType) override; \
  NS_IMETHOD OpenAlternativeOutputStream(const nsACString& type, int64_t predictedSize, nsIAsyncOutputStream **_retval) override; \
  NS_IMETHOD OpenAlternativeInputStream(const nsACString& type, nsIInputStream **_retval) override; \
  NS_IMETHOD GetLoadContextInfo(nsILoadContextInfo **aLoadContextInfo) override; \
  NS_IMETHOD Close(void) override; \
  NS_IMETHOD MarkValid(void) override; \
  NS_IMETHOD MaybeMarkValid(void) override; \
  NS_IMETHOD HasWriteAccess(bool aWriteAllowed, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICACHEENTRY \
  nsresult GetKey(nsACString& aKey); \
  nsresult GetCacheEntryId(uint64_t *aCacheEntryId); \
  nsresult GetPersistent(bool *aPersistent); \
  nsresult GetFetchCount(int32_t *aFetchCount); \
  nsresult GetLastFetched(uint32_t *aLastFetched); \
  nsresult GetLastModified(uint32_t *aLastModified); \
  nsresult GetExpirationTime(uint32_t *aExpirationTime); \
  nsresult SetExpirationTime(uint32_t expirationTime); \
  nsresult GetOnStartTime(uint64_t *aOnStartTime); \
  nsresult GetOnStopTime(uint64_t *aOnStopTime); \
  nsresult SetNetworkTimes(uint64_t onStartTime, uint64_t onStopTime); \
  nsresult SetContentType(uint8_t contentType); \
  nsresult ForceValidFor(uint32_t aSecondsToTheFuture); \
  nsresult GetIsForcedValid(bool *aIsForcedValid); \
  nsresult OpenInputStream(int64_t offset, nsIInputStream **_retval); \
  nsresult OpenOutputStream(int64_t offset, int64_t predictedSize, nsIOutputStream **_retval); \
  nsresult GetSecurityInfo(nsISupports **aSecurityInfo); \
  nsresult SetSecurityInfo(nsISupports *aSecurityInfo); \
  nsresult GetStorageDataSize(uint32_t *aStorageDataSize); \
  nsresult AsyncDoom(nsICacheEntryDoomCallback *listener); \
  nsresult GetMetaDataElement(const char * key, char * *_retval); \
  nsresult SetMetaDataElement(const char * key, const char * value); \
  nsresult VisitMetaData(nsICacheEntryMetaDataVisitor *visitor); \
  nsresult MetaDataReady(void); \
  nsresult SetValid(void); \
  nsresult Dismiss(void); \
  nsresult GetDiskStorageSizeInKB(uint32_t *aDiskStorageSizeInKB); \
  nsresult Recreate(bool aMemoryOnly, nsICacheEntry **_retval); \
  nsresult GetDataSize(int64_t *aDataSize); \
  nsresult GetAltDataSize(int64_t *aAltDataSize); \
  nsresult GetAltDataType(nsACString& aAltDataType); \
  nsresult OpenAlternativeOutputStream(const nsACString& type, int64_t predictedSize, nsIAsyncOutputStream **_retval); \
  nsresult OpenAlternativeInputStream(const nsACString& type, nsIInputStream **_retval); \
  nsresult GetLoadContextInfo(nsILoadContextInfo **aLoadContextInfo); \
  nsresult Close(void); \
  nsresult MarkValid(void); \
  nsresult MaybeMarkValid(void); \
  nsresult HasWriteAccess(bool aWriteAllowed, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICACHEENTRY(_to) \
  NS_IMETHOD GetKey(nsACString& aKey) override { return _to GetKey(aKey); } \
  NS_IMETHOD GetCacheEntryId(uint64_t *aCacheEntryId) override { return _to GetCacheEntryId(aCacheEntryId); } \
  NS_IMETHOD GetPersistent(bool *aPersistent) override { return _to GetPersistent(aPersistent); } \
  NS_IMETHOD GetFetchCount(int32_t *aFetchCount) override { return _to GetFetchCount(aFetchCount); } \
  NS_IMETHOD GetLastFetched(uint32_t *aLastFetched) override { return _to GetLastFetched(aLastFetched); } \
  NS_IMETHOD GetLastModified(uint32_t *aLastModified) override { return _to GetLastModified(aLastModified); } \
  NS_IMETHOD GetExpirationTime(uint32_t *aExpirationTime) override { return _to GetExpirationTime(aExpirationTime); } \
  NS_IMETHOD SetExpirationTime(uint32_t expirationTime) override { return _to SetExpirationTime(expirationTime); } \
  NS_IMETHOD GetOnStartTime(uint64_t *aOnStartTime) override { return _to GetOnStartTime(aOnStartTime); } \
  NS_IMETHOD GetOnStopTime(uint64_t *aOnStopTime) override { return _to GetOnStopTime(aOnStopTime); } \
  NS_IMETHOD SetNetworkTimes(uint64_t onStartTime, uint64_t onStopTime) override { return _to SetNetworkTimes(onStartTime, onStopTime); } \
  NS_IMETHOD SetContentType(uint8_t contentType) override { return _to SetContentType(contentType); } \
  NS_IMETHOD ForceValidFor(uint32_t aSecondsToTheFuture) override { return _to ForceValidFor(aSecondsToTheFuture); } \
  NS_IMETHOD GetIsForcedValid(bool *aIsForcedValid) override { return _to GetIsForcedValid(aIsForcedValid); } \
  NS_IMETHOD OpenInputStream(int64_t offset, nsIInputStream **_retval) override { return _to OpenInputStream(offset, _retval); } \
  NS_IMETHOD OpenOutputStream(int64_t offset, int64_t predictedSize, nsIOutputStream **_retval) override { return _to OpenOutputStream(offset, predictedSize, _retval); } \
  NS_IMETHOD GetSecurityInfo(nsISupports **aSecurityInfo) override { return _to GetSecurityInfo(aSecurityInfo); } \
  NS_IMETHOD SetSecurityInfo(nsISupports *aSecurityInfo) override { return _to SetSecurityInfo(aSecurityInfo); } \
  NS_IMETHOD GetStorageDataSize(uint32_t *aStorageDataSize) override { return _to GetStorageDataSize(aStorageDataSize); } \
  NS_IMETHOD AsyncDoom(nsICacheEntryDoomCallback *listener) override { return _to AsyncDoom(listener); } \
  NS_IMETHOD GetMetaDataElement(const char * key, char * *_retval) override { return _to GetMetaDataElement(key, _retval); } \
  NS_IMETHOD SetMetaDataElement(const char * key, const char * value) override { return _to SetMetaDataElement(key, value); } \
  NS_IMETHOD VisitMetaData(nsICacheEntryMetaDataVisitor *visitor) override { return _to VisitMetaData(visitor); } \
  NS_IMETHOD MetaDataReady(void) override { return _to MetaDataReady(); } \
  NS_IMETHOD SetValid(void) override { return _to SetValid(); } \
  NS_IMETHOD Dismiss(void) override { return _to Dismiss(); } \
  NS_IMETHOD GetDiskStorageSizeInKB(uint32_t *aDiskStorageSizeInKB) override { return _to GetDiskStorageSizeInKB(aDiskStorageSizeInKB); } \
  NS_IMETHOD Recreate(bool aMemoryOnly, nsICacheEntry **_retval) override { return _to Recreate(aMemoryOnly, _retval); } \
  NS_IMETHOD GetDataSize(int64_t *aDataSize) override { return _to GetDataSize(aDataSize); } \
  NS_IMETHOD GetAltDataSize(int64_t *aAltDataSize) override { return _to GetAltDataSize(aAltDataSize); } \
  NS_IMETHOD GetAltDataType(nsACString& aAltDataType) override { return _to GetAltDataType(aAltDataType); } \
  NS_IMETHOD OpenAlternativeOutputStream(const nsACString& type, int64_t predictedSize, nsIAsyncOutputStream **_retval) override { return _to OpenAlternativeOutputStream(type, predictedSize, _retval); } \
  NS_IMETHOD OpenAlternativeInputStream(const nsACString& type, nsIInputStream **_retval) override { return _to OpenAlternativeInputStream(type, _retval); } \
  NS_IMETHOD GetLoadContextInfo(nsILoadContextInfo **aLoadContextInfo) override { return _to GetLoadContextInfo(aLoadContextInfo); } \
  NS_IMETHOD Close(void) override { return _to Close(); } \
  NS_IMETHOD MarkValid(void) override { return _to MarkValid(); } \
  NS_IMETHOD MaybeMarkValid(void) override { return _to MaybeMarkValid(); } \
  NS_IMETHOD HasWriteAccess(bool aWriteAllowed, bool *_retval) override { return _to HasWriteAccess(aWriteAllowed, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICACHEENTRY(_to) \
  NS_IMETHOD GetKey(nsACString& aKey) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetKey(aKey); } \
  NS_IMETHOD GetCacheEntryId(uint64_t *aCacheEntryId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCacheEntryId(aCacheEntryId); } \
  NS_IMETHOD GetPersistent(bool *aPersistent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPersistent(aPersistent); } \
  NS_IMETHOD GetFetchCount(int32_t *aFetchCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFetchCount(aFetchCount); } \
  NS_IMETHOD GetLastFetched(uint32_t *aLastFetched) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastFetched(aLastFetched); } \
  NS_IMETHOD GetLastModified(uint32_t *aLastModified) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastModified(aLastModified); } \
  NS_IMETHOD GetExpirationTime(uint32_t *aExpirationTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetExpirationTime(aExpirationTime); } \
  NS_IMETHOD SetExpirationTime(uint32_t expirationTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetExpirationTime(expirationTime); } \
  NS_IMETHOD GetOnStartTime(uint64_t *aOnStartTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOnStartTime(aOnStartTime); } \
  NS_IMETHOD GetOnStopTime(uint64_t *aOnStopTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOnStopTime(aOnStopTime); } \
  NS_IMETHOD SetNetworkTimes(uint64_t onStartTime, uint64_t onStopTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetNetworkTimes(onStartTime, onStopTime); } \
  NS_IMETHOD SetContentType(uint8_t contentType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetContentType(contentType); } \
  NS_IMETHOD ForceValidFor(uint32_t aSecondsToTheFuture) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ForceValidFor(aSecondsToTheFuture); } \
  NS_IMETHOD GetIsForcedValid(bool *aIsForcedValid) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsForcedValid(aIsForcedValid); } \
  NS_IMETHOD OpenInputStream(int64_t offset, nsIInputStream **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OpenInputStream(offset, _retval); } \
  NS_IMETHOD OpenOutputStream(int64_t offset, int64_t predictedSize, nsIOutputStream **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OpenOutputStream(offset, predictedSize, _retval); } \
  NS_IMETHOD GetSecurityInfo(nsISupports **aSecurityInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSecurityInfo(aSecurityInfo); } \
  NS_IMETHOD SetSecurityInfo(nsISupports *aSecurityInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSecurityInfo(aSecurityInfo); } \
  NS_IMETHOD GetStorageDataSize(uint32_t *aStorageDataSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStorageDataSize(aStorageDataSize); } \
  NS_IMETHOD AsyncDoom(nsICacheEntryDoomCallback *listener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncDoom(listener); } \
  NS_IMETHOD GetMetaDataElement(const char * key, char * *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMetaDataElement(key, _retval); } \
  NS_IMETHOD SetMetaDataElement(const char * key, const char * value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMetaDataElement(key, value); } \
  NS_IMETHOD VisitMetaData(nsICacheEntryMetaDataVisitor *visitor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->VisitMetaData(visitor); } \
  NS_IMETHOD MetaDataReady(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MetaDataReady(); } \
  NS_IMETHOD SetValid(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetValid(); } \
  NS_IMETHOD Dismiss(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Dismiss(); } \
  NS_IMETHOD GetDiskStorageSizeInKB(uint32_t *aDiskStorageSizeInKB) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDiskStorageSizeInKB(aDiskStorageSizeInKB); } \
  NS_IMETHOD Recreate(bool aMemoryOnly, nsICacheEntry **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Recreate(aMemoryOnly, _retval); } \
  NS_IMETHOD GetDataSize(int64_t *aDataSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDataSize(aDataSize); } \
  NS_IMETHOD GetAltDataSize(int64_t *aAltDataSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAltDataSize(aAltDataSize); } \
  NS_IMETHOD GetAltDataType(nsACString& aAltDataType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAltDataType(aAltDataType); } \
  NS_IMETHOD OpenAlternativeOutputStream(const nsACString& type, int64_t predictedSize, nsIAsyncOutputStream **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OpenAlternativeOutputStream(type, predictedSize, _retval); } \
  NS_IMETHOD OpenAlternativeInputStream(const nsACString& type, nsIInputStream **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OpenAlternativeInputStream(type, _retval); } \
  NS_IMETHOD GetLoadContextInfo(nsILoadContextInfo **aLoadContextInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoadContextInfo(aLoadContextInfo); } \
  NS_IMETHOD Close(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Close(); } \
  NS_IMETHOD MarkValid(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MarkValid(); } \
  NS_IMETHOD MaybeMarkValid(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MaybeMarkValid(); } \
  NS_IMETHOD HasWriteAccess(bool aWriteAllowed, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasWriteAccess(aWriteAllowed, _retval); } 


/* starting interface:    nsICacheEntryMetaDataVisitor */
#define NS_ICACHEENTRYMETADATAVISITOR_IID_STR "fea3e276-6ba5-4ceb-a581-807d1f43f6d0"

#define NS_ICACHEENTRYMETADATAVISITOR_IID \
  {0xfea3e276, 0x6ba5, 0x4ceb, \
    { 0xa5, 0x81, 0x80, 0x7d, 0x1f, 0x43, 0xf6, 0xd0 }}

class NS_NO_VTABLE nsICacheEntryMetaDataVisitor : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICACHEENTRYMETADATAVISITOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICacheEntryMetaDataVisitor;

  /* void onMetaDataElement (in string key, in string value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnMetaDataElement(const char * key, const char * value) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICacheEntryMetaDataVisitor, NS_ICACHEENTRYMETADATAVISITOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICACHEENTRYMETADATAVISITOR \
  NS_IMETHOD OnMetaDataElement(const char * key, const char * value) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICACHEENTRYMETADATAVISITOR \
  nsresult OnMetaDataElement(const char * key, const char * value); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICACHEENTRYMETADATAVISITOR(_to) \
  NS_IMETHOD OnMetaDataElement(const char * key, const char * value) override { return _to OnMetaDataElement(key, value); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICACHEENTRYMETADATAVISITOR(_to) \
  NS_IMETHOD OnMetaDataElement(const char * key, const char * value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnMetaDataElement(key, value); } 


#endif /* __gen_nsICacheEntry_h__ */
