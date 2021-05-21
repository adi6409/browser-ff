//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/system/nsICrashReporter.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICrashReporter",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute boolean enabled; */
                    Method {
                        name: "GetEnabled",
                        params: &[Param { name: "aEnabled", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] void setEnabled (in bool enabled); */
                    Method {
                        name: "SetEnabled",
                        params: &[Param { name: "enabled", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIURL serverURL; */
                    Method {
                        name: "GetServerURL",
                        params: &[Param { name: "aServerURL", ty: "*mut*const nsIURL" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetServerURL",
                        params: &[Param { name: "aServerURL", ty: "*const nsIURL" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIFile minidumpPath; */
                    Method {
                        name: "GetMinidumpPath",
                        params: &[Param { name: "aMinidumpPath", ty: "*mut*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetMinidumpPath",
                        params: &[Param { name: "aMinidumpPath", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIFile getMinidumpForID (in AString id); */
                    Method {
                        name: "GetMinidumpForID",
                        params: &[Param { name: "id", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIFile getExtraFileForID (in AString id); */
                    Method {
                        name: "GetExtraFileForID",
                        params: &[Param { name: "id", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void annotateCrashReport (in AUTF8String key, in AUTF8String data); */
                    Method {
                        name: "AnnotateCrashReport",
                        params: &[Param { name: "key", ty: "*const ::nsstring::nsACString" }, Param { name: "data", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeCrashReportAnnotation (in AUTF8String key); */
                    Method {
                        name: "RemoveCrashReportAnnotation",
                        params: &[Param { name: "key", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isAnnotationWhitelistedForPing (in ACString value); */
                    Method {
                        name: "IsAnnotationWhitelistedForPing",
                        params: &[Param { name: "value", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void appendAppNotesToCrashReport (in ACString data); */
                    Method {
                        name: "AppendAppNotesToCrashReport",
                        params: &[Param { name: "data", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void registerAppMemory (in unsigned long long ptr, in unsigned long long size); */
                    Method {
                        name: "RegisterAppMemory",
                        params: &[Param { name: "ptr", ty: "u64" }, Param { name: "size", ty: "u64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] void writeMinidumpForException (in voidPtr aExceptionInfo); */
                    Method {
                        name: "WriteMinidumpForException",
                        params: &[Param { name: "aExceptionInfo", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] void appendObjCExceptionInfoToAppNotes (in voidPtr aException); */
                    Method {
                        name: "AppendObjCExceptionInfoToAppNotes",
                        params: &[Param { name: "aException", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean submitReports; */
                    Method {
                        name: "GetSubmitReports",
                        params: &[Param { name: "aSubmitReports", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetSubmitReports",
                        params: &[Param { name: "aSubmitReports", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void UpdateCrashEventsDir (); */
                    Method {
                        name: "UpdateCrashEventsDir",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void saveMemoryReport (); */
                    Method {
                        name: "SaveMemoryReport",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

