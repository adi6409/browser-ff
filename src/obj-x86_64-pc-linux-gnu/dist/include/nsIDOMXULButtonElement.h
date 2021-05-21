/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/xul/nsIDOMXULButtonElement.idl
 */

#ifndef __gen_nsIDOMXULButtonElement_h__
#define __gen_nsIDOMXULButtonElement_h__


#ifndef __gen_nsIDOMXULControlElement_h__
#include "nsIDOMXULControlElement.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIDOMXULButtonElement */
#define NS_IDOMXULBUTTONELEMENT_IID_STR "6ed53cfb-9e59-424c-af8d-e74582381951"

#define NS_IDOMXULBUTTONELEMENT_IID \
  {0x6ed53cfb, 0x9e59, 0x424c, \
    { 0xaf, 0x8d, 0xe7, 0x45, 0x82, 0x38, 0x19, 0x51 }}

class NS_NO_VTABLE nsIDOMXULButtonElement : public nsIDOMXULControlElement {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOMXULBUTTONELEMENT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDOMXULButtonElement;

  /* attribute AString type; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetType(nsAString& aType) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetType(const nsAString& aType) = 0;

  /* attribute boolean open; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetOpen(bool *aOpen) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetOpen(bool aOpen) = 0;

  /* attribute boolean checked; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetChecked(bool *aChecked) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetChecked(bool aChecked) = 0;

  /* attribute AString group; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetGroup(nsAString& aGroup) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetGroup(const nsAString& aGroup) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDOMXULButtonElement, NS_IDOMXULBUTTONELEMENT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOMXULBUTTONELEMENT \
  NS_IMETHOD GetType(nsAString& aType) override; \
  NS_IMETHOD SetType(const nsAString& aType) override; \
  NS_IMETHOD GetOpen(bool *aOpen) override; \
  NS_IMETHOD SetOpen(bool aOpen) override; \
  NS_IMETHOD GetChecked(bool *aChecked) override; \
  NS_IMETHOD SetChecked(bool aChecked) override; \
  NS_IMETHOD GetGroup(nsAString& aGroup) override; \
  NS_IMETHOD SetGroup(const nsAString& aGroup) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOMXULBUTTONELEMENT \
  nsresult GetType(nsAString& aType); \
  nsresult SetType(const nsAString& aType); \
  nsresult GetOpen(bool *aOpen); \
  nsresult SetOpen(bool aOpen); \
  nsresult GetChecked(bool *aChecked); \
  nsresult SetChecked(bool aChecked); \
  nsresult GetGroup(nsAString& aGroup); \
  nsresult SetGroup(const nsAString& aGroup); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOMXULBUTTONELEMENT(_to) \
  NS_IMETHOD GetType(nsAString& aType) override { return _to GetType(aType); } \
  NS_IMETHOD SetType(const nsAString& aType) override { return _to SetType(aType); } \
  NS_IMETHOD GetOpen(bool *aOpen) override { return _to GetOpen(aOpen); } \
  NS_IMETHOD SetOpen(bool aOpen) override { return _to SetOpen(aOpen); } \
  NS_IMETHOD GetChecked(bool *aChecked) override { return _to GetChecked(aChecked); } \
  NS_IMETHOD SetChecked(bool aChecked) override { return _to SetChecked(aChecked); } \
  NS_IMETHOD GetGroup(nsAString& aGroup) override { return _to GetGroup(aGroup); } \
  NS_IMETHOD SetGroup(const nsAString& aGroup) override { return _to SetGroup(aGroup); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOMXULBUTTONELEMENT(_to) \
  NS_IMETHOD GetType(nsAString& aType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetType(aType); } \
  NS_IMETHOD SetType(const nsAString& aType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetType(aType); } \
  NS_IMETHOD GetOpen(bool *aOpen) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOpen(aOpen); } \
  NS_IMETHOD SetOpen(bool aOpen) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetOpen(aOpen); } \
  NS_IMETHOD GetChecked(bool *aChecked) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChecked(aChecked); } \
  NS_IMETHOD SetChecked(bool aChecked) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetChecked(aChecked); } \
  NS_IMETHOD GetGroup(nsAString& aGroup) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetGroup(aGroup); } \
  NS_IMETHOD SetGroup(const nsAString& aGroup) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetGroup(aGroup); } 


#endif /* __gen_nsIDOMXULButtonElement_h__ */
