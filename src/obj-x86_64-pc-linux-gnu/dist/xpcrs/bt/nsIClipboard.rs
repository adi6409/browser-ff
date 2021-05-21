//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIClipboard.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIClipboard",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void setData (in nsITransferable aTransferable, in nsIClipboardOwner anOwner, in long aWhichClipboard); */
                    Method {
                        name: "SetData",
                        params: &[Param { name: "aTransferable", ty: "*const nsITransferable" }, Param { name: "anOwner", ty: "*const nsIClipboardOwner" }, Param { name: "aWhichClipboard", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getData (in nsITransferable aTransferable, in long aWhichClipboard); */
                    Method {
                        name: "GetData",
                        params: &[Param { name: "aTransferable", ty: "*const nsITransferable" }, Param { name: "aWhichClipboard", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void emptyClipboard (in long aWhichClipboard); */
                    Method {
                        name: "EmptyClipboard",
                        params: &[Param { name: "aWhichClipboard", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean hasDataMatchingFlavors (in Array<ACString> aFlavorList, in long aWhichClipboard); */
                    Method {
                        name: "HasDataMatchingFlavors",
                        params: &[Param { name: "aFlavorList", ty: "*const thin_vec::ThinVec<::nsstring::nsCString>" }, Param { name: "aWhichClipboard", ty: "i32" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean supportsSelectionClipboard (); */
                    Method {
                        name: "SupportsSelectionClipboard",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean supportsFindClipboard (); */
                    Method {
                        name: "SupportsFindClipboard",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

