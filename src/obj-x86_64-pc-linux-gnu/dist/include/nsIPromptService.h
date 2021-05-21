/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/windowwatcher/nsIPromptService.idl
 */

#ifndef __gen_nsIPromptService_h__
#define __gen_nsIPromptService_h__


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
class mozIDOMWindowProxy; /* forward declaration */

class nsIAuthPromptCallback; /* forward declaration */

class nsIAuthInformation; /* forward declaration */

class nsICancelable; /* forward declaration */

class nsIChannel; /* forward declaration */

namespace mozilla {
namespace dom {
class BrowsingContext; /* webidl BrowsingContext */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIPromptService */
#define NS_IPROMPTSERVICE_IID_STR "404ebfa2-d8f4-4c94-8416-e65a55f9df5a"

#define NS_IPROMPTSERVICE_IID \
  {0x404ebfa2, 0xd8f4, 0x4c94, \
    { 0x84, 0x16, 0xe6, 0x5a, 0x55, 0xf9, 0xdf, 0x5a }}

class NS_NO_VTABLE nsIPromptService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPROMPTSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPromptService;

  /* void alert (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Alert(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText) = 0;

  /* void alertBC (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AlertBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText) = 0;

  /* Promise asyncAlert (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncAlert(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, ::mozilla::dom::Promise * * _retval) = 0;

  /* void alertCheck (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, in wstring aCheckMsg, inout boolean aCheckState); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AlertCheck(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aCheckMsg, bool *aCheckState) = 0;

  /* void alertCheckBC (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in wstring aCheckMsg, inout boolean aCheckState); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AlertCheckBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aCheckMsg, bool *aCheckState) = 0;

  /* Promise asyncAlertCheck (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in wstring aCheckMsg, in boolean aCheckState); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncAlertCheck(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aCheckMsg, bool aCheckState, ::mozilla::dom::Promise * * _retval) = 0;

  /* boolean confirm (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Confirm(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, bool *_retval) = 0;

  /* boolean confirmBC (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ConfirmBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, bool *_retval) = 0;

  /* Promise asyncConfirm (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncConfirm(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, ::mozilla::dom::Promise * * _retval) = 0;

  /* boolean confirmCheck (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, in wstring aCheckMsg, inout boolean aCheckState); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ConfirmCheck(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) = 0;

  /* boolean confirmCheckBC (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in wstring aCheckMsg, inout boolean aCheckState); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ConfirmCheckBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) = 0;

  /* Promise asyncConfirmCheck (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in wstring aCheckMsg, in boolean aCheckState); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncConfirmCheck(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aCheckMsg, bool aCheckState, ::mozilla::dom::Promise * * _retval) = 0;

  enum {
    BUTTON_POS_0 = 1U,
    BUTTON_POS_1 = 256U,
    BUTTON_POS_2 = 65536U,
    BUTTON_TITLE_OK = 1U,
    BUTTON_TITLE_CANCEL = 2U,
    BUTTON_TITLE_YES = 3U,
    BUTTON_TITLE_NO = 4U,
    BUTTON_TITLE_SAVE = 5U,
    BUTTON_TITLE_DONT_SAVE = 6U,
    BUTTON_TITLE_REVERT = 7U,
    BUTTON_TITLE_IS_STRING = 127U,
    BUTTON_POS_0_DEFAULT = 0U,
    BUTTON_POS_1_DEFAULT = 16777216U,
    BUTTON_POS_2_DEFAULT = 33554432U,
    BUTTON_DELAY_ENABLE = 67108864U,
    STD_OK_CANCEL_BUTTONS = 513U,
    STD_YES_NO_BUTTONS = 1027U,
    MODAL_TYPE_CONTENT = 1U,
    MODAL_TYPE_TAB = 2U,
    MODAL_TYPE_WINDOW = 3U
  };

  /* int32_t confirmEx (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, in unsigned long aButtonFlags, in wstring aButton0Title, in wstring aButton1Title, in wstring aButton2Title, in wstring aCheckMsg, inout boolean aCheckState); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ConfirmEx(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, uint32_t aButtonFlags, const char16_t * aButton0Title, const char16_t * aButton1Title, const char16_t * aButton2Title, const char16_t * aCheckMsg, bool *aCheckState, int32_t *_retval) = 0;

  /* int32_t confirmExBC (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in unsigned long aButtonFlags, in wstring aButton0Title, in wstring aButton1Title, in wstring aButton2Title, in wstring aCheckMsg, inout boolean aCheckState); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ConfirmExBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, uint32_t aButtonFlags, const char16_t * aButton0Title, const char16_t * aButton1Title, const char16_t * aButton2Title, const char16_t * aCheckMsg, bool *aCheckState, int32_t *_retval) = 0;

  /* Promise asyncConfirmEx (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in unsigned long aButtonFlags, in wstring aButton0Title, in wstring aButton1Title, in wstring aButton2Title, in wstring aCheckMsg, in boolean aCheckState, [optional] in jsval aExtraArgs); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncConfirmEx(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, uint32_t aButtonFlags, const char16_t * aButton0Title, const char16_t * aButton1Title, const char16_t * aButton2Title, const char16_t * aCheckMsg, bool aCheckState, JS::HandleValue aExtraArgs, ::mozilla::dom::Promise * * _retval) = 0;

  /* boolean prompt (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, inout wstring aValue, in wstring aCheckMsg, inout boolean aCheckState); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Prompt(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, char16_t * *aValue, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) = 0;

  /* boolean promptBC (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, inout wstring aValue, in wstring aCheckMsg, inout boolean aCheckState); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PromptBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, char16_t * *aValue, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) = 0;

  /* Promise asyncPrompt (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in wstring aValue, in wstring aCheckMsg, in boolean aCheckState); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncPrompt(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aValue, const char16_t * aCheckMsg, bool aCheckState, ::mozilla::dom::Promise * * _retval) = 0;

  /* boolean promptUsernameAndPassword (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, inout wstring aUsername, inout wstring aPassword, in wstring aCheckMsg, inout boolean aCheckState); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PromptUsernameAndPassword(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, char16_t * *aUsername, char16_t * *aPassword, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) = 0;

  /* boolean promptUsernameAndPasswordBC (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, inout wstring aUsername, inout wstring aPassword, in wstring aCheckMsg, inout boolean aCheckState); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PromptUsernameAndPasswordBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, char16_t * *aUsername, char16_t * *aPassword, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) = 0;

  /* Promise asyncPromptUsernameAndPassword (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in wstring aUsername, in wstring aPassword, in wstring aCheckMsg, in boolean aCheckState); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncPromptUsernameAndPassword(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aUsername, const char16_t * aPassword, const char16_t * aCheckMsg, bool aCheckState, ::mozilla::dom::Promise * * _retval) = 0;

  /* boolean promptPassword (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, inout wstring aPassword, in wstring aCheckMsg, inout boolean aCheckState); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PromptPassword(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, char16_t * *aPassword, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) = 0;

  /* boolean promptPasswordBC (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, inout wstring aPassword, in wstring aCheckMsg, inout boolean aCheckState); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PromptPasswordBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, char16_t * *aPassword, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) = 0;

  /* Promise asyncPromptPassword (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in wstring aPassword, in wstring aCheckMsg, in boolean aCheckState); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncPromptPassword(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aPassword, const char16_t * aCheckMsg, bool aCheckState, ::mozilla::dom::Promise * * _retval) = 0;

  /* boolean select (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, in Array<AString> aSelectList, out long aOutSelection); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Select(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, const nsTArray<nsString >& aSelectList, int32_t *aOutSelection, bool *_retval) = 0;

  /* boolean selectBC (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in Array<AString> aSelectList, out long aOutSelection); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SelectBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const nsTArray<nsString >& aSelectList, int32_t *aOutSelection, bool *_retval) = 0;

  /* Promise asyncSelect (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in Array<AString> aSelectList); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncSelect(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const nsTArray<nsString >& aSelectList, ::mozilla::dom::Promise * * _retval) = 0;

  /* boolean promptAuth (in mozIDOMWindowProxy aParent, in nsIChannel aChannel, in uint32_t level, in nsIAuthInformation authInfo, in wstring checkboxLabel, inout boolean checkValue); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PromptAuth(mozIDOMWindowProxy *aParent, nsIChannel *aChannel, uint32_t level, nsIAuthInformation *authInfo, const char16_t * checkboxLabel, bool *checkValue, bool *_retval) = 0;

  /* boolean promptAuthBC (in BrowsingContext aBrowsingContext, in unsigned long modalType, in nsIChannel aChannel, in uint32_t level, in nsIAuthInformation authInfo, in wstring checkboxLabel, inout boolean checkValue); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PromptAuthBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, nsIChannel *aChannel, uint32_t level, nsIAuthInformation *authInfo, const char16_t * checkboxLabel, bool *checkValue, bool *_retval) = 0;

  /* Promise asyncPromptAuth (in BrowsingContext aBrowsingContext, in unsigned long modalType, in nsIChannel aChannel, in uint32_t level, in nsIAuthInformation authInfo, in wstring checkboxLabel, in boolean checkValue); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncPromptAuth(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, nsIChannel *aChannel, uint32_t level, nsIAuthInformation *authInfo, const char16_t * checkboxLabel, bool checkValue, ::mozilla::dom::Promise * * _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPromptService, NS_IPROMPTSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPROMPTSERVICE \
  NS_IMETHOD Alert(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText) override; \
  NS_IMETHOD AlertBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText) override; \
  NS_IMETHOD AsyncAlert(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD AlertCheck(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aCheckMsg, bool *aCheckState) override; \
  NS_IMETHOD AlertCheckBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aCheckMsg, bool *aCheckState) override; \
  NS_IMETHOD AsyncAlertCheck(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aCheckMsg, bool aCheckState, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD Confirm(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, bool *_retval) override; \
  NS_IMETHOD ConfirmBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, bool *_retval) override; \
  NS_IMETHOD AsyncConfirm(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD ConfirmCheck(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) override; \
  NS_IMETHOD ConfirmCheckBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) override; \
  NS_IMETHOD AsyncConfirmCheck(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aCheckMsg, bool aCheckState, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD ConfirmEx(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, uint32_t aButtonFlags, const char16_t * aButton0Title, const char16_t * aButton1Title, const char16_t * aButton2Title, const char16_t * aCheckMsg, bool *aCheckState, int32_t *_retval) override; \
  NS_IMETHOD ConfirmExBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, uint32_t aButtonFlags, const char16_t * aButton0Title, const char16_t * aButton1Title, const char16_t * aButton2Title, const char16_t * aCheckMsg, bool *aCheckState, int32_t *_retval) override; \
  NS_IMETHOD AsyncConfirmEx(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, uint32_t aButtonFlags, const char16_t * aButton0Title, const char16_t * aButton1Title, const char16_t * aButton2Title, const char16_t * aCheckMsg, bool aCheckState, JS::HandleValue aExtraArgs, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD Prompt(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, char16_t * *aValue, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) override; \
  NS_IMETHOD PromptBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, char16_t * *aValue, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) override; \
  NS_IMETHOD AsyncPrompt(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aValue, const char16_t * aCheckMsg, bool aCheckState, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD PromptUsernameAndPassword(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, char16_t * *aUsername, char16_t * *aPassword, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) override; \
  NS_IMETHOD PromptUsernameAndPasswordBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, char16_t * *aUsername, char16_t * *aPassword, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) override; \
  NS_IMETHOD AsyncPromptUsernameAndPassword(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aUsername, const char16_t * aPassword, const char16_t * aCheckMsg, bool aCheckState, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD PromptPassword(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, char16_t * *aPassword, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) override; \
  NS_IMETHOD PromptPasswordBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, char16_t * *aPassword, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) override; \
  NS_IMETHOD AsyncPromptPassword(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aPassword, const char16_t * aCheckMsg, bool aCheckState, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD Select(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, const nsTArray<nsString >& aSelectList, int32_t *aOutSelection, bool *_retval) override; \
  NS_IMETHOD SelectBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const nsTArray<nsString >& aSelectList, int32_t *aOutSelection, bool *_retval) override; \
  NS_IMETHOD AsyncSelect(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const nsTArray<nsString >& aSelectList, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD PromptAuth(mozIDOMWindowProxy *aParent, nsIChannel *aChannel, uint32_t level, nsIAuthInformation *authInfo, const char16_t * checkboxLabel, bool *checkValue, bool *_retval) override; \
  NS_IMETHOD PromptAuthBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, nsIChannel *aChannel, uint32_t level, nsIAuthInformation *authInfo, const char16_t * checkboxLabel, bool *checkValue, bool *_retval) override; \
  NS_IMETHOD AsyncPromptAuth(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, nsIChannel *aChannel, uint32_t level, nsIAuthInformation *authInfo, const char16_t * checkboxLabel, bool checkValue, ::mozilla::dom::Promise * * _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPROMPTSERVICE \
  nsresult Alert(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText); \
  nsresult AlertBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText); \
  nsresult AsyncAlert(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, ::mozilla::dom::Promise * * _retval); \
  nsresult AlertCheck(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aCheckMsg, bool *aCheckState); \
  nsresult AlertCheckBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aCheckMsg, bool *aCheckState); \
  nsresult AsyncAlertCheck(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aCheckMsg, bool aCheckState, ::mozilla::dom::Promise * * _retval); \
  nsresult Confirm(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, bool *_retval); \
  nsresult ConfirmBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, bool *_retval); \
  nsresult AsyncConfirm(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, ::mozilla::dom::Promise * * _retval); \
  nsresult ConfirmCheck(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval); \
  nsresult ConfirmCheckBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval); \
  nsresult AsyncConfirmCheck(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aCheckMsg, bool aCheckState, ::mozilla::dom::Promise * * _retval); \
  nsresult ConfirmEx(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, uint32_t aButtonFlags, const char16_t * aButton0Title, const char16_t * aButton1Title, const char16_t * aButton2Title, const char16_t * aCheckMsg, bool *aCheckState, int32_t *_retval); \
  nsresult ConfirmExBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, uint32_t aButtonFlags, const char16_t * aButton0Title, const char16_t * aButton1Title, const char16_t * aButton2Title, const char16_t * aCheckMsg, bool *aCheckState, int32_t *_retval); \
  nsresult AsyncConfirmEx(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, uint32_t aButtonFlags, const char16_t * aButton0Title, const char16_t * aButton1Title, const char16_t * aButton2Title, const char16_t * aCheckMsg, bool aCheckState, JS::HandleValue aExtraArgs, ::mozilla::dom::Promise * * _retval); \
  nsresult Prompt(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, char16_t * *aValue, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval); \
  nsresult PromptBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, char16_t * *aValue, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval); \
  nsresult AsyncPrompt(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aValue, const char16_t * aCheckMsg, bool aCheckState, ::mozilla::dom::Promise * * _retval); \
  nsresult PromptUsernameAndPassword(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, char16_t * *aUsername, char16_t * *aPassword, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval); \
  nsresult PromptUsernameAndPasswordBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, char16_t * *aUsername, char16_t * *aPassword, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval); \
  nsresult AsyncPromptUsernameAndPassword(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aUsername, const char16_t * aPassword, const char16_t * aCheckMsg, bool aCheckState, ::mozilla::dom::Promise * * _retval); \
  nsresult PromptPassword(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, char16_t * *aPassword, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval); \
  nsresult PromptPasswordBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, char16_t * *aPassword, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval); \
  nsresult AsyncPromptPassword(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aPassword, const char16_t * aCheckMsg, bool aCheckState, ::mozilla::dom::Promise * * _retval); \
  nsresult Select(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, const nsTArray<nsString >& aSelectList, int32_t *aOutSelection, bool *_retval); \
  nsresult SelectBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const nsTArray<nsString >& aSelectList, int32_t *aOutSelection, bool *_retval); \
  nsresult AsyncSelect(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const nsTArray<nsString >& aSelectList, ::mozilla::dom::Promise * * _retval); \
  nsresult PromptAuth(mozIDOMWindowProxy *aParent, nsIChannel *aChannel, uint32_t level, nsIAuthInformation *authInfo, const char16_t * checkboxLabel, bool *checkValue, bool *_retval); \
  nsresult PromptAuthBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, nsIChannel *aChannel, uint32_t level, nsIAuthInformation *authInfo, const char16_t * checkboxLabel, bool *checkValue, bool *_retval); \
  nsresult AsyncPromptAuth(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, nsIChannel *aChannel, uint32_t level, nsIAuthInformation *authInfo, const char16_t * checkboxLabel, bool checkValue, ::mozilla::dom::Promise * * _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPROMPTSERVICE(_to) \
  NS_IMETHOD Alert(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText) override { return _to Alert(aParent, aDialogTitle, aText); } \
  NS_IMETHOD AlertBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText) override { return _to AlertBC(aBrowsingContext, modalType, aDialogTitle, aText); } \
  NS_IMETHOD AsyncAlert(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, ::mozilla::dom::Promise * * _retval) override { return _to AsyncAlert(aBrowsingContext, modalType, aDialogTitle, aText, _retval); } \
  NS_IMETHOD AlertCheck(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aCheckMsg, bool *aCheckState) override { return _to AlertCheck(aParent, aDialogTitle, aText, aCheckMsg, aCheckState); } \
  NS_IMETHOD AlertCheckBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aCheckMsg, bool *aCheckState) override { return _to AlertCheckBC(aBrowsingContext, modalType, aDialogTitle, aText, aCheckMsg, aCheckState); } \
  NS_IMETHOD AsyncAlertCheck(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aCheckMsg, bool aCheckState, ::mozilla::dom::Promise * * _retval) override { return _to AsyncAlertCheck(aBrowsingContext, modalType, aDialogTitle, aText, aCheckMsg, aCheckState, _retval); } \
  NS_IMETHOD Confirm(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, bool *_retval) override { return _to Confirm(aParent, aDialogTitle, aText, _retval); } \
  NS_IMETHOD ConfirmBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, bool *_retval) override { return _to ConfirmBC(aBrowsingContext, modalType, aDialogTitle, aText, _retval); } \
  NS_IMETHOD AsyncConfirm(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, ::mozilla::dom::Promise * * _retval) override { return _to AsyncConfirm(aBrowsingContext, modalType, aDialogTitle, aText, _retval); } \
  NS_IMETHOD ConfirmCheck(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) override { return _to ConfirmCheck(aParent, aDialogTitle, aText, aCheckMsg, aCheckState, _retval); } \
  NS_IMETHOD ConfirmCheckBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) override { return _to ConfirmCheckBC(aBrowsingContext, modalType, aDialogTitle, aText, aCheckMsg, aCheckState, _retval); } \
  NS_IMETHOD AsyncConfirmCheck(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aCheckMsg, bool aCheckState, ::mozilla::dom::Promise * * _retval) override { return _to AsyncConfirmCheck(aBrowsingContext, modalType, aDialogTitle, aText, aCheckMsg, aCheckState, _retval); } \
  NS_IMETHOD ConfirmEx(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, uint32_t aButtonFlags, const char16_t * aButton0Title, const char16_t * aButton1Title, const char16_t * aButton2Title, const char16_t * aCheckMsg, bool *aCheckState, int32_t *_retval) override { return _to ConfirmEx(aParent, aDialogTitle, aText, aButtonFlags, aButton0Title, aButton1Title, aButton2Title, aCheckMsg, aCheckState, _retval); } \
  NS_IMETHOD ConfirmExBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, uint32_t aButtonFlags, const char16_t * aButton0Title, const char16_t * aButton1Title, const char16_t * aButton2Title, const char16_t * aCheckMsg, bool *aCheckState, int32_t *_retval) override { return _to ConfirmExBC(aBrowsingContext, modalType, aDialogTitle, aText, aButtonFlags, aButton0Title, aButton1Title, aButton2Title, aCheckMsg, aCheckState, _retval); } \
  NS_IMETHOD AsyncConfirmEx(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, uint32_t aButtonFlags, const char16_t * aButton0Title, const char16_t * aButton1Title, const char16_t * aButton2Title, const char16_t * aCheckMsg, bool aCheckState, JS::HandleValue aExtraArgs, ::mozilla::dom::Promise * * _retval) override { return _to AsyncConfirmEx(aBrowsingContext, modalType, aDialogTitle, aText, aButtonFlags, aButton0Title, aButton1Title, aButton2Title, aCheckMsg, aCheckState, aExtraArgs, _retval); } \
  NS_IMETHOD Prompt(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, char16_t * *aValue, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) override { return _to Prompt(aParent, aDialogTitle, aText, aValue, aCheckMsg, aCheckState, _retval); } \
  NS_IMETHOD PromptBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, char16_t * *aValue, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) override { return _to PromptBC(aBrowsingContext, modalType, aDialogTitle, aText, aValue, aCheckMsg, aCheckState, _retval); } \
  NS_IMETHOD AsyncPrompt(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aValue, const char16_t * aCheckMsg, bool aCheckState, ::mozilla::dom::Promise * * _retval) override { return _to AsyncPrompt(aBrowsingContext, modalType, aDialogTitle, aText, aValue, aCheckMsg, aCheckState, _retval); } \
  NS_IMETHOD PromptUsernameAndPassword(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, char16_t * *aUsername, char16_t * *aPassword, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) override { return _to PromptUsernameAndPassword(aParent, aDialogTitle, aText, aUsername, aPassword, aCheckMsg, aCheckState, _retval); } \
  NS_IMETHOD PromptUsernameAndPasswordBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, char16_t * *aUsername, char16_t * *aPassword, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) override { return _to PromptUsernameAndPasswordBC(aBrowsingContext, modalType, aDialogTitle, aText, aUsername, aPassword, aCheckMsg, aCheckState, _retval); } \
  NS_IMETHOD AsyncPromptUsernameAndPassword(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aUsername, const char16_t * aPassword, const char16_t * aCheckMsg, bool aCheckState, ::mozilla::dom::Promise * * _retval) override { return _to AsyncPromptUsernameAndPassword(aBrowsingContext, modalType, aDialogTitle, aText, aUsername, aPassword, aCheckMsg, aCheckState, _retval); } \
  NS_IMETHOD PromptPassword(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, char16_t * *aPassword, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) override { return _to PromptPassword(aParent, aDialogTitle, aText, aPassword, aCheckMsg, aCheckState, _retval); } \
  NS_IMETHOD PromptPasswordBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, char16_t * *aPassword, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) override { return _to PromptPasswordBC(aBrowsingContext, modalType, aDialogTitle, aText, aPassword, aCheckMsg, aCheckState, _retval); } \
  NS_IMETHOD AsyncPromptPassword(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aPassword, const char16_t * aCheckMsg, bool aCheckState, ::mozilla::dom::Promise * * _retval) override { return _to AsyncPromptPassword(aBrowsingContext, modalType, aDialogTitle, aText, aPassword, aCheckMsg, aCheckState, _retval); } \
  NS_IMETHOD Select(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, const nsTArray<nsString >& aSelectList, int32_t *aOutSelection, bool *_retval) override { return _to Select(aParent, aDialogTitle, aText, aSelectList, aOutSelection, _retval); } \
  NS_IMETHOD SelectBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const nsTArray<nsString >& aSelectList, int32_t *aOutSelection, bool *_retval) override { return _to SelectBC(aBrowsingContext, modalType, aDialogTitle, aText, aSelectList, aOutSelection, _retval); } \
  NS_IMETHOD AsyncSelect(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const nsTArray<nsString >& aSelectList, ::mozilla::dom::Promise * * _retval) override { return _to AsyncSelect(aBrowsingContext, modalType, aDialogTitle, aText, aSelectList, _retval); } \
  NS_IMETHOD PromptAuth(mozIDOMWindowProxy *aParent, nsIChannel *aChannel, uint32_t level, nsIAuthInformation *authInfo, const char16_t * checkboxLabel, bool *checkValue, bool *_retval) override { return _to PromptAuth(aParent, aChannel, level, authInfo, checkboxLabel, checkValue, _retval); } \
  NS_IMETHOD PromptAuthBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, nsIChannel *aChannel, uint32_t level, nsIAuthInformation *authInfo, const char16_t * checkboxLabel, bool *checkValue, bool *_retval) override { return _to PromptAuthBC(aBrowsingContext, modalType, aChannel, level, authInfo, checkboxLabel, checkValue, _retval); } \
  NS_IMETHOD AsyncPromptAuth(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, nsIChannel *aChannel, uint32_t level, nsIAuthInformation *authInfo, const char16_t * checkboxLabel, bool checkValue, ::mozilla::dom::Promise * * _retval) override { return _to AsyncPromptAuth(aBrowsingContext, modalType, aChannel, level, authInfo, checkboxLabel, checkValue, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPROMPTSERVICE(_to) \
  NS_IMETHOD Alert(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Alert(aParent, aDialogTitle, aText); } \
  NS_IMETHOD AlertBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AlertBC(aBrowsingContext, modalType, aDialogTitle, aText); } \
  NS_IMETHOD AsyncAlert(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncAlert(aBrowsingContext, modalType, aDialogTitle, aText, _retval); } \
  NS_IMETHOD AlertCheck(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aCheckMsg, bool *aCheckState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AlertCheck(aParent, aDialogTitle, aText, aCheckMsg, aCheckState); } \
  NS_IMETHOD AlertCheckBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aCheckMsg, bool *aCheckState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AlertCheckBC(aBrowsingContext, modalType, aDialogTitle, aText, aCheckMsg, aCheckState); } \
  NS_IMETHOD AsyncAlertCheck(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aCheckMsg, bool aCheckState, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncAlertCheck(aBrowsingContext, modalType, aDialogTitle, aText, aCheckMsg, aCheckState, _retval); } \
  NS_IMETHOD Confirm(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Confirm(aParent, aDialogTitle, aText, _retval); } \
  NS_IMETHOD ConfirmBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ConfirmBC(aBrowsingContext, modalType, aDialogTitle, aText, _retval); } \
  NS_IMETHOD AsyncConfirm(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncConfirm(aBrowsingContext, modalType, aDialogTitle, aText, _retval); } \
  NS_IMETHOD ConfirmCheck(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ConfirmCheck(aParent, aDialogTitle, aText, aCheckMsg, aCheckState, _retval); } \
  NS_IMETHOD ConfirmCheckBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ConfirmCheckBC(aBrowsingContext, modalType, aDialogTitle, aText, aCheckMsg, aCheckState, _retval); } \
  NS_IMETHOD AsyncConfirmCheck(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aCheckMsg, bool aCheckState, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncConfirmCheck(aBrowsingContext, modalType, aDialogTitle, aText, aCheckMsg, aCheckState, _retval); } \
  NS_IMETHOD ConfirmEx(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, uint32_t aButtonFlags, const char16_t * aButton0Title, const char16_t * aButton1Title, const char16_t * aButton2Title, const char16_t * aCheckMsg, bool *aCheckState, int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ConfirmEx(aParent, aDialogTitle, aText, aButtonFlags, aButton0Title, aButton1Title, aButton2Title, aCheckMsg, aCheckState, _retval); } \
  NS_IMETHOD ConfirmExBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, uint32_t aButtonFlags, const char16_t * aButton0Title, const char16_t * aButton1Title, const char16_t * aButton2Title, const char16_t * aCheckMsg, bool *aCheckState, int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ConfirmExBC(aBrowsingContext, modalType, aDialogTitle, aText, aButtonFlags, aButton0Title, aButton1Title, aButton2Title, aCheckMsg, aCheckState, _retval); } \
  NS_IMETHOD AsyncConfirmEx(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, uint32_t aButtonFlags, const char16_t * aButton0Title, const char16_t * aButton1Title, const char16_t * aButton2Title, const char16_t * aCheckMsg, bool aCheckState, JS::HandleValue aExtraArgs, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncConfirmEx(aBrowsingContext, modalType, aDialogTitle, aText, aButtonFlags, aButton0Title, aButton1Title, aButton2Title, aCheckMsg, aCheckState, aExtraArgs, _retval); } \
  NS_IMETHOD Prompt(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, char16_t * *aValue, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Prompt(aParent, aDialogTitle, aText, aValue, aCheckMsg, aCheckState, _retval); } \
  NS_IMETHOD PromptBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, char16_t * *aValue, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PromptBC(aBrowsingContext, modalType, aDialogTitle, aText, aValue, aCheckMsg, aCheckState, _retval); } \
  NS_IMETHOD AsyncPrompt(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aValue, const char16_t * aCheckMsg, bool aCheckState, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncPrompt(aBrowsingContext, modalType, aDialogTitle, aText, aValue, aCheckMsg, aCheckState, _retval); } \
  NS_IMETHOD PromptUsernameAndPassword(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, char16_t * *aUsername, char16_t * *aPassword, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PromptUsernameAndPassword(aParent, aDialogTitle, aText, aUsername, aPassword, aCheckMsg, aCheckState, _retval); } \
  NS_IMETHOD PromptUsernameAndPasswordBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, char16_t * *aUsername, char16_t * *aPassword, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PromptUsernameAndPasswordBC(aBrowsingContext, modalType, aDialogTitle, aText, aUsername, aPassword, aCheckMsg, aCheckState, _retval); } \
  NS_IMETHOD AsyncPromptUsernameAndPassword(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aUsername, const char16_t * aPassword, const char16_t * aCheckMsg, bool aCheckState, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncPromptUsernameAndPassword(aBrowsingContext, modalType, aDialogTitle, aText, aUsername, aPassword, aCheckMsg, aCheckState, _retval); } \
  NS_IMETHOD PromptPassword(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, char16_t * *aPassword, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PromptPassword(aParent, aDialogTitle, aText, aPassword, aCheckMsg, aCheckState, _retval); } \
  NS_IMETHOD PromptPasswordBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, char16_t * *aPassword, const char16_t * aCheckMsg, bool *aCheckState, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PromptPasswordBC(aBrowsingContext, modalType, aDialogTitle, aText, aPassword, aCheckMsg, aCheckState, _retval); } \
  NS_IMETHOD AsyncPromptPassword(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const char16_t * aPassword, const char16_t * aCheckMsg, bool aCheckState, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncPromptPassword(aBrowsingContext, modalType, aDialogTitle, aText, aPassword, aCheckMsg, aCheckState, _retval); } \
  NS_IMETHOD Select(mozIDOMWindowProxy *aParent, const char16_t * aDialogTitle, const char16_t * aText, const nsTArray<nsString >& aSelectList, int32_t *aOutSelection, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Select(aParent, aDialogTitle, aText, aSelectList, aOutSelection, _retval); } \
  NS_IMETHOD SelectBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const nsTArray<nsString >& aSelectList, int32_t *aOutSelection, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SelectBC(aBrowsingContext, modalType, aDialogTitle, aText, aSelectList, aOutSelection, _retval); } \
  NS_IMETHOD AsyncSelect(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, const char16_t * aDialogTitle, const char16_t * aText, const nsTArray<nsString >& aSelectList, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncSelect(aBrowsingContext, modalType, aDialogTitle, aText, aSelectList, _retval); } \
  NS_IMETHOD PromptAuth(mozIDOMWindowProxy *aParent, nsIChannel *aChannel, uint32_t level, nsIAuthInformation *authInfo, const char16_t * checkboxLabel, bool *checkValue, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PromptAuth(aParent, aChannel, level, authInfo, checkboxLabel, checkValue, _retval); } \
  NS_IMETHOD PromptAuthBC(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, nsIChannel *aChannel, uint32_t level, nsIAuthInformation *authInfo, const char16_t * checkboxLabel, bool *checkValue, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PromptAuthBC(aBrowsingContext, modalType, aChannel, level, authInfo, checkboxLabel, checkValue, _retval); } \
  NS_IMETHOD AsyncPromptAuth(mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t modalType, nsIChannel *aChannel, uint32_t level, nsIAuthInformation *authInfo, const char16_t * checkboxLabel, bool checkValue, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncPromptAuth(aBrowsingContext, modalType, aChannel, level, authInfo, checkboxLabel, checkValue, _retval); } 


#endif /* __gen_nsIPromptService_h__ */
