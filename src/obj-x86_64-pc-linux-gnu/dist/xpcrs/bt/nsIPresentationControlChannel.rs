//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationControlChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPresentationChannelDescription",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute uint8_t type; */
                    Method {
                        name: "GetType",
                        params: &[Param { name: "aType", ty: "*mut uint8_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIArray tcpAddress; */
                    Method {
                        name: "GetTcpAddress",
                        params: &[Param { name: "aTcpAddress", ty: "*mut*const nsIArray" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint16_t tcpPort; */
                    Method {
                        name: "GetTcpPort",
                        params: &[Param { name: "aTcpPort", ty: "*mut uint16_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString dataChannelSDP; */
                    Method {
                        name: "GetDataChannelSDP",
                        params: &[Param { name: "aDataChannelSDP", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIPresentationControlChannelListener",
            base: Some("nsISupports"),
            methods: Ok(&[
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

                    /* void notifyConnected (); */
                    Method {
                        name: "NotifyConnected",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void notifyDisconnected (in nsresult reason); */
                    Method {
                        name: "NotifyDisconnected",
                        params: &[Param { name: "reason", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void notifyReconnected (); */
                    Method {
                        name: "NotifyReconnected",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIPresentationControlChannel",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute nsIPresentationControlChannelListener listener; */
                    Method {
                        name: "GetListener",
                        params: &[Param { name: "aListener", ty: "*mut *const nsIPresentationControlChannelListener" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetListener",
                        params: &[Param { name: "aListener", ty: "*const nsIPresentationControlChannelListener" }],
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

                    /* void launch (in AString presentationId, in AString url); */
                    Method {
                        name: "Launch",
                        params: &[Param { name: "presentationId", ty: "*const ::nsstring::nsAString" }, Param { name: "url", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void terminate (in AString presentationId); */
                    Method {
                        name: "Terminate",
                        params: &[Param { name: "presentationId", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void disconnect (in nsresult reason); */
                    Method {
                        name: "Disconnect",
                        params: &[Param { name: "reason", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void reconnect (in AString presentationId, in AString url); */
                    Method {
                        name: "Reconnect",
                        params: &[Param { name: "presentationId", ty: "*const ::nsstring::nsAString" }, Param { name: "url", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

