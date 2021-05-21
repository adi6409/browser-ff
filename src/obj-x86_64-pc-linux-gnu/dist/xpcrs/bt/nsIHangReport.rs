//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/ipc/nsIHangReport.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHangReport",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute unsigned long hangType; */
                    Method {
                        name: "GetHangType",
                        params: &[Param { name: "aHangType", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute Element scriptBrowser; */
                    Method {
                        name: "GetScriptBrowser",
                        params: &[Param { name: "aScriptBrowser", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString scriptFileName; */
                    Method {
                        name: "GetScriptFileName",
                        params: &[Param { name: "aScriptFileName", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute double hangDuration; */
                    Method {
                        name: "GetHangDuration",
                        params: &[Param { name: "aHangDuration", ty: "*mut libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString addonId; */
                    Method {
                        name: "GetAddonId",
                        params: &[Param { name: "aAddonId", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString pluginName; */
                    Method {
                        name: "GetPluginName",
                        params: &[Param { name: "aPluginName", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long long childID; */
                    Method {
                        name: "GetChildID",
                        params: &[Param { name: "aChildID", ty: "*mut u64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void userCanceled (); */
                    Method {
                        name: "UserCanceled",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void terminateScript (); */
                    Method {
                        name: "TerminateScript",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void terminateGlobal (); */
                    Method {
                        name: "TerminateGlobal",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void terminatePlugin (); */
                    Method {
                        name: "TerminatePlugin",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void beginStartingDebugger (); */
                    Method {
                        name: "BeginStartingDebugger",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void endStartingDebugger (); */
                    Method {
                        name: "EndStartingDebugger",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* bool isReportForBrowser (in FrameLoader aFrameLoader); */
                    Method {
                        name: "IsReportForBrowser",
                        params: &[Param { name: "aFrameLoader", ty: "*const libc::c_void" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

