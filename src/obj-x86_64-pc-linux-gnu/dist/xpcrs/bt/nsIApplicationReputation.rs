//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/reputationservice/nsIApplicationReputation.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIApplicationReputationService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void queryReputation (in nsIApplicationReputationQuery aQuery, in nsIApplicationReputationCallback aCallback); */
                    Method {
                        name: "QueryReputation",
                        params: &[Param { name: "aQuery", ty: "*const nsIApplicationReputationQuery" }, Param { name: "aCallback", ty: "*const nsIApplicationReputationCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* bool isBinary (in AUTF8String aFilename); */
                    Method {
                        name: "IsBinary",
                        params: &[Param { name: "aFilename", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIApplicationReputationQuery",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsIURI sourceURI; */
                    Method {
                        name: "GetSourceURI",
                        params: &[Param { name: "aSourceURI", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIReferrerInfo referrerInfo; */
                    Method {
                        name: "GetReferrerInfo",
                        params: &[Param { name: "aReferrerInfo", ty: "*mut*const nsIReferrerInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String suggestedFileName; */
                    Method {
                        name: "GetSuggestedFileName",
                        params: &[Param { name: "aSuggestedFileName", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long fileSize; */
                    Method {
                        name: "GetFileSize",
                        params: &[Param { name: "aFileSize", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString sha256Hash; */
                    Method {
                        name: "GetSha256Hash",
                        params: &[Param { name: "aSha256Hash", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute Array<Array<Array<uint8_t>>> signatureInfo; */
                    Method {
                        name: "GetSignatureInfo",
                        params: &[Param { name: "aSignatureInfo", ty: "*mut thin_vec::ThinVec<thin_vec::ThinVec<thin_vec::ThinVec<uint8_t>>>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIArray redirects; */
                    Method {
                        name: "GetRedirects",
                        params: &[Param { name: "aRedirects", ty: "*mut*const nsIArray" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIApplicationReputationCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onComplete (in bool aShouldBlock, in nsresult aStatus, in unsigned long aVerdict); */
                    Method {
                        name: "OnComplete",
                        params: &[Param { name: "aShouldBlock", ty: "bool" }, Param { name: "aStatus", ty: "::nserror::nsresult" }, Param { name: "aVerdict", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

