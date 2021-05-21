/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIOSReauthenticator.idl
 */

#ifndef __gen_nsIOSReauthenticator_h__
#define __gen_nsIOSReauthenticator_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIDOMWindow; /* forward declaration */


/* starting interface:    nsIOSReauthenticator */
#define NS_IOSREAUTHENTICATOR_IID_STR "4fe082ae-6ff0-4b41-b24f-eaa664f6e46a"

#define NS_IOSREAUTHENTICATOR_IID \
  {0x4fe082ae, 0x6ff0, 0x4b41, \
    { 0xb2, 0x4f, 0xea, 0xa6, 0x64, 0xf6, 0xe4, 0x6a }}

class NS_NO_VTABLE nsIOSReauthenticator : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IOSREAUTHENTICATOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIOSReauthenticator;

  /* [implicit_jscontext,must_use] Promise asyncReauthenticateUser (in AString prompt, in AString caption, in mozIDOMWindow parentWindow); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD AsyncReauthenticateUser(const nsAString& prompt, const nsAString& caption, mozIDOMWindow *parentWindow, JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIOSReauthenticator, NS_IOSREAUTHENTICATOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIOSREAUTHENTICATOR \
  [[nodiscard]] NS_IMETHOD AsyncReauthenticateUser(const nsAString& prompt, const nsAString& caption, mozIDOMWindow *parentWindow, JSContext* cx, ::mozilla::dom::Promise * * _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIOSREAUTHENTICATOR \
  [[nodiscard]] nsresult AsyncReauthenticateUser(const nsAString& prompt, const nsAString& caption, mozIDOMWindow *parentWindow, JSContext* cx, ::mozilla::dom::Promise * * _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIOSREAUTHENTICATOR(_to) \
  [[nodiscard]] NS_IMETHOD AsyncReauthenticateUser(const nsAString& prompt, const nsAString& caption, mozIDOMWindow *parentWindow, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to AsyncReauthenticateUser(prompt, caption, parentWindow, cx, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIOSREAUTHENTICATOR(_to) \
  [[nodiscard]] NS_IMETHOD AsyncReauthenticateUser(const nsAString& prompt, const nsAString& caption, mozIDOMWindow *parentWindow, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncReauthenticateUser(prompt, caption, parentWindow, cx, _retval); } 


#endif /* __gen_nsIOSReauthenticator_h__ */
