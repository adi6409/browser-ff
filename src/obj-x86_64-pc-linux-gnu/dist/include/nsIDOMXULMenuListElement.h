/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/xul/nsIDOMXULMenuListElement.idl
 */

#ifndef __gen_nsIDOMXULMenuListElement_h__
#define __gen_nsIDOMXULMenuListElement_h__


#ifndef __gen_nsIDOMXULSelectCntrlEl_h__
#include "nsIDOMXULSelectCntrlEl.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIDOMXULMenuListElement */
#define NS_IDOMXULMENULISTELEMENT_IID_STR "36c16a17-c0e9-4b35-951b-81a147314ef1"

#define NS_IDOMXULMENULISTELEMENT_IID \
  {0x36c16a17, 0xc0e9, 0x4b35, \
    { 0x95, 0x1b, 0x81, 0xa1, 0x47, 0x31, 0x4e, 0xf1 }}

class NS_NO_VTABLE nsIDOMXULMenuListElement : public nsIDOMXULSelectControlElement {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOMXULMENULISTELEMENT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDOMXULMenuListElement;

  /* attribute boolean editable; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEditable(bool *aEditable) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetEditable(bool aEditable) = 0;

  /* attribute boolean open; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetOpen(bool *aOpen) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetOpen(bool aOpen) = 0;

  /* readonly attribute AString label; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLabel(nsAString& aLabel) = 0;

  /* attribute AString crop; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCrop(nsAString& aCrop) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCrop(const nsAString& aCrop) = 0;

  /* attribute AString image; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetImage(nsAString& aImage) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetImage(const nsAString& aImage) = 0;

  /* readonly attribute Element inputField; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetInputField(mozilla::dom::Element **aInputField) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDOMXULMenuListElement, NS_IDOMXULMENULISTELEMENT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOMXULMENULISTELEMENT \
  NS_IMETHOD GetEditable(bool *aEditable) override; \
  NS_IMETHOD SetEditable(bool aEditable) override; \
  NS_IMETHOD GetOpen(bool *aOpen) override; \
  NS_IMETHOD SetOpen(bool aOpen) override; \
  NS_IMETHOD GetLabel(nsAString& aLabel) override; \
  NS_IMETHOD GetCrop(nsAString& aCrop) override; \
  NS_IMETHOD SetCrop(const nsAString& aCrop) override; \
  NS_IMETHOD GetImage(nsAString& aImage) override; \
  NS_IMETHOD SetImage(const nsAString& aImage) override; \
  NS_IMETHOD GetInputField(mozilla::dom::Element **aInputField) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOMXULMENULISTELEMENT \
  nsresult GetEditable(bool *aEditable); \
  nsresult SetEditable(bool aEditable); \
  nsresult GetOpen(bool *aOpen); \
  nsresult SetOpen(bool aOpen); \
  nsresult GetLabel(nsAString& aLabel); \
  nsresult GetCrop(nsAString& aCrop); \
  nsresult SetCrop(const nsAString& aCrop); \
  nsresult GetImage(nsAString& aImage); \
  nsresult SetImage(const nsAString& aImage); \
  nsresult GetInputField(mozilla::dom::Element **aInputField); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOMXULMENULISTELEMENT(_to) \
  NS_IMETHOD GetEditable(bool *aEditable) override { return _to GetEditable(aEditable); } \
  NS_IMETHOD SetEditable(bool aEditable) override { return _to SetEditable(aEditable); } \
  NS_IMETHOD GetOpen(bool *aOpen) override { return _to GetOpen(aOpen); } \
  NS_IMETHOD SetOpen(bool aOpen) override { return _to SetOpen(aOpen); } \
  NS_IMETHOD GetLabel(nsAString& aLabel) override { return _to GetLabel(aLabel); } \
  NS_IMETHOD GetCrop(nsAString& aCrop) override { return _to GetCrop(aCrop); } \
  NS_IMETHOD SetCrop(const nsAString& aCrop) override { return _to SetCrop(aCrop); } \
  NS_IMETHOD GetImage(nsAString& aImage) override { return _to GetImage(aImage); } \
  NS_IMETHOD SetImage(const nsAString& aImage) override { return _to SetImage(aImage); } \
  NS_IMETHOD GetInputField(mozilla::dom::Element **aInputField) override { return _to GetInputField(aInputField); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOMXULMENULISTELEMENT(_to) \
  NS_IMETHOD GetEditable(bool *aEditable) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEditable(aEditable); } \
  NS_IMETHOD SetEditable(bool aEditable) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetEditable(aEditable); } \
  NS_IMETHOD GetOpen(bool *aOpen) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOpen(aOpen); } \
  NS_IMETHOD SetOpen(bool aOpen) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetOpen(aOpen); } \
  NS_IMETHOD GetLabel(nsAString& aLabel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLabel(aLabel); } \
  NS_IMETHOD GetCrop(nsAString& aCrop) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCrop(aCrop); } \
  NS_IMETHOD SetCrop(const nsAString& aCrop) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCrop(aCrop); } \
  NS_IMETHOD GetImage(nsAString& aImage) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetImage(aImage); } \
  NS_IMETHOD SetImage(const nsAString& aImage) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetImage(aImage); } \
  NS_IMETHOD GetInputField(mozilla::dom::Element **aInputField) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInputField(aInputField); } 


#endif /* __gen_nsIDOMXULMenuListElement_h__ */
