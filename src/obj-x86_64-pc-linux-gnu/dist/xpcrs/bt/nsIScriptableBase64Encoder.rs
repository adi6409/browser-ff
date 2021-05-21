//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIScriptableBase64Encoder.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIScriptableBase64Encoder",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* ACString encodeToCString (in nsIInputStream stream, in unsigned long length); */
                    Method {
                        name: "EncodeToCString",
                        params: &[Param { name: "stream", ty: "*const nsIInputStream" }, Param { name: "length", ty: "u32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString encodeToString (in nsIInputStream stream, in unsigned long length); */
                    Method {
                        name: "EncodeToString",
                        params: &[Param { name: "stream", ty: "*const nsIInputStream" }, Param { name: "length", ty: "u32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

