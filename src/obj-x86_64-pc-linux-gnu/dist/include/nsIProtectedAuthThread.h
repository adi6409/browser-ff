/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIProtectedAuthThread.idl
 */

#ifndef __gen_nsIProtectedAuthThread_h__
#define __gen_nsIProtectedAuthThread_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIObserver_h__
#include "nsIObserver.h"
#endif

#ifndef __gen_nsIPKCS11Slot_h__
#include "nsIPKCS11Slot.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIProtectedAuthThread */
#define NS_IPROTECTEDAUTHTHREAD_IID_STR "4bb27cb7-8984-4cee-8ce7-9b014c3d091b"

#define NS_IPROTECTEDAUTHTHREAD_IID \
  {0x4bb27cb7, 0x8984, 0x4cee, \
    { 0x8c, 0xe7, 0x9b, 0x01, 0x4c, 0x3d, 0x09, 0x1b }}

class NS_NO_VTABLE nsIProtectedAuthThread : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPROTECTEDAUTHTHREAD_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIProtectedAuthThread;

  /* [must_use] void login (in nsIObserver observer); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD Login(nsIObserver *observer) = 0;

  /* [must_use] readonly attribute nsIPKCS11Slot slot; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetSlot(nsIPKCS11Slot **aSlot) = 0;

  /* [must_use] AString getTokenName (); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetTokenName(nsAString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIProtectedAuthThread, NS_IPROTECTEDAUTHTHREAD_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPROTECTEDAUTHTHREAD \
  [[nodiscard]] NS_IMETHOD Login(nsIObserver *observer) override; \
  [[nodiscard]] NS_IMETHOD GetSlot(nsIPKCS11Slot **aSlot) override; \
  [[nodiscard]] NS_IMETHOD GetTokenName(nsAString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPROTECTEDAUTHTHREAD \
  [[nodiscard]] nsresult Login(nsIObserver *observer); \
  [[nodiscard]] nsresult GetSlot(nsIPKCS11Slot **aSlot); \
  [[nodiscard]] nsresult GetTokenName(nsAString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPROTECTEDAUTHTHREAD(_to) \
  [[nodiscard]] NS_IMETHOD Login(nsIObserver *observer) override { return _to Login(observer); } \
  [[nodiscard]] NS_IMETHOD GetSlot(nsIPKCS11Slot **aSlot) override { return _to GetSlot(aSlot); } \
  [[nodiscard]] NS_IMETHOD GetTokenName(nsAString& _retval) override { return _to GetTokenName(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPROTECTEDAUTHTHREAD(_to) \
  [[nodiscard]] NS_IMETHOD Login(nsIObserver *observer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Login(observer); } \
  [[nodiscard]] NS_IMETHOD GetSlot(nsIPKCS11Slot **aSlot) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSlot(aSlot); } \
  [[nodiscard]] NS_IMETHOD GetTokenName(nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTokenName(_retval); } 

// {45334489-3D30-47c6-920B-0A55A313AEBF}
#define NS_PROTECTEDAUTHTHREAD_CID \
{ 0x45334489, 0x3d30, 0x47c6, { 0x92, 0x0b, 0x0a, 0x55, 0xa3, 0x13, 0xae, 0xbf } }
#define NS_PROTECTEDAUTHTHREAD_CONTRACTID "@mozilla.org/security/protectedauththread;1"

#endif /* __gen_nsIProtectedAuthThread_h__ */
