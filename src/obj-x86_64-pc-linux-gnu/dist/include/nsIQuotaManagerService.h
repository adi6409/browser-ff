/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/quota/nsIQuotaManagerService.idl
 */

#ifndef __gen_nsIQuotaManagerService_h__
#define __gen_nsIQuotaManagerService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIPrincipal; /* forward declaration */

class nsIQuotaRequest; /* forward declaration */

class nsIQuotaCallback; /* forward declaration */

class nsIQuotaUsageCallback; /* forward declaration */

class nsIQuotaUsageRequest; /* forward declaration */


/* starting interface:    nsIQuotaManagerService */
#define NS_IQUOTAMANAGERSERVICE_IID_STR "1b3d0a38-8151-4cf9-89fa-4f92c2ef0e7e"

#define NS_IQUOTAMANAGERSERVICE_IID \
  {0x1b3d0a38, 0x8151, 0x4cf9, \
    { 0x89, 0xfa, 0x4f, 0x92, 0xc2, 0xef, 0x0e, 0x7e }}

class NS_NO_VTABLE nsIQuotaManagerService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IQUOTAMANAGERSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIQuotaManagerService;

  /* [must_use] nsIQuotaRequest storageName (); */
  [[nodiscard]] NS_IMETHOD StorageName(nsIQuotaRequest **_retval) = 0;

  /* [must_use] nsIQuotaRequest storageInitialized (); */
  [[nodiscard]] NS_IMETHOD StorageInitialized(nsIQuotaRequest **_retval) = 0;

  /* [must_use] nsIQuotaRequest temporaryStorageInitialized (); */
  [[nodiscard]] NS_IMETHOD TemporaryStorageInitialized(nsIQuotaRequest **_retval) = 0;

  /* [must_use] nsIQuotaRequest init (); */
  [[nodiscard]] NS_IMETHOD Init(nsIQuotaRequest **_retval) = 0;

  /* [must_use] nsIQuotaRequest initTemporaryStorage (); */
  [[nodiscard]] NS_IMETHOD InitTemporaryStorage(nsIQuotaRequest **_retval) = 0;

  /* [must_use] nsIQuotaRequest initializePersistentOrigin (in nsIPrincipal aPrincipal); */
  [[nodiscard]] NS_IMETHOD InitializePersistentOrigin(nsIPrincipal *aPrincipal, nsIQuotaRequest **_retval) = 0;

  /* [must_use] nsIQuotaRequest initializeTemporaryOrigin (in ACString aPersistenceType, in nsIPrincipal aPrincipal); */
  [[nodiscard]] NS_IMETHOD InitializeTemporaryOrigin(const nsACString& aPersistenceType, nsIPrincipal *aPrincipal, nsIQuotaRequest **_retval) = 0;

  /* [must_use] nsIQuotaUsageRequest getUsage (in nsIQuotaUsageCallback aCallback, [optional] in boolean aGetAll); */
  [[nodiscard]] NS_IMETHOD GetUsage(nsIQuotaUsageCallback *aCallback, bool aGetAll, nsIQuotaUsageRequest **_retval) = 0;

  /* [must_use] nsIQuotaUsageRequest getUsageForPrincipal (in nsIPrincipal aPrincipal, in nsIQuotaUsageCallback aCallback, [optional] in boolean aFromMemory); */
  [[nodiscard]] NS_IMETHOD GetUsageForPrincipal(nsIPrincipal *aPrincipal, nsIQuotaUsageCallback *aCallback, bool aFromMemory, nsIQuotaUsageRequest **_retval) = 0;

  /* [must_use] nsIQuotaRequest listOrigins (); */
  [[nodiscard]] NS_IMETHOD ListOrigins(nsIQuotaRequest **_retval) = 0;

  /* [must_use] nsIQuotaRequest clear (); */
  [[nodiscard]] NS_IMETHOD Clear(nsIQuotaRequest **_retval) = 0;

  /* [must_use] nsIQuotaRequest clearStoragesForOriginAttributesPattern (in AString aPattern); */
  [[nodiscard]] NS_IMETHOD ClearStoragesForOriginAttributesPattern(const nsAString& aPattern, nsIQuotaRequest **_retval) = 0;

  /* [must_use] nsIQuotaRequest clearStoragesForPrincipal (in nsIPrincipal aPrincipal, [optional] in ACString aPersistenceType, [optional] in AString aClientType, [optional] in boolean aClearAll); */
  [[nodiscard]] NS_IMETHOD ClearStoragesForPrincipal(nsIPrincipal *aPrincipal, const nsACString& aPersistenceType, const nsAString& aClientType, bool aClearAll, nsIQuotaRequest **_retval) = 0;

  /* [must_use] nsIQuotaRequest reset (); */
  [[nodiscard]] NS_IMETHOD Reset(nsIQuotaRequest **_retval) = 0;

  /* [must_use] nsIQuotaRequest resetStoragesForPrincipal (in nsIPrincipal aPrincipal, [optional] in ACString aPersistenceType, [optional] in AString aClientType); */
  [[nodiscard]] NS_IMETHOD ResetStoragesForPrincipal(nsIPrincipal *aPrincipal, const nsACString& aPersistenceType, const nsAString& aClientType, nsIQuotaRequest **_retval) = 0;

  /* [must_use] nsIQuotaRequest persisted (in nsIPrincipal aPrincipal); */
  [[nodiscard]] NS_IMETHOD Persisted(nsIPrincipal *aPrincipal, nsIQuotaRequest **_retval) = 0;

  /* [must_use] nsIQuotaRequest persist (in nsIPrincipal aPrincipal); */
  [[nodiscard]] NS_IMETHOD Persist(nsIPrincipal *aPrincipal, nsIQuotaRequest **_retval) = 0;

  /* [must_use] nsIQuotaRequest estimate (in nsIPrincipal aPrincipal); */
  [[nodiscard]] NS_IMETHOD Estimate(nsIPrincipal *aPrincipal, nsIQuotaRequest **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIQuotaManagerService, NS_IQUOTAMANAGERSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIQUOTAMANAGERSERVICE \
  [[nodiscard]] NS_IMETHOD StorageName(nsIQuotaRequest **_retval) override; \
  [[nodiscard]] NS_IMETHOD StorageInitialized(nsIQuotaRequest **_retval) override; \
  [[nodiscard]] NS_IMETHOD TemporaryStorageInitialized(nsIQuotaRequest **_retval) override; \
  [[nodiscard]] NS_IMETHOD Init(nsIQuotaRequest **_retval) override; \
  [[nodiscard]] NS_IMETHOD InitTemporaryStorage(nsIQuotaRequest **_retval) override; \
  [[nodiscard]] NS_IMETHOD InitializePersistentOrigin(nsIPrincipal *aPrincipal, nsIQuotaRequest **_retval) override; \
  [[nodiscard]] NS_IMETHOD InitializeTemporaryOrigin(const nsACString& aPersistenceType, nsIPrincipal *aPrincipal, nsIQuotaRequest **_retval) override; \
  [[nodiscard]] NS_IMETHOD GetUsage(nsIQuotaUsageCallback *aCallback, bool aGetAll, nsIQuotaUsageRequest **_retval) override; \
  [[nodiscard]] NS_IMETHOD GetUsageForPrincipal(nsIPrincipal *aPrincipal, nsIQuotaUsageCallback *aCallback, bool aFromMemory, nsIQuotaUsageRequest **_retval) override; \
  [[nodiscard]] NS_IMETHOD ListOrigins(nsIQuotaRequest **_retval) override; \
  [[nodiscard]] NS_IMETHOD Clear(nsIQuotaRequest **_retval) override; \
  [[nodiscard]] NS_IMETHOD ClearStoragesForOriginAttributesPattern(const nsAString& aPattern, nsIQuotaRequest **_retval) override; \
  [[nodiscard]] NS_IMETHOD ClearStoragesForPrincipal(nsIPrincipal *aPrincipal, const nsACString& aPersistenceType, const nsAString& aClientType, bool aClearAll, nsIQuotaRequest **_retval) override; \
  [[nodiscard]] NS_IMETHOD Reset(nsIQuotaRequest **_retval) override; \
  [[nodiscard]] NS_IMETHOD ResetStoragesForPrincipal(nsIPrincipal *aPrincipal, const nsACString& aPersistenceType, const nsAString& aClientType, nsIQuotaRequest **_retval) override; \
  [[nodiscard]] NS_IMETHOD Persisted(nsIPrincipal *aPrincipal, nsIQuotaRequest **_retval) override; \
  [[nodiscard]] NS_IMETHOD Persist(nsIPrincipal *aPrincipal, nsIQuotaRequest **_retval) override; \
  [[nodiscard]] NS_IMETHOD Estimate(nsIPrincipal *aPrincipal, nsIQuotaRequest **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIQUOTAMANAGERSERVICE \
  [[nodiscard]] nsresult StorageName(nsIQuotaRequest **_retval); \
  [[nodiscard]] nsresult StorageInitialized(nsIQuotaRequest **_retval); \
  [[nodiscard]] nsresult TemporaryStorageInitialized(nsIQuotaRequest **_retval); \
  [[nodiscard]] nsresult Init(nsIQuotaRequest **_retval); \
  [[nodiscard]] nsresult InitTemporaryStorage(nsIQuotaRequest **_retval); \
  [[nodiscard]] nsresult InitializePersistentOrigin(nsIPrincipal *aPrincipal, nsIQuotaRequest **_retval); \
  [[nodiscard]] nsresult InitializeTemporaryOrigin(const nsACString& aPersistenceType, nsIPrincipal *aPrincipal, nsIQuotaRequest **_retval); \
  [[nodiscard]] nsresult GetUsage(nsIQuotaUsageCallback *aCallback, bool aGetAll, nsIQuotaUsageRequest **_retval); \
  [[nodiscard]] nsresult GetUsageForPrincipal(nsIPrincipal *aPrincipal, nsIQuotaUsageCallback *aCallback, bool aFromMemory, nsIQuotaUsageRequest **_retval); \
  [[nodiscard]] nsresult ListOrigins(nsIQuotaRequest **_retval); \
  [[nodiscard]] nsresult Clear(nsIQuotaRequest **_retval); \
  [[nodiscard]] nsresult ClearStoragesForOriginAttributesPattern(const nsAString& aPattern, nsIQuotaRequest **_retval); \
  [[nodiscard]] nsresult ClearStoragesForPrincipal(nsIPrincipal *aPrincipal, const nsACString& aPersistenceType, const nsAString& aClientType, bool aClearAll, nsIQuotaRequest **_retval); \
  [[nodiscard]] nsresult Reset(nsIQuotaRequest **_retval); \
  [[nodiscard]] nsresult ResetStoragesForPrincipal(nsIPrincipal *aPrincipal, const nsACString& aPersistenceType, const nsAString& aClientType, nsIQuotaRequest **_retval); \
  [[nodiscard]] nsresult Persisted(nsIPrincipal *aPrincipal, nsIQuotaRequest **_retval); \
  [[nodiscard]] nsresult Persist(nsIPrincipal *aPrincipal, nsIQuotaRequest **_retval); \
  [[nodiscard]] nsresult Estimate(nsIPrincipal *aPrincipal, nsIQuotaRequest **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIQUOTAMANAGERSERVICE(_to) \
  [[nodiscard]] NS_IMETHOD StorageName(nsIQuotaRequest **_retval) override { return _to StorageName(_retval); } \
  [[nodiscard]] NS_IMETHOD StorageInitialized(nsIQuotaRequest **_retval) override { return _to StorageInitialized(_retval); } \
  [[nodiscard]] NS_IMETHOD TemporaryStorageInitialized(nsIQuotaRequest **_retval) override { return _to TemporaryStorageInitialized(_retval); } \
  [[nodiscard]] NS_IMETHOD Init(nsIQuotaRequest **_retval) override { return _to Init(_retval); } \
  [[nodiscard]] NS_IMETHOD InitTemporaryStorage(nsIQuotaRequest **_retval) override { return _to InitTemporaryStorage(_retval); } \
  [[nodiscard]] NS_IMETHOD InitializePersistentOrigin(nsIPrincipal *aPrincipal, nsIQuotaRequest **_retval) override { return _to InitializePersistentOrigin(aPrincipal, _retval); } \
  [[nodiscard]] NS_IMETHOD InitializeTemporaryOrigin(const nsACString& aPersistenceType, nsIPrincipal *aPrincipal, nsIQuotaRequest **_retval) override { return _to InitializeTemporaryOrigin(aPersistenceType, aPrincipal, _retval); } \
  [[nodiscard]] NS_IMETHOD GetUsage(nsIQuotaUsageCallback *aCallback, bool aGetAll, nsIQuotaUsageRequest **_retval) override { return _to GetUsage(aCallback, aGetAll, _retval); } \
  [[nodiscard]] NS_IMETHOD GetUsageForPrincipal(nsIPrincipal *aPrincipal, nsIQuotaUsageCallback *aCallback, bool aFromMemory, nsIQuotaUsageRequest **_retval) override { return _to GetUsageForPrincipal(aPrincipal, aCallback, aFromMemory, _retval); } \
  [[nodiscard]] NS_IMETHOD ListOrigins(nsIQuotaRequest **_retval) override { return _to ListOrigins(_retval); } \
  [[nodiscard]] NS_IMETHOD Clear(nsIQuotaRequest **_retval) override { return _to Clear(_retval); } \
  [[nodiscard]] NS_IMETHOD ClearStoragesForOriginAttributesPattern(const nsAString& aPattern, nsIQuotaRequest **_retval) override { return _to ClearStoragesForOriginAttributesPattern(aPattern, _retval); } \
  [[nodiscard]] NS_IMETHOD ClearStoragesForPrincipal(nsIPrincipal *aPrincipal, const nsACString& aPersistenceType, const nsAString& aClientType, bool aClearAll, nsIQuotaRequest **_retval) override { return _to ClearStoragesForPrincipal(aPrincipal, aPersistenceType, aClientType, aClearAll, _retval); } \
  [[nodiscard]] NS_IMETHOD Reset(nsIQuotaRequest **_retval) override { return _to Reset(_retval); } \
  [[nodiscard]] NS_IMETHOD ResetStoragesForPrincipal(nsIPrincipal *aPrincipal, const nsACString& aPersistenceType, const nsAString& aClientType, nsIQuotaRequest **_retval) override { return _to ResetStoragesForPrincipal(aPrincipal, aPersistenceType, aClientType, _retval); } \
  [[nodiscard]] NS_IMETHOD Persisted(nsIPrincipal *aPrincipal, nsIQuotaRequest **_retval) override { return _to Persisted(aPrincipal, _retval); } \
  [[nodiscard]] NS_IMETHOD Persist(nsIPrincipal *aPrincipal, nsIQuotaRequest **_retval) override { return _to Persist(aPrincipal, _retval); } \
  [[nodiscard]] NS_IMETHOD Estimate(nsIPrincipal *aPrincipal, nsIQuotaRequest **_retval) override { return _to Estimate(aPrincipal, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIQUOTAMANAGERSERVICE(_to) \
  [[nodiscard]] NS_IMETHOD StorageName(nsIQuotaRequest **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StorageName(_retval); } \
  [[nodiscard]] NS_IMETHOD StorageInitialized(nsIQuotaRequest **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StorageInitialized(_retval); } \
  [[nodiscard]] NS_IMETHOD TemporaryStorageInitialized(nsIQuotaRequest **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TemporaryStorageInitialized(_retval); } \
  [[nodiscard]] NS_IMETHOD Init(nsIQuotaRequest **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(_retval); } \
  [[nodiscard]] NS_IMETHOD InitTemporaryStorage(nsIQuotaRequest **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitTemporaryStorage(_retval); } \
  [[nodiscard]] NS_IMETHOD InitializePersistentOrigin(nsIPrincipal *aPrincipal, nsIQuotaRequest **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitializePersistentOrigin(aPrincipal, _retval); } \
  [[nodiscard]] NS_IMETHOD InitializeTemporaryOrigin(const nsACString& aPersistenceType, nsIPrincipal *aPrincipal, nsIQuotaRequest **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitializeTemporaryOrigin(aPersistenceType, aPrincipal, _retval); } \
  [[nodiscard]] NS_IMETHOD GetUsage(nsIQuotaUsageCallback *aCallback, bool aGetAll, nsIQuotaUsageRequest **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUsage(aCallback, aGetAll, _retval); } \
  [[nodiscard]] NS_IMETHOD GetUsageForPrincipal(nsIPrincipal *aPrincipal, nsIQuotaUsageCallback *aCallback, bool aFromMemory, nsIQuotaUsageRequest **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUsageForPrincipal(aPrincipal, aCallback, aFromMemory, _retval); } \
  [[nodiscard]] NS_IMETHOD ListOrigins(nsIQuotaRequest **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ListOrigins(_retval); } \
  [[nodiscard]] NS_IMETHOD Clear(nsIQuotaRequest **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Clear(_retval); } \
  [[nodiscard]] NS_IMETHOD ClearStoragesForOriginAttributesPattern(const nsAString& aPattern, nsIQuotaRequest **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearStoragesForOriginAttributesPattern(aPattern, _retval); } \
  [[nodiscard]] NS_IMETHOD ClearStoragesForPrincipal(nsIPrincipal *aPrincipal, const nsACString& aPersistenceType, const nsAString& aClientType, bool aClearAll, nsIQuotaRequest **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearStoragesForPrincipal(aPrincipal, aPersistenceType, aClientType, aClearAll, _retval); } \
  [[nodiscard]] NS_IMETHOD Reset(nsIQuotaRequest **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Reset(_retval); } \
  [[nodiscard]] NS_IMETHOD ResetStoragesForPrincipal(nsIPrincipal *aPrincipal, const nsACString& aPersistenceType, const nsAString& aClientType, nsIQuotaRequest **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ResetStoragesForPrincipal(aPrincipal, aPersistenceType, aClientType, _retval); } \
  [[nodiscard]] NS_IMETHOD Persisted(nsIPrincipal *aPrincipal, nsIQuotaRequest **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Persisted(aPrincipal, _retval); } \
  [[nodiscard]] NS_IMETHOD Persist(nsIPrincipal *aPrincipal, nsIQuotaRequest **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Persist(aPrincipal, _retval); } \
  [[nodiscard]] NS_IMETHOD Estimate(nsIPrincipal *aPrincipal, nsIQuotaRequest **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Estimate(aPrincipal, _retval); } 


#endif /* __gen_nsIQuotaManagerService_h__ */
