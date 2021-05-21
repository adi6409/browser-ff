//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/xul/nsIDOMXULRadioGroupElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMXULRadioGroupElement",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute Element focusedItem; */
                    Method {
                        name: "GetFocusedItem",
                        params: &[Param { name: "aFocusedItem", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetFocusedItem",
                        params: &[Param { name: "aFocusedItem", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

