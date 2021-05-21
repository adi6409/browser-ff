//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/layout/xul/tree/nsITreeView.idl
//


/// `interface nsITreeView : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITreeView {
    vtable: *const nsITreeViewVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITreeView.
unsafe impl XpCom for nsITreeView {
    const IID: nsIID = nsID(0x091116f0, 0x0bdc, 0x4b32,
        [0xb9, 0xc8, 0xc8, 0xd5, 0xa3, 0x7c, 0xb0, 0x88]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITreeView {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITreeView.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITreeViewCoerce {
    /// Cheaply cast a value of this type from a `nsITreeView`.
    fn coerce_from(v: &nsITreeView) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITreeViewCoerce for nsITreeView {
    #[inline]
    fn coerce_from(v: &nsITreeView) -> &Self {
        v
    }
}

impl nsITreeView {
    /// Cast this `nsITreeView` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITreeViewCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITreeView {
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
impl<T: nsISupportsCoerce> nsITreeViewCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITreeView) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITreeView
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITreeViewVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute long rowCount; */
    pub GetRowCount: unsafe extern "system" fn (this: *const nsITreeView, aRowCount: *mut i32) -> ::nserror::nsresult,

    /* attribute nsITreeSelection selection; */
    pub GetSelection: unsafe extern "system" fn (this: *const nsITreeView, aSelection: *mut*const nsITreeSelection) -> ::nserror::nsresult,

    /* attribute nsITreeSelection selection; */
    pub SetSelection: unsafe extern "system" fn (this: *const nsITreeView, aSelection: *const nsITreeSelection) -> ::nserror::nsresult,

    /* AString getRowProperties (in long index); */
    pub GetRowProperties: unsafe extern "system" fn (this: *const nsITreeView, index: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString getCellProperties (in long row, in TreeColumn col); */
    pub GetCellProperties: unsafe extern "system" fn (this: *const nsITreeView, row: i32, col: *const libc::c_void, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString getColumnProperties (in TreeColumn col); */
    pub GetColumnProperties: unsafe extern "system" fn (this: *const nsITreeView, col: *const libc::c_void, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* boolean isContainer (in long index); */
    pub IsContainer: unsafe extern "system" fn (this: *const nsITreeView, index: i32, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean isContainerOpen (in long index); */
    pub IsContainerOpen: unsafe extern "system" fn (this: *const nsITreeView, index: i32, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean isContainerEmpty (in long index); */
    pub IsContainerEmpty: unsafe extern "system" fn (this: *const nsITreeView, index: i32, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean isSeparator (in long index); */
    pub IsSeparator: unsafe extern "system" fn (this: *const nsITreeView, index: i32, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean isSorted (); */
    pub IsSorted: unsafe extern "system" fn (this: *const nsITreeView, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean canDrop (in long index, in long orientation, in DataTransfer dataTransfer); */
    pub CanDrop: unsafe extern "system" fn (this: *const nsITreeView, index: i32, orientation: i32, dataTransfer: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult,

    /* void drop (in long row, in long orientation, in DataTransfer dataTransfer); */
    pub Drop: unsafe extern "system" fn (this: *const nsITreeView, row: i32, orientation: i32, dataTransfer: *const libc::c_void) -> ::nserror::nsresult,

    /* long getParentIndex (in long rowIndex); */
    pub GetParentIndex: unsafe extern "system" fn (this: *const nsITreeView, rowIndex: i32, _retval: *mut i32) -> ::nserror::nsresult,

    /* boolean hasNextSibling (in long rowIndex, in long afterIndex); */
    pub HasNextSibling: unsafe extern "system" fn (this: *const nsITreeView, rowIndex: i32, afterIndex: i32, _retval: *mut bool) -> ::nserror::nsresult,

    /* long getLevel (in long index); */
    pub GetLevel: unsafe extern "system" fn (this: *const nsITreeView, index: i32, _retval: *mut i32) -> ::nserror::nsresult,

    /* AString getImageSrc (in long row, in TreeColumn col); */
    pub GetImageSrc: unsafe extern "system" fn (this: *const nsITreeView, row: i32, col: *const libc::c_void, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString getCellValue (in long row, in TreeColumn col); */
    pub GetCellValue: unsafe extern "system" fn (this: *const nsITreeView, row: i32, col: *const libc::c_void, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString getCellText (in long row, in TreeColumn col); */
    pub GetCellText: unsafe extern "system" fn (this: *const nsITreeView, row: i32, col: *const libc::c_void, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void setTree (in XULTreeElement tree); */
    pub SetTree: unsafe extern "system" fn (this: *const nsITreeView, tree: *const libc::c_void) -> ::nserror::nsresult,

    /* void toggleOpenState (in long index); */
    pub ToggleOpenState: unsafe extern "system" fn (this: *const nsITreeView, index: i32) -> ::nserror::nsresult,

    /* void cycleHeader (in TreeColumn col); */
    pub CycleHeader: unsafe extern "system" fn (this: *const nsITreeView, col: *const libc::c_void) -> ::nserror::nsresult,

    /* [binaryname(SelectionChangedXPCOM)] void selectionChanged (); */
    pub SelectionChangedXPCOM: unsafe extern "system" fn (this: *const nsITreeView) -> ::nserror::nsresult,

    /* void cycleCell (in long row, in TreeColumn col); */
    pub CycleCell: unsafe extern "system" fn (this: *const nsITreeView, row: i32, col: *const libc::c_void) -> ::nserror::nsresult,

    /* boolean isEditable (in long row, in TreeColumn col); */
    pub IsEditable: unsafe extern "system" fn (this: *const nsITreeView, row: i32, col: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult,

    /* void setCellValue (in long row, in TreeColumn col, in AString value); */
    pub SetCellValue: unsafe extern "system" fn (this: *const nsITreeView, row: i32, col: *const libc::c_void, value: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void setCellText (in long row, in TreeColumn col, in AString value); */
    pub SetCellText: unsafe extern "system" fn (this: *const nsITreeView, row: i32, col: *const libc::c_void, value: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITreeView {

    pub const DROP_BEFORE: i64 = -1;


    pub const DROP_ON: i64 = 0;


    pub const DROP_AFTER: i64 = 1;

    /// ```text
    /// /**
    ///    * The total number of rows in the tree (including the offscreen rows).
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
    ///    * The selection for this view.
    ///    */
    /// ```
    ///

    /// `attribute nsITreeSelection selection;`
    #[inline]
    pub unsafe fn GetSelection(&self, aSelection: *mut*const nsITreeSelection) -> ::nserror::nsresult {
        ((*self.vtable).GetSelection)(self, aSelection)
    }


    /// ```text
    /// /**
    ///    * The selection for this view.
    ///    */
    /// ```
    ///

    /// `attribute nsITreeSelection selection;`
    #[inline]
    pub unsafe fn SetSelection(&self, aSelection: *const nsITreeSelection) -> ::nserror::nsresult {
        ((*self.vtable).SetSelection)(self, aSelection)
    }


    /// ```text
    /// /**
    ///    * A whitespace delimited list of properties.  For each property X the view
    ///    * gives back will cause the pseudoclasses  ::-moz-tree-cell(x),
    ///    * ::-moz-tree-row(x), ::-moz-tree-twisty(x), ::-moz-tree-image(x),
    ///    * ::-moz-tree-cell-text(x).  to be matched on the pseudoelement
    ///    * ::moz-tree-row.
    ///    */
    /// ```
    ///

    /// `AString getRowProperties (in long index);`
    #[inline]
    pub unsafe fn GetRowProperties(&self, index: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetRowProperties)(self, index, _retval)
    }


    /// ```text
    /// /**
    ///    * A whitespace delimited list of properties for a given cell.  Each
    ///    * property, x, that the view gives back will cause the pseudoclasses
    ///    *  ::-moz-tree-cell(x), ::-moz-tree-row(x), ::-moz-tree-twisty(x),
    ///    *  ::-moz-tree-image(x), ::-moz-tree-cell-text(x). to be matched on the
    ///    *  cell.
    ///    */
    /// ```
    ///

    /// `AString getCellProperties (in long row, in TreeColumn col);`
    #[inline]
    pub unsafe fn GetCellProperties(&self, row: i32, col: *const libc::c_void, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetCellProperties)(self, row, col, _retval)
    }


    /// ```text
    /// /**
    ///    * Called to get properties to paint a column background.  For shading the sort
    ///    * column, etc.
    ///    */
    /// ```
    ///

    /// `AString getColumnProperties (in TreeColumn col);`
    #[inline]
    pub unsafe fn GetColumnProperties(&self, col: *const libc::c_void, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetColumnProperties)(self, col, _retval)
    }


    /// ```text
    /// /**
    ///    * Methods that can be used to test whether or not a twisty should be drawn,
    ///    * and if so, whether an open or closed twisty should be used.
    ///    */
    /// ```
    ///

    /// `boolean isContainer (in long index);`
    #[inline]
    pub unsafe fn IsContainer(&self, index: i32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsContainer)(self, index, _retval)
    }



    /// `boolean isContainerOpen (in long index);`
    #[inline]
    pub unsafe fn IsContainerOpen(&self, index: i32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsContainerOpen)(self, index, _retval)
    }



    /// `boolean isContainerEmpty (in long index);`
    #[inline]
    pub unsafe fn IsContainerEmpty(&self, index: i32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsContainerEmpty)(self, index, _retval)
    }


    /// ```text
    /// /**
    ///    * isSeparator is used to determine if the row at index is a separator.
    ///    * A value of true will result in the tree drawing a horizontal separator.
    ///    * The tree uses the ::moz-tree-separator pseudoclass to draw the separator.
    ///    */
    /// ```
    ///

    /// `boolean isSeparator (in long index);`
    #[inline]
    pub unsafe fn IsSeparator(&self, index: i32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsSeparator)(self, index, _retval)
    }


    /// ```text
    /// /**
    ///    * Specifies if there is currently a sort on any column. Used mostly by dragdrop
    ///    * to affect drop feedback.
    ///    */
    /// ```
    ///

    /// `boolean isSorted ();`
    #[inline]
    pub unsafe fn IsSorted(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsSorted)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Methods used by the drag feedback code to determine if a drag is allowable at
    ///    * the current location. To get the behavior where drops are only allowed on
    ///    * items, such as the mailNews folder pane, always return false when
    ///    * the orientation is not DROP_ON.
    ///    */
    /// ```
    ///

    /// `boolean canDrop (in long index, in long orientation, in DataTransfer dataTransfer);`
    #[inline]
    pub unsafe fn CanDrop(&self, index: i32, orientation: i32, dataTransfer: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).CanDrop)(self, index, orientation, dataTransfer, _retval)
    }


    /// ```text
    /// /**
    ///    * Called when the user drops something on this view. The |orientation| param
    ///    * specifies before/on/after the given |row|.
    ///    */
    /// ```
    ///

    /// `void drop (in long row, in long orientation, in DataTransfer dataTransfer);`
    #[inline]
    pub unsafe fn Drop(&self, row: i32, orientation: i32, dataTransfer: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).Drop)(self, row, orientation, dataTransfer)
    }


    /// ```text
    /// /**
    ///    * Methods used by the tree to draw thread lines in the tree.
    ///    * getParentIndex is used to obtain the index of a parent row.
    ///    * If there is no parent row, getParentIndex returns -1.
    ///    */
    /// ```
    ///

    /// `long getParentIndex (in long rowIndex);`
    #[inline]
    pub unsafe fn GetParentIndex(&self, rowIndex: i32, _retval: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetParentIndex)(self, rowIndex, _retval)
    }


    /// ```text
    /// /**
    ///    * hasNextSibling is used to determine if the row at rowIndex has a nextSibling
    ///    * that occurs *after* the index specified by afterIndex.  Code that is forced
    ///    * to march down the view looking at levels can optimize the march by starting
    ///    * at afterIndex+1.
    ///    */
    /// ```
    ///

    /// `boolean hasNextSibling (in long rowIndex, in long afterIndex);`
    #[inline]
    pub unsafe fn HasNextSibling(&self, rowIndex: i32, afterIndex: i32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasNextSibling)(self, rowIndex, afterIndex, _retval)
    }


    /// ```text
    /// /**
    ///    * The level is an integer value that represents
    ///    * the level of indentation.  It is multiplied by the width specified in the
    ///    * :moz-tree-indentation pseudoelement to compute the exact indendation.
    ///    */
    /// ```
    ///

    /// `long getLevel (in long index);`
    #[inline]
    pub unsafe fn GetLevel(&self, index: i32, _retval: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetLevel)(self, index, _retval)
    }


    /// ```text
    /// /**
    ///    * The image path for a given cell. For defining an icon for a cell.
    ///    * If the empty string is returned, the :moz-tree-image pseudoelement
    ///    * will be used.
    ///    */
    /// ```
    ///

    /// `AString getImageSrc (in long row, in TreeColumn col);`
    #[inline]
    pub unsafe fn GetImageSrc(&self, row: i32, col: *const libc::c_void, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetImageSrc)(self, row, col, _retval)
    }


    /// ```text
    /// /**
    ///    * The value for a given cell. This method is only called for columns
    ///    * of type other than |text|.
    ///    */
    /// ```
    ///

    /// `AString getCellValue (in long row, in TreeColumn col);`
    #[inline]
    pub unsafe fn GetCellValue(&self, row: i32, col: *const libc::c_void, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetCellValue)(self, row, col, _retval)
    }


    /// ```text
    /// /**
    ///    * The text for a given cell.  If a column consists only of an image, then
    ///    * the empty string is returned.
    ///    */
    /// ```
    ///

    /// `AString getCellText (in long row, in TreeColumn col);`
    #[inline]
    pub unsafe fn GetCellText(&self, row: i32, col: *const libc::c_void, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetCellText)(self, row, col, _retval)
    }


    /// ```text
    /// /**
    ///    * Called during initialization to link the view to the front end box object.
    ///    */
    /// ```
    ///

    /// `void setTree (in XULTreeElement tree);`
    #[inline]
    pub unsafe fn SetTree(&self, tree: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).SetTree)(self, tree)
    }


    /// ```text
    /// /**
    ///    * Called on the view when an item is opened or closed.
    ///    */
    /// ```
    ///

    /// `void toggleOpenState (in long index);`
    #[inline]
    pub unsafe fn ToggleOpenState(&self, index: i32) -> ::nserror::nsresult {
        ((*self.vtable).ToggleOpenState)(self, index)
    }


    /// ```text
    /// /**
    ///    * Called on the view when a header is clicked.
    ///    */
    /// ```
    ///

    /// `void cycleHeader (in TreeColumn col);`
    #[inline]
    pub unsafe fn CycleHeader(&self, col: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).CycleHeader)(self, col)
    }


    /// ```text
    /// /**
    ///    * Should be called from a XUL onselect handler whenever the selection changes.
    ///    */
    /// ```
    ///

    /// `[binaryname(SelectionChangedXPCOM)] void selectionChanged ();`
    #[inline]
    pub unsafe fn SelectionChangedXPCOM(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SelectionChangedXPCOM)(self, )
    }


    /// ```text
    /// /**
    ///    * Called on the view when a cell in a non-selectable cycling column (e.g., unread/flag/etc.) is clicked.
    ///    */
    /// ```
    ///

    /// `void cycleCell (in long row, in TreeColumn col);`
    #[inline]
    pub unsafe fn CycleCell(&self, row: i32, col: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).CycleCell)(self, row, col)
    }


    /// ```text
    /// /**
    ///    * isEditable is called to ask the view if the cell contents are editable.
    ///    * A value of true will result in the tree popping up a text field when
    ///    * the user tries to inline edit the cell.
    ///    */
    /// ```
    ///

    /// `boolean isEditable (in long row, in TreeColumn col);`
    #[inline]
    pub unsafe fn IsEditable(&self, row: i32, col: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsEditable)(self, row, col, _retval)
    }


    /// ```text
    /// /**
    ///    * setCellValue is called when the value of the cell has been set by the user.
    ///    * This method is only called for columns of type other than |text|.
    ///    */
    /// ```
    ///

    /// `void setCellValue (in long row, in TreeColumn col, in AString value);`
    #[inline]
    pub unsafe fn SetCellValue(&self, row: i32, col: *const libc::c_void, value: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetCellValue)(self, row, col, value)
    }


    /// ```text
    /// /**
    ///    * setCellText is called when the contents of the cell have been edited by the user.
    ///    */
    /// ```
    ///

    /// `void setCellText (in long row, in TreeColumn col, in AString value);`
    #[inline]
    pub unsafe fn SetCellText(&self, row: i32, col: *const libc::c_void, value: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetCellText)(self, row, col, value)
    }


}


