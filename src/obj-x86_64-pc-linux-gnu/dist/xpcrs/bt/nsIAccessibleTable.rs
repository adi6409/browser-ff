//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleTable.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleTable",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsIAccessible caption; */
                    Method {
                        name: "GetCaption",
                        params: &[Param { name: "aCaption", ty: "*mut*const nsIAccessible" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString summary; */
                    Method {
                        name: "GetSummary",
                        params: &[Param { name: "aSummary", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long columnCount; */
                    Method {
                        name: "GetColumnCount",
                        params: &[Param { name: "aColumnCount", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long rowCount; */
                    Method {
                        name: "GetRowCount",
                        params: &[Param { name: "aRowCount", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIAccessible getCellAt (in long rowIndex, in long columnIndex); */
                    Method {
                        name: "GetCellAt",
                        params: &[Param { name: "rowIndex", ty: "i32" }, Param { name: "columnIndex", ty: "i32" }, Param { name: "_retval", ty: "*mut*const nsIAccessible" }],
                        ret: "::nserror::nsresult",
                    },

                    /* long getCellIndexAt (in long rowIndex, in long columnIndex); */
                    Method {
                        name: "GetCellIndexAt",
                        params: &[Param { name: "rowIndex", ty: "i32" }, Param { name: "columnIndex", ty: "i32" }, Param { name: "_retval", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* long getColumnIndexAt (in long cellIndex); */
                    Method {
                        name: "GetColumnIndexAt",
                        params: &[Param { name: "cellIndex", ty: "i32" }, Param { name: "_retval", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* long getRowIndexAt (in long cellIndex); */
                    Method {
                        name: "GetRowIndexAt",
                        params: &[Param { name: "cellIndex", ty: "i32" }, Param { name: "_retval", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getRowAndColumnIndicesAt (in long cellIndex, out long rowIndex, out long columnIndex); */
                    Method {
                        name: "GetRowAndColumnIndicesAt",
                        params: &[Param { name: "cellIndex", ty: "i32" }, Param { name: "rowIndex", ty: "*mut i32" }, Param { name: "columnIndex", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* long getColumnExtentAt (in long row, in long column); */
                    Method {
                        name: "GetColumnExtentAt",
                        params: &[Param { name: "row", ty: "i32" }, Param { name: "column", ty: "i32" }, Param { name: "_retval", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* long getRowExtentAt (in long row, in long column); */
                    Method {
                        name: "GetRowExtentAt",
                        params: &[Param { name: "row", ty: "i32" }, Param { name: "column", ty: "i32" }, Param { name: "_retval", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getColumnDescription (in long columnIndex); */
                    Method {
                        name: "GetColumnDescription",
                        params: &[Param { name: "columnIndex", ty: "i32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getRowDescription (in long rowIndex); */
                    Method {
                        name: "GetRowDescription",
                        params: &[Param { name: "rowIndex", ty: "i32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isColumnSelected (in long columnIndex); */
                    Method {
                        name: "IsColumnSelected",
                        params: &[Param { name: "columnIndex", ty: "i32" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isRowSelected (in long rowIndex); */
                    Method {
                        name: "IsRowSelected",
                        params: &[Param { name: "rowIndex", ty: "i32" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isCellSelected (in long rowIndex, in long columnIndex); */
                    Method {
                        name: "IsCellSelected",
                        params: &[Param { name: "rowIndex", ty: "i32" }, Param { name: "columnIndex", ty: "i32" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long selectedCellCount; */
                    Method {
                        name: "GetSelectedCellCount",
                        params: &[Param { name: "aSelectedCellCount", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long selectedColumnCount; */
                    Method {
                        name: "GetSelectedColumnCount",
                        params: &[Param { name: "aSelectedColumnCount", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long selectedRowCount; */
                    Method {
                        name: "GetSelectedRowCount",
                        params: &[Param { name: "aSelectedRowCount", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIArray selectedCells; */
                    Method {
                        name: "GetSelectedCells",
                        params: &[Param { name: "aSelectedCells", ty: "*mut*const nsIArray" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<uint32_t> getSelectedCellIndices (); */
                    Method {
                        name: "GetSelectedCellIndices",
                        params: &[Param { name: "_retval", ty: "*mut thin_vec::ThinVec<uint32_t>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<uint32_t> getSelectedColumnIndices (); */
                    Method {
                        name: "GetSelectedColumnIndices",
                        params: &[Param { name: "_retval", ty: "*mut thin_vec::ThinVec<uint32_t>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<uint32_t> getSelectedRowIndices (); */
                    Method {
                        name: "GetSelectedRowIndices",
                        params: &[Param { name: "_retval", ty: "*mut thin_vec::ThinVec<uint32_t>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void selectRow (in long rowIndex); */
                    Method {
                        name: "SelectRow",
                        params: &[Param { name: "rowIndex", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void selectColumn (in long columnIndex); */
                    Method {
                        name: "SelectColumn",
                        params: &[Param { name: "columnIndex", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void unselectRow (in long rowIndex); */
                    Method {
                        name: "UnselectRow",
                        params: &[Param { name: "rowIndex", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void unselectColumn (in long columnIndex); */
                    Method {
                        name: "UnselectColumn",
                        params: &[Param { name: "columnIndex", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isProbablyForLayout (); */
                    Method {
                        name: "IsProbablyForLayout",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIAccessibleTableCell",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsIAccessibleTable table; */
                    Method {
                        name: "GetTable",
                        params: &[Param { name: "aTable", ty: "*mut *const nsIAccessibleTable" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long columnIndex; */
                    Method {
                        name: "GetColumnIndex",
                        params: &[Param { name: "aColumnIndex", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long rowIndex; */
                    Method {
                        name: "GetRowIndex",
                        params: &[Param { name: "aRowIndex", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long columnExtent; */
                    Method {
                        name: "GetColumnExtent",
                        params: &[Param { name: "aColumnExtent", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long rowExtent; */
                    Method {
                        name: "GetRowExtent",
                        params: &[Param { name: "aRowExtent", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIArray columnHeaderCells; */
                    Method {
                        name: "GetColumnHeaderCells",
                        params: &[Param { name: "aColumnHeaderCells", ty: "*mut*const nsIArray" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIArray rowHeaderCells; */
                    Method {
                        name: "GetRowHeaderCells",
                        params: &[Param { name: "aRowHeaderCells", ty: "*mut*const nsIArray" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isSelected (); */
                    Method {
                        name: "IsSelected",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

