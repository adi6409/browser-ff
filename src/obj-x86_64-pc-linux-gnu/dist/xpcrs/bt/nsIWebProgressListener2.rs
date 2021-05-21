//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/base/nsIWebProgressListener2.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebProgressListener2",
            base: Some("nsIWebProgressListener"),
            methods: Ok(&[
                    /* void onProgressChange64 (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in long long aCurSelfProgress, in long long aMaxSelfProgress, in long long aCurTotalProgress, in long long aMaxTotalProgress); */
                    Method {
                        name: "OnProgressChange64",
                        params: &[Param { name: "aWebProgress", ty: "*const nsIWebProgress" }, Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aCurSelfProgress", ty: "i64" }, Param { name: "aMaxSelfProgress", ty: "i64" }, Param { name: "aCurTotalProgress", ty: "i64" }, Param { name: "aMaxTotalProgress", ty: "i64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean onRefreshAttempted (in nsIWebProgress aWebProgress, in nsIURI aRefreshURI, in long aMillis, in boolean aSameURI); */
                    Method {
                        name: "OnRefreshAttempted",
                        params: &[Param { name: "aWebProgress", ty: "*const nsIWebProgress" }, Param { name: "aRefreshURI", ty: "*const nsIURI" }, Param { name: "aMillis", ty: "i32" }, Param { name: "aSameURI", ty: "bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

