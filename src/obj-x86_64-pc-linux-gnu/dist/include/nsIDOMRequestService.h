/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsIDOMRequestService.idl
 */

#ifndef __gen_nsIDOMRequestService_h__
#define __gen_nsIDOMRequestService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIDOMWindow; /* forward declaration */

namespace mozilla {
namespace dom {
class DOMRequest; /* webidl DOMRequest */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIDOMRequestService */
#define NS_IDOMREQUESTSERVICE_IID_STR "9a57e5de-ce93-45fa-8145-755722834f7c"

#define NS_IDOMREQUESTSERVICE_IID \
  {0x9a57e5de, 0xce93, 0x45fa, \
    { 0x81, 0x45, 0x75, 0x57, 0x22, 0x83, 0x4f, 0x7c }}

class NS_NO_VTABLE nsIDOMRequestService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOMREQUESTSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDOMRequestService;

  /* DOMRequest createRequest (in mozIDOMWindow window); */
  NS_IMETHOD CreateRequest(mozIDOMWindow *window, mozilla::dom::DOMRequest **_retval) = 0;

  /* void fireSuccess (in DOMRequest request, in jsval result); */
  NS_IMETHOD FireSuccess(mozilla::dom::DOMRequest *request, JS::HandleValue result) = 0;

  /* void fireError (in DOMRequest request, in AString error); */
  NS_IMETHOD FireError(mozilla::dom::DOMRequest *request, const nsAString& error) = 0;

  /* void fireSuccessAsync (in DOMRequest request, in jsval result); */
  NS_IMETHOD FireSuccessAsync(mozilla::dom::DOMRequest *request, JS::HandleValue result) = 0;

  /* void fireErrorAsync (in DOMRequest request, in AString error); */
  NS_IMETHOD FireErrorAsync(mozilla::dom::DOMRequest *request, const nsAString& error) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDOMRequestService, NS_IDOMREQUESTSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOMREQUESTSERVICE \
  NS_IMETHOD CreateRequest(mozIDOMWindow *window, mozilla::dom::DOMRequest **_retval) override; \
  NS_IMETHOD FireSuccess(mozilla::dom::DOMRequest *request, JS::HandleValue result) override; \
  NS_IMETHOD FireError(mozilla::dom::DOMRequest *request, const nsAString& error) override; \
  NS_IMETHOD FireSuccessAsync(mozilla::dom::DOMRequest *request, JS::HandleValue result) override; \
  NS_IMETHOD FireErrorAsync(mozilla::dom::DOMRequest *request, const nsAString& error) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOMREQUESTSERVICE \
  nsresult CreateRequest(mozIDOMWindow *window, mozilla::dom::DOMRequest **_retval); \
  nsresult FireSuccess(mozilla::dom::DOMRequest *request, JS::HandleValue result); \
  nsresult FireError(mozilla::dom::DOMRequest *request, const nsAString& error); \
  nsresult FireSuccessAsync(mozilla::dom::DOMRequest *request, JS::HandleValue result); \
  nsresult FireErrorAsync(mozilla::dom::DOMRequest *request, const nsAString& error); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOMREQUESTSERVICE(_to) \
  NS_IMETHOD CreateRequest(mozIDOMWindow *window, mozilla::dom::DOMRequest **_retval) override { return _to CreateRequest(window, _retval); } \
  NS_IMETHOD FireSuccess(mozilla::dom::DOMRequest *request, JS::HandleValue result) override { return _to FireSuccess(request, result); } \
  NS_IMETHOD FireError(mozilla::dom::DOMRequest *request, const nsAString& error) override { return _to FireError(request, error); } \
  NS_IMETHOD FireSuccessAsync(mozilla::dom::DOMRequest *request, JS::HandleValue result) override { return _to FireSuccessAsync(request, result); } \
  NS_IMETHOD FireErrorAsync(mozilla::dom::DOMRequest *request, const nsAString& error) override { return _to FireErrorAsync(request, error); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOMREQUESTSERVICE(_to) \
  NS_IMETHOD CreateRequest(mozIDOMWindow *window, mozilla::dom::DOMRequest **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateRequest(window, _retval); } \
  NS_IMETHOD FireSuccess(mozilla::dom::DOMRequest *request, JS::HandleValue result) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FireSuccess(request, result); } \
  NS_IMETHOD FireError(mozilla::dom::DOMRequest *request, const nsAString& error) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FireError(request, error); } \
  NS_IMETHOD FireSuccessAsync(mozilla::dom::DOMRequest *request, JS::HandleValue result) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FireSuccessAsync(request, result); } \
  NS_IMETHOD FireErrorAsync(mozilla::dom::DOMRequest *request, const nsAString& error) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FireErrorAsync(request, error); } 


#endif /* __gen_nsIDOMRequestService_h__ */
