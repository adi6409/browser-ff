/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/layout/xul/tree/nsITreeView.idl
 */

#ifndef __gen_nsITreeView_h__
#define __gen_nsITreeView_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsITreeSelection; /* forward declaration */

namespace mozilla {
namespace dom {
class DataTransfer; /* webidl DataTransfer */
} // namespace dom
} // namespace mozilla

class nsTreeColumn; /* webidl TreeColumn */

namespace mozilla {
namespace dom {
class XULTreeElement; /* webidl XULTreeElement */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsITreeView */
#define NS_ITREEVIEW_IID_STR "091116f0-0bdc-4b32-b9c8-c8d5a37cb088"

#define NS_ITREEVIEW_IID \
  {0x091116f0, 0x0bdc, 0x4b32, \
    { 0xb9, 0xc8, 0xc8, 0xd5, 0xa3, 0x7c, 0xb0, 0x88 }}

class NS_NO_VTABLE nsITreeView : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITREEVIEW_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITreeView;

  /* readonly attribute long rowCount; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRowCount(int32_t *aRowCount) = 0;

  /* attribute nsITreeSelection selection; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSelection(nsITreeSelection **aSelection) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSelection(nsITreeSelection *aSelection) = 0;

  /* AString getRowProperties (in long index); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRowProperties(int32_t index, nsAString& _retval) = 0;

  /* AString getCellProperties (in long row, in TreeColumn col); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCellProperties(int32_t row, nsTreeColumn *col, nsAString& _retval) = 0;

  /* AString getColumnProperties (in TreeColumn col); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetColumnProperties(nsTreeColumn *col, nsAString& _retval) = 0;

  /* boolean isContainer (in long index); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsContainer(int32_t index, bool *_retval) = 0;

  /* boolean isContainerOpen (in long index); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsContainerOpen(int32_t index, bool *_retval) = 0;

  /* boolean isContainerEmpty (in long index); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsContainerEmpty(int32_t index, bool *_retval) = 0;

  /* boolean isSeparator (in long index); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsSeparator(int32_t index, bool *_retval) = 0;

  /* boolean isSorted (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsSorted(bool *_retval) = 0;

  enum {
    DROP_BEFORE = -1,
    DROP_ON = 0,
    DROP_AFTER = 1
  };

  /* boolean canDrop (in long index, in long orientation, in DataTransfer dataTransfer); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CanDrop(int32_t index, int32_t orientation, mozilla::dom::DataTransfer *dataTransfer, bool *_retval) = 0;

  /* void drop (in long row, in long orientation, in DataTransfer dataTransfer); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Drop(int32_t row, int32_t orientation, mozilla::dom::DataTransfer *dataTransfer) = 0;

  /* long getParentIndex (in long rowIndex); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetParentIndex(int32_t rowIndex, int32_t *_retval) = 0;

  /* boolean hasNextSibling (in long rowIndex, in long afterIndex); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HasNextSibling(int32_t rowIndex, int32_t afterIndex, bool *_retval) = 0;

  /* long getLevel (in long index); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLevel(int32_t index, int32_t *_retval) = 0;

  /* AString getImageSrc (in long row, in TreeColumn col); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetImageSrc(int32_t row, nsTreeColumn *col, nsAString& _retval) = 0;

  /* AString getCellValue (in long row, in TreeColumn col); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCellValue(int32_t row, nsTreeColumn *col, nsAString& _retval) = 0;

  /* AString getCellText (in long row, in TreeColumn col); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCellText(int32_t row, nsTreeColumn *col, nsAString& _retval) = 0;

  /* void setTree (in XULTreeElement tree); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetTree(mozilla::dom::XULTreeElement *tree) = 0;

  /* void toggleOpenState (in long index); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ToggleOpenState(int32_t index) = 0;

  /* void cycleHeader (in TreeColumn col); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CycleHeader(nsTreeColumn *col) = 0;

  /* [binaryname(SelectionChangedXPCOM)] void selectionChanged (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SelectionChangedXPCOM(void) = 0;

  /* void cycleCell (in long row, in TreeColumn col); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CycleCell(int32_t row, nsTreeColumn *col) = 0;

  /* boolean isEditable (in long row, in TreeColumn col); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsEditable(int32_t row, nsTreeColumn *col, bool *_retval) = 0;

  /* void setCellValue (in long row, in TreeColumn col, in AString value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCellValue(int32_t row, nsTreeColumn *col, const nsAString& value) = 0;

  /* void setCellText (in long row, in TreeColumn col, in AString value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCellText(int32_t row, nsTreeColumn *col, const nsAString& value) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITreeView, NS_ITREEVIEW_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITREEVIEW \
  NS_IMETHOD GetRowCount(int32_t *aRowCount) override; \
  NS_IMETHOD GetSelection(nsITreeSelection **aSelection) override; \
  NS_IMETHOD SetSelection(nsITreeSelection *aSelection) override; \
  NS_IMETHOD GetRowProperties(int32_t index, nsAString& _retval) override; \
  NS_IMETHOD GetCellProperties(int32_t row, nsTreeColumn *col, nsAString& _retval) override; \
  NS_IMETHOD GetColumnProperties(nsTreeColumn *col, nsAString& _retval) override; \
  NS_IMETHOD IsContainer(int32_t index, bool *_retval) override; \
  NS_IMETHOD IsContainerOpen(int32_t index, bool *_retval) override; \
  NS_IMETHOD IsContainerEmpty(int32_t index, bool *_retval) override; \
  NS_IMETHOD IsSeparator(int32_t index, bool *_retval) override; \
  NS_IMETHOD IsSorted(bool *_retval) override; \
  NS_IMETHOD CanDrop(int32_t index, int32_t orientation, mozilla::dom::DataTransfer *dataTransfer, bool *_retval) override; \
  NS_IMETHOD Drop(int32_t row, int32_t orientation, mozilla::dom::DataTransfer *dataTransfer) override; \
  NS_IMETHOD GetParentIndex(int32_t rowIndex, int32_t *_retval) override; \
  NS_IMETHOD HasNextSibling(int32_t rowIndex, int32_t afterIndex, bool *_retval) override; \
  NS_IMETHOD GetLevel(int32_t index, int32_t *_retval) override; \
  NS_IMETHOD GetImageSrc(int32_t row, nsTreeColumn *col, nsAString& _retval) override; \
  NS_IMETHOD GetCellValue(int32_t row, nsTreeColumn *col, nsAString& _retval) override; \
  NS_IMETHOD GetCellText(int32_t row, nsTreeColumn *col, nsAString& _retval) override; \
  NS_IMETHOD SetTree(mozilla::dom::XULTreeElement *tree) override; \
  NS_IMETHOD ToggleOpenState(int32_t index) override; \
  NS_IMETHOD CycleHeader(nsTreeColumn *col) override; \
  NS_IMETHOD SelectionChangedXPCOM(void) override; \
  NS_IMETHOD CycleCell(int32_t row, nsTreeColumn *col) override; \
  NS_IMETHOD IsEditable(int32_t row, nsTreeColumn *col, bool *_retval) override; \
  NS_IMETHOD SetCellValue(int32_t row, nsTreeColumn *col, const nsAString& value) override; \
  NS_IMETHOD SetCellText(int32_t row, nsTreeColumn *col, const nsAString& value) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITREEVIEW \
  nsresult GetRowCount(int32_t *aRowCount); \
  nsresult GetSelection(nsITreeSelection **aSelection); \
  nsresult SetSelection(nsITreeSelection *aSelection); \
  nsresult GetRowProperties(int32_t index, nsAString& _retval); \
  nsresult GetCellProperties(int32_t row, nsTreeColumn *col, nsAString& _retval); \
  nsresult GetColumnProperties(nsTreeColumn *col, nsAString& _retval); \
  nsresult IsContainer(int32_t index, bool *_retval); \
  nsresult IsContainerOpen(int32_t index, bool *_retval); \
  nsresult IsContainerEmpty(int32_t index, bool *_retval); \
  nsresult IsSeparator(int32_t index, bool *_retval); \
  nsresult IsSorted(bool *_retval); \
  nsresult CanDrop(int32_t index, int32_t orientation, mozilla::dom::DataTransfer *dataTransfer, bool *_retval); \
  nsresult Drop(int32_t row, int32_t orientation, mozilla::dom::DataTransfer *dataTransfer); \
  nsresult GetParentIndex(int32_t rowIndex, int32_t *_retval); \
  nsresult HasNextSibling(int32_t rowIndex, int32_t afterIndex, bool *_retval); \
  nsresult GetLevel(int32_t index, int32_t *_retval); \
  nsresult GetImageSrc(int32_t row, nsTreeColumn *col, nsAString& _retval); \
  nsresult GetCellValue(int32_t row, nsTreeColumn *col, nsAString& _retval); \
  nsresult GetCellText(int32_t row, nsTreeColumn *col, nsAString& _retval); \
  nsresult SetTree(mozilla::dom::XULTreeElement *tree); \
  nsresult ToggleOpenState(int32_t index); \
  nsresult CycleHeader(nsTreeColumn *col); \
  nsresult SelectionChangedXPCOM(void); \
  nsresult CycleCell(int32_t row, nsTreeColumn *col); \
  nsresult IsEditable(int32_t row, nsTreeColumn *col, bool *_retval); \
  nsresult SetCellValue(int32_t row, nsTreeColumn *col, const nsAString& value); \
  nsresult SetCellText(int32_t row, nsTreeColumn *col, const nsAString& value); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITREEVIEW(_to) \
  NS_IMETHOD GetRowCount(int32_t *aRowCount) override { return _to GetRowCount(aRowCount); } \
  NS_IMETHOD GetSelection(nsITreeSelection **aSelection) override { return _to GetSelection(aSelection); } \
  NS_IMETHOD SetSelection(nsITreeSelection *aSelection) override { return _to SetSelection(aSelection); } \
  NS_IMETHOD GetRowProperties(int32_t index, nsAString& _retval) override { return _to GetRowProperties(index, _retval); } \
  NS_IMETHOD GetCellProperties(int32_t row, nsTreeColumn *col, nsAString& _retval) override { return _to GetCellProperties(row, col, _retval); } \
  NS_IMETHOD GetColumnProperties(nsTreeColumn *col, nsAString& _retval) override { return _to GetColumnProperties(col, _retval); } \
  NS_IMETHOD IsContainer(int32_t index, bool *_retval) override { return _to IsContainer(index, _retval); } \
  NS_IMETHOD IsContainerOpen(int32_t index, bool *_retval) override { return _to IsContainerOpen(index, _retval); } \
  NS_IMETHOD IsContainerEmpty(int32_t index, bool *_retval) override { return _to IsContainerEmpty(index, _retval); } \
  NS_IMETHOD IsSeparator(int32_t index, bool *_retval) override { return _to IsSeparator(index, _retval); } \
  NS_IMETHOD IsSorted(bool *_retval) override { return _to IsSorted(_retval); } \
  NS_IMETHOD CanDrop(int32_t index, int32_t orientation, mozilla::dom::DataTransfer *dataTransfer, bool *_retval) override { return _to CanDrop(index, orientation, dataTransfer, _retval); } \
  NS_IMETHOD Drop(int32_t row, int32_t orientation, mozilla::dom::DataTransfer *dataTransfer) override { return _to Drop(row, orientation, dataTransfer); } \
  NS_IMETHOD GetParentIndex(int32_t rowIndex, int32_t *_retval) override { return _to GetParentIndex(rowIndex, _retval); } \
  NS_IMETHOD HasNextSibling(int32_t rowIndex, int32_t afterIndex, bool *_retval) override { return _to HasNextSibling(rowIndex, afterIndex, _retval); } \
  NS_IMETHOD GetLevel(int32_t index, int32_t *_retval) override { return _to GetLevel(index, _retval); } \
  NS_IMETHOD GetImageSrc(int32_t row, nsTreeColumn *col, nsAString& _retval) override { return _to GetImageSrc(row, col, _retval); } \
  NS_IMETHOD GetCellValue(int32_t row, nsTreeColumn *col, nsAString& _retval) override { return _to GetCellValue(row, col, _retval); } \
  NS_IMETHOD GetCellText(int32_t row, nsTreeColumn *col, nsAString& _retval) override { return _to GetCellText(row, col, _retval); } \
  NS_IMETHOD SetTree(mozilla::dom::XULTreeElement *tree) override { return _to SetTree(tree); } \
  NS_IMETHOD ToggleOpenState(int32_t index) override { return _to ToggleOpenState(index); } \
  NS_IMETHOD CycleHeader(nsTreeColumn *col) override { return _to CycleHeader(col); } \
  NS_IMETHOD SelectionChangedXPCOM(void) override { return _to SelectionChangedXPCOM(); } \
  NS_IMETHOD CycleCell(int32_t row, nsTreeColumn *col) override { return _to CycleCell(row, col); } \
  NS_IMETHOD IsEditable(int32_t row, nsTreeColumn *col, bool *_retval) override { return _to IsEditable(row, col, _retval); } \
  NS_IMETHOD SetCellValue(int32_t row, nsTreeColumn *col, const nsAString& value) override { return _to SetCellValue(row, col, value); } \
  NS_IMETHOD SetCellText(int32_t row, nsTreeColumn *col, const nsAString& value) override { return _to SetCellText(row, col, value); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITREEVIEW(_to) \
  NS_IMETHOD GetRowCount(int32_t *aRowCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRowCount(aRowCount); } \
  NS_IMETHOD GetSelection(nsITreeSelection **aSelection) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelection(aSelection); } \
  NS_IMETHOD SetSelection(nsITreeSelection *aSelection) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSelection(aSelection); } \
  NS_IMETHOD GetRowProperties(int32_t index, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRowProperties(index, _retval); } \
  NS_IMETHOD GetCellProperties(int32_t row, nsTreeColumn *col, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCellProperties(row, col, _retval); } \
  NS_IMETHOD GetColumnProperties(nsTreeColumn *col, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetColumnProperties(col, _retval); } \
  NS_IMETHOD IsContainer(int32_t index, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsContainer(index, _retval); } \
  NS_IMETHOD IsContainerOpen(int32_t index, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsContainerOpen(index, _retval); } \
  NS_IMETHOD IsContainerEmpty(int32_t index, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsContainerEmpty(index, _retval); } \
  NS_IMETHOD IsSeparator(int32_t index, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsSeparator(index, _retval); } \
  NS_IMETHOD IsSorted(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsSorted(_retval); } \
  NS_IMETHOD CanDrop(int32_t index, int32_t orientation, mozilla::dom::DataTransfer *dataTransfer, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CanDrop(index, orientation, dataTransfer, _retval); } \
  NS_IMETHOD Drop(int32_t row, int32_t orientation, mozilla::dom::DataTransfer *dataTransfer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Drop(row, orientation, dataTransfer); } \
  NS_IMETHOD GetParentIndex(int32_t rowIndex, int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetParentIndex(rowIndex, _retval); } \
  NS_IMETHOD HasNextSibling(int32_t rowIndex, int32_t afterIndex, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasNextSibling(rowIndex, afterIndex, _retval); } \
  NS_IMETHOD GetLevel(int32_t index, int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLevel(index, _retval); } \
  NS_IMETHOD GetImageSrc(int32_t row, nsTreeColumn *col, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetImageSrc(row, col, _retval); } \
  NS_IMETHOD GetCellValue(int32_t row, nsTreeColumn *col, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCellValue(row, col, _retval); } \
  NS_IMETHOD GetCellText(int32_t row, nsTreeColumn *col, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCellText(row, col, _retval); } \
  NS_IMETHOD SetTree(mozilla::dom::XULTreeElement *tree) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTree(tree); } \
  NS_IMETHOD ToggleOpenState(int32_t index) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ToggleOpenState(index); } \
  NS_IMETHOD CycleHeader(nsTreeColumn *col) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CycleHeader(col); } \
  NS_IMETHOD SelectionChangedXPCOM(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SelectionChangedXPCOM(); } \
  NS_IMETHOD CycleCell(int32_t row, nsTreeColumn *col) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CycleCell(row, col); } \
  NS_IMETHOD IsEditable(int32_t row, nsTreeColumn *col, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsEditable(row, col, _retval); } \
  NS_IMETHOD SetCellValue(int32_t row, nsTreeColumn *col, const nsAString& value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCellValue(row, col, value); } \
  NS_IMETHOD SetCellText(int32_t row, nsTreeColumn *col, const nsAString& value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCellText(row, col, value); } 


#endif /* __gen_nsITreeView_h__ */
