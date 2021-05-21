/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/payments/nsIPaymentActionResponse.idl
 */

#ifndef __gen_nsIPaymentActionResponse_h__
#define __gen_nsIPaymentActionResponse_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIVariant_h__
#include "nsIVariant.h"
#endif

#ifndef __gen_nsIPaymentAddress_h__
#include "nsIPaymentAddress.h"
#endif

#include "js/Value.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIPaymentResponseData */
#define NS_IPAYMENTRESPONSEDATA_IID_STR "2a338575-c688-40ee-a157-7488ab292ef2"

#define NS_IPAYMENTRESPONSEDATA_IID \
  {0x2a338575, 0xc688, 0x40ee, \
    { 0xa1, 0x57, 0x74, 0x88, 0xab, 0x29, 0x2e, 0xf2 }}

class NS_NO_VTABLE nsIPaymentResponseData : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPAYMENTRESPONSEDATA_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPaymentResponseData;

  enum {
    GENERAL_RESPONSE = 0U,
    BASICCARD_RESPONSE = 1U
  };

  /* readonly attribute uint32_t type; */
  NS_IMETHOD GetType(uint32_t *aType) = 0;

  /* void init (in uint32_t aType); */
  NS_IMETHOD Init(uint32_t aType) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPaymentResponseData, NS_IPAYMENTRESPONSEDATA_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPAYMENTRESPONSEDATA \
  NS_IMETHOD GetType(uint32_t *aType) override; \
  NS_IMETHOD Init(uint32_t aType) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPAYMENTRESPONSEDATA \
  nsresult GetType(uint32_t *aType); \
  nsresult Init(uint32_t aType); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPAYMENTRESPONSEDATA(_to) \
  NS_IMETHOD GetType(uint32_t *aType) override { return _to GetType(aType); } \
  NS_IMETHOD Init(uint32_t aType) override { return _to Init(aType); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPAYMENTRESPONSEDATA(_to) \
  NS_IMETHOD GetType(uint32_t *aType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetType(aType); } \
  NS_IMETHOD Init(uint32_t aType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aType); } 


/* starting interface:    nsIGeneralResponseData */
#define NS_IGENERALRESPONSEDATA_IID_STR "b986773e-2b30-4ed2-b8fe-6a96631c8000"

#define NS_IGENERALRESPONSEDATA_IID \
  {0xb986773e, 0x2b30, 0x4ed2, \
    { 0xb8, 0xfe, 0x6a, 0x96, 0x63, 0x1c, 0x80, 0x00 }}

class NS_NO_VTABLE nsIGeneralResponseData : public nsIPaymentResponseData {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IGENERALRESPONSEDATA_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIGeneralResponseData;

  /* readonly attribute AString data; */
  NS_IMETHOD GetData(nsAString& aData) = 0;

  /* [implicit_jscontext] void initData (in jsval aData); */
  NS_IMETHOD InitData(JS::HandleValue aData, JSContext* cx) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIGeneralResponseData, NS_IGENERALRESPONSEDATA_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIGENERALRESPONSEDATA \
  NS_IMETHOD GetData(nsAString& aData) override; \
  NS_IMETHOD InitData(JS::HandleValue aData, JSContext* cx) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIGENERALRESPONSEDATA \
  nsresult GetData(nsAString& aData); \
  nsresult InitData(JS::HandleValue aData, JSContext* cx); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIGENERALRESPONSEDATA(_to) \
  NS_IMETHOD GetData(nsAString& aData) override { return _to GetData(aData); } \
  NS_IMETHOD InitData(JS::HandleValue aData, JSContext* cx) override { return _to InitData(aData, cx); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIGENERALRESPONSEDATA(_to) \
  NS_IMETHOD GetData(nsAString& aData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetData(aData); } \
  NS_IMETHOD InitData(JS::HandleValue aData, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitData(aData, cx); } 


/* starting interface:    nsIBasicCardResponseData */
#define NS_IBASICCARDRESPONSEDATA_IID_STR "0d55a5e6-d185-44f0-b992-a8e1321e4bce"

#define NS_IBASICCARDRESPONSEDATA_IID \
  {0x0d55a5e6, 0xd185, 0x44f0, \
    { 0xb9, 0x92, 0xa8, 0xe1, 0x32, 0x1e, 0x4b, 0xce }}

class NS_NO_VTABLE nsIBasicCardResponseData : public nsIPaymentResponseData {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IBASICCARDRESPONSEDATA_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIBasicCardResponseData;

  /* readonly attribute AString cardholderName; */
  NS_IMETHOD GetCardholderName(nsAString& aCardholderName) = 0;

  /* readonly attribute AString cardNumber; */
  NS_IMETHOD GetCardNumber(nsAString& aCardNumber) = 0;

  /* readonly attribute AString expiryMonth; */
  NS_IMETHOD GetExpiryMonth(nsAString& aExpiryMonth) = 0;

  /* readonly attribute AString expiryYear; */
  NS_IMETHOD GetExpiryYear(nsAString& aExpiryYear) = 0;

  /* readonly attribute AString cardSecurityCode; */
  NS_IMETHOD GetCardSecurityCode(nsAString& aCardSecurityCode) = 0;

  /* readonly attribute nsIPaymentAddress billingAddress; */
  NS_IMETHOD GetBillingAddress(nsIPaymentAddress **aBillingAddress) = 0;

  /* void initData (in AString aCardholderName, in AString aCardNumber, in AString aExpiryMonth, in AString aExpiryYear, in AString aCardSecurityCode, in nsIPaymentAddress billingAddress); */
  NS_IMETHOD InitData(const nsAString& aCardholderName, const nsAString& aCardNumber, const nsAString& aExpiryMonth, const nsAString& aExpiryYear, const nsAString& aCardSecurityCode, nsIPaymentAddress *billingAddress) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIBasicCardResponseData, NS_IBASICCARDRESPONSEDATA_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIBASICCARDRESPONSEDATA \
  NS_IMETHOD GetCardholderName(nsAString& aCardholderName) override; \
  NS_IMETHOD GetCardNumber(nsAString& aCardNumber) override; \
  NS_IMETHOD GetExpiryMonth(nsAString& aExpiryMonth) override; \
  NS_IMETHOD GetExpiryYear(nsAString& aExpiryYear) override; \
  NS_IMETHOD GetCardSecurityCode(nsAString& aCardSecurityCode) override; \
  NS_IMETHOD GetBillingAddress(nsIPaymentAddress **aBillingAddress) override; \
  NS_IMETHOD InitData(const nsAString& aCardholderName, const nsAString& aCardNumber, const nsAString& aExpiryMonth, const nsAString& aExpiryYear, const nsAString& aCardSecurityCode, nsIPaymentAddress *billingAddress) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIBASICCARDRESPONSEDATA \
  nsresult GetCardholderName(nsAString& aCardholderName); \
  nsresult GetCardNumber(nsAString& aCardNumber); \
  nsresult GetExpiryMonth(nsAString& aExpiryMonth); \
  nsresult GetExpiryYear(nsAString& aExpiryYear); \
  nsresult GetCardSecurityCode(nsAString& aCardSecurityCode); \
  nsresult GetBillingAddress(nsIPaymentAddress **aBillingAddress); \
  nsresult InitData(const nsAString& aCardholderName, const nsAString& aCardNumber, const nsAString& aExpiryMonth, const nsAString& aExpiryYear, const nsAString& aCardSecurityCode, nsIPaymentAddress *billingAddress); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIBASICCARDRESPONSEDATA(_to) \
  NS_IMETHOD GetCardholderName(nsAString& aCardholderName) override { return _to GetCardholderName(aCardholderName); } \
  NS_IMETHOD GetCardNumber(nsAString& aCardNumber) override { return _to GetCardNumber(aCardNumber); } \
  NS_IMETHOD GetExpiryMonth(nsAString& aExpiryMonth) override { return _to GetExpiryMonth(aExpiryMonth); } \
  NS_IMETHOD GetExpiryYear(nsAString& aExpiryYear) override { return _to GetExpiryYear(aExpiryYear); } \
  NS_IMETHOD GetCardSecurityCode(nsAString& aCardSecurityCode) override { return _to GetCardSecurityCode(aCardSecurityCode); } \
  NS_IMETHOD GetBillingAddress(nsIPaymentAddress **aBillingAddress) override { return _to GetBillingAddress(aBillingAddress); } \
  NS_IMETHOD InitData(const nsAString& aCardholderName, const nsAString& aCardNumber, const nsAString& aExpiryMonth, const nsAString& aExpiryYear, const nsAString& aCardSecurityCode, nsIPaymentAddress *billingAddress) override { return _to InitData(aCardholderName, aCardNumber, aExpiryMonth, aExpiryYear, aCardSecurityCode, billingAddress); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIBASICCARDRESPONSEDATA(_to) \
  NS_IMETHOD GetCardholderName(nsAString& aCardholderName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCardholderName(aCardholderName); } \
  NS_IMETHOD GetCardNumber(nsAString& aCardNumber) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCardNumber(aCardNumber); } \
  NS_IMETHOD GetExpiryMonth(nsAString& aExpiryMonth) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetExpiryMonth(aExpiryMonth); } \
  NS_IMETHOD GetExpiryYear(nsAString& aExpiryYear) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetExpiryYear(aExpiryYear); } \
  NS_IMETHOD GetCardSecurityCode(nsAString& aCardSecurityCode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCardSecurityCode(aCardSecurityCode); } \
  NS_IMETHOD GetBillingAddress(nsIPaymentAddress **aBillingAddress) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBillingAddress(aBillingAddress); } \
  NS_IMETHOD InitData(const nsAString& aCardholderName, const nsAString& aCardNumber, const nsAString& aExpiryMonth, const nsAString& aExpiryYear, const nsAString& aCardSecurityCode, nsIPaymentAddress *billingAddress) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitData(aCardholderName, aCardNumber, aExpiryMonth, aExpiryYear, aCardSecurityCode, billingAddress); } 


/* starting interface:    nsIPaymentActionResponse */
#define NS_IPAYMENTACTIONRESPONSE_IID_STR "a607c095-ef60-4a9b-a3d0-0506c60728b3"

#define NS_IPAYMENTACTIONRESPONSE_IID \
  {0xa607c095, 0xef60, 0x4a9b, \
    { 0xa3, 0xd0, 0x05, 0x06, 0xc6, 0x07, 0x28, 0xb3 }}

class NS_NO_VTABLE nsIPaymentActionResponse : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPAYMENTACTIONRESPONSE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPaymentActionResponse;

  enum {
    NO_TYPE = 0U,
    CANMAKE_ACTION = 2U,
    SHOW_ACTION = 3U,
    ABORT_ACTION = 4U,
    COMPLETE_ACTION = 5U,
    ABORT_SUCCEEDED = 1U,
    ABORT_FAILED = 0U,
    PAYMENT_REJECTED = 0U,
    PAYMENT_ACCEPTED = 1U,
    PAYMENT_NOTSUPPORTED = 2U,
    COMPLETE_SUCCEEDED = 1U,
    COMPLETE_FAILED = 0U
  };

  /* readonly attribute AString requestId; */
  NS_IMETHOD GetRequestId(nsAString& aRequestId) = 0;

  /* readonly attribute uint32_t type; */
  NS_IMETHOD GetType(uint32_t *aType) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPaymentActionResponse, NS_IPAYMENTACTIONRESPONSE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPAYMENTACTIONRESPONSE \
  NS_IMETHOD GetRequestId(nsAString& aRequestId) override; \
  NS_IMETHOD GetType(uint32_t *aType) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPAYMENTACTIONRESPONSE \
  nsresult GetRequestId(nsAString& aRequestId); \
  nsresult GetType(uint32_t *aType); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPAYMENTACTIONRESPONSE(_to) \
  NS_IMETHOD GetRequestId(nsAString& aRequestId) override { return _to GetRequestId(aRequestId); } \
  NS_IMETHOD GetType(uint32_t *aType) override { return _to GetType(aType); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPAYMENTACTIONRESPONSE(_to) \
  NS_IMETHOD GetRequestId(nsAString& aRequestId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRequestId(aRequestId); } \
  NS_IMETHOD GetType(uint32_t *aType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetType(aType); } 


/* starting interface:    nsIPaymentCanMakeActionResponse */
#define NS_IPAYMENTCANMAKEACTIONRESPONSE_IID_STR "52fc3f9f-c0cb-4874-b3d4-ee4b6e9cbe9c"

#define NS_IPAYMENTCANMAKEACTIONRESPONSE_IID \
  {0x52fc3f9f, 0xc0cb, 0x4874, \
    { 0xb3, 0xd4, 0xee, 0x4b, 0x6e, 0x9c, 0xbe, 0x9c }}

class NS_NO_VTABLE nsIPaymentCanMakeActionResponse : public nsIPaymentActionResponse {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPAYMENTCANMAKEACTIONRESPONSE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPaymentCanMakeActionResponse;

  /* readonly attribute bool result; */
  NS_IMETHOD GetResult(bool *aResult) = 0;

  /* void init (in AString aRequestId, in bool aResult); */
  NS_IMETHOD Init(const nsAString& aRequestId, bool aResult) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPaymentCanMakeActionResponse, NS_IPAYMENTCANMAKEACTIONRESPONSE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPAYMENTCANMAKEACTIONRESPONSE \
  NS_IMETHOD GetResult(bool *aResult) override; \
  NS_IMETHOD Init(const nsAString& aRequestId, bool aResult) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPAYMENTCANMAKEACTIONRESPONSE \
  nsresult GetResult(bool *aResult); \
  nsresult Init(const nsAString& aRequestId, bool aResult); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPAYMENTCANMAKEACTIONRESPONSE(_to) \
  NS_IMETHOD GetResult(bool *aResult) override { return _to GetResult(aResult); } \
  NS_IMETHOD Init(const nsAString& aRequestId, bool aResult) override { return _to Init(aRequestId, aResult); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPAYMENTCANMAKEACTIONRESPONSE(_to) \
  NS_IMETHOD GetResult(bool *aResult) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResult(aResult); } \
  NS_IMETHOD Init(const nsAString& aRequestId, bool aResult) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aRequestId, aResult); } 


/* starting interface:    nsIPaymentShowActionResponse */
#define NS_IPAYMENTSHOWACTIONRESPONSE_IID_STR "184385cb-2d35-4b99-a9a3-7c780bf66b9b"

#define NS_IPAYMENTSHOWACTIONRESPONSE_IID \
  {0x184385cb, 0x2d35, 0x4b99, \
    { 0xa9, 0xa3, 0x7c, 0x78, 0x0b, 0xf6, 0x6b, 0x9b }}

class NS_NO_VTABLE nsIPaymentShowActionResponse : public nsIPaymentActionResponse {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPAYMENTSHOWACTIONRESPONSE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPaymentShowActionResponse;

  /* readonly attribute uint32_t acceptStatus; */
  NS_IMETHOD GetAcceptStatus(uint32_t *aAcceptStatus) = 0;

  /* readonly attribute AString methodName; */
  NS_IMETHOD GetMethodName(nsAString& aMethodName) = 0;

  /* readonly attribute nsIPaymentResponseData data; */
  NS_IMETHOD GetData(nsIPaymentResponseData **aData) = 0;

  /* readonly attribute AString payerName; */
  NS_IMETHOD GetPayerName(nsAString& aPayerName) = 0;

  /* readonly attribute AString payerEmail; */
  NS_IMETHOD GetPayerEmail(nsAString& aPayerEmail) = 0;

  /* readonly attribute AString payerPhone; */
  NS_IMETHOD GetPayerPhone(nsAString& aPayerPhone) = 0;

  /* void init (in AString aRequestId, in uint32_t aAcceptStatus, in AString aMethodName, in nsIPaymentResponseData aData, in AString aPayerName, in AString aPayerEmail, in AString aPayerPhone); */
  NS_IMETHOD Init(const nsAString& aRequestId, uint32_t aAcceptStatus, const nsAString& aMethodName, nsIPaymentResponseData *aData, const nsAString& aPayerName, const nsAString& aPayerEmail, const nsAString& aPayerPhone) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPaymentShowActionResponse, NS_IPAYMENTSHOWACTIONRESPONSE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPAYMENTSHOWACTIONRESPONSE \
  NS_IMETHOD GetAcceptStatus(uint32_t *aAcceptStatus) override; \
  NS_IMETHOD GetMethodName(nsAString& aMethodName) override; \
  NS_IMETHOD GetData(nsIPaymentResponseData **aData) override; \
  NS_IMETHOD GetPayerName(nsAString& aPayerName) override; \
  NS_IMETHOD GetPayerEmail(nsAString& aPayerEmail) override; \
  NS_IMETHOD GetPayerPhone(nsAString& aPayerPhone) override; \
  NS_IMETHOD Init(const nsAString& aRequestId, uint32_t aAcceptStatus, const nsAString& aMethodName, nsIPaymentResponseData *aData, const nsAString& aPayerName, const nsAString& aPayerEmail, const nsAString& aPayerPhone) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPAYMENTSHOWACTIONRESPONSE \
  nsresult GetAcceptStatus(uint32_t *aAcceptStatus); \
  nsresult GetMethodName(nsAString& aMethodName); \
  nsresult GetData(nsIPaymentResponseData **aData); \
  nsresult GetPayerName(nsAString& aPayerName); \
  nsresult GetPayerEmail(nsAString& aPayerEmail); \
  nsresult GetPayerPhone(nsAString& aPayerPhone); \
  nsresult Init(const nsAString& aRequestId, uint32_t aAcceptStatus, const nsAString& aMethodName, nsIPaymentResponseData *aData, const nsAString& aPayerName, const nsAString& aPayerEmail, const nsAString& aPayerPhone); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPAYMENTSHOWACTIONRESPONSE(_to) \
  NS_IMETHOD GetAcceptStatus(uint32_t *aAcceptStatus) override { return _to GetAcceptStatus(aAcceptStatus); } \
  NS_IMETHOD GetMethodName(nsAString& aMethodName) override { return _to GetMethodName(aMethodName); } \
  NS_IMETHOD GetData(nsIPaymentResponseData **aData) override { return _to GetData(aData); } \
  NS_IMETHOD GetPayerName(nsAString& aPayerName) override { return _to GetPayerName(aPayerName); } \
  NS_IMETHOD GetPayerEmail(nsAString& aPayerEmail) override { return _to GetPayerEmail(aPayerEmail); } \
  NS_IMETHOD GetPayerPhone(nsAString& aPayerPhone) override { return _to GetPayerPhone(aPayerPhone); } \
  NS_IMETHOD Init(const nsAString& aRequestId, uint32_t aAcceptStatus, const nsAString& aMethodName, nsIPaymentResponseData *aData, const nsAString& aPayerName, const nsAString& aPayerEmail, const nsAString& aPayerPhone) override { return _to Init(aRequestId, aAcceptStatus, aMethodName, aData, aPayerName, aPayerEmail, aPayerPhone); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPAYMENTSHOWACTIONRESPONSE(_to) \
  NS_IMETHOD GetAcceptStatus(uint32_t *aAcceptStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAcceptStatus(aAcceptStatus); } \
  NS_IMETHOD GetMethodName(nsAString& aMethodName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMethodName(aMethodName); } \
  NS_IMETHOD GetData(nsIPaymentResponseData **aData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetData(aData); } \
  NS_IMETHOD GetPayerName(nsAString& aPayerName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPayerName(aPayerName); } \
  NS_IMETHOD GetPayerEmail(nsAString& aPayerEmail) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPayerEmail(aPayerEmail); } \
  NS_IMETHOD GetPayerPhone(nsAString& aPayerPhone) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPayerPhone(aPayerPhone); } \
  NS_IMETHOD Init(const nsAString& aRequestId, uint32_t aAcceptStatus, const nsAString& aMethodName, nsIPaymentResponseData *aData, const nsAString& aPayerName, const nsAString& aPayerEmail, const nsAString& aPayerPhone) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aRequestId, aAcceptStatus, aMethodName, aData, aPayerName, aPayerEmail, aPayerPhone); } 


/* starting interface:    nsIPaymentAbortActionResponse */
#define NS_IPAYMENTABORTACTIONRESPONSE_IID_STR "8c72bcdb-0c37-4786-a9e5-510afa2f8ede"

#define NS_IPAYMENTABORTACTIONRESPONSE_IID \
  {0x8c72bcdb, 0x0c37, 0x4786, \
    { 0xa9, 0xe5, 0x51, 0x0a, 0xfa, 0x2f, 0x8e, 0xde }}

class NS_NO_VTABLE nsIPaymentAbortActionResponse : public nsIPaymentActionResponse {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPAYMENTABORTACTIONRESPONSE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPaymentAbortActionResponse;

  /* readonly attribute uint32_t abortStatus; */
  NS_IMETHOD GetAbortStatus(uint32_t *aAbortStatus) = 0;

  /* void init (in AString aRequestId, in uint32_t aAbortStatus); */
  NS_IMETHOD Init(const nsAString& aRequestId, uint32_t aAbortStatus) = 0;

  /* bool isSucceeded (); */
  NS_IMETHOD IsSucceeded(bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPaymentAbortActionResponse, NS_IPAYMENTABORTACTIONRESPONSE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPAYMENTABORTACTIONRESPONSE \
  NS_IMETHOD GetAbortStatus(uint32_t *aAbortStatus) override; \
  NS_IMETHOD Init(const nsAString& aRequestId, uint32_t aAbortStatus) override; \
  NS_IMETHOD IsSucceeded(bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPAYMENTABORTACTIONRESPONSE \
  nsresult GetAbortStatus(uint32_t *aAbortStatus); \
  nsresult Init(const nsAString& aRequestId, uint32_t aAbortStatus); \
  nsresult IsSucceeded(bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPAYMENTABORTACTIONRESPONSE(_to) \
  NS_IMETHOD GetAbortStatus(uint32_t *aAbortStatus) override { return _to GetAbortStatus(aAbortStatus); } \
  NS_IMETHOD Init(const nsAString& aRequestId, uint32_t aAbortStatus) override { return _to Init(aRequestId, aAbortStatus); } \
  NS_IMETHOD IsSucceeded(bool *_retval) override { return _to IsSucceeded(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPAYMENTABORTACTIONRESPONSE(_to) \
  NS_IMETHOD GetAbortStatus(uint32_t *aAbortStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAbortStatus(aAbortStatus); } \
  NS_IMETHOD Init(const nsAString& aRequestId, uint32_t aAbortStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aRequestId, aAbortStatus); } \
  NS_IMETHOD IsSucceeded(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsSucceeded(_retval); } 


/* starting interface:    nsIPaymentCompleteActionResponse */
#define NS_IPAYMENTCOMPLETEACTIONRESPONSE_IID_STR "62c01e69-9ca4-4060-99e4-b95f628c8e6d"

#define NS_IPAYMENTCOMPLETEACTIONRESPONSE_IID \
  {0x62c01e69, 0x9ca4, 0x4060, \
    { 0x99, 0xe4, 0xb9, 0x5f, 0x62, 0x8c, 0x8e, 0x6d }}

class NS_NO_VTABLE nsIPaymentCompleteActionResponse : public nsIPaymentActionResponse {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPAYMENTCOMPLETEACTIONRESPONSE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPaymentCompleteActionResponse;

  /* readonly attribute uint32_t completeStatus; */
  NS_IMETHOD GetCompleteStatus(uint32_t *aCompleteStatus) = 0;

  /* void init (in AString aRequestId, in uint32_t aCompleteStatus); */
  NS_IMETHOD Init(const nsAString& aRequestId, uint32_t aCompleteStatus) = 0;

  /* bool isCompleted (); */
  NS_IMETHOD IsCompleted(bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPaymentCompleteActionResponse, NS_IPAYMENTCOMPLETEACTIONRESPONSE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPAYMENTCOMPLETEACTIONRESPONSE \
  NS_IMETHOD GetCompleteStatus(uint32_t *aCompleteStatus) override; \
  NS_IMETHOD Init(const nsAString& aRequestId, uint32_t aCompleteStatus) override; \
  NS_IMETHOD IsCompleted(bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPAYMENTCOMPLETEACTIONRESPONSE \
  nsresult GetCompleteStatus(uint32_t *aCompleteStatus); \
  nsresult Init(const nsAString& aRequestId, uint32_t aCompleteStatus); \
  nsresult IsCompleted(bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPAYMENTCOMPLETEACTIONRESPONSE(_to) \
  NS_IMETHOD GetCompleteStatus(uint32_t *aCompleteStatus) override { return _to GetCompleteStatus(aCompleteStatus); } \
  NS_IMETHOD Init(const nsAString& aRequestId, uint32_t aCompleteStatus) override { return _to Init(aRequestId, aCompleteStatus); } \
  NS_IMETHOD IsCompleted(bool *_retval) override { return _to IsCompleted(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPAYMENTCOMPLETEACTIONRESPONSE(_to) \
  NS_IMETHOD GetCompleteStatus(uint32_t *aCompleteStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCompleteStatus(aCompleteStatus); } \
  NS_IMETHOD Init(const nsAString& aRequestId, uint32_t aCompleteStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aRequestId, aCompleteStatus); } \
  NS_IMETHOD IsCompleted(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsCompleted(_retval); } 


/* starting interface:    nsIMethodChangeDetails */
#define NS_IMETHODCHANGEDETAILS_IID_STR "2035e0a9-c9ab-4c9f-b8e9-28b2ed61548c"

#define NS_IMETHODCHANGEDETAILS_IID \
  {0x2035e0a9, 0xc9ab, 0x4c9f, \
    { 0xb8, 0xe9, 0x28, 0xb2, 0xed, 0x61, 0x54, 0x8c }}

class NS_NO_VTABLE nsIMethodChangeDetails : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IMETHODCHANGEDETAILS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIMethodChangeDetails;

  enum {
    GENERAL_DETAILS = 0U,
    BASICCARD_DETAILS = 1U
  };

  /* readonly attribute uint32_t type; */
  NS_IMETHOD GetType(uint32_t *aType) = 0;

  /* void init (in uint32_t aType); */
  NS_IMETHOD Init(uint32_t aType) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIMethodChangeDetails, NS_IMETHODCHANGEDETAILS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIMETHODCHANGEDETAILS \
  NS_IMETHOD GetType(uint32_t *aType) override; \
  NS_IMETHOD Init(uint32_t aType) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIMETHODCHANGEDETAILS \
  nsresult GetType(uint32_t *aType); \
  nsresult Init(uint32_t aType); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIMETHODCHANGEDETAILS(_to) \
  NS_IMETHOD GetType(uint32_t *aType) override { return _to GetType(aType); } \
  NS_IMETHOD Init(uint32_t aType) override { return _to Init(aType); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIMETHODCHANGEDETAILS(_to) \
  NS_IMETHOD GetType(uint32_t *aType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetType(aType); } \
  NS_IMETHOD Init(uint32_t aType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aType); } 


/* starting interface:    nsIGeneralChangeDetails */
#define NS_IGENERALCHANGEDETAILS_IID_STR "e031267e-bec8-4f3c-b0b1-396b77ca260c"

#define NS_IGENERALCHANGEDETAILS_IID \
  {0xe031267e, 0xbec8, 0x4f3c, \
    { 0xb0, 0xb1, 0x39, 0x6b, 0x77, 0xca, 0x26, 0x0c }}

class NS_NO_VTABLE nsIGeneralChangeDetails : public nsIMethodChangeDetails {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IGENERALCHANGEDETAILS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIGeneralChangeDetails;

  /* readonly attribute AString details; */
  NS_IMETHOD GetDetails(nsAString& aDetails) = 0;

  /* [implicit_jscontext] void initData (in jsval aDetails); */
  NS_IMETHOD InitData(JS::HandleValue aDetails, JSContext* cx) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIGeneralChangeDetails, NS_IGENERALCHANGEDETAILS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIGENERALCHANGEDETAILS \
  NS_IMETHOD GetDetails(nsAString& aDetails) override; \
  NS_IMETHOD InitData(JS::HandleValue aDetails, JSContext* cx) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIGENERALCHANGEDETAILS \
  nsresult GetDetails(nsAString& aDetails); \
  nsresult InitData(JS::HandleValue aDetails, JSContext* cx); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIGENERALCHANGEDETAILS(_to) \
  NS_IMETHOD GetDetails(nsAString& aDetails) override { return _to GetDetails(aDetails); } \
  NS_IMETHOD InitData(JS::HandleValue aDetails, JSContext* cx) override { return _to InitData(aDetails, cx); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIGENERALCHANGEDETAILS(_to) \
  NS_IMETHOD GetDetails(nsAString& aDetails) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDetails(aDetails); } \
  NS_IMETHOD InitData(JS::HandleValue aDetails, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitData(aDetails, cx); } 


/* starting interface:    nsIBasicCardChangeDetails */
#define NS_IBASICCARDCHANGEDETAILS_IID_STR "5296f79e-15ea-40c3-8196-19cfa64d328c"

#define NS_IBASICCARDCHANGEDETAILS_IID \
  {0x5296f79e, 0x15ea, 0x40c3, \
    { 0x81, 0x96, 0x19, 0xcf, 0xa6, 0x4d, 0x32, 0x8c }}

class NS_NO_VTABLE nsIBasicCardChangeDetails : public nsIMethodChangeDetails {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IBASICCARDCHANGEDETAILS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIBasicCardChangeDetails;

  /* readonly attribute nsIPaymentAddress billingAddress; */
  NS_IMETHOD GetBillingAddress(nsIPaymentAddress **aBillingAddress) = 0;

  /* void initData (in nsIPaymentAddress billingAddress); */
  NS_IMETHOD InitData(nsIPaymentAddress *billingAddress) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIBasicCardChangeDetails, NS_IBASICCARDCHANGEDETAILS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIBASICCARDCHANGEDETAILS \
  NS_IMETHOD GetBillingAddress(nsIPaymentAddress **aBillingAddress) override; \
  NS_IMETHOD InitData(nsIPaymentAddress *billingAddress) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIBASICCARDCHANGEDETAILS \
  nsresult GetBillingAddress(nsIPaymentAddress **aBillingAddress); \
  nsresult InitData(nsIPaymentAddress *billingAddress); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIBASICCARDCHANGEDETAILS(_to) \
  NS_IMETHOD GetBillingAddress(nsIPaymentAddress **aBillingAddress) override { return _to GetBillingAddress(aBillingAddress); } \
  NS_IMETHOD InitData(nsIPaymentAddress *billingAddress) override { return _to InitData(billingAddress); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIBASICCARDCHANGEDETAILS(_to) \
  NS_IMETHOD GetBillingAddress(nsIPaymentAddress **aBillingAddress) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBillingAddress(aBillingAddress); } \
  NS_IMETHOD InitData(nsIPaymentAddress *billingAddress) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitData(billingAddress); } 

#define NS_GENERAL_RESPONSE_DATA_CID \
  { 0xb986773e, 0x2b30, 0x4ed2, { 0xb8, 0xfe, 0x6a, 0x96, 0x63, 0x1c, 0x80, 0x00 } }
#define NS_GENERAL_RESPONSE_DATA_CONTRACT_ID \
  "@mozilla.org/dom/payments/general-response-data;1"
#define NS_BASICCARD_RESPONSE_DATA_CID \
  { 0x0d55a5e6, 0xd185, 0x44f0, { 0xb9, 0x92, 0xa8, 0xe1, 0x32, 0x1e, 0x4b, 0xce } }
#define NS_BASICCARD_RESPONSE_DATA_CONTRACT_ID \
  "@mozilla.org/dom/payments/basiccard-response-data;1"
#define NS_PAYMENT_CANMAKE_ACTION_RESPONSE_CID \
  { 0x52fc3f9f, 0xc0cb, 0x4874, { 0xb3, 0xd4, 0xee, 0x4b, 0x6e, 0x9c, 0xbe, 0x9c } }
#define NS_PAYMENT_CANMAKE_ACTION_RESPONSE_CONTRACT_ID \
  "@mozilla.org/dom/payments/payment-canmake-action-response;1"
#define NS_PAYMENT_SHOW_ACTION_RESPONSE_CID \
  { 0x184385cb, 0x2d35, 0x4b99, { 0xa9, 0xa3, 0x7c, 0x78, 0x0b, 0xf6, 0x6b, 0x9b } }
#define NS_PAYMENT_SHOW_ACTION_RESPONSE_CONTRACT_ID \
  "@mozilla.org/dom/payments/payment-show-action-response;1"
#define NS_PAYMENT_ABORT_ACTION_RESPONSE_CID \
  { 0x8c72bcdb, 0x0c37, 0x4786, { 0xa9, 0xe5, 0x51, 0x0a, 0xfa, 0x2f, 0x8e, 0xde } }
#define NS_PAYMENT_ABORT_ACTION_RESPONSE_CONTRACT_ID \
  "@mozilla.org/dom/payments/payment-abort-action-response;1"
#define NS_PAYMENT_COMPLETE_ACTION_RESPONSE_CID \
  { 0x62c01e69, 0x9ca4, 0x4060, { 0x99, 0xe4, 0xb9, 0x5f, 0x62, 0x8c, 0x8e, 0x6d } }
#define NS_PAYMENT_COMPLETE_ACTION_RESPONSE_CONTRACT_ID \
  "@mozilla.org/dom/payments/payment-complete-action-response;1"
#define NS_GENERAL_CHANGE_DETAILS_CID \
  { 0xe031267e, 0xbec8, 0x4f3c, { 0xb0, 0xb1, 0x39, 0x6b, 0x77, 0xca, 0x26, 0x0c } }
#define NS_GENERAL_CHANGE_DETAILS_CONTRACT_ID \
  "@mozilla.org/dom/payments/general-change-details;1"
#define NS_BASICCARD_CHANGE_DETAILS_CID \
  { 0x5296f79e, 0x15ea, 0x40c3, { 0x81, 0x96, 0x19, 0xcf, 0xa6, 0x4d, 0x32, 0x8c } }
#define NS_BASICCARD_CHANGE_DETAILS_CONTRACT_ID \
  "@mozilla.org/dom/payments/basiccard-change-details;1"

#endif /* __gen_nsIPaymentActionResponse_h__ */
