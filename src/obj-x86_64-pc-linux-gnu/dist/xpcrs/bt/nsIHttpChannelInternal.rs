//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIHttpChannelInternal.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHttpUpgradeListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] void onTransportAvailable (in nsISocketTransport aTransport, in nsIAsyncInputStream aSocketIn, in nsIAsyncOutputStream aSocketOut); */
                    Method {
                        name: "OnTransportAvailable",
                        params: &[Param { name: "aTransport", ty: "*const nsISocketTransport" }, Param { name: "aSocketIn", ty: "*const nsIAsyncInputStream" }, Param { name: "aSocketOut", ty: "*const nsIAsyncOutputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void onUpgradeFailed (in nsresult aErrorCode); */
                    Method {
                        name: "OnUpgradeFailed",
                        params: &[Param { name: "aErrorCode", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIHttpChannelInternal",
            base: Some("nsISupports"),
            methods: Err("native type nsCOMArray<nsISecurityConsoleMessage> unsupported"),
        },

        ]; D}

