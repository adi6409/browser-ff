//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/url-classifier/nsIURIClassifier.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIURIClassifierCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onClassifyComplete (in nsresult aErrorCode, in ACString aList, in ACString aProvider, in ACString aFullHash); */
                    Method {
                        name: "OnClassifyComplete",
                        params: &[Param { name: "aErrorCode", ty: "::nserror::nsresult" }, Param { name: "aList", ty: "*const ::nsstring::nsACString" }, Param { name: "aProvider", ty: "*const ::nsstring::nsACString" }, Param { name: "aFullHash", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIURIClassifier",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* boolean classify (in nsIPrincipal aPrincipal, in nsISerialEventTarget aEventTarget, in nsIURIClassifierCallback aCallback); */
                    Method {
                        name: "Classify",
                        params: &[Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aEventTarget", ty: "*const nsISerialEventTarget" }, Param { name: "aCallback", ty: "*const nsIURIClassifierCallback" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void asyncClassifyLocalWithFeatures (in nsIURI aURI, in Array<nsIUrlClassifierFeature> aFeatures, in nsIUrlClassifierFeature_listType aListType, in nsIUrlClassifierFeatureCallback aCallback); */
                    Method {
                        name: "AsyncClassifyLocalWithFeatures",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aFeatures", ty: "*const thin_vec::ThinVec<RefPtr<nsIUrlClassifierFeature>>" }, Param { name: "aListType", ty: " u8" }, Param { name: "aCallback", ty: "*const nsIUrlClassifierFeatureCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIUrlClassifierFeature getFeatureByName (in ACString aFeatureName); */
                    Method {
                        name: "GetFeatureByName",
                        params: &[Param { name: "aFeatureName", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut *const nsIUrlClassifierFeature" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<ACString> getFeatureNames (); */
                    Method {
                        name: "GetFeatureNames",
                        params: &[Param { name: "_retval", ty: "*mut thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIUrlClassifierFeature createFeatureWithTables (in ACString aName, in Array<ACString> aBlocklistTables, in Array<ACString> aEntitylistTables); */
                    Method {
                        name: "CreateFeatureWithTables",
                        params: &[Param { name: "aName", ty: "*const ::nsstring::nsACString" }, Param { name: "aBlocklistTables", ty: "*const thin_vec::ThinVec<::nsstring::nsCString>" }, Param { name: "aEntitylistTables", ty: "*const thin_vec::ThinVec<::nsstring::nsCString>" }, Param { name: "_retval", ty: "*mut *const nsIUrlClassifierFeature" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void sendThreatHitReport (in nsIChannel aChannel, in ACString aProvider, in ACString aList, in ACString aFullHash); */
                    Method {
                        name: "SendThreatHitReport",
                        params: &[Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "aProvider", ty: "*const ::nsstring::nsACString" }, Param { name: "aList", ty: "*const ::nsstring::nsACString" }, Param { name: "aFullHash", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

