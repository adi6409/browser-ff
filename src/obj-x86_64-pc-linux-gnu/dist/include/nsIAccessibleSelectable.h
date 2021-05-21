/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleSelectable.idl
 */

#ifndef __gen_nsIAccessibleSelectable_h__
#define __gen_nsIAccessibleSelectable_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIAccessible; /* forward declaration */

class nsIArray; /* forward declaration */


/* starting interface:    nsIAccessibleSelectable */
#define NS_IACCESSIBLESELECTABLE_IID_STR "8efb03d4-1354-4875-94cf-261336057626"

#define NS_IACCESSIBLESELECTABLE_IID \
  {0x8efb03d4, 0x1354, 0x4875, \
    { 0x94, 0xcf, 0x26, 0x13, 0x36, 0x05, 0x76, 0x26 }}

class NS_NO_VTABLE nsIAccessibleSelectable : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IACCESSIBLESELECTABLE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAccessibleSelectable;

  /* readonly attribute nsIArray selectedItems; */
  NS_IMETHOD GetSelectedItems(nsIArray **aSelectedItems) = 0;

  /* readonly attribute unsigned long selectedItemCount; */
  NS_IMETHOD GetSelectedItemCount(uint32_t *aSelectedItemCount) = 0;

  /* nsIAccessible getSelectedItemAt (in unsigned long index); */
  NS_IMETHOD GetSelectedItemAt(uint32_t index, nsIAccessible **_retval) = 0;

  /* boolean isItemSelected (in unsigned long index); */
  NS_IMETHOD IsItemSelected(uint32_t index, bool *_retval) = 0;

  /* void addItemToSelection (in unsigned long index); */
  NS_IMETHOD AddItemToSelection(uint32_t index) = 0;

  /* void removeItemFromSelection (in unsigned long index); */
  NS_IMETHOD RemoveItemFromSelection(uint32_t index) = 0;

  /* boolean selectAll (); */
  NS_IMETHOD SelectAll(bool *_retval) = 0;

  /* void unselectAll (); */
  NS_IMETHOD UnselectAll(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAccessibleSelectable, NS_IACCESSIBLESELECTABLE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIACCESSIBLESELECTABLE \
  NS_IMETHOD GetSelectedItems(nsIArray **aSelectedItems) override; \
  NS_IMETHOD GetSelectedItemCount(uint32_t *aSelectedItemCount) override; \
  NS_IMETHOD GetSelectedItemAt(uint32_t index, nsIAccessible **_retval) override; \
  NS_IMETHOD IsItemSelected(uint32_t index, bool *_retval) override; \
  NS_IMETHOD AddItemToSelection(uint32_t index) override; \
  NS_IMETHOD RemoveItemFromSelection(uint32_t index) override; \
  NS_IMETHOD SelectAll(bool *_retval) override; \
  NS_IMETHOD UnselectAll(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIACCESSIBLESELECTABLE \
  nsresult GetSelectedItems(nsIArray **aSelectedItems); \
  nsresult GetSelectedItemCount(uint32_t *aSelectedItemCount); \
  nsresult GetSelectedItemAt(uint32_t index, nsIAccessible **_retval); \
  nsresult IsItemSelected(uint32_t index, bool *_retval); \
  nsresult AddItemToSelection(uint32_t index); \
  nsresult RemoveItemFromSelection(uint32_t index); \
  nsresult SelectAll(bool *_retval); \
  nsresult UnselectAll(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIACCESSIBLESELECTABLE(_to) \
  NS_IMETHOD GetSelectedItems(nsIArray **aSelectedItems) override { return _to GetSelectedItems(aSelectedItems); } \
  NS_IMETHOD GetSelectedItemCount(uint32_t *aSelectedItemCount) override { return _to GetSelectedItemCount(aSelectedItemCount); } \
  NS_IMETHOD GetSelectedItemAt(uint32_t index, nsIAccessible **_retval) override { return _to GetSelectedItemAt(index, _retval); } \
  NS_IMETHOD IsItemSelected(uint32_t index, bool *_retval) override { return _to IsItemSelected(index, _retval); } \
  NS_IMETHOD AddItemToSelection(uint32_t index) override { return _to AddItemToSelection(index); } \
  NS_IMETHOD RemoveItemFromSelection(uint32_t index) override { return _to RemoveItemFromSelection(index); } \
  NS_IMETHOD SelectAll(bool *_retval) override { return _to SelectAll(_retval); } \
  NS_IMETHOD UnselectAll(void) override { return _to UnselectAll(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIACCESSIBLESELECTABLE(_to) \
  NS_IMETHOD GetSelectedItems(nsIArray **aSelectedItems) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelectedItems(aSelectedItems); } \
  NS_IMETHOD GetSelectedItemCount(uint32_t *aSelectedItemCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelectedItemCount(aSelectedItemCount); } \
  NS_IMETHOD GetSelectedItemAt(uint32_t index, nsIAccessible **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelectedItemAt(index, _retval); } \
  NS_IMETHOD IsItemSelected(uint32_t index, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsItemSelected(index, _retval); } \
  NS_IMETHOD AddItemToSelection(uint32_t index) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddItemToSelection(index); } \
  NS_IMETHOD RemoveItemFromSelection(uint32_t index) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveItemFromSelection(index); } \
  NS_IMETHOD SelectAll(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SelectAll(_retval); } \
  NS_IMETHOD UnselectAll(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UnselectAll(); } 


#endif /* __gen_nsIAccessibleSelectable_h__ */
