//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIErrorService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIErrorService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void registerErrorStringBundle (in short errorModule, in string stringBundleURL); */
                    Method {
                        name: "RegisterErrorStringBundle",
                        params: &[Param { name: "errorModule", ty: "i16" }, Param { name: "stringBundleURL", ty: "*const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void unregisterErrorStringBundle (in short errorModule); */
                    Method {
                        name: "UnregisterErrorStringBundle",
                        params: &[Param { name: "errorModule", ty: "i16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* string getErrorStringBundle (in short errorModule); */
                    Method {
                        name: "GetErrorStringBundle",
                        params: &[Param { name: "errorModule", ty: "i16" }, Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

