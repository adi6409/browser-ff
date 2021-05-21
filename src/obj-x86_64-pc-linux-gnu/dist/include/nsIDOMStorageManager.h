/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/storage/nsIDOMStorageManager.idl
 */

#ifndef __gen_nsIDOMStorageManager_h__
#define __gen_nsIDOMStorageManager_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIPrincipal; /* forward declaration */

class mozIDOMWindow; /* forward declaration */

namespace mozilla {
namespace dom {
class Storage; /* webidl Storage */
} // namespace dom
} // namespace mozilla

namespace mozilla {
namespace dom {
class SessionStorageCache;
}  // namespace dom
}  // namespace mozilla

/* starting interface:    nsIDOMStorageManager */
#define NS_IDOMSTORAGEMANAGER_IID_STR "a20c742e-3ed1-44fb-b897-4080a75b1662"

#define NS_IDOMSTORAGEMANAGER_IID \
  {0xa20c742e, 0x3ed1, 0x44fb, \
    { 0xb8, 0x97, 0x40, 0x80, 0xa7, 0x5b, 0x16, 0x62 }}

class NS_NO_VTABLE nsIDOMStorageManager : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOMSTORAGEMANAGER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDOMStorageManager;

  /* Storage precacheStorage (in nsIPrincipal aPrincipal, in nsIPrincipal aStoragePrincipal); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PrecacheStorage(nsIPrincipal *aPrincipal, nsIPrincipal *aStoragePrincipal, mozilla::dom::Storage **_retval) = 0;

  /* Storage createStorage (in mozIDOMWindow aWindow, in nsIPrincipal aPrincipal, in nsIPrincipal aStoragePrincipal, in AString aDocumentURI, [optional] in bool aPrivate); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CreateStorage(mozIDOMWindow *aWindow, nsIPrincipal *aPrincipal, nsIPrincipal *aStoragePrincipal, const nsAString& aDocumentURI, bool aPrivate, mozilla::dom::Storage **_retval) = 0;

  /* Storage getStorage (in mozIDOMWindow aWindow, in nsIPrincipal aPrincipal, in nsIPrincipal aStoragePrincipal, [optional] in bool aPrivate); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetStorage(mozIDOMWindow *aWindow, nsIPrincipal *aPrincipal, nsIPrincipal *aStoragePrincipal, bool aPrivate, mozilla::dom::Storage **_retval) = 0;

  /* void cloneStorage (in Storage aStorageToCloneFrom); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CloneStorage(mozilla::dom::Storage *aStorageToCloneFrom) = 0;

  /* bool checkStorage (in nsIPrincipal aPrincipal, in Storage aStorage); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CheckStorage(nsIPrincipal *aPrincipal, mozilla::dom::Storage *aStorage, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDOMStorageManager, NS_IDOMSTORAGEMANAGER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOMSTORAGEMANAGER \
  NS_IMETHOD PrecacheStorage(nsIPrincipal *aPrincipal, nsIPrincipal *aStoragePrincipal, mozilla::dom::Storage **_retval) override; \
  NS_IMETHOD CreateStorage(mozIDOMWindow *aWindow, nsIPrincipal *aPrincipal, nsIPrincipal *aStoragePrincipal, const nsAString& aDocumentURI, bool aPrivate, mozilla::dom::Storage **_retval) override; \
  NS_IMETHOD GetStorage(mozIDOMWindow *aWindow, nsIPrincipal *aPrincipal, nsIPrincipal *aStoragePrincipal, bool aPrivate, mozilla::dom::Storage **_retval) override; \
  NS_IMETHOD CloneStorage(mozilla::dom::Storage *aStorageToCloneFrom) override; \
  NS_IMETHOD CheckStorage(nsIPrincipal *aPrincipal, mozilla::dom::Storage *aStorage, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOMSTORAGEMANAGER \
  nsresult PrecacheStorage(nsIPrincipal *aPrincipal, nsIPrincipal *aStoragePrincipal, mozilla::dom::Storage **_retval); \
  nsresult CreateStorage(mozIDOMWindow *aWindow, nsIPrincipal *aPrincipal, nsIPrincipal *aStoragePrincipal, const nsAString& aDocumentURI, bool aPrivate, mozilla::dom::Storage **_retval); \
  nsresult GetStorage(mozIDOMWindow *aWindow, nsIPrincipal *aPrincipal, nsIPrincipal *aStoragePrincipal, bool aPrivate, mozilla::dom::Storage **_retval); \
  nsresult CloneStorage(mozilla::dom::Storage *aStorageToCloneFrom); \
  nsresult CheckStorage(nsIPrincipal *aPrincipal, mozilla::dom::Storage *aStorage, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOMSTORAGEMANAGER(_to) \
  NS_IMETHOD PrecacheStorage(nsIPrincipal *aPrincipal, nsIPrincipal *aStoragePrincipal, mozilla::dom::Storage **_retval) override { return _to PrecacheStorage(aPrincipal, aStoragePrincipal, _retval); } \
  NS_IMETHOD CreateStorage(mozIDOMWindow *aWindow, nsIPrincipal *aPrincipal, nsIPrincipal *aStoragePrincipal, const nsAString& aDocumentURI, bool aPrivate, mozilla::dom::Storage **_retval) override { return _to CreateStorage(aWindow, aPrincipal, aStoragePrincipal, aDocumentURI, aPrivate, _retval); } \
  NS_IMETHOD GetStorage(mozIDOMWindow *aWindow, nsIPrincipal *aPrincipal, nsIPrincipal *aStoragePrincipal, bool aPrivate, mozilla::dom::Storage **_retval) override { return _to GetStorage(aWindow, aPrincipal, aStoragePrincipal, aPrivate, _retval); } \
  NS_IMETHOD CloneStorage(mozilla::dom::Storage *aStorageToCloneFrom) override { return _to CloneStorage(aStorageToCloneFrom); } \
  NS_IMETHOD CheckStorage(nsIPrincipal *aPrincipal, mozilla::dom::Storage *aStorage, bool *_retval) override { return _to CheckStorage(aPrincipal, aStorage, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOMSTORAGEMANAGER(_to) \
  NS_IMETHOD PrecacheStorage(nsIPrincipal *aPrincipal, nsIPrincipal *aStoragePrincipal, mozilla::dom::Storage **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PrecacheStorage(aPrincipal, aStoragePrincipal, _retval); } \
  NS_IMETHOD CreateStorage(mozIDOMWindow *aWindow, nsIPrincipal *aPrincipal, nsIPrincipal *aStoragePrincipal, const nsAString& aDocumentURI, bool aPrivate, mozilla::dom::Storage **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateStorage(aWindow, aPrincipal, aStoragePrincipal, aDocumentURI, aPrivate, _retval); } \
  NS_IMETHOD GetStorage(mozIDOMWindow *aWindow, nsIPrincipal *aPrincipal, nsIPrincipal *aStoragePrincipal, bool aPrivate, mozilla::dom::Storage **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStorage(aWindow, aPrincipal, aStoragePrincipal, aPrivate, _retval); } \
  NS_IMETHOD CloneStorage(mozilla::dom::Storage *aStorageToCloneFrom) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CloneStorage(aStorageToCloneFrom); } \
  NS_IMETHOD CheckStorage(nsIPrincipal *aPrincipal, mozilla::dom::Storage *aStorage, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CheckStorage(aPrincipal, aStorage, _retval); } 


/* starting interface:    nsIDOMSessionStorageManager */
#define NS_IDOMSESSIONSTORAGEMANAGER_IID_STR "b3bfbbd0-e738-4cbf-b0f0-b65f25265e82"

#define NS_IDOMSESSIONSTORAGEMANAGER_IID \
  {0xb3bfbbd0, 0xe738, 0x4cbf, \
    { 0xb0, 0xf0, 0xb6, 0x5f, 0x25, 0x26, 0x5e, 0x82 }}

class NS_NO_VTABLE nsIDOMSessionStorageManager : public nsIDOMStorageManager {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOMSESSIONSTORAGEMANAGER_IID)

  /* [noscript] SessionStorageCacheAddRefed getSessionStorageCache (in nsIPrincipal aPrincipal, in nsIPrincipal aStoragePrincipal); */
  NS_IMETHOD GetSessionStorageCache(nsIPrincipal *aPrincipal, nsIPrincipal *aStoragePrincipal, RefPtr<mozilla::dom::SessionStorageCache> * _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDOMSessionStorageManager, NS_IDOMSESSIONSTORAGEMANAGER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOMSESSIONSTORAGEMANAGER \
  NS_IMETHOD GetSessionStorageCache(nsIPrincipal *aPrincipal, nsIPrincipal *aStoragePrincipal, RefPtr<mozilla::dom::SessionStorageCache> * _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOMSESSIONSTORAGEMANAGER \
  nsresult GetSessionStorageCache(nsIPrincipal *aPrincipal, nsIPrincipal *aStoragePrincipal, RefPtr<mozilla::dom::SessionStorageCache> * _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOMSESSIONSTORAGEMANAGER(_to) \
  NS_IMETHOD GetSessionStorageCache(nsIPrincipal *aPrincipal, nsIPrincipal *aStoragePrincipal, RefPtr<mozilla::dom::SessionStorageCache> * _retval) override { return _to GetSessionStorageCache(aPrincipal, aStoragePrincipal, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOMSESSIONSTORAGEMANAGER(_to) \
  NS_IMETHOD GetSessionStorageCache(nsIPrincipal *aPrincipal, nsIPrincipal *aStoragePrincipal, RefPtr<mozilla::dom::SessionStorageCache> * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSessionStorageCache(aPrincipal, aStoragePrincipal, _retval); } 


#endif /* __gen_nsIDOMStorageManager_h__ */
