//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/url-classifier/nsIUrlClassifierUtils.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUrlClassifierParseFindFullHashCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onCompleteHashFound (in ACString aCompleteHash, in ACString aTableNames, in unsigned long aPerHashCacheDuration); */
                    Method {
                        name: "OnCompleteHashFound",
                        params: &[Param { name: "aCompleteHash", ty: "*const ::nsstring::nsACString" }, Param { name: "aTableNames", ty: "*const ::nsstring::nsACString" }, Param { name: "aPerHashCacheDuration", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onResponseParsed (in unsigned long aMinWaitDuration, in unsigned long aNegCacheDuration); */
                    Method {
                        name: "OnResponseParsed",
                        params: &[Param { name: "aMinWaitDuration", ty: "u32" }, Param { name: "aNegCacheDuration", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIUrlClassifierUtils",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* ACString getKeyForURI (in nsIURI uri); */
                    Method {
                        name: "GetKeyForURI",
                        params: &[Param { name: "uri", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString getProvider (in ACString tableName); */
                    Method {
                        name: "GetProvider",
                        params: &[Param { name: "tableName", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString getTelemetryProvider (in ACString tableName); */
                    Method {
                        name: "GetTelemetryProvider",
                        params: &[Param { name: "tableName", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString getProtocolVersion (in ACString provider); */
                    Method {
                        name: "GetProtocolVersion",
                        params: &[Param { name: "provider", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString convertThreatTypeToListNames (in uint32_t threatType); */
                    Method {
                        name: "ConvertThreatTypeToListNames",
                        params: &[Param { name: "threatType", ty: "uint32_t" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* uint32_t convertListNameToThreatType (in ACString listName); */
                    Method {
                        name: "ConvertListNameToThreatType",
                        params: &[Param { name: "listName", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString makeUpdateRequestV4 (in Array<ACString> aListNames, in Array<ACString> aStatesBase64); */
                    Method {
                        name: "MakeUpdateRequestV4",
                        params: &[Param { name: "aListNames", ty: "*const thin_vec::ThinVec<::nsstring::nsCString>" }, Param { name: "aStatesBase64", ty: "*const thin_vec::ThinVec<::nsstring::nsCString>" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString makeFindFullHashRequestV4 (in Array<ACString> aListNames, in Array<ACString> aListStatesBase64, in Array<ACString> aPrefixes); */
                    Method {
                        name: "MakeFindFullHashRequestV4",
                        params: &[Param { name: "aListNames", ty: "*const thin_vec::ThinVec<::nsstring::nsCString>" }, Param { name: "aListStatesBase64", ty: "*const thin_vec::ThinVec<::nsstring::nsCString>" }, Param { name: "aPrefixes", ty: "*const thin_vec::ThinVec<::nsstring::nsCString>" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString makeThreatHitReport (in nsIChannel aChannel, in ACString aListName, in ACString aHashBase64); */
                    Method {
                        name: "MakeThreatHitReport",
                        params: &[Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "aListName", ty: "*const ::nsstring::nsACString" }, Param { name: "aHashBase64", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void parseFindFullHashResponseV4 (in ACString aResponse, in nsIUrlClassifierParseFindFullHashCallback aCallback); */
                    Method {
                        name: "ParseFindFullHashResponseV4",
                        params: &[Param { name: "aResponse", ty: "*const ::nsstring::nsACString" }, Param { name: "aCallback", ty: "*const nsIUrlClassifierParseFindFullHashCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

