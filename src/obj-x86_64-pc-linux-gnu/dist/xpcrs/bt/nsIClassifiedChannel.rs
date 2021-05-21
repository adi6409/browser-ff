//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIClassifiedChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIClassifiedChannel",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void setMatchedInfo (in ACString aList, in ACString aProvider, in ACString aFullHash); */
                    Method {
                        name: "SetMatchedInfo",
                        params: &[Param { name: "aList", ty: "*const ::nsstring::nsACString" }, Param { name: "aProvider", ty: "*const ::nsstring::nsACString" }, Param { name: "aFullHash", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString matchedList; */
                    Method {
                        name: "GetMatchedList",
                        params: &[Param { name: "aMatchedList", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString matchedProvider; */
                    Method {
                        name: "GetMatchedProvider",
                        params: &[Param { name: "aMatchedProvider", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString matchedFullHash; */
                    Method {
                        name: "GetMatchedFullHash",
                        params: &[Param { name: "aMatchedFullHash", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setMatchedTrackingInfo (in Array<ACString> aLists, in Array<ACString> aFullHashes); */
                    Method {
                        name: "SetMatchedTrackingInfo",
                        params: &[Param { name: "aLists", ty: "*const thin_vec::ThinVec<::nsstring::nsCString>" }, Param { name: "aFullHashes", ty: "*const thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute Array<ACString> matchedTrackingLists; */
                    Method {
                        name: "GetMatchedTrackingLists",
                        params: &[Param { name: "aMatchedTrackingLists", ty: "*mut thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute Array<ACString> matchedTrackingFullHashes; */
                    Method {
                        name: "GetMatchedTrackingFullHashes",
                        params: &[Param { name: "aMatchedTrackingFullHashes", ty: "*mut thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [infallible] readonly attribute unsigned long firstPartyClassificationFlags; */
                    Method {
                        name: "GetFirstPartyClassificationFlags",
                        params: &[Param { name: "aFirstPartyClassificationFlags", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [infallible] readonly attribute unsigned long thirdPartyClassificationFlags; */
                    Method {
                        name: "GetThirdPartyClassificationFlags",
                        params: &[Param { name: "aThirdPartyClassificationFlags", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [infallible] readonly attribute unsigned long classificationFlags; */
                    Method {
                        name: "GetClassificationFlags",
                        params: &[Param { name: "aClassificationFlags", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isThirdPartyTrackingResource (); */
                    Method {
                        name: "IsThirdPartyTrackingResource",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isThirdPartySocialTrackingResource (); */
                    Method {
                        name: "IsThirdPartySocialTrackingResource",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

