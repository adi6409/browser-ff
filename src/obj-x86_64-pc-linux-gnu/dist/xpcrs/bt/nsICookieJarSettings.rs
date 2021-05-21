//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cookie/nsICookieJarSettings.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICookieJarSettings",
            base: Some("nsISerializable"),
            methods: Ok(&[
                    /* [infallible] readonly attribute unsigned long cookieBehavior; */
                    Method {
                        name: "GetCookieBehavior",
                        params: &[Param { name: "aCookieBehavior", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [infallible] readonly attribute boolean isFirstPartyIsolated; */
                    Method {
                        name: "GetIsFirstPartyIsolated",
                        params: &[Param { name: "aIsFirstPartyIsolated", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [infallible] readonly attribute boolean rejectThirdPartyContexts; */
                    Method {
                        name: "GetRejectThirdPartyContexts",
                        params: &[Param { name: "aRejectThirdPartyContexts", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [infallible] readonly attribute boolean limitForeignContexts; */
                    Method {
                        name: "GetLimitForeignContexts",
                        params: &[Param { name: "aLimitForeignContexts", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [infallible] attribute boolean partitionForeign; */
                    Method {
                        name: "GetPartitionForeign",
                        params: &[Param { name: "aPartitionForeign", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetPartitionForeign",
                        params: &[Param { name: "aPartitionForeign", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [infallible] readonly attribute boolean isOnContentBlockingAllowList; */
                    Method {
                        name: "GetIsOnContentBlockingAllowList",
                        params: &[Param { name: "aIsOnContentBlockingAllowList", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString partitionKey; */
                    Method {
                        name: "GetPartitionKey",
                        params: &[Param { name: "aPartitionKey", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* unsigned long cookiePermission (in nsIPrincipal aPrincipal); */
                    Method {
                        name: "CookiePermission",
                        params: &[Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "_retval", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

