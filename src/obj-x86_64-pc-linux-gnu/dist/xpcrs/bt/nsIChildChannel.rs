//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIChildChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIChildChannel",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void connectParent (in uint32_t registrarId); */
                    Method {
                        name: "ConnectParent",
                        params: &[Param { name: "registrarId", ty: "uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void completeRedirectSetup (in nsIStreamListener aListener); */
                    Method {
                        name: "CompleteRedirectSetup",
                        params: &[Param { name: "aListener", ty: "*const nsIStreamListener" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

