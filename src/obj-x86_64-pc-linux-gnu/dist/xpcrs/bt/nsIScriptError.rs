//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/bindings/nsIScriptError.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIScriptErrorNote",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AString errorMessage; */
                    Method {
                        name: "GetErrorMessage",
                        params: &[Param { name: "aErrorMessage", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString sourceName; */
                    Method {
                        name: "GetSourceName",
                        params: &[Param { name: "aSourceName", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint32_t sourceId; */
                    Method {
                        name: "GetSourceId",
                        params: &[Param { name: "aSourceId", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint32_t lineNumber; */
                    Method {
                        name: "GetLineNumber",
                        params: &[Param { name: "aLineNumber", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint32_t columnNumber; */
                    Method {
                        name: "GetColumnNumber",
                        params: &[Param { name: "aColumnNumber", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AUTF8String toString (); */
                    Method {
                        name: "ToString",
                        params: &[Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIScriptError",
            base: Some("nsIConsoleMessage"),
            methods: Err("specialtype jsval unsupported"),
        },

        ]; D}

