//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/url-classifier/nsIUrlClassifierHashCompleter.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFullHashMatch",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute ACString tableName; */
                    Method {
                        name: "GetTableName",
                        params: &[Param { name: "aTableName", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString fullHash; */
                    Method {
                        name: "GetFullHash",
                        params: &[Param { name: "aFullHash", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint32_t cacheDuration; */
                    Method {
                        name: "GetCacheDuration",
                        params: &[Param { name: "aCacheDuration", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIUrlClassifierHashCompleterCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void completionV2 (in ACString hash, in ACString table, in uint32_t chunkId); */
                    Method {
                        name: "CompletionV2",
                        params: &[Param { name: "hash", ty: "*const ::nsstring::nsACString" }, Param { name: "table", ty: "*const ::nsstring::nsACString" }, Param { name: "chunkId", ty: "uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void completionV4 (in ACString partialHash, in ACString table, in uint32_t negativeCacheDuration, in nsIArray fullHashes); */
                    Method {
                        name: "CompletionV4",
                        params: &[Param { name: "partialHash", ty: "*const ::nsstring::nsACString" }, Param { name: "table", ty: "*const ::nsstring::nsACString" }, Param { name: "negativeCacheDuration", ty: "uint32_t" }, Param { name: "fullHashes", ty: "*const nsIArray" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void completionFinished (in nsresult status); */
                    Method {
                        name: "CompletionFinished",
                        params: &[Param { name: "status", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIUrlClassifierHashCompleter",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void complete (in ACString partialHash, in ACString gethashUrl, in ACString tableName, in nsIUrlClassifierHashCompleterCallback callback); */
                    Method {
                        name: "Complete",
                        params: &[Param { name: "partialHash", ty: "*const ::nsstring::nsACString" }, Param { name: "gethashUrl", ty: "*const ::nsstring::nsACString" }, Param { name: "tableName", ty: "*const ::nsstring::nsACString" }, Param { name: "callback", ty: "*const nsIUrlClassifierHashCompleterCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

