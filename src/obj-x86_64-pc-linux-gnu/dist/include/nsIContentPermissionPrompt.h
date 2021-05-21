/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIContentPermissionPrompt.idl
 */

#ifndef __gen_nsIContentPermissionPrompt_h__
#define __gen_nsIContentPermissionPrompt_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIPrincipal; /* forward declaration */

class mozIDOMWindow; /* forward declaration */

class nsIArray; /* forward declaration */

namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIContentPermissionType */
#define NS_ICONTENTPERMISSIONTYPE_IID_STR "ef4db3b8-ca9c-4b1d-8f81-fd88ec32af13"

#define NS_ICONTENTPERMISSIONTYPE_IID \
  {0xef4db3b8, 0xca9c, 0x4b1d, \
    { 0x8f, 0x81, 0xfd, 0x88, 0xec, 0x32, 0xaf, 0x13 }}

class NS_NO_VTABLE nsIContentPermissionType : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICONTENTPERMISSIONTYPE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIContentPermissionType;

  /* readonly attribute ACString type; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetType(nsACString& aType) = 0;

  /* readonly attribute nsIArray options; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetOptions(nsIArray **aOptions) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIContentPermissionType, NS_ICONTENTPERMISSIONTYPE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICONTENTPERMISSIONTYPE \
  NS_IMETHOD GetType(nsACString& aType) override; \
  NS_IMETHOD GetOptions(nsIArray **aOptions) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICONTENTPERMISSIONTYPE \
  nsresult GetType(nsACString& aType); \
  nsresult GetOptions(nsIArray **aOptions); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICONTENTPERMISSIONTYPE(_to) \
  NS_IMETHOD GetType(nsACString& aType) override { return _to GetType(aType); } \
  NS_IMETHOD GetOptions(nsIArray **aOptions) override { return _to GetOptions(aOptions); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICONTENTPERMISSIONTYPE(_to) \
  NS_IMETHOD GetType(nsACString& aType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetType(aType); } \
  NS_IMETHOD GetOptions(nsIArray **aOptions) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOptions(aOptions); } 


/* starting interface:    nsIContentPermissionRequest */
#define NS_ICONTENTPERMISSIONREQUEST_IID_STR "875733da-0ac0-4a26-8c76-70a30876be46"

#define NS_ICONTENTPERMISSIONREQUEST_IID \
  {0x875733da, 0x0ac0, 0x4a26, \
    { 0x8c, 0x76, 0x70, 0xa3, 0x08, 0x76, 0xbe, 0x46 }}

class NS_NO_VTABLE nsIContentPermissionRequest : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICONTENTPERMISSIONREQUEST_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIContentPermissionRequest;

  /* readonly attribute nsIArray types; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTypes(nsIArray **aTypes) = 0;

  /* readonly attribute nsIPrincipal principal; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) = 0;

  /* readonly attribute nsIPrincipal topLevelPrincipal; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTopLevelPrincipal(nsIPrincipal **aTopLevelPrincipal) = 0;

  /* readonly attribute mozIDOMWindow window; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetWindow(mozIDOMWindow **aWindow) = 0;

  /* readonly attribute Element element; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetElement(mozilla::dom::Element **aElement) = 0;

  /* readonly attribute boolean isHandlingUserInput; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsHandlingUserInput(bool *aIsHandlingUserInput) = 0;

  /* readonly attribute boolean maybeUnsafePermissionDelegate; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMaybeUnsafePermissionDelegate(bool *aMaybeUnsafePermissionDelegate) = 0;

  /* nsIPrincipal getDelegatePrincipal (in ACString aType); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDelegatePrincipal(const nsACString& aType, nsIPrincipal **_retval) = 0;

  /* [can_run_script] void cancel (); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD Cancel(void) = 0;

  /* [can_run_script] void allow ([optional] in jsval choices); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD Allow(JS::HandleValue choices) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIContentPermissionRequest, NS_ICONTENTPERMISSIONREQUEST_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICONTENTPERMISSIONREQUEST \
  NS_IMETHOD GetTypes(nsIArray **aTypes) override; \
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) override; \
  NS_IMETHOD GetTopLevelPrincipal(nsIPrincipal **aTopLevelPrincipal) override; \
  NS_IMETHOD GetWindow(mozIDOMWindow **aWindow) override; \
  NS_IMETHOD GetElement(mozilla::dom::Element **aElement) override; \
  NS_IMETHOD GetIsHandlingUserInput(bool *aIsHandlingUserInput) override; \
  NS_IMETHOD GetMaybeUnsafePermissionDelegate(bool *aMaybeUnsafePermissionDelegate) override; \
  NS_IMETHOD GetDelegatePrincipal(const nsACString& aType, nsIPrincipal **_retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Cancel(void) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Allow(JS::HandleValue choices) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICONTENTPERMISSIONREQUEST \
  nsresult GetTypes(nsIArray **aTypes); \
  nsresult GetPrincipal(nsIPrincipal **aPrincipal); \
  nsresult GetTopLevelPrincipal(nsIPrincipal **aTopLevelPrincipal); \
  nsresult GetWindow(mozIDOMWindow **aWindow); \
  nsresult GetElement(mozilla::dom::Element **aElement); \
  nsresult GetIsHandlingUserInput(bool *aIsHandlingUserInput); \
  nsresult GetMaybeUnsafePermissionDelegate(bool *aMaybeUnsafePermissionDelegate); \
  nsresult GetDelegatePrincipal(const nsACString& aType, nsIPrincipal **_retval); \
  MOZ_CAN_RUN_SCRIPT nsresult Cancel(void); \
  MOZ_CAN_RUN_SCRIPT nsresult Allow(JS::HandleValue choices); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICONTENTPERMISSIONREQUEST(_to) \
  NS_IMETHOD GetTypes(nsIArray **aTypes) override { return _to GetTypes(aTypes); } \
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) override { return _to GetPrincipal(aPrincipal); } \
  NS_IMETHOD GetTopLevelPrincipal(nsIPrincipal **aTopLevelPrincipal) override { return _to GetTopLevelPrincipal(aTopLevelPrincipal); } \
  NS_IMETHOD GetWindow(mozIDOMWindow **aWindow) override { return _to GetWindow(aWindow); } \
  NS_IMETHOD GetElement(mozilla::dom::Element **aElement) override { return _to GetElement(aElement); } \
  NS_IMETHOD GetIsHandlingUserInput(bool *aIsHandlingUserInput) override { return _to GetIsHandlingUserInput(aIsHandlingUserInput); } \
  NS_IMETHOD GetMaybeUnsafePermissionDelegate(bool *aMaybeUnsafePermissionDelegate) override { return _to GetMaybeUnsafePermissionDelegate(aMaybeUnsafePermissionDelegate); } \
  NS_IMETHOD GetDelegatePrincipal(const nsACString& aType, nsIPrincipal **_retval) override { return _to GetDelegatePrincipal(aType, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Cancel(void) override { return _to Cancel(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Allow(JS::HandleValue choices) override { return _to Allow(choices); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICONTENTPERMISSIONREQUEST(_to) \
  NS_IMETHOD GetTypes(nsIArray **aTypes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTypes(aTypes); } \
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrincipal(aPrincipal); } \
  NS_IMETHOD GetTopLevelPrincipal(nsIPrincipal **aTopLevelPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTopLevelPrincipal(aTopLevelPrincipal); } \
  NS_IMETHOD GetWindow(mozIDOMWindow **aWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWindow(aWindow); } \
  NS_IMETHOD GetElement(mozilla::dom::Element **aElement) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetElement(aElement); } \
  NS_IMETHOD GetIsHandlingUserInput(bool *aIsHandlingUserInput) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsHandlingUserInput(aIsHandlingUserInput); } \
  NS_IMETHOD GetMaybeUnsafePermissionDelegate(bool *aMaybeUnsafePermissionDelegate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMaybeUnsafePermissionDelegate(aMaybeUnsafePermissionDelegate); } \
  NS_IMETHOD GetDelegatePrincipal(const nsACString& aType, nsIPrincipal **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDelegatePrincipal(aType, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Cancel(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Cancel(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Allow(JS::HandleValue choices) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Allow(choices); } 


/* starting interface:    nsIContentPermissionPrompt */
#define NS_ICONTENTPERMISSIONPROMPT_IID_STR "f72de90d-e954-4e69-9a61-917303029301"

#define NS_ICONTENTPERMISSIONPROMPT_IID \
  {0xf72de90d, 0xe954, 0x4e69, \
    { 0x9a, 0x61, 0x91, 0x73, 0x03, 0x02, 0x93, 0x01 }}

class NS_NO_VTABLE nsIContentPermissionPrompt : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICONTENTPERMISSIONPROMPT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIContentPermissionPrompt;

  /* void prompt (in nsIContentPermissionRequest request); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Prompt(nsIContentPermissionRequest *request) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIContentPermissionPrompt, NS_ICONTENTPERMISSIONPROMPT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICONTENTPERMISSIONPROMPT \
  NS_IMETHOD Prompt(nsIContentPermissionRequest *request) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICONTENTPERMISSIONPROMPT \
  nsresult Prompt(nsIContentPermissionRequest *request); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICONTENTPERMISSIONPROMPT(_to) \
  NS_IMETHOD Prompt(nsIContentPermissionRequest *request) override { return _to Prompt(request); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICONTENTPERMISSIONPROMPT(_to) \
  NS_IMETHOD Prompt(nsIContentPermissionRequest *request) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Prompt(request); } 

#define NS_CONTENT_PERMISSION_PROMPT_CONTRACTID   "@mozilla.org/content-permission/prompt;1"

#endif /* __gen_nsIContentPermissionPrompt_h__ */
