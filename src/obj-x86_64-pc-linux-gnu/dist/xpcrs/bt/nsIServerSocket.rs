//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIServerSocket.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIServerSocket",
            base: Some("nsISupports"),
            methods: Err("native type union PRNetAddr unsupported"),
        },

        Interface {
            name: "nsIServerSocketListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onSocketAccepted (in nsIServerSocket aServ, in nsISocketTransport aTransport); */
                    Method {
                        name: "OnSocketAccepted",
                        params: &[Param { name: "aServ", ty: "*const nsIServerSocket" }, Param { name: "aTransport", ty: "*const nsISocketTransport" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onStopListening (in nsIServerSocket aServ, in nsresult aStatus); */
                    Method {
                        name: "OnStopListening",
                        params: &[Param { name: "aServ", ty: "*const nsIServerSocket" }, Param { name: "aStatus", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

