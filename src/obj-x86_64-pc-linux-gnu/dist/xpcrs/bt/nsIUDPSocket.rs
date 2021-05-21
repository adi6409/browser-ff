//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIUDPSocket.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUDPSocket",
            base: Some("nsISupports"),
            methods: Err("optional_argc is unsupported"),
        },

        Interface {
            name: "nsIUDPSocketListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onPacketReceived (in nsIUDPSocket aSocket, in nsIUDPMessage aMessage); */
                    Method {
                        name: "OnPacketReceived",
                        params: &[Param { name: "aSocket", ty: "*const nsIUDPSocket" }, Param { name: "aMessage", ty: "*const nsIUDPMessage" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onStopListening (in nsIUDPSocket aSocket, in nsresult aStatus); */
                    Method {
                        name: "OnStopListening",
                        params: &[Param { name: "aSocket", ty: "*const nsIUDPSocket" }, Param { name: "aStatus", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIUDPMessage",
            base: Some("nsISupports"),
            methods: Err("specialtype jsval unsupported"),
        },

        ]; D}

