//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/xul/nsIDOMXULSelectCntrlEl.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMXULSelectControlElement",
            base: Some("nsIDOMXULControlElement"),
            methods: Ok(&[
                    /* attribute Element selectedItem; */
                    Method {
                        name: "GetSelectedItem",
                        params: &[Param { name: "aSelectedItem", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetSelectedItem",
                        params: &[Param { name: "aSelectedItem", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute long selectedIndex; */
                    Method {
                        name: "GetSelectedIndex",
                        params: &[Param { name: "aSelectedIndex", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetSelectedIndex",
                        params: &[Param { name: "aSelectedIndex", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString value; */
                    Method {
                        name: "GetValue",
                        params: &[Param { name: "aValue", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetValue",
                        params: &[Param { name: "aValue", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long itemCount; */
                    Method {
                        name: "GetItemCount",
                        params: &[Param { name: "aItemCount", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* long getIndexOfItem (in nsIDOMXULSelectControlItemElement item); */
                    Method {
                        name: "GetIndexOfItem",
                        params: &[Param { name: "item", ty: "*const nsIDOMXULSelectControlItemElement" }, Param { name: "_retval", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Element getItemAtIndex (in long index); */
                    Method {
                        name: "GetItemAtIndex",
                        params: &[Param { name: "index", ty: "i32" }, Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

