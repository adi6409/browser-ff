//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleTable.idl
//


/// `interface nsIAccessibleTable : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAccessibleTable {
    vtable: *const nsIAccessibleTableVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessibleTable.
unsafe impl XpCom for nsIAccessibleTable {
    const IID: nsIID = nsID(0xcb0bf7b9, 0x117e, 0x40e2,
        [0x9e, 0x46, 0x18, 0x9c, 0x3d, 0x43, 0xce, 0x4a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessibleTable {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessibleTable.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessibleTableCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessibleTable`.
    fn coerce_from(v: &nsIAccessibleTable) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessibleTableCoerce for nsIAccessibleTable {
    #[inline]
    fn coerce_from(v: &nsIAccessibleTable) -> &Self {
        v
    }
}

impl nsIAccessibleTable {
    /// Cast this `nsIAccessibleTable` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessibleTableCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessibleTable {
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
impl<T: nsISupportsCoerce> nsIAccessibleTableCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleTable) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessibleTable
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessibleTableVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIAccessible caption; */
    pub GetCaption: unsafe extern "system" fn (this: *const nsIAccessibleTable, aCaption: *mut*const nsIAccessible) -> ::nserror::nsresult,

    /* readonly attribute AString summary; */
    pub GetSummary: unsafe extern "system" fn (this: *const nsIAccessibleTable, aSummary: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute long columnCount; */
    pub GetColumnCount: unsafe extern "system" fn (this: *const nsIAccessibleTable, aColumnCount: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute long rowCount; */
    pub GetRowCount: unsafe extern "system" fn (this: *const nsIAccessibleTable, aRowCount: *mut i32) -> ::nserror::nsresult,

    /* nsIAccessible getCellAt (in long rowIndex, in long columnIndex); */
    pub GetCellAt: unsafe extern "system" fn (this: *const nsIAccessibleTable, rowIndex: i32, columnIndex: i32, _retval: *mut*const nsIAccessible) -> ::nserror::nsresult,

    /* long getCellIndexAt (in long rowIndex, in long columnIndex); */
    pub GetCellIndexAt: unsafe extern "system" fn (this: *const nsIAccessibleTable, rowIndex: i32, columnIndex: i32, _retval: *mut i32) -> ::nserror::nsresult,

    /* long getColumnIndexAt (in long cellIndex); */
    pub GetColumnIndexAt: unsafe extern "system" fn (this: *const nsIAccessibleTable, cellIndex: i32, _retval: *mut i32) -> ::nserror::nsresult,

    /* long getRowIndexAt (in long cellIndex); */
    pub GetRowIndexAt: unsafe extern "system" fn (this: *const nsIAccessibleTable, cellIndex: i32, _retval: *mut i32) -> ::nserror::nsresult,

    /* void getRowAndColumnIndicesAt (in long cellIndex, out long rowIndex, out long columnIndex); */
    pub GetRowAndColumnIndicesAt: unsafe extern "system" fn (this: *const nsIAccessibleTable, cellIndex: i32, rowIndex: *mut i32, columnIndex: *mut i32) -> ::nserror::nsresult,

    /* long getColumnExtentAt (in long row, in long column); */
    pub GetColumnExtentAt: unsafe extern "system" fn (this: *const nsIAccessibleTable, row: i32, column: i32, _retval: *mut i32) -> ::nserror::nsresult,

    /* long getRowExtentAt (in long row, in long column); */
    pub GetRowExtentAt: unsafe extern "system" fn (this: *const nsIAccessibleTable, row: i32, column: i32, _retval: *mut i32) -> ::nserror::nsresult,

    /* AString getColumnDescription (in long columnIndex); */
    pub GetColumnDescription: unsafe extern "system" fn (this: *const nsIAccessibleTable, columnIndex: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString getRowDescription (in long rowIndex); */
    pub GetRowDescription: unsafe extern "system" fn (this: *const nsIAccessibleTable, rowIndex: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* boolean isColumnSelected (in long columnIndex); */
    pub IsColumnSelected: unsafe extern "system" fn (this: *const nsIAccessibleTable, columnIndex: i32, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean isRowSelected (in long rowIndex); */
    pub IsRowSelected: unsafe extern "system" fn (this: *const nsIAccessibleTable, rowIndex: i32, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean isCellSelected (in long rowIndex, in long columnIndex); */
    pub IsCellSelected: unsafe extern "system" fn (this: *const nsIAccessibleTable, rowIndex: i32, columnIndex: i32, _retval: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute unsigned long selectedCellCount; */
    pub GetSelectedCellCount: unsafe extern "system" fn (this: *const nsIAccessibleTable, aSelectedCellCount: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute unsigned long selectedColumnCount; */
    pub GetSelectedColumnCount: unsafe extern "system" fn (this: *const nsIAccessibleTable, aSelectedColumnCount: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute unsigned long selectedRowCount; */
    pub GetSelectedRowCount: unsafe extern "system" fn (this: *const nsIAccessibleTable, aSelectedRowCount: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute nsIArray selectedCells; */
    pub GetSelectedCells: unsafe extern "system" fn (this: *const nsIAccessibleTable, aSelectedCells: *mut*const nsIArray) -> ::nserror::nsresult,

    /* Array<uint32_t> getSelectedCellIndices (); */
    pub GetSelectedCellIndices: unsafe extern "system" fn (this: *const nsIAccessibleTable, _retval: *mut thin_vec::ThinVec<uint32_t>) -> ::nserror::nsresult,

    /* Array<uint32_t> getSelectedColumnIndices (); */
    pub GetSelectedColumnIndices: unsafe extern "system" fn (this: *const nsIAccessibleTable, _retval: *mut thin_vec::ThinVec<uint32_t>) -> ::nserror::nsresult,

    /* Array<uint32_t> getSelectedRowIndices (); */
    pub GetSelectedRowIndices: unsafe extern "system" fn (this: *const nsIAccessibleTable, _retval: *mut thin_vec::ThinVec<uint32_t>) -> ::nserror::nsresult,

    /* void selectRow (in long rowIndex); */
    pub SelectRow: unsafe extern "system" fn (this: *const nsIAccessibleTable, rowIndex: i32) -> ::nserror::nsresult,

    /* void selectColumn (in long columnIndex); */
    pub SelectColumn: unsafe extern "system" fn (this: *const nsIAccessibleTable, columnIndex: i32) -> ::nserror::nsresult,

    /* void unselectRow (in long rowIndex); */
    pub UnselectRow: unsafe extern "system" fn (this: *const nsIAccessibleTable, rowIndex: i32) -> ::nserror::nsresult,

    /* void unselectColumn (in long columnIndex); */
    pub UnselectColumn: unsafe extern "system" fn (this: *const nsIAccessibleTable, columnIndex: i32) -> ::nserror::nsresult,

    /* boolean isProbablyForLayout (); */
    pub IsProbablyForLayout: unsafe extern "system" fn (this: *const nsIAccessibleTable, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessibleTable {

    /// ```text
    /// /**
    ///    * Return the caption accessible for the table. For example, html:caption
    ///    * element of html:table element.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIAccessible caption;`
    #[inline]
    pub unsafe fn GetCaption(&self, aCaption: *mut*const nsIAccessible) -> ::nserror::nsresult {
        ((*self.vtable).GetCaption)(self, aCaption)
    }


    /// ```text
    /// /**
    ///    * Return summary description for the table. For example, @summary attribute
    ///    * on html:table element.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString summary;`
    #[inline]
    pub unsafe fn GetSummary(&self, aSummary: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSummary)(self, aSummary)
    }


    /// ```text
    /// /**
    ///    * Return columns count in the table.
    ///    */
    /// ```
    ///

    /// `readonly attribute long columnCount;`
    #[inline]
    pub unsafe fn GetColumnCount(&self, aColumnCount: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetColumnCount)(self, aColumnCount)
    }


    /// ```text
    /// /**
    ///    * Return rows count in the table.
    ///    */
    /// ```
    ///

    /// `readonly attribute long rowCount;`
    #[inline]
    pub unsafe fn GetRowCount(&self, aRowCount: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetRowCount)(self, aRowCount)
    }


    /// ```text
    /// /**
    ///    * Return the accessible object at the specified row and column in the table.
    ///    * If both row and column index are valid then the corresponding accessible
    ///    * object is returned that represents the requested cell regardless of whether
    ///    * the cell is currently visible (on the screen).
    ///    *
    ///    * @param  rowIndex     [in] the row index to retrieve the cell at
    ///    * @param  columnIndex  [in] the column index to retrieve the cell at
    ///    */
    /// ```
    ///

    /// `nsIAccessible getCellAt (in long rowIndex, in long columnIndex);`
    #[inline]
    pub unsafe fn GetCellAt(&self, rowIndex: i32, columnIndex: i32, _retval: *mut*const nsIAccessible) -> ::nserror::nsresult {
        ((*self.vtable).GetCellAt)(self, rowIndex, columnIndex, _retval)
    }


    /// ```text
    /// /**
    ///    * Translate the given row and column indices into the corresponding cell
    ///    * index.
    ///    *
    ///    * @param  rowIndex    [in] the row index to return cell index at
    ///    * @param  columnIndex [in] the column index to return cell index at
    ///    */
    /// ```
    ///

    /// `long getCellIndexAt (in long rowIndex, in long columnIndex);`
    #[inline]
    pub unsafe fn GetCellIndexAt(&self, rowIndex: i32, columnIndex: i32, _retval: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetCellIndexAt)(self, rowIndex, columnIndex, _retval)
    }


    /// ```text
    /// /**
    ///    * Translate the given cell index into the corresponding column index.
    ///    *
    ///    * @param  cellIndex  [in] index of the table cell to return column index for
    ///    */
    /// ```
    ///

    /// `long getColumnIndexAt (in long cellIndex);`
    #[inline]
    pub unsafe fn GetColumnIndexAt(&self, cellIndex: i32, _retval: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetColumnIndexAt)(self, cellIndex, _retval)
    }


    /// ```text
    /// /**
    ///    * Translate the given cell index into the corresponding row index.
    ///    *
    ///    * @param cellIndex  [in] index of the table cell to return row index for
    ///    */
    /// ```
    ///

    /// `long getRowIndexAt (in long cellIndex);`
    #[inline]
    pub unsafe fn GetRowIndexAt(&self, cellIndex: i32, _retval: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetRowIndexAt)(self, cellIndex, _retval)
    }


    /// ```text
    /// /**
    ///    * Translate the given cell index into the corresponding row and column
    ///    * indices.
    ///    *
    ///    * @param cellIndex    [in] cell index to return row and column indices for
    ///    * @param rowIndex     [out] row index at the given cell index
    ///    * @param columnIndex  [out] column index at the given cell index
    ///    */
    /// ```
    ///

    /// `void getRowAndColumnIndicesAt (in long cellIndex, out long rowIndex, out long columnIndex);`
    #[inline]
    pub unsafe fn GetRowAndColumnIndicesAt(&self, cellIndex: i32, rowIndex: *mut i32, columnIndex: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetRowAndColumnIndicesAt)(self, cellIndex, rowIndex, columnIndex)
    }


    /// ```text
    /// /**
    ///    * Return the number of columns occupied by the accessible cell at
    ///    * the specified row and column in the table. The result differs from 1 if
    ///    * the specified cell spans multiple columns.
    ///    *
    ///    * @param  row     [in] row index of the cell to return the column extent for
    ///    * @param  column  [in] column index of the cell to return the column extent
    ///    *                  for
    ///    */
    /// ```
    ///

    /// `long getColumnExtentAt (in long row, in long column);`
    #[inline]
    pub unsafe fn GetColumnExtentAt(&self, row: i32, column: i32, _retval: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetColumnExtentAt)(self, row, column, _retval)
    }


    /// ```text
    /// /**
    ///    * Return the number of rows occupied by the accessible cell at the specified
    ///    * row and column in the table. The result differs from 1 if the specified
    ///    * cell spans multiple rows.
    ///    *
    ///    * @param  row     [in] row index of the cell to return the column extent for
    ///    * @param  column  [in] column index of the cell to return the column extent
    ///    *                  for
    ///    */
    /// ```
    ///

    /// `long getRowExtentAt (in long row, in long column);`
    #[inline]
    pub unsafe fn GetRowExtentAt(&self, row: i32, column: i32, _retval: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetRowExtentAt)(self, row, column, _retval)
    }


    /// ```text
    /// /**
    ///    * Return the description text of the specified column in the table.
    ///    *
    ///    * @param  columnIndex  [in] the column index to retrieve description for
    ///    */
    /// ```
    ///

    /// `AString getColumnDescription (in long columnIndex);`
    #[inline]
    pub unsafe fn GetColumnDescription(&self, columnIndex: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetColumnDescription)(self, columnIndex, _retval)
    }


    /// ```text
    /// /**
    ///    * Return the description text of the specified row in the table.
    ///    *
    ///    * @param  rowIndex  [in] the row index to retrieve description for
    ///    */
    /// ```
    ///

    /// `AString getRowDescription (in long rowIndex);`
    #[inline]
    pub unsafe fn GetRowDescription(&self, rowIndex: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetRowDescription)(self, rowIndex, _retval)
    }


    /// ```text
    /// /**
    ///    * Return a boolean value indicating whether the specified column is
    ///    * selected, i.e. all cells within the column are selected.
    ///    *
    ///    * @param  columnIndex  [in] the column index to determine if it's selected
    ///    */
    /// ```
    ///

    /// `boolean isColumnSelected (in long columnIndex);`
    #[inline]
    pub unsafe fn IsColumnSelected(&self, columnIndex: i32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsColumnSelected)(self, columnIndex, _retval)
    }


    /// ```text
    /// /**
    ///    * Return a boolean value indicating whether the specified row is selected,
    ///    * i.e. all cells within the row are selected.
    ///    *
    ///    * @param  rowIndex  [in] the row index to determine whether it's selected
    ///    */
    /// ```
    ///

    /// `boolean isRowSelected (in long rowIndex);`
    #[inline]
    pub unsafe fn IsRowSelected(&self, rowIndex: i32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsRowSelected)(self, rowIndex, _retval)
    }


    /// ```text
    /// /**
    ///    * Return a boolean value indicating whether the specified cell is selected.
    ///    *
    ///    * @param  rowIndex     [in] the row index of the cell
    ///    * @param  columnIndex  [in] the column index of the cell
    ///    */
    /// ```
    ///

    /// `boolean isCellSelected (in long rowIndex, in long columnIndex);`
    #[inline]
    pub unsafe fn IsCellSelected(&self, rowIndex: i32, columnIndex: i32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsCellSelected)(self, rowIndex, columnIndex, _retval)
    }


    /// ```text
    /// /**
    ///    * Return the total number of selected cells.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long selectedCellCount;`
    #[inline]
    pub unsafe fn GetSelectedCellCount(&self, aSelectedCellCount: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetSelectedCellCount)(self, aSelectedCellCount)
    }


    /// ```text
    /// /**
    ///    * Return the total number of selected columns.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long selectedColumnCount;`
    #[inline]
    pub unsafe fn GetSelectedColumnCount(&self, aSelectedColumnCount: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetSelectedColumnCount)(self, aSelectedColumnCount)
    }


    /// ```text
    /// /**
    ///    * Return the total number of selected rows.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long selectedRowCount;`
    #[inline]
    pub unsafe fn GetSelectedRowCount(&self, aSelectedRowCount: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetSelectedRowCount)(self, aSelectedRowCount)
    }


    /// ```text
    /// /**
    ///    * Return an array of selected cells.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIArray selectedCells;`
    #[inline]
    pub unsafe fn GetSelectedCells(&self, aSelectedCells: *mut*const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetSelectedCells)(self, aSelectedCells)
    }


    /// ```text
    /// /**
    ///    * Return an array of cell indices currently selected.
    ///    *
    ///    * @return array of indexes of selected cells
    ///    */
    /// ```
    ///

    /// `Array<uint32_t> getSelectedCellIndices ();`
    #[inline]
    pub unsafe fn GetSelectedCellIndices(&self, _retval: *mut thin_vec::ThinVec<uint32_t>) -> ::nserror::nsresult {
        ((*self.vtable).GetSelectedCellIndices)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Return an array of column indices currently selected.
    ///    *
    ///    * @return array of indices of selected columns
    ///    */
    /// ```
    ///

    /// `Array<uint32_t> getSelectedColumnIndices ();`
    #[inline]
    pub unsafe fn GetSelectedColumnIndices(&self, _retval: *mut thin_vec::ThinVec<uint32_t>) -> ::nserror::nsresult {
        ((*self.vtable).GetSelectedColumnIndices)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Return an array of row indices currently selected.
    ///    *
    ///    * @return array of indices of selected rows
    ///    */
    /// ```
    ///

    /// `Array<uint32_t> getSelectedRowIndices ();`
    #[inline]
    pub unsafe fn GetSelectedRowIndices(&self, _retval: *mut thin_vec::ThinVec<uint32_t>) -> ::nserror::nsresult {
        ((*self.vtable).GetSelectedRowIndices)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Select a row and unselects all previously selected rows.
    ///    *
    ///    * @param  rowIndex  [in] the row index to select
    ///    */
    /// ```
    ///

    /// `void selectRow (in long rowIndex);`
    #[inline]
    pub unsafe fn SelectRow(&self, rowIndex: i32) -> ::nserror::nsresult {
        ((*self.vtable).SelectRow)(self, rowIndex)
    }


    /// ```text
    /// /**
    ///    * Select a column and unselects all previously selected columns.
    ///    *
    ///    * @param  columnIndex  [in] the column index to select
    ///    */
    /// ```
    ///

    /// `void selectColumn (in long columnIndex);`
    #[inline]
    pub unsafe fn SelectColumn(&self, columnIndex: i32) -> ::nserror::nsresult {
        ((*self.vtable).SelectColumn)(self, columnIndex)
    }


    /// ```text
    /// /**
    ///    * Unselect the given row, leaving other selected rows selected (if any).
    ///    *
    ///    * @param  rowIndex  [in] the row index to select
    ///   */
    /// ```
    ///

    /// `void unselectRow (in long rowIndex);`
    #[inline]
    pub unsafe fn UnselectRow(&self, rowIndex: i32) -> ::nserror::nsresult {
        ((*self.vtable).UnselectRow)(self, rowIndex)
    }


    /// ```text
    /// /**
    ///    * Unselect the given column, leaving other selected columns selected (if any).
    ///    *
    ///    * @param  columnIndex  [in] the column index to select
    ///    */
    /// ```
    ///

    /// `void unselectColumn (in long columnIndex);`
    #[inline]
    pub unsafe fn UnselectColumn(&self, columnIndex: i32) -> ::nserror::nsresult {
        ((*self.vtable).UnselectColumn)(self, columnIndex)
    }


    /// ```text
    /// /**
    ///    * Use heuristics to determine if table is most likely used for layout.
    ///    */
    /// ```
    ///

    /// `boolean isProbablyForLayout ();`
    #[inline]
    pub unsafe fn IsProbablyForLayout(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsProbablyForLayout)(self, _retval)
    }


}


/// `interface nsIAccessibleTableCell : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAccessibleTableCell {
    vtable: *const nsIAccessibleTableCellVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessibleTableCell.
unsafe impl XpCom for nsIAccessibleTableCell {
    const IID: nsIID = nsID(0x654e296d, 0xfae6, 0x452b,
        [0x98, 0x7d, 0x74, 0x6b, 0x20, 0xb9, 0x51, 0x4b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessibleTableCell {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessibleTableCell.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessibleTableCellCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessibleTableCell`.
    fn coerce_from(v: &nsIAccessibleTableCell) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessibleTableCellCoerce for nsIAccessibleTableCell {
    #[inline]
    fn coerce_from(v: &nsIAccessibleTableCell) -> &Self {
        v
    }
}

impl nsIAccessibleTableCell {
    /// Cast this `nsIAccessibleTableCell` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessibleTableCellCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessibleTableCell {
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
impl<T: nsISupportsCoerce> nsIAccessibleTableCellCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleTableCell) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessibleTableCell
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessibleTableCellVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIAccessibleTable table; */
    pub GetTable: unsafe extern "system" fn (this: *const nsIAccessibleTableCell, aTable: *mut *const nsIAccessibleTable) -> ::nserror::nsresult,

    /* readonly attribute long columnIndex; */
    pub GetColumnIndex: unsafe extern "system" fn (this: *const nsIAccessibleTableCell, aColumnIndex: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute long rowIndex; */
    pub GetRowIndex: unsafe extern "system" fn (this: *const nsIAccessibleTableCell, aRowIndex: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute long columnExtent; */
    pub GetColumnExtent: unsafe extern "system" fn (this: *const nsIAccessibleTableCell, aColumnExtent: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute long rowExtent; */
    pub GetRowExtent: unsafe extern "system" fn (this: *const nsIAccessibleTableCell, aRowExtent: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute nsIArray columnHeaderCells; */
    pub GetColumnHeaderCells: unsafe extern "system" fn (this: *const nsIAccessibleTableCell, aColumnHeaderCells: *mut*const nsIArray) -> ::nserror::nsresult,

    /* readonly attribute nsIArray rowHeaderCells; */
    pub GetRowHeaderCells: unsafe extern "system" fn (this: *const nsIAccessibleTableCell, aRowHeaderCells: *mut*const nsIArray) -> ::nserror::nsresult,

    /* boolean isSelected (); */
    pub IsSelected: unsafe extern "system" fn (this: *const nsIAccessibleTableCell, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessibleTableCell {

    /// ```text
    /// /**
    ///    * Return host table accessible.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIAccessibleTable table;`
    #[inline]
    pub unsafe fn GetTable(&self, aTable: *mut *const nsIAccessibleTable) -> ::nserror::nsresult {
        ((*self.vtable).GetTable)(self, aTable)
    }


    /// ```text
    /// /**
    ///    * Return column index of this cell.
    ///    */
    /// ```
    ///

    /// `readonly attribute long columnIndex;`
    #[inline]
    pub unsafe fn GetColumnIndex(&self, aColumnIndex: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetColumnIndex)(self, aColumnIndex)
    }


    /// ```text
    /// /**
    ///    * Return row index of this cell.
    ///    */
    /// ```
    ///

    /// `readonly attribute long rowIndex;`
    #[inline]
    pub unsafe fn GetRowIndex(&self, aRowIndex: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetRowIndex)(self, aRowIndex)
    }


    /// ```text
    /// /**
    ///    * Return the number of columns occupied by this cell. The result differs
    ///    * from 1 if the specified cell spans multiple columns.
    ///    */
    /// ```
    ///

    /// `readonly attribute long columnExtent;`
    #[inline]
    pub unsafe fn GetColumnExtent(&self, aColumnExtent: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetColumnExtent)(self, aColumnExtent)
    }


    /// ```text
    /// /**
    ///    * Return the number of rows occupied by this accessible cell. The result
    ///    * differs from 1 if the specified cell spans multiple rows.
    ///    */
    /// ```
    ///

    /// `readonly attribute long rowExtent;`
    #[inline]
    pub unsafe fn GetRowExtent(&self, aRowExtent: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetRowExtent)(self, aRowExtent)
    }


    /// ```text
    /// /**
    ///    * Return an array of column header cells for this cell.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIArray columnHeaderCells;`
    #[inline]
    pub unsafe fn GetColumnHeaderCells(&self, aColumnHeaderCells: *mut*const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetColumnHeaderCells)(self, aColumnHeaderCells)
    }


    /// ```text
    /// /**
    ///    * Return an array of row header cells for this cell.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIArray rowHeaderCells;`
    #[inline]
    pub unsafe fn GetRowHeaderCells(&self, aRowHeaderCells: *mut*const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetRowHeaderCells)(self, aRowHeaderCells)
    }


    /// ```text
    /// /**
    ///    * Return a boolean value indicating whether this cell is selected.
    ///    */
    /// ```
    ///

    /// `boolean isSelected ();`
    #[inline]
    pub unsafe fn IsSelected(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsSelected)(self, _retval)
    }


}


