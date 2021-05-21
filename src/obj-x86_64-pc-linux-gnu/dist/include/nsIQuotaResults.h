/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/quota/nsIQuotaResults.idl
 */

#ifndef __gen_nsIQuotaResults_h__
#define __gen_nsIQuotaResults_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIQuotaUsageResult */
#define NS_IQUOTAUSAGERESULT_IID_STR "d8c9328b-9aa8-4f5d-90e6-482de4a6d5b8"

#define NS_IQUOTAUSAGERESULT_IID \
  {0xd8c9328b, 0x9aa8, 0x4f5d, \
    { 0x90, 0xe6, 0x48, 0x2d, 0xe4, 0xa6, 0xd5, 0xb8 }}

class NS_NO_VTABLE nsIQuotaUsageResult : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IQUOTAUSAGERESULT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIQuotaUsageResult;

  /* readonly attribute ACString origin; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetOrigin(nsACString& aOrigin) = 0;

  /* readonly attribute boolean persisted; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPersisted(bool *aPersisted) = 0;

  /* readonly attribute unsigned long long usage; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUsage(uint64_t *aUsage) = 0;

  /* readonly attribute unsigned long long lastAccessed; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLastAccessed(uint64_t *aLastAccessed) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIQuotaUsageResult, NS_IQUOTAUSAGERESULT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIQUOTAUSAGERESULT \
  NS_IMETHOD GetOrigin(nsACString& aOrigin) override; \
  NS_IMETHOD GetPersisted(bool *aPersisted) override; \
  NS_IMETHOD GetUsage(uint64_t *aUsage) override; \
  NS_IMETHOD GetLastAccessed(uint64_t *aLastAccessed) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIQUOTAUSAGERESULT \
  nsresult GetOrigin(nsACString& aOrigin); \
  nsresult GetPersisted(bool *aPersisted); \
  nsresult GetUsage(uint64_t *aUsage); \
  nsresult GetLastAccessed(uint64_t *aLastAccessed); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIQUOTAUSAGERESULT(_to) \
  NS_IMETHOD GetOrigin(nsACString& aOrigin) override { return _to GetOrigin(aOrigin); } \
  NS_IMETHOD GetPersisted(bool *aPersisted) override { return _to GetPersisted(aPersisted); } \
  NS_IMETHOD GetUsage(uint64_t *aUsage) override { return _to GetUsage(aUsage); } \
  NS_IMETHOD GetLastAccessed(uint64_t *aLastAccessed) override { return _to GetLastAccessed(aLastAccessed); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIQUOTAUSAGERESULT(_to) \
  NS_IMETHOD GetOrigin(nsACString& aOrigin) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOrigin(aOrigin); } \
  NS_IMETHOD GetPersisted(bool *aPersisted) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPersisted(aPersisted); } \
  NS_IMETHOD GetUsage(uint64_t *aUsage) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUsage(aUsage); } \
  NS_IMETHOD GetLastAccessed(uint64_t *aLastAccessed) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastAccessed(aLastAccessed); } 


/* starting interface:    nsIQuotaOriginUsageResult */
#define NS_IQUOTAORIGINUSAGERESULT_IID_STR "96df03d2-116a-493f-bb0b-118c212a6b32"

#define NS_IQUOTAORIGINUSAGERESULT_IID \
  {0x96df03d2, 0x116a, 0x493f, \
    { 0xbb, 0x0b, 0x11, 0x8c, 0x21, 0x2a, 0x6b, 0x32 }}

class NS_NO_VTABLE nsIQuotaOriginUsageResult : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IQUOTAORIGINUSAGERESULT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIQuotaOriginUsageResult;

  /* readonly attribute unsigned long long usage; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUsage(uint64_t *aUsage) = 0;

  /* readonly attribute unsigned long long fileUsage; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFileUsage(uint64_t *aFileUsage) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIQuotaOriginUsageResult, NS_IQUOTAORIGINUSAGERESULT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIQUOTAORIGINUSAGERESULT \
  NS_IMETHOD GetUsage(uint64_t *aUsage) override; \
  NS_IMETHOD GetFileUsage(uint64_t *aFileUsage) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIQUOTAORIGINUSAGERESULT \
  nsresult GetUsage(uint64_t *aUsage); \
  nsresult GetFileUsage(uint64_t *aFileUsage); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIQUOTAORIGINUSAGERESULT(_to) \
  NS_IMETHOD GetUsage(uint64_t *aUsage) override { return _to GetUsage(aUsage); } \
  NS_IMETHOD GetFileUsage(uint64_t *aFileUsage) override { return _to GetFileUsage(aFileUsage); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIQUOTAORIGINUSAGERESULT(_to) \
  NS_IMETHOD GetUsage(uint64_t *aUsage) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUsage(aUsage); } \
  NS_IMETHOD GetFileUsage(uint64_t *aFileUsage) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFileUsage(aFileUsage); } 


/* starting interface:    nsIQuotaEstimateResult */
#define NS_IQUOTAESTIMATERESULT_IID_STR "9827fc69-7ea9-48ef-b30d-2e2ae0451ec0"

#define NS_IQUOTAESTIMATERESULT_IID \
  {0x9827fc69, 0x7ea9, 0x48ef, \
    { 0xb3, 0x0d, 0x2e, 0x2a, 0xe0, 0x45, 0x1e, 0xc0 }}

class NS_NO_VTABLE nsIQuotaEstimateResult : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IQUOTAESTIMATERESULT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIQuotaEstimateResult;

  /* readonly attribute unsigned long long usage; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUsage(uint64_t *aUsage) = 0;

  /* readonly attribute unsigned long long limit; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLimit(uint64_t *aLimit) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIQuotaEstimateResult, NS_IQUOTAESTIMATERESULT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIQUOTAESTIMATERESULT \
  NS_IMETHOD GetUsage(uint64_t *aUsage) override; \
  NS_IMETHOD GetLimit(uint64_t *aLimit) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIQUOTAESTIMATERESULT \
  nsresult GetUsage(uint64_t *aUsage); \
  nsresult GetLimit(uint64_t *aLimit); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIQUOTAESTIMATERESULT(_to) \
  NS_IMETHOD GetUsage(uint64_t *aUsage) override { return _to GetUsage(aUsage); } \
  NS_IMETHOD GetLimit(uint64_t *aLimit) override { return _to GetLimit(aLimit); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIQUOTAESTIMATERESULT(_to) \
  NS_IMETHOD GetUsage(uint64_t *aUsage) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUsage(aUsage); } \
  NS_IMETHOD GetLimit(uint64_t *aLimit) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLimit(aLimit); } 


#endif /* __gen_nsIQuotaResults_h__ */
