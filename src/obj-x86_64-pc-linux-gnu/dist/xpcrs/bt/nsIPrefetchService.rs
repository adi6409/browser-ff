//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/prefetch/nsIPrefetchService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPrefetchService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void prefetchURI (in nsIURI aURI, in nsIReferrerInfo aReferrerInfo, in Node aSource, in boolean aExplicit); */
                    Method {
                        name: "PrefetchURI",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aReferrerInfo", ty: "*const nsIReferrerInfo" }, Param { name: "aSource", ty: "*const libc::c_void" }, Param { name: "aExplicit", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void preloadURI (in nsIURI aURI, in nsIReferrerInfo aReferrerInfo, in Node aSource, in nsContentPolicyType aPolicyType); */
                    Method {
                        name: "PreloadURI",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aReferrerInfo", ty: "*const nsIReferrerInfo" }, Param { name: "aSource", ty: "*const libc::c_void" }, Param { name: "aPolicyType", ty: "nsContentPolicyType" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean hasMoreElements (); */
                    Method {
                        name: "HasMoreElements",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void cancelPrefetchPreloadURI (in nsIURI aURI, in Node aSource); */
                    Method {
                        name: "CancelPrefetchPreloadURI",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aSource", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

