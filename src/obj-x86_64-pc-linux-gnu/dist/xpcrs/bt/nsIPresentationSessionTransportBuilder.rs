//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationSessionTransportBuilder.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPresentationSessionTransportBuilderListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onSessionTransport (in nsIPresentationSessionTransport transport); */
                    Method {
                        name: "OnSessionTransport",
                        params: &[Param { name: "transport", ty: "*const nsIPresentationSessionTransport" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onError (in nsresult reason); */
                    Method {
                        name: "OnError",
                        params: &[Param { name: "reason", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void sendOffer (in nsIPresentationChannelDescription offer); */
                    Method {
                        name: "SendOffer",
                        params: &[Param { name: "offer", ty: "*const nsIPresentationChannelDescription" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void sendAnswer (in nsIPresentationChannelDescription answer); */
                    Method {
                        name: "SendAnswer",
                        params: &[Param { name: "answer", ty: "*const nsIPresentationChannelDescription" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void sendIceCandidate (in AString candidate); */
                    Method {
                        name: "SendIceCandidate",
                        params: &[Param { name: "candidate", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void close (in nsresult reason); */
                    Method {
                        name: "Close",
                        params: &[Param { name: "reason", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIPresentationSessionTransportBuilder",
            base: Some("nsISupports"),
            methods: Ok(&[
                    ]),
        },

        Interface {
            name: "nsIPresentationTransportBuilderConstructor",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIPresentationSessionTransportBuilder createTransportBuilder (in uint8_t type); */
                    Method {
                        name: "CreateTransportBuilder",
                        params: &[Param { name: "type_", ty: "uint8_t" }, Param { name: "_retval", ty: "*mut *const nsIPresentationSessionTransportBuilder" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIPresentationTCPSessionTransportBuilder",
            base: Some("nsIPresentationSessionTransportBuilder"),
            methods: Ok(&[
                    /* void buildTCPSenderTransport (in nsISocketTransport aTransport, in nsIPresentationSessionTransportBuilderListener aListener); */
                    Method {
                        name: "BuildTCPSenderTransport",
                        params: &[Param { name: "aTransport", ty: "*const nsISocketTransport" }, Param { name: "aListener", ty: "*const nsIPresentationSessionTransportBuilderListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void buildTCPReceiverTransport (in nsIPresentationChannelDescription aDescription, in nsIPresentationSessionTransportBuilderListener aListener); */
                    Method {
                        name: "BuildTCPReceiverTransport",
                        params: &[Param { name: "aDescription", ty: "*const nsIPresentationChannelDescription" }, Param { name: "aListener", ty: "*const nsIPresentationSessionTransportBuilderListener" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIPresentationDataChannelSessionTransportBuilder",
            base: Some("nsIPresentationSessionTransportBuilder"),
            methods: Ok(&[
                    /* void buildDataChannelTransport (in uint8_t aRole, in mozIDOMWindow aWindow, in nsIPresentationSessionTransportBuilderListener aListener); */
                    Method {
                        name: "BuildDataChannelTransport",
                        params: &[Param { name: "aRole", ty: "uint8_t" }, Param { name: "aWindow", ty: "*const mozIDOMWindow" }, Param { name: "aListener", ty: "*const nsIPresentationSessionTransportBuilderListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onOffer (in nsIPresentationChannelDescription offer); */
                    Method {
                        name: "OnOffer",
                        params: &[Param { name: "offer", ty: "*const nsIPresentationChannelDescription" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onAnswer (in nsIPresentationChannelDescription answer); */
                    Method {
                        name: "OnAnswer",
                        params: &[Param { name: "answer", ty: "*const nsIPresentationChannelDescription" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onIceCandidate (in AString candidate); */
                    Method {
                        name: "OnIceCandidate",
                        params: &[Param { name: "candidate", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void notifyDisconnected (in nsresult reason); */
                    Method {
                        name: "NotifyDisconnected",
                        params: &[Param { name: "reason", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

