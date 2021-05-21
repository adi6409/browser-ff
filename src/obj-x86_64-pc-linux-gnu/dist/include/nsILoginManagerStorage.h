/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/passwordmgr/nsILoginManagerStorage.idl
 */

#ifndef __gen_nsILoginManagerStorage_h__
#define __gen_nsILoginManagerStorage_h__


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
class nsIFile; /* forward declaration */

class nsILoginInfo; /* forward declaration */

class nsIPropertyBag; /* forward declaration */


/* starting interface:    nsILoginManagerStorage */
#define NS_ILOGINMANAGERSTORAGE_IID_STR "5df81a93-25e6-4b45-a696-089479e15c7d"

#define NS_ILOGINMANAGERSTORAGE_IID \
  {0x5df81a93, 0x25e6, 0x4b45, \
    { 0xa6, 0x96, 0x08, 0x94, 0x79, 0xe1, 0x5c, 0x7d }}

class NS_NO_VTABLE nsILoginManagerStorage : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ILOGINMANAGERSTORAGE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsILoginManagerStorage;

  /* Promise initialize (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Initialize(::mozilla::dom::Promise * * _retval) = 0;

  /* Promise terminate (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Terminate(::mozilla::dom::Promise * * _retval) = 0;

  /* nsILoginInfo addLogin (in nsILoginInfo aLogin, [optional] in boolean aPreEncrypted, [optional] in jsval aPlaintextUsername, [optional] in jsval aPlaintextPassword); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddLogin(nsILoginInfo *aLogin, bool aPreEncrypted, JS::HandleValue aPlaintextUsername, JS::HandleValue aPlaintextPassword, nsILoginInfo **_retval) = 0;

  /* void removeLogin (in nsILoginInfo aLogin); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveLogin(nsILoginInfo *aLogin) = 0;

  /* void modifyLogin (in nsILoginInfo oldLogin, in nsISupports newLoginData); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ModifyLogin(nsILoginInfo *oldLogin, nsISupports *newLoginData) = 0;

  /* void recordPasswordUse (in nsILoginInfo aLogin); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RecordPasswordUse(nsILoginInfo *aLogin) = 0;

  /* void removeAllUserFacingLogins (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveAllUserFacingLogins(void) = 0;

  /* void removeAllLogins (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveAllLogins(void) = 0;

  /* Array<nsILoginInfo> getAllLogins (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAllLogins(nsTArray<RefPtr<nsILoginInfo>>& _retval) = 0;

  /* Promise getAllLoginsAsync (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAllLoginsAsync(::mozilla::dom::Promise * * _retval) = 0;

  /* Promise searchLoginsAsync (in jsval matchData); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SearchLoginsAsync(JS::HandleValue matchData, ::mozilla::dom::Promise * * _retval) = 0;

  /* Array<nsILoginInfo> searchLogins (in nsIPropertyBag matchData); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SearchLogins(nsIPropertyBag *matchData, nsTArray<RefPtr<nsILoginInfo>>& _retval) = 0;

  /* Array<nsILoginInfo> findLogins (in AString aOrigin, in AString aActionOrigin, in AString aHttpRealm); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FindLogins(const nsAString& aOrigin, const nsAString& aActionOrigin, const nsAString& aHttpRealm, nsTArray<RefPtr<nsILoginInfo>>& _retval) = 0;

  /* unsigned long countLogins (in AString aOrigin, in AString aActionOrigin, in AString aHttpRealm); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CountLogins(const nsAString& aOrigin, const nsAString& aActionOrigin, const nsAString& aHttpRealm, uint32_t *_retval) = 0;

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

  NS_DEFINE_STATIC_IID_ACCESSOR(nsILoginManagerStorage, NS_ILOGINMANAGERSTORAGE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSILOGINMANAGERSTORAGE \
  NS_IMETHOD Initialize(::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD Terminate(::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD AddLogin(nsILoginInfo *aLogin, bool aPreEncrypted, JS::HandleValue aPlaintextUsername, JS::HandleValue aPlaintextPassword, nsILoginInfo **_retval) override; \
  NS_IMETHOD RemoveLogin(nsILoginInfo *aLogin) override; \
  NS_IMETHOD ModifyLogin(nsILoginInfo *oldLogin, nsISupports *newLoginData) override; \
  NS_IMETHOD RecordPasswordUse(nsILoginInfo *aLogin) override; \
  NS_IMETHOD RemoveAllUserFacingLogins(void) override; \
  NS_IMETHOD RemoveAllLogins(void) override; \
  NS_IMETHOD GetAllLogins(nsTArray<RefPtr<nsILoginInfo>>& _retval) override; \
  NS_IMETHOD GetAllLoginsAsync(::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD SearchLoginsAsync(JS::HandleValue matchData, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD SearchLogins(nsIPropertyBag *matchData, nsTArray<RefPtr<nsILoginInfo>>& _retval) override; \
  NS_IMETHOD FindLogins(const nsAString& aOrigin, const nsAString& aActionOrigin, const nsAString& aHttpRealm, nsTArray<RefPtr<nsILoginInfo>>& _retval) override; \
  NS_IMETHOD CountLogins(const nsAString& aOrigin, const nsAString& aActionOrigin, const nsAString& aHttpRealm, uint32_t *_retval) override; \
  NS_IMETHOD GetSyncID(::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD SetSyncID(const nsAString& syncID, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD GetLastSync(::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD SetLastSync(double timestamp, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD GetUiBusy(bool *aUiBusy) override; \
  NS_IMETHOD GetIsLoggedIn(bool *aIsLoggedIn) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSILOGINMANAGERSTORAGE \
  nsresult Initialize(::mozilla::dom::Promise * * _retval); \
  nsresult Terminate(::mozilla::dom::Promise * * _retval); \
  nsresult AddLogin(nsILoginInfo *aLogin, bool aPreEncrypted, JS::HandleValue aPlaintextUsername, JS::HandleValue aPlaintextPassword, nsILoginInfo **_retval); \
  nsresult RemoveLogin(nsILoginInfo *aLogin); \
  nsresult ModifyLogin(nsILoginInfo *oldLogin, nsISupports *newLoginData); \
  nsresult RecordPasswordUse(nsILoginInfo *aLogin); \
  nsresult RemoveAllUserFacingLogins(void); \
  nsresult RemoveAllLogins(void); \
  nsresult GetAllLogins(nsTArray<RefPtr<nsILoginInfo>>& _retval); \
  nsresult GetAllLoginsAsync(::mozilla::dom::Promise * * _retval); \
  nsresult SearchLoginsAsync(JS::HandleValue matchData, ::mozilla::dom::Promise * * _retval); \
  nsresult SearchLogins(nsIPropertyBag *matchData, nsTArray<RefPtr<nsILoginInfo>>& _retval); \
  nsresult FindLogins(const nsAString& aOrigin, const nsAString& aActionOrigin, const nsAString& aHttpRealm, nsTArray<RefPtr<nsILoginInfo>>& _retval); \
  nsresult CountLogins(const nsAString& aOrigin, const nsAString& aActionOrigin, const nsAString& aHttpRealm, uint32_t *_retval); \
  nsresult GetSyncID(::mozilla::dom::Promise * * _retval); \
  nsresult SetSyncID(const nsAString& syncID, ::mozilla::dom::Promise * * _retval); \
  nsresult GetLastSync(::mozilla::dom::Promise * * _retval); \
  nsresult SetLastSync(double timestamp, ::mozilla::dom::Promise * * _retval); \
  nsresult GetUiBusy(bool *aUiBusy); \
  nsresult GetIsLoggedIn(bool *aIsLoggedIn); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSILOGINMANAGERSTORAGE(_to) \
  NS_IMETHOD Initialize(::mozilla::dom::Promise * * _retval) override { return _to Initialize(_retval); } \
  NS_IMETHOD Terminate(::mozilla::dom::Promise * * _retval) override { return _to Terminate(_retval); } \
  NS_IMETHOD AddLogin(nsILoginInfo *aLogin, bool aPreEncrypted, JS::HandleValue aPlaintextUsername, JS::HandleValue aPlaintextPassword, nsILoginInfo **_retval) override { return _to AddLogin(aLogin, aPreEncrypted, aPlaintextUsername, aPlaintextPassword, _retval); } \
  NS_IMETHOD RemoveLogin(nsILoginInfo *aLogin) override { return _to RemoveLogin(aLogin); } \
  NS_IMETHOD ModifyLogin(nsILoginInfo *oldLogin, nsISupports *newLoginData) override { return _to ModifyLogin(oldLogin, newLoginData); } \
  NS_IMETHOD RecordPasswordUse(nsILoginInfo *aLogin) override { return _to RecordPasswordUse(aLogin); } \
  NS_IMETHOD RemoveAllUserFacingLogins(void) override { return _to RemoveAllUserFacingLogins(); } \
  NS_IMETHOD RemoveAllLogins(void) override { return _to RemoveAllLogins(); } \
  NS_IMETHOD GetAllLogins(nsTArray<RefPtr<nsILoginInfo>>& _retval) override { return _to GetAllLogins(_retval); } \
  NS_IMETHOD GetAllLoginsAsync(::mozilla::dom::Promise * * _retval) override { return _to GetAllLoginsAsync(_retval); } \
  NS_IMETHOD SearchLoginsAsync(JS::HandleValue matchData, ::mozilla::dom::Promise * * _retval) override { return _to SearchLoginsAsync(matchData, _retval); } \
  NS_IMETHOD SearchLogins(nsIPropertyBag *matchData, nsTArray<RefPtr<nsILoginInfo>>& _retval) override { return _to SearchLogins(matchData, _retval); } \
  NS_IMETHOD FindLogins(const nsAString& aOrigin, const nsAString& aActionOrigin, const nsAString& aHttpRealm, nsTArray<RefPtr<nsILoginInfo>>& _retval) override { return _to FindLogins(aOrigin, aActionOrigin, aHttpRealm, _retval); } \
  NS_IMETHOD CountLogins(const nsAString& aOrigin, const nsAString& aActionOrigin, const nsAString& aHttpRealm, uint32_t *_retval) override { return _to CountLogins(aOrigin, aActionOrigin, aHttpRealm, _retval); } \
  NS_IMETHOD GetSyncID(::mozilla::dom::Promise * * _retval) override { return _to GetSyncID(_retval); } \
  NS_IMETHOD SetSyncID(const nsAString& syncID, ::mozilla::dom::Promise * * _retval) override { return _to SetSyncID(syncID, _retval); } \
  NS_IMETHOD GetLastSync(::mozilla::dom::Promise * * _retval) override { return _to GetLastSync(_retval); } \
  NS_IMETHOD SetLastSync(double timestamp, ::mozilla::dom::Promise * * _retval) override { return _to SetLastSync(timestamp, _retval); } \
  NS_IMETHOD GetUiBusy(bool *aUiBusy) override { return _to GetUiBusy(aUiBusy); } \
  NS_IMETHOD GetIsLoggedIn(bool *aIsLoggedIn) override { return _to GetIsLoggedIn(aIsLoggedIn); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSILOGINMANAGERSTORAGE(_to) \
  NS_IMETHOD Initialize(::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Initialize(_retval); } \
  NS_IMETHOD Terminate(::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Terminate(_retval); } \
  NS_IMETHOD AddLogin(nsILoginInfo *aLogin, bool aPreEncrypted, JS::HandleValue aPlaintextUsername, JS::HandleValue aPlaintextPassword, nsILoginInfo **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddLogin(aLogin, aPreEncrypted, aPlaintextUsername, aPlaintextPassword, _retval); } \
  NS_IMETHOD RemoveLogin(nsILoginInfo *aLogin) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveLogin(aLogin); } \
  NS_IMETHOD ModifyLogin(nsILoginInfo *oldLogin, nsISupports *newLoginData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ModifyLogin(oldLogin, newLoginData); } \
  NS_IMETHOD RecordPasswordUse(nsILoginInfo *aLogin) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RecordPasswordUse(aLogin); } \
  NS_IMETHOD RemoveAllUserFacingLogins(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveAllUserFacingLogins(); } \
  NS_IMETHOD RemoveAllLogins(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveAllLogins(); } \
  NS_IMETHOD GetAllLogins(nsTArray<RefPtr<nsILoginInfo>>& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllLogins(_retval); } \
  NS_IMETHOD GetAllLoginsAsync(::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllLoginsAsync(_retval); } \
  NS_IMETHOD SearchLoginsAsync(JS::HandleValue matchData, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SearchLoginsAsync(matchData, _retval); } \
  NS_IMETHOD SearchLogins(nsIPropertyBag *matchData, nsTArray<RefPtr<nsILoginInfo>>& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SearchLogins(matchData, _retval); } \
  NS_IMETHOD FindLogins(const nsAString& aOrigin, const nsAString& aActionOrigin, const nsAString& aHttpRealm, nsTArray<RefPtr<nsILoginInfo>>& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FindLogins(aOrigin, aActionOrigin, aHttpRealm, _retval); } \
  NS_IMETHOD CountLogins(const nsAString& aOrigin, const nsAString& aActionOrigin, const nsAString& aHttpRealm, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CountLogins(aOrigin, aActionOrigin, aHttpRealm, _retval); } \
  NS_IMETHOD GetSyncID(::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSyncID(_retval); } \
  NS_IMETHOD SetSyncID(const nsAString& syncID, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSyncID(syncID, _retval); } \
  NS_IMETHOD GetLastSync(::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastSync(_retval); } \
  NS_IMETHOD SetLastSync(double timestamp, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLastSync(timestamp, _retval); } \
  NS_IMETHOD GetUiBusy(bool *aUiBusy) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUiBusy(aUiBusy); } \
  NS_IMETHOD GetIsLoggedIn(bool *aIsLoggedIn) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsLoggedIn(aIsLoggedIn); } 


#endif /* __gen_nsILoginManagerStorage_h__ */
