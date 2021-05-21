//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsISpeculativeConnect.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISpeculativeConnect",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void speculativeConnect (in nsIURI aURI, in nsIPrincipal aPrincipal, in nsIInterfaceRequestor aCallbacks); */
                    Method {
                        name: "SpeculativeConnect",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aCallbacks", ty: "*const nsIInterfaceRequestor" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void speculativeAnonymousConnect (in nsIURI aURI, in nsIPrincipal aPrincipal, in nsIInterfaceRequestor aCallbacks); */
                    Method {
                        name: "SpeculativeAnonymousConnect",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aCallbacks", ty: "*const nsIInterfaceRequestor" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISpeculativeConnectionOverrider",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [infallible] readonly attribute unsigned long parallelSpeculativeConnectLimit; */
                    Method {
                        name: "GetParallelSpeculativeConnectLimit",
                        params: &[Param { name: "aParallelSpeculativeConnectLimit", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [infallible] readonly attribute boolean ignoreIdle; */
                    Method {
                        name: "GetIgnoreIdle",
                        params: &[Param { name: "aIgnoreIdle", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [infallible] readonly attribute boolean isFromPredictor; */
                    Method {
                        name: "GetIsFromPredictor",
                        params: &[Param { name: "aIsFromPredictor", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [infallible] readonly attribute boolean allow1918; */
                    Method {
                        name: "GetAllow1918",
                        params: &[Param { name: "aAllow1918", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

