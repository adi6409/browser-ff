/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIPK11Token.idl
 */

#ifndef __gen_nsIPK11Token_h__
#define __gen_nsIPK11Token_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIPK11Token */
#define NS_IPK11TOKEN_IID_STR "51191434-1dd2-11b2-a17c-e49c4e99a4e3"

#define NS_IPK11TOKEN_IID \
  {0x51191434, 0x1dd2, 0x11b2, \
    { 0xa1, 0x7c, 0xe4, 0x9c, 0x4e, 0x99, 0xa4, 0xe3 }}

class NS_NO_VTABLE nsIPK11Token : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPK11TOKEN_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPK11Token;

  /* [must_use] readonly attribute AUTF8String tokenName; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetTokenName(nsACString& aTokenName) = 0;

  /* [must_use] readonly attribute boolean isInternalKeyToken; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetIsInternalKeyToken(bool *aIsInternalKeyToken) = 0;

  /* [must_use] readonly attribute AUTF8String tokenManID; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetTokenManID(nsACString& aTokenManID) = 0;

  /* [must_use] readonly attribute AUTF8String tokenHWVersion; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetTokenHWVersion(nsACString& aTokenHWVersion) = 0;

  /* [must_use] readonly attribute AUTF8String tokenFWVersion; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetTokenFWVersion(nsACString& aTokenFWVersion) = 0;

  /* [must_use] readonly attribute AUTF8String tokenSerialNumber; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetTokenSerialNumber(nsACString& aTokenSerialNumber) = 0;

  /* [must_use] boolean isLoggedIn (); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD IsLoggedIn(bool *_retval) = 0;

  /* [must_use] void login (in boolean force); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD Login(bool force) = 0;

  /* [must_use] void logoutSimple (); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD LogoutSimple(void) = 0;

  /* [must_use] void logoutAndDropAuthenticatedResources (); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD LogoutAndDropAuthenticatedResources(void) = 0;

  /* [must_use] boolean needsLogin (); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD NeedsLogin(bool *_retval) = 0;

  /* [must_use] readonly attribute boolean needsUserInit; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetNeedsUserInit(bool *aNeedsUserInit) = 0;

  /* [must_use] void reset (); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD Reset(void) = 0;

  /* [must_use] boolean checkPassword (in AUTF8String password); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD CheckPassword(const nsACString& password, bool *_retval) = 0;

  /* [must_use] void initPassword (in AUTF8String initialPassword); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD InitPassword(const nsACString& initialPassword) = 0;

  /* [must_use] void changePassword (in AUTF8String oldPassword, in AUTF8String newPassword); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD ChangePassword(const nsACString& oldPassword, const nsACString& newPassword) = 0;

  /* [must_use] readonly attribute boolean hasPassword; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetHasPassword(bool *aHasPassword) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPK11Token, NS_IPK11TOKEN_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPK11TOKEN \
  [[nodiscard]] NS_IMETHOD GetTokenName(nsACString& aTokenName) override; \
  [[nodiscard]] NS_IMETHOD GetIsInternalKeyToken(bool *aIsInternalKeyToken) override; \
  [[nodiscard]] NS_IMETHOD GetTokenManID(nsACString& aTokenManID) override; \
  [[nodiscard]] NS_IMETHOD GetTokenHWVersion(nsACString& aTokenHWVersion) override; \
  [[nodiscard]] NS_IMETHOD GetTokenFWVersion(nsACString& aTokenFWVersion) override; \
  [[nodiscard]] NS_IMETHOD GetTokenSerialNumber(nsACString& aTokenSerialNumber) override; \
  [[nodiscard]] NS_IMETHOD IsLoggedIn(bool *_retval) override; \
  [[nodiscard]] NS_IMETHOD Login(bool force) override; \
  [[nodiscard]] NS_IMETHOD LogoutSimple(void) override; \
  [[nodiscard]] NS_IMETHOD LogoutAndDropAuthenticatedResources(void) override; \
  [[nodiscard]] NS_IMETHOD NeedsLogin(bool *_retval) override; \
  [[nodiscard]] NS_IMETHOD GetNeedsUserInit(bool *aNeedsUserInit) override; \
  [[nodiscard]] NS_IMETHOD Reset(void) override; \
  [[nodiscard]] NS_IMETHOD CheckPassword(const nsACString& password, bool *_retval) override; \
  [[nodiscard]] NS_IMETHOD InitPassword(const nsACString& initialPassword) override; \
  [[nodiscard]] NS_IMETHOD ChangePassword(const nsACString& oldPassword, const nsACString& newPassword) override; \
  [[nodiscard]] NS_IMETHOD GetHasPassword(bool *aHasPassword) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPK11TOKEN \
  [[nodiscard]] nsresult GetTokenName(nsACString& aTokenName); \
  [[nodiscard]] nsresult GetIsInternalKeyToken(bool *aIsInternalKeyToken); \
  [[nodiscard]] nsresult GetTokenManID(nsACString& aTokenManID); \
  [[nodiscard]] nsresult GetTokenHWVersion(nsACString& aTokenHWVersion); \
  [[nodiscard]] nsresult GetTokenFWVersion(nsACString& aTokenFWVersion); \
  [[nodiscard]] nsresult GetTokenSerialNumber(nsACString& aTokenSerialNumber); \
  [[nodiscard]] nsresult IsLoggedIn(bool *_retval); \
  [[nodiscard]] nsresult Login(bool force); \
  [[nodiscard]] nsresult LogoutSimple(void); \
  [[nodiscard]] nsresult LogoutAndDropAuthenticatedResources(void); \
  [[nodiscard]] nsresult NeedsLogin(bool *_retval); \
  [[nodiscard]] nsresult GetNeedsUserInit(bool *aNeedsUserInit); \
  [[nodiscard]] nsresult Reset(void); \
  [[nodiscard]] nsresult CheckPassword(const nsACString& password, bool *_retval); \
  [[nodiscard]] nsresult InitPassword(const nsACString& initialPassword); \
  [[nodiscard]] nsresult ChangePassword(const nsACString& oldPassword, const nsACString& newPassword); \
  [[nodiscard]] nsresult GetHasPassword(bool *aHasPassword); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPK11TOKEN(_to) \
  [[nodiscard]] NS_IMETHOD GetTokenName(nsACString& aTokenName) override { return _to GetTokenName(aTokenName); } \
  [[nodiscard]] NS_IMETHOD GetIsInternalKeyToken(bool *aIsInternalKeyToken) override { return _to GetIsInternalKeyToken(aIsInternalKeyToken); } \
  [[nodiscard]] NS_IMETHOD GetTokenManID(nsACString& aTokenManID) override { return _to GetTokenManID(aTokenManID); } \
  [[nodiscard]] NS_IMETHOD GetTokenHWVersion(nsACString& aTokenHWVersion) override { return _to GetTokenHWVersion(aTokenHWVersion); } \
  [[nodiscard]] NS_IMETHOD GetTokenFWVersion(nsACString& aTokenFWVersion) override { return _to GetTokenFWVersion(aTokenFWVersion); } \
  [[nodiscard]] NS_IMETHOD GetTokenSerialNumber(nsACString& aTokenSerialNumber) override { return _to GetTokenSerialNumber(aTokenSerialNumber); } \
  [[nodiscard]] NS_IMETHOD IsLoggedIn(bool *_retval) override { return _to IsLoggedIn(_retval); } \
  [[nodiscard]] NS_IMETHOD Login(bool force) override { return _to Login(force); } \
  [[nodiscard]] NS_IMETHOD LogoutSimple(void) override { return _to LogoutSimple(); } \
  [[nodiscard]] NS_IMETHOD LogoutAndDropAuthenticatedResources(void) override { return _to LogoutAndDropAuthenticatedResources(); } \
  [[nodiscard]] NS_IMETHOD NeedsLogin(bool *_retval) override { return _to NeedsLogin(_retval); } \
  [[nodiscard]] NS_IMETHOD GetNeedsUserInit(bool *aNeedsUserInit) override { return _to GetNeedsUserInit(aNeedsUserInit); } \
  [[nodiscard]] NS_IMETHOD Reset(void) override { return _to Reset(); } \
  [[nodiscard]] NS_IMETHOD CheckPassword(const nsACString& password, bool *_retval) override { return _to CheckPassword(password, _retval); } \
  [[nodiscard]] NS_IMETHOD InitPassword(const nsACString& initialPassword) override { return _to InitPassword(initialPassword); } \
  [[nodiscard]] NS_IMETHOD ChangePassword(const nsACString& oldPassword, const nsACString& newPassword) override { return _to ChangePassword(oldPassword, newPassword); } \
  [[nodiscard]] NS_IMETHOD GetHasPassword(bool *aHasPassword) override { return _to GetHasPassword(aHasPassword); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPK11TOKEN(_to) \
  [[nodiscard]] NS_IMETHOD GetTokenName(nsACString& aTokenName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTokenName(aTokenName); } \
  [[nodiscard]] NS_IMETHOD GetIsInternalKeyToken(bool *aIsInternalKeyToken) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsInternalKeyToken(aIsInternalKeyToken); } \
  [[nodiscard]] NS_IMETHOD GetTokenManID(nsACString& aTokenManID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTokenManID(aTokenManID); } \
  [[nodiscard]] NS_IMETHOD GetTokenHWVersion(nsACString& aTokenHWVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTokenHWVersion(aTokenHWVersion); } \
  [[nodiscard]] NS_IMETHOD GetTokenFWVersion(nsACString& aTokenFWVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTokenFWVersion(aTokenFWVersion); } \
  [[nodiscard]] NS_IMETHOD GetTokenSerialNumber(nsACString& aTokenSerialNumber) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTokenSerialNumber(aTokenSerialNumber); } \
  [[nodiscard]] NS_IMETHOD IsLoggedIn(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsLoggedIn(_retval); } \
  [[nodiscard]] NS_IMETHOD Login(bool force) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Login(force); } \
  [[nodiscard]] NS_IMETHOD LogoutSimple(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LogoutSimple(); } \
  [[nodiscard]] NS_IMETHOD LogoutAndDropAuthenticatedResources(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LogoutAndDropAuthenticatedResources(); } \
  [[nodiscard]] NS_IMETHOD NeedsLogin(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NeedsLogin(_retval); } \
  [[nodiscard]] NS_IMETHOD GetNeedsUserInit(bool *aNeedsUserInit) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNeedsUserInit(aNeedsUserInit); } \
  [[nodiscard]] NS_IMETHOD Reset(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Reset(); } \
  [[nodiscard]] NS_IMETHOD CheckPassword(const nsACString& password, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CheckPassword(password, _retval); } \
  [[nodiscard]] NS_IMETHOD InitPassword(const nsACString& initialPassword) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitPassword(initialPassword); } \
  [[nodiscard]] NS_IMETHOD ChangePassword(const nsACString& oldPassword, const nsACString& newPassword) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ChangePassword(oldPassword, newPassword); } \
  [[nodiscard]] NS_IMETHOD GetHasPassword(bool *aHasPassword) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHasPassword(aHasPassword); } 


#endif /* __gen_nsIPK11Token_h__ */
