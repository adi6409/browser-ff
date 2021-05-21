//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/nsITableEditor.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITableEditor",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [can_run_script] void insertTableCell (in long aNumberOfColumnsToInsert, in boolean aInsertAfterSelectedCell); */
                    Method {
                        name: "InsertTableCell",
                        params: &[Param { name: "aNumberOfColumnsToInsert", ty: "i32" }, Param { name: "aInsertAfterSelectedCell", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void insertTableColumn (in long aNumberOfColumnsToInsert, in boolean aInsertAfterSelectedCell); */
                    Method {
                        name: "InsertTableColumn",
                        params: &[Param { name: "aNumberOfColumnsToInsert", ty: "i32" }, Param { name: "aInsertAfterSelectedCell", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void insertTableRow (in long aNumberOfRowsToInsert, in boolean aInsertAfterSelectedCell); */
                    Method {
                        name: "InsertTableRow",
                        params: &[Param { name: "aNumberOfRowsToInsert", ty: "i32" }, Param { name: "aInsertAfterSelectedCell", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void deleteTable (); */
                    Method {
                        name: "DeleteTable",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void deleteTableCellContents (); */
                    Method {
                        name: "DeleteTableCellContents",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void deleteTableCell (in long aNumberOfCellsToDelete); */
                    Method {
                        name: "DeleteTableCell",
                        params: &[Param { name: "aNumberOfCellsToDelete", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void deleteTableColumn (in long aNumberOfColumnsToDelete); */
                    Method {
                        name: "DeleteTableColumn",
                        params: &[Param { name: "aNumberOfColumnsToDelete", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void deleteTableRow (in long aNumberOfRowsToDelete); */
                    Method {
                        name: "DeleteTableRow",
                        params: &[Param { name: "aNumberOfRowsToDelete", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void selectTableCell (); */
                    Method {
                        name: "SelectTableCell",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void selectTableRow (); */
                    Method {
                        name: "SelectTableRow",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void selectTableColumn (); */
                    Method {
                        name: "SelectTableColumn",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void selectTable (); */
                    Method {
                        name: "SelectTable",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void selectAllTableCells (); */
                    Method {
                        name: "SelectAllTableCells",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] Element switchTableCellHeaderType (in Element aSourceCell); */
                    Method {
                        name: "SwitchTableCellHeaderType",
                        params: &[Param { name: "aSourceCell", ty: "*const libc::c_void" }, Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void joinTableCells (in boolean aMergeNonContiguousContents); */
                    Method {
                        name: "JoinTableCells",
                        params: &[Param { name: "aMergeNonContiguousContents", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void splitTableCell (); */
                    Method {
                        name: "SplitTableCell",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void normalizeTable (in Element aTable); */
                    Method {
                        name: "NormalizeTable",
                        params: &[Param { name: "aTable", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void getCellIndexes (in Element aCellElement, out long aRowIndex, out long aColumnIndex); */
                    Method {
                        name: "GetCellIndexes",
                        params: &[Param { name: "aCellElement", ty: "*const libc::c_void" }, Param { name: "aRowIndex", ty: "*mut i32" }, Param { name: "aColumnIndex", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getTableSize (in Element aTableOrElementInTable, out long aRowCount, out long aColCount); */
                    Method {
                        name: "GetTableSize",
                        params: &[Param { name: "aTableOrElementInTable", ty: "*const libc::c_void" }, Param { name: "aRowCount", ty: "*mut i32" }, Param { name: "aColCount", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Element getCellAt (in Element aTableElement, in long aRowIndex, in long aColumnIndex); */
                    Method {
                        name: "GetCellAt",
                        params: &[Param { name: "aTableElement", ty: "*const libc::c_void" }, Param { name: "aRowIndex", ty: "i32" }, Param { name: "aColumnIndex", ty: "i32" }, Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getCellDataAt (in Element aTableElement, in long aRowIndex, in long aColumnIndex, out Element aCellElement, out long aStartRowIndex, out long aStartColumnIndex, out long aRowSpan, out long aColSpan, out long aEffectiveRowSpan, out long aEffectiveColSpan, out boolean aIsSelected); */
                    Method {
                        name: "GetCellDataAt",
                        params: &[Param { name: "aTableElement", ty: "*const libc::c_void" }, Param { name: "aRowIndex", ty: "i32" }, Param { name: "aColumnIndex", ty: "i32" }, Param { name: "aCellElement", ty: "*mut *const libc::c_void" }, Param { name: "aStartRowIndex", ty: "*mut i32" }, Param { name: "aStartColumnIndex", ty: "*mut i32" }, Param { name: "aRowSpan", ty: "*mut i32" }, Param { name: "aColSpan", ty: "*mut i32" }, Param { name: "aEffectiveRowSpan", ty: "*mut i32" }, Param { name: "aEffectiveColSpan", ty: "*mut i32" }, Param { name: "aIsSelected", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Element getFirstRow (in Element aTableElement); */
                    Method {
                        name: "GetFirstRow",
                        params: &[Param { name: "aTableElement", ty: "*const libc::c_void" }, Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Element getSelectedOrParentTableElement (out AString aTagName, out long aCount); */
                    Method {
                        name: "GetSelectedOrParentTableElement",
                        params: &[Param { name: "aTagName", ty: "*mut ::nsstring::nsAString" }, Param { name: "aCount", ty: "*mut i32" }, Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] uint32_t getSelectedCellsType (in Element aElement); */
                    Method {
                        name: "GetSelectedCellsType",
                        params: &[Param { name: "aElement", ty: "*const libc::c_void" }, Param { name: "_retval", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] Element getFirstSelectedCellInTable (out long aRowIndex, out long aColIndex); */
                    Method {
                        name: "GetFirstSelectedCellInTable",
                        params: &[Param { name: "aRowIndex", ty: "*mut i32" }, Param { name: "aColIndex", ty: "*mut i32" }, Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<Element> getSelectedCells (); */
                    Method {
                        name: "GetSelectedCells",
                        params: &[Param { name: "_retval", ty: "*mut thin_vec::ThinVec<*const libc::c_void>" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

