/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIServiceWorkerManager.idl
 */

#ifndef __gen_nsIServiceWorkerManager_h__
#define __gen_nsIServiceWorkerManager_h__


#ifndef __gen_domstubs_h__
#include "domstubs.h"
#endif

#ifndef __gen_nsIRequest_h__
#include "nsIRequest.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIDOMWindow; /* forward declaration */

class nsPIDOMWindowInner; /* forward declaration */

class mozIDOMWindowProxy; /* forward declaration */

class nsIArray; /* forward declaration */

class nsIInterceptedChannel; /* forward declaration */

class nsIPrincipal; /* forward declaration */

class nsIRunnable; /* forward declaration */

class nsIURI; /* forward declaration */

namespace mozilla {
namespace dom {
class ClientInfo;
class ServiceWorkerDescriptor;
} // namespace dom
} // namespace mozilla

/* starting interface:    nsIServiceWorkerUnregisterCallback */
#define NS_ISERVICEWORKERUNREGISTERCALLBACK_IID_STR "52ee2c9d-ee87-4caf-9588-23ae77ff8798"

#define NS_ISERVICEWORKERUNREGISTERCALLBACK_IID \
  {0x52ee2c9d, 0xee87, 0x4caf, \
    { 0x95, 0x88, 0x23, 0xae, 0x77, 0xff, 0x87, 0x98 }}

class NS_NO_VTABLE nsIServiceWorkerUnregisterCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISERVICEWORKERUNREGISTERCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIServiceWorkerUnregisterCallback;

  /* void unregisterSucceeded (in bool aState); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UnregisterSucceeded(bool aState) = 0;

  /* void unregisterFailed (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UnregisterFailed(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIServiceWorkerUnregisterCallback, NS_ISERVICEWORKERUNREGISTERCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISERVICEWORKERUNREGISTERCALLBACK \
  NS_IMETHOD UnregisterSucceeded(bool aState) override; \
  NS_IMETHOD UnregisterFailed(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISERVICEWORKERUNREGISTERCALLBACK \
  nsresult UnregisterSucceeded(bool aState); \
  nsresult UnregisterFailed(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISERVICEWORKERUNREGISTERCALLBACK(_to) \
  NS_IMETHOD UnregisterSucceeded(bool aState) override { return _to UnregisterSucceeded(aState); } \
  NS_IMETHOD UnregisterFailed(void) override { return _to UnregisterFailed(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISERVICEWORKERUNREGISTERCALLBACK(_to) \
  NS_IMETHOD UnregisterSucceeded(bool aState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UnregisterSucceeded(aState); } \
  NS_IMETHOD UnregisterFailed(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UnregisterFailed(); } 

class nsIWorkerDebugger; /* forward declaration */


/* starting interface:    nsIServiceWorkerInfo */
#define NS_ISERVICEWORKERINFO_IID_STR "76e357ed-208d-4e4c-9165-1c4059707879"

#define NS_ISERVICEWORKERINFO_IID \
  {0x76e357ed, 0x208d, 0x4e4c, \
    { 0x91, 0x65, 0x1c, 0x40, 0x59, 0x70, 0x78, 0x79 }}

class NS_NO_VTABLE nsIServiceWorkerInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISERVICEWORKERINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIServiceWorkerInfo;

  enum {
    STATE_PARSED = 0U,
    STATE_INSTALLING = 1U,
    STATE_INSTALLED = 2U,
    STATE_ACTIVATING = 3U,
    STATE_ACTIVATED = 4U,
    STATE_REDUNDANT = 5U,
    STATE_UNKNOWN = 6U
  };

  /* readonly attribute AString id; */
  NS_IMETHOD GetId(nsAString& aId) = 0;

  /* readonly attribute AString scriptSpec; */
  NS_IMETHOD GetScriptSpec(nsAString& aScriptSpec) = 0;

  /* readonly attribute AString cacheName; */
  NS_IMETHOD GetCacheName(nsAString& aCacheName) = 0;

  /* readonly attribute unsigned short state; */
  NS_IMETHOD GetState(uint16_t *aState) = 0;

  /* readonly attribute nsIWorkerDebugger debugger; */
  NS_IMETHOD GetDebugger(nsIWorkerDebugger **aDebugger) = 0;

  /* readonly attribute bool handlesFetchEvents; */
  NS_IMETHOD GetHandlesFetchEvents(bool *aHandlesFetchEvents) = 0;

  /* readonly attribute PRTime installedTime; */
  NS_IMETHOD GetInstalledTime(PRTime *aInstalledTime) = 0;

  /* readonly attribute PRTime activatedTime; */
  NS_IMETHOD GetActivatedTime(PRTime *aActivatedTime) = 0;

  /* readonly attribute PRTime redundantTime; */
  NS_IMETHOD GetRedundantTime(PRTime *aRedundantTime) = 0;

  /* void attachDebugger (); */
  NS_IMETHOD AttachDebugger(void) = 0;

  /* void detachDebugger (); */
  NS_IMETHOD DetachDebugger(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIServiceWorkerInfo, NS_ISERVICEWORKERINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISERVICEWORKERINFO \
  NS_IMETHOD GetId(nsAString& aId) override; \
  NS_IMETHOD GetScriptSpec(nsAString& aScriptSpec) override; \
  NS_IMETHOD GetCacheName(nsAString& aCacheName) override; \
  NS_IMETHOD GetState(uint16_t *aState) override; \
  NS_IMETHOD GetDebugger(nsIWorkerDebugger **aDebugger) override; \
  NS_IMETHOD GetHandlesFetchEvents(bool *aHandlesFetchEvents) override; \
  NS_IMETHOD GetInstalledTime(PRTime *aInstalledTime) override; \
  NS_IMETHOD GetActivatedTime(PRTime *aActivatedTime) override; \
  NS_IMETHOD GetRedundantTime(PRTime *aRedundantTime) override; \
  NS_IMETHOD AttachDebugger(void) override; \
  NS_IMETHOD DetachDebugger(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISERVICEWORKERINFO \
  nsresult GetId(nsAString& aId); \
  nsresult GetScriptSpec(nsAString& aScriptSpec); \
  nsresult GetCacheName(nsAString& aCacheName); \
  nsresult GetState(uint16_t *aState); \
  nsresult GetDebugger(nsIWorkerDebugger **aDebugger); \
  nsresult GetHandlesFetchEvents(bool *aHandlesFetchEvents); \
  nsresult GetInstalledTime(PRTime *aInstalledTime); \
  nsresult GetActivatedTime(PRTime *aActivatedTime); \
  nsresult GetRedundantTime(PRTime *aRedundantTime); \
  nsresult AttachDebugger(void); \
  nsresult DetachDebugger(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISERVICEWORKERINFO(_to) \
  NS_IMETHOD GetId(nsAString& aId) override { return _to GetId(aId); } \
  NS_IMETHOD GetScriptSpec(nsAString& aScriptSpec) override { return _to GetScriptSpec(aScriptSpec); } \
  NS_IMETHOD GetCacheName(nsAString& aCacheName) override { return _to GetCacheName(aCacheName); } \
  NS_IMETHOD GetState(uint16_t *aState) override { return _to GetState(aState); } \
  NS_IMETHOD GetDebugger(nsIWorkerDebugger **aDebugger) override { return _to GetDebugger(aDebugger); } \
  NS_IMETHOD GetHandlesFetchEvents(bool *aHandlesFetchEvents) override { return _to GetHandlesFetchEvents(aHandlesFetchEvents); } \
  NS_IMETHOD GetInstalledTime(PRTime *aInstalledTime) override { return _to GetInstalledTime(aInstalledTime); } \
  NS_IMETHOD GetActivatedTime(PRTime *aActivatedTime) override { return _to GetActivatedTime(aActivatedTime); } \
  NS_IMETHOD GetRedundantTime(PRTime *aRedundantTime) override { return _to GetRedundantTime(aRedundantTime); } \
  NS_IMETHOD AttachDebugger(void) override { return _to AttachDebugger(); } \
  NS_IMETHOD DetachDebugger(void) override { return _to DetachDebugger(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISERVICEWORKERINFO(_to) \
  NS_IMETHOD GetId(nsAString& aId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetId(aId); } \
  NS_IMETHOD GetScriptSpec(nsAString& aScriptSpec) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScriptSpec(aScriptSpec); } \
  NS_IMETHOD GetCacheName(nsAString& aCacheName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCacheName(aCacheName); } \
  NS_IMETHOD GetState(uint16_t *aState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetState(aState); } \
  NS_IMETHOD GetDebugger(nsIWorkerDebugger **aDebugger) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDebugger(aDebugger); } \
  NS_IMETHOD GetHandlesFetchEvents(bool *aHandlesFetchEvents) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHandlesFetchEvents(aHandlesFetchEvents); } \
  NS_IMETHOD GetInstalledTime(PRTime *aInstalledTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInstalledTime(aInstalledTime); } \
  NS_IMETHOD GetActivatedTime(PRTime *aActivatedTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetActivatedTime(aActivatedTime); } \
  NS_IMETHOD GetRedundantTime(PRTime *aRedundantTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRedundantTime(aRedundantTime); } \
  NS_IMETHOD AttachDebugger(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AttachDebugger(); } \
  NS_IMETHOD DetachDebugger(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DetachDebugger(); } 


/* starting interface:    nsIServiceWorkerRegistrationInfoListener */
#define NS_ISERVICEWORKERREGISTRATIONINFOLISTENER_IID_STR "87e63548-d440-4b8a-b158-65ad1de0211e"

#define NS_ISERVICEWORKERREGISTRATIONINFOLISTENER_IID \
  {0x87e63548, 0xd440, 0x4b8a, \
    { 0xb1, 0x58, 0x65, 0xad, 0x1d, 0xe0, 0x21, 0x1e }}

class NS_NO_VTABLE nsIServiceWorkerRegistrationInfoListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISERVICEWORKERREGISTRATIONINFOLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIServiceWorkerRegistrationInfoListener;

  /* void onChange (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnChange(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIServiceWorkerRegistrationInfoListener, NS_ISERVICEWORKERREGISTRATIONINFOLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISERVICEWORKERREGISTRATIONINFOLISTENER \
  NS_IMETHOD OnChange(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISERVICEWORKERREGISTRATIONINFOLISTENER \
  nsresult OnChange(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISERVICEWORKERREGISTRATIONINFOLISTENER(_to) \
  NS_IMETHOD OnChange(void) override { return _to OnChange(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISERVICEWORKERREGISTRATIONINFOLISTENER(_to) \
  NS_IMETHOD OnChange(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnChange(); } 


/* starting interface:    nsIServiceWorkerRegistrationInfo */
#define NS_ISERVICEWORKERREGISTRATIONINFO_IID_STR "ddbc1fd4-2f2e-4fca-a395-6e010bbedfe3"

#define NS_ISERVICEWORKERREGISTRATIONINFO_IID \
  {0xddbc1fd4, 0x2f2e, 0x4fca, \
    { 0xa3, 0x95, 0x6e, 0x01, 0x0b, 0xbe, 0xdf, 0xe3 }}

class NS_NO_VTABLE nsIServiceWorkerRegistrationInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISERVICEWORKERREGISTRATIONINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIServiceWorkerRegistrationInfo;

  enum {
    UPDATE_VIA_CACHE_IMPORTS = 0U,
    UPDATE_VIA_CACHE_ALL = 1U,
    UPDATE_VIA_CACHE_NONE = 2U
  };

  /* readonly attribute nsIPrincipal principal; */
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) = 0;

  /* readonly attribute AString scope; */
  NS_IMETHOD GetScope(nsAString& aScope) = 0;

  /* readonly attribute AString scriptSpec; */
  NS_IMETHOD GetScriptSpec(nsAString& aScriptSpec) = 0;

  /* readonly attribute unsigned short updateViaCache; */
  NS_IMETHOD GetUpdateViaCache(uint16_t *aUpdateViaCache) = 0;

  /* readonly attribute PRTime lastUpdateTime; */
  NS_IMETHOD GetLastUpdateTime(PRTime *aLastUpdateTime) = 0;

  /* readonly attribute nsIServiceWorkerInfo evaluatingWorker; */
  NS_IMETHOD GetEvaluatingWorker(nsIServiceWorkerInfo **aEvaluatingWorker) = 0;

  /* readonly attribute nsIServiceWorkerInfo installingWorker; */
  NS_IMETHOD GetInstallingWorker(nsIServiceWorkerInfo **aInstallingWorker) = 0;

  /* readonly attribute nsIServiceWorkerInfo waitingWorker; */
  NS_IMETHOD GetWaitingWorker(nsIServiceWorkerInfo **aWaitingWorker) = 0;

  /* readonly attribute nsIServiceWorkerInfo activeWorker; */
  NS_IMETHOD GetActiveWorker(nsIServiceWorkerInfo **aActiveWorker) = 0;

  /* nsIServiceWorkerInfo getWorkerByID (in unsigned long long aID); */
  NS_IMETHOD GetWorkerByID(uint64_t aID, nsIServiceWorkerInfo **_retval) = 0;

  /* void addListener (in nsIServiceWorkerRegistrationInfoListener listener); */
  NS_IMETHOD AddListener(nsIServiceWorkerRegistrationInfoListener *listener) = 0;

  /* void removeListener (in nsIServiceWorkerRegistrationInfoListener listener); */
  NS_IMETHOD RemoveListener(nsIServiceWorkerRegistrationInfoListener *listener) = 0;

  /* void forceShutdown (); */
  NS_IMETHOD ForceShutdown(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIServiceWorkerRegistrationInfo, NS_ISERVICEWORKERREGISTRATIONINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISERVICEWORKERREGISTRATIONINFO \
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) override; \
  NS_IMETHOD GetScope(nsAString& aScope) override; \
  NS_IMETHOD GetScriptSpec(nsAString& aScriptSpec) override; \
  NS_IMETHOD GetUpdateViaCache(uint16_t *aUpdateViaCache) override; \
  NS_IMETHOD GetLastUpdateTime(PRTime *aLastUpdateTime) override; \
  NS_IMETHOD GetEvaluatingWorker(nsIServiceWorkerInfo **aEvaluatingWorker) override; \
  NS_IMETHOD GetInstallingWorker(nsIServiceWorkerInfo **aInstallingWorker) override; \
  NS_IMETHOD GetWaitingWorker(nsIServiceWorkerInfo **aWaitingWorker) override; \
  NS_IMETHOD GetActiveWorker(nsIServiceWorkerInfo **aActiveWorker) override; \
  NS_IMETHOD GetWorkerByID(uint64_t aID, nsIServiceWorkerInfo **_retval) override; \
  NS_IMETHOD AddListener(nsIServiceWorkerRegistrationInfoListener *listener) override; \
  NS_IMETHOD RemoveListener(nsIServiceWorkerRegistrationInfoListener *listener) override; \
  NS_IMETHOD ForceShutdown(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISERVICEWORKERREGISTRATIONINFO \
  nsresult GetPrincipal(nsIPrincipal **aPrincipal); \
  nsresult GetScope(nsAString& aScope); \
  nsresult GetScriptSpec(nsAString& aScriptSpec); \
  nsresult GetUpdateViaCache(uint16_t *aUpdateViaCache); \
  nsresult GetLastUpdateTime(PRTime *aLastUpdateTime); \
  nsresult GetEvaluatingWorker(nsIServiceWorkerInfo **aEvaluatingWorker); \
  nsresult GetInstallingWorker(nsIServiceWorkerInfo **aInstallingWorker); \
  nsresult GetWaitingWorker(nsIServiceWorkerInfo **aWaitingWorker); \
  nsresult GetActiveWorker(nsIServiceWorkerInfo **aActiveWorker); \
  nsresult GetWorkerByID(uint64_t aID, nsIServiceWorkerInfo **_retval); \
  nsresult AddListener(nsIServiceWorkerRegistrationInfoListener *listener); \
  nsresult RemoveListener(nsIServiceWorkerRegistrationInfoListener *listener); \
  nsresult ForceShutdown(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISERVICEWORKERREGISTRATIONINFO(_to) \
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) override { return _to GetPrincipal(aPrincipal); } \
  NS_IMETHOD GetScope(nsAString& aScope) override { return _to GetScope(aScope); } \
  NS_IMETHOD GetScriptSpec(nsAString& aScriptSpec) override { return _to GetScriptSpec(aScriptSpec); } \
  NS_IMETHOD GetUpdateViaCache(uint16_t *aUpdateViaCache) override { return _to GetUpdateViaCache(aUpdateViaCache); } \
  NS_IMETHOD GetLastUpdateTime(PRTime *aLastUpdateTime) override { return _to GetLastUpdateTime(aLastUpdateTime); } \
  NS_IMETHOD GetEvaluatingWorker(nsIServiceWorkerInfo **aEvaluatingWorker) override { return _to GetEvaluatingWorker(aEvaluatingWorker); } \
  NS_IMETHOD GetInstallingWorker(nsIServiceWorkerInfo **aInstallingWorker) override { return _to GetInstallingWorker(aInstallingWorker); } \
  NS_IMETHOD GetWaitingWorker(nsIServiceWorkerInfo **aWaitingWorker) override { return _to GetWaitingWorker(aWaitingWorker); } \
  NS_IMETHOD GetActiveWorker(nsIServiceWorkerInfo **aActiveWorker) override { return _to GetActiveWorker(aActiveWorker); } \
  NS_IMETHOD GetWorkerByID(uint64_t aID, nsIServiceWorkerInfo **_retval) override { return _to GetWorkerByID(aID, _retval); } \
  NS_IMETHOD AddListener(nsIServiceWorkerRegistrationInfoListener *listener) override { return _to AddListener(listener); } \
  NS_IMETHOD RemoveListener(nsIServiceWorkerRegistrationInfoListener *listener) override { return _to RemoveListener(listener); } \
  NS_IMETHOD ForceShutdown(void) override { return _to ForceShutdown(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISERVICEWORKERREGISTRATIONINFO(_to) \
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrincipal(aPrincipal); } \
  NS_IMETHOD GetScope(nsAString& aScope) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScope(aScope); } \
  NS_IMETHOD GetScriptSpec(nsAString& aScriptSpec) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScriptSpec(aScriptSpec); } \
  NS_IMETHOD GetUpdateViaCache(uint16_t *aUpdateViaCache) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUpdateViaCache(aUpdateViaCache); } \
  NS_IMETHOD GetLastUpdateTime(PRTime *aLastUpdateTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastUpdateTime(aLastUpdateTime); } \
  NS_IMETHOD GetEvaluatingWorker(nsIServiceWorkerInfo **aEvaluatingWorker) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEvaluatingWorker(aEvaluatingWorker); } \
  NS_IMETHOD GetInstallingWorker(nsIServiceWorkerInfo **aInstallingWorker) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInstallingWorker(aInstallingWorker); } \
  NS_IMETHOD GetWaitingWorker(nsIServiceWorkerInfo **aWaitingWorker) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWaitingWorker(aWaitingWorker); } \
  NS_IMETHOD GetActiveWorker(nsIServiceWorkerInfo **aActiveWorker) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetActiveWorker(aActiveWorker); } \
  NS_IMETHOD GetWorkerByID(uint64_t aID, nsIServiceWorkerInfo **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWorkerByID(aID, _retval); } \
  NS_IMETHOD AddListener(nsIServiceWorkerRegistrationInfoListener *listener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddListener(listener); } \
  NS_IMETHOD RemoveListener(nsIServiceWorkerRegistrationInfoListener *listener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveListener(listener); } \
  NS_IMETHOD ForceShutdown(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ForceShutdown(); } 


/* starting interface:    nsIServiceWorkerManagerListener */
#define NS_ISERVICEWORKERMANAGERLISTENER_IID_STR "9e523e7c-ad6f-4df0-8077-c74aebbc679d"

#define NS_ISERVICEWORKERMANAGERLISTENER_IID \
  {0x9e523e7c, 0xad6f, 0x4df0, \
    { 0x80, 0x77, 0xc7, 0x4a, 0xeb, 0xbc, 0x67, 0x9d }}

class NS_NO_VTABLE nsIServiceWorkerManagerListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISERVICEWORKERMANAGERLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIServiceWorkerManagerListener;

  /* void onRegister (in nsIServiceWorkerRegistrationInfo aInfo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnRegister(nsIServiceWorkerRegistrationInfo *aInfo) = 0;

  /* void onUnregister (in nsIServiceWorkerRegistrationInfo aInfo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnUnregister(nsIServiceWorkerRegistrationInfo *aInfo) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIServiceWorkerManagerListener, NS_ISERVICEWORKERMANAGERLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISERVICEWORKERMANAGERLISTENER \
  NS_IMETHOD OnRegister(nsIServiceWorkerRegistrationInfo *aInfo) override; \
  NS_IMETHOD OnUnregister(nsIServiceWorkerRegistrationInfo *aInfo) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISERVICEWORKERMANAGERLISTENER \
  nsresult OnRegister(nsIServiceWorkerRegistrationInfo *aInfo); \
  nsresult OnUnregister(nsIServiceWorkerRegistrationInfo *aInfo); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISERVICEWORKERMANAGERLISTENER(_to) \
  NS_IMETHOD OnRegister(nsIServiceWorkerRegistrationInfo *aInfo) override { return _to OnRegister(aInfo); } \
  NS_IMETHOD OnUnregister(nsIServiceWorkerRegistrationInfo *aInfo) override { return _to OnUnregister(aInfo); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISERVICEWORKERMANAGERLISTENER(_to) \
  NS_IMETHOD OnRegister(nsIServiceWorkerRegistrationInfo *aInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnRegister(aInfo); } \
  NS_IMETHOD OnUnregister(nsIServiceWorkerRegistrationInfo *aInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnUnregister(aInfo); } 


/* starting interface:    nsIServiceWorkerManager */
#define NS_ISERVICEWORKERMANAGER_IID_STR "7404c8e8-4d47-4449-8ed1-47d1261d4e33"

#define NS_ISERVICEWORKERMANAGER_IID \
  {0x7404c8e8, 0x4d47, 0x4449, \
    { 0x8e, 0xd1, 0x47, 0xd1, 0x26, 0x1d, 0x4e, 0x33 }}

class NS_NO_VTABLE nsIServiceWorkerManager : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISERVICEWORKERMANAGER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIServiceWorkerManager;

  /* [implicit_jscontext] Promise registerForTest (in nsIPrincipal aPrincipal, in AString aScope, in AString aScriptURL); */
  NS_IMETHOD RegisterForTest(nsIPrincipal *aPrincipal, const nsAString& aScope, const nsAString& aScriptURL, JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

  /* [implicit_jscontext] Promise registerForAddonPrincipal (in nsIPrincipal aPrincipal); */
  NS_IMETHOD RegisterForAddonPrincipal(nsIPrincipal *aPrincipal, JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

  /* void unregister (in nsIPrincipal aPrincipal, in nsIServiceWorkerUnregisterCallback aCallback, in AString aScope); */
  NS_IMETHOD Unregister(nsIPrincipal *aPrincipal, nsIServiceWorkerUnregisterCallback *aCallback, const nsAString& aScope) = 0;

  /* nsIServiceWorkerRegistrationInfo getRegistrationByPrincipal (in nsIPrincipal aPrincipal, in AString aScope); */
  NS_IMETHOD GetRegistrationByPrincipal(nsIPrincipal *aPrincipal, const nsAString& aScope, nsIServiceWorkerRegistrationInfo **_retval) = 0;

  /* [nostdcall,notxpcom] bool StartControlling (in const_ClientInfoRef aClientInfo, in const_ServiceWorkerDescriptorRef aServiceWorker); */
  virtual bool StartControlling(const mozilla::dom::ClientInfo & aClientInfo, const mozilla::dom::ServiceWorkerDescriptor & aServiceWorker) = 0;

  /* AString getScopeForUrl (in nsIPrincipal aPrincipal, in AString aPath); */
  NS_IMETHOD GetScopeForUrl(nsIPrincipal *aPrincipal, const nsAString& aPath, nsAString& _retval) = 0;

  /* nsIArray getAllRegistrations (); */
  NS_IMETHOD GetAllRegistrations(nsIArray **_retval) = 0;

  /* void removeRegistrationsByOriginAttributes (in AString aOriginAttributes); */
  NS_IMETHOD RemoveRegistrationsByOriginAttributes(const nsAString& aOriginAttributes) = 0;

  /* [implicit_jscontext] void propagateSoftUpdate (in jsval aOriginAttributes, in AString aScope); */
  NS_IMETHOD PropagateSoftUpdate(JS::HandleValue aOriginAttributes, const nsAString& aScope, JSContext* cx) = 0;

  /* void propagateUnregister (in nsIPrincipal aPrincipal, in nsIServiceWorkerUnregisterCallback aCallback, in AString aScope); */
  NS_IMETHOD PropagateUnregister(nsIPrincipal *aPrincipal, nsIServiceWorkerUnregisterCallback *aCallback, const nsAString& aScope) = 0;

  /* void sendNotificationClickEvent (in ACString aOriginSuffix, in ACString scope, in AString aID, in AString aTitle, in AString aDir, in AString aLang, in AString aBody, in AString aTag, in AString aIcon, in AString aData, in AString aBehavior); */
  NS_IMETHOD SendNotificationClickEvent(const nsACString& aOriginSuffix, const nsACString& scope, const nsAString& aID, const nsAString& aTitle, const nsAString& aDir, const nsAString& aLang, const nsAString& aBody, const nsAString& aTag, const nsAString& aIcon, const nsAString& aData, const nsAString& aBehavior) = 0;

  /* void sendNotificationCloseEvent (in ACString aOriginSuffix, in ACString scope, in AString aID, in AString aTitle, in AString aDir, in AString aLang, in AString aBody, in AString aTag, in AString aIcon, in AString aData, in AString aBehavior); */
  NS_IMETHOD SendNotificationCloseEvent(const nsACString& aOriginSuffix, const nsACString& scope, const nsAString& aID, const nsAString& aTitle, const nsAString& aDir, const nsAString& aLang, const nsAString& aBody, const nsAString& aTag, const nsAString& aIcon, const nsAString& aData, const nsAString& aBehavior) = 0;

  /* [optional_argc] void sendPushEvent (in ACString aOriginAttributes, in ACString aScope, [optional] in Array<uint8_t> aDataBytes); */
  NS_IMETHOD SendPushEvent(const nsACString& aOriginAttributes, const nsACString& aScope, const nsTArray<uint8_t >& aDataBytes, uint8_t _argc) = 0;

  /* void sendPushSubscriptionChangeEvent (in ACString aOriginAttributes, in ACString scope); */
  NS_IMETHOD SendPushSubscriptionChangeEvent(const nsACString& aOriginAttributes, const nsACString& scope) = 0;

  /* void addListener (in nsIServiceWorkerManagerListener aListener); */
  NS_IMETHOD AddListener(nsIServiceWorkerManagerListener *aListener) = 0;

  /* void removeListener (in nsIServiceWorkerManagerListener aListener); */
  NS_IMETHOD RemoveListener(nsIServiceWorkerManagerListener *aListener) = 0;

  /* bool isParentInterceptEnabled (); */
  NS_IMETHOD IsParentInterceptEnabled(bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIServiceWorkerManager, NS_ISERVICEWORKERMANAGER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISERVICEWORKERMANAGER \
  NS_IMETHOD RegisterForTest(nsIPrincipal *aPrincipal, const nsAString& aScope, const nsAString& aScriptURL, JSContext* cx, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD RegisterForAddonPrincipal(nsIPrincipal *aPrincipal, JSContext* cx, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD Unregister(nsIPrincipal *aPrincipal, nsIServiceWorkerUnregisterCallback *aCallback, const nsAString& aScope) override; \
  NS_IMETHOD GetRegistrationByPrincipal(nsIPrincipal *aPrincipal, const nsAString& aScope, nsIServiceWorkerRegistrationInfo **_retval) override; \
  virtual bool StartControlling(const mozilla::dom::ClientInfo & aClientInfo, const mozilla::dom::ServiceWorkerDescriptor & aServiceWorker) override; \
  NS_IMETHOD GetScopeForUrl(nsIPrincipal *aPrincipal, const nsAString& aPath, nsAString& _retval) override; \
  NS_IMETHOD GetAllRegistrations(nsIArray **_retval) override; \
  NS_IMETHOD RemoveRegistrationsByOriginAttributes(const nsAString& aOriginAttributes) override; \
  NS_IMETHOD PropagateSoftUpdate(JS::HandleValue aOriginAttributes, const nsAString& aScope, JSContext* cx) override; \
  NS_IMETHOD PropagateUnregister(nsIPrincipal *aPrincipal, nsIServiceWorkerUnregisterCallback *aCallback, const nsAString& aScope) override; \
  NS_IMETHOD SendNotificationClickEvent(const nsACString& aOriginSuffix, const nsACString& scope, const nsAString& aID, const nsAString& aTitle, const nsAString& aDir, const nsAString& aLang, const nsAString& aBody, const nsAString& aTag, const nsAString& aIcon, const nsAString& aData, const nsAString& aBehavior) override; \
  NS_IMETHOD SendNotificationCloseEvent(const nsACString& aOriginSuffix, const nsACString& scope, const nsAString& aID, const nsAString& aTitle, const nsAString& aDir, const nsAString& aLang, const nsAString& aBody, const nsAString& aTag, const nsAString& aIcon, const nsAString& aData, const nsAString& aBehavior) override; \
  NS_IMETHOD SendPushEvent(const nsACString& aOriginAttributes, const nsACString& aScope, const nsTArray<uint8_t >& aDataBytes, uint8_t _argc) override; \
  NS_IMETHOD SendPushSubscriptionChangeEvent(const nsACString& aOriginAttributes, const nsACString& scope) override; \
  NS_IMETHOD AddListener(nsIServiceWorkerManagerListener *aListener) override; \
  NS_IMETHOD RemoveListener(nsIServiceWorkerManagerListener *aListener) override; \
  NS_IMETHOD IsParentInterceptEnabled(bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISERVICEWORKERMANAGER \
  nsresult RegisterForTest(nsIPrincipal *aPrincipal, const nsAString& aScope, const nsAString& aScriptURL, JSContext* cx, ::mozilla::dom::Promise * * _retval); \
  nsresult RegisterForAddonPrincipal(nsIPrincipal *aPrincipal, JSContext* cx, ::mozilla::dom::Promise * * _retval); \
  nsresult Unregister(nsIPrincipal *aPrincipal, nsIServiceWorkerUnregisterCallback *aCallback, const nsAString& aScope); \
  nsresult GetRegistrationByPrincipal(nsIPrincipal *aPrincipal, const nsAString& aScope, nsIServiceWorkerRegistrationInfo **_retval); \
  bool StartControlling(const mozilla::dom::ClientInfo & aClientInfo, const mozilla::dom::ServiceWorkerDescriptor & aServiceWorker); \
  nsresult GetScopeForUrl(nsIPrincipal *aPrincipal, const nsAString& aPath, nsAString& _retval); \
  nsresult GetAllRegistrations(nsIArray **_retval); \
  nsresult RemoveRegistrationsByOriginAttributes(const nsAString& aOriginAttributes); \
  nsresult PropagateSoftUpdate(JS::HandleValue aOriginAttributes, const nsAString& aScope, JSContext* cx); \
  nsresult PropagateUnregister(nsIPrincipal *aPrincipal, nsIServiceWorkerUnregisterCallback *aCallback, const nsAString& aScope); \
  nsresult SendNotificationClickEvent(const nsACString& aOriginSuffix, const nsACString& scope, const nsAString& aID, const nsAString& aTitle, const nsAString& aDir, const nsAString& aLang, const nsAString& aBody, const nsAString& aTag, const nsAString& aIcon, const nsAString& aData, const nsAString& aBehavior); \
  nsresult SendNotificationCloseEvent(const nsACString& aOriginSuffix, const nsACString& scope, const nsAString& aID, const nsAString& aTitle, const nsAString& aDir, const nsAString& aLang, const nsAString& aBody, const nsAString& aTag, const nsAString& aIcon, const nsAString& aData, const nsAString& aBehavior); \
  nsresult SendPushEvent(const nsACString& aOriginAttributes, const nsACString& aScope, const nsTArray<uint8_t >& aDataBytes, uint8_t _argc); \
  nsresult SendPushSubscriptionChangeEvent(const nsACString& aOriginAttributes, const nsACString& scope); \
  nsresult AddListener(nsIServiceWorkerManagerListener *aListener); \
  nsresult RemoveListener(nsIServiceWorkerManagerListener *aListener); \
  nsresult IsParentInterceptEnabled(bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISERVICEWORKERMANAGER(_to) \
  NS_IMETHOD RegisterForTest(nsIPrincipal *aPrincipal, const nsAString& aScope, const nsAString& aScriptURL, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to RegisterForTest(aPrincipal, aScope, aScriptURL, cx, _retval); } \
  NS_IMETHOD RegisterForAddonPrincipal(nsIPrincipal *aPrincipal, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to RegisterForAddonPrincipal(aPrincipal, cx, _retval); } \
  NS_IMETHOD Unregister(nsIPrincipal *aPrincipal, nsIServiceWorkerUnregisterCallback *aCallback, const nsAString& aScope) override { return _to Unregister(aPrincipal, aCallback, aScope); } \
  NS_IMETHOD GetRegistrationByPrincipal(nsIPrincipal *aPrincipal, const nsAString& aScope, nsIServiceWorkerRegistrationInfo **_retval) override { return _to GetRegistrationByPrincipal(aPrincipal, aScope, _retval); } \
  virtual bool StartControlling(const mozilla::dom::ClientInfo & aClientInfo, const mozilla::dom::ServiceWorkerDescriptor & aServiceWorker) override { return _to StartControlling(aClientInfo, aServiceWorker); } \
  NS_IMETHOD GetScopeForUrl(nsIPrincipal *aPrincipal, const nsAString& aPath, nsAString& _retval) override { return _to GetScopeForUrl(aPrincipal, aPath, _retval); } \
  NS_IMETHOD GetAllRegistrations(nsIArray **_retval) override { return _to GetAllRegistrations(_retval); } \
  NS_IMETHOD RemoveRegistrationsByOriginAttributes(const nsAString& aOriginAttributes) override { return _to RemoveRegistrationsByOriginAttributes(aOriginAttributes); } \
  NS_IMETHOD PropagateSoftUpdate(JS::HandleValue aOriginAttributes, const nsAString& aScope, JSContext* cx) override { return _to PropagateSoftUpdate(aOriginAttributes, aScope, cx); } \
  NS_IMETHOD PropagateUnregister(nsIPrincipal *aPrincipal, nsIServiceWorkerUnregisterCallback *aCallback, const nsAString& aScope) override { return _to PropagateUnregister(aPrincipal, aCallback, aScope); } \
  NS_IMETHOD SendNotificationClickEvent(const nsACString& aOriginSuffix, const nsACString& scope, const nsAString& aID, const nsAString& aTitle, const nsAString& aDir, const nsAString& aLang, const nsAString& aBody, const nsAString& aTag, const nsAString& aIcon, const nsAString& aData, const nsAString& aBehavior) override { return _to SendNotificationClickEvent(aOriginSuffix, scope, aID, aTitle, aDir, aLang, aBody, aTag, aIcon, aData, aBehavior); } \
  NS_IMETHOD SendNotificationCloseEvent(const nsACString& aOriginSuffix, const nsACString& scope, const nsAString& aID, const nsAString& aTitle, const nsAString& aDir, const nsAString& aLang, const nsAString& aBody, const nsAString& aTag, const nsAString& aIcon, const nsAString& aData, const nsAString& aBehavior) override { return _to SendNotificationCloseEvent(aOriginSuffix, scope, aID, aTitle, aDir, aLang, aBody, aTag, aIcon, aData, aBehavior); } \
  NS_IMETHOD SendPushEvent(const nsACString& aOriginAttributes, const nsACString& aScope, const nsTArray<uint8_t >& aDataBytes, uint8_t _argc) override { return _to SendPushEvent(aOriginAttributes, aScope, aDataBytes, _argc); } \
  NS_IMETHOD SendPushSubscriptionChangeEvent(const nsACString& aOriginAttributes, const nsACString& scope) override { return _to SendPushSubscriptionChangeEvent(aOriginAttributes, scope); } \
  NS_IMETHOD AddListener(nsIServiceWorkerManagerListener *aListener) override { return _to AddListener(aListener); } \
  NS_IMETHOD RemoveListener(nsIServiceWorkerManagerListener *aListener) override { return _to RemoveListener(aListener); } \
  NS_IMETHOD IsParentInterceptEnabled(bool *_retval) override { return _to IsParentInterceptEnabled(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISERVICEWORKERMANAGER(_to) \
  NS_IMETHOD RegisterForTest(nsIPrincipal *aPrincipal, const nsAString& aScope, const nsAString& aScriptURL, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterForTest(aPrincipal, aScope, aScriptURL, cx, _retval); } \
  NS_IMETHOD RegisterForAddonPrincipal(nsIPrincipal *aPrincipal, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterForAddonPrincipal(aPrincipal, cx, _retval); } \
  NS_IMETHOD Unregister(nsIPrincipal *aPrincipal, nsIServiceWorkerUnregisterCallback *aCallback, const nsAString& aScope) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Unregister(aPrincipal, aCallback, aScope); } \
  NS_IMETHOD GetRegistrationByPrincipal(nsIPrincipal *aPrincipal, const nsAString& aScope, nsIServiceWorkerRegistrationInfo **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRegistrationByPrincipal(aPrincipal, aScope, _retval); } \
  virtual bool StartControlling(const mozilla::dom::ClientInfo & aClientInfo, const mozilla::dom::ServiceWorkerDescriptor & aServiceWorker) override; \
  NS_IMETHOD GetScopeForUrl(nsIPrincipal *aPrincipal, const nsAString& aPath, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScopeForUrl(aPrincipal, aPath, _retval); } \
  NS_IMETHOD GetAllRegistrations(nsIArray **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllRegistrations(_retval); } \
  NS_IMETHOD RemoveRegistrationsByOriginAttributes(const nsAString& aOriginAttributes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveRegistrationsByOriginAttributes(aOriginAttributes); } \
  NS_IMETHOD PropagateSoftUpdate(JS::HandleValue aOriginAttributes, const nsAString& aScope, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PropagateSoftUpdate(aOriginAttributes, aScope, cx); } \
  NS_IMETHOD PropagateUnregister(nsIPrincipal *aPrincipal, nsIServiceWorkerUnregisterCallback *aCallback, const nsAString& aScope) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PropagateUnregister(aPrincipal, aCallback, aScope); } \
  NS_IMETHOD SendNotificationClickEvent(const nsACString& aOriginSuffix, const nsACString& scope, const nsAString& aID, const nsAString& aTitle, const nsAString& aDir, const nsAString& aLang, const nsAString& aBody, const nsAString& aTag, const nsAString& aIcon, const nsAString& aData, const nsAString& aBehavior) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendNotificationClickEvent(aOriginSuffix, scope, aID, aTitle, aDir, aLang, aBody, aTag, aIcon, aData, aBehavior); } \
  NS_IMETHOD SendNotificationCloseEvent(const nsACString& aOriginSuffix, const nsACString& scope, const nsAString& aID, const nsAString& aTitle, const nsAString& aDir, const nsAString& aLang, const nsAString& aBody, const nsAString& aTag, const nsAString& aIcon, const nsAString& aData, const nsAString& aBehavior) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendNotificationCloseEvent(aOriginSuffix, scope, aID, aTitle, aDir, aLang, aBody, aTag, aIcon, aData, aBehavior); } \
  NS_IMETHOD SendPushEvent(const nsACString& aOriginAttributes, const nsACString& aScope, const nsTArray<uint8_t >& aDataBytes, uint8_t _argc) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendPushEvent(aOriginAttributes, aScope, aDataBytes, _argc); } \
  NS_IMETHOD SendPushSubscriptionChangeEvent(const nsACString& aOriginAttributes, const nsACString& scope) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendPushSubscriptionChangeEvent(aOriginAttributes, scope); } \
  NS_IMETHOD AddListener(nsIServiceWorkerManagerListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddListener(aListener); } \
  NS_IMETHOD RemoveListener(nsIServiceWorkerManagerListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveListener(aListener); } \
  NS_IMETHOD IsParentInterceptEnabled(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsParentInterceptEnabled(_retval); } 

#define SERVICEWORKERMANAGER_CONTRACTID "@mozilla.org/serviceworkers/manager;1"

#endif /* __gen_nsIServiceWorkerManager_h__ */
