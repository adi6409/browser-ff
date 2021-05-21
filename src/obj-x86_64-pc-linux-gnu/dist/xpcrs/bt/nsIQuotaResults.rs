//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/quota/nsIQuotaResults.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIQuotaUsageResult",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute ACString origin; */
                    Method {
                        name: "GetOrigin",
                        params: &[Param { name: "aOrigin", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean persisted; */
                    Method {
                        name: "GetPersisted",
                        params: &[Param { name: "aPersisted", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long long usage; */
                    Method {
                        name: "GetUsage",
                        params: &[Param { name: "aUsage", ty: "*mut u64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long long lastAccessed; */
                    Method {
                        name: "GetLastAccessed",
                        params: &[Param { name: "aLastAccessed", ty: "*mut u64" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIQuotaOriginUsageResult",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute unsigned long long usage; */
                    Method {
                        name: "GetUsage",
                        params: &[Param { name: "aUsage", ty: "*mut u64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long long fileUsage; */
                    Method {
                        name: "GetFileUsage",
                        params: &[Param { name: "aFileUsage", ty: "*mut u64" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIQuotaEstimateResult",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute unsigned long long usage; */
                    Method {
                        name: "GetUsage",
                        params: &[Param { name: "aUsage", ty: "*mut u64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long long limit; */
                    Method {
                        name: "GetLimit",
                        params: &[Param { name: "aLimit", ty: "*mut u64" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

