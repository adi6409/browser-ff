//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/websocket/nsIWebSocketListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebSocketListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] void onStart (in nsISupports aContext); */
                    Method {
                        name: "OnStart",
                        params: &[Param { name: "aContext", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void onStop (in nsISupports aContext, in nsresult aStatusCode); */
                    Method {
                        name: "OnStop",
                        params: &[Param { name: "aContext", ty: "*const nsISupports" }, Param { name: "aStatusCode", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void onMessageAvailable (in nsISupports aContext, in AUTF8String aMsg); */
                    Method {
                        name: "OnMessageAvailable",
                        params: &[Param { name: "aContext", ty: "*const nsISupports" }, Param { name: "aMsg", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void onBinaryMessageAvailable (in nsISupports aContext, in ACString aMsg); */
                    Method {
                        name: "OnBinaryMessageAvailable",
                        params: &[Param { name: "aContext", ty: "*const nsISupports" }, Param { name: "aMsg", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void onAcknowledge (in nsISupports aContext, in uint32_t aSize); */
                    Method {
                        name: "OnAcknowledge",
                        params: &[Param { name: "aContext", ty: "*const nsISupports" }, Param { name: "aSize", ty: "uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void onServerClose (in nsISupports aContext, in unsigned short aCode, in AUTF8String aReason); */
                    Method {
                        name: "OnServerClose",
                        params: &[Param { name: "aContext", ty: "*const nsISupports" }, Param { name: "aCode", ty: "u16" }, Param { name: "aReason", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void OnError (); */
                    Method {
                        name: "OnError",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

