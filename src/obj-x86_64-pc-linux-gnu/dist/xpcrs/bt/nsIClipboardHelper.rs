//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIClipboardHelper.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIClipboardHelper",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void copyStringToClipboard (in AString aString, in long aClipboardID); */
                    Method {
                        name: "CopyStringToClipboard",
                        params: &[Param { name: "aString", ty: "*const ::nsstring::nsAString" }, Param { name: "aClipboardID", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void copyString (in AString aString); */
                    Method {
                        name: "CopyString",
                        params: &[Param { name: "aString", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

