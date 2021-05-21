//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIRefreshURI.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRefreshURI",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void refreshURI (in nsIURI aURI, in nsIPrincipal aPrincipal, in long aMillis, in boolean aRepeat, in boolean aMetaRefresh); */
                    Method {
                        name: "RefreshURI",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aMillis", ty: "i32" }, Param { name: "aRepeat", ty: "bool" }, Param { name: "aMetaRefresh", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void forceRefreshURI (in nsIURI aURI, in nsIPrincipal aPrincipal, in long aMillis, in boolean aMetaRefresh); */
                    Method {
                        name: "ForceRefreshURI",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aMillis", ty: "i32" }, Param { name: "aMetaRefresh", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setupRefreshURI (in nsIChannel aChannel); */
                    Method {
                        name: "SetupRefreshURI",
                        params: &[Param { name: "aChannel", ty: "*const nsIChannel" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setupRefreshURIFromHeader (in nsIURI aBaseURI, in nsIPrincipal principal, in unsigned long long aInnerWindowID, in ACString aHeader); */
                    Method {
                        name: "SetupRefreshURIFromHeader",
                        params: &[Param { name: "aBaseURI", ty: "*const nsIURI" }, Param { name: "principal", ty: "*const nsIPrincipal" }, Param { name: "aInnerWindowID", ty: "u64" }, Param { name: "aHeader", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void cancelRefreshURITimers (); */
                    Method {
                        name: "CancelRefreshURITimers",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean refreshPending; */
                    Method {
                        name: "GetRefreshPending",
                        params: &[Param { name: "aRefreshPending", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

