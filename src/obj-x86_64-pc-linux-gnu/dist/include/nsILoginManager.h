/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/passwordmgr/nsILoginManager.idl
 */

#ifndef __gen_nsILoginManager_h__
#define __gen_nsILoginManager_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
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
class nsILoginInfo; /* forward declaration */

class nsIPropertyBag; /* forward declaration */


/* starting interface:    nsILoginManager */
#define NS_ILOGINMANAGER_IID_STR "38c7f6af-7df9-49c7-b558-2776b24e6cc1"

#define NS_ILOGINMANAGER_IID \
  {0x38c7f6af, 0x7df9, 0x49c7, \
    { 0xb5, 0x58, 0x27, 0x76, 0xb2, 0x4e, 0x6c, 0xc1 }}

class NS_NO_VTABLE nsILoginManager : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ILOGINMANAGER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsILoginManager;

  /* readonly attribute Promise initializationPromise; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetInitializationPromise(::mozilla::dom::Promise * * aInitializationPromise) = 0;

  /* nsILoginInfo addLogin (in nsILoginInfo aLogin); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddLogin(nsILoginInfo *aLogin, nsILoginInfo **_retval) = 0;

  /* Promise addLogins (in jsval aLogins); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddLogins(JS::HandleValue aLogins, ::mozilla::dom::Promise * * _retval) = 0;

  /* void removeLogin (in nsILoginInfo aLogin); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveLogin(nsILoginInfo *aLogin) = 0;

  /* void modifyLogin (in nsILoginInfo oldLogin, in nsISupports newLoginData); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ModifyLogin(nsILoginInfo *oldLogin, nsISupports *newLoginData) = 0;

  /* void recordPasswordUse (in nsILoginInfo aLogin, in boolean aPrivateContextWithoutExplicitConsent, in AString aLoginType, in boolean aFilled); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RecordPasswordUse(nsILoginInfo *aLogin, bool aPrivateContextWithoutExplicitConsent, const nsAString& aLoginType, bool aFilled) = 0;

  /* void removeAllUserFacingLogins (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveAllUserFacingLogins(void) = 0;

  /* void removeAllLogins (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveAllLogins(void) = 0;

  /* Array<nsILoginInfo> getAllLogins (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAllLogins(nsTArray<RefPtr<nsILoginInfo>>& _retval) = 0;

  /* Promise getAllLoginsAsync (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAllLoginsAsync(::mozilla::dom::Promise * * _retval) = 0;

  /* Array<AString> getAllDisabledHosts (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAllDisabledHosts(nsTArray<nsString >& _retval) = 0;

  /* boolean getLoginSavingEnabled (in AString aHost); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLoginSavingEnabled(const nsAString& aHost, bool *_retval) = 0;

  /* void setLoginSavingEnabled (in AString aHost, in boolean isEnabled); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetLoginSavingEnabled(const nsAString& aHost, bool isEnabled) = 0;

  /* Array<nsILoginInfo> findLogins (in AString aOrigin, in AString aActionOrigin, in AString aHttpRealm); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FindLogins(const nsAString& aOrigin, const nsAString& aActionOrigin, const nsAString& aHttpRealm, nsTArray<RefPtr<nsILoginInfo>>& _retval) = 0;

  /* unsigned long countLogins (in AString aOrigin, in AString aActionOrigin, in AString aHttpRealm); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CountLogins(const nsAString& aOrigin, const nsAString& aActionOrigin, const nsAString& aHttpRealm, uint32_t *_retval) = 0;

  /* Promise searchLoginsAsync (in jsval matchData); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SearchLoginsAsync(JS::HandleValue matchData, ::mozilla::dom::Promise * * _retval) = 0;

  /* Array<nsILoginInfo> searchLogins (in nsIPropertyBag matchData); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SearchLogins(nsIPropertyBag *matchData, nsTArray<RefPtr<nsILoginInfo>>& _retval) = 0;

  /* Promise getSyncID (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSyncID(::mozilla::dom::Promise * * _retval) = 0;

  /* Promise setSyncID (in AString syncID); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSyncID(const nsAString& syncID, ::mozilla::dom::Promise * * _retval) = 0;

  /* Promise getLastSync (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLastSync(::mozilla::dom::Promise * * _retval) = 0;

  /* Promise setLastSync (in double timestamp); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetLastSync(double timestamp, ::mozilla::dom::Promise * * _retval) = 0;

  /* readonly attribute boolean uiBusy; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUiBusy(bool *aUiBusy) = 0;

  /* readonly attribute boolean isLoggedIn; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsLoggedIn(bool *aIsLoggedIn) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsILoginManager, NS_ILOGINMANAGER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSILOGINMANAGER \
  NS_IMETHOD GetInitializationPromise(::mozilla::dom::Promise * * aInitializationPromise) override; \
  NS_IMETHOD AddLogin(nsILoginInfo *aLogin, nsILoginInfo **_retval) override; \
  NS_IMETHOD AddLogins(JS::HandleValue aLogins, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD RemoveLogin(nsILoginInfo *aLogin) override; \
  NS_IMETHOD ModifyLogin(nsILoginInfo *oldLogin, nsISupports *newLoginData) override; \
  NS_IMETHOD RecordPasswordUse(nsILoginInfo *aLogin, bool aPrivateContextWithoutExplicitConsent, const nsAString& aLoginType, bool aFilled) override; \
  NS_IMETHOD RemoveAllUserFacingLogins(void) override; \
  NS_IMETHOD RemoveAllLogins(void) override; \
  NS_IMETHOD GetAllLogins(nsTArray<RefPtr<nsILoginInfo>>& _retval) override; \
  NS_IMETHOD GetAllLoginsAsync(::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD GetAllDisabledHosts(nsTArray<nsString >& _retval) override; \
  NS_IMETHOD GetLoginSavingEnabled(const nsAString& aHost, bool *_retval) override; \
  NS_IMETHOD SetLoginSavingEnabled(const nsAString& aHost, bool isEnabled) override; \
  NS_IMETHOD FindLogins(const nsAString& aOrigin, const nsAString& aActionOrigin, const nsAString& aHttpRealm, nsTArray<RefPtr<nsILoginInfo>>& _retval) override; \
  NS_IMETHOD CountLogins(const nsAString& aOrigin, const nsAString& aActionOrigin, const nsAString& aHttpRealm, uint32_t *_retval) override; \
  NS_IMETHOD SearchLoginsAsync(JS::HandleValue matchData, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD SearchLogins(nsIPropertyBag *matchData, nsTArray<RefPtr<nsILoginInfo>>& _retval) override; \
  NS_IMETHOD GetSyncID(::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD SetSyncID(const nsAString& syncID, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD GetLastSync(::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD SetLastSync(double timestamp, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD GetUiBusy(bool *aUiBusy) override; \
  NS_IMETHOD GetIsLoggedIn(bool *aIsLoggedIn) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSILOGINMANAGER \
  nsresult GetInitializationPromise(::mozilla::dom::Promise * * aInitializationPromise); \
  nsresult AddLogin(nsILoginInfo *aLogin, nsILoginInfo **_retval); \
  nsresult AddLogins(JS::HandleValue aLogins, ::mozilla::dom::Promise * * _retval); \
  nsresult RemoveLogin(nsILoginInfo *aLogin); \
  nsresult ModifyLogin(nsILoginInfo *oldLogin, nsISupports *newLoginData); \
  nsresult RecordPasswordUse(nsILoginInfo *aLogin, bool aPrivateContextWithoutExplicitConsent, const nsAString& aLoginType, bool aFilled); \
  nsresult RemoveAllUserFacingLogins(void); \
  nsresult RemoveAllLogins(void); \
  nsresult GetAllLogins(nsTArray<RefPtr<nsILoginInfo>>& _retval); \
  nsresult GetAllLoginsAsync(::mozilla::dom::Promise * * _retval); \
  nsresult GetAllDisabledHosts(nsTArray<nsString >& _retval); \
  nsresult GetLoginSavingEnabled(const nsAString& aHost, bool *_retval); \
  nsresult SetLoginSavingEnabled(const nsAString& aHost, bool isEnabled); \
  nsresult FindLogins(const nsAString& aOrigin, const nsAString& aActionOrigin, const nsAString& aHttpRealm, nsTArray<RefPtr<nsILoginInfo>>& _retval); \
  nsresult CountLogins(const nsAString& aOrigin, const nsAString& aActionOrigin, const nsAString& aHttpRealm, uint32_t *_retval); \
  nsresult SearchLoginsAsync(JS::HandleValue matchData, ::mozilla::dom::Promise * * _retval); \
  nsresult SearchLogins(nsIPropertyBag *matchData, nsTArray<RefPtr<nsILoginInfo>>& _retval); \
  nsresult GetSyncID(::mozilla::dom::Promise * * _retval); \
  nsresult SetSyncID(const nsAString& syncID, ::mozilla::dom::Promise * * _retval); \
  nsresult GetLastSync(::mozilla::dom::Promise * * _retval); \
  nsresult SetLastSync(double timestamp, ::mozilla::dom::Promise * * _retval); \
  nsresult GetUiBusy(bool *aUiBusy); \
  nsresult GetIsLoggedIn(bool *aIsLoggedIn); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSILOGINMANAGER(_to) \
  NS_IMETHOD GetInitializationPromise(::mozilla::dom::Promise * * aInitializationPromise) override { return _to GetInitializationPromise(aInitializationPromise); } \
  NS_IMETHOD AddLogin(nsILoginInfo *aLogin, nsILoginInfo **_retval) override { return _to AddLogin(aLogin, _retval); } \
  NS_IMETHOD AddLogins(JS::HandleValue aLogins, ::mozilla::dom::Promise * * _retval) override { return _to AddLogins(aLogins, _retval); } \
  NS_IMETHOD RemoveLogin(nsILoginInfo *aLogin) override { return _to RemoveLogin(aLogin); } \
  NS_IMETHOD ModifyLogin(nsILoginInfo *oldLogin, nsISupports *newLoginData) override { return _to ModifyLogin(oldLogin, newLoginData); } \
  NS_IMETHOD RecordPasswordUse(nsILoginInfo *aLogin, bool aPrivateContextWithoutExplicitConsent, const nsAString& aLoginType, bool aFilled) override { return _to RecordPasswordUse(aLogin, aPrivateContextWithoutExplicitConsent, aLoginType, aFilled); } \
  NS_IMETHOD RemoveAllUserFacingLogins(void) override { return _to RemoveAllUserFacingLogins(); } \
  NS_IMETHOD RemoveAllLogins(void) override { return _to RemoveAllLogins(); } \
  NS_IMETHOD GetAllLogins(nsTArray<RefPtr<nsILoginInfo>>& _retval) override { return _to GetAllLogins(_retval); } \
  NS_IMETHOD GetAllLoginsAsync(::mozilla::dom::Promise * * _retval) override { return _to GetAllLoginsAsync(_retval); } \
  NS_IMETHOD GetAllDisabledHosts(nsTArray<nsString >& _retval) override { return _to GetAllDisabledHosts(_retval); } \
  NS_IMETHOD GetLoginSavingEnabled(const nsAString& aHost, bool *_retval) override { return _to GetLoginSavingEnabled(aHost, _retval); } \
  NS_IMETHOD SetLoginSavingEnabled(const nsAString& aHost, bool isEnabled) override { return _to SetLoginSavingEnabled(aHost, isEnabled); } \
  NS_IMETHOD FindLogins(const nsAString& aOrigin, const nsAString& aActionOrigin, const nsAString& aHttpRealm, nsTArray<RefPtr<nsILoginInfo>>& _retval) override { return _to FindLogins(aOrigin, aActionOrigin, aHttpRealm, _retval); } \
  NS_IMETHOD CountLogins(const nsAString& aOrigin, const nsAString& aActionOrigin, const nsAString& aHttpRealm, uint32_t *_retval) override { return _to CountLogins(aOrigin, aActionOrigin, aHttpRealm, _retval); } \
  NS_IMETHOD SearchLoginsAsync(JS::HandleValue matchData, ::mozilla::dom::Promise * * _retval) override { return _to SearchLoginsAsync(matchData, _retval); } \
  NS_IMETHOD SearchLogins(nsIPropertyBag *matchData, nsTArray<RefPtr<nsILoginInfo>>& _retval) override { return _to SearchLogins(matchData, _retval); } \
  NS_IMETHOD GetSyncID(::mozilla::dom::Promise * * _retval) override { return _to GetSyncID(_retval); } \
  NS_IMETHOD SetSyncID(const nsAString& syncID, ::mozilla::dom::Promise * * _retval) override { return _to SetSyncID(syncID, _retval); } \
  NS_IMETHOD GetLastSync(::mozilla::dom::Promise * * _retval) override { return _to GetLastSync(_retval); } \
  NS_IMETHOD SetLastSync(double timestamp, ::mozilla::dom::Promise * * _retval) override { return _to SetLastSync(timestamp, _retval); } \
  NS_IMETHOD GetUiBusy(bool *aUiBusy) override { return _to GetUiBusy(aUiBusy); } \
  NS_IMETHOD GetIsLoggedIn(bool *aIsLoggedIn) override { return _to GetIsLoggedIn(aIsLoggedIn); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSILOGINMANAGER(_to) \
  NS_IMETHOD GetInitializationPromise(::mozilla::dom::Promise * * aInitializationPromise) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInitializationPromise(aInitializationPromise); } \
  NS_IMETHOD AddLogin(nsILoginInfo *aLogin, nsILoginInfo **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddLogin(aLogin, _retval); } \
  NS_IMETHOD AddLogins(JS::HandleValue aLogins, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddLogins(aLogins, _retval); } \
  NS_IMETHOD RemoveLogin(nsILoginInfo *aLogin) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveLogin(aLogin); } \
  NS_IMETHOD ModifyLogin(nsILoginInfo *oldLogin, nsISupports *newLoginData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ModifyLogin(oldLogin, newLoginData); } \
  NS_IMETHOD RecordPasswordUse(nsILoginInfo *aLogin, bool aPrivateContextWithoutExplicitConsent, const nsAString& aLoginType, bool aFilled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RecordPasswordUse(aLogin, aPrivateContextWithoutExplicitConsent, aLoginType, aFilled); } \
  NS_IMETHOD RemoveAllUserFacingLogins(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveAllUserFacingLogins(); } \
  NS_IMETHOD RemoveAllLogins(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveAllLogins(); } \
  NS_IMETHOD GetAllLogins(nsTArray<RefPtr<nsILoginInfo>>& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllLogins(_retval); } \
  NS_IMETHOD GetAllLoginsAsync(::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllLoginsAsync(_retval); } \
  NS_IMETHOD GetAllDisabledHosts(nsTArray<nsString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllDisabledHosts(_retval); } \
  NS_IMETHOD GetLoginSavingEnabled(const nsAString& aHost, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoginSavingEnabled(aHost, _retval); } \
  NS_IMETHOD SetLoginSavingEnabled(const nsAString& aHost, bool isEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLoginSavingEnabled(aHost, isEnabled); } \
  NS_IMETHOD FindLogins(const nsAString& aOrigin, const nsAString& aActionOrigin, const nsAString& aHttpRealm, nsTArray<RefPtr<nsILoginInfo>>& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FindLogins(aOrigin, aActionOrigin, aHttpRealm, _retval); } \
  NS_IMETHOD CountLogins(const nsAString& aOrigin, const nsAString& aActionOrigin, const nsAString& aHttpRealm, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CountLogins(aOrigin, aActionOrigin, aHttpRealm, _retval); } \
  NS_IMETHOD SearchLoginsAsync(JS::HandleValue matchData, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SearchLoginsAsync(matchData, _retval); } \
  NS_IMETHOD SearchLogins(nsIPropertyBag *matchData, nsTArray<RefPtr<nsILoginInfo>>& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SearchLogins(matchData, _retval); } \
  NS_IMETHOD GetSyncID(::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSyncID(_retval); } \
  NS_IMETHOD SetSyncID(const nsAString& syncID, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSyncID(syncID, _retval); } \
  NS_IMETHOD GetLastSync(::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastSync(_retval); } \
  NS_IMETHOD SetLastSync(double timestamp, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLastSync(timestamp, _retval); } \
  NS_IMETHOD GetUiBusy(bool *aUiBusy) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUiBusy(aUiBusy); } \
  NS_IMETHOD GetIsLoggedIn(bool *aIsLoggedIn) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsLoggedIn(aIsLoggedIn); } 


#define NS_LOGINMANAGER_CONTRACTID "@mozilla.org/login-manager;1"

#endif /* __gen_nsILoginManager_h__ */
