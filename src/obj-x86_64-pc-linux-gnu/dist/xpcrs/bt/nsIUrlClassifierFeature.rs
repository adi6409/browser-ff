//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/url-classifier/nsIUrlClassifierFeature.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUrlClassifierFeature",
            base: Some("nsISupports"),
            methods: Err("native type nsTArray<nsCString> unsupported"),
        },

        Interface {
            name: "nsIUrlClassifierFeatureResult",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsIURI uri; */
                    Method {
                        name: "GetUri",
                        params: &[Param { name: "aUri", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIUrlClassifierFeature feature; */
                    Method {
                        name: "GetFeature",
                        params: &[Param { name: "aFeature", ty: "*mut *const nsIUrlClassifierFeature" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString list; */
                    Method {
                        name: "GetList",
                        params: &[Param { name: "aList", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIUrlClassifierFeatureCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onClassifyComplete (in Array<nsIUrlClassifierFeatureResult> aResults); */
                    Method {
                        name: "OnClassifyComplete",
                        params: &[Param { name: "aResults", ty: "*const thin_vec::ThinVec<RefPtr<nsIUrlClassifierFeatureResult>>" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

