//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIHttpChannelAuthProvider.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHttpChannelAuthProvider",
            base: Some("nsICancelable"),
            methods: Ok(&[
                    /* [must_use] void init (in nsIHttpAuthenticableChannel channel); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "channel", ty: "*const nsIHttpAuthenticableChannel" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void processAuthentication (in unsigned long httpStatus, in boolean sslConnectFailed); */
                    Method {
                        name: "ProcessAuthentication",
                        params: &[Param { name: "httpStatus", ty: "u32" }, Param { name: "sslConnectFailed", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void addAuthorizationHeaders (in boolean dontUseCachedWWWCreds); */
                    Method {
                        name: "AddAuthorizationHeaders",
                        params: &[Param { name: "dontUseCachedWWWCreds", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void checkForSuperfluousAuth (); */
                    Method {
                        name: "CheckForSuperfluousAuth",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void disconnect (in nsresult status); */
                    Method {
                        name: "Disconnect",
                        params: &[Param { name: "status", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void clearProxyIdent (); */
                    Method {
                        name: "ClearProxyIdent",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

