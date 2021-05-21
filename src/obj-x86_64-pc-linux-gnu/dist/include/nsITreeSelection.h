/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/layout/xul/tree/nsITreeSelection.idl
 */

#ifndef __gen_nsITreeSelection_h__
#define __gen_nsITreeSelection_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
namespace dom {
class XULTreeElement; /* webidl XULTreeElement */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsITreeSelection */
#define NS_ITREESELECTION_IID_STR "ab6fe746-300b-4ab4-abb9-1c0e3977874c"

#define NS_ITREESELECTION_IID \
  {0xab6fe746, 0x300b, 0x4ab4, \
    { 0xab, 0xb9, 0x1c, 0x0e, 0x39, 0x77, 0x87, 0x4c }}

class NS_NO_VTABLE nsITreeSelection : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITREESELECTION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITreeSelection;

  /* attribute XULTreeElement tree; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTree(mozilla::dom::XULTreeElement **aTree) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetTree(mozilla::dom::XULTreeElement *aTree) = 0;

  /* readonly attribute boolean single; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSingle(bool *aSingle) = 0;

  /* readonly attribute long count; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCount(int32_t *aCount) = 0;

  /* boolean isSelected (in long index); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsSelected(int32_t index, bool *_retval) = 0;

  /* void select (in long index); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Select(int32_t index) = 0;

  /* void timedSelect (in long index, in long delay); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD TimedSelect(int32_t index, int32_t delay) = 0;

  /* void toggleSelect (in long index); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ToggleSelect(int32_t index) = 0;

  /* void rangedSelect (in long startIndex, in long endIndex, in boolean augment); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RangedSelect(int32_t startIndex, int32_t endIndex, bool augment) = 0;

  /* void clearRange (in long startIndex, in long endIndex); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ClearRange(int32_t startIndex, int32_t endIndex) = 0;

  /* void clearSelection (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ClearSelection(void) = 0;

  /* void selectAll (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SelectAll(void) = 0;

  /* long getRangeCount (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRangeCount(int32_t *_retval) = 0;

  /* void getRangeAt (in long i, out long min, out long max); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRangeAt(int32_t i, int32_t *min, int32_t *max) = 0;

  /* void invalidateSelection (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD InvalidateSelection(void) = 0;

  /* void adjustSelection (in long index, in long count); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AdjustSelection(int32_t index, int32_t count) = 0;

  /* attribute boolean selectEventsSuppressed; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSelectEventsSuppressed(bool *aSelectEventsSuppressed) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSelectEventsSuppressed(bool aSelectEventsSuppressed) = 0;

  /* attribute long currentIndex; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCurrentIndex(int32_t *aCurrentIndex) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCurrentIndex(int32_t aCurrentIndex) = 0;

  /* readonly attribute long shiftSelectPivot; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetShiftSelectPivot(int32_t *aShiftSelectPivot) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITreeSelection, NS_ITREESELECTION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITREESELECTION \
  NS_IMETHOD GetTree(mozilla::dom::XULTreeElement **aTree) override; \
  NS_IMETHOD SetTree(mozilla::dom::XULTreeElement *aTree) override; \
  NS_IMETHOD GetSingle(bool *aSingle) override; \
  NS_IMETHOD GetCount(int32_t *aCount) override; \
  NS_IMETHOD IsSelected(int32_t index, bool *_retval) override; \
  NS_IMETHOD Select(int32_t index) override; \
  NS_IMETHOD TimedSelect(int32_t index, int32_t delay) override; \
  NS_IMETHOD ToggleSelect(int32_t index) override; \
  NS_IMETHOD RangedSelect(int32_t startIndex, int32_t endIndex, bool augment) override; \
  NS_IMETHOD ClearRange(int32_t startIndex, int32_t endIndex) override; \
  NS_IMETHOD ClearSelection(void) override; \
  NS_IMETHOD SelectAll(void) override; \
  NS_IMETHOD GetRangeCount(int32_t *_retval) override; \
  NS_IMETHOD GetRangeAt(int32_t i, int32_t *min, int32_t *max) override; \
  NS_IMETHOD InvalidateSelection(void) override; \
  NS_IMETHOD AdjustSelection(int32_t index, int32_t count) override; \
  NS_IMETHOD GetSelectEventsSuppressed(bool *aSelectEventsSuppressed) override; \
  NS_IMETHOD SetSelectEventsSuppressed(bool aSelectEventsSuppressed) override; \
  NS_IMETHOD GetCurrentIndex(int32_t *aCurrentIndex) override; \
  NS_IMETHOD SetCurrentIndex(int32_t aCurrentIndex) override; \
  NS_IMETHOD GetShiftSelectPivot(int32_t *aShiftSelectPivot) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITREESELECTION \
  nsresult GetTree(mozilla::dom::XULTreeElement **aTree); \
  nsresult SetTree(mozilla::dom::XULTreeElement *aTree); \
  nsresult GetSingle(bool *aSingle); \
  nsresult GetCount(int32_t *aCount); \
  nsresult IsSelected(int32_t index, bool *_retval); \
  nsresult Select(int32_t index); \
  nsresult TimedSelect(int32_t index, int32_t delay); \
  nsresult ToggleSelect(int32_t index); \
  nsresult RangedSelect(int32_t startIndex, int32_t endIndex, bool augment); \
  nsresult ClearRange(int32_t startIndex, int32_t endIndex); \
  nsresult ClearSelection(void); \
  nsresult SelectAll(void); \
  nsresult GetRangeCount(int32_t *_retval); \
  nsresult GetRangeAt(int32_t i, int32_t *min, int32_t *max); \
  nsresult InvalidateSelection(void); \
  nsresult AdjustSelection(int32_t index, int32_t count); \
  nsresult GetSelectEventsSuppressed(bool *aSelectEventsSuppressed); \
  nsresult SetSelectEventsSuppressed(bool aSelectEventsSuppressed); \
  nsresult GetCurrentIndex(int32_t *aCurrentIndex); \
  nsresult SetCurrentIndex(int32_t aCurrentIndex); \
  nsresult GetShiftSelectPivot(int32_t *aShiftSelectPivot); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITREESELECTION(_to) \
  NS_IMETHOD GetTree(mozilla::dom::XULTreeElement **aTree) override { return _to GetTree(aTree); } \
  NS_IMETHOD SetTree(mozilla::dom::XULTreeElement *aTree) override { return _to SetTree(aTree); } \
  NS_IMETHOD GetSingle(bool *aSingle) override { return _to GetSingle(aSingle); } \
  NS_IMETHOD GetCount(int32_t *aCount) override { return _to GetCount(aCount); } \
  NS_IMETHOD IsSelected(int32_t index, bool *_retval) override { return _to IsSelected(index, _retval); } \
  NS_IMETHOD Select(int32_t index) override { return _to Select(index); } \
  NS_IMETHOD TimedSelect(int32_t index, int32_t delay) override { return _to TimedSelect(index, delay); } \
  NS_IMETHOD ToggleSelect(int32_t index) override { return _to ToggleSelect(index); } \
  NS_IMETHOD RangedSelect(int32_t startIndex, int32_t endIndex, bool augment) override { return _to RangedSelect(startIndex, endIndex, augment); } \
  NS_IMETHOD ClearRange(int32_t startIndex, int32_t endIndex) override { return _to ClearRange(startIndex, endIndex); } \
  NS_IMETHOD ClearSelection(void) override { return _to ClearSelection(); } \
  NS_IMETHOD SelectAll(void) override { return _to SelectAll(); } \
  NS_IMETHOD GetRangeCount(int32_t *_retval) override { return _to GetRangeCount(_retval); } \
  NS_IMETHOD GetRangeAt(int32_t i, int32_t *min, int32_t *max) override { return _to GetRangeAt(i, min, max); } \
  NS_IMETHOD InvalidateSelection(void) override { return _to InvalidateSelection(); } \
  NS_IMETHOD AdjustSelection(int32_t index, int32_t count) override { return _to AdjustSelection(index, count); } \
  NS_IMETHOD GetSelectEventsSuppressed(bool *aSelectEventsSuppressed) override { return _to GetSelectEventsSuppressed(aSelectEventsSuppressed); } \
  NS_IMETHOD SetSelectEventsSuppressed(bool aSelectEventsSuppressed) override { return _to SetSelectEventsSuppressed(aSelectEventsSuppressed); } \
  NS_IMETHOD GetCurrentIndex(int32_t *aCurrentIndex) override { return _to GetCurrentIndex(aCurrentIndex); } \
  NS_IMETHOD SetCurrentIndex(int32_t aCurrentIndex) override { return _to SetCurrentIndex(aCurrentIndex); } \
  NS_IMETHOD GetShiftSelectPivot(int32_t *aShiftSelectPivot) override { return _to GetShiftSelectPivot(aShiftSelectPivot); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITREESELECTION(_to) \
  NS_IMETHOD GetTree(mozilla::dom::XULTreeElement **aTree) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTree(aTree); } \
  NS_IMETHOD SetTree(mozilla::dom::XULTreeElement *aTree) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTree(aTree); } \
  NS_IMETHOD GetSingle(bool *aSingle) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSingle(aSingle); } \
  NS_IMETHOD GetCount(int32_t *aCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCount(aCount); } \
  NS_IMETHOD IsSelected(int32_t index, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsSelected(index, _retval); } \
  NS_IMETHOD Select(int32_t index) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Select(index); } \
  NS_IMETHOD TimedSelect(int32_t index, int32_t delay) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TimedSelect(index, delay); } \
  NS_IMETHOD ToggleSelect(int32_t index) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ToggleSelect(index); } \
  NS_IMETHOD RangedSelect(int32_t startIndex, int32_t endIndex, bool augment) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RangedSelect(startIndex, endIndex, augment); } \
  NS_IMETHOD ClearRange(int32_t startIndex, int32_t endIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearRange(startIndex, endIndex); } \
  NS_IMETHOD ClearSelection(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearSelection(); } \
  NS_IMETHOD SelectAll(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SelectAll(); } \
  NS_IMETHOD GetRangeCount(int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRangeCount(_retval); } \
  NS_IMETHOD GetRangeAt(int32_t i, int32_t *min, int32_t *max) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRangeAt(i, min, max); } \
  NS_IMETHOD InvalidateSelection(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InvalidateSelection(); } \
  NS_IMETHOD AdjustSelection(int32_t index, int32_t count) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AdjustSelection(index, count); } \
  NS_IMETHOD GetSelectEventsSuppressed(bool *aSelectEventsSuppressed) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelectEventsSuppressed(aSelectEventsSuppressed); } \
  NS_IMETHOD SetSelectEventsSuppressed(bool aSelectEventsSuppressed) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSelectEventsSuppressed(aSelectEventsSuppressed); } \
  NS_IMETHOD GetCurrentIndex(int32_t *aCurrentIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCurrentIndex(aCurrentIndex); } \
  NS_IMETHOD SetCurrentIndex(int32_t aCurrentIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCurrentIndex(aCurrentIndex); } \
  NS_IMETHOD GetShiftSelectPivot(int32_t *aShiftSelectPivot) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetShiftSelectPivot(aShiftSelectPivot); } 


/* starting interface:    nsINativeTreeSelection */
#define NS_INATIVETREESELECTION_IID_STR "1bd59678-5cb3-4316-b246-31a91b19aabe"

#define NS_INATIVETREESELECTION_IID \
  {0x1bd59678, 0x5cb3, 0x4316, \
    { 0xb2, 0x46, 0x31, 0xa9, 0x1b, 0x19, 0xaa, 0xbe }}

class NS_NO_VTABLE nsINativeTreeSelection : public nsITreeSelection {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INATIVETREESELECTION_IID)

  /* [noscript] void ensureNative (); */
  NS_IMETHOD EnsureNative(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINativeTreeSelection, NS_INATIVETREESELECTION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINATIVETREESELECTION \
  NS_IMETHOD EnsureNative(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINATIVETREESELECTION \
  nsresult EnsureNative(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINATIVETREESELECTION(_to) \
  NS_IMETHOD EnsureNative(void) override { return _to EnsureNative(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINATIVETREESELECTION(_to) \
  NS_IMETHOD EnsureNative(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EnsureNative(); } 


#endif /* __gen_nsITreeSelection_h__ */
