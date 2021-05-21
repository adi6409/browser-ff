/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache/nsICacheEntryDescriptor.idl
 */

#ifndef __gen_nsICacheEntryDescriptor_h__
#define __gen_nsICacheEntryDescriptor_h__


#ifndef __gen_nsICacheVisitor_h__
#include "nsICacheVisitor.h"
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

class nsIInputStream; /* forward declaration */

class nsIOutputStream; /* forward declaration */

class nsIFile; /* forward declaration */

class nsICacheMetaDataVisitor; /* forward declaration */


/* starting interface:    nsICacheEntryDescriptor */
#define NS_ICACHEENTRYDESCRIPTOR_IID_STR "90b17d31-46aa-4fb1-a206-473c966cbc18"

#define NS_ICACHEENTRYDESCRIPTOR_IID \
  {0x90b17d31, 0x46aa, 0x4fb1, \
    { 0xa2, 0x06, 0x47, 0x3c, 0x96, 0x6c, 0xbc, 0x18 }}

class NS_NO_VTABLE nsICacheEntryDescriptor : public nsICacheEntryInfo {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICACHEENTRYDESCRIPTOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICacheEntryDescriptor;

  /* void setExpirationTime (in uint32_t expirationTime); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetExpirationTime(uint32_t expirationTime) = 0;

  /* void setDataSize (in unsigned long size); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDataSize(uint32_t size) = 0;

  /* nsIInputStream openInputStream (in unsigned long offset); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OpenInputStream(uint32_t offset, nsIInputStream **_retval) = 0;

  /* nsIOutputStream openOutputStream (in unsigned long offset); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OpenOutputStream(uint32_t offset, nsIOutputStream **_retval) = 0;

  /* attribute nsISupports cacheElement; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCacheElement(nsISupports **aCacheElement) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCacheElement(nsISupports *aCacheElement) = 0;

  /* attribute int64_t predictedDataSize; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPredictedDataSize(int64_t *aPredictedDataSize) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPredictedDataSize(int64_t aPredictedDataSize) = 0;

  /* readonly attribute nsCacheAccessMode accessGranted; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAccessGranted(nsCacheAccessMode *aAccessGranted) = 0;

  /* attribute nsCacheStoragePolicy storagePolicy; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetStoragePolicy(nsCacheStoragePolicy *aStoragePolicy) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetStoragePolicy(nsCacheStoragePolicy aStoragePolicy) = 0;

  /* readonly attribute nsIFile file; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFile(nsIFile **aFile) = 0;

  /* attribute nsISupports securityInfo; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSecurityInfo(nsISupports **aSecurityInfo) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSecurityInfo(nsISupports *aSecurityInfo) = 0;

  /* readonly attribute unsigned long storageDataSize; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetStorageDataSize(uint32_t *aStorageDataSize) = 0;

  /* void doom (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Doom(void) = 0;

  /* void doomAndFailPendingRequests (in nsresult status); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DoomAndFailPendingRequests(nsresult status) = 0;

  /* void asyncDoom (in nsICacheListener listener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncDoom(nsICacheListener *listener) = 0;

  /* void markValid (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MarkValid(void) = 0;

  /* void close (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Close(void) = 0;

  /* string getMetaDataElement (in string key); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMetaDataElement(const char * key, char * *_retval) = 0;

  /* void setMetaDataElement (in string key, in string value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetMetaDataElement(const char * key, const char * value) = 0;

  /* void visitMetaData (in nsICacheMetaDataVisitor visitor); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD VisitMetaData(nsICacheMetaDataVisitor *visitor) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICacheEntryDescriptor, NS_ICACHEENTRYDESCRIPTOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICACHEENTRYDESCRIPTOR \
  NS_IMETHOD SetExpirationTime(uint32_t expirationTime) override; \
  NS_IMETHOD SetDataSize(uint32_t size) override; \
  NS_IMETHOD OpenInputStream(uint32_t offset, nsIInputStream **_retval) override; \
  NS_IMETHOD OpenOutputStream(uint32_t offset, nsIOutputStream **_retval) override; \
  NS_IMETHOD GetCacheElement(nsISupports **aCacheElement) override; \
  NS_IMETHOD SetCacheElement(nsISupports *aCacheElement) override; \
  NS_IMETHOD GetPredictedDataSize(int64_t *aPredictedDataSize) override; \
  NS_IMETHOD SetPredictedDataSize(int64_t aPredictedDataSize) override; \
  NS_IMETHOD GetAccessGranted(nsCacheAccessMode *aAccessGranted) override; \
  NS_IMETHOD GetStoragePolicy(nsCacheStoragePolicy *aStoragePolicy) override; \
  NS_IMETHOD SetStoragePolicy(nsCacheStoragePolicy aStoragePolicy) override; \
  NS_IMETHOD GetFile(nsIFile **aFile) override; \
  NS_IMETHOD GetSecurityInfo(nsISupports **aSecurityInfo) override; \
  NS_IMETHOD SetSecurityInfo(nsISupports *aSecurityInfo) override; \
  NS_IMETHOD GetStorageDataSize(uint32_t *aStorageDataSize) override; \
  NS_IMETHOD Doom(void) override; \
  NS_IMETHOD DoomAndFailPendingRequests(nsresult status) override; \
  NS_IMETHOD AsyncDoom(nsICacheListener *listener) override; \
  NS_IMETHOD MarkValid(void) override; \
  NS_IMETHOD Close(void) override; \
  NS_IMETHOD GetMetaDataElement(const char * key, char * *_retval) override; \
  NS_IMETHOD SetMetaDataElement(const char * key, const char * value) override; \
  NS_IMETHOD VisitMetaData(nsICacheMetaDataVisitor *visitor) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICACHEENTRYDESCRIPTOR \
  nsresult SetExpirationTime(uint32_t expirationTime); \
  nsresult SetDataSize(uint32_t size); \
  nsresult OpenInputStream(uint32_t offset, nsIInputStream **_retval); \
  nsresult OpenOutputStream(uint32_t offset, nsIOutputStream **_retval); \
  nsresult GetCacheElement(nsISupports **aCacheElement); \
  nsresult SetCacheElement(nsISupports *aCacheElement); \
  nsresult GetPredictedDataSize(int64_t *aPredictedDataSize); \
  nsresult SetPredictedDataSize(int64_t aPredictedDataSize); \
  nsresult GetAccessGranted(nsCacheAccessMode *aAccessGranted); \
  nsresult GetStoragePolicy(nsCacheStoragePolicy *aStoragePolicy); \
  nsresult SetStoragePolicy(nsCacheStoragePolicy aStoragePolicy); \
  nsresult GetFile(nsIFile **aFile); \
  nsresult GetSecurityInfo(nsISupports **aSecurityInfo); \
  nsresult SetSecurityInfo(nsISupports *aSecurityInfo); \
  nsresult GetStorageDataSize(uint32_t *aStorageDataSize); \
  nsresult Doom(void); \
  nsresult DoomAndFailPendingRequests(nsresult status); \
  nsresult AsyncDoom(nsICacheListener *listener); \
  nsresult MarkValid(void); \
  nsresult Close(void); \
  nsresult GetMetaDataElement(const char * key, char * *_retval); \
  nsresult SetMetaDataElement(const char * key, const char * value); \
  nsresult VisitMetaData(nsICacheMetaDataVisitor *visitor); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICACHEENTRYDESCRIPTOR(_to) \
  NS_IMETHOD SetExpirationTime(uint32_t expirationTime) override { return _to SetExpirationTime(expirationTime); } \
  NS_IMETHOD SetDataSize(uint32_t size) override { return _to SetDataSize(size); } \
  NS_IMETHOD OpenInputStream(uint32_t offset, nsIInputStream **_retval) override { return _to OpenInputStream(offset, _retval); } \
  NS_IMETHOD OpenOutputStream(uint32_t offset, nsIOutputStream **_retval) override { return _to OpenOutputStream(offset, _retval); } \
  NS_IMETHOD GetCacheElement(nsISupports **aCacheElement) override { return _to GetCacheElement(aCacheElement); } \
  NS_IMETHOD SetCacheElement(nsISupports *aCacheElement) override { return _to SetCacheElement(aCacheElement); } \
  NS_IMETHOD GetPredictedDataSize(int64_t *aPredictedDataSize) override { return _to GetPredictedDataSize(aPredictedDataSize); } \
  NS_IMETHOD SetPredictedDataSize(int64_t aPredictedDataSize) override { return _to SetPredictedDataSize(aPredictedDataSize); } \
  NS_IMETHOD GetAccessGranted(nsCacheAccessMode *aAccessGranted) override { return _to GetAccessGranted(aAccessGranted); } \
  NS_IMETHOD GetStoragePolicy(nsCacheStoragePolicy *aStoragePolicy) override { return _to GetStoragePolicy(aStoragePolicy); } \
  NS_IMETHOD SetStoragePolicy(nsCacheStoragePolicy aStoragePolicy) override { return _to SetStoragePolicy(aStoragePolicy); } \
  NS_IMETHOD GetFile(nsIFile **aFile) override { return _to GetFile(aFile); } \
  NS_IMETHOD GetSecurityInfo(nsISupports **aSecurityInfo) override { return _to GetSecurityInfo(aSecurityInfo); } \
  NS_IMETHOD SetSecurityInfo(nsISupports *aSecurityInfo) override { return _to SetSecurityInfo(aSecurityInfo); } \
  NS_IMETHOD GetStorageDataSize(uint32_t *aStorageDataSize) override { return _to GetStorageDataSize(aStorageDataSize); } \
  NS_IMETHOD Doom(void) override { return _to Doom(); } \
  NS_IMETHOD DoomAndFailPendingRequests(nsresult status) override { return _to DoomAndFailPendingRequests(status); } \
  NS_IMETHOD AsyncDoom(nsICacheListener *listener) override { return _to AsyncDoom(listener); } \
  NS_IMETHOD MarkValid(void) override { return _to MarkValid(); } \
  NS_IMETHOD Close(void) override { return _to Close(); } \
  NS_IMETHOD GetMetaDataElement(const char * key, char * *_retval) override { return _to GetMetaDataElement(key, _retval); } \
  NS_IMETHOD SetMetaDataElement(const char * key, const char * value) override { return _to SetMetaDataElement(key, value); } \
  NS_IMETHOD VisitMetaData(nsICacheMetaDataVisitor *visitor) override { return _to VisitMetaData(visitor); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICACHEENTRYDESCRIPTOR(_to) \
  NS_IMETHOD SetExpirationTime(uint32_t expirationTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetExpirationTime(expirationTime); } \
  NS_IMETHOD SetDataSize(uint32_t size) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDataSize(size); } \
  NS_IMETHOD OpenInputStream(uint32_t offset, nsIInputStream **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OpenInputStream(offset, _retval); } \
  NS_IMETHOD OpenOutputStream(uint32_t offset, nsIOutputStream **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OpenOutputStream(offset, _retval); } \
  NS_IMETHOD GetCacheElement(nsISupports **aCacheElement) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCacheElement(aCacheElement); } \
  NS_IMETHOD SetCacheElement(nsISupports *aCacheElement) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCacheElement(aCacheElement); } \
  NS_IMETHOD GetPredictedDataSize(int64_t *aPredictedDataSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPredictedDataSize(aPredictedDataSize); } \
  NS_IMETHOD SetPredictedDataSize(int64_t aPredictedDataSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPredictedDataSize(aPredictedDataSize); } \
  NS_IMETHOD GetAccessGranted(nsCacheAccessMode *aAccessGranted) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAccessGranted(aAccessGranted); } \
  NS_IMETHOD GetStoragePolicy(nsCacheStoragePolicy *aStoragePolicy) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStoragePolicy(aStoragePolicy); } \
  NS_IMETHOD SetStoragePolicy(nsCacheStoragePolicy aStoragePolicy) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetStoragePolicy(aStoragePolicy); } \
  NS_IMETHOD GetFile(nsIFile **aFile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFile(aFile); } \
  NS_IMETHOD GetSecurityInfo(nsISupports **aSecurityInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSecurityInfo(aSecurityInfo); } \
  NS_IMETHOD SetSecurityInfo(nsISupports *aSecurityInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSecurityInfo(aSecurityInfo); } \
  NS_IMETHOD GetStorageDataSize(uint32_t *aStorageDataSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStorageDataSize(aStorageDataSize); } \
  NS_IMETHOD Doom(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Doom(); } \
  NS_IMETHOD DoomAndFailPendingRequests(nsresult status) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DoomAndFailPendingRequests(status); } \
  NS_IMETHOD AsyncDoom(nsICacheListener *listener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncDoom(listener); } \
  NS_IMETHOD MarkValid(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MarkValid(); } \
  NS_IMETHOD Close(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Close(); } \
  NS_IMETHOD GetMetaDataElement(const char * key, char * *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMetaDataElement(key, _retval); } \
  NS_IMETHOD SetMetaDataElement(const char * key, const char * value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMetaDataElement(key, value); } \
  NS_IMETHOD VisitMetaData(nsICacheMetaDataVisitor *visitor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->VisitMetaData(visitor); } 


/* starting interface:    nsICacheMetaDataVisitor */
#define NS_ICACHEMETADATAVISITOR_IID_STR "22f9a49c-3cf8-4c23-8006-54efb11ac562"

#define NS_ICACHEMETADATAVISITOR_IID \
  {0x22f9a49c, 0x3cf8, 0x4c23, \
    { 0x80, 0x06, 0x54, 0xef, 0xb1, 0x1a, 0xc5, 0x62 }}

class NS_NO_VTABLE nsICacheMetaDataVisitor : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICACHEMETADATAVISITOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICacheMetaDataVisitor;

  /* boolean visitMetaDataElement (in string key, in string value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD VisitMetaDataElement(const char * key, const char * value, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICacheMetaDataVisitor, NS_ICACHEMETADATAVISITOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICACHEMETADATAVISITOR \
  NS_IMETHOD VisitMetaDataElement(const char * key, const char * value, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICACHEMETADATAVISITOR \
  nsresult VisitMetaDataElement(const char * key, const char * value, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICACHEMETADATAVISITOR(_to) \
  NS_IMETHOD VisitMetaDataElement(const char * key, const char * value, bool *_retval) override { return _to VisitMetaDataElement(key, value, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICACHEMETADATAVISITOR(_to) \
  NS_IMETHOD VisitMetaDataElement(const char * key, const char * value, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->VisitMetaDataElement(key, value, _retval); } 


#endif /* __gen_nsICacheEntryDescriptor_h__ */
