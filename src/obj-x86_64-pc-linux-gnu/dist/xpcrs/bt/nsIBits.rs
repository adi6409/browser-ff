//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/bitsdownload/nsIBits.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIBits",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute boolean initialized; */
                    Method {
                        name: "GetInitialized",
                        params: &[Param { name: "aInitialized", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void init (in AUTF8String jobName, in AUTF8String savePathPrefix, in unsigned long monitorTimeoutMs); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "jobName", ty: "*const ::nsstring::nsACString" }, Param { name: "savePathPrefix", ty: "*const ::nsstring::nsACString" }, Param { name: "monitorTimeoutMs", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void startDownload (in AUTF8String downloadURL, in AUTF8String saveRelativePath, in nsProxyUsage proxy, in unsigned long noProgressTimeoutSecs, in unsigned long monitorIntervalMs, in nsIRequestObserver observer, in nsISupports context, in nsIBitsNewRequestCallback callback); */
                    Method {
                        name: "StartDownload",
                        params: &[Param { name: "downloadURL", ty: "*const ::nsstring::nsACString" }, Param { name: "saveRelativePath", ty: "*const ::nsstring::nsACString" }, Param { name: "proxy", ty: "nsProxyUsage" }, Param { name: "noProgressTimeoutSecs", ty: "u32" }, Param { name: "monitorIntervalMs", ty: "u32" }, Param { name: "observer", ty: "*const nsIRequestObserver" }, Param { name: "context", ty: "*const nsISupports" }, Param { name: "callback", ty: "*const nsIBitsNewRequestCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void monitorDownload (in AUTF8String id, in unsigned long monitorIntervalMs, in nsIRequestObserver observer, in nsISupports context, in nsIBitsNewRequestCallback callback); */
                    Method {
                        name: "MonitorDownload",
                        params: &[Param { name: "id", ty: "*const ::nsstring::nsACString" }, Param { name: "monitorIntervalMs", ty: "u32" }, Param { name: "observer", ty: "*const nsIRequestObserver" }, Param { name: "context", ty: "*const nsISupports" }, Param { name: "callback", ty: "*const nsIBitsNewRequestCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIBitsNewRequestCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void success (in nsIBitsRequest request); */
                    Method {
                        name: "Success",
                        params: &[Param { name: "request", ty: "*const nsIBitsRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void failure (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage); */
                    Method {
                        name: "Failure",
                        params: &[Param { name: "errorType", ty: "nsBitsErrorType" }, Param { name: "errorAction", ty: "nsBitsErrorAction" }, Param { name: "errorStage", ty: "nsBitsErrorStage" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void failureNsresult (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage, in nsresult errorCode); */
                    Method {
                        name: "FailureNsresult",
                        params: &[Param { name: "errorType", ty: "nsBitsErrorType" }, Param { name: "errorAction", ty: "nsBitsErrorAction" }, Param { name: "errorStage", ty: "nsBitsErrorStage" }, Param { name: "errorCode", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void failureHresult (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage, in long errorCode); */
                    Method {
                        name: "FailureHresult",
                        params: &[Param { name: "errorType", ty: "nsBitsErrorType" }, Param { name: "errorAction", ty: "nsBitsErrorAction" }, Param { name: "errorStage", ty: "nsBitsErrorStage" }, Param { name: "errorCode", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void failureString (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage, in AUTF8String errorMessage); */
                    Method {
                        name: "FailureString",
                        params: &[Param { name: "errorType", ty: "nsBitsErrorType" }, Param { name: "errorAction", ty: "nsBitsErrorAction" }, Param { name: "errorStage", ty: "nsBitsErrorStage" }, Param { name: "errorMessage", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIBitsRequest",
            base: Some("nsIRequest"),
            methods: Ok(&[
                    /* readonly attribute AUTF8String bitsId; */
                    Method {
                        name: "GetBitsId",
                        params: &[Param { name: "aBitsId", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsBitsErrorType transferError; */
                    Method {
                        name: "GetTransferError",
                        params: &[Param { name: "aTransferError", ty: "*mut nsBitsErrorType" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void changeMonitorInterval (in unsigned long monitorIntervalMs, in nsIBitsCallback callback); */
                    Method {
                        name: "ChangeMonitorInterval",
                        params: &[Param { name: "monitorIntervalMs", ty: "u32" }, Param { name: "callback", ty: "*const nsIBitsCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void cancelAsync (in nsresult status, in nsIBitsCallback callback); */
                    Method {
                        name: "CancelAsync",
                        params: &[Param { name: "status", ty: "::nserror::nsresult" }, Param { name: "callback", ty: "*const nsIBitsCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setPriorityHigh (in nsIBitsCallback callback); */
                    Method {
                        name: "SetPriorityHigh",
                        params: &[Param { name: "callback", ty: "*const nsIBitsCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setPriorityLow (in nsIBitsCallback callback); */
                    Method {
                        name: "SetPriorityLow",
                        params: &[Param { name: "callback", ty: "*const nsIBitsCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setNoProgressTimeout (in unsigned long timeoutSecs, in nsIBitsCallback callback); */
                    Method {
                        name: "SetNoProgressTimeout",
                        params: &[Param { name: "timeoutSecs", ty: "u32" }, Param { name: "callback", ty: "*const nsIBitsCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void complete (in nsIBitsCallback callback); */
                    Method {
                        name: "Complete",
                        params: &[Param { name: "callback", ty: "*const nsIBitsCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void suspendAsync (in nsIBitsCallback callback); */
                    Method {
                        name: "SuspendAsync",
                        params: &[Param { name: "callback", ty: "*const nsIBitsCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void resumeAsync (in nsIBitsCallback callback); */
                    Method {
                        name: "ResumeAsync",
                        params: &[Param { name: "callback", ty: "*const nsIBitsCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIBitsCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void success (); */
                    Method {
                        name: "Success",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void failure (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage); */
                    Method {
                        name: "Failure",
                        params: &[Param { name: "errorType", ty: "nsBitsErrorType" }, Param { name: "errorAction", ty: "nsBitsErrorAction" }, Param { name: "errorStage", ty: "nsBitsErrorStage" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void failureNsresult (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage, in nsresult errorCode); */
                    Method {
                        name: "FailureNsresult",
                        params: &[Param { name: "errorType", ty: "nsBitsErrorType" }, Param { name: "errorAction", ty: "nsBitsErrorAction" }, Param { name: "errorStage", ty: "nsBitsErrorStage" }, Param { name: "errorCode", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void failureHresult (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage, in long errorCode); */
                    Method {
                        name: "FailureHresult",
                        params: &[Param { name: "errorType", ty: "nsBitsErrorType" }, Param { name: "errorAction", ty: "nsBitsErrorAction" }, Param { name: "errorStage", ty: "nsBitsErrorStage" }, Param { name: "errorCode", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void failureString (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage, in AUTF8String errorMessage); */
                    Method {
                        name: "FailureString",
                        params: &[Param { name: "errorType", ty: "nsBitsErrorType" }, Param { name: "errorAction", ty: "nsBitsErrorAction" }, Param { name: "errorStage", ty: "nsBitsErrorStage" }, Param { name: "errorMessage", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

