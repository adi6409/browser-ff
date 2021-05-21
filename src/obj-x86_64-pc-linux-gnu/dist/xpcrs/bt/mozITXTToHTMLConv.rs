//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/streamconv/mozITXTToHTMLConv.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozITXTToHTMLConv",
            base: Some("nsIStreamConverter"),
            methods: Ok(&[
                    /* AString scanTXT (in AString text, in unsigned long whattodo); */
                    Method {
                        name: "ScanTXT",
                        params: &[Param { name: "text", ty: "*const ::nsstring::nsAString" }, Param { name: "whattodo", ty: "u32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString scanHTML (in AString text, in unsigned long whattodo); */
                    Method {
                        name: "ScanHTML",
                        params: &[Param { name: "text", ty: "*const ::nsstring::nsAString" }, Param { name: "whattodo", ty: "u32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* unsigned long citeLevelTXT (in wstring line, out unsigned long logLineStart); */
                    Method {
                        name: "CiteLevelTXT",
                        params: &[Param { name: "line", ty: "*const i16" }, Param { name: "logLineStart", ty: "*mut u32" }, Param { name: "_retval", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void findURLInPlaintext (in wstring text, in long aLength, in long aPos, out long aStartPos, out long aEndPos); */
                    Method {
                        name: "FindURLInPlaintext",
                        params: &[Param { name: "text", ty: "*const i16" }, Param { name: "aLength", ty: "i32" }, Param { name: "aPos", ty: "i32" }, Param { name: "aStartPos", ty: "*mut i32" }, Param { name: "aEndPos", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

