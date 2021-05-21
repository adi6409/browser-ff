//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/websocket/nsIWebSocketImpl.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebSocketImpl",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] void sendMessage (in AString aMessage); */
                    Method {
                        name: "SendMessage",
                        params: &[Param { name: "aMessage", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

