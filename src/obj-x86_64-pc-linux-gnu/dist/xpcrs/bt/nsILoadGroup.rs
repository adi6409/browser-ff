//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsILoadGroup.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsILoadGroup",
            base: Some("nsIRequest"),
            methods: Ok(&[
                    /* attribute nsIRequestObserver groupObserver; */
                    Method {
                        name: "GetGroupObserver",
                        params: &[Param { name: "aGroupObserver", ty: "*mut*const nsIRequestObserver" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetGroupObserver",
                        params: &[Param { name: "aGroupObserver", ty: "*const nsIRequestObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIRequest defaultLoadRequest; */
                    Method {
                        name: "GetDefaultLoadRequest",
                        params: &[Param { name: "aDefaultLoadRequest", ty: "*mut *const nsIRequest" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetDefaultLoadRequest",
                        params: &[Param { name: "aDefaultLoadRequest", ty: "*const nsIRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addRequest (in nsIRequest aRequest, in nsISupports aContext); */
                    Method {
                        name: "AddRequest",
                        params: &[Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aContext", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeRequest (in nsIRequest aRequest, in nsISupports aContext, in nsresult aStatus); */
                    Method {
                        name: "RemoveRequest",
                        params: &[Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aContext", ty: "*const nsISupports" }, Param { name: "aStatus", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsISimpleEnumerator requests; */
                    Method {
                        name: "GetRequests",
                        params: &[Param { name: "aRequests", ty: "*mut*const nsISimpleEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long activeCount; */
                    Method {
                        name: "GetActiveCount",
                        params: &[Param { name: "aActiveCount", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIInterfaceRequestor notificationCallbacks; */
                    Method {
                        name: "GetNotificationCallbacks",
                        params: &[Param { name: "aNotificationCallbacks", ty: "*mut*const nsIInterfaceRequestor" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetNotificationCallbacks",
                        params: &[Param { name: "aNotificationCallbacks", ty: "*const nsIInterfaceRequestor" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long long requestContextID; */
                    Method {
                        name: "GetRequestContextID",
                        params: &[Param { name: "aRequestContextID", ty: "*mut u64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsLoadFlags defaultLoadFlags; */
                    Method {
                        name: "GetDefaultLoadFlags",
                        params: &[Param { name: "aDefaultLoadFlags", ty: "*mut nsLoadFlags" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetDefaultLoadFlags",
                        params: &[Param { name: "aDefaultLoadFlags", ty: "nsLoadFlags" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [infallible] readonly attribute boolean isBrowsingContextDiscarded; */
                    Method {
                        name: "GetIsBrowsingContextDiscarded",
                        params: &[Param { name: "aIsBrowsingContextDiscarded", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

