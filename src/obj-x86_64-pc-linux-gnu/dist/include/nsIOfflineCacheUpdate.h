/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/prefetch/nsIOfflineCacheUpdate.idl
 */

#ifndef __gen_nsIOfflineCacheUpdate_h__
#define __gen_nsIOfflineCacheUpdate_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIDOMWindow; /* forward declaration */

class nsIURI; /* forward declaration */

class nsIOfflineCacheUpdate; /* forward declaration */

class nsIPrincipal; /* forward declaration */

class nsIPrefBranch; /* forward declaration */

class nsIApplicationCache; /* forward declaration */

class nsIFile; /* forward declaration */

class nsIObserver; /* forward declaration */

class nsICookieJarSettings; /* forward declaration */

namespace mozilla {
namespace dom {
class Document; /* webidl Document */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIOfflineCacheUpdateObserver */
#define NS_IOFFLINECACHEUPDATEOBSERVER_IID_STR "47360d57-8ef4-4a5d-8865-1a27a739ad1a"

#define NS_IOFFLINECACHEUPDATEOBSERVER_IID \
  {0x47360d57, 0x8ef4, 0x4a5d, \
    { 0x88, 0x65, 0x1a, 0x27, 0xa7, 0x39, 0xad, 0x1a }}

class NS_NO_VTABLE nsIOfflineCacheUpdateObserver : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IOFFLINECACHEUPDATEOBSERVER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIOfflineCacheUpdateObserver;

  enum {
    STATE_ERROR = 1U,
    STATE_CHECKING = 2U,
    STATE_NOUPDATE = 3U,
    STATE_OBSOLETE = 4U,
    STATE_DOWNLOADING = 5U,
    STATE_ITEMSTARTED = 6U,
    STATE_ITEMCOMPLETED = 7U,
    STATE_ITEMPROGRESS = 8U,
    STATE_FINISHED = 10U
  };

  /* void updateStateChanged (in nsIOfflineCacheUpdate aUpdate, in uint32_t state); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UpdateStateChanged(nsIOfflineCacheUpdate *aUpdate, uint32_t state) = 0;

  /* void applicationCacheAvailable (in nsIApplicationCache applicationCache); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ApplicationCacheAvailable(nsIApplicationCache *applicationCache) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIOfflineCacheUpdateObserver, NS_IOFFLINECACHEUPDATEOBSERVER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIOFFLINECACHEUPDATEOBSERVER \
  NS_IMETHOD UpdateStateChanged(nsIOfflineCacheUpdate *aUpdate, uint32_t state) override; \
  NS_IMETHOD ApplicationCacheAvailable(nsIApplicationCache *applicationCache) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIOFFLINECACHEUPDATEOBSERVER \
  nsresult UpdateStateChanged(nsIOfflineCacheUpdate *aUpdate, uint32_t state); \
  nsresult ApplicationCacheAvailable(nsIApplicationCache *applicationCache); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIOFFLINECACHEUPDATEOBSERVER(_to) \
  NS_IMETHOD UpdateStateChanged(nsIOfflineCacheUpdate *aUpdate, uint32_t state) override { return _to UpdateStateChanged(aUpdate, state); } \
  NS_IMETHOD ApplicationCacheAvailable(nsIApplicationCache *applicationCache) override { return _to ApplicationCacheAvailable(applicationCache); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIOFFLINECACHEUPDATEOBSERVER(_to) \
  NS_IMETHOD UpdateStateChanged(nsIOfflineCacheUpdate *aUpdate, uint32_t state) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UpdateStateChanged(aUpdate, state); } \
  NS_IMETHOD ApplicationCacheAvailable(nsIApplicationCache *applicationCache) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ApplicationCacheAvailable(applicationCache); } 


/* starting interface:    nsIOfflineCacheUpdate */
#define NS_IOFFLINECACHEUPDATE_IID_STR "6e3e26ea-45b2-4db7-9e4a-93b965679298"

#define NS_IOFFLINECACHEUPDATE_IID \
  {0x6e3e26ea, 0x45b2, 0x4db7, \
    { 0x9e, 0x4a, 0x93, 0xb9, 0x65, 0x67, 0x92, 0x98 }}

class NS_NO_VTABLE nsIOfflineCacheUpdate : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IOFFLINECACHEUPDATE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIOfflineCacheUpdate;

  /* readonly attribute unsigned short status; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetStatus(uint16_t *aStatus) = 0;

  /* readonly attribute boolean partial; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPartial(bool *aPartial) = 0;

  /* readonly attribute boolean isUpgrade; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsUpgrade(bool *aIsUpgrade) = 0;

  /* readonly attribute ACString updateDomain; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUpdateDomain(nsACString& aUpdateDomain) = 0;

  /* readonly attribute nsIURI manifestURI; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetManifestURI(nsIURI **aManifestURI) = 0;

  /* readonly attribute nsIPrincipal loadingPrincipal; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLoadingPrincipal(nsIPrincipal **aLoadingPrincipal) = 0;

  /* readonly attribute boolean succeeded; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSucceeded(bool *aSucceeded) = 0;

  /* void init (in nsIURI aManifestURI, in nsIURI aDocumentURI, in nsIPrincipal aLoadingPrincipal, in Document aDocument, [optional] in nsIFile aCustomProfileDir); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(nsIURI *aManifestURI, nsIURI *aDocumentURI, nsIPrincipal *aLoadingPrincipal, mozilla::dom::Document *aDocument, nsIFile *aCustomProfileDir) = 0;

  /* void initPartial (in nsIURI aManifestURI, in ACString aClientID, in nsIURI aDocumentURI, in nsIPrincipal aPrincipal, in nsICookieJarSettings aCookieJarSettings); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD InitPartial(nsIURI *aManifestURI, const nsACString& aClientID, nsIURI *aDocumentURI, nsIPrincipal *aPrincipal, nsICookieJarSettings *aCookieJarSettings) = 0;

  /* void initForUpdateCheck (in nsIURI aManifestURI, in nsIPrincipal aLoadingPrincipal, in nsIObserver aObserver); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD InitForUpdateCheck(nsIURI *aManifestURI, nsIPrincipal *aLoadingPrincipal, nsIObserver *aObserver) = 0;

  /* void addDynamicURI (in nsIURI aURI); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddDynamicURI(nsIURI *aURI) = 0;

  /* void schedule (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Schedule(void) = 0;

  /* void addObserver (in nsIOfflineCacheUpdateObserver aObserver, [optional] in boolean aHoldWeak); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddObserver(nsIOfflineCacheUpdateObserver *aObserver, bool aHoldWeak) = 0;

  /* void removeObserver (in nsIOfflineCacheUpdateObserver aObserver); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveObserver(nsIOfflineCacheUpdateObserver *aObserver) = 0;

  /* void cancel (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Cancel(void) = 0;

  /* readonly attribute uint64_t byteProgress; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetByteProgress(uint64_t *aByteProgress) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIOfflineCacheUpdate, NS_IOFFLINECACHEUPDATE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIOFFLINECACHEUPDATE \
  NS_IMETHOD GetStatus(uint16_t *aStatus) override; \
  NS_IMETHOD GetPartial(bool *aPartial) override; \
  NS_IMETHOD GetIsUpgrade(bool *aIsUpgrade) override; \
  NS_IMETHOD GetUpdateDomain(nsACString& aUpdateDomain) override; \
  NS_IMETHOD GetManifestURI(nsIURI **aManifestURI) override; \
  NS_IMETHOD GetLoadingPrincipal(nsIPrincipal **aLoadingPrincipal) override; \
  NS_IMETHOD GetSucceeded(bool *aSucceeded) override; \
  NS_IMETHOD Init(nsIURI *aManifestURI, nsIURI *aDocumentURI, nsIPrincipal *aLoadingPrincipal, mozilla::dom::Document *aDocument, nsIFile *aCustomProfileDir) override; \
  NS_IMETHOD InitPartial(nsIURI *aManifestURI, const nsACString& aClientID, nsIURI *aDocumentURI, nsIPrincipal *aPrincipal, nsICookieJarSettings *aCookieJarSettings) override; \
  NS_IMETHOD InitForUpdateCheck(nsIURI *aManifestURI, nsIPrincipal *aLoadingPrincipal, nsIObserver *aObserver) override; \
  NS_IMETHOD AddDynamicURI(nsIURI *aURI) override; \
  NS_IMETHOD Schedule(void) override; \
  NS_IMETHOD AddObserver(nsIOfflineCacheUpdateObserver *aObserver, bool aHoldWeak) override; \
  NS_IMETHOD RemoveObserver(nsIOfflineCacheUpdateObserver *aObserver) override; \
  NS_IMETHOD Cancel(void) override; \
  NS_IMETHOD GetByteProgress(uint64_t *aByteProgress) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIOFFLINECACHEUPDATE \
  nsresult GetStatus(uint16_t *aStatus); \
  nsresult GetPartial(bool *aPartial); \
  nsresult GetIsUpgrade(bool *aIsUpgrade); \
  nsresult GetUpdateDomain(nsACString& aUpdateDomain); \
  nsresult GetManifestURI(nsIURI **aManifestURI); \
  nsresult GetLoadingPrincipal(nsIPrincipal **aLoadingPrincipal); \
  nsresult GetSucceeded(bool *aSucceeded); \
  nsresult Init(nsIURI *aManifestURI, nsIURI *aDocumentURI, nsIPrincipal *aLoadingPrincipal, mozilla::dom::Document *aDocument, nsIFile *aCustomProfileDir); \
  nsresult InitPartial(nsIURI *aManifestURI, const nsACString& aClientID, nsIURI *aDocumentURI, nsIPrincipal *aPrincipal, nsICookieJarSettings *aCookieJarSettings); \
  nsresult InitForUpdateCheck(nsIURI *aManifestURI, nsIPrincipal *aLoadingPrincipal, nsIObserver *aObserver); \
  nsresult AddDynamicURI(nsIURI *aURI); \
  nsresult Schedule(void); \
  nsresult AddObserver(nsIOfflineCacheUpdateObserver *aObserver, bool aHoldWeak); \
  nsresult RemoveObserver(nsIOfflineCacheUpdateObserver *aObserver); \
  nsresult Cancel(void); \
  nsresult GetByteProgress(uint64_t *aByteProgress); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIOFFLINECACHEUPDATE(_to) \
  NS_IMETHOD GetStatus(uint16_t *aStatus) override { return _to GetStatus(aStatus); } \
  NS_IMETHOD GetPartial(bool *aPartial) override { return _to GetPartial(aPartial); } \
  NS_IMETHOD GetIsUpgrade(bool *aIsUpgrade) override { return _to GetIsUpgrade(aIsUpgrade); } \
  NS_IMETHOD GetUpdateDomain(nsACString& aUpdateDomain) override { return _to GetUpdateDomain(aUpdateDomain); } \
  NS_IMETHOD GetManifestURI(nsIURI **aManifestURI) override { return _to GetManifestURI(aManifestURI); } \
  NS_IMETHOD GetLoadingPrincipal(nsIPrincipal **aLoadingPrincipal) override { return _to GetLoadingPrincipal(aLoadingPrincipal); } \
  NS_IMETHOD GetSucceeded(bool *aSucceeded) override { return _to GetSucceeded(aSucceeded); } \
  NS_IMETHOD Init(nsIURI *aManifestURI, nsIURI *aDocumentURI, nsIPrincipal *aLoadingPrincipal, mozilla::dom::Document *aDocument, nsIFile *aCustomProfileDir) override { return _to Init(aManifestURI, aDocumentURI, aLoadingPrincipal, aDocument, aCustomProfileDir); } \
  NS_IMETHOD InitPartial(nsIURI *aManifestURI, const nsACString& aClientID, nsIURI *aDocumentURI, nsIPrincipal *aPrincipal, nsICookieJarSettings *aCookieJarSettings) override { return _to InitPartial(aManifestURI, aClientID, aDocumentURI, aPrincipal, aCookieJarSettings); } \
  NS_IMETHOD InitForUpdateCheck(nsIURI *aManifestURI, nsIPrincipal *aLoadingPrincipal, nsIObserver *aObserver) override { return _to InitForUpdateCheck(aManifestURI, aLoadingPrincipal, aObserver); } \
  NS_IMETHOD AddDynamicURI(nsIURI *aURI) override { return _to AddDynamicURI(aURI); } \
  NS_IMETHOD Schedule(void) override { return _to Schedule(); } \
  NS_IMETHOD AddObserver(nsIOfflineCacheUpdateObserver *aObserver, bool aHoldWeak) override { return _to AddObserver(aObserver, aHoldWeak); } \
  NS_IMETHOD RemoveObserver(nsIOfflineCacheUpdateObserver *aObserver) override { return _to RemoveObserver(aObserver); } \
  NS_IMETHOD Cancel(void) override { return _to Cancel(); } \
  NS_IMETHOD GetByteProgress(uint64_t *aByteProgress) override { return _to GetByteProgress(aByteProgress); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIOFFLINECACHEUPDATE(_to) \
  NS_IMETHOD GetStatus(uint16_t *aStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStatus(aStatus); } \
  NS_IMETHOD GetPartial(bool *aPartial) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPartial(aPartial); } \
  NS_IMETHOD GetIsUpgrade(bool *aIsUpgrade) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsUpgrade(aIsUpgrade); } \
  NS_IMETHOD GetUpdateDomain(nsACString& aUpdateDomain) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUpdateDomain(aUpdateDomain); } \
  NS_IMETHOD GetManifestURI(nsIURI **aManifestURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetManifestURI(aManifestURI); } \
  NS_IMETHOD GetLoadingPrincipal(nsIPrincipal **aLoadingPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoadingPrincipal(aLoadingPrincipal); } \
  NS_IMETHOD GetSucceeded(bool *aSucceeded) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSucceeded(aSucceeded); } \
  NS_IMETHOD Init(nsIURI *aManifestURI, nsIURI *aDocumentURI, nsIPrincipal *aLoadingPrincipal, mozilla::dom::Document *aDocument, nsIFile *aCustomProfileDir) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aManifestURI, aDocumentURI, aLoadingPrincipal, aDocument, aCustomProfileDir); } \
  NS_IMETHOD InitPartial(nsIURI *aManifestURI, const nsACString& aClientID, nsIURI *aDocumentURI, nsIPrincipal *aPrincipal, nsICookieJarSettings *aCookieJarSettings) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitPartial(aManifestURI, aClientID, aDocumentURI, aPrincipal, aCookieJarSettings); } \
  NS_IMETHOD InitForUpdateCheck(nsIURI *aManifestURI, nsIPrincipal *aLoadingPrincipal, nsIObserver *aObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitForUpdateCheck(aManifestURI, aLoadingPrincipal, aObserver); } \
  NS_IMETHOD AddDynamicURI(nsIURI *aURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddDynamicURI(aURI); } \
  NS_IMETHOD Schedule(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Schedule(); } \
  NS_IMETHOD AddObserver(nsIOfflineCacheUpdateObserver *aObserver, bool aHoldWeak) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddObserver(aObserver, aHoldWeak); } \
  NS_IMETHOD RemoveObserver(nsIOfflineCacheUpdateObserver *aObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveObserver(aObserver); } \
  NS_IMETHOD Cancel(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Cancel(); } \
  NS_IMETHOD GetByteProgress(uint64_t *aByteProgress) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetByteProgress(aByteProgress); } 


/* starting interface:    nsIOfflineCacheUpdateService */
#define NS_IOFFLINECACHEUPDATESERVICE_IID_STR "44971e74-37e4-4140-8677-a4cf213a3f4b"

#define NS_IOFFLINECACHEUPDATESERVICE_IID \
  {0x44971e74, 0x37e4, 0x4140, \
    { 0x86, 0x77, 0xa4, 0xcf, 0x21, 0x3a, 0x3f, 0x4b }}

class NS_NO_VTABLE nsIOfflineCacheUpdateService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IOFFLINECACHEUPDATESERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIOfflineCacheUpdateService;

  enum {
    ALLOW_NO_WARN = 3U
  };

  /* readonly attribute unsigned long numUpdates; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNumUpdates(uint32_t *aNumUpdates) = 0;

  /* nsIOfflineCacheUpdate getUpdate (in unsigned long index); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUpdate(uint32_t index, nsIOfflineCacheUpdate **_retval) = 0;

  /* nsIOfflineCacheUpdate scheduleUpdate (in nsIURI aManifestURI, in nsIURI aDocumentURI, in nsIPrincipal aLoadingPrincipal, in mozIDOMWindow aWindow); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ScheduleUpdate(nsIURI *aManifestURI, nsIURI *aDocumentURI, nsIPrincipal *aLoadingPrincipal, mozIDOMWindow *aWindow, nsIOfflineCacheUpdate **_retval) = 0;

  /* nsIOfflineCacheUpdate scheduleAppUpdate (in nsIURI aManifestURI, in nsIURI aDocumentURI, in nsIPrincipal aLoadingPrincipal, in nsIFile aProfileDir); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ScheduleAppUpdate(nsIURI *aManifestURI, nsIURI *aDocumentURI, nsIPrincipal *aLoadingPrincipal, nsIFile *aProfileDir, nsIOfflineCacheUpdate **_retval) = 0;

  /* void scheduleOnDocumentStop (in nsIURI aManifestURI, in nsIURI aDocumentURI, in nsIPrincipal aLoadingPrincipal, in Document aDocument); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ScheduleOnDocumentStop(nsIURI *aManifestURI, nsIURI *aDocumentURI, nsIPrincipal *aLoadingPrincipal, mozilla::dom::Document *aDocument) = 0;

  /* void checkForUpdate (in nsIURI aManifestURI, in nsIPrincipal aLoadingPrincipal, in nsIObserver aObserver); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CheckForUpdate(nsIURI *aManifestURI, nsIPrincipal *aLoadingPrincipal, nsIObserver *aObserver) = 0;

  /* boolean offlineAppAllowed (in nsIPrincipal aPrincipal); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OfflineAppAllowed(nsIPrincipal *aPrincipal, bool *_retval) = 0;

  /* boolean offlineAppAllowedForURI (in nsIURI aURI); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OfflineAppAllowedForURI(nsIURI *aURI, bool *_retval) = 0;

  /* void allowOfflineApp (in nsIPrincipal aPrincipal); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AllowOfflineApp(nsIPrincipal *aPrincipal) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIOfflineCacheUpdateService, NS_IOFFLINECACHEUPDATESERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIOFFLINECACHEUPDATESERVICE \
  NS_IMETHOD GetNumUpdates(uint32_t *aNumUpdates) override; \
  NS_IMETHOD GetUpdate(uint32_t index, nsIOfflineCacheUpdate **_retval) override; \
  NS_IMETHOD ScheduleUpdate(nsIURI *aManifestURI, nsIURI *aDocumentURI, nsIPrincipal *aLoadingPrincipal, mozIDOMWindow *aWindow, nsIOfflineCacheUpdate **_retval) override; \
  NS_IMETHOD ScheduleAppUpdate(nsIURI *aManifestURI, nsIURI *aDocumentURI, nsIPrincipal *aLoadingPrincipal, nsIFile *aProfileDir, nsIOfflineCacheUpdate **_retval) override; \
  NS_IMETHOD ScheduleOnDocumentStop(nsIURI *aManifestURI, nsIURI *aDocumentURI, nsIPrincipal *aLoadingPrincipal, mozilla::dom::Document *aDocument) override; \
  NS_IMETHOD CheckForUpdate(nsIURI *aManifestURI, nsIPrincipal *aLoadingPrincipal, nsIObserver *aObserver) override; \
  NS_IMETHOD OfflineAppAllowed(nsIPrincipal *aPrincipal, bool *_retval) override; \
  NS_IMETHOD OfflineAppAllowedForURI(nsIURI *aURI, bool *_retval) override; \
  NS_IMETHOD AllowOfflineApp(nsIPrincipal *aPrincipal) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIOFFLINECACHEUPDATESERVICE \
  nsresult GetNumUpdates(uint32_t *aNumUpdates); \
  nsresult GetUpdate(uint32_t index, nsIOfflineCacheUpdate **_retval); \
  nsresult ScheduleUpdate(nsIURI *aManifestURI, nsIURI *aDocumentURI, nsIPrincipal *aLoadingPrincipal, mozIDOMWindow *aWindow, nsIOfflineCacheUpdate **_retval); \
  nsresult ScheduleAppUpdate(nsIURI *aManifestURI, nsIURI *aDocumentURI, nsIPrincipal *aLoadingPrincipal, nsIFile *aProfileDir, nsIOfflineCacheUpdate **_retval); \
  nsresult ScheduleOnDocumentStop(nsIURI *aManifestURI, nsIURI *aDocumentURI, nsIPrincipal *aLoadingPrincipal, mozilla::dom::Document *aDocument); \
  nsresult CheckForUpdate(nsIURI *aManifestURI, nsIPrincipal *aLoadingPrincipal, nsIObserver *aObserver); \
  nsresult OfflineAppAllowed(nsIPrincipal *aPrincipal, bool *_retval); \
  nsresult OfflineAppAllowedForURI(nsIURI *aURI, bool *_retval); \
  nsresult AllowOfflineApp(nsIPrincipal *aPrincipal); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIOFFLINECACHEUPDATESERVICE(_to) \
  NS_IMETHOD GetNumUpdates(uint32_t *aNumUpdates) override { return _to GetNumUpdates(aNumUpdates); } \
  NS_IMETHOD GetUpdate(uint32_t index, nsIOfflineCacheUpdate **_retval) override { return _to GetUpdate(index, _retval); } \
  NS_IMETHOD ScheduleUpdate(nsIURI *aManifestURI, nsIURI *aDocumentURI, nsIPrincipal *aLoadingPrincipal, mozIDOMWindow *aWindow, nsIOfflineCacheUpdate **_retval) override { return _to ScheduleUpdate(aManifestURI, aDocumentURI, aLoadingPrincipal, aWindow, _retval); } \
  NS_IMETHOD ScheduleAppUpdate(nsIURI *aManifestURI, nsIURI *aDocumentURI, nsIPrincipal *aLoadingPrincipal, nsIFile *aProfileDir, nsIOfflineCacheUpdate **_retval) override { return _to ScheduleAppUpdate(aManifestURI, aDocumentURI, aLoadingPrincipal, aProfileDir, _retval); } \
  NS_IMETHOD ScheduleOnDocumentStop(nsIURI *aManifestURI, nsIURI *aDocumentURI, nsIPrincipal *aLoadingPrincipal, mozilla::dom::Document *aDocument) override { return _to ScheduleOnDocumentStop(aManifestURI, aDocumentURI, aLoadingPrincipal, aDocument); } \
  NS_IMETHOD CheckForUpdate(nsIURI *aManifestURI, nsIPrincipal *aLoadingPrincipal, nsIObserver *aObserver) override { return _to CheckForUpdate(aManifestURI, aLoadingPrincipal, aObserver); } \
  NS_IMETHOD OfflineAppAllowed(nsIPrincipal *aPrincipal, bool *_retval) override { return _to OfflineAppAllowed(aPrincipal, _retval); } \
  NS_IMETHOD OfflineAppAllowedForURI(nsIURI *aURI, bool *_retval) override { return _to OfflineAppAllowedForURI(aURI, _retval); } \
  NS_IMETHOD AllowOfflineApp(nsIPrincipal *aPrincipal) override { return _to AllowOfflineApp(aPrincipal); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIOFFLINECACHEUPDATESERVICE(_to) \
  NS_IMETHOD GetNumUpdates(uint32_t *aNumUpdates) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNumUpdates(aNumUpdates); } \
  NS_IMETHOD GetUpdate(uint32_t index, nsIOfflineCacheUpdate **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUpdate(index, _retval); } \
  NS_IMETHOD ScheduleUpdate(nsIURI *aManifestURI, nsIURI *aDocumentURI, nsIPrincipal *aLoadingPrincipal, mozIDOMWindow *aWindow, nsIOfflineCacheUpdate **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ScheduleUpdate(aManifestURI, aDocumentURI, aLoadingPrincipal, aWindow, _retval); } \
  NS_IMETHOD ScheduleAppUpdate(nsIURI *aManifestURI, nsIURI *aDocumentURI, nsIPrincipal *aLoadingPrincipal, nsIFile *aProfileDir, nsIOfflineCacheUpdate **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ScheduleAppUpdate(aManifestURI, aDocumentURI, aLoadingPrincipal, aProfileDir, _retval); } \
  NS_IMETHOD ScheduleOnDocumentStop(nsIURI *aManifestURI, nsIURI *aDocumentURI, nsIPrincipal *aLoadingPrincipal, mozilla::dom::Document *aDocument) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ScheduleOnDocumentStop(aManifestURI, aDocumentURI, aLoadingPrincipal, aDocument); } \
  NS_IMETHOD CheckForUpdate(nsIURI *aManifestURI, nsIPrincipal *aLoadingPrincipal, nsIObserver *aObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CheckForUpdate(aManifestURI, aLoadingPrincipal, aObserver); } \
  NS_IMETHOD OfflineAppAllowed(nsIPrincipal *aPrincipal, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OfflineAppAllowed(aPrincipal, _retval); } \
  NS_IMETHOD OfflineAppAllowedForURI(nsIURI *aURI, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OfflineAppAllowedForURI(aURI, _retval); } \
  NS_IMETHOD AllowOfflineApp(nsIPrincipal *aPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AllowOfflineApp(aPrincipal); } 


#endif /* __gen_nsIOfflineCacheUpdate_h__ */
