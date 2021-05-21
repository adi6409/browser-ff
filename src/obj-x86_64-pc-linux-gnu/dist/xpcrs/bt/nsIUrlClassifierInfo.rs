//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/url-classifier/nsIUrlClassifierInfo.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUrlClassifierPositiveCacheEntry",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute ACString fullhash; */
                    Method {
                        name: "GetFullhash",
                        params: &[Param { name: "aFullhash", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long long expiry; */
                    Method {
                        name: "GetExpiry",
                        params: &[Param { name: "aExpiry", ty: "*mut i64" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIUrlClassifierCacheEntry",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute ACString prefix; */
                    Method {
                        name: "GetPrefix",
                        params: &[Param { name: "aPrefix", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long long expiry; */
                    Method {
                        name: "GetExpiry",
                        params: &[Param { name: "aExpiry", ty: "*mut i64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIArray matches; */
                    Method {
                        name: "GetMatches",
                        params: &[Param { name: "aMatches", ty: "*mut *const nsIArray" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIUrlClassifierCacheInfo",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute ACString table; */
                    Method {
                        name: "GetTable",
                        params: &[Param { name: "aTable", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIArray entries; */
                    Method {
                        name: "GetEntries",
                        params: &[Param { name: "aEntries", ty: "*mut *const nsIArray" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIUrlClassifierGetCacheCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onGetCacheComplete (in nsIUrlClassifierCacheInfo info); */
                    Method {
                        name: "OnGetCacheComplete",
                        params: &[Param { name: "info", ty: "*const nsIUrlClassifierCacheInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIUrlClassifierInfo",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void getCacheInfo (in ACString table, in nsIUrlClassifierGetCacheCallback callback); */
                    Method {
                        name: "GetCacheInfo",
                        params: &[Param { name: "table", ty: "*const ::nsstring::nsACString" }, Param { name: "callback", ty: "*const nsIUrlClassifierGetCacheCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

