/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/payments/nsIPaymentUIService.idl
 */

#ifndef __gen_nsIPaymentUIService_h__
#define __gen_nsIPaymentUIService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIPaymentActionResponse_h__
#include "nsIPaymentActionResponse.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIPaymentUIService */
#define NS_IPAYMENTUISERVICE_IID_STR "01f8bd55-9017-438b-85ec-7c15d2b35cdc"

#define NS_IPAYMENTUISERVICE_IID \
  {0x01f8bd55, 0x9017, 0x438b, \
    { 0x85, 0xec, 0x7c, 0x15, 0xd2, 0xb3, 0x5c, 0xdc }}

class NS_NO_VTABLE nsIPaymentUIService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPAYMENTUISERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPaymentUIService;

  /* void showPayment (in AString requestId); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ShowPayment(const nsAString& requestId) = 0;

  /* void abortPayment (in AString requestId); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AbortPayment(const nsAString& requestId) = 0;

  /* void completePayment (in AString requestId); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CompletePayment(const nsAString& requestId) = 0;

  /* void updatePayment (in AString requestId); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UpdatePayment(const nsAString& requestId) = 0;

  /* void closePayment (in AString requestId); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ClosePayment(const nsAString& requestId) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPaymentUIService, NS_IPAYMENTUISERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPAYMENTUISERVICE \
  NS_IMETHOD ShowPayment(const nsAString& requestId) override; \
  NS_IMETHOD AbortPayment(const nsAString& requestId) override; \
  NS_IMETHOD CompletePayment(const nsAString& requestId) override; \
  NS_IMETHOD UpdatePayment(const nsAString& requestId) override; \
  NS_IMETHOD ClosePayment(const nsAString& requestId) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPAYMENTUISERVICE \
  nsresult ShowPayment(const nsAString& requestId); \
  nsresult AbortPayment(const nsAString& requestId); \
  nsresult CompletePayment(const nsAString& requestId); \
  nsresult UpdatePayment(const nsAString& requestId); \
  nsresult ClosePayment(const nsAString& requestId); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPAYMENTUISERVICE(_to) \
  NS_IMETHOD ShowPayment(const nsAString& requestId) override { return _to ShowPayment(requestId); } \
  NS_IMETHOD AbortPayment(const nsAString& requestId) override { return _to AbortPayment(requestId); } \
  NS_IMETHOD CompletePayment(const nsAString& requestId) override { return _to CompletePayment(requestId); } \
  NS_IMETHOD UpdatePayment(const nsAString& requestId) override { return _to UpdatePayment(requestId); } \
  NS_IMETHOD ClosePayment(const nsAString& requestId) override { return _to ClosePayment(requestId); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPAYMENTUISERVICE(_to) \
  NS_IMETHOD ShowPayment(const nsAString& requestId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ShowPayment(requestId); } \
  NS_IMETHOD AbortPayment(const nsAString& requestId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AbortPayment(requestId); } \
  NS_IMETHOD CompletePayment(const nsAString& requestId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CompletePayment(requestId); } \
  NS_IMETHOD UpdatePayment(const nsAString& requestId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UpdatePayment(requestId); } \
  NS_IMETHOD ClosePayment(const nsAString& requestId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClosePayment(requestId); } 

#define NS_PAYMENT_UI_SERVICE_CID \
  { 0x01f8bd55, 0x9017, 0x438b, { 0x85, 0xec, 0x7c, 0x15, 0xd2, 0xb3, 0x5c, 0xdc } }
#define NS_PAYMENT_UI_SERVICE_CONTRACT_ID \
  "@mozilla.org/dom/payments/payment-ui-service;1"

#endif /* __gen_nsIPaymentUIService_h__ */
