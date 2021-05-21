//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/osfile/nsINativeOSFileInternals.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsINativeOSFileResult",
            base: Some("nsISupports"),
            methods: Err("specialtype jsval unsupported"),
        },

        Interface {
            name: "nsINativeOSFileSuccessCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void complete (in nsINativeOSFileResult result); */
                    Method {
                        name: "Complete",
                        params: &[Param { name: "result", ty: "*const nsINativeOSFileResult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsINativeOSFileErrorCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void complete (in ACString operation, in long OSstatus); */
                    Method {
                        name: "Complete",
                        params: &[Param { name: "operation", ty: "*const ::nsstring::nsACString" }, Param { name: "OSstatus", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsINativeOSFileInternalsService",
            base: Some("nsISupports"),
            methods: Err("specialtype jsval unsupported"),
        },

        ]; D}

