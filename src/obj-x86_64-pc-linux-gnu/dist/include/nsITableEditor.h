/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/nsITableEditor.idl
 */

#ifndef __gen_nsITableEditor_h__
#define __gen_nsITableEditor_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla

class nsINode; /* webidl Node */

class nsRange; /* webidl Range */


/* starting interface:    nsITableEditor */
#define NS_ITABLEEDITOR_IID_STR "4805e684-49b9-11d3-9ce4-ed60bd6cb5bc"

#define NS_ITABLEEDITOR_IID \
  {0x4805e684, 0x49b9, 0x11d3, \
    { 0x9c, 0xe4, 0xed, 0x60, 0xbd, 0x6c, 0xb5, 0xbc }}

class NS_NO_VTABLE nsITableEditor : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITABLEEDITOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITableEditor;

  enum {
    eNoSearch = 0,
    ePreviousColumn = 1,
    ePreviousRow = 2
  };

  /* [can_run_script] void insertTableCell (in long aNumberOfColumnsToInsert, in boolean aInsertAfterSelectedCell); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertTableCell(int32_t aNumberOfColumnsToInsert, bool aInsertAfterSelectedCell) = 0;

  /* [can_run_script] void insertTableColumn (in long aNumberOfColumnsToInsert, in boolean aInsertAfterSelectedCell); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertTableColumn(int32_t aNumberOfColumnsToInsert, bool aInsertAfterSelectedCell) = 0;

  /* [can_run_script] void insertTableRow (in long aNumberOfRowsToInsert, in boolean aInsertAfterSelectedCell); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertTableRow(int32_t aNumberOfRowsToInsert, bool aInsertAfterSelectedCell) = 0;

  /* [can_run_script] void deleteTable (); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteTable(void) = 0;

  /* [can_run_script] void deleteTableCellContents (); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteTableCellContents(void) = 0;

  /* [can_run_script] void deleteTableCell (in long aNumberOfCellsToDelete); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteTableCell(int32_t aNumberOfCellsToDelete) = 0;

  /* [can_run_script] void deleteTableColumn (in long aNumberOfColumnsToDelete); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteTableColumn(int32_t aNumberOfColumnsToDelete) = 0;

  /* [can_run_script] void deleteTableRow (in long aNumberOfRowsToDelete); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteTableRow(int32_t aNumberOfRowsToDelete) = 0;

  /* [can_run_script] void selectTableCell (); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectTableCell(void) = 0;

  /* [can_run_script] void selectTableRow (); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectTableRow(void) = 0;

  /* [can_run_script] void selectTableColumn (); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectTableColumn(void) = 0;

  /* [can_run_script] void selectTable (); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectTable(void) = 0;

  /* [can_run_script] void selectAllTableCells (); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectAllTableCells(void) = 0;

  /* [can_run_script] Element switchTableCellHeaderType (in Element aSourceCell); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SwitchTableCellHeaderType(mozilla::dom::Element *aSourceCell, mozilla::dom::Element **_retval) = 0;

  /* [can_run_script] void joinTableCells (in boolean aMergeNonContiguousContents); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD JoinTableCells(bool aMergeNonContiguousContents) = 0;

  /* [can_run_script] void splitTableCell (); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SplitTableCell(void) = 0;

  /* [can_run_script] void normalizeTable (in Element aTable); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD NormalizeTable(mozilla::dom::Element *aTable) = 0;

  /* [can_run_script] void getCellIndexes (in Element aCellElement, out long aRowIndex, out long aColumnIndex); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetCellIndexes(mozilla::dom::Element *aCellElement, int32_t *aRowIndex, int32_t *aColumnIndex) = 0;

  /* void getTableSize (in Element aTableOrElementInTable, out long aRowCount, out long aColCount); */
  NS_IMETHOD GetTableSize(mozilla::dom::Element *aTableOrElementInTable, int32_t *aRowCount, int32_t *aColCount) = 0;

  /* Element getCellAt (in Element aTableElement, in long aRowIndex, in long aColumnIndex); */
  NS_IMETHOD GetCellAt(mozilla::dom::Element *aTableElement, int32_t aRowIndex, int32_t aColumnIndex, mozilla::dom::Element **_retval) = 0;

  /* void getCellDataAt (in Element aTableElement, in long aRowIndex, in long aColumnIndex, out Element aCellElement, out long aStartRowIndex, out long aStartColumnIndex, out long aRowSpan, out long aColSpan, out long aEffectiveRowSpan, out long aEffectiveColSpan, out boolean aIsSelected); */
  NS_IMETHOD GetCellDataAt(mozilla::dom::Element *aTableElement, int32_t aRowIndex, int32_t aColumnIndex, mozilla::dom::Element **aCellElement, int32_t *aStartRowIndex, int32_t *aStartColumnIndex, int32_t *aRowSpan, int32_t *aColSpan, int32_t *aEffectiveRowSpan, int32_t *aEffectiveColSpan, bool *aIsSelected) = 0;

  /* Element getFirstRow (in Element aTableElement); */
  NS_IMETHOD GetFirstRow(mozilla::dom::Element *aTableElement, mozilla::dom::Element **_retval) = 0;

  /* Element getSelectedOrParentTableElement (out AString aTagName, out long aCount); */
  NS_IMETHOD GetSelectedOrParentTableElement(nsAString& aTagName, int32_t *aCount, mozilla::dom::Element **_retval) = 0;

  /* [can_run_script] uint32_t getSelectedCellsType (in Element aElement); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetSelectedCellsType(mozilla::dom::Element *aElement, uint32_t *_retval) = 0;

  /* [can_run_script] Element getFirstSelectedCellInTable (out long aRowIndex, out long aColIndex); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetFirstSelectedCellInTable(int32_t *aRowIndex, int32_t *aColIndex, mozilla::dom::Element **_retval) = 0;

  /* Array<Element> getSelectedCells (); */
  NS_IMETHOD GetSelectedCells(nsTArray<RefPtr<mozilla::dom::Element>>& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITableEditor, NS_ITABLEEDITOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITABLEEDITOR \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertTableCell(int32_t aNumberOfColumnsToInsert, bool aInsertAfterSelectedCell) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertTableColumn(int32_t aNumberOfColumnsToInsert, bool aInsertAfterSelectedCell) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertTableRow(int32_t aNumberOfRowsToInsert, bool aInsertAfterSelectedCell) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteTable(void) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteTableCellContents(void) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteTableCell(int32_t aNumberOfCellsToDelete) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteTableColumn(int32_t aNumberOfColumnsToDelete) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteTableRow(int32_t aNumberOfRowsToDelete) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectTableCell(void) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectTableRow(void) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectTableColumn(void) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectTable(void) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectAllTableCells(void) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SwitchTableCellHeaderType(mozilla::dom::Element *aSourceCell, mozilla::dom::Element **_retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD JoinTableCells(bool aMergeNonContiguousContents) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SplitTableCell(void) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD NormalizeTable(mozilla::dom::Element *aTable) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetCellIndexes(mozilla::dom::Element *aCellElement, int32_t *aRowIndex, int32_t *aColumnIndex) override; \
  NS_IMETHOD GetTableSize(mozilla::dom::Element *aTableOrElementInTable, int32_t *aRowCount, int32_t *aColCount) override; \
  NS_IMETHOD GetCellAt(mozilla::dom::Element *aTableElement, int32_t aRowIndex, int32_t aColumnIndex, mozilla::dom::Element **_retval) override; \
  NS_IMETHOD GetCellDataAt(mozilla::dom::Element *aTableElement, int32_t aRowIndex, int32_t aColumnIndex, mozilla::dom::Element **aCellElement, int32_t *aStartRowIndex, int32_t *aStartColumnIndex, int32_t *aRowSpan, int32_t *aColSpan, int32_t *aEffectiveRowSpan, int32_t *aEffectiveColSpan, bool *aIsSelected) override; \
  NS_IMETHOD GetFirstRow(mozilla::dom::Element *aTableElement, mozilla::dom::Element **_retval) override; \
  NS_IMETHOD GetSelectedOrParentTableElement(nsAString& aTagName, int32_t *aCount, mozilla::dom::Element **_retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetSelectedCellsType(mozilla::dom::Element *aElement, uint32_t *_retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetFirstSelectedCellInTable(int32_t *aRowIndex, int32_t *aColIndex, mozilla::dom::Element **_retval) override; \
  NS_IMETHOD GetSelectedCells(nsTArray<RefPtr<mozilla::dom::Element>>& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITABLEEDITOR \
  MOZ_CAN_RUN_SCRIPT nsresult InsertTableCell(int32_t aNumberOfColumnsToInsert, bool aInsertAfterSelectedCell); \
  MOZ_CAN_RUN_SCRIPT nsresult InsertTableColumn(int32_t aNumberOfColumnsToInsert, bool aInsertAfterSelectedCell); \
  MOZ_CAN_RUN_SCRIPT nsresult InsertTableRow(int32_t aNumberOfRowsToInsert, bool aInsertAfterSelectedCell); \
  MOZ_CAN_RUN_SCRIPT nsresult DeleteTable(void); \
  MOZ_CAN_RUN_SCRIPT nsresult DeleteTableCellContents(void); \
  MOZ_CAN_RUN_SCRIPT nsresult DeleteTableCell(int32_t aNumberOfCellsToDelete); \
  MOZ_CAN_RUN_SCRIPT nsresult DeleteTableColumn(int32_t aNumberOfColumnsToDelete); \
  MOZ_CAN_RUN_SCRIPT nsresult DeleteTableRow(int32_t aNumberOfRowsToDelete); \
  MOZ_CAN_RUN_SCRIPT nsresult SelectTableCell(void); \
  MOZ_CAN_RUN_SCRIPT nsresult SelectTableRow(void); \
  MOZ_CAN_RUN_SCRIPT nsresult SelectTableColumn(void); \
  MOZ_CAN_RUN_SCRIPT nsresult SelectTable(void); \
  MOZ_CAN_RUN_SCRIPT nsresult SelectAllTableCells(void); \
  MOZ_CAN_RUN_SCRIPT nsresult SwitchTableCellHeaderType(mozilla::dom::Element *aSourceCell, mozilla::dom::Element **_retval); \
  MOZ_CAN_RUN_SCRIPT nsresult JoinTableCells(bool aMergeNonContiguousContents); \
  MOZ_CAN_RUN_SCRIPT nsresult SplitTableCell(void); \
  MOZ_CAN_RUN_SCRIPT nsresult NormalizeTable(mozilla::dom::Element *aTable); \
  MOZ_CAN_RUN_SCRIPT nsresult GetCellIndexes(mozilla::dom::Element *aCellElement, int32_t *aRowIndex, int32_t *aColumnIndex); \
  nsresult GetTableSize(mozilla::dom::Element *aTableOrElementInTable, int32_t *aRowCount, int32_t *aColCount); \
  nsresult GetCellAt(mozilla::dom::Element *aTableElement, int32_t aRowIndex, int32_t aColumnIndex, mozilla::dom::Element **_retval); \
  nsresult GetCellDataAt(mozilla::dom::Element *aTableElement, int32_t aRowIndex, int32_t aColumnIndex, mozilla::dom::Element **aCellElement, int32_t *aStartRowIndex, int32_t *aStartColumnIndex, int32_t *aRowSpan, int32_t *aColSpan, int32_t *aEffectiveRowSpan, int32_t *aEffectiveColSpan, bool *aIsSelected); \
  nsresult GetFirstRow(mozilla::dom::Element *aTableElement, mozilla::dom::Element **_retval); \
  nsresult GetSelectedOrParentTableElement(nsAString& aTagName, int32_t *aCount, mozilla::dom::Element **_retval); \
  MOZ_CAN_RUN_SCRIPT nsresult GetSelectedCellsType(mozilla::dom::Element *aElement, uint32_t *_retval); \
  MOZ_CAN_RUN_SCRIPT nsresult GetFirstSelectedCellInTable(int32_t *aRowIndex, int32_t *aColIndex, mozilla::dom::Element **_retval); \
  nsresult GetSelectedCells(nsTArray<RefPtr<mozilla::dom::Element>>& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITABLEEDITOR(_to) \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertTableCell(int32_t aNumberOfColumnsToInsert, bool aInsertAfterSelectedCell) override { return _to InsertTableCell(aNumberOfColumnsToInsert, aInsertAfterSelectedCell); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertTableColumn(int32_t aNumberOfColumnsToInsert, bool aInsertAfterSelectedCell) override { return _to InsertTableColumn(aNumberOfColumnsToInsert, aInsertAfterSelectedCell); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertTableRow(int32_t aNumberOfRowsToInsert, bool aInsertAfterSelectedCell) override { return _to InsertTableRow(aNumberOfRowsToInsert, aInsertAfterSelectedCell); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteTable(void) override { return _to DeleteTable(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteTableCellContents(void) override { return _to DeleteTableCellContents(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteTableCell(int32_t aNumberOfCellsToDelete) override { return _to DeleteTableCell(aNumberOfCellsToDelete); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteTableColumn(int32_t aNumberOfColumnsToDelete) override { return _to DeleteTableColumn(aNumberOfColumnsToDelete); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteTableRow(int32_t aNumberOfRowsToDelete) override { return _to DeleteTableRow(aNumberOfRowsToDelete); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectTableCell(void) override { return _to SelectTableCell(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectTableRow(void) override { return _to SelectTableRow(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectTableColumn(void) override { return _to SelectTableColumn(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectTable(void) override { return _to SelectTable(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectAllTableCells(void) override { return _to SelectAllTableCells(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SwitchTableCellHeaderType(mozilla::dom::Element *aSourceCell, mozilla::dom::Element **_retval) override { return _to SwitchTableCellHeaderType(aSourceCell, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD JoinTableCells(bool aMergeNonContiguousContents) override { return _to JoinTableCells(aMergeNonContiguousContents); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SplitTableCell(void) override { return _to SplitTableCell(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD NormalizeTable(mozilla::dom::Element *aTable) override { return _to NormalizeTable(aTable); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetCellIndexes(mozilla::dom::Element *aCellElement, int32_t *aRowIndex, int32_t *aColumnIndex) override { return _to GetCellIndexes(aCellElement, aRowIndex, aColumnIndex); } \
  NS_IMETHOD GetTableSize(mozilla::dom::Element *aTableOrElementInTable, int32_t *aRowCount, int32_t *aColCount) override { return _to GetTableSize(aTableOrElementInTable, aRowCount, aColCount); } \
  NS_IMETHOD GetCellAt(mozilla::dom::Element *aTableElement, int32_t aRowIndex, int32_t aColumnIndex, mozilla::dom::Element **_retval) override { return _to GetCellAt(aTableElement, aRowIndex, aColumnIndex, _retval); } \
  NS_IMETHOD GetCellDataAt(mozilla::dom::Element *aTableElement, int32_t aRowIndex, int32_t aColumnIndex, mozilla::dom::Element **aCellElement, int32_t *aStartRowIndex, int32_t *aStartColumnIndex, int32_t *aRowSpan, int32_t *aColSpan, int32_t *aEffectiveRowSpan, int32_t *aEffectiveColSpan, bool *aIsSelected) override { return _to GetCellDataAt(aTableElement, aRowIndex, aColumnIndex, aCellElement, aStartRowIndex, aStartColumnIndex, aRowSpan, aColSpan, aEffectiveRowSpan, aEffectiveColSpan, aIsSelected); } \
  NS_IMETHOD GetFirstRow(mozilla::dom::Element *aTableElement, mozilla::dom::Element **_retval) override { return _to GetFirstRow(aTableElement, _retval); } \
  NS_IMETHOD GetSelectedOrParentTableElement(nsAString& aTagName, int32_t *aCount, mozilla::dom::Element **_retval) override { return _to GetSelectedOrParentTableElement(aTagName, aCount, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetSelectedCellsType(mozilla::dom::Element *aElement, uint32_t *_retval) override { return _to GetSelectedCellsType(aElement, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetFirstSelectedCellInTable(int32_t *aRowIndex, int32_t *aColIndex, mozilla::dom::Element **_retval) override { return _to GetFirstSelectedCellInTable(aRowIndex, aColIndex, _retval); } \
  NS_IMETHOD GetSelectedCells(nsTArray<RefPtr<mozilla::dom::Element>>& _retval) override { return _to GetSelectedCells(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITABLEEDITOR(_to) \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertTableCell(int32_t aNumberOfColumnsToInsert, bool aInsertAfterSelectedCell) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InsertTableCell(aNumberOfColumnsToInsert, aInsertAfterSelectedCell); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertTableColumn(int32_t aNumberOfColumnsToInsert, bool aInsertAfterSelectedCell) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InsertTableColumn(aNumberOfColumnsToInsert, aInsertAfterSelectedCell); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertTableRow(int32_t aNumberOfRowsToInsert, bool aInsertAfterSelectedCell) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InsertTableRow(aNumberOfRowsToInsert, aInsertAfterSelectedCell); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteTable(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DeleteTable(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteTableCellContents(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DeleteTableCellContents(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteTableCell(int32_t aNumberOfCellsToDelete) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DeleteTableCell(aNumberOfCellsToDelete); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteTableColumn(int32_t aNumberOfColumnsToDelete) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DeleteTableColumn(aNumberOfColumnsToDelete); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteTableRow(int32_t aNumberOfRowsToDelete) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DeleteTableRow(aNumberOfRowsToDelete); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectTableCell(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SelectTableCell(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectTableRow(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SelectTableRow(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectTableColumn(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SelectTableColumn(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectTable(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SelectTable(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectAllTableCells(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SelectAllTableCells(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SwitchTableCellHeaderType(mozilla::dom::Element *aSourceCell, mozilla::dom::Element **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SwitchTableCellHeaderType(aSourceCell, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD JoinTableCells(bool aMergeNonContiguousContents) override { return !_to ? NS_ERROR_NULL_POINTER : _to->JoinTableCells(aMergeNonContiguousContents); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SplitTableCell(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SplitTableCell(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD NormalizeTable(mozilla::dom::Element *aTable) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NormalizeTable(aTable); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetCellIndexes(mozilla::dom::Element *aCellElement, int32_t *aRowIndex, int32_t *aColumnIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCellIndexes(aCellElement, aRowIndex, aColumnIndex); } \
  NS_IMETHOD GetTableSize(mozilla::dom::Element *aTableOrElementInTable, int32_t *aRowCount, int32_t *aColCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTableSize(aTableOrElementInTable, aRowCount, aColCount); } \
  NS_IMETHOD GetCellAt(mozilla::dom::Element *aTableElement, int32_t aRowIndex, int32_t aColumnIndex, mozilla::dom::Element **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCellAt(aTableElement, aRowIndex, aColumnIndex, _retval); } \
  NS_IMETHOD GetCellDataAt(mozilla::dom::Element *aTableElement, int32_t aRowIndex, int32_t aColumnIndex, mozilla::dom::Element **aCellElement, int32_t *aStartRowIndex, int32_t *aStartColumnIndex, int32_t *aRowSpan, int32_t *aColSpan, int32_t *aEffectiveRowSpan, int32_t *aEffectiveColSpan, bool *aIsSelected) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCellDataAt(aTableElement, aRowIndex, aColumnIndex, aCellElement, aStartRowIndex, aStartColumnIndex, aRowSpan, aColSpan, aEffectiveRowSpan, aEffectiveColSpan, aIsSelected); } \
  NS_IMETHOD GetFirstRow(mozilla::dom::Element *aTableElement, mozilla::dom::Element **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFirstRow(aTableElement, _retval); } \
  NS_IMETHOD GetSelectedOrParentTableElement(nsAString& aTagName, int32_t *aCount, mozilla::dom::Element **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelectedOrParentTableElement(aTagName, aCount, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetSelectedCellsType(mozilla::dom::Element *aElement, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelectedCellsType(aElement, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetFirstSelectedCellInTable(int32_t *aRowIndex, int32_t *aColIndex, mozilla::dom::Element **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFirstSelectedCellInTable(aRowIndex, aColIndex, _retval); } \
  NS_IMETHOD GetSelectedCells(nsTArray<RefPtr<mozilla::dom::Element>>& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelectedCells(_retval); } 


#endif /* __gen_nsITableEditor_h__ */
