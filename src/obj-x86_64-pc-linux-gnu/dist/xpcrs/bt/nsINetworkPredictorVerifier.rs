//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsINetworkPredictorVerifier.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsINetworkPredictorVerifier",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onPredictPrefetch (in nsIURI uri, in uint32_t status); */
                    Method {
                        name: "OnPredictPrefetch",
                        params: &[Param { name: "uri", ty: "*const nsIURI" }, Param { name: "status", ty: "uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onPredictPreconnect (in nsIURI uri); */
                    Method {
                        name: "OnPredictPreconnect",
                        params: &[Param { name: "uri", ty: "*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onPredictDNS (in nsIURI uri); */
                    Method {
                        name: "OnPredictDNS",
                        params: &[Param { name: "uri", ty: "*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

