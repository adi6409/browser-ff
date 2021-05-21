//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/quota/nsIQuotaRequests.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIQuotaRequestBase",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsIPrincipal principal; */
                    Method {
                        name: "GetPrincipal",
                        params: &[Param { name: "aPrincipal", ty: "*mut*const nsIPrincipal" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute nsresult resultCode; */
                    Method {
                        name: "GetResultCode",
                        params: &[Param { name: "aResultCode", ty: "*mut ::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute ACString resultName; */
                    Method {
                        name: "GetResultName",
                        params: &[Param { name: "aResultName", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIQuotaUsageRequest",
            base: Some("nsIQuotaRequestBase"),
            methods: Ok(&[
                    /* [must_use] readonly attribute nsIVariant result; */
                    Method {
                        name: "GetResult",
                        params: &[Param { name: "aResult", ty: "*mut*const nsIVariant" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIQuotaUsageCallback callback; */
                    Method {
                        name: "GetCallback",
                        params: &[Param { name: "aCallback", ty: "*mut*const nsIQuotaUsageCallback" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetCallback",
                        params: &[Param { name: "aCallback", ty: "*const nsIQuotaUsageCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void cancel (); */
                    Method {
                        name: "Cancel",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIQuotaRequest",
            base: Some("nsIQuotaRequestBase"),
            methods: Ok(&[
                    /* [must_use] readonly attribute nsIVariant result; */
                    Method {
                        name: "GetResult",
                        params: &[Param { name: "aResult", ty: "*mut*const nsIVariant" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIQuotaCallback callback; */
                    Method {
                        name: "GetCallback",
                        params: &[Param { name: "aCallback", ty: "*mut*const nsIQuotaCallback" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetCallback",
                        params: &[Param { name: "aCallback", ty: "*const nsIQuotaCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

