/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/passwordmgr/nsILoginManagerPrompter.idl
 */

#ifndef __gen_nsILoginManagerPrompter_h__
#define __gen_nsILoginManagerPrompter_h__


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

class nsIDOMWindow; /* forward declaration */

namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsILoginManagerPrompter */
#define NS_ILOGINMANAGERPROMPTER_IID_STR "c47ff942-9678-44a5-bc9b-05e0d676c79c"

#define NS_ILOGINMANAGERPROMPTER_IID \
  {0xc47ff942, 0x9678, 0x44a5, \
    { 0xbc, 0x9b, 0x05, 0xe0, 0xd6, 0x76, 0xc7, 0x9c }}

class NS_NO_VTABLE nsILoginManagerPrompter : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ILOGINMANAGERPROMPTER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsILoginManagerPrompter;

  /* void promptToSavePassword (in Element aBrowser, in nsILoginInfo aLogin, [optional] in boolean dismissed, [optional] in boolean notifySaved, [optional] in AString autoFilledLoginGuid, [optional] in jsval possibleValues); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PromptToSavePassword(mozilla::dom::Element *aBrowser, nsILoginInfo *aLogin, bool dismissed, bool notifySaved, const nsAString& autoFilledLoginGuid, JS::HandleValue possibleValues) = 0;

  /* void promptToChangePassword (in Element aBrowser, in nsILoginInfo aOldLogin, in nsILoginInfo aNewLogin, [optional] in boolean dismissed, [optional] in boolean notifySaved, [optional] in AString autoSavedLoginGuid, [optional] in AString autoFilledLoginGuid, [optional] in jsval possibleValues); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PromptToChangePassword(mozilla::dom::Element *aBrowser, nsILoginInfo *aOldLogin, nsILoginInfo *aNewLogin, bool dismissed, bool notifySaved, const nsAString& autoSavedLoginGuid, const nsAString& autoFilledLoginGuid, JS::HandleValue possibleValues) = 0;

  /* void promptToChangePasswordWithUsernames (in Element aBrowser, in Array<nsILoginInfo> logins, in nsILoginInfo aNewLogin); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PromptToChangePasswordWithUsernames(mozilla::dom::Element *aBrowser, const nsTArray<RefPtr<nsILoginInfo>>& logins, nsILoginInfo *aNewLogin) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsILoginManagerPrompter, NS_ILOGINMANAGERPROMPTER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSILOGINMANAGERPROMPTER \
  NS_IMETHOD PromptToSavePassword(mozilla::dom::Element *aBrowser, nsILoginInfo *aLogin, bool dismissed, bool notifySaved, const nsAString& autoFilledLoginGuid, JS::HandleValue possibleValues) override; \
  NS_IMETHOD PromptToChangePassword(mozilla::dom::Element *aBrowser, nsILoginInfo *aOldLogin, nsILoginInfo *aNewLogin, bool dismissed, bool notifySaved, const nsAString& autoSavedLoginGuid, const nsAString& autoFilledLoginGuid, JS::HandleValue possibleValues) override; \
  NS_IMETHOD PromptToChangePasswordWithUsernames(mozilla::dom::Element *aBrowser, const nsTArray<RefPtr<nsILoginInfo>>& logins, nsILoginInfo *aNewLogin) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSILOGINMANAGERPROMPTER \
  nsresult PromptToSavePassword(mozilla::dom::Element *aBrowser, nsILoginInfo *aLogin, bool dismissed, bool notifySaved, const nsAString& autoFilledLoginGuid, JS::HandleValue possibleValues); \
  nsresult PromptToChangePassword(mozilla::dom::Element *aBrowser, nsILoginInfo *aOldLogin, nsILoginInfo *aNewLogin, bool dismissed, bool notifySaved, const nsAString& autoSavedLoginGuid, const nsAString& autoFilledLoginGuid, JS::HandleValue possibleValues); \
  nsresult PromptToChangePasswordWithUsernames(mozilla::dom::Element *aBrowser, const nsTArray<RefPtr<nsILoginInfo>>& logins, nsILoginInfo *aNewLogin); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSILOGINMANAGERPROMPTER(_to) \
  NS_IMETHOD PromptToSavePassword(mozilla::dom::Element *aBrowser, nsILoginInfo *aLogin, bool dismissed, bool notifySaved, const nsAString& autoFilledLoginGuid, JS::HandleValue possibleValues) override { return _to PromptToSavePassword(aBrowser, aLogin, dismissed, notifySaved, autoFilledLoginGuid, possibleValues); } \
  NS_IMETHOD PromptToChangePassword(mozilla::dom::Element *aBrowser, nsILoginInfo *aOldLogin, nsILoginInfo *aNewLogin, bool dismissed, bool notifySaved, const nsAString& autoSavedLoginGuid, const nsAString& autoFilledLoginGuid, JS::HandleValue possibleValues) override { return _to PromptToChangePassword(aBrowser, aOldLogin, aNewLogin, dismissed, notifySaved, autoSavedLoginGuid, autoFilledLoginGuid, possibleValues); } \
  NS_IMETHOD PromptToChangePasswordWithUsernames(mozilla::dom::Element *aBrowser, const nsTArray<RefPtr<nsILoginInfo>>& logins, nsILoginInfo *aNewLogin) override { return _to PromptToChangePasswordWithUsernames(aBrowser, logins, aNewLogin); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSILOGINMANAGERPROMPTER(_to) \
  NS_IMETHOD PromptToSavePassword(mozilla::dom::Element *aBrowser, nsILoginInfo *aLogin, bool dismissed, bool notifySaved, const nsAString& autoFilledLoginGuid, JS::HandleValue possibleValues) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PromptToSavePassword(aBrowser, aLogin, dismissed, notifySaved, autoFilledLoginGuid, possibleValues); } \
  NS_IMETHOD PromptToChangePassword(mozilla::dom::Element *aBrowser, nsILoginInfo *aOldLogin, nsILoginInfo *aNewLogin, bool dismissed, bool notifySaved, const nsAString& autoSavedLoginGuid, const nsAString& autoFilledLoginGuid, JS::HandleValue possibleValues) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PromptToChangePassword(aBrowser, aOldLogin, aNewLogin, dismissed, notifySaved, autoSavedLoginGuid, autoFilledLoginGuid, possibleValues); } \
  NS_IMETHOD PromptToChangePasswordWithUsernames(mozilla::dom::Element *aBrowser, const nsTArray<RefPtr<nsILoginInfo>>& logins, nsILoginInfo *aNewLogin) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PromptToChangePasswordWithUsernames(aBrowser, logins, aNewLogin); } 


#define NS_LOGINMANAGERPROMPTER_CONTRACTID "@mozilla.org/login-manager/prompter/;1"

#endif /* __gen_nsILoginManagerPrompter_h__ */
