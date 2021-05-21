//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/autocomplete/nsIAutoCompletePopup.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAutoCompletePopup",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsIAutoCompleteInput input; */
                    Method {
                        name: "GetInput",
                        params: &[Param { name: "aInput", ty: "*mut*const nsIAutoCompleteInput" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString overrideValue; */
                    Method {
                        name: "GetOverrideValue",
                        params: &[Param { name: "aOverrideValue", ty: "*mut ::nsstring::nsAString" }],
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

                    /* readonly attribute boolean popupOpen; */
                    Method {
                        name: "GetPopupOpen",
                        params: &[Param { name: "aPopupOpen", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void openAutocompletePopup (in nsIAutoCompleteInput input, in Element element); */
                    Method {
                        name: "OpenAutocompletePopup",
                        params: &[Param { name: "input", ty: "*const nsIAutoCompleteInput" }, Param { name: "element", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void closePopup (); */
                    Method {
                        name: "ClosePopup",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void invalidate (in unsigned short reason); */
                    Method {
                        name: "Invalidate",
                        params: &[Param { name: "reason", ty: "u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void selectBy (in boolean reverse, in boolean page); */
                    Method {
                        name: "SelectBy",
                        params: &[Param { name: "reverse", ty: "bool" }, Param { name: "page", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

