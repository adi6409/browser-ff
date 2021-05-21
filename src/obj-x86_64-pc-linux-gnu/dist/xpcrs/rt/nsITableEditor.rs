//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/nsITableEditor.idl
//


/// `interface nsITableEditor : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITableEditor {
    vtable: *const nsITableEditorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITableEditor.
unsafe impl XpCom for nsITableEditor {
    const IID: nsIID = nsID(0x4805e684, 0x49b9, 0x11d3,
        [0x9c, 0xe4, 0xed, 0x60, 0xbd, 0x6c, 0xb5, 0xbc]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITableEditor {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITableEditor.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITableEditorCoerce {
    /// Cheaply cast a value of this type from a `nsITableEditor`.
    fn coerce_from(v: &nsITableEditor) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITableEditorCoerce for nsITableEditor {
    #[inline]
    fn coerce_from(v: &nsITableEditor) -> &Self {
        v
    }
}

impl nsITableEditor {
    /// Cast this `nsITableEditor` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITableEditorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITableEditor {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsCoerce> nsITableEditorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITableEditor) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITableEditor
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITableEditorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [can_run_script] void insertTableCell (in long aNumberOfColumnsToInsert, in boolean aInsertAfterSelectedCell); */
    pub InsertTableCell: unsafe extern "system" fn (this: *const nsITableEditor, aNumberOfColumnsToInsert: i32, aInsertAfterSelectedCell: bool) -> ::nserror::nsresult,

    /* [can_run_script] void insertTableColumn (in long aNumberOfColumnsToInsert, in boolean aInsertAfterSelectedCell); */
    pub InsertTableColumn: unsafe extern "system" fn (this: *const nsITableEditor, aNumberOfColumnsToInsert: i32, aInsertAfterSelectedCell: bool) -> ::nserror::nsresult,

    /* [can_run_script] void insertTableRow (in long aNumberOfRowsToInsert, in boolean aInsertAfterSelectedCell); */
    pub InsertTableRow: unsafe extern "system" fn (this: *const nsITableEditor, aNumberOfRowsToInsert: i32, aInsertAfterSelectedCell: bool) -> ::nserror::nsresult,

    /* [can_run_script] void deleteTable (); */
    pub DeleteTable: unsafe extern "system" fn (this: *const nsITableEditor) -> ::nserror::nsresult,

    /* [can_run_script] void deleteTableCellContents (); */
    pub DeleteTableCellContents: unsafe extern "system" fn (this: *const nsITableEditor) -> ::nserror::nsresult,

    /* [can_run_script] void deleteTableCell (in long aNumberOfCellsToDelete); */
    pub DeleteTableCell: unsafe extern "system" fn (this: *const nsITableEditor, aNumberOfCellsToDelete: i32) -> ::nserror::nsresult,

    /* [can_run_script] void deleteTableColumn (in long aNumberOfColumnsToDelete); */
    pub DeleteTableColumn: unsafe extern "system" fn (this: *const nsITableEditor, aNumberOfColumnsToDelete: i32) -> ::nserror::nsresult,

    /* [can_run_script] void deleteTableRow (in long aNumberOfRowsToDelete); */
    pub DeleteTableRow: unsafe extern "system" fn (this: *const nsITableEditor, aNumberOfRowsToDelete: i32) -> ::nserror::nsresult,

    /* [can_run_script] void selectTableCell (); */
    pub SelectTableCell: unsafe extern "system" fn (this: *const nsITableEditor) -> ::nserror::nsresult,

    /* [can_run_script] void selectTableRow (); */
    pub SelectTableRow: unsafe extern "system" fn (this: *const nsITableEditor) -> ::nserror::nsresult,

    /* [can_run_script] void selectTableColumn (); */
    pub SelectTableColumn: unsafe extern "system" fn (this: *const nsITableEditor) -> ::nserror::nsresult,

    /* [can_run_script] void selectTable (); */
    pub SelectTable: unsafe extern "system" fn (this: *const nsITableEditor) -> ::nserror::nsresult,

    /* [can_run_script] void selectAllTableCells (); */
    pub SelectAllTableCells: unsafe extern "system" fn (this: *const nsITableEditor) -> ::nserror::nsresult,

    /* [can_run_script] Element switchTableCellHeaderType (in Element aSourceCell); */
    pub SwitchTableCellHeaderType: unsafe extern "system" fn (this: *const nsITableEditor, aSourceCell: *const libc::c_void, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* [can_run_script] void joinTableCells (in boolean aMergeNonContiguousContents); */
    pub JoinTableCells: unsafe extern "system" fn (this: *const nsITableEditor, aMergeNonContiguousContents: bool) -> ::nserror::nsresult,

    /* [can_run_script] void splitTableCell (); */
    pub SplitTableCell: unsafe extern "system" fn (this: *const nsITableEditor) -> ::nserror::nsresult,

    /* [can_run_script] void normalizeTable (in Element aTable); */
    pub NormalizeTable: unsafe extern "system" fn (this: *const nsITableEditor, aTable: *const libc::c_void) -> ::nserror::nsresult,

    /* [can_run_script] void getCellIndexes (in Element aCellElement, out long aRowIndex, out long aColumnIndex); */
    pub GetCellIndexes: unsafe extern "system" fn (this: *const nsITableEditor, aCellElement: *const libc::c_void, aRowIndex: *mut i32, aColumnIndex: *mut i32) -> ::nserror::nsresult,

    /* void getTableSize (in Element aTableOrElementInTable, out long aRowCount, out long aColCount); */
    pub GetTableSize: unsafe extern "system" fn (this: *const nsITableEditor, aTableOrElementInTable: *const libc::c_void, aRowCount: *mut i32, aColCount: *mut i32) -> ::nserror::nsresult,

    /* Element getCellAt (in Element aTableElement, in long aRowIndex, in long aColumnIndex); */
    pub GetCellAt: unsafe extern "system" fn (this: *const nsITableEditor, aTableElement: *const libc::c_void, aRowIndex: i32, aColumnIndex: i32, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* void getCellDataAt (in Element aTableElement, in long aRowIndex, in long aColumnIndex, out Element aCellElement, out long aStartRowIndex, out long aStartColumnIndex, out long aRowSpan, out long aColSpan, out long aEffectiveRowSpan, out long aEffectiveColSpan, out boolean aIsSelected); */
    pub GetCellDataAt: unsafe extern "system" fn (this: *const nsITableEditor, aTableElement: *const libc::c_void, aRowIndex: i32, aColumnIndex: i32, aCellElement: *mut *const libc::c_void, aStartRowIndex: *mut i32, aStartColumnIndex: *mut i32, aRowSpan: *mut i32, aColSpan: *mut i32, aEffectiveRowSpan: *mut i32, aEffectiveColSpan: *mut i32, aIsSelected: *mut bool) -> ::nserror::nsresult,

    /* Element getFirstRow (in Element aTableElement); */
    pub GetFirstRow: unsafe extern "system" fn (this: *const nsITableEditor, aTableElement: *const libc::c_void, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* Element getSelectedOrParentTableElement (out AString aTagName, out long aCount); */
    pub GetSelectedOrParentTableElement: unsafe extern "system" fn (this: *const nsITableEditor, aTagName: *mut ::nsstring::nsAString, aCount: *mut i32, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* [can_run_script] uint32_t getSelectedCellsType (in Element aElement); */
    pub GetSelectedCellsType: unsafe extern "system" fn (this: *const nsITableEditor, aElement: *const libc::c_void, _retval: *mut uint32_t) -> ::nserror::nsresult,

    /* [can_run_script] Element getFirstSelectedCellInTable (out long aRowIndex, out long aColIndex); */
    pub GetFirstSelectedCellInTable: unsafe extern "system" fn (this: *const nsITableEditor, aRowIndex: *mut i32, aColIndex: *mut i32, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* Array<Element> getSelectedCells (); */
    pub GetSelectedCells: unsafe extern "system" fn (this: *const nsITableEditor, _retval: *mut thin_vec::ThinVec<*const libc::c_void>) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITableEditor {

    pub const eNoSearch: i64 = 0;


    pub const ePreviousColumn: i64 = 1;


    pub const ePreviousRow: i64 = 2;

    /// ```text
    /// /**
    ///    * insertTableCell() inserts <td> elements before or after a cell element
    ///    * containing first selection range.  I.e., if the cell spans columns and
    ///    * aInsertPosition is true, new columns will be inserted after the
    ///    * right-most column which contains the cell.  Note that this simply
    ///    * inserts <td> elements, i.e., colspan and rowspan around the cell
    ///    * containing selection are not modified.  So, for example, adding a cell
    ///    * to rectangular table changes non-rectangular table.  And if a cell
    ///    * containing selection is at left of row-spanning cell, it may be moved to
    ///    * right side of the row-spanning cell after inserting some cell elements
    ///    * before it.  Similarly, colspan won't be adjusted for keeping table
    ///    * rectangle.
    ///    * If first selection range is not in table cell element, this does nothing
    ///    * without exception.
    ///    *
    ///    * @param aNumberOfCellssToInsert     Number of cells to insert.
    ///    * @param aInsertAfterSelectedCell    true if new cells should be inserted
    ///    *                                    before current cell.  Otherwise, will
    ///    *                                    be inserted after the cell.
    ///    */
    /// ```
    ///

    /// `[can_run_script] void insertTableCell (in long aNumberOfColumnsToInsert, in boolean aInsertAfterSelectedCell);`
    #[inline]
    pub unsafe fn InsertTableCell(&self, aNumberOfColumnsToInsert: i32, aInsertAfterSelectedCell: bool) -> ::nserror::nsresult {
        ((*self.vtable).InsertTableCell)(self, aNumberOfColumnsToInsert, aInsertAfterSelectedCell)
    }


    /// ```text
    /// /**
    ///    * insertTableColumn() inserts columns before or after a cell element
    ///    * containing first selection range.  I.e., if the cell spans columns and
    ///    * aInsertAfterSelectedCell is tre, new columns will be inserted after the
    ///    * right-most column which contains the cell.  If first selection range is
    ///    * not in table cell element, this does nothing without exception.
    ///    *
    ///    * @param aNumberOfColumnsToInsert    Number of columns to insert.
    ///    * @param aInsertAfterSelectedCell    true if new columns will be inserted
    ///    *                                    before current cell.  Otherwise, will
    ///    *                                    be inserted after the cell.
    ///    */
    /// ```
    ///

    /// `[can_run_script] void insertTableColumn (in long aNumberOfColumnsToInsert, in boolean aInsertAfterSelectedCell);`
    #[inline]
    pub unsafe fn InsertTableColumn(&self, aNumberOfColumnsToInsert: i32, aInsertAfterSelectedCell: bool) -> ::nserror::nsresult {
        ((*self.vtable).InsertTableColumn)(self, aNumberOfColumnsToInsert, aInsertAfterSelectedCell)
    }



    /// `[can_run_script] void insertTableRow (in long aNumberOfRowsToInsert, in boolean aInsertAfterSelectedCell);`
    #[inline]
    pub unsafe fn InsertTableRow(&self, aNumberOfRowsToInsert: i32, aInsertAfterSelectedCell: bool) -> ::nserror::nsresult {
        ((*self.vtable).InsertTableRow)(self, aNumberOfRowsToInsert, aInsertAfterSelectedCell)
    }


    /// ```text
    /// /** Delete table methods
    ///     * Delete starting at the selected cell or the
    ///     *  cell (or table) enclosing the selection anchor
    ///     * The selection is collapsed and is left in the
    ///     *  cell at the same row,col location as
    ///     *  the previous selection anchor, if possible,
    ///     *  else in the closest neighboring cell
    ///     *
    ///     * @param aNumber    Number of items to insert/delete
    ///     */
    /// ```
    ///

    /// `[can_run_script] void deleteTable ();`
    #[inline]
    pub unsafe fn DeleteTable(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).DeleteTable)(self, )
    }


    /// ```text
    /// /**
    ///    * deleteTableCellContents() removes any contents in cell elements.  If two
    ///    * or more cell elements are selected, this removes all selected cells'
    ///    * contents.  Otherwise, this removes contents of a cell which contains
    ///    * first selection range.  This does nothing without exception if selection
    ///    * is not in cell element.
    ///    */
    /// ```
    ///

    /// `[can_run_script] void deleteTableCellContents ();`
    #[inline]
    pub unsafe fn DeleteTableCellContents(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).DeleteTableCellContents)(self, )
    }


    /// ```text
    /// /**
    ///    * deleteTableCell() removes table cell elements.  If two or more cell
    ///    * elements are selected, this removes all selected cell elements.
    ///    * Otherwise, this removes some cell elements starting from selected cell
    ///    * element or a cell containing first selection range.  When this removes
    ///    * last cell element in <tr> or <table>, this removes the <tr> or the
    ///    * <table> too.  Note that when removing a cell causes number of its row
    ///    * becomes less than the others, this method does NOT fill the place with
    ///    * rowspan nor colspan.  This does nothing without exception if selection is
    ///    * not in cell element.
    ///    *
    ///    * @param aNumberOfCellsToDelete  Number of cells to remove.  This is ignored
    ///    *                                if 2 or more cells are selected.
    ///    */
    /// ```
    ///

    /// `[can_run_script] void deleteTableCell (in long aNumberOfCellsToDelete);`
    #[inline]
    pub unsafe fn DeleteTableCell(&self, aNumberOfCellsToDelete: i32) -> ::nserror::nsresult {
        ((*self.vtable).DeleteTableCell)(self, aNumberOfCellsToDelete)
    }


    /// ```text
    /// /**
    ///    * deleteTableColumn() removes cell elements which belong to same columns
    ///    * of selected cell elements.
    ///    * If only one cell element is selected or first selection range is
    ///    * in a cell, removes cell elements which belong to same column.
    ///    * If 2 or more cell elements are selected, removes cell elements which
    ///    * belong to any of all selected columns.  In this case,
    ///    * aNumberOfColumnsToDelete is ignored.
    ///    * If there is no selection ranges, throws exception.
    ///    * If selection is not in a cell element, just does nothing without
    ///    * throwing exception.
    ///    * WARNING: This does not remove <col> nor <colgroup> elements.
    ///    *
    ///    * @param aNumberOfColumnsToDelete    Number of columns to remove.  This is
    ///    *                                    ignored if 2 ore more cells are
    ///    *                                    selected.
    ///    */
    /// ```
    ///

    /// `[can_run_script] void deleteTableColumn (in long aNumberOfColumnsToDelete);`
    #[inline]
    pub unsafe fn DeleteTableColumn(&self, aNumberOfColumnsToDelete: i32) -> ::nserror::nsresult {
        ((*self.vtable).DeleteTableColumn)(self, aNumberOfColumnsToDelete)
    }


    /// ```text
    /// /**
    ///    * deleteTableRow() removes <tr> elements.
    ///    * If only one cell element is selected or first selection range is
    ///    * in a cell, removes <tr> elements starting from a <tr> element
    ///    * containing the selected cell or first selection range.
    ///    * If 2 or more cell elements are selected, all <tr> elements
    ///    * which contains selected cell(s).  In this case, aNumberOfRowsToDelete
    ///    * is ignored.
    ///    * If there is no selection ranges, throws exception.
    ///    * If selection is not in a cell element, just does nothing without
    ///    * throwing exception.
    ///    *
    ///    * @param aNumberOfRowsToDelete   Number of rows to remove.  This is ignored
    ///    *                                if 2 or more cells are selected.
    ///    */
    /// ```
    ///

    /// `[can_run_script] void deleteTableRow (in long aNumberOfRowsToDelete);`
    #[inline]
    pub unsafe fn DeleteTableRow(&self, aNumberOfRowsToDelete: i32) -> ::nserror::nsresult {
        ((*self.vtable).DeleteTableRow)(self, aNumberOfRowsToDelete)
    }


    /// ```text
    /// /** Table Selection methods
    ///     * Selecting a row or column actually
    ///     * selects all cells (not TR in the case of rows)
    ///     */
    /// ```
    ///

    /// `[can_run_script] void selectTableCell ();`
    #[inline]
    pub unsafe fn SelectTableCell(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SelectTableCell)(self, )
    }



    /// `[can_run_script] void selectTableRow ();`
    #[inline]
    pub unsafe fn SelectTableRow(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SelectTableRow)(self, )
    }



    /// `[can_run_script] void selectTableColumn ();`
    #[inline]
    pub unsafe fn SelectTableColumn(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SelectTableColumn)(self, )
    }



    /// `[can_run_script] void selectTable ();`
    #[inline]
    pub unsafe fn SelectTable(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SelectTable)(self, )
    }



    /// `[can_run_script] void selectAllTableCells ();`
    #[inline]
    pub unsafe fn SelectAllTableCells(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SelectAllTableCells)(self, )
    }


    /// ```text
    /// /** Create a new TD or TH element, the opposite type of the supplied aSourceCell
    ///     *   1. Copy all attributes from aSourceCell to the new cell
    ///     *   2. Move all contents of aSourceCell to the new cell
    ///     *   3. Replace aSourceCell in the table with the new cell
    ///     *
    ///     *  @param aSourceCell   The cell to be replaced
    ///     *  @return              The new cell that replaces aSourceCell
    ///     */
    /// ```
    ///

    /// `[can_run_script] Element switchTableCellHeaderType (in Element aSourceCell);`
    #[inline]
    pub unsafe fn SwitchTableCellHeaderType(&self, aSourceCell: *const libc::c_void, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).SwitchTableCellHeaderType)(self, aSourceCell, _retval)
    }


    /// ```text
    /// /** Merges contents of all selected cells
    ///     * for selected cells that are adjacent,
    ///     * this will result in a larger cell with appropriate
    ///     * rowspan and colspan, and original cells are deleted
    ///     * The resulting cell is in the location of the
    ///     *   cell at the upper-left corner of the adjacent
    ///     *   block of selected cells
    ///     *
    ///     * @param aMergeNonContiguousContents:
    ///     *       If true:
    ///     *         Non-contiguous cells are not deleted,
    ///     *         but their contents are still moved
    ///     *         to the upper-left cell
    ///     *       If false: contiguous cells are ignored
    ///     *
    ///     * If there are no selected cells,
    ///     *   and selection or caret is in a cell,
    ///     *   that cell and the one to the right
    ///     *   are merged
    ///     */
    /// ```
    ///

    /// `[can_run_script] void joinTableCells (in boolean aMergeNonContiguousContents);`
    #[inline]
    pub unsafe fn JoinTableCells(&self, aMergeNonContiguousContents: bool) -> ::nserror::nsresult {
        ((*self.vtable).JoinTableCells)(self, aMergeNonContiguousContents)
    }


    /// ```text
    /// /** Split a cell that has rowspan and/or colspan > 0
    ///     *   into cells such that all new cells have
    ///     *   rowspan = 1 and colspan = 1
    ///     *  All of the contents are not touched --
    ///     *   they will appear to be in the upper-left cell
    ///     */
    /// ```
    ///

    /// `[can_run_script] void splitTableCell ();`
    #[inline]
    pub unsafe fn SplitTableCell(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SplitTableCell)(self, )
    }


    /// ```text
    /// /** Scan through all rows and add cells as needed so
    ///     *   all locations in the cellmap are occupied.
    ///     *   Used after inserting single cells or pasting
    ///     *   a collection of cells that extend past the
    ///     *   previous size of the table
    ///     * If aTable is null, it uses table enclosing the selection anchor
    ///     * This doesn't doesn't change the selection,
    ///     *   thus it can be used to fixup all tables
    ///     *   in a page independent of the selection
    ///     */
    /// ```
    ///

    /// `[can_run_script] void normalizeTable (in Element aTable);`
    #[inline]
    pub unsafe fn NormalizeTable(&self, aTable: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).NormalizeTable)(self, aTable)
    }


    /// ```text
    /// /**
    ///    * getCellIndexes() computes row index and column index of a table cell.
    ///    * Note that this depends on layout information.  Therefore, all pending
    ///    * layout should've been flushed before calling this.
    ///    *
    ///    * @param aCellElement        If not null, this computes indexes of the cell.
    ///    *                            If null, this computes indexes of a cell which
    ///    *                            contains anchor of Selection.
    ///    * @param aRowIndex           Must be an object, whose .value will be set
    ///    *                            to row index of the cell.  0 is the first row.
    ///    *                            If rowspan is set to 2 or more, the start
    ///    *                            row index is used.
    ///    * @param aColumnIndex        Must be an object, whose .value will be set
    ///    *                            to column index of the cell.  0 is the first
    ///    *                            column.  If colspan is set to 2 or more, the
    ///    *                            start column index is used.
    ///    */
    /// ```
    ///

    /// `[can_run_script] void getCellIndexes (in Element aCellElement, out long aRowIndex, out long aColumnIndex);`
    #[inline]
    pub unsafe fn GetCellIndexes(&self, aCellElement: *const libc::c_void, aRowIndex: *mut i32, aColumnIndex: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetCellIndexes)(self, aCellElement, aRowIndex, aColumnIndex)
    }


    /// ```text
    /// /**
    ///    * getTableSize() computes number of rows and columns.
    ///    * Note that this depends on layout information.  Therefore, all pending
    ///    * layout should've been flushed before calling this.
    ///    *
    ///    * @param aTableOrElementInTable  If a <table> element, this computes number
    ///    *                                of rows and columns of it.
    ///    *                                If another element and in a <table>, this
    ///    *                                computes number of rows and columns of
    ///    *                                the nearest ancestor <table> element.
    ///    *                                If element is not in <table> element,
    ///    *                                throwing an exception.
    ///    *                                If null, this looks for nearest ancestor
    ///    *                                <table> element containing anchor of
    ///    *                                Selection.  If found, computes the number
    ///    *                                of rows and columns of the <table>.
    ///    *                                Otherwise, throwing an exception.
    ///    * @param aRowCount               Number of *actual* row count.
    ///    *                                I.e., rowspan does NOT increase this value.
    ///    * @param aColumnCount            Number of column count.
    ///    *                                I.e., if colspan is specified with bigger
    ///    *                                number than actual, the value is used
    ///    *                                as this.
    ///    */
    /// ```
    ///

    /// `void getTableSize (in Element aTableOrElementInTable, out long aRowCount, out long aColCount);`
    #[inline]
    pub unsafe fn GetTableSize(&self, aTableOrElementInTable: *const libc::c_void, aRowCount: *mut i32, aColCount: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetTableSize)(self, aTableOrElementInTable, aRowCount, aColCount)
    }


    /// ```text
    /// /**
    ///    * getCellAt() returns a <td> or <th> element in a <table> if there is a
    ///    * cell at the indexes.
    ///    *
    ///    * @param aTableElement       If not null, must be a <table> element.
    ///    *                            If null, looks for the nearest ancestor <table>
    ///    *                            to look for a cell.
    ///    * @param aRowIndex           Row index of the cell.
    ///    * @param aColumnIndex        Column index of the cell.
    ///    * @return                    Returns a <td> or <th> element if there is.
    ///    *                            Otherwise, returns null without throwing
    ///    *                            exception.
    ///    *                            If aTableElement is not null and not a <table>
    ///    *                            element, throwing an exception.
    ///    *                            If aTableElement is null and anchor of Selection
    ///    *                            is not in any <table> element, throwing an
    ///    *                            exception.
    ///    */
    /// ```
    ///

    /// `Element getCellAt (in Element aTableElement, in long aRowIndex, in long aColumnIndex);`
    #[inline]
    pub unsafe fn GetCellAt(&self, aTableElement: *const libc::c_void, aRowIndex: i32, aColumnIndex: i32, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetCellAt)(self, aTableElement, aRowIndex, aColumnIndex, _retval)
    }


    /// ```text
    /// /**
    ///    * Get cell element and its various information from <table> element and
    ///    * indexes in it.  If aTableElement is null, this looks for an ancestor
    ///    * <table> element of anchor of Selection.  If there is no <table> element
    ///    * at that point, this throws exception.  Note that this requires layout
    ///    * information.  So, you need to flush the layout after changing the DOM
    ///    * tree.
    ///    * If there is no cell element at the indexes, this throws exception.
    ///    * XXX Perhaps, this is wrong behavior, this should return null without
    ///    *     exception since the caller cannot distinguish whether the exception
    ///    *     is caused by "not found" or other unexpected situation.
    ///    *
    ///    * @param aTableElement       A <table> element.  If this is null, this
    ///    *                            uses ancestor of anchor of Selection.
    ///    * @param aRowIndex           Row index in aTableElement.  Starting from 0.
    ///    * @param aColumnIndex        Column index in aTableElement.  Starting from
    ///    *                            0.
    ///    * @param aCellElement        [OUT] The cell element at the indexes.
    ///    * @param aStartRowIndex      [OUT] First row index which contains
    ///    *                            aCellElement.  E.g., if the cell's rowspan is
    ///    *                            not 1, this returns its first row index.
    ///    *                            I.e., this can be smaller than aRowIndex.
    ///    * @param aStartColumnIndex   [OUT] First column index which contains the
    ///    *                            aCellElement.  E.g., if the cell's colspan is
    ///    *                            larger than 1, this returns its first column
    ///    *                            index.  I.e., this can be smaller than
    ///    *                            aColumIndex.
    ///    * @param aRowSpan            [OUT] rowspan attribute value in most cases.
    ///    *                            If the specified value is invalid, this
    ///    *                            returns 1.  Only when the document is written
    ///    *                            in HTML5 or later, this can be 0.
    ///    * @param aColSpan            [OUT] colspan attribute value in most cases.
    ///    *                            If the specified value is invalid, this
    ///    *                            returns 1.
    ///    * @param aEffectiveRowSpan   [OUT] Effective rowspan value at aRowIndex.
    ///    *                            This is same as:
    ///    *                              aRowSpan - (aRowIndex - aStartRowIndex)
    ///    * @param aEffectiveColSpan   [OUT] Effective colspan value at aColumnIndex.
    ///    *                            This is same as:
    ///    *                              aColSpan - (aColumnIndex - aStartColumnIndex)
    ///    * @param aIsSelected         [OUT] Returns true if aCellElement or its
    ///    *                            <tr> or <table> element is selected.
    ///    *                            Otherwise, e.g., aCellElement just contains
    ///    *                            selection range, returns false.
    ///    */
    /// ```
    ///

    /// `void getCellDataAt (in Element aTableElement, in long aRowIndex, in long aColumnIndex, out Element aCellElement, out long aStartRowIndex, out long aStartColumnIndex, out long aRowSpan, out long aColSpan, out long aEffectiveRowSpan, out long aEffectiveColSpan, out boolean aIsSelected);`
    #[inline]
    pub unsafe fn GetCellDataAt(&self, aTableElement: *const libc::c_void, aRowIndex: i32, aColumnIndex: i32, aCellElement: *mut *const libc::c_void, aStartRowIndex: *mut i32, aStartColumnIndex: *mut i32, aRowSpan: *mut i32, aColSpan: *mut i32, aEffectiveRowSpan: *mut i32, aEffectiveColSpan: *mut i32, aIsSelected: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCellDataAt)(self, aTableElement, aRowIndex, aColumnIndex, aCellElement, aStartRowIndex, aStartColumnIndex, aRowSpan, aColSpan, aEffectiveRowSpan, aEffectiveColSpan, aIsSelected)
    }


    /// ```text
    /// /**
    ///    * getFirstRow() returns first <tr> element in a <table> element.
    ///    *
    ///    * @param aTableOrElementInTable  If a <table> element, returns its first
    ///    *                                <tr> element.
    ///    *                                If another element, looks for nearest
    ///    *                                ancestor <table> element first.  Then,
    ///    *                                return its first <tr> element.
    ///    * @return                        <tr> element in the <table> element.
    ///    *                                If <table> element is not found, this
    ///    *                                throws an exception.
    ///    *                                If there is a <table> element but it
    ///    *                                does not have <tr> elements, returns
    ///    *                                null without throwing exception.
    ///    *                                Note that this may return anonymous <tr>
    ///    *                                element if <table> has one or more cells
    ///    *                                but <tr> element is not in the source.
    ///    */
    /// ```
    ///

    /// `Element getFirstRow (in Element aTableElement);`
    #[inline]
    pub unsafe fn GetFirstRow(&self, aTableElement: *const libc::c_void, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetFirstRow)(self, aTableElement, _retval)
    }


    /// ```text
    /// /** Preferred direction to search for neighboring cell
    ///     * when trying to locate a cell to place caret in after
    ///     * a table editing action.
    ///     * Used for aDirection param in SetSelectionAfterTableEdit
    ///     */
    /// /**
    ///    * getSelectedOrParentTableElement() returns a <td>, <th>, <tr> or <table>.
    ///    * If first selection range selects a <td> or <th>, returns it.  aTagName
    ///    * is set to "td" even if the result is a <th> and aCount is set to
    ///    * Selection.rangeCount.
    ///    * If first selection range does not select <td> nor <th>, but selection
    ///    * anchor refers <table>, returns it.  aTagName is set to "table" and
    ///    * aCount is set to 1.
    ///    * If first selection range does not select <td> nor <th>, but selection
    ///    * anchor refers <tr>, returns it.  aTagName is set to "tr" and aCount is
    ///    * set to 1.
    ///    * If first selection range does not select <td> nor <th>, but selection
    ///    * anchor refers <td> (not include <th>!), returns it.  aTagName is set to
    ///    * "td" and aCount is set to 0.
    ///    * Otherwise, if container of selection anchor is in a <td> or <th>,
    ///    * returns it.  aTagName is set to "td" but aCount is set to 0.
    ///    * Otherwise, returns null, aTagName is set to empty string and aCount is
    ///    * set to 0.  I.e., does not throw exception even if a cell is not found.
    ///    * NOTE: Calling this resets internal counter of getFirstSelectedCell()
    ///    *       and getNextSelectedCell().  I.e., getNextSelectedCell() will
    ///    *       return second selected cell element.
    ///    */
    /// ```
    ///

    /// `Element getSelectedOrParentTableElement (out AString aTagName, out long aCount);`
    #[inline]
    pub unsafe fn GetSelectedOrParentTableElement(&self, aTagName: *mut ::nsstring::nsAString, aCount: *mut i32, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetSelectedOrParentTableElement)(self, aTagName, aCount, _retval)
    }


    /// ```text
    /// /** Generally used after GetSelectedOrParentTableElement
    ///     *   to test if selected cells are complete rows or columns
    ///     *
    ///     * @param aElement           Any table or cell element or any element
    ///     *                           inside a table
    ///     *                           Used to get enclosing table.
    ///     *                           If null, selection's anchorNode is used
    ///     *
    ///     * @return
    ///     *     0                        aCellElement was not a cell
    ///     *                              (returned result = NS_ERROR_FAILURE)
    ///     *     TableSelectionMode::Cell     There are 1 or more cells selected but
    ///     *                              complete rows or columns are not selected
    ///     *     TableSelectionMode::Row      All cells are in 1 or more rows
    ///     *                              and in each row, all cells selected
    ///     *                              Note: This is the value if all rows
    ///     *                              (thus all cells) are selected
    ///     *     TableSelectionMode::Column   All cells are in 1 or more columns
    ///     *                              and in each column, all cells are selected
    ///     */
    /// ```
    ///

    /// `[can_run_script] uint32_t getSelectedCellsType (in Element aElement);`
    #[inline]
    pub unsafe fn GetSelectedCellsType(&self, aElement: *const libc::c_void, _retval: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetSelectedCellsType)(self, aElement, _retval)
    }


    /// ```text
    /// /**
    ///    * getFirstSelectedCellInTable() returns a cell element, its row index and
    ///    * its column index if first range of Selection selects a cell.  Note that
    ///    * that "selects a cell" means that the range container is a <tr> element
    ///    * and endOffset is startOffset + 1.  So, even if first range of Selection
    ///    * is in a cell element, this treats the range does not select a cell.
    ///    * NOTE: Calling this resets internal counter of getFirstSelectedCell()
    ///    *       and getNextSelectedCell().  I.e., getNextSelectedCell() will
    ///    *       return second selected cell element.
    ///    *
    ///    * @param aRowIndex    [OUT} Returns row index of the found cell.  If not
    ///    *                     found, returns 0.
    ///    * @param aColumnIndex [OUT] Returns column index of the found cell.  If
    ///    *                     not found, returns 0.
    ///    * @return             The cell element which is selected by the first
    ///    *                     range of Selection.  Even if this is not found,
    ///    *                     this returns null, not throwing exception.
    ///    */
    /// ```
    ///

    /// `[can_run_script] Element getFirstSelectedCellInTable (out long aRowIndex, out long aColIndex);`
    #[inline]
    pub unsafe fn GetFirstSelectedCellInTable(&self, aRowIndex: *mut i32, aColIndex: *mut i32, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetFirstSelectedCellInTable)(self, aRowIndex, aColIndex, _retval)
    }


    /// ```text
    /// /**
    ///    * getSelectedCells() returns an array of `<td>` and `<th>` elements which
    ///    * are selected in **any** `<table>` elements (i.e., some cells may be
        ///    * in different `<table>` element).
    ///    * If first range does not select a table cell element, this returns empty
    ///    * array because editor considers that selection is not in table cell
    ///    * selection mode.
    ///    * If second or later ranges do not select only a table cell element, this
    ///    * ignores the ranges.
    ///    */
    /// ```
    ///

    /// `Array<Element> getSelectedCells ();`
    #[inline]
    pub unsafe fn GetSelectedCells(&self, _retval: *mut thin_vec::ThinVec<*const libc::c_void>) -> ::nserror::nsresult {
        ((*self.vtable).GetSelectedCells)(self, _retval)
    }


}


