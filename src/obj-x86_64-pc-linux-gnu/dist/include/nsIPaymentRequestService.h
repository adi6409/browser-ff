/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/payments/nsIPaymentRequestService.idl
 */

#ifndef __gen_nsIPaymentRequestService_h__
#define __gen_nsIPaymentRequestService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIVariant_h__
#include "nsIVariant.h"
#endif

#ifndef __gen_nsIPaymentRequest_h__
#include "nsIPaymentRequest.h"
#endif

#ifndef __gen_nsIPaymentActionResponse_h__
#include "nsIPaymentActionResponse.h"
#endif

#ifndef __gen_nsIPaymentAddress_h__
#include "nsIPaymentAddress.h"
#endif

#ifndef __gen_nsISimpleEnumerator_h__
#include "nsISimpleEnumerator.h"
#endif

#ifndef __gen_nsIPaymentUIService_h__
#include "nsIPaymentUIService.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIPaymentRequestService */
#define NS_IPAYMENTREQUESTSERVICE_IID_STR "cccd665f-edf3-41fc-ab9b-fc55b37340aa"

#define NS_IPAYMENTREQUESTSERVICE_IID \
  {0xcccd665f, 0xedf3, 0x41fc, \
    { 0xab, 0x9b, 0xfc, 0x55, 0xb3, 0x73, 0x40, 0xaa }}

class NS_NO_VTABLE nsIPaymentRequestService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPAYMENTREQUESTSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPaymentRequestService;

  /* nsIPaymentRequest getPaymentRequestById (in AString aRequestId); */
  NS_IMETHOD GetPaymentRequestById(const nsAString& aRequestId, nsIPaymentRequest **_retval) = 0;

  /* nsISimpleEnumerator enumerate (); */
  NS_IMETHOD Enumerate(nsISimpleEnumerator **_retval) = 0;

  /* void respondPayment (in nsIPaymentActionResponse aResponse); */
  NS_IMETHOD RespondPayment(nsIPaymentActionResponse *aResponse) = 0;

  /* void changeShippingAddress (in AString requestId, in nsIPaymentAddress aAddress); */
  NS_IMETHOD ChangeShippingAddress(const nsAString& requestId, nsIPaymentAddress *aAddress) = 0;

  /* void changeShippingOption (in AString requestId, in AString option); */
  NS_IMETHOD ChangeShippingOption(const nsAString& requestId, const nsAString& option) = 0;

  /* void changePayerDetail (in AString requestId, in AString aPayerName, in AString aPayerEmail, in AString aPayerPhone); */
  NS_IMETHOD ChangePayerDetail(const nsAString& requestId, const nsAString& aPayerName, const nsAString& aPayerEmail, const nsAString& aPayerPhone) = 0;

  /* void changePaymentMethod (in AString requestId, in AString aMethodName, in nsIMethodChangeDetails aMethodDetails); */
  NS_IMETHOD ChangePaymentMethod(const nsAString& requestId, const nsAString& aMethodName, nsIMethodChangeDetails *aMethodDetails) = 0;

  /* void cleanup (); */
  NS_IMETHOD Cleanup(void) = 0;

  /* void setTestingUIService (in nsIPaymentUIService aUIService); */
  NS_IMETHOD SetTestingUIService(nsIPaymentUIService *aUIService) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPaymentRequestService, NS_IPAYMENTREQUESTSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPAYMENTREQUESTSERVICE \
  NS_IMETHOD GetPaymentRequestById(const nsAString& aRequestId, nsIPaymentRequest **_retval) override; \
  NS_IMETHOD Enumerate(nsISimpleEnumerator **_retval) override; \
  NS_IMETHOD RespondPayment(nsIPaymentActionResponse *aResponse) override; \
  NS_IMETHOD ChangeShippingAddress(const nsAString& requestId, nsIPaymentAddress *aAddress) override; \
  NS_IMETHOD ChangeShippingOption(const nsAString& requestId, const nsAString& option) override; \
  NS_IMETHOD ChangePayerDetail(const nsAString& requestId, const nsAString& aPayerName, const nsAString& aPayerEmail, const nsAString& aPayerPhone) override; \
  NS_IMETHOD ChangePaymentMethod(const nsAString& requestId, const nsAString& aMethodName, nsIMethodChangeDetails *aMethodDetails) override; \
  NS_IMETHOD Cleanup(void) override; \
  NS_IMETHOD SetTestingUIService(nsIPaymentUIService *aUIService) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPAYMENTREQUESTSERVICE \
  nsresult GetPaymentRequestById(const nsAString& aRequestId, nsIPaymentRequest **_retval); \
  nsresult Enumerate(nsISimpleEnumerator **_retval); \
  nsresult RespondPayment(nsIPaymentActionResponse *aResponse); \
  nsresult ChangeShippingAddress(const nsAString& requestId, nsIPaymentAddress *aAddress); \
  nsresult ChangeShippingOption(const nsAString& requestId, const nsAString& option); \
  nsresult ChangePayerDetail(const nsAString& requestId, const nsAString& aPayerName, const nsAString& aPayerEmail, const nsAString& aPayerPhone); \
  nsresult ChangePaymentMethod(const nsAString& requestId, const nsAString& aMethodName, nsIMethodChangeDetails *aMethodDetails); \
  nsresult Cleanup(void); \
  nsresult SetTestingUIService(nsIPaymentUIService *aUIService); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPAYMENTREQUESTSERVICE(_to) \
  NS_IMETHOD GetPaymentRequestById(const nsAString& aRequestId, nsIPaymentRequest **_retval) override { return _to GetPaymentRequestById(aRequestId, _retval); } \
  NS_IMETHOD Enumerate(nsISimpleEnumerator **_retval) override { return _to Enumerate(_retval); } \
  NS_IMETHOD RespondPayment(nsIPaymentActionResponse *aResponse) override { return _to RespondPayment(aResponse); } \
  NS_IMETHOD ChangeShippingAddress(const nsAString& requestId, nsIPaymentAddress *aAddress) override { return _to ChangeShippingAddress(requestId, aAddress); } \
  NS_IMETHOD ChangeShippingOption(const nsAString& requestId, const nsAString& option) override { return _to ChangeShippingOption(requestId, option); } \
  NS_IMETHOD ChangePayerDetail(const nsAString& requestId, const nsAString& aPayerName, const nsAString& aPayerEmail, const nsAString& aPayerPhone) override { return _to ChangePayerDetail(requestId, aPayerName, aPayerEmail, aPayerPhone); } \
  NS_IMETHOD ChangePaymentMethod(const nsAString& requestId, const nsAString& aMethodName, nsIMethodChangeDetails *aMethodDetails) override { return _to ChangePaymentMethod(requestId, aMethodName, aMethodDetails); } \
  NS_IMETHOD Cleanup(void) override { return _to Cleanup(); } \
  NS_IMETHOD SetTestingUIService(nsIPaymentUIService *aUIService) override { return _to SetTestingUIService(aUIService); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPAYMENTREQUESTSERVICE(_to) \
  NS_IMETHOD GetPaymentRequestById(const nsAString& aRequestId, nsIPaymentRequest **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPaymentRequestById(aRequestId, _retval); } \
  NS_IMETHOD Enumerate(nsISimpleEnumerator **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Enumerate(_retval); } \
  NS_IMETHOD RespondPayment(nsIPaymentActionResponse *aResponse) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RespondPayment(aResponse); } \
  NS_IMETHOD ChangeShippingAddress(const nsAString& requestId, nsIPaymentAddress *aAddress) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ChangeShippingAddress(requestId, aAddress); } \
  NS_IMETHOD ChangeShippingOption(const nsAString& requestId, const nsAString& option) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ChangeShippingOption(requestId, option); } \
  NS_IMETHOD ChangePayerDetail(const nsAString& requestId, const nsAString& aPayerName, const nsAString& aPayerEmail, const nsAString& aPayerPhone) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ChangePayerDetail(requestId, aPayerName, aPayerEmail, aPayerPhone); } \
  NS_IMETHOD ChangePaymentMethod(const nsAString& requestId, const nsAString& aMethodName, nsIMethodChangeDetails *aMethodDetails) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ChangePaymentMethod(requestId, aMethodName, aMethodDetails); } \
  NS_IMETHOD Cleanup(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Cleanup(); } \
  NS_IMETHOD SetTestingUIService(nsIPaymentUIService *aUIService) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTestingUIService(aUIService); } 

#define NS_PAYMENT_REQUEST_SERVICE_CID \
  { 0xcccd665f, 0xedf3, 0x41fc, { 0xab, 0x9b, 0xfc, 0x55, 0xb3, 0x73, 0x40, 0xaa } }
#define NS_PAYMENT_REQUEST_SERVICE_CONTRACT_ID \
  "@mozilla.org/dom/payments/payment-request-service;1"

#endif /* __gen_nsIPaymentRequestService_h__ */
