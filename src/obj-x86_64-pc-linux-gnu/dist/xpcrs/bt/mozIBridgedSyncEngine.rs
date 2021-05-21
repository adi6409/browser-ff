//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/services/interfaces/mozIBridgedSyncEngine.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIBridgedSyncEngineCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void handleSuccess (in nsIVariant result); */
                    Method {
                        name: "HandleSuccess",
                        params: &[Param { name: "result", ty: "*const nsIVariant" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void handleError (in nsresult code, in AUTF8String message); */
                    Method {
                        name: "HandleError",
                        params: &[Param { name: "code", ty: "::nserror::nsresult" }, Param { name: "message", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "mozIBridgedSyncEngineApplyCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void handleSuccess (in Array<AUTF8String> outgoingEnvelopesAsJSON); */
                    Method {
                        name: "HandleSuccess",
                        params: &[Param { name: "outgoingEnvelopesAsJSON", ty: "*const thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void handleError (in nsresult code, in AUTF8String message); */
                    Method {
                        name: "HandleError",
                        params: &[Param { name: "code", ty: "::nserror::nsresult" }, Param { name: "message", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "mozIBridgedSyncEngine",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute long storageVersion; */
                    Method {
                        name: "GetStorageVersion",
                        params: &[Param { name: "aStorageVersion", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean allowSkippedRecord; */
                    Method {
                        name: "GetAllowSkippedRecord",
                        params: &[Param { name: "aAllowSkippedRecord", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute mozIServicesLogSink logger; */
                    Method {
                        name: "GetLogger",
                        params: &[Param { name: "aLogger", ty: "*mut *const mozIServicesLogSink" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetLogger",
                        params: &[Param { name: "aLogger", ty: "*const mozIServicesLogSink" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getLastSync (in mozIBridgedSyncEngineCallback callback); */
                    Method {
                        name: "GetLastSync",
                        params: &[Param { name: "callback", ty: "*const mozIBridgedSyncEngineCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setLastSync (in long long lastSyncMillis, in mozIBridgedSyncEngineCallback callback); */
                    Method {
                        name: "SetLastSync",
                        params: &[Param { name: "lastSyncMillis", ty: "i64" }, Param { name: "callback", ty: "*const mozIBridgedSyncEngineCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getSyncId (in mozIBridgedSyncEngineCallback callback); */
                    Method {
                        name: "GetSyncId",
                        params: &[Param { name: "callback", ty: "*const mozIBridgedSyncEngineCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void resetSyncId (in mozIBridgedSyncEngineCallback callback); */
                    Method {
                        name: "ResetSyncId",
                        params: &[Param { name: "callback", ty: "*const mozIBridgedSyncEngineCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void ensureCurrentSyncId (in AUTF8String newSyncId, in mozIBridgedSyncEngineCallback callback); */
                    Method {
                        name: "EnsureCurrentSyncId",
                        params: &[Param { name: "newSyncId", ty: "*const ::nsstring::nsACString" }, Param { name: "callback", ty: "*const mozIBridgedSyncEngineCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void syncStarted (in mozIBridgedSyncEngineCallback callback); */
                    Method {
                        name: "SyncStarted",
                        params: &[Param { name: "callback", ty: "*const mozIBridgedSyncEngineCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void storeIncoming (in Array<AUTF8String> incomingEnvelopesAsJSON, in mozIBridgedSyncEngineCallback callback); */
                    Method {
                        name: "StoreIncoming",
                        params: &[Param { name: "incomingEnvelopesAsJSON", ty: "*const thin_vec::ThinVec<::nsstring::nsCString>" }, Param { name: "callback", ty: "*const mozIBridgedSyncEngineCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void apply (in mozIBridgedSyncEngineApplyCallback callback); */
                    Method {
                        name: "Apply",
                        params: &[Param { name: "callback", ty: "*const mozIBridgedSyncEngineApplyCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setUploaded (in long long newTimestampMillis, in Array<AUTF8String> uploadedIds, in mozIBridgedSyncEngineCallback callback); */
                    Method {
                        name: "SetUploaded",
                        params: &[Param { name: "newTimestampMillis", ty: "i64" }, Param { name: "uploadedIds", ty: "*const thin_vec::ThinVec<::nsstring::nsCString>" }, Param { name: "callback", ty: "*const mozIBridgedSyncEngineCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void syncFinished (in mozIBridgedSyncEngineCallback callback); */
                    Method {
                        name: "SyncFinished",
                        params: &[Param { name: "callback", ty: "*const mozIBridgedSyncEngineCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void reset (in mozIBridgedSyncEngineCallback callback); */
                    Method {
                        name: "Reset",
                        params: &[Param { name: "callback", ty: "*const mozIBridgedSyncEngineCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void wipe (in mozIBridgedSyncEngineCallback callback); */
                    Method {
                        name: "Wipe",
                        params: &[Param { name: "callback", ty: "*const mozIBridgedSyncEngineCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

