//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/layout/xul/tree/nsITreeView.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITreeView",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute long rowCount; */
                    Method {
                        name: "GetRowCount",
                        params: &[Param { name: "aRowCount", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsITreeSelection selection; */
                    Method {
                        name: "GetSelection",
                        params: &[Param { name: "aSelection", ty: "*mut*const nsITreeSelection" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetSelection",
                        params: &[Param { name: "aSelection", ty: "*const nsITreeSelection" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getRowProperties (in long index); */
                    Method {
                        name: "GetRowProperties",
                        params: &[Param { name: "index", ty: "i32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getCellProperties (in long row, in TreeColumn col); */
                    Method {
                        name: "GetCellProperties",
                        params: &[Param { name: "row", ty: "i32" }, Param { name: "col", ty: "*const libc::c_void" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getColumnProperties (in TreeColumn col); */
                    Method {
                        name: "GetColumnProperties",
                        params: &[Param { name: "col", ty: "*const libc::c_void" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isContainer (in long index); */
                    Method {
                        name: "IsContainer",
                        params: &[Param { name: "index", ty: "i32" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isContainerOpen (in long index); */
                    Method {
                        name: "IsContainerOpen",
                        params: &[Param { name: "index", ty: "i32" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isContainerEmpty (in long index); */
                    Method {
                        name: "IsContainerEmpty",
                        params: &[Param { name: "index", ty: "i32" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isSeparator (in long index); */
                    Method {
                        name: "IsSeparator",
                        params: &[Param { name: "index", ty: "i32" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isSorted (); */
                    Method {
                        name: "IsSorted",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean canDrop (in long index, in long orientation, in DataTransfer dataTransfer); */
                    Method {
                        name: "CanDrop",
                        params: &[Param { name: "index", ty: "i32" }, Param { name: "orientation", ty: "i32" }, Param { name: "dataTransfer", ty: "*const libc::c_void" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void drop (in long row, in long orientation, in DataTransfer dataTransfer); */
                    Method {
                        name: "Drop",
                        params: &[Param { name: "row", ty: "i32" }, Param { name: "orientation", ty: "i32" }, Param { name: "dataTransfer", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* long getParentIndex (in long rowIndex); */
                    Method {
                        name: "GetParentIndex",
                        params: &[Param { name: "rowIndex", ty: "i32" }, Param { name: "_retval", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean hasNextSibling (in long rowIndex, in long afterIndex); */
                    Method {
                        name: "HasNextSibling",
                        params: &[Param { name: "rowIndex", ty: "i32" }, Param { name: "afterIndex", ty: "i32" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* long getLevel (in long index); */
                    Method {
                        name: "GetLevel",
                        params: &[Param { name: "index", ty: "i32" }, Param { name: "_retval", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getImageSrc (in long row, in TreeColumn col); */
                    Method {
                        name: "GetImageSrc",
                        params: &[Param { name: "row", ty: "i32" }, Param { name: "col", ty: "*const libc::c_void" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getCellValue (in long row, in TreeColumn col); */
                    Method {
                        name: "GetCellValue",
                        params: &[Param { name: "row", ty: "i32" }, Param { name: "col", ty: "*const libc::c_void" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getCellText (in long row, in TreeColumn col); */
                    Method {
                        name: "GetCellText",
                        params: &[Param { name: "row", ty: "i32" }, Param { name: "col", ty: "*const libc::c_void" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setTree (in XULTreeElement tree); */
                    Method {
                        name: "SetTree",
                        params: &[Param { name: "tree", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void toggleOpenState (in long index); */
                    Method {
                        name: "ToggleOpenState",
                        params: &[Param { name: "index", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void cycleHeader (in TreeColumn col); */
                    Method {
                        name: "CycleHeader",
                        params: &[Param { name: "col", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [binaryname(SelectionChangedXPCOM)] void selectionChanged (); */
                    Method {
                        name: "SelectionChangedXPCOM",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void cycleCell (in long row, in TreeColumn col); */
                    Method {
                        name: "CycleCell",
                        params: &[Param { name: "row", ty: "i32" }, Param { name: "col", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isEditable (in long row, in TreeColumn col); */
                    Method {
                        name: "IsEditable",
                        params: &[Param { name: "row", ty: "i32" }, Param { name: "col", ty: "*const libc::c_void" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setCellValue (in long row, in TreeColumn col, in AString value); */
                    Method {
                        name: "SetCellValue",
                        params: &[Param { name: "row", ty: "i32" }, Param { name: "col", ty: "*const libc::c_void" }, Param { name: "value", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setCellText (in long row, in TreeColumn col, in AString value); */
                    Method {
                        name: "SetCellText",
                        params: &[Param { name: "row", ty: "i32" }, Param { name: "col", ty: "*const libc::c_void" }, Param { name: "value", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

