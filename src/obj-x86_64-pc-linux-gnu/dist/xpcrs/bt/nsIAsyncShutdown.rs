//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/asyncshutdown/nsIAsyncShutdown.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAsyncShutdownBlocker",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AString name; */
                    Method {
                        name: "GetName",
                        params: &[Param { name: "aName", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void blockShutdown (in nsIAsyncShutdownClient aBarrierClient); */
                    Method {
                        name: "BlockShutdown",
                        params: &[Param { name: "aBarrierClient", ty: "*const nsIAsyncShutdownClient" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIPropertyBag state; */
                    Method {
                        name: "GetState",
                        params: &[Param { name: "aState", ty: "*mut *const nsIPropertyBag" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIAsyncShutdownClient",
            base: Some("nsISupports"),
            methods: Err("specialtype jsval unsupported"),
        },

        Interface {
            name: "nsIAsyncShutdownCompletionCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void done (); */
                    Method {
                        name: "Done",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIAsyncShutdownBarrier",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsIAsyncShutdownClient client; */
                    Method {
                        name: "GetClient",
                        params: &[Param { name: "aClient", ty: "*mut *const nsIAsyncShutdownClient" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIPropertyBag state; */
                    Method {
                        name: "GetState",
                        params: &[Param { name: "aState", ty: "*mut *const nsIPropertyBag" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void wait (in nsIAsyncShutdownCompletionCallback aOnReady); */
                    Method {
                        name: "Wait",
                        params: &[Param { name: "aOnReady", ty: "*const nsIAsyncShutdownCompletionCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIAsyncShutdownService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIAsyncShutdownBarrier makeBarrier (in AString aName); */
                    Method {
                        name: "MakeBarrier",
                        params: &[Param { name: "aName", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut *const nsIAsyncShutdownBarrier" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIAsyncShutdownClient profileBeforeChange; */
                    Method {
                        name: "GetProfileBeforeChange",
                        params: &[Param { name: "aProfileBeforeChange", ty: "*mut *const nsIAsyncShutdownClient" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIAsyncShutdownClient profileChangeTeardown; */
                    Method {
                        name: "GetProfileChangeTeardown",
                        params: &[Param { name: "aProfileChangeTeardown", ty: "*mut *const nsIAsyncShutdownClient" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIAsyncShutdownClient quitApplicationGranted; */
                    Method {
                        name: "GetQuitApplicationGranted",
                        params: &[Param { name: "aQuitApplicationGranted", ty: "*mut *const nsIAsyncShutdownClient" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIAsyncShutdownClient sendTelemetry; */
                    Method {
                        name: "GetSendTelemetry",
                        params: &[Param { name: "aSendTelemetry", ty: "*mut *const nsIAsyncShutdownClient" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIAsyncShutdownClient webWorkersShutdown; */
                    Method {
                        name: "GetWebWorkersShutdown",
                        params: &[Param { name: "aWebWorkersShutdown", ty: "*mut *const nsIAsyncShutdownClient" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIAsyncShutdownClient xpcomWillShutdown; */
                    Method {
                        name: "GetXpcomWillShutdown",
                        params: &[Param { name: "aXpcomWillShutdown", ty: "*mut *const nsIAsyncShutdownClient" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

