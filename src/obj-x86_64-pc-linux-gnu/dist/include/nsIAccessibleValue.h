/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleValue.idl
 */

#ifndef __gen_nsIAccessibleValue_h__
#define __gen_nsIAccessibleValue_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIAccessibleValue */
#define NS_IACCESSIBLEVALUE_IID_STR "42a1e1dc-58cf-419d-bff0-ed3314c70016"

#define NS_IACCESSIBLEVALUE_IID \
  {0x42a1e1dc, 0x58cf, 0x419d, \
    { 0xbf, 0xf0, 0xed, 0x33, 0x14, 0xc7, 0x00, 0x16 }}

class NS_NO_VTABLE nsIAccessibleValue : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IACCESSIBLEVALUE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAccessibleValue;

  /* readonly attribute double maximumValue; */
  NS_IMETHOD GetMaximumValue(double *aMaximumValue) = 0;

  /* readonly attribute double minimumValue; */
  NS_IMETHOD GetMinimumValue(double *aMinimumValue) = 0;

  /* attribute double currentValue; */
  NS_IMETHOD GetCurrentValue(double *aCurrentValue) = 0;
  NS_IMETHOD SetCurrentValue(double aCurrentValue) = 0;

  /* readonly attribute double minimumIncrement; */
  NS_IMETHOD GetMinimumIncrement(double *aMinimumIncrement) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAccessibleValue, NS_IACCESSIBLEVALUE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIACCESSIBLEVALUE \
  NS_IMETHOD GetMaximumValue(double *aMaximumValue) override; \
  NS_IMETHOD GetMinimumValue(double *aMinimumValue) override; \
  NS_IMETHOD GetCurrentValue(double *aCurrentValue) override; \
  NS_IMETHOD SetCurrentValue(double aCurrentValue) override; \
  NS_IMETHOD GetMinimumIncrement(double *aMinimumIncrement) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIACCESSIBLEVALUE \
  nsresult GetMaximumValue(double *aMaximumValue); \
  nsresult GetMinimumValue(double *aMinimumValue); \
  nsresult GetCurrentValue(double *aCurrentValue); \
  nsresult SetCurrentValue(double aCurrentValue); \
  nsresult GetMinimumIncrement(double *aMinimumIncrement); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIACCESSIBLEVALUE(_to) \
  NS_IMETHOD GetMaximumValue(double *aMaximumValue) override { return _to GetMaximumValue(aMaximumValue); } \
  NS_IMETHOD GetMinimumValue(double *aMinimumValue) override { return _to GetMinimumValue(aMinimumValue); } \
  NS_IMETHOD GetCurrentValue(double *aCurrentValue) override { return _to GetCurrentValue(aCurrentValue); } \
  NS_IMETHOD SetCurrentValue(double aCurrentValue) override { return _to SetCurrentValue(aCurrentValue); } \
  NS_IMETHOD GetMinimumIncrement(double *aMinimumIncrement) override { return _to GetMinimumIncrement(aMinimumIncrement); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIACCESSIBLEVALUE(_to) \
  NS_IMETHOD GetMaximumValue(double *aMaximumValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMaximumValue(aMaximumValue); } \
  NS_IMETHOD GetMinimumValue(double *aMinimumValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMinimumValue(aMinimumValue); } \
  NS_IMETHOD GetCurrentValue(double *aCurrentValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCurrentValue(aCurrentValue); } \
  NS_IMETHOD SetCurrentValue(double aCurrentValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCurrentValue(aCurrentValue); } \
  NS_IMETHOD GetMinimumIncrement(double *aMinimumIncrement) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMinimumIncrement(aMinimumIncrement); } 


#endif /* __gen_nsIAccessibleValue_h__ */
