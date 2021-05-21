/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/passwordmgr/nsILoginInfo.idl
 */

#ifndef __gen_nsILoginInfo_h__
#define __gen_nsILoginInfo_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsILoginInfo */
#define NS_ILOGININFO_IID_STR "c41b7dff-6b9b-42fe-b78d-113051facb05"

#define NS_ILOGININFO_IID \
  {0xc41b7dff, 0x6b9b, 0x42fe, \
    { 0xb7, 0x8d, 0x11, 0x30, 0x51, 0xfa, 0xcb, 0x05 }}

class NS_NO_VTABLE nsILoginInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ILOGININFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsILoginInfo;

  /* readonly attribute AString displayOrigin; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDisplayOrigin(nsAString& aDisplayOrigin) = 0;

  /* attribute AString origin; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetOrigin(nsAString& aOrigin) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetOrigin(const nsAString& aOrigin) = 0;

  /* attribute AString hostname; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHostname(nsAString& aHostname) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetHostname(const nsAString& aHostname) = 0;

  /* attribute AString formActionOrigin; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFormActionOrigin(nsAString& aFormActionOrigin) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetFormActionOrigin(const nsAString& aFormActionOrigin) = 0;

  /* attribute AString formSubmitURL; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFormSubmitURL(nsAString& aFormSubmitURL) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetFormSubmitURL(const nsAString& aFormSubmitURL) = 0;

  /* attribute AString httpRealm; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHttpRealm(nsAString& aHttpRealm) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetHttpRealm(const nsAString& aHttpRealm) = 0;

  /* attribute AString username; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUsername(nsAString& aUsername) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetUsername(const nsAString& aUsername) = 0;

  /* attribute AString usernameField; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUsernameField(nsAString& aUsernameField) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetUsernameField(const nsAString& aUsernameField) = 0;

  /* attribute AString password; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPassword(nsAString& aPassword) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPassword(const nsAString& aPassword) = 0;

  /* attribute AString passwordField; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPasswordField(nsAString& aPasswordField) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPasswordField(const nsAString& aPasswordField) = 0;

  /* void init (in AString aOrigin, in AString aFormActionOrigin, in AString aHttpRealm, in AString aUsername, in AString aPassword, [optional] in AString aUsernameField, [optional] in AString aPasswordField); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(const nsAString& aOrigin, const nsAString& aFormActionOrigin, const nsAString& aHttpRealm, const nsAString& aUsername, const nsAString& aPassword, const nsAString& aUsernameField, const nsAString& aPasswordField) = 0;

  /* boolean equals (in nsILoginInfo aLoginInfo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Equals(nsILoginInfo *aLoginInfo, bool *_retval) = 0;

  /* boolean matches (in nsILoginInfo aLoginInfo, in boolean ignorePassword); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Matches(nsILoginInfo *aLoginInfo, bool ignorePassword, bool *_retval) = 0;

  /* nsILoginInfo clone (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Clone(nsILoginInfo **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsILoginInfo, NS_ILOGININFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSILOGININFO \
  NS_IMETHOD GetDisplayOrigin(nsAString& aDisplayOrigin) override; \
  NS_IMETHOD GetOrigin(nsAString& aOrigin) override; \
  NS_IMETHOD SetOrigin(const nsAString& aOrigin) override; \
  NS_IMETHOD GetHostname(nsAString& aHostname) override; \
  NS_IMETHOD SetHostname(const nsAString& aHostname) override; \
  NS_IMETHOD GetFormActionOrigin(nsAString& aFormActionOrigin) override; \
  NS_IMETHOD SetFormActionOrigin(const nsAString& aFormActionOrigin) override; \
  NS_IMETHOD GetFormSubmitURL(nsAString& aFormSubmitURL) override; \
  NS_IMETHOD SetFormSubmitURL(const nsAString& aFormSubmitURL) override; \
  NS_IMETHOD GetHttpRealm(nsAString& aHttpRealm) override; \
  NS_IMETHOD SetHttpRealm(const nsAString& aHttpRealm) override; \
  NS_IMETHOD GetUsername(nsAString& aUsername) override; \
  NS_IMETHOD SetUsername(const nsAString& aUsername) override; \
  NS_IMETHOD GetUsernameField(nsAString& aUsernameField) override; \
  NS_IMETHOD SetUsernameField(const nsAString& aUsernameField) override; \
  NS_IMETHOD GetPassword(nsAString& aPassword) override; \
  NS_IMETHOD SetPassword(const nsAString& aPassword) override; \
  NS_IMETHOD GetPasswordField(nsAString& aPasswordField) override; \
  NS_IMETHOD SetPasswordField(const nsAString& aPasswordField) override; \
  NS_IMETHOD Init(const nsAString& aOrigin, const nsAString& aFormActionOrigin, const nsAString& aHttpRealm, const nsAString& aUsername, const nsAString& aPassword, const nsAString& aUsernameField, const nsAString& aPasswordField) override; \
  NS_IMETHOD Equals(nsILoginInfo *aLoginInfo, bool *_retval) override; \
  NS_IMETHOD Matches(nsILoginInfo *aLoginInfo, bool ignorePassword, bool *_retval) override; \
  NS_IMETHOD Clone(nsILoginInfo **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSILOGININFO \
  nsresult GetDisplayOrigin(nsAString& aDisplayOrigin); \
  nsresult GetOrigin(nsAString& aOrigin); \
  nsresult SetOrigin(const nsAString& aOrigin); \
  nsresult GetHostname(nsAString& aHostname); \
  nsresult SetHostname(const nsAString& aHostname); \
  nsresult GetFormActionOrigin(nsAString& aFormActionOrigin); \
  nsresult SetFormActionOrigin(const nsAString& aFormActionOrigin); \
  nsresult GetFormSubmitURL(nsAString& aFormSubmitURL); \
  nsresult SetFormSubmitURL(const nsAString& aFormSubmitURL); \
  nsresult GetHttpRealm(nsAString& aHttpRealm); \
  nsresult SetHttpRealm(const nsAString& aHttpRealm); \
  nsresult GetUsername(nsAString& aUsername); \
  nsresult SetUsername(const nsAString& aUsername); \
  nsresult GetUsernameField(nsAString& aUsernameField); \
  nsresult SetUsernameField(const nsAString& aUsernameField); \
  nsresult GetPassword(nsAString& aPassword); \
  nsresult SetPassword(const nsAString& aPassword); \
  nsresult GetPasswordField(nsAString& aPasswordField); \
  nsresult SetPasswordField(const nsAString& aPasswordField); \
  nsresult Init(const nsAString& aOrigin, const nsAString& aFormActionOrigin, const nsAString& aHttpRealm, const nsAString& aUsername, const nsAString& aPassword, const nsAString& aUsernameField, const nsAString& aPasswordField); \
  nsresult Equals(nsILoginInfo *aLoginInfo, bool *_retval); \
  nsresult Matches(nsILoginInfo *aLoginInfo, bool ignorePassword, bool *_retval); \
  nsresult Clone(nsILoginInfo **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSILOGININFO(_to) \
  NS_IMETHOD GetDisplayOrigin(nsAString& aDisplayOrigin) override { return _to GetDisplayOrigin(aDisplayOrigin); } \
  NS_IMETHOD GetOrigin(nsAString& aOrigin) override { return _to GetOrigin(aOrigin); } \
  NS_IMETHOD SetOrigin(const nsAString& aOrigin) override { return _to SetOrigin(aOrigin); } \
  NS_IMETHOD GetHostname(nsAString& aHostname) override { return _to GetHostname(aHostname); } \
  NS_IMETHOD SetHostname(const nsAString& aHostname) override { return _to SetHostname(aHostname); } \
  NS_IMETHOD GetFormActionOrigin(nsAString& aFormActionOrigin) override { return _to GetFormActionOrigin(aFormActionOrigin); } \
  NS_IMETHOD SetFormActionOrigin(const nsAString& aFormActionOrigin) override { return _to SetFormActionOrigin(aFormActionOrigin); } \
  NS_IMETHOD GetFormSubmitURL(nsAString& aFormSubmitURL) override { return _to GetFormSubmitURL(aFormSubmitURL); } \
  NS_IMETHOD SetFormSubmitURL(const nsAString& aFormSubmitURL) override { return _to SetFormSubmitURL(aFormSubmitURL); } \
  NS_IMETHOD GetHttpRealm(nsAString& aHttpRealm) override { return _to GetHttpRealm(aHttpRealm); } \
  NS_IMETHOD SetHttpRealm(const nsAString& aHttpRealm) override { return _to SetHttpRealm(aHttpRealm); } \
  NS_IMETHOD GetUsername(nsAString& aUsername) override { return _to GetUsername(aUsername); } \
  NS_IMETHOD SetUsername(const nsAString& aUsername) override { return _to SetUsername(aUsername); } \
  NS_IMETHOD GetUsernameField(nsAString& aUsernameField) override { return _to GetUsernameField(aUsernameField); } \
  NS_IMETHOD SetUsernameField(const nsAString& aUsernameField) override { return _to SetUsernameField(aUsernameField); } \
  NS_IMETHOD GetPassword(nsAString& aPassword) override { return _to GetPassword(aPassword); } \
  NS_IMETHOD SetPassword(const nsAString& aPassword) override { return _to SetPassword(aPassword); } \
  NS_IMETHOD GetPasswordField(nsAString& aPasswordField) override { return _to GetPasswordField(aPasswordField); } \
  NS_IMETHOD SetPasswordField(const nsAString& aPasswordField) override { return _to SetPasswordField(aPasswordField); } \
  NS_IMETHOD Init(const nsAString& aOrigin, const nsAString& aFormActionOrigin, const nsAString& aHttpRealm, const nsAString& aUsername, const nsAString& aPassword, const nsAString& aUsernameField, const nsAString& aPasswordField) override { return _to Init(aOrigin, aFormActionOrigin, aHttpRealm, aUsername, aPassword, aUsernameField, aPasswordField); } \
  NS_IMETHOD Equals(nsILoginInfo *aLoginInfo, bool *_retval) override { return _to Equals(aLoginInfo, _retval); } \
  NS_IMETHOD Matches(nsILoginInfo *aLoginInfo, bool ignorePassword, bool *_retval) override { return _to Matches(aLoginInfo, ignorePassword, _retval); } \
  NS_IMETHOD Clone(nsILoginInfo **_retval) override { return _to Clone(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSILOGININFO(_to) \
  NS_IMETHOD GetDisplayOrigin(nsAString& aDisplayOrigin) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDisplayOrigin(aDisplayOrigin); } \
  NS_IMETHOD GetOrigin(nsAString& aOrigin) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOrigin(aOrigin); } \
  NS_IMETHOD SetOrigin(const nsAString& aOrigin) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetOrigin(aOrigin); } \
  NS_IMETHOD GetHostname(nsAString& aHostname) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHostname(aHostname); } \
  NS_IMETHOD SetHostname(const nsAString& aHostname) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetHostname(aHostname); } \
  NS_IMETHOD GetFormActionOrigin(nsAString& aFormActionOrigin) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFormActionOrigin(aFormActionOrigin); } \
  NS_IMETHOD SetFormActionOrigin(const nsAString& aFormActionOrigin) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFormActionOrigin(aFormActionOrigin); } \
  NS_IMETHOD GetFormSubmitURL(nsAString& aFormSubmitURL) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFormSubmitURL(aFormSubmitURL); } \
  NS_IMETHOD SetFormSubmitURL(const nsAString& aFormSubmitURL) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFormSubmitURL(aFormSubmitURL); } \
  NS_IMETHOD GetHttpRealm(nsAString& aHttpRealm) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHttpRealm(aHttpRealm); } \
  NS_IMETHOD SetHttpRealm(const nsAString& aHttpRealm) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetHttpRealm(aHttpRealm); } \
  NS_IMETHOD GetUsername(nsAString& aUsername) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUsername(aUsername); } \
  NS_IMETHOD SetUsername(const nsAString& aUsername) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetUsername(aUsername); } \
  NS_IMETHOD GetUsernameField(nsAString& aUsernameField) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUsernameField(aUsernameField); } \
  NS_IMETHOD SetUsernameField(const nsAString& aUsernameField) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetUsernameField(aUsernameField); } \
  NS_IMETHOD GetPassword(nsAString& aPassword) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPassword(aPassword); } \
  NS_IMETHOD SetPassword(const nsAString& aPassword) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPassword(aPassword); } \
  NS_IMETHOD GetPasswordField(nsAString& aPasswordField) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPasswordField(aPasswordField); } \
  NS_IMETHOD SetPasswordField(const nsAString& aPasswordField) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPasswordField(aPasswordField); } \
  NS_IMETHOD Init(const nsAString& aOrigin, const nsAString& aFormActionOrigin, const nsAString& aHttpRealm, const nsAString& aUsername, const nsAString& aPassword, const nsAString& aUsernameField, const nsAString& aPasswordField) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aOrigin, aFormActionOrigin, aHttpRealm, aUsername, aPassword, aUsernameField, aPasswordField); } \
  NS_IMETHOD Equals(nsILoginInfo *aLoginInfo, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Equals(aLoginInfo, _retval); } \
  NS_IMETHOD Matches(nsILoginInfo *aLoginInfo, bool ignorePassword, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Matches(aLoginInfo, ignorePassword, _retval); } \
  NS_IMETHOD Clone(nsILoginInfo **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Clone(_retval); } 


#define NS_LOGININFO_CONTRACTID "@mozilla.org/login-manager/loginInfo;1"

#endif /* __gen_nsILoginInfo_h__ */
