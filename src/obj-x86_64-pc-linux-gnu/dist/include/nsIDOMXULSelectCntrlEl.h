/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/xul/nsIDOMXULSelectCntrlEl.idl
 */

#ifndef __gen_nsIDOMXULSelectCntrlEl_h__
#define __gen_nsIDOMXULSelectCntrlEl_h__


#ifndef __gen_nsIDOMXULControlElement_h__
#include "nsIDOMXULControlElement.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIDOMXULSelectControlItemElement; /* forward declaration */

namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIDOMXULSelectControlElement */
#define NS_IDOMXULSELECTCONTROLELEMENT_IID_STR "9bf188a7-d6f9-431b-b5c7-118013998e8b"

#define NS_IDOMXULSELECTCONTROLELEMENT_IID \
  {0x9bf188a7, 0xd6f9, 0x431b, \
    { 0xb5, 0xc7, 0x11, 0x80, 0x13, 0x99, 0x8e, 0x8b }}

class NS_NO_VTABLE nsIDOMXULSelectControlElement : public nsIDOMXULControlElement {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOMXULSELECTCONTROLELEMENT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDOMXULSelectControlElement;

  /* attribute Element selectedItem; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSelectedItem(mozilla::dom::Element **aSelectedItem) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSelectedItem(mozilla::dom::Element *aSelectedItem) = 0;

  /* attribute long selectedIndex; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSelectedIndex(int32_t *aSelectedIndex) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSelectedIndex(int32_t aSelectedIndex) = 0;

  /* attribute AString value; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetValue(nsAString& aValue) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetValue(const nsAString& aValue) = 0;

  /* readonly attribute unsigned long itemCount; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetItemCount(uint32_t *aItemCount) = 0;

  /* long getIndexOfItem (in nsIDOMXULSelectControlItemElement item); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIndexOfItem(nsIDOMXULSelectControlItemElement *item, int32_t *_retval) = 0;

  /* Element getItemAtIndex (in long index); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetItemAtIndex(int32_t index, mozilla::dom::Element **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDOMXULSelectControlElement, NS_IDOMXULSELECTCONTROLELEMENT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOMXULSELECTCONTROLELEMENT \
  NS_IMETHOD GetSelectedItem(mozilla::dom::Element **aSelectedItem) override; \
  NS_IMETHOD SetSelectedItem(mozilla::dom::Element *aSelectedItem) override; \
  NS_IMETHOD GetSelectedIndex(int32_t *aSelectedIndex) override; \
  NS_IMETHOD SetSelectedIndex(int32_t aSelectedIndex) override; \
  NS_IMETHOD GetValue(nsAString& aValue) override; \
  NS_IMETHOD SetValue(const nsAString& aValue) override; \
  NS_IMETHOD GetItemCount(uint32_t *aItemCount) override; \
  NS_IMETHOD GetIndexOfItem(nsIDOMXULSelectControlItemElement *item, int32_t *_retval) override; \
  NS_IMETHOD GetItemAtIndex(int32_t index, mozilla::dom::Element **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOMXULSELECTCONTROLELEMENT \
  nsresult GetSelectedItem(mozilla::dom::Element **aSelectedItem); \
  nsresult SetSelectedItem(mozilla::dom::Element *aSelectedItem); \
  nsresult GetSelectedIndex(int32_t *aSelectedIndex); \
  nsresult SetSelectedIndex(int32_t aSelectedIndex); \
  nsresult GetValue(nsAString& aValue); \
  nsresult SetValue(const nsAString& aValue); \
  nsresult GetItemCount(uint32_t *aItemCount); \
  nsresult GetIndexOfItem(nsIDOMXULSelectControlItemElement *item, int32_t *_retval); \
  nsresult GetItemAtIndex(int32_t index, mozilla::dom::Element **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOMXULSELECTCONTROLELEMENT(_to) \
  NS_IMETHOD GetSelectedItem(mozilla::dom::Element **aSelectedItem) override { return _to GetSelectedItem(aSelectedItem); } \
  NS_IMETHOD SetSelectedItem(mozilla::dom::Element *aSelectedItem) override { return _to SetSelectedItem(aSelectedItem); } \
  NS_IMETHOD GetSelectedIndex(int32_t *aSelectedIndex) override { return _to GetSelectedIndex(aSelectedIndex); } \
  NS_IMETHOD SetSelectedIndex(int32_t aSelectedIndex) override { return _to SetSelectedIndex(aSelectedIndex); } \
  NS_IMETHOD GetValue(nsAString& aValue) override { return _to GetValue(aValue); } \
  NS_IMETHOD SetValue(const nsAString& aValue) override { return _to SetValue(aValue); } \
  NS_IMETHOD GetItemCount(uint32_t *aItemCount) override { return _to GetItemCount(aItemCount); } \
  NS_IMETHOD GetIndexOfItem(nsIDOMXULSelectControlItemElement *item, int32_t *_retval) override { return _to GetIndexOfItem(item, _retval); } \
  NS_IMETHOD GetItemAtIndex(int32_t index, mozilla::dom::Element **_retval) override { return _to GetItemAtIndex(index, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOMXULSELECTCONTROLELEMENT(_to) \
  NS_IMETHOD GetSelectedItem(mozilla::dom::Element **aSelectedItem) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelectedItem(aSelectedItem); } \
  NS_IMETHOD SetSelectedItem(mozilla::dom::Element *aSelectedItem) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSelectedItem(aSelectedItem); } \
  NS_IMETHOD GetSelectedIndex(int32_t *aSelectedIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelectedIndex(aSelectedIndex); } \
  NS_IMETHOD SetSelectedIndex(int32_t aSelectedIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSelectedIndex(aSelectedIndex); } \
  NS_IMETHOD GetValue(nsAString& aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetValue(aValue); } \
  NS_IMETHOD SetValue(const nsAString& aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetValue(aValue); } \
  NS_IMETHOD GetItemCount(uint32_t *aItemCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetItemCount(aItemCount); } \
  NS_IMETHOD GetIndexOfItem(nsIDOMXULSelectControlItemElement *item, int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIndexOfItem(item, _retval); } \
  NS_IMETHOD GetItemAtIndex(int32_t index, mozilla::dom::Element **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetItemAtIndex(index, _retval); } 


#endif /* __gen_nsIDOMXULSelectCntrlEl_h__ */
