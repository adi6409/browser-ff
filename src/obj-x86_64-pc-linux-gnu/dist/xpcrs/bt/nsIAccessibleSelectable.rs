//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleSelectable.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleSelectable",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsIArray selectedItems; */
                    Method {
                        name: "GetSelectedItems",
                        params: &[Param { name: "aSelectedItems", ty: "*mut*const nsIArray" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long selectedItemCount; */
                    Method {
                        name: "GetSelectedItemCount",
                        params: &[Param { name: "aSelectedItemCount", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIAccessible getSelectedItemAt (in unsigned long index); */
                    Method {
                        name: "GetSelectedItemAt",
                        params: &[Param { name: "index", ty: "u32" }, Param { name: "_retval", ty: "*mut*const nsIAccessible" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isItemSelected (in unsigned long index); */
                    Method {
                        name: "IsItemSelected",
                        params: &[Param { name: "index", ty: "u32" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addItemToSelection (in unsigned long index); */
                    Method {
                        name: "AddItemToSelection",
                        params: &[Param { name: "index", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeItemFromSelection (in unsigned long index); */
                    Method {
                        name: "RemoveItemFromSelection",
                        params: &[Param { name: "index", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean selectAll (); */
                    Method {
                        name: "SelectAll",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void unselectAll (); */
                    Method {
                        name: "UnselectAll",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

