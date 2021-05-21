//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/search/nsISearchService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISearchSubmission",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsIInputStream postData; */
                    Method {
                        name: "GetPostData",
                        params: &[Param { name: "aPostData", ty: "*mut*const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIURI uri; */
                    Method {
                        name: "GetUri",
                        params: &[Param { name: "aUri", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISearchEngine",
            base: Some("nsISupports"),
            methods: Err("specialtype jsval unsupported"),
        },

        Interface {
            name: "nsISearchParseSubmissionResult",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsISearchEngine engine; */
                    Method {
                        name: "GetEngine",
                        params: &[Param { name: "aEngine", ty: "*mut *const nsISearchEngine" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString terms; */
                    Method {
                        name: "GetTerms",
                        params: &[Param { name: "aTerms", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString termsParameterName; */
                    Method {
                        name: "GetTermsParameterName",
                        params: &[Param { name: "aTermsParameterName", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long termsOffset; */
                    Method {
                        name: "GetTermsOffset",
                        params: &[Param { name: "aTermsOffset", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long termsLength; */
                    Method {
                        name: "GetTermsLength",
                        params: &[Param { name: "aTermsLength", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISearchService",
            base: Some("nsISupports"),
            methods: Err("specialtype promise unsupported"),
        },

        ]; D}

