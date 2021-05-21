//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIRequestObserver.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRequestObserver",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onStartRequest (in nsIRequest aRequest); */
                    Method {
                        name: "OnStartRequest",
                        params: &[Param { name: "aRequest", ty: "*const nsIRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onStopRequest (in nsIRequest aRequest, in nsresult aStatusCode); */
                    Method {
                        name: "OnStopRequest",
                        params: &[Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aStatusCode", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

