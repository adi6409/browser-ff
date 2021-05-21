//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/places/mozIAsyncHistory.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIVisitInfo",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute long long visitId; */
                    Method {
                        name: "GetVisitId",
                        params: &[Param { name: "aVisitId", ty: "*mut i64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute PRTime visitDate; */
                    Method {
                        name: "GetVisitDate",
                        params: &[Param { name: "aVisitDate", ty: "*mut PRTime" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long transitionType; */
                    Method {
                        name: "GetTransitionType",
                        params: &[Param { name: "aTransitionType", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIURI referrerURI; */
                    Method {
                        name: "GetReferrerURI",
                        params: &[Param { name: "aReferrerURI", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "mozIPlaceInfo",
            base: Some("nsISupports"),
            methods: Err("specialtype jsval unsupported"),
        },

        Interface {
            name: "mozIVisitInfoCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void handleError (in nsresult aResultCode, in mozIPlaceInfo aPlaceInfo); */
                    Method {
                        name: "HandleError",
                        params: &[Param { name: "aResultCode", ty: "::nserror::nsresult" }, Param { name: "aPlaceInfo", ty: "*const mozIPlaceInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void handleResult (in mozIPlaceInfo aPlaceInfo); */
                    Method {
                        name: "HandleResult",
                        params: &[Param { name: "aPlaceInfo", ty: "*const mozIPlaceInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void handleCompletion (in unsigned long aUpdatedItems); */
                    Method {
                        name: "HandleCompletion",
                        params: &[Param { name: "aUpdatedItems", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute bool ignoreResults; */
                    Method {
                        name: "GetIgnoreResults",
                        params: &[Param { name: "aIgnoreResults", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute bool ignoreErrors; */
                    Method {
                        name: "GetIgnoreErrors",
                        params: &[Param { name: "aIgnoreErrors", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "mozIVisitedStatusCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void isVisited (in nsIURI aURI, in boolean aVisitedStatus); */
                    Method {
                        name: "IsVisited",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aVisitedStatus", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "mozIAsyncHistory",
            base: Some("nsISupports"),
            methods: Err("specialtype jsval unsupported"),
        },

        ]; D}

