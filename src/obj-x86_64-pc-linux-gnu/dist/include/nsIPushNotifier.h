/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/push/nsIPushNotifier.idl
 */

#ifndef __gen_nsIPushNotifier_h__
#define __gen_nsIPushNotifier_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/Value.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#define PUSHNOTIFIER_CONTRACTID \
  "@mozilla.org/push/Notifier;1"
// These constants are duplicated in `PushComponents.js`.
#define OBSERVER_TOPIC_PUSH "push-message"
#define OBSERVER_TOPIC_SUBSCRIPTION_CHANGE "push-subscription-change"
#define OBSERVER_TOPIC_SUBSCRIPTION_MODIFIED "push-subscription-modified"
class nsIPrincipal; /* forward declaration */


/* starting interface:    nsIPushNotifier */
#define NS_IPUSHNOTIFIER_IID_STR "b00dfdeb-14e5-425b-adc7-b531442e3216"

#define NS_IPUSHNOTIFIER_IID \
  {0xb00dfdeb, 0x14e5, 0x425b, \
    { 0xad, 0xc7, 0xb5, 0x31, 0x44, 0x2e, 0x32, 0x16 }}

class NS_NO_VTABLE nsIPushNotifier : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPUSHNOTIFIER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPushNotifier;

  /* void notifyPush (in ACString scope, in nsIPrincipal principal, in AString messageId); */
  NS_IMETHOD NotifyPush(const nsACString& scope, nsIPrincipal *principal, const nsAString& messageId) = 0;

  /* void notifyPushWithData (in ACString scope, in nsIPrincipal principal, in AString messageId, in Array<uint8_t> data); */
  NS_IMETHOD NotifyPushWithData(const nsACString& scope, nsIPrincipal *principal, const nsAString& messageId, const nsTArray<uint8_t >& data) = 0;

  /* void notifySubscriptionChange (in ACString scope, in nsIPrincipal principal); */
  NS_IMETHOD NotifySubscriptionChange(const nsACString& scope, nsIPrincipal *principal) = 0;

  /* void notifySubscriptionModified (in ACString scope, in nsIPrincipal principal); */
  NS_IMETHOD NotifySubscriptionModified(const nsACString& scope, nsIPrincipal *principal) = 0;

  /* void notifyError (in ACString scope, in nsIPrincipal principal, in AString message, in uint32_t flags); */
  NS_IMETHOD NotifyError(const nsACString& scope, nsIPrincipal *principal, const nsAString& message, uint32_t flags) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPushNotifier, NS_IPUSHNOTIFIER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPUSHNOTIFIER \
  NS_IMETHOD NotifyPush(const nsACString& scope, nsIPrincipal *principal, const nsAString& messageId) override; \
  NS_IMETHOD NotifyPushWithData(const nsACString& scope, nsIPrincipal *principal, const nsAString& messageId, const nsTArray<uint8_t >& data) override; \
  NS_IMETHOD NotifySubscriptionChange(const nsACString& scope, nsIPrincipal *principal) override; \
  NS_IMETHOD NotifySubscriptionModified(const nsACString& scope, nsIPrincipal *principal) override; \
  NS_IMETHOD NotifyError(const nsACString& scope, nsIPrincipal *principal, const nsAString& message, uint32_t flags) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPUSHNOTIFIER \
  nsresult NotifyPush(const nsACString& scope, nsIPrincipal *principal, const nsAString& messageId); \
  nsresult NotifyPushWithData(const nsACString& scope, nsIPrincipal *principal, const nsAString& messageId, const nsTArray<uint8_t >& data); \
  nsresult NotifySubscriptionChange(const nsACString& scope, nsIPrincipal *principal); \
  nsresult NotifySubscriptionModified(const nsACString& scope, nsIPrincipal *principal); \
  nsresult NotifyError(const nsACString& scope, nsIPrincipal *principal, const nsAString& message, uint32_t flags); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPUSHNOTIFIER(_to) \
  NS_IMETHOD NotifyPush(const nsACString& scope, nsIPrincipal *principal, const nsAString& messageId) override { return _to NotifyPush(scope, principal, messageId); } \
  NS_IMETHOD NotifyPushWithData(const nsACString& scope, nsIPrincipal *principal, const nsAString& messageId, const nsTArray<uint8_t >& data) override { return _to NotifyPushWithData(scope, principal, messageId, data); } \
  NS_IMETHOD NotifySubscriptionChange(const nsACString& scope, nsIPrincipal *principal) override { return _to NotifySubscriptionChange(scope, principal); } \
  NS_IMETHOD NotifySubscriptionModified(const nsACString& scope, nsIPrincipal *principal) override { return _to NotifySubscriptionModified(scope, principal); } \
  NS_IMETHOD NotifyError(const nsACString& scope, nsIPrincipal *principal, const nsAString& message, uint32_t flags) override { return _to NotifyError(scope, principal, message, flags); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPUSHNOTIFIER(_to) \
  NS_IMETHOD NotifyPush(const nsACString& scope, nsIPrincipal *principal, const nsAString& messageId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyPush(scope, principal, messageId); } \
  NS_IMETHOD NotifyPushWithData(const nsACString& scope, nsIPrincipal *principal, const nsAString& messageId, const nsTArray<uint8_t >& data) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyPushWithData(scope, principal, messageId, data); } \
  NS_IMETHOD NotifySubscriptionChange(const nsACString& scope, nsIPrincipal *principal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifySubscriptionChange(scope, principal); } \
  NS_IMETHOD NotifySubscriptionModified(const nsACString& scope, nsIPrincipal *principal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifySubscriptionModified(scope, principal); } \
  NS_IMETHOD NotifyError(const nsACString& scope, nsIPrincipal *principal, const nsAString& message, uint32_t flags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyError(scope, principal, message, flags); } 


/* starting interface:    nsIPushData */
#define NS_IPUSHDATA_IID_STR "dfc4f151-cead-40df-8eb7-7a7a67c54b16"

#define NS_IPUSHDATA_IID \
  {0xdfc4f151, 0xcead, 0x40df, \
    { 0x8e, 0xb7, 0x7a, 0x7a, 0x67, 0xc5, 0x4b, 0x16 }}

class NS_NO_VTABLE nsIPushData : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPUSHDATA_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPushData;

  /* AString text (); */
  NS_IMETHOD Text(nsAString& _retval) = 0;

  /* [implicit_jscontext] jsval json (); */
  NS_IMETHOD Json(JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* Array<uint8_t> binary (); */
  NS_IMETHOD Binary(nsTArray<uint8_t >& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPushData, NS_IPUSHDATA_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPUSHDATA \
  NS_IMETHOD Text(nsAString& _retval) override; \
  NS_IMETHOD Json(JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD Binary(nsTArray<uint8_t >& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPUSHDATA \
  nsresult Text(nsAString& _retval); \
  nsresult Json(JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult Binary(nsTArray<uint8_t >& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPUSHDATA(_to) \
  NS_IMETHOD Text(nsAString& _retval) override { return _to Text(_retval); } \
  NS_IMETHOD Json(JSContext* cx, JS::MutableHandleValue _retval) override { return _to Json(cx, _retval); } \
  NS_IMETHOD Binary(nsTArray<uint8_t >& _retval) override { return _to Binary(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPUSHDATA(_to) \
  NS_IMETHOD Text(nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Text(_retval); } \
  NS_IMETHOD Json(JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Json(cx, _retval); } \
  NS_IMETHOD Binary(nsTArray<uint8_t >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Binary(_retval); } 


/* starting interface:    nsIPushMessage */
#define NS_IPUSHMESSAGE_IID_STR "b9d063ca-0e3f-4fee-be4b-ea9103263433"

#define NS_IPUSHMESSAGE_IID \
  {0xb9d063ca, 0x0e3f, 0x4fee, \
    { 0xbe, 0x4b, 0xea, 0x91, 0x03, 0x26, 0x34, 0x33 }}

class NS_NO_VTABLE nsIPushMessage : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPUSHMESSAGE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPushMessage;

  /* readonly attribute nsIPrincipal principal; */
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) = 0;

  /* readonly attribute nsIPushData data; */
  NS_IMETHOD GetData(nsIPushData **aData) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPushMessage, NS_IPUSHMESSAGE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPUSHMESSAGE \
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) override; \
  NS_IMETHOD GetData(nsIPushData **aData) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPUSHMESSAGE \
  nsresult GetPrincipal(nsIPrincipal **aPrincipal); \
  nsresult GetData(nsIPushData **aData); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPUSHMESSAGE(_to) \
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) override { return _to GetPrincipal(aPrincipal); } \
  NS_IMETHOD GetData(nsIPushData **aData) override { return _to GetData(aData); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPUSHMESSAGE(_to) \
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrincipal(aPrincipal); } \
  NS_IMETHOD GetData(nsIPushData **aData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetData(aData); } 


#endif /* __gen_nsIPushNotifier_h__ */
