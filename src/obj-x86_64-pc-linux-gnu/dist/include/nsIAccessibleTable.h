/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleTable.idl
 */

#ifndef __gen_nsIAccessibleTable_h__
#define __gen_nsIAccessibleTable_h__


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
class nsIAccessible; /* forward declaration */

class nsIArray; /* forward declaration */


/* starting interface:    nsIAccessibleTable */
#define NS_IACCESSIBLETABLE_IID_STR "cb0bf7b9-117e-40e2-9e46-189c3d43ce4a"

#define NS_IACCESSIBLETABLE_IID \
  {0xcb0bf7b9, 0x117e, 0x40e2, \
    { 0x9e, 0x46, 0x18, 0x9c, 0x3d, 0x43, 0xce, 0x4a }}

class NS_NO_VTABLE nsIAccessibleTable : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IACCESSIBLETABLE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAccessibleTable;

  /* readonly attribute nsIAccessible caption; */
  NS_IMETHOD GetCaption(nsIAccessible **aCaption) = 0;

  /* readonly attribute AString summary; */
  NS_IMETHOD GetSummary(nsAString& aSummary) = 0;

  /* readonly attribute long columnCount; */
  NS_IMETHOD GetColumnCount(int32_t *aColumnCount) = 0;

  /* readonly attribute long rowCount; */
  NS_IMETHOD GetRowCount(int32_t *aRowCount) = 0;

  /* nsIAccessible getCellAt (in long rowIndex, in long columnIndex); */
  NS_IMETHOD GetCellAt(int32_t rowIndex, int32_t columnIndex, nsIAccessible **_retval) = 0;

  /* long getCellIndexAt (in long rowIndex, in long columnIndex); */
  NS_IMETHOD GetCellIndexAt(int32_t rowIndex, int32_t columnIndex, int32_t *_retval) = 0;

  /* long getColumnIndexAt (in long cellIndex); */
  NS_IMETHOD GetColumnIndexAt(int32_t cellIndex, int32_t *_retval) = 0;

  /* long getRowIndexAt (in long cellIndex); */
  NS_IMETHOD GetRowIndexAt(int32_t cellIndex, int32_t *_retval) = 0;

  /* void getRowAndColumnIndicesAt (in long cellIndex, out long rowIndex, out long columnIndex); */
  NS_IMETHOD GetRowAndColumnIndicesAt(int32_t cellIndex, int32_t *rowIndex, int32_t *columnIndex) = 0;

  /* long getColumnExtentAt (in long row, in long column); */
  NS_IMETHOD GetColumnExtentAt(int32_t row, int32_t column, int32_t *_retval) = 0;

  /* long getRowExtentAt (in long row, in long column); */
  NS_IMETHOD GetRowExtentAt(int32_t row, int32_t column, int32_t *_retval) = 0;

  /* AString getColumnDescription (in long columnIndex); */
  NS_IMETHOD GetColumnDescription(int32_t columnIndex, nsAString& _retval) = 0;

  /* AString getRowDescription (in long rowIndex); */
  NS_IMETHOD GetRowDescription(int32_t rowIndex, nsAString& _retval) = 0;

  /* boolean isColumnSelected (in long columnIndex); */
  NS_IMETHOD IsColumnSelected(int32_t columnIndex, bool *_retval) = 0;

  /* boolean isRowSelected (in long rowIndex); */
  NS_IMETHOD IsRowSelected(int32_t rowIndex, bool *_retval) = 0;

  /* boolean isCellSelected (in long rowIndex, in long columnIndex); */
  NS_IMETHOD IsCellSelected(int32_t rowIndex, int32_t columnIndex, bool *_retval) = 0;

  /* readonly attribute unsigned long selectedCellCount; */
  NS_IMETHOD GetSelectedCellCount(uint32_t *aSelectedCellCount) = 0;

  /* readonly attribute unsigned long selectedColumnCount; */
  NS_IMETHOD GetSelectedColumnCount(uint32_t *aSelectedColumnCount) = 0;

  /* readonly attribute unsigned long selectedRowCount; */
  NS_IMETHOD GetSelectedRowCount(uint32_t *aSelectedRowCount) = 0;

  /* readonly attribute nsIArray selectedCells; */
  NS_IMETHOD GetSelectedCells(nsIArray **aSelectedCells) = 0;

  /* Array<uint32_t> getSelectedCellIndices (); */
  NS_IMETHOD GetSelectedCellIndices(nsTArray<uint32_t >& _retval) = 0;

  /* Array<uint32_t> getSelectedColumnIndices (); */
  NS_IMETHOD GetSelectedColumnIndices(nsTArray<uint32_t >& _retval) = 0;

  /* Array<uint32_t> getSelectedRowIndices (); */
  NS_IMETHOD GetSelectedRowIndices(nsTArray<uint32_t >& _retval) = 0;

  /* void selectRow (in long rowIndex); */
  NS_IMETHOD SelectRow(int32_t rowIndex) = 0;

  /* void selectColumn (in long columnIndex); */
  NS_IMETHOD SelectColumn(int32_t columnIndex) = 0;

  /* void unselectRow (in long rowIndex); */
  NS_IMETHOD UnselectRow(int32_t rowIndex) = 0;

  /* void unselectColumn (in long columnIndex); */
  NS_IMETHOD UnselectColumn(int32_t columnIndex) = 0;

  /* boolean isProbablyForLayout (); */
  NS_IMETHOD IsProbablyForLayout(bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAccessibleTable, NS_IACCESSIBLETABLE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIACCESSIBLETABLE \
  NS_IMETHOD GetCaption(nsIAccessible **aCaption) override; \
  NS_IMETHOD GetSummary(nsAString& aSummary) override; \
  NS_IMETHOD GetColumnCount(int32_t *aColumnCount) override; \
  NS_IMETHOD GetRowCount(int32_t *aRowCount) override; \
  NS_IMETHOD GetCellAt(int32_t rowIndex, int32_t columnIndex, nsIAccessible **_retval) override; \
  NS_IMETHOD GetCellIndexAt(int32_t rowIndex, int32_t columnIndex, int32_t *_retval) override; \
  NS_IMETHOD GetColumnIndexAt(int32_t cellIndex, int32_t *_retval) override; \
  NS_IMETHOD GetRowIndexAt(int32_t cellIndex, int32_t *_retval) override; \
  NS_IMETHOD GetRowAndColumnIndicesAt(int32_t cellIndex, int32_t *rowIndex, int32_t *columnIndex) override; \
  NS_IMETHOD GetColumnExtentAt(int32_t row, int32_t column, int32_t *_retval) override; \
  NS_IMETHOD GetRowExtentAt(int32_t row, int32_t column, int32_t *_retval) override; \
  NS_IMETHOD GetColumnDescription(int32_t columnIndex, nsAString& _retval) override; \
  NS_IMETHOD GetRowDescription(int32_t rowIndex, nsAString& _retval) override; \
  NS_IMETHOD IsColumnSelected(int32_t columnIndex, bool *_retval) override; \
  NS_IMETHOD IsRowSelected(int32_t rowIndex, bool *_retval) override; \
  NS_IMETHOD IsCellSelected(int32_t rowIndex, int32_t columnIndex, bool *_retval) override; \
  NS_IMETHOD GetSelectedCellCount(uint32_t *aSelectedCellCount) override; \
  NS_IMETHOD GetSelectedColumnCount(uint32_t *aSelectedColumnCount) override; \
  NS_IMETHOD GetSelectedRowCount(uint32_t *aSelectedRowCount) override; \
  NS_IMETHOD GetSelectedCells(nsIArray **aSelectedCells) override; \
  NS_IMETHOD GetSelectedCellIndices(nsTArray<uint32_t >& _retval) override; \
  NS_IMETHOD GetSelectedColumnIndices(nsTArray<uint32_t >& _retval) override; \
  NS_IMETHOD GetSelectedRowIndices(nsTArray<uint32_t >& _retval) override; \
  NS_IMETHOD SelectRow(int32_t rowIndex) override; \
  NS_IMETHOD SelectColumn(int32_t columnIndex) override; \
  NS_IMETHOD UnselectRow(int32_t rowIndex) override; \
  NS_IMETHOD UnselectColumn(int32_t columnIndex) override; \
  NS_IMETHOD IsProbablyForLayout(bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIACCESSIBLETABLE \
  nsresult GetCaption(nsIAccessible **aCaption); \
  nsresult GetSummary(nsAString& aSummary); \
  nsresult GetColumnCount(int32_t *aColumnCount); \
  nsresult GetRowCount(int32_t *aRowCount); \
  nsresult GetCellAt(int32_t rowIndex, int32_t columnIndex, nsIAccessible **_retval); \
  nsresult GetCellIndexAt(int32_t rowIndex, int32_t columnIndex, int32_t *_retval); \
  nsresult GetColumnIndexAt(int32_t cellIndex, int32_t *_retval); \
  nsresult GetRowIndexAt(int32_t cellIndex, int32_t *_retval); \
  nsresult GetRowAndColumnIndicesAt(int32_t cellIndex, int32_t *rowIndex, int32_t *columnIndex); \
  nsresult GetColumnExtentAt(int32_t row, int32_t column, int32_t *_retval); \
  nsresult GetRowExtentAt(int32_t row, int32_t column, int32_t *_retval); \
  nsresult GetColumnDescription(int32_t columnIndex, nsAString& _retval); \
  nsresult GetRowDescription(int32_t rowIndex, nsAString& _retval); \
  nsresult IsColumnSelected(int32_t columnIndex, bool *_retval); \
  nsresult IsRowSelected(int32_t rowIndex, bool *_retval); \
  nsresult IsCellSelected(int32_t rowIndex, int32_t columnIndex, bool *_retval); \
  nsresult GetSelectedCellCount(uint32_t *aSelectedCellCount); \
  nsresult GetSelectedColumnCount(uint32_t *aSelectedColumnCount); \
  nsresult GetSelectedRowCount(uint32_t *aSelectedRowCount); \
  nsresult GetSelectedCells(nsIArray **aSelectedCells); \
  nsresult GetSelectedCellIndices(nsTArray<uint32_t >& _retval); \
  nsresult GetSelectedColumnIndices(nsTArray<uint32_t >& _retval); \
  nsresult GetSelectedRowIndices(nsTArray<uint32_t >& _retval); \
  nsresult SelectRow(int32_t rowIndex); \
  nsresult SelectColumn(int32_t columnIndex); \
  nsresult UnselectRow(int32_t rowIndex); \
  nsresult UnselectColumn(int32_t columnIndex); \
  nsresult IsProbablyForLayout(bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIACCESSIBLETABLE(_to) \
  NS_IMETHOD GetCaption(nsIAccessible **aCaption) override { return _to GetCaption(aCaption); } \
  NS_IMETHOD GetSummary(nsAString& aSummary) override { return _to GetSummary(aSummary); } \
  NS_IMETHOD GetColumnCount(int32_t *aColumnCount) override { return _to GetColumnCount(aColumnCount); } \
  NS_IMETHOD GetRowCount(int32_t *aRowCount) override { return _to GetRowCount(aRowCount); } \
  NS_IMETHOD GetCellAt(int32_t rowIndex, int32_t columnIndex, nsIAccessible **_retval) override { return _to GetCellAt(rowIndex, columnIndex, _retval); } \
  NS_IMETHOD GetCellIndexAt(int32_t rowIndex, int32_t columnIndex, int32_t *_retval) override { return _to GetCellIndexAt(rowIndex, columnIndex, _retval); } \
  NS_IMETHOD GetColumnIndexAt(int32_t cellIndex, int32_t *_retval) override { return _to GetColumnIndexAt(cellIndex, _retval); } \
  NS_IMETHOD GetRowIndexAt(int32_t cellIndex, int32_t *_retval) override { return _to GetRowIndexAt(cellIndex, _retval); } \
  NS_IMETHOD GetRowAndColumnIndicesAt(int32_t cellIndex, int32_t *rowIndex, int32_t *columnIndex) override { return _to GetRowAndColumnIndicesAt(cellIndex, rowIndex, columnIndex); } \
  NS_IMETHOD GetColumnExtentAt(int32_t row, int32_t column, int32_t *_retval) override { return _to GetColumnExtentAt(row, column, _retval); } \
  NS_IMETHOD GetRowExtentAt(int32_t row, int32_t column, int32_t *_retval) override { return _to GetRowExtentAt(row, column, _retval); } \
  NS_IMETHOD GetColumnDescription(int32_t columnIndex, nsAString& _retval) override { return _to GetColumnDescription(columnIndex, _retval); } \
  NS_IMETHOD GetRowDescription(int32_t rowIndex, nsAString& _retval) override { return _to GetRowDescription(rowIndex, _retval); } \
  NS_IMETHOD IsColumnSelected(int32_t columnIndex, bool *_retval) override { return _to IsColumnSelected(columnIndex, _retval); } \
  NS_IMETHOD IsRowSelected(int32_t rowIndex, bool *_retval) override { return _to IsRowSelected(rowIndex, _retval); } \
  NS_IMETHOD IsCellSelected(int32_t rowIndex, int32_t columnIndex, bool *_retval) override { return _to IsCellSelected(rowIndex, columnIndex, _retval); } \
  NS_IMETHOD GetSelectedCellCount(uint32_t *aSelectedCellCount) override { return _to GetSelectedCellCount(aSelectedCellCount); } \
  NS_IMETHOD GetSelectedColumnCount(uint32_t *aSelectedColumnCount) override { return _to GetSelectedColumnCount(aSelectedColumnCount); } \
  NS_IMETHOD GetSelectedRowCount(uint32_t *aSelectedRowCount) override { return _to GetSelectedRowCount(aSelectedRowCount); } \
  NS_IMETHOD GetSelectedCells(nsIArray **aSelectedCells) override { return _to GetSelectedCells(aSelectedCells); } \
  NS_IMETHOD GetSelectedCellIndices(nsTArray<uint32_t >& _retval) override { return _to GetSelectedCellIndices(_retval); } \
  NS_IMETHOD GetSelectedColumnIndices(nsTArray<uint32_t >& _retval) override { return _to GetSelectedColumnIndices(_retval); } \
  NS_IMETHOD GetSelectedRowIndices(nsTArray<uint32_t >& _retval) override { return _to GetSelectedRowIndices(_retval); } \
  NS_IMETHOD SelectRow(int32_t rowIndex) override { return _to SelectRow(rowIndex); } \
  NS_IMETHOD SelectColumn(int32_t columnIndex) override { return _to SelectColumn(columnIndex); } \
  NS_IMETHOD UnselectRow(int32_t rowIndex) override { return _to UnselectRow(rowIndex); } \
  NS_IMETHOD UnselectColumn(int32_t columnIndex) override { return _to UnselectColumn(columnIndex); } \
  NS_IMETHOD IsProbablyForLayout(bool *_retval) override { return _to IsProbablyForLayout(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIACCESSIBLETABLE(_to) \
  NS_IMETHOD GetCaption(nsIAccessible **aCaption) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCaption(aCaption); } \
  NS_IMETHOD GetSummary(nsAString& aSummary) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSummary(aSummary); } \
  NS_IMETHOD GetColumnCount(int32_t *aColumnCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetColumnCount(aColumnCount); } \
  NS_IMETHOD GetRowCount(int32_t *aRowCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRowCount(aRowCount); } \
  NS_IMETHOD GetCellAt(int32_t rowIndex, int32_t columnIndex, nsIAccessible **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCellAt(rowIndex, columnIndex, _retval); } \
  NS_IMETHOD GetCellIndexAt(int32_t rowIndex, int32_t columnIndex, int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCellIndexAt(rowIndex, columnIndex, _retval); } \
  NS_IMETHOD GetColumnIndexAt(int32_t cellIndex, int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetColumnIndexAt(cellIndex, _retval); } \
  NS_IMETHOD GetRowIndexAt(int32_t cellIndex, int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRowIndexAt(cellIndex, _retval); } \
  NS_IMETHOD GetRowAndColumnIndicesAt(int32_t cellIndex, int32_t *rowIndex, int32_t *columnIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRowAndColumnIndicesAt(cellIndex, rowIndex, columnIndex); } \
  NS_IMETHOD GetColumnExtentAt(int32_t row, int32_t column, int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetColumnExtentAt(row, column, _retval); } \
  NS_IMETHOD GetRowExtentAt(int32_t row, int32_t column, int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRowExtentAt(row, column, _retval); } \
  NS_IMETHOD GetColumnDescription(int32_t columnIndex, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetColumnDescription(columnIndex, _retval); } \
  NS_IMETHOD GetRowDescription(int32_t rowIndex, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRowDescription(rowIndex, _retval); } \
  NS_IMETHOD IsColumnSelected(int32_t columnIndex, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsColumnSelected(columnIndex, _retval); } \
  NS_IMETHOD IsRowSelected(int32_t rowIndex, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsRowSelected(rowIndex, _retval); } \
  NS_IMETHOD IsCellSelected(int32_t rowIndex, int32_t columnIndex, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsCellSelected(rowIndex, columnIndex, _retval); } \
  NS_IMETHOD GetSelectedCellCount(uint32_t *aSelectedCellCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelectedCellCount(aSelectedCellCount); } \
  NS_IMETHOD GetSelectedColumnCount(uint32_t *aSelectedColumnCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelectedColumnCount(aSelectedColumnCount); } \
  NS_IMETHOD GetSelectedRowCount(uint32_t *aSelectedRowCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelectedRowCount(aSelectedRowCount); } \
  NS_IMETHOD GetSelectedCells(nsIArray **aSelectedCells) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelectedCells(aSelectedCells); } \
  NS_IMETHOD GetSelectedCellIndices(nsTArray<uint32_t >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelectedCellIndices(_retval); } \
  NS_IMETHOD GetSelectedColumnIndices(nsTArray<uint32_t >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelectedColumnIndices(_retval); } \
  NS_IMETHOD GetSelectedRowIndices(nsTArray<uint32_t >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelectedRowIndices(_retval); } \
  NS_IMETHOD SelectRow(int32_t rowIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SelectRow(rowIndex); } \
  NS_IMETHOD SelectColumn(int32_t columnIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SelectColumn(columnIndex); } \
  NS_IMETHOD UnselectRow(int32_t rowIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UnselectRow(rowIndex); } \
  NS_IMETHOD UnselectColumn(int32_t columnIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UnselectColumn(columnIndex); } \
  NS_IMETHOD IsProbablyForLayout(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsProbablyForLayout(_retval); } 


/* starting interface:    nsIAccessibleTableCell */
#define NS_IACCESSIBLETABLECELL_IID_STR "654e296d-fae6-452b-987d-746b20b9514b"

#define NS_IACCESSIBLETABLECELL_IID \
  {0x654e296d, 0xfae6, 0x452b, \
    { 0x98, 0x7d, 0x74, 0x6b, 0x20, 0xb9, 0x51, 0x4b }}

class NS_NO_VTABLE nsIAccessibleTableCell : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IACCESSIBLETABLECELL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAccessibleTableCell;

  /* readonly attribute nsIAccessibleTable table; */
  NS_IMETHOD GetTable(nsIAccessibleTable **aTable) = 0;

  /* readonly attribute long columnIndex; */
  NS_IMETHOD GetColumnIndex(int32_t *aColumnIndex) = 0;

  /* readonly attribute long rowIndex; */
  NS_IMETHOD GetRowIndex(int32_t *aRowIndex) = 0;

  /* readonly attribute long columnExtent; */
  NS_IMETHOD GetColumnExtent(int32_t *aColumnExtent) = 0;

  /* readonly attribute long rowExtent; */
  NS_IMETHOD GetRowExtent(int32_t *aRowExtent) = 0;

  /* readonly attribute nsIArray columnHeaderCells; */
  NS_IMETHOD GetColumnHeaderCells(nsIArray **aColumnHeaderCells) = 0;

  /* readonly attribute nsIArray rowHeaderCells; */
  NS_IMETHOD GetRowHeaderCells(nsIArray **aRowHeaderCells) = 0;

  /* boolean isSelected (); */
  NS_IMETHOD IsSelected(bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAccessibleTableCell, NS_IACCESSIBLETABLECELL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIACCESSIBLETABLECELL \
  NS_IMETHOD GetTable(nsIAccessibleTable **aTable) override; \
  NS_IMETHOD GetColumnIndex(int32_t *aColumnIndex) override; \
  NS_IMETHOD GetRowIndex(int32_t *aRowIndex) override; \
  NS_IMETHOD GetColumnExtent(int32_t *aColumnExtent) override; \
  NS_IMETHOD GetRowExtent(int32_t *aRowExtent) override; \
  NS_IMETHOD GetColumnHeaderCells(nsIArray **aColumnHeaderCells) override; \
  NS_IMETHOD GetRowHeaderCells(nsIArray **aRowHeaderCells) override; \
  NS_IMETHOD IsSelected(bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIACCESSIBLETABLECELL \
  nsresult GetTable(nsIAccessibleTable **aTable); \
  nsresult GetColumnIndex(int32_t *aColumnIndex); \
  nsresult GetRowIndex(int32_t *aRowIndex); \
  nsresult GetColumnExtent(int32_t *aColumnExtent); \
  nsresult GetRowExtent(int32_t *aRowExtent); \
  nsresult GetColumnHeaderCells(nsIArray **aColumnHeaderCells); \
  nsresult GetRowHeaderCells(nsIArray **aRowHeaderCells); \
  nsresult IsSelected(bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIACCESSIBLETABLECELL(_to) \
  NS_IMETHOD GetTable(nsIAccessibleTable **aTable) override { return _to GetTable(aTable); } \
  NS_IMETHOD GetColumnIndex(int32_t *aColumnIndex) override { return _to GetColumnIndex(aColumnIndex); } \
  NS_IMETHOD GetRowIndex(int32_t *aRowIndex) override { return _to GetRowIndex(aRowIndex); } \
  NS_IMETHOD GetColumnExtent(int32_t *aColumnExtent) override { return _to GetColumnExtent(aColumnExtent); } \
  NS_IMETHOD GetRowExtent(int32_t *aRowExtent) override { return _to GetRowExtent(aRowExtent); } \
  NS_IMETHOD GetColumnHeaderCells(nsIArray **aColumnHeaderCells) override { return _to GetColumnHeaderCells(aColumnHeaderCells); } \
  NS_IMETHOD GetRowHeaderCells(nsIArray **aRowHeaderCells) override { return _to GetRowHeaderCells(aRowHeaderCells); } \
  NS_IMETHOD IsSelected(bool *_retval) override { return _to IsSelected(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIACCESSIBLETABLECELL(_to) \
  NS_IMETHOD GetTable(nsIAccessibleTable **aTable) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTable(aTable); } \
  NS_IMETHOD GetColumnIndex(int32_t *aColumnIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetColumnIndex(aColumnIndex); } \
  NS_IMETHOD GetRowIndex(int32_t *aRowIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRowIndex(aRowIndex); } \
  NS_IMETHOD GetColumnExtent(int32_t *aColumnExtent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetColumnExtent(aColumnExtent); } \
  NS_IMETHOD GetRowExtent(int32_t *aRowExtent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRowExtent(aRowExtent); } \
  NS_IMETHOD GetColumnHeaderCells(nsIArray **aColumnHeaderCells) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetColumnHeaderCells(aColumnHeaderCells); } \
  NS_IMETHOD GetRowHeaderCells(nsIArray **aRowHeaderCells) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRowHeaderCells(aRowHeaderCells); } \
  NS_IMETHOD IsSelected(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsSelected(_retval); } 


#endif /* __gen_nsIAccessibleTable_h__ */
