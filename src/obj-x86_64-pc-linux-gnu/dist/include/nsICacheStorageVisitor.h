/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache2/nsICacheStorageVisitor.idl
 */

#ifndef __gen_nsICacheStorageVisitor_h__
#define __gen_nsICacheStorageVisitor_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class nsIFile; /* forward declaration */

class nsILoadContextInfo; /* forward declaration */


/* starting interface:    nsICacheStorageVisitor */
#define NS_ICACHESTORAGEVISITOR_IID_STR "6cc7c253-93b6-482b-8e9d-1e04d8e9d655"

#define NS_ICACHESTORAGEVISITOR_IID \
  {0x6cc7c253, 0x93b6, 0x482b, \
    { 0x8e, 0x9d, 0x1e, 0x04, 0xd8, 0xe9, 0xd6, 0x55 }}

class NS_NO_VTABLE nsICacheStorageVisitor : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICACHESTORAGEVISITOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICacheStorageVisitor;

  /* void onCacheStorageInfo (in uint32_t aEntryCount, in uint64_t aConsumption, in uint64_t aCapacity, in nsIFile aDiskDirectory); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnCacheStorageInfo(uint32_t aEntryCount, uint64_t aConsumption, uint64_t aCapacity, nsIFile *aDiskDirectory) = 0;

  /* void onCacheEntryInfo (in nsIURI aURI, in ACString aIdEnhance, in int64_t aDataSize, in long aFetchCount, in uint32_t aLastModifiedTime, in uint32_t aExpirationTime, in boolean aPinned, in nsILoadContextInfo aInfo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnCacheEntryInfo(nsIURI *aURI, const nsACString& aIdEnhance, int64_t aDataSize, int32_t aFetchCount, uint32_t aLastModifiedTime, uint32_t aExpirationTime, bool aPinned, nsILoadContextInfo *aInfo) = 0;

  /* void onCacheEntryVisitCompleted (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnCacheEntryVisitCompleted(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICacheStorageVisitor, NS_ICACHESTORAGEVISITOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICACHESTORAGEVISITOR \
  NS_IMETHOD OnCacheStorageInfo(uint32_t aEntryCount, uint64_t aConsumption, uint64_t aCapacity, nsIFile *aDiskDirectory) override; \
  NS_IMETHOD OnCacheEntryInfo(nsIURI *aURI, const nsACString& aIdEnhance, int64_t aDataSize, int32_t aFetchCount, uint32_t aLastModifiedTime, uint32_t aExpirationTime, bool aPinned, nsILoadContextInfo *aInfo) override; \
  NS_IMETHOD OnCacheEntryVisitCompleted(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICACHESTORAGEVISITOR \
  nsresult OnCacheStorageInfo(uint32_t aEntryCount, uint64_t aConsumption, uint64_t aCapacity, nsIFile *aDiskDirectory); \
  nsresult OnCacheEntryInfo(nsIURI *aURI, const nsACString& aIdEnhance, int64_t aDataSize, int32_t aFetchCount, uint32_t aLastModifiedTime, uint32_t aExpirationTime, bool aPinned, nsILoadContextInfo *aInfo); \
  nsresult OnCacheEntryVisitCompleted(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICACHESTORAGEVISITOR(_to) \
  NS_IMETHOD OnCacheStorageInfo(uint32_t aEntryCount, uint64_t aConsumption, uint64_t aCapacity, nsIFile *aDiskDirectory) override { return _to OnCacheStorageInfo(aEntryCount, aConsumption, aCapacity, aDiskDirectory); } \
  NS_IMETHOD OnCacheEntryInfo(nsIURI *aURI, const nsACString& aIdEnhance, int64_t aDataSize, int32_t aFetchCount, uint32_t aLastModifiedTime, uint32_t aExpirationTime, bool aPinned, nsILoadContextInfo *aInfo) override { return _to OnCacheEntryInfo(aURI, aIdEnhance, aDataSize, aFetchCount, aLastModifiedTime, aExpirationTime, aPinned, aInfo); } \
  NS_IMETHOD OnCacheEntryVisitCompleted(void) override { return _to OnCacheEntryVisitCompleted(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICACHESTORAGEVISITOR(_to) \
  NS_IMETHOD OnCacheStorageInfo(uint32_t aEntryCount, uint64_t aConsumption, uint64_t aCapacity, nsIFile *aDiskDirectory) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnCacheStorageInfo(aEntryCount, aConsumption, aCapacity, aDiskDirectory); } \
  NS_IMETHOD OnCacheEntryInfo(nsIURI *aURI, const nsACString& aIdEnhance, int64_t aDataSize, int32_t aFetchCount, uint32_t aLastModifiedTime, uint32_t aExpirationTime, bool aPinned, nsILoadContextInfo *aInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnCacheEntryInfo(aURI, aIdEnhance, aDataSize, aFetchCount, aLastModifiedTime, aExpirationTime, aPinned, aInfo); } \
  NS_IMETHOD OnCacheEntryVisitCompleted(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnCacheEntryVisitCompleted(); } 


#endif /* __gen_nsICacheStorageVisitor_h__ */
