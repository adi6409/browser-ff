/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/payments/nsIPaymentRequest.idl
 */

#ifndef __gen_nsIPaymentRequest_h__
#define __gen_nsIPaymentRequest_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIVariant_h__
#include "nsIVariant.h"
#endif

#ifndef __gen_nsIPrincipal_h__
#include "nsIPrincipal.h"
#endif

#include "js/Value.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIArray; /* forward declaration */


/* starting interface:    nsIPaymentMethodData */
#define NS_IPAYMENTMETHODDATA_IID_STR "2fe296cc-d917-4820-b492-aa42df23f9b4"

#define NS_IPAYMENTMETHODDATA_IID \
  {0x2fe296cc, 0xd917, 0x4820, \
    { 0xb4, 0x92, 0xaa, 0x42, 0xdf, 0x23, 0xf9, 0xb4 }}

class NS_NO_VTABLE nsIPaymentMethodData : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPAYMENTMETHODDATA_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPaymentMethodData;

  /* readonly attribute AString supportedMethods; */
  NS_IMETHOD GetSupportedMethods(nsAString& aSupportedMethods) = 0;

  /* [implicit_jscontext] readonly attribute jsval data; */
  NS_IMETHOD GetData(JSContext* cx, JS::MutableHandleValue aData) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPaymentMethodData, NS_IPAYMENTMETHODDATA_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPAYMENTMETHODDATA \
  NS_IMETHOD GetSupportedMethods(nsAString& aSupportedMethods) override; \
  NS_IMETHOD GetData(JSContext* cx, JS::MutableHandleValue aData) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPAYMENTMETHODDATA \
  nsresult GetSupportedMethods(nsAString& aSupportedMethods); \
  nsresult GetData(JSContext* cx, JS::MutableHandleValue aData); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPAYMENTMETHODDATA(_to) \
  NS_IMETHOD GetSupportedMethods(nsAString& aSupportedMethods) override { return _to GetSupportedMethods(aSupportedMethods); } \
  NS_IMETHOD GetData(JSContext* cx, JS::MutableHandleValue aData) override { return _to GetData(cx, aData); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPAYMENTMETHODDATA(_to) \
  NS_IMETHOD GetSupportedMethods(nsAString& aSupportedMethods) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSupportedMethods(aSupportedMethods); } \
  NS_IMETHOD GetData(JSContext* cx, JS::MutableHandleValue aData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetData(cx, aData); } 


/* starting interface:    nsIPaymentCurrencyAmount */
#define NS_IPAYMENTCURRENCYAMOUNT_IID_STR "d22a6f5f-767b-4fea-bf92-68b0b8003eba"

#define NS_IPAYMENTCURRENCYAMOUNT_IID \
  {0xd22a6f5f, 0x767b, 0x4fea, \
    { 0xbf, 0x92, 0x68, 0xb0, 0xb8, 0x00, 0x3e, 0xba }}

class NS_NO_VTABLE nsIPaymentCurrencyAmount : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPAYMENTCURRENCYAMOUNT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPaymentCurrencyAmount;

  /* readonly attribute AString currency; */
  NS_IMETHOD GetCurrency(nsAString& aCurrency) = 0;

  /* readonly attribute AString value; */
  NS_IMETHOD GetValue(nsAString& aValue) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPaymentCurrencyAmount, NS_IPAYMENTCURRENCYAMOUNT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPAYMENTCURRENCYAMOUNT \
  NS_IMETHOD GetCurrency(nsAString& aCurrency) override; \
  NS_IMETHOD GetValue(nsAString& aValue) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPAYMENTCURRENCYAMOUNT \
  nsresult GetCurrency(nsAString& aCurrency); \
  nsresult GetValue(nsAString& aValue); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPAYMENTCURRENCYAMOUNT(_to) \
  NS_IMETHOD GetCurrency(nsAString& aCurrency) override { return _to GetCurrency(aCurrency); } \
  NS_IMETHOD GetValue(nsAString& aValue) override { return _to GetValue(aValue); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPAYMENTCURRENCYAMOUNT(_to) \
  NS_IMETHOD GetCurrency(nsAString& aCurrency) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCurrency(aCurrency); } \
  NS_IMETHOD GetValue(nsAString& aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetValue(aValue); } 


/* starting interface:    nsIPaymentItem */
#define NS_IPAYMENTITEM_IID_STR "4f78a59f-b5ff-4fb5-ab48-3b37d0101b02"

#define NS_IPAYMENTITEM_IID \
  {0x4f78a59f, 0xb5ff, 0x4fb5, \
    { 0xab, 0x48, 0x3b, 0x37, 0xd0, 0x10, 0x1b, 0x02 }}

class NS_NO_VTABLE nsIPaymentItem : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPAYMENTITEM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPaymentItem;

  /* readonly attribute AString label; */
  NS_IMETHOD GetLabel(nsAString& aLabel) = 0;

  /* readonly attribute nsIPaymentCurrencyAmount amount; */
  NS_IMETHOD GetAmount(nsIPaymentCurrencyAmount **aAmount) = 0;

  /* readonly attribute boolean pending; */
  NS_IMETHOD GetPending(bool *aPending) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPaymentItem, NS_IPAYMENTITEM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPAYMENTITEM \
  NS_IMETHOD GetLabel(nsAString& aLabel) override; \
  NS_IMETHOD GetAmount(nsIPaymentCurrencyAmount **aAmount) override; \
  NS_IMETHOD GetPending(bool *aPending) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPAYMENTITEM \
  nsresult GetLabel(nsAString& aLabel); \
  nsresult GetAmount(nsIPaymentCurrencyAmount **aAmount); \
  nsresult GetPending(bool *aPending); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPAYMENTITEM(_to) \
  NS_IMETHOD GetLabel(nsAString& aLabel) override { return _to GetLabel(aLabel); } \
  NS_IMETHOD GetAmount(nsIPaymentCurrencyAmount **aAmount) override { return _to GetAmount(aAmount); } \
  NS_IMETHOD GetPending(bool *aPending) override { return _to GetPending(aPending); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPAYMENTITEM(_to) \
  NS_IMETHOD GetLabel(nsAString& aLabel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLabel(aLabel); } \
  NS_IMETHOD GetAmount(nsIPaymentCurrencyAmount **aAmount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAmount(aAmount); } \
  NS_IMETHOD GetPending(bool *aPending) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPending(aPending); } 


/* starting interface:    nsIPaymentDetailsModifier */
#define NS_IPAYMENTDETAILSMODIFIER_IID_STR "74259861-c318-40e8-b3d5-518e701bed80"

#define NS_IPAYMENTDETAILSMODIFIER_IID \
  {0x74259861, 0xc318, 0x40e8, \
    { 0xb3, 0xd5, 0x51, 0x8e, 0x70, 0x1b, 0xed, 0x80 }}

class NS_NO_VTABLE nsIPaymentDetailsModifier : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPAYMENTDETAILSMODIFIER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPaymentDetailsModifier;

  /* readonly attribute AString supportedMethods; */
  NS_IMETHOD GetSupportedMethods(nsAString& aSupportedMethods) = 0;

  /* readonly attribute nsIPaymentItem total; */
  NS_IMETHOD GetTotal(nsIPaymentItem **aTotal) = 0;

  /* readonly attribute nsIArray additionalDisplayItems; */
  NS_IMETHOD GetAdditionalDisplayItems(nsIArray **aAdditionalDisplayItems) = 0;

  /* [implicit_jscontext] readonly attribute jsval data; */
  NS_IMETHOD GetData(JSContext* cx, JS::MutableHandleValue aData) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPaymentDetailsModifier, NS_IPAYMENTDETAILSMODIFIER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPAYMENTDETAILSMODIFIER \
  NS_IMETHOD GetSupportedMethods(nsAString& aSupportedMethods) override; \
  NS_IMETHOD GetTotal(nsIPaymentItem **aTotal) override; \
  NS_IMETHOD GetAdditionalDisplayItems(nsIArray **aAdditionalDisplayItems) override; \
  NS_IMETHOD GetData(JSContext* cx, JS::MutableHandleValue aData) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPAYMENTDETAILSMODIFIER \
  nsresult GetSupportedMethods(nsAString& aSupportedMethods); \
  nsresult GetTotal(nsIPaymentItem **aTotal); \
  nsresult GetAdditionalDisplayItems(nsIArray **aAdditionalDisplayItems); \
  nsresult GetData(JSContext* cx, JS::MutableHandleValue aData); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPAYMENTDETAILSMODIFIER(_to) \
  NS_IMETHOD GetSupportedMethods(nsAString& aSupportedMethods) override { return _to GetSupportedMethods(aSupportedMethods); } \
  NS_IMETHOD GetTotal(nsIPaymentItem **aTotal) override { return _to GetTotal(aTotal); } \
  NS_IMETHOD GetAdditionalDisplayItems(nsIArray **aAdditionalDisplayItems) override { return _to GetAdditionalDisplayItems(aAdditionalDisplayItems); } \
  NS_IMETHOD GetData(JSContext* cx, JS::MutableHandleValue aData) override { return _to GetData(cx, aData); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPAYMENTDETAILSMODIFIER(_to) \
  NS_IMETHOD GetSupportedMethods(nsAString& aSupportedMethods) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSupportedMethods(aSupportedMethods); } \
  NS_IMETHOD GetTotal(nsIPaymentItem **aTotal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTotal(aTotal); } \
  NS_IMETHOD GetAdditionalDisplayItems(nsIArray **aAdditionalDisplayItems) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAdditionalDisplayItems(aAdditionalDisplayItems); } \
  NS_IMETHOD GetData(JSContext* cx, JS::MutableHandleValue aData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetData(cx, aData); } 


/* starting interface:    nsIPaymentShippingOption */
#define NS_IPAYMENTSHIPPINGOPTION_IID_STR "68341551-3605-4381-b936-41e830aa88fb"

#define NS_IPAYMENTSHIPPINGOPTION_IID \
  {0x68341551, 0x3605, 0x4381, \
    { 0xb9, 0x36, 0x41, 0xe8, 0x30, 0xaa, 0x88, 0xfb }}

class NS_NO_VTABLE nsIPaymentShippingOption : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPAYMENTSHIPPINGOPTION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPaymentShippingOption;

  /* readonly attribute AString id; */
  NS_IMETHOD GetId(nsAString& aId) = 0;

  /* readonly attribute AString label; */
  NS_IMETHOD GetLabel(nsAString& aLabel) = 0;

  /* readonly attribute nsIPaymentCurrencyAmount amount; */
  NS_IMETHOD GetAmount(nsIPaymentCurrencyAmount **aAmount) = 0;

  /* attribute boolean selected; */
  NS_IMETHOD GetSelected(bool *aSelected) = 0;
  NS_IMETHOD SetSelected(bool aSelected) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPaymentShippingOption, NS_IPAYMENTSHIPPINGOPTION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPAYMENTSHIPPINGOPTION \
  NS_IMETHOD GetId(nsAString& aId) override; \
  NS_IMETHOD GetLabel(nsAString& aLabel) override; \
  NS_IMETHOD GetAmount(nsIPaymentCurrencyAmount **aAmount) override; \
  NS_IMETHOD GetSelected(bool *aSelected) override; \
  NS_IMETHOD SetSelected(bool aSelected) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPAYMENTSHIPPINGOPTION \
  nsresult GetId(nsAString& aId); \
  nsresult GetLabel(nsAString& aLabel); \
  nsresult GetAmount(nsIPaymentCurrencyAmount **aAmount); \
  nsresult GetSelected(bool *aSelected); \
  nsresult SetSelected(bool aSelected); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPAYMENTSHIPPINGOPTION(_to) \
  NS_IMETHOD GetId(nsAString& aId) override { return _to GetId(aId); } \
  NS_IMETHOD GetLabel(nsAString& aLabel) override { return _to GetLabel(aLabel); } \
  NS_IMETHOD GetAmount(nsIPaymentCurrencyAmount **aAmount) override { return _to GetAmount(aAmount); } \
  NS_IMETHOD GetSelected(bool *aSelected) override { return _to GetSelected(aSelected); } \
  NS_IMETHOD SetSelected(bool aSelected) override { return _to SetSelected(aSelected); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPAYMENTSHIPPINGOPTION(_to) \
  NS_IMETHOD GetId(nsAString& aId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetId(aId); } \
  NS_IMETHOD GetLabel(nsAString& aLabel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLabel(aLabel); } \
  NS_IMETHOD GetAmount(nsIPaymentCurrencyAmount **aAmount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAmount(aAmount); } \
  NS_IMETHOD GetSelected(bool *aSelected) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelected(aSelected); } \
  NS_IMETHOD SetSelected(bool aSelected) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSelected(aSelected); } 


/* starting interface:    nsIPaymentDetails */
#define NS_IPAYMENTDETAILS_IID_STR "73a5a3f1-45b9-4605-a6e6-7aa60daa9039"

#define NS_IPAYMENTDETAILS_IID \
  {0x73a5a3f1, 0x45b9, 0x4605, \
    { 0xa6, 0xe6, 0x7a, 0xa6, 0x0d, 0xaa, 0x90, 0x39 }}

class NS_NO_VTABLE nsIPaymentDetails : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPAYMENTDETAILS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPaymentDetails;

  /* readonly attribute AString id; */
  NS_IMETHOD GetId(nsAString& aId) = 0;

  /* readonly attribute nsIPaymentItem totalItem; */
  NS_IMETHOD GetTotalItem(nsIPaymentItem **aTotalItem) = 0;

  /* readonly attribute nsIArray displayItems; */
  NS_IMETHOD GetDisplayItems(nsIArray **aDisplayItems) = 0;

  /* readonly attribute nsIArray shippingOptions; */
  NS_IMETHOD GetShippingOptions(nsIArray **aShippingOptions) = 0;

  /* readonly attribute nsIArray modifiers; */
  NS_IMETHOD GetModifiers(nsIArray **aModifiers) = 0;

  /* readonly attribute AString error; */
  NS_IMETHOD GetError(nsAString& aError) = 0;

  /* [implicit_jscontext] readonly attribute jsval shippingAddressErrors; */
  NS_IMETHOD GetShippingAddressErrors(JSContext* cx, JS::MutableHandleValue aShippingAddressErrors) = 0;

  /* [implicit_jscontext] readonly attribute jsval payerErrors; */
  NS_IMETHOD GetPayerErrors(JSContext* cx, JS::MutableHandleValue aPayerErrors) = 0;

  /* [implicit_jscontext] readonly attribute jsval paymentMethodErrors; */
  NS_IMETHOD GetPaymentMethodErrors(JSContext* cx, JS::MutableHandleValue aPaymentMethodErrors) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPaymentDetails, NS_IPAYMENTDETAILS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPAYMENTDETAILS \
  NS_IMETHOD GetId(nsAString& aId) override; \
  NS_IMETHOD GetTotalItem(nsIPaymentItem **aTotalItem) override; \
  NS_IMETHOD GetDisplayItems(nsIArray **aDisplayItems) override; \
  NS_IMETHOD GetShippingOptions(nsIArray **aShippingOptions) override; \
  NS_IMETHOD GetModifiers(nsIArray **aModifiers) override; \
  NS_IMETHOD GetError(nsAString& aError) override; \
  NS_IMETHOD GetShippingAddressErrors(JSContext* cx, JS::MutableHandleValue aShippingAddressErrors) override; \
  NS_IMETHOD GetPayerErrors(JSContext* cx, JS::MutableHandleValue aPayerErrors) override; \
  NS_IMETHOD GetPaymentMethodErrors(JSContext* cx, JS::MutableHandleValue aPaymentMethodErrors) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPAYMENTDETAILS \
  nsresult GetId(nsAString& aId); \
  nsresult GetTotalItem(nsIPaymentItem **aTotalItem); \
  nsresult GetDisplayItems(nsIArray **aDisplayItems); \
  nsresult GetShippingOptions(nsIArray **aShippingOptions); \
  nsresult GetModifiers(nsIArray **aModifiers); \
  nsresult GetError(nsAString& aError); \
  nsresult GetShippingAddressErrors(JSContext* cx, JS::MutableHandleValue aShippingAddressErrors); \
  nsresult GetPayerErrors(JSContext* cx, JS::MutableHandleValue aPayerErrors); \
  nsresult GetPaymentMethodErrors(JSContext* cx, JS::MutableHandleValue aPaymentMethodErrors); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPAYMENTDETAILS(_to) \
  NS_IMETHOD GetId(nsAString& aId) override { return _to GetId(aId); } \
  NS_IMETHOD GetTotalItem(nsIPaymentItem **aTotalItem) override { return _to GetTotalItem(aTotalItem); } \
  NS_IMETHOD GetDisplayItems(nsIArray **aDisplayItems) override { return _to GetDisplayItems(aDisplayItems); } \
  NS_IMETHOD GetShippingOptions(nsIArray **aShippingOptions) override { return _to GetShippingOptions(aShippingOptions); } \
  NS_IMETHOD GetModifiers(nsIArray **aModifiers) override { return _to GetModifiers(aModifiers); } \
  NS_IMETHOD GetError(nsAString& aError) override { return _to GetError(aError); } \
  NS_IMETHOD GetShippingAddressErrors(JSContext* cx, JS::MutableHandleValue aShippingAddressErrors) override { return _to GetShippingAddressErrors(cx, aShippingAddressErrors); } \
  NS_IMETHOD GetPayerErrors(JSContext* cx, JS::MutableHandleValue aPayerErrors) override { return _to GetPayerErrors(cx, aPayerErrors); } \
  NS_IMETHOD GetPaymentMethodErrors(JSContext* cx, JS::MutableHandleValue aPaymentMethodErrors) override { return _to GetPaymentMethodErrors(cx, aPaymentMethodErrors); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPAYMENTDETAILS(_to) \
  NS_IMETHOD GetId(nsAString& aId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetId(aId); } \
  NS_IMETHOD GetTotalItem(nsIPaymentItem **aTotalItem) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTotalItem(aTotalItem); } \
  NS_IMETHOD GetDisplayItems(nsIArray **aDisplayItems) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDisplayItems(aDisplayItems); } \
  NS_IMETHOD GetShippingOptions(nsIArray **aShippingOptions) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetShippingOptions(aShippingOptions); } \
  NS_IMETHOD GetModifiers(nsIArray **aModifiers) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetModifiers(aModifiers); } \
  NS_IMETHOD GetError(nsAString& aError) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetError(aError); } \
  NS_IMETHOD GetShippingAddressErrors(JSContext* cx, JS::MutableHandleValue aShippingAddressErrors) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetShippingAddressErrors(cx, aShippingAddressErrors); } \
  NS_IMETHOD GetPayerErrors(JSContext* cx, JS::MutableHandleValue aPayerErrors) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPayerErrors(cx, aPayerErrors); } \
  NS_IMETHOD GetPaymentMethodErrors(JSContext* cx, JS::MutableHandleValue aPaymentMethodErrors) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPaymentMethodErrors(cx, aPaymentMethodErrors); } 


/* starting interface:    nsIPaymentOptions */
#define NS_IPAYMENTOPTIONS_IID_STR "d53f9f20-138e-47cc-9fd5-db16a3f6d301"

#define NS_IPAYMENTOPTIONS_IID \
  {0xd53f9f20, 0x138e, 0x47cc, \
    { 0x9f, 0xd5, 0xdb, 0x16, 0xa3, 0xf6, 0xd3, 0x01 }}

class NS_NO_VTABLE nsIPaymentOptions : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPAYMENTOPTIONS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPaymentOptions;

  /* readonly attribute boolean requestPayerName; */
  NS_IMETHOD GetRequestPayerName(bool *aRequestPayerName) = 0;

  /* readonly attribute boolean requestPayerEmail; */
  NS_IMETHOD GetRequestPayerEmail(bool *aRequestPayerEmail) = 0;

  /* readonly attribute boolean requestPayerPhone; */
  NS_IMETHOD GetRequestPayerPhone(bool *aRequestPayerPhone) = 0;

  /* readonly attribute boolean requestShipping; */
  NS_IMETHOD GetRequestShipping(bool *aRequestShipping) = 0;

  /* readonly attribute boolean requestBillingAddress; */
  NS_IMETHOD GetRequestBillingAddress(bool *aRequestBillingAddress) = 0;

  /* readonly attribute AString shippingType; */
  NS_IMETHOD GetShippingType(nsAString& aShippingType) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPaymentOptions, NS_IPAYMENTOPTIONS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPAYMENTOPTIONS \
  NS_IMETHOD GetRequestPayerName(bool *aRequestPayerName) override; \
  NS_IMETHOD GetRequestPayerEmail(bool *aRequestPayerEmail) override; \
  NS_IMETHOD GetRequestPayerPhone(bool *aRequestPayerPhone) override; \
  NS_IMETHOD GetRequestShipping(bool *aRequestShipping) override; \
  NS_IMETHOD GetRequestBillingAddress(bool *aRequestBillingAddress) override; \
  NS_IMETHOD GetShippingType(nsAString& aShippingType) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPAYMENTOPTIONS \
  nsresult GetRequestPayerName(bool *aRequestPayerName); \
  nsresult GetRequestPayerEmail(bool *aRequestPayerEmail); \
  nsresult GetRequestPayerPhone(bool *aRequestPayerPhone); \
  nsresult GetRequestShipping(bool *aRequestShipping); \
  nsresult GetRequestBillingAddress(bool *aRequestBillingAddress); \
  nsresult GetShippingType(nsAString& aShippingType); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPAYMENTOPTIONS(_to) \
  NS_IMETHOD GetRequestPayerName(bool *aRequestPayerName) override { return _to GetRequestPayerName(aRequestPayerName); } \
  NS_IMETHOD GetRequestPayerEmail(bool *aRequestPayerEmail) override { return _to GetRequestPayerEmail(aRequestPayerEmail); } \
  NS_IMETHOD GetRequestPayerPhone(bool *aRequestPayerPhone) override { return _to GetRequestPayerPhone(aRequestPayerPhone); } \
  NS_IMETHOD GetRequestShipping(bool *aRequestShipping) override { return _to GetRequestShipping(aRequestShipping); } \
  NS_IMETHOD GetRequestBillingAddress(bool *aRequestBillingAddress) override { return _to GetRequestBillingAddress(aRequestBillingAddress); } \
  NS_IMETHOD GetShippingType(nsAString& aShippingType) override { return _to GetShippingType(aShippingType); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPAYMENTOPTIONS(_to) \
  NS_IMETHOD GetRequestPayerName(bool *aRequestPayerName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRequestPayerName(aRequestPayerName); } \
  NS_IMETHOD GetRequestPayerEmail(bool *aRequestPayerEmail) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRequestPayerEmail(aRequestPayerEmail); } \
  NS_IMETHOD GetRequestPayerPhone(bool *aRequestPayerPhone) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRequestPayerPhone(aRequestPayerPhone); } \
  NS_IMETHOD GetRequestShipping(bool *aRequestShipping) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRequestShipping(aRequestShipping); } \
  NS_IMETHOD GetRequestBillingAddress(bool *aRequestBillingAddress) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRequestBillingAddress(aRequestBillingAddress); } \
  NS_IMETHOD GetShippingType(nsAString& aShippingType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetShippingType(aShippingType); } 


/* starting interface:    nsIPaymentRequest */
#define NS_IPAYMENTREQUEST_IID_STR "2fa36783-d684-4487-b7a8-9def6ae3128f"

#define NS_IPAYMENTREQUEST_IID \
  {0x2fa36783, 0xd684, 0x4487, \
    { 0xb7, 0xa8, 0x9d, 0xef, 0x6a, 0xe3, 0x12, 0x8f }}

class NS_NO_VTABLE nsIPaymentRequest : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPAYMENTREQUEST_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPaymentRequest;

  /* readonly attribute uint64_t topOuterWindowId; */
  NS_IMETHOD GetTopOuterWindowId(uint64_t *aTopOuterWindowId) = 0;

  /* readonly attribute nsIPrincipal topLevelPrincipal; */
  NS_IMETHOD GetTopLevelPrincipal(nsIPrincipal **aTopLevelPrincipal) = 0;

  /* readonly attribute AString requestId; */
  NS_IMETHOD GetRequestId(nsAString& aRequestId) = 0;

  /* readonly attribute AString completeStatus; */
  NS_IMETHOD GetCompleteStatus(nsAString& aCompleteStatus) = 0;

  /* readonly attribute nsIArray paymentMethods; */
  NS_IMETHOD GetPaymentMethods(nsIArray **aPaymentMethods) = 0;

  /* readonly attribute nsIPaymentDetails paymentDetails; */
  NS_IMETHOD GetPaymentDetails(nsIPaymentDetails **aPaymentDetails) = 0;

  /* readonly attribute nsIPaymentOptions paymentOptions; */
  NS_IMETHOD GetPaymentOptions(nsIPaymentOptions **aPaymentOptions) = 0;

  /* readonly attribute AString shippingOption; */
  NS_IMETHOD GetShippingOption(nsAString& aShippingOption) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPaymentRequest, NS_IPAYMENTREQUEST_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPAYMENTREQUEST \
  NS_IMETHOD GetTopOuterWindowId(uint64_t *aTopOuterWindowId) override; \
  NS_IMETHOD GetTopLevelPrincipal(nsIPrincipal **aTopLevelPrincipal) override; \
  NS_IMETHOD GetRequestId(nsAString& aRequestId) override; \
  NS_IMETHOD GetCompleteStatus(nsAString& aCompleteStatus) override; \
  NS_IMETHOD GetPaymentMethods(nsIArray **aPaymentMethods) override; \
  NS_IMETHOD GetPaymentDetails(nsIPaymentDetails **aPaymentDetails) override; \
  NS_IMETHOD GetPaymentOptions(nsIPaymentOptions **aPaymentOptions) override; \
  NS_IMETHOD GetShippingOption(nsAString& aShippingOption) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPAYMENTREQUEST \
  nsresult GetTopOuterWindowId(uint64_t *aTopOuterWindowId); \
  nsresult GetTopLevelPrincipal(nsIPrincipal **aTopLevelPrincipal); \
  nsresult GetRequestId(nsAString& aRequestId); \
  nsresult GetCompleteStatus(nsAString& aCompleteStatus); \
  nsresult GetPaymentMethods(nsIArray **aPaymentMethods); \
  nsresult GetPaymentDetails(nsIPaymentDetails **aPaymentDetails); \
  nsresult GetPaymentOptions(nsIPaymentOptions **aPaymentOptions); \
  nsresult GetShippingOption(nsAString& aShippingOption); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPAYMENTREQUEST(_to) \
  NS_IMETHOD GetTopOuterWindowId(uint64_t *aTopOuterWindowId) override { return _to GetTopOuterWindowId(aTopOuterWindowId); } \
  NS_IMETHOD GetTopLevelPrincipal(nsIPrincipal **aTopLevelPrincipal) override { return _to GetTopLevelPrincipal(aTopLevelPrincipal); } \
  NS_IMETHOD GetRequestId(nsAString& aRequestId) override { return _to GetRequestId(aRequestId); } \
  NS_IMETHOD GetCompleteStatus(nsAString& aCompleteStatus) override { return _to GetCompleteStatus(aCompleteStatus); } \
  NS_IMETHOD GetPaymentMethods(nsIArray **aPaymentMethods) override { return _to GetPaymentMethods(aPaymentMethods); } \
  NS_IMETHOD GetPaymentDetails(nsIPaymentDetails **aPaymentDetails) override { return _to GetPaymentDetails(aPaymentDetails); } \
  NS_IMETHOD GetPaymentOptions(nsIPaymentOptions **aPaymentOptions) override { return _to GetPaymentOptions(aPaymentOptions); } \
  NS_IMETHOD GetShippingOption(nsAString& aShippingOption) override { return _to GetShippingOption(aShippingOption); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPAYMENTREQUEST(_to) \
  NS_IMETHOD GetTopOuterWindowId(uint64_t *aTopOuterWindowId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTopOuterWindowId(aTopOuterWindowId); } \
  NS_IMETHOD GetTopLevelPrincipal(nsIPrincipal **aTopLevelPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTopLevelPrincipal(aTopLevelPrincipal); } \
  NS_IMETHOD GetRequestId(nsAString& aRequestId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRequestId(aRequestId); } \
  NS_IMETHOD GetCompleteStatus(nsAString& aCompleteStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCompleteStatus(aCompleteStatus); } \
  NS_IMETHOD GetPaymentMethods(nsIArray **aPaymentMethods) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPaymentMethods(aPaymentMethods); } \
  NS_IMETHOD GetPaymentDetails(nsIPaymentDetails **aPaymentDetails) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPaymentDetails(aPaymentDetails); } \
  NS_IMETHOD GetPaymentOptions(nsIPaymentOptions **aPaymentOptions) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPaymentOptions(aPaymentOptions); } \
  NS_IMETHOD GetShippingOption(nsAString& aShippingOption) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetShippingOption(aShippingOption); } 


#endif /* __gen_nsIPaymentRequest_h__ */
