/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/payments/nsIPaymentAddress.idl
 */

#ifndef __gen_nsIPaymentAddress_h__
#define __gen_nsIPaymentAddress_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIArray; /* forward declaration */


/* starting interface:    nsIPaymentAddress */
#define NS_IPAYMENTADDRESS_IID_STR "49a02241-7e48-477a-9345-9f246925dcb3"

#define NS_IPAYMENTADDRESS_IID \
  {0x49a02241, 0x7e48, 0x477a, \
    { 0x93, 0x45, 0x9f, 0x24, 0x69, 0x25, 0xdc, 0xb3 }}

class NS_NO_VTABLE nsIPaymentAddress : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPAYMENTADDRESS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPaymentAddress;

  /* readonly attribute AString country; */
  NS_IMETHOD GetCountry(nsAString& aCountry) = 0;

  /* readonly attribute nsIArray addressLine; */
  NS_IMETHOD GetAddressLine(nsIArray **aAddressLine) = 0;

  /* readonly attribute AString region; */
  NS_IMETHOD GetRegion(nsAString& aRegion) = 0;

  /* readonly attribute AString regionCode; */
  NS_IMETHOD GetRegionCode(nsAString& aRegionCode) = 0;

  /* readonly attribute AString city; */
  NS_IMETHOD GetCity(nsAString& aCity) = 0;

  /* readonly attribute AString dependentLocality; */
  NS_IMETHOD GetDependentLocality(nsAString& aDependentLocality) = 0;

  /* readonly attribute AString postalCode; */
  NS_IMETHOD GetPostalCode(nsAString& aPostalCode) = 0;

  /* readonly attribute AString sortingCode; */
  NS_IMETHOD GetSortingCode(nsAString& aSortingCode) = 0;

  /* readonly attribute AString organization; */
  NS_IMETHOD GetOrganization(nsAString& aOrganization) = 0;

  /* readonly attribute AString recipient; */
  NS_IMETHOD GetRecipient(nsAString& aRecipient) = 0;

  /* readonly attribute AString phone; */
  NS_IMETHOD GetPhone(nsAString& aPhone) = 0;

  /* void init (in AString aCountry, in nsIArray aAddressLine, in AString aRegion, in AString aRegionCode, in AString aCity, in AString aDependentLocality, in AString aPostalCode, in AString aSortingCode, in AString aOrganization, in AString aRecipient, in AString aPhone); */
  NS_IMETHOD Init(const nsAString& aCountry, nsIArray *aAddressLine, const nsAString& aRegion, const nsAString& aRegionCode, const nsAString& aCity, const nsAString& aDependentLocality, const nsAString& aPostalCode, const nsAString& aSortingCode, const nsAString& aOrganization, const nsAString& aRecipient, const nsAString& aPhone) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPaymentAddress, NS_IPAYMENTADDRESS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPAYMENTADDRESS \
  NS_IMETHOD GetCountry(nsAString& aCountry) override; \
  NS_IMETHOD GetAddressLine(nsIArray **aAddressLine) override; \
  NS_IMETHOD GetRegion(nsAString& aRegion) override; \
  NS_IMETHOD GetRegionCode(nsAString& aRegionCode) override; \
  NS_IMETHOD GetCity(nsAString& aCity) override; \
  NS_IMETHOD GetDependentLocality(nsAString& aDependentLocality) override; \
  NS_IMETHOD GetPostalCode(nsAString& aPostalCode) override; \
  NS_IMETHOD GetSortingCode(nsAString& aSortingCode) override; \
  NS_IMETHOD GetOrganization(nsAString& aOrganization) override; \
  NS_IMETHOD GetRecipient(nsAString& aRecipient) override; \
  NS_IMETHOD GetPhone(nsAString& aPhone) override; \
  NS_IMETHOD Init(const nsAString& aCountry, nsIArray *aAddressLine, const nsAString& aRegion, const nsAString& aRegionCode, const nsAString& aCity, const nsAString& aDependentLocality, const nsAString& aPostalCode, const nsAString& aSortingCode, const nsAString& aOrganization, const nsAString& aRecipient, const nsAString& aPhone) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPAYMENTADDRESS \
  nsresult GetCountry(nsAString& aCountry); \
  nsresult GetAddressLine(nsIArray **aAddressLine); \
  nsresult GetRegion(nsAString& aRegion); \
  nsresult GetRegionCode(nsAString& aRegionCode); \
  nsresult GetCity(nsAString& aCity); \
  nsresult GetDependentLocality(nsAString& aDependentLocality); \
  nsresult GetPostalCode(nsAString& aPostalCode); \
  nsresult GetSortingCode(nsAString& aSortingCode); \
  nsresult GetOrganization(nsAString& aOrganization); \
  nsresult GetRecipient(nsAString& aRecipient); \
  nsresult GetPhone(nsAString& aPhone); \
  nsresult Init(const nsAString& aCountry, nsIArray *aAddressLine, const nsAString& aRegion, const nsAString& aRegionCode, const nsAString& aCity, const nsAString& aDependentLocality, const nsAString& aPostalCode, const nsAString& aSortingCode, const nsAString& aOrganization, const nsAString& aRecipient, const nsAString& aPhone); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPAYMENTADDRESS(_to) \
  NS_IMETHOD GetCountry(nsAString& aCountry) override { return _to GetCountry(aCountry); } \
  NS_IMETHOD GetAddressLine(nsIArray **aAddressLine) override { return _to GetAddressLine(aAddressLine); } \
  NS_IMETHOD GetRegion(nsAString& aRegion) override { return _to GetRegion(aRegion); } \
  NS_IMETHOD GetRegionCode(nsAString& aRegionCode) override { return _to GetRegionCode(aRegionCode); } \
  NS_IMETHOD GetCity(nsAString& aCity) override { return _to GetCity(aCity); } \
  NS_IMETHOD GetDependentLocality(nsAString& aDependentLocality) override { return _to GetDependentLocality(aDependentLocality); } \
  NS_IMETHOD GetPostalCode(nsAString& aPostalCode) override { return _to GetPostalCode(aPostalCode); } \
  NS_IMETHOD GetSortingCode(nsAString& aSortingCode) override { return _to GetSortingCode(aSortingCode); } \
  NS_IMETHOD GetOrganization(nsAString& aOrganization) override { return _to GetOrganization(aOrganization); } \
  NS_IMETHOD GetRecipient(nsAString& aRecipient) override { return _to GetRecipient(aRecipient); } \
  NS_IMETHOD GetPhone(nsAString& aPhone) override { return _to GetPhone(aPhone); } \
  NS_IMETHOD Init(const nsAString& aCountry, nsIArray *aAddressLine, const nsAString& aRegion, const nsAString& aRegionCode, const nsAString& aCity, const nsAString& aDependentLocality, const nsAString& aPostalCode, const nsAString& aSortingCode, const nsAString& aOrganization, const nsAString& aRecipient, const nsAString& aPhone) override { return _to Init(aCountry, aAddressLine, aRegion, aRegionCode, aCity, aDependentLocality, aPostalCode, aSortingCode, aOrganization, aRecipient, aPhone); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPAYMENTADDRESS(_to) \
  NS_IMETHOD GetCountry(nsAString& aCountry) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCountry(aCountry); } \
  NS_IMETHOD GetAddressLine(nsIArray **aAddressLine) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAddressLine(aAddressLine); } \
  NS_IMETHOD GetRegion(nsAString& aRegion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRegion(aRegion); } \
  NS_IMETHOD GetRegionCode(nsAString& aRegionCode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRegionCode(aRegionCode); } \
  NS_IMETHOD GetCity(nsAString& aCity) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCity(aCity); } \
  NS_IMETHOD GetDependentLocality(nsAString& aDependentLocality) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDependentLocality(aDependentLocality); } \
  NS_IMETHOD GetPostalCode(nsAString& aPostalCode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPostalCode(aPostalCode); } \
  NS_IMETHOD GetSortingCode(nsAString& aSortingCode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSortingCode(aSortingCode); } \
  NS_IMETHOD GetOrganization(nsAString& aOrganization) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOrganization(aOrganization); } \
  NS_IMETHOD GetRecipient(nsAString& aRecipient) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRecipient(aRecipient); } \
  NS_IMETHOD GetPhone(nsAString& aPhone) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPhone(aPhone); } \
  NS_IMETHOD Init(const nsAString& aCountry, nsIArray *aAddressLine, const nsAString& aRegion, const nsAString& aRegionCode, const nsAString& aCity, const nsAString& aDependentLocality, const nsAString& aPostalCode, const nsAString& aSortingCode, const nsAString& aOrganization, const nsAString& aRecipient, const nsAString& aPhone) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aCountry, aAddressLine, aRegion, aRegionCode, aCity, aDependentLocality, aPostalCode, aSortingCode, aOrganization, aRecipient, aPhone); } 

#define NS_PAYMENT_ADDRESS_CID \
  { 0x49a02241, 0x7e48, 0x477a, { 0x93, 0x45, 0x9f, 0x24, 0x69, 0x25, 0xdc, 0xb3 } }
#define NS_PAYMENT_ADDRESS_CONTRACT_ID \
  "@mozilla.org/dom/payments/payment-address;1"

#endif /* __gen_nsIPaymentAddress_h__ */
