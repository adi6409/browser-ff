//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageBaseStatement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIStorageBaseStatement",
            base: Some("mozIStorageBindingParams"),
            methods: Ok(&[
                    /* void finalize (); */
                    Method {
                        name: "Finalize",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void bindParameters (in mozIStorageBindingParamsArray aParameters); */
                    Method {
                        name: "BindParameters",
                        params: &[Param { name: "aParameters", ty: "*const mozIStorageBindingParamsArray" }],
                        ret: "::nserror::nsresult",
                    },

                    /* mozIStorageBindingParamsArray newBindingParamsArray (); */
                    Method {
                        name: "NewBindingParamsArray",
                        params: &[Param { name: "_retval", ty: "*mut*const mozIStorageBindingParamsArray" }],
                        ret: "::nserror::nsresult",
                    },

                    /* mozIStoragePendingStatement executeAsync ([optional] in mozIStorageStatementCallback aCallback); */
                    Method {
                        name: "ExecuteAsync",
                        params: &[Param { name: "aCallback", ty: "*const mozIStorageStatementCallback" }, Param { name: "_retval", ty: "*mut*const mozIStoragePendingStatement" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long state; */
                    Method {
                        name: "GetState",
                        params: &[Param { name: "aState", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString escapeStringForLIKE (in AString aValue, in wchar aEscapeChar); */
                    Method {
                        name: "EscapeStringForLIKE",
                        params: &[Param { name: "aValue", ty: "*const ::nsstring::nsAString" }, Param { name: "aEscapeChar", ty: "i16" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AUTF8String escapeUTF8StringForLIKE (in AUTF8String aValue, in char aEscapeChar); */
                    Method {
                        name: "EscapeUTF8StringForLIKE",
                        params: &[Param { name: "aValue", ty: "*const ::nsstring::nsACString" }, Param { name: "aEscapeChar", ty: "libc::c_char" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

