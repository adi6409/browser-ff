/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/xul/nsIDOMXULMultSelectCntrlEl.idl
 */

#ifndef __gen_nsIDOMXULMultSelectCntrlEl_h__
#define __gen_nsIDOMXULMultSelectCntrlEl_h__


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

class nsINodeList; /* webidl NodeList */


/* starting interface:    nsIDOMXULMultiSelectControlElement */
#define NS_IDOMXULMULTISELECTCONTROLELEMENT_IID_STR "40654a10-8204-4f06-9f21-7baa31c7b1dd"

#define NS_IDOMXULMULTISELECTCONTROLELEMENT_IID \
  {0x40654a10, 0x8204, 0x4f06, \
    { 0x9f, 0x21, 0x7b, 0xaa, 0x31, 0xc7, 0xb1, 0xdd }}

class NS_NO_VTABLE nsIDOMXULMultiSelectControlElement : public nsIDOMXULSelectControlElement {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOMXULMULTISELECTCONTROLELEMENT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDOMXULMultiSelectControlElement;

  /* attribute AString selType; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSelType(nsAString& aSelType) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSelType(const nsAString& aSelType) = 0;

  /* attribute Element currentItem; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCurrentItem(mozilla::dom::Element **aCurrentItem) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCurrentItem(mozilla::dom::Element *aCurrentItem) = 0;

  /* attribute long currentIndex; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCurrentIndex(int32_t *aCurrentIndex) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCurrentIndex(int32_t aCurrentIndex) = 0;

  /* readonly attribute NodeList selectedItems; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSelectedItems(nsINodeList **aSelectedItems) = 0;

  /* void addItemToSelection (in nsIDOMXULSelectControlItemElement item); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddItemToSelection(nsIDOMXULSelectControlItemElement *item) = 0;

  /* void removeItemFromSelection (in nsIDOMXULSelectControlItemElement item); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveItemFromSelection(nsIDOMXULSelectControlItemElement *item) = 0;

  /* void toggleItemSelection (in nsIDOMXULSelectControlItemElement item); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ToggleItemSelection(nsIDOMXULSelectControlItemElement *item) = 0;

  /* void selectItem (in nsIDOMXULSelectControlItemElement item); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SelectItem(nsIDOMXULSelectControlItemElement *item) = 0;

  /* void selectItemRange (in nsIDOMXULSelectControlItemElement startItem, in nsIDOMXULSelectControlItemElement item); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SelectItemRange(nsIDOMXULSelectControlItemElement *startItem, nsIDOMXULSelectControlItemElement *item) = 0;

  /* void selectAll (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SelectAll(void) = 0;

  /* void clearSelection (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ClearSelection(void) = 0;

  /* readonly attribute long selectedCount; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSelectedCount(int32_t *aSelectedCount) = 0;

  /* [binaryname(MultiGetSelectedItem)] Element getSelectedItem (in long index); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MultiGetSelectedItem(int32_t index, mozilla::dom::Element **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDOMXULMultiSelectControlElement, NS_IDOMXULMULTISELECTCONTROLELEMENT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOMXULMULTISELECTCONTROLELEMENT \
  NS_IMETHOD GetSelType(nsAString& aSelType) override; \
  NS_IMETHOD SetSelType(const nsAString& aSelType) override; \
  NS_IMETHOD GetCurrentItem(mozilla::dom::Element **aCurrentItem) override; \
  NS_IMETHOD SetCurrentItem(mozilla::dom::Element *aCurrentItem) override; \
  NS_IMETHOD GetCurrentIndex(int32_t *aCurrentIndex) override; \
  NS_IMETHOD SetCurrentIndex(int32_t aCurrentIndex) override; \
  NS_IMETHOD GetSelectedItems(nsINodeList **aSelectedItems) override; \
  NS_IMETHOD AddItemToSelection(nsIDOMXULSelectControlItemElement *item) override; \
  NS_IMETHOD RemoveItemFromSelection(nsIDOMXULSelectControlItemElement *item) override; \
  NS_IMETHOD ToggleItemSelection(nsIDOMXULSelectControlItemElement *item) override; \
  NS_IMETHOD SelectItem(nsIDOMXULSelectControlItemElement *item) override; \
  NS_IMETHOD SelectItemRange(nsIDOMXULSelectControlItemElement *startItem, nsIDOMXULSelectControlItemElement *item) override; \
  NS_IMETHOD SelectAll(void) override; \
  NS_IMETHOD ClearSelection(void) override; \
  NS_IMETHOD GetSelectedCount(int32_t *aSelectedCount) override; \
  NS_IMETHOD MultiGetSelectedItem(int32_t index, mozilla::dom::Element **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOMXULMULTISELECTCONTROLELEMENT \
  nsresult GetSelType(nsAString& aSelType); \
  nsresult SetSelType(const nsAString& aSelType); \
  nsresult GetCurrentItem(mozilla::dom::Element **aCurrentItem); \
  nsresult SetCurrentItem(mozilla::dom::Element *aCurrentItem); \
  nsresult GetCurrentIndex(int32_t *aCurrentIndex); \
  nsresult SetCurrentIndex(int32_t aCurrentIndex); \
  nsresult GetSelectedItems(nsINodeList **aSelectedItems); \
  nsresult AddItemToSelection(nsIDOMXULSelectControlItemElement *item); \
  nsresult RemoveItemFromSelection(nsIDOMXULSelectControlItemElement *item); \
  nsresult ToggleItemSelection(nsIDOMXULSelectControlItemElement *item); \
  nsresult SelectItem(nsIDOMXULSelectControlItemElement *item); \
  nsresult SelectItemRange(nsIDOMXULSelectControlItemElement *startItem, nsIDOMXULSelectControlItemElement *item); \
  nsresult SelectAll(void); \
  nsresult ClearSelection(void); \
  nsresult GetSelectedCount(int32_t *aSelectedCount); \
  nsresult MultiGetSelectedItem(int32_t index, mozilla::dom::Element **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOMXULMULTISELECTCONTROLELEMENT(_to) \
  NS_IMETHOD GetSelType(nsAString& aSelType) override { return _to GetSelType(aSelType); } \
  NS_IMETHOD SetSelType(const nsAString& aSelType) override { return _to SetSelType(aSelType); } \
  NS_IMETHOD GetCurrentItem(mozilla::dom::Element **aCurrentItem) override { return _to GetCurrentItem(aCurrentItem); } \
  NS_IMETHOD SetCurrentItem(mozilla::dom::Element *aCurrentItem) override { return _to SetCurrentItem(aCurrentItem); } \
  NS_IMETHOD GetCurrentIndex(int32_t *aCurrentIndex) override { return _to GetCurrentIndex(aCurrentIndex); } \
  NS_IMETHOD SetCurrentIndex(int32_t aCurrentIndex) override { return _to SetCurrentIndex(aCurrentIndex); } \
  NS_IMETHOD GetSelectedItems(nsINodeList **aSelectedItems) override { return _to GetSelectedItems(aSelectedItems); } \
  NS_IMETHOD AddItemToSelection(nsIDOMXULSelectControlItemElement *item) override { return _to AddItemToSelection(item); } \
  NS_IMETHOD RemoveItemFromSelection(nsIDOMXULSelectControlItemElement *item) override { return _to RemoveItemFromSelection(item); } \
  NS_IMETHOD ToggleItemSelection(nsIDOMXULSelectControlItemElement *item) override { return _to ToggleItemSelection(item); } \
  NS_IMETHOD SelectItem(nsIDOMXULSelectControlItemElement *item) override { return _to SelectItem(item); } \
  NS_IMETHOD SelectItemRange(nsIDOMXULSelectControlItemElement *startItem, nsIDOMXULSelectControlItemElement *item) override { return _to SelectItemRange(startItem, item); } \
  NS_IMETHOD SelectAll(void) override { return _to SelectAll(); } \
  NS_IMETHOD ClearSelection(void) override { return _to ClearSelection(); } \
  NS_IMETHOD GetSelectedCount(int32_t *aSelectedCount) override { return _to GetSelectedCount(aSelectedCount); } \
  NS_IMETHOD MultiGetSelectedItem(int32_t index, mozilla::dom::Element **_retval) override { return _to MultiGetSelectedItem(index, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOMXULMULTISELECTCONTROLELEMENT(_to) \
  NS_IMETHOD GetSelType(nsAString& aSelType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelType(aSelType); } \
  NS_IMETHOD SetSelType(const nsAString& aSelType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSelType(aSelType); } \
  NS_IMETHOD GetCurrentItem(mozilla::dom::Element **aCurrentItem) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCurrentItem(aCurrentItem); } \
  NS_IMETHOD SetCurrentItem(mozilla::dom::Element *aCurrentItem) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCurrentItem(aCurrentItem); } \
  NS_IMETHOD GetCurrentIndex(int32_t *aCurrentIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCurrentIndex(aCurrentIndex); } \
  NS_IMETHOD SetCurrentIndex(int32_t aCurrentIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCurrentIndex(aCurrentIndex); } \
  NS_IMETHOD GetSelectedItems(nsINodeList **aSelectedItems) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelectedItems(aSelectedItems); } \
  NS_IMETHOD AddItemToSelection(nsIDOMXULSelectControlItemElement *item) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddItemToSelection(item); } \
  NS_IMETHOD RemoveItemFromSelection(nsIDOMXULSelectControlItemElement *item) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveItemFromSelection(item); } \
  NS_IMETHOD ToggleItemSelection(nsIDOMXULSelectControlItemElement *item) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ToggleItemSelection(item); } \
  NS_IMETHOD SelectItem(nsIDOMXULSelectControlItemElement *item) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SelectItem(item); } \
  NS_IMETHOD SelectItemRange(nsIDOMXULSelectControlItemElement *startItem, nsIDOMXULSelectControlItemElement *item) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SelectItemRange(startItem, item); } \
  NS_IMETHOD SelectAll(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SelectAll(); } \
  NS_IMETHOD ClearSelection(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearSelection(); } \
  NS_IMETHOD GetSelectedCount(int32_t *aSelectedCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelectedCount(aSelectedCount); } \
  NS_IMETHOD MultiGetSelectedItem(int32_t index, mozilla::dom::Element **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MultiGetSelectedItem(index, _retval); } 


#endif /* __gen_nsIDOMXULMultSelectCntrlEl_h__ */
