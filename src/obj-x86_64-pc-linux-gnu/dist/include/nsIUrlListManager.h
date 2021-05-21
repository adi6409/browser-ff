/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/url-classifier/nsIUrlListManager.idl
 */

#ifndef __gen_nsIUrlListManager_h__
#define __gen_nsIUrlListManager_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIPrincipal; /* forward declaration */


/* starting interface:    nsIUrlListManager */
#define NS_IURLLISTMANAGER_IID_STR "d60a08ee-5c83-4eb6-bdfb-79fd0716501e"

#define NS_IURLLISTMANAGER_IID \
  {0xd60a08ee, 0x5c83, 0x4eb6, \
    { 0xbd, 0xfb, 0x79, 0xfd, 0x07, 0x16, 0x50, 0x1e }}

class NS_NO_VTABLE nsIUrlListManager : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURLLISTMANAGER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUrlListManager;

  /* ACString getGethashUrl (in ACString tableName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetGethashUrl(const nsACString& tableName, nsACString& _retval) = 0;

  /* ACString getUpdateUrl (in ACString tableName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUpdateUrl(const nsACString& tableName, nsACString& _retval) = 0;

  /* boolean registerTable (in ACString tableName, in ACString providerName, in ACString updateUrl, in ACString gethashUrl); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RegisterTable(const nsACString& tableName, const nsACString& providerName, const nsACString& updateUrl, const nsACString& gethashUrl, bool *_retval) = 0;

  /* void unregisterTable (in ACString tableName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UnregisterTable(const nsACString& tableName) = 0;

  /* void enableUpdate (in ACString tableName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD EnableUpdate(const nsACString& tableName) = 0;

  /* void disableAllUpdates (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DisableAllUpdates(void) = 0;

  /* void disableUpdate (in ACString tableName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DisableUpdate(const nsACString& tableName) = 0;

  /* void maybeToggleUpdateChecking (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MaybeToggleUpdateChecking(void) = 0;

  /* boolean checkForUpdates (in ACString updateUrl); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CheckForUpdates(const nsACString& updateUrl, bool *_retval) = 0;

  /* boolean forceUpdates (in ACString tableNames); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ForceUpdates(const nsACString& tableNames, bool *_retval) = 0;

  /* uint64_t getBackOffTime (in ACString provider); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetBackOffTime(const nsACString& provider, uint64_t *_retval) = 0;

  /* boolean isRegistered (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsRegistered(bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUrlListManager, NS_IURLLISTMANAGER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURLLISTMANAGER \
  NS_IMETHOD GetGethashUrl(const nsACString& tableName, nsACString& _retval) override; \
  NS_IMETHOD GetUpdateUrl(const nsACString& tableName, nsACString& _retval) override; \
  NS_IMETHOD RegisterTable(const nsACString& tableName, const nsACString& providerName, const nsACString& updateUrl, const nsACString& gethashUrl, bool *_retval) override; \
  NS_IMETHOD UnregisterTable(const nsACString& tableName) override; \
  NS_IMETHOD EnableUpdate(const nsACString& tableName) override; \
  NS_IMETHOD DisableAllUpdates(void) override; \
  NS_IMETHOD DisableUpdate(const nsACString& tableName) override; \
  NS_IMETHOD MaybeToggleUpdateChecking(void) override; \
  NS_IMETHOD CheckForUpdates(const nsACString& updateUrl, bool *_retval) override; \
  NS_IMETHOD ForceUpdates(const nsACString& tableNames, bool *_retval) override; \
  NS_IMETHOD GetBackOffTime(const nsACString& provider, uint64_t *_retval) override; \
  NS_IMETHOD IsRegistered(bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURLLISTMANAGER \
  nsresult GetGethashUrl(const nsACString& tableName, nsACString& _retval); \
  nsresult GetUpdateUrl(const nsACString& tableName, nsACString& _retval); \
  nsresult RegisterTable(const nsACString& tableName, const nsACString& providerName, const nsACString& updateUrl, const nsACString& gethashUrl, bool *_retval); \
  nsresult UnregisterTable(const nsACString& tableName); \
  nsresult EnableUpdate(const nsACString& tableName); \
  nsresult DisableAllUpdates(void); \
  nsresult DisableUpdate(const nsACString& tableName); \
  nsresult MaybeToggleUpdateChecking(void); \
  nsresult CheckForUpdates(const nsACString& updateUrl, bool *_retval); \
  nsresult ForceUpdates(const nsACString& tableNames, bool *_retval); \
  nsresult GetBackOffTime(const nsACString& provider, uint64_t *_retval); \
  nsresult IsRegistered(bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURLLISTMANAGER(_to) \
  NS_IMETHOD GetGethashUrl(const nsACString& tableName, nsACString& _retval) override { return _to GetGethashUrl(tableName, _retval); } \
  NS_IMETHOD GetUpdateUrl(const nsACString& tableName, nsACString& _retval) override { return _to GetUpdateUrl(tableName, _retval); } \
  NS_IMETHOD RegisterTable(const nsACString& tableName, const nsACString& providerName, const nsACString& updateUrl, const nsACString& gethashUrl, bool *_retval) override { return _to RegisterTable(tableName, providerName, updateUrl, gethashUrl, _retval); } \
  NS_IMETHOD UnregisterTable(const nsACString& tableName) override { return _to UnregisterTable(tableName); } \
  NS_IMETHOD EnableUpdate(const nsACString& tableName) override { return _to EnableUpdate(tableName); } \
  NS_IMETHOD DisableAllUpdates(void) override { return _to DisableAllUpdates(); } \
  NS_IMETHOD DisableUpdate(const nsACString& tableName) override { return _to DisableUpdate(tableName); } \
  NS_IMETHOD MaybeToggleUpdateChecking(void) override { return _to MaybeToggleUpdateChecking(); } \
  NS_IMETHOD CheckForUpdates(const nsACString& updateUrl, bool *_retval) override { return _to CheckForUpdates(updateUrl, _retval); } \
  NS_IMETHOD ForceUpdates(const nsACString& tableNames, bool *_retval) override { return _to ForceUpdates(tableNames, _retval); } \
  NS_IMETHOD GetBackOffTime(const nsACString& provider, uint64_t *_retval) override { return _to GetBackOffTime(provider, _retval); } \
  NS_IMETHOD IsRegistered(bool *_retval) override { return _to IsRegistered(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURLLISTMANAGER(_to) \
  NS_IMETHOD GetGethashUrl(const nsACString& tableName, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetGethashUrl(tableName, _retval); } \
  NS_IMETHOD GetUpdateUrl(const nsACString& tableName, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUpdateUrl(tableName, _retval); } \
  NS_IMETHOD RegisterTable(const nsACString& tableName, const nsACString& providerName, const nsACString& updateUrl, const nsACString& gethashUrl, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterTable(tableName, providerName, updateUrl, gethashUrl, _retval); } \
  NS_IMETHOD UnregisterTable(const nsACString& tableName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UnregisterTable(tableName); } \
  NS_IMETHOD EnableUpdate(const nsACString& tableName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EnableUpdate(tableName); } \
  NS_IMETHOD DisableAllUpdates(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DisableAllUpdates(); } \
  NS_IMETHOD DisableUpdate(const nsACString& tableName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DisableUpdate(tableName); } \
  NS_IMETHOD MaybeToggleUpdateChecking(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MaybeToggleUpdateChecking(); } \
  NS_IMETHOD CheckForUpdates(const nsACString& updateUrl, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CheckForUpdates(updateUrl, _retval); } \
  NS_IMETHOD ForceUpdates(const nsACString& tableNames, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ForceUpdates(tableNames, _retval); } \
  NS_IMETHOD GetBackOffTime(const nsACString& provider, uint64_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBackOffTime(provider, _retval); } \
  NS_IMETHOD IsRegistered(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsRegistered(_retval); } 


#endif /* __gen_nsIUrlListManager_h__ */
