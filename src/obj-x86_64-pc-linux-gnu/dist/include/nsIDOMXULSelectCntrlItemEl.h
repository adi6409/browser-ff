/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/xul/nsIDOMXULSelectCntrlItemEl.idl
 */

#ifndef __gen_nsIDOMXULSelectCntrlItemEl_h__
#define __gen_nsIDOMXULSelectCntrlItemEl_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIDOMXULSelectControlElement; /* forward declaration */

namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIDOMXULSelectControlItemElement */
#define NS_IDOMXULSELECTCONTROLITEMELEMENT_IID_STR "5c6be58f-17df-4750-88a5-4a59ac28adc9"

#define NS_IDOMXULSELECTCONTROLITEMELEMENT_IID \
  {0x5c6be58f, 0x17df, 0x4750, \
    { 0x88, 0xa5, 0x4a, 0x59, 0xac, 0x28, 0xad, 0xc9 }}

class NS_NO_VTABLE nsIDOMXULSelectControlItemElement : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOMXULSELECTCONTROLITEMELEMENT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDOMXULSelectControlItemElement;

  /* attribute boolean disabled; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDisabled(bool *aDisabled) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDisabled(bool aDisabled) = 0;

  /* attribute AString crop; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCrop(nsAString& aCrop) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCrop(const nsAString& aCrop) = 0;

  /* attribute AString image; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetImage(nsAString& aImage) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetImage(const nsAString& aImage) = 0;

  /* attribute AString label; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLabel(nsAString& aLabel) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetLabel(const nsAString& aLabel) = 0;

  /* attribute AString accessKey; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAccessKey(nsAString& aAccessKey) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetAccessKey(const nsAString& aAccessKey) = 0;

  /* attribute AString command; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCommand(nsAString& aCommand) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCommand(const nsAString& aCommand) = 0;

  /* attribute AString value; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetValue(nsAString& aValue) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetValue(const nsAString& aValue) = 0;

  /* readonly attribute boolean selected; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSelected(bool *aSelected) = 0;

  /* readonly attribute Element control; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetControl(mozilla::dom::Element **aControl) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDOMXULSelectControlItemElement, NS_IDOMXULSELECTCONTROLITEMELEMENT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOMXULSELECTCONTROLITEMELEMENT \
  NS_IMETHOD GetDisabled(bool *aDisabled) override; \
  NS_IMETHOD SetDisabled(bool aDisabled) override; \
  NS_IMETHOD GetCrop(nsAString& aCrop) override; \
  NS_IMETHOD SetCrop(const nsAString& aCrop) override; \
  NS_IMETHOD GetImage(nsAString& aImage) override; \
  NS_IMETHOD SetImage(const nsAString& aImage) override; \
  NS_IMETHOD GetLabel(nsAString& aLabel) override; \
  NS_IMETHOD SetLabel(const nsAString& aLabel) override; \
  NS_IMETHOD GetAccessKey(nsAString& aAccessKey) override; \
  NS_IMETHOD SetAccessKey(const nsAString& aAccessKey) override; \
  NS_IMETHOD GetCommand(nsAString& aCommand) override; \
  NS_IMETHOD SetCommand(const nsAString& aCommand) override; \
  NS_IMETHOD GetValue(nsAString& aValue) override; \
  NS_IMETHOD SetValue(const nsAString& aValue) override; \
  NS_IMETHOD GetSelected(bool *aSelected) override; \
  NS_IMETHOD GetControl(mozilla::dom::Element **aControl) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOMXULSELECTCONTROLITEMELEMENT \
  nsresult GetDisabled(bool *aDisabled); \
  nsresult SetDisabled(bool aDisabled); \
  nsresult GetCrop(nsAString& aCrop); \
  nsresult SetCrop(const nsAString& aCrop); \
  nsresult GetImage(nsAString& aImage); \
  nsresult SetImage(const nsAString& aImage); \
  nsresult GetLabel(nsAString& aLabel); \
  nsresult SetLabel(const nsAString& aLabel); \
  nsresult GetAccessKey(nsAString& aAccessKey); \
  nsresult SetAccessKey(const nsAString& aAccessKey); \
  nsresult GetCommand(nsAString& aCommand); \
  nsresult SetCommand(const nsAString& aCommand); \
  nsresult GetValue(nsAString& aValue); \
  nsresult SetValue(const nsAString& aValue); \
  nsresult GetSelected(bool *aSelected); \
  nsresult GetControl(mozilla::dom::Element **aControl); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOMXULSELECTCONTROLITEMELEMENT(_to) \
  NS_IMETHOD GetDisabled(bool *aDisabled) override { return _to GetDisabled(aDisabled); } \
  NS_IMETHOD SetDisabled(bool aDisabled) override { return _to SetDisabled(aDisabled); } \
  NS_IMETHOD GetCrop(nsAString& aCrop) override { return _to GetCrop(aCrop); } \
  NS_IMETHOD SetCrop(const nsAString& aCrop) override { return _to SetCrop(aCrop); } \
  NS_IMETHOD GetImage(nsAString& aImage) override { return _to GetImage(aImage); } \
  NS_IMETHOD SetImage(const nsAString& aImage) override { return _to SetImage(aImage); } \
  NS_IMETHOD GetLabel(nsAString& aLabel) override { return _to GetLabel(aLabel); } \
  NS_IMETHOD SetLabel(const nsAString& aLabel) override { return _to SetLabel(aLabel); } \
  NS_IMETHOD GetAccessKey(nsAString& aAccessKey) override { return _to GetAccessKey(aAccessKey); } \
  NS_IMETHOD SetAccessKey(const nsAString& aAccessKey) override { return _to SetAccessKey(aAccessKey); } \
  NS_IMETHOD GetCommand(nsAString& aCommand) override { return _to GetCommand(aCommand); } \
  NS_IMETHOD SetCommand(const nsAString& aCommand) override { return _to SetCommand(aCommand); } \
  NS_IMETHOD GetValue(nsAString& aValue) override { return _to GetValue(aValue); } \
  NS_IMETHOD SetValue(const nsAString& aValue) override { return _to SetValue(aValue); } \
  NS_IMETHOD GetSelected(bool *aSelected) override { return _to GetSelected(aSelected); } \
  NS_IMETHOD GetControl(mozilla::dom::Element **aControl) override { return _to GetControl(aControl); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOMXULSELECTCONTROLITEMELEMENT(_to) \
  NS_IMETHOD GetDisabled(bool *aDisabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDisabled(aDisabled); } \
  NS_IMETHOD SetDisabled(bool aDisabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDisabled(aDisabled); } \
  NS_IMETHOD GetCrop(nsAString& aCrop) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCrop(aCrop); } \
  NS_IMETHOD SetCrop(const nsAString& aCrop) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCrop(aCrop); } \
  NS_IMETHOD GetImage(nsAString& aImage) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetImage(aImage); } \
  NS_IMETHOD SetImage(const nsAString& aImage) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetImage(aImage); } \
  NS_IMETHOD GetLabel(nsAString& aLabel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLabel(aLabel); } \
  NS_IMETHOD SetLabel(const nsAString& aLabel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLabel(aLabel); } \
  NS_IMETHOD GetAccessKey(nsAString& aAccessKey) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAccessKey(aAccessKey); } \
  NS_IMETHOD SetAccessKey(const nsAString& aAccessKey) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAccessKey(aAccessKey); } \
  NS_IMETHOD GetCommand(nsAString& aCommand) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCommand(aCommand); } \
  NS_IMETHOD SetCommand(const nsAString& aCommand) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCommand(aCommand); } \
  NS_IMETHOD GetValue(nsAString& aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetValue(aValue); } \
  NS_IMETHOD SetValue(const nsAString& aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetValue(aValue); } \
  NS_IMETHOD GetSelected(bool *aSelected) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelected(aSelected); } \
  NS_IMETHOD GetControl(mozilla::dom::Element **aControl) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetControl(aControl); } 


#endif /* __gen_nsIDOMXULSelectCntrlItemEl_h__ */
