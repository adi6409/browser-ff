//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIMemoryInfoDumper.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFinishDumpingCallback",
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
            name: "nsIDumpGCAndCCLogsCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onDump (in nsIFile aGCLog, in nsIFile aCCLog, in bool aIsParent); */
                    Method {
                        name: "OnDump",
                        params: &[Param { name: "aGCLog", ty: "*const nsIFile" }, Param { name: "aCCLog", ty: "*const nsIFile" }, Param { name: "aIsParent", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onFinish (); */
                    Method {
                        name: "OnFinish",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIMemoryInfoDumper",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void dumpMemoryReportsToNamedFile (in AString aFilename, in nsIFinishDumpingCallback aFinishDumping, in nsISupports aFinishDumpingData, in boolean aAnonymize, in boolean aMinimizeMemoryUsage); */
                    Method {
                        name: "DumpMemoryReportsToNamedFile",
                        params: &[Param { name: "aFilename", ty: "*const ::nsstring::nsAString" }, Param { name: "aFinishDumping", ty: "*const nsIFinishDumpingCallback" }, Param { name: "aFinishDumpingData", ty: "*const nsISupports" }, Param { name: "aAnonymize", ty: "bool" }, Param { name: "aMinimizeMemoryUsage", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void dumpMemoryInfoToTempDir (in AString aIdentifier, in boolean aAnonymize, in boolean aMinimizeMemoryUsage); */
                    Method {
                        name: "DumpMemoryInfoToTempDir",
                        params: &[Param { name: "aIdentifier", ty: "*const ::nsstring::nsAString" }, Param { name: "aAnonymize", ty: "bool" }, Param { name: "aMinimizeMemoryUsage", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void dumpGCAndCCLogsToFile (in AString aIdentifier, in bool aDumpAllTraces, in bool aDumpChildProcesses, in nsIDumpGCAndCCLogsCallback aCallback); */
                    Method {
                        name: "DumpGCAndCCLogsToFile",
                        params: &[Param { name: "aIdentifier", ty: "*const ::nsstring::nsAString" }, Param { name: "aDumpAllTraces", ty: "bool" }, Param { name: "aDumpChildProcesses", ty: "bool" }, Param { name: "aCallback", ty: "*const nsIDumpGCAndCCLogsCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void dumpGCAndCCLogsToSink (in bool aDumpAllTraces, in nsICycleCollectorLogSink aSink); */
                    Method {
                        name: "DumpGCAndCCLogsToSink",
                        params: &[Param { name: "aDumpAllTraces", ty: "bool" }, Param { name: "aSink", ty: "*const nsICycleCollectorLogSink" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

