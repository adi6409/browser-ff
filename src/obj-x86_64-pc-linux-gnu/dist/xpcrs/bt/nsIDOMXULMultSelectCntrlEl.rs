//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/xul/nsIDOMXULMultSelectCntrlEl.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMXULMultiSelectControlElement",
            base: Some("nsIDOMXULSelectControlElement"),
            methods: Ok(&[
                    /* attribute AString selType; */
                    Method {
                        name: "GetSelType",
                        params: &[Param { name: "aSelType", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetSelType",
                        params: &[Param { name: "aSelType", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute Element currentItem; */
                    Method {
                        name: "GetCurrentItem",
                        params: &[Param { name: "aCurrentItem", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetCurrentItem",
                        params: &[Param { name: "aCurrentItem", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute long currentIndex; */
                    Method {
                        name: "GetCurrentIndex",
                        params: &[Param { name: "aCurrentIndex", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetCurrentIndex",
                        params: &[Param { name: "aCurrentIndex", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute NodeList selectedItems; */
                    Method {
                        name: "GetSelectedItems",
                        params: &[Param { name: "aSelectedItems", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addItemToSelection (in nsIDOMXULSelectControlItemElement item); */
                    Method {
                        name: "AddItemToSelection",
                        params: &[Param { name: "item", ty: "*const nsIDOMXULSelectControlItemElement" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeItemFromSelection (in nsIDOMXULSelectControlItemElement item); */
                    Method {
                        name: "RemoveItemFromSelection",
                        params: &[Param { name: "item", ty: "*const nsIDOMXULSelectControlItemElement" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void toggleItemSelection (in nsIDOMXULSelectControlItemElement item); */
                    Method {
                        name: "ToggleItemSelection",
                        params: &[Param { name: "item", ty: "*const nsIDOMXULSelectControlItemElement" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void selectItem (in nsIDOMXULSelectControlItemElement item); */
                    Method {
                        name: "SelectItem",
                        params: &[Param { name: "item", ty: "*const nsIDOMXULSelectControlItemElement" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void selectItemRange (in nsIDOMXULSelectControlItemElement startItem, in nsIDOMXULSelectControlItemElement item); */
                    Method {
                        name: "SelectItemRange",
                        params: &[Param { name: "startItem", ty: "*const nsIDOMXULSelectControlItemElement" }, Param { name: "item", ty: "*const nsIDOMXULSelectControlItemElement" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void selectAll (); */
                    Method {
                        name: "SelectAll",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void clearSelection (); */
                    Method {
                        name: "ClearSelection",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long selectedCount; */
                    Method {
                        name: "GetSelectedCount",
                        params: &[Param { name: "aSelectedCount", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [binaryname(MultiGetSelectedItem)] Element getSelectedItem (in long index); */
                    Method {
                        name: "MultiGetSelectedItem",
                        params: &[Param { name: "index", ty: "i32" }, Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

