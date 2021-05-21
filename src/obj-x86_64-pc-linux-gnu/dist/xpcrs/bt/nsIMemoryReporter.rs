//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIMemoryReporter.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHandleReportCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void callback (in ACString process, in AUTF8String path, in int32_t kind, in int32_t units, in int64_t amount, in AUTF8String description, in nsISupports data); */
                    Method {
                        name: "Callback",
                        params: &[Param { name: "process", ty: "*const ::nsstring::nsACString" }, Param { name: "path", ty: "*const ::nsstring::nsACString" }, Param { name: "kind", ty: "int32_t" }, Param { name: "units", ty: "int32_t" }, Param { name: "amount", ty: "int64_t" }, Param { name: "description", ty: "*const ::nsstring::nsACString" }, Param { name: "data", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIMemoryReporter",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void collectReports (in nsIHandleReportCallback callback, in nsISupports data, in boolean anonymize); */
                    Method {
                        name: "CollectReports",
                        params: &[Param { name: "callback", ty: "*const nsIHandleReportCallback" }, Param { name: "data", ty: "*const nsISupports" }, Param { name: "anonymize", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIFinishReportingCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void callback (in nsISupports data); */
                    Method {
                        name: "Callback",
                        params: &[Param { name: "data", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIHeapAllocatedCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void callback (in int64_t bytesAllocated); */
                    Method {
                        name: "Callback",
                        params: &[Param { name: "bytesAllocated", ty: "int64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIMemoryReporterManager",
            base: Some("nsISupports"),
            methods: Err("native type FILE unsupported"),
        },

        ]; D}

