//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationSessionTransport.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPresentationSessionTransportCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void notifyTransportReady (); */
                    Method {
                        name: "NotifyTransportReady",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void notifyTransportClosed (in nsresult reason); */
                    Method {
                        name: "NotifyTransportClosed",
                        params: &[Param { name: "reason", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void notifyData (in ACString data, in boolean isBinary); */
                    Method {
                        name: "NotifyData",
                        params: &[Param { name: "data", ty: "*const ::nsstring::nsACString" }, Param { name: "isBinary", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIPresentationSessionTransport",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute nsIPresentationSessionTransportCallback callback; */
                    Method {
                        name: "GetCallback",
                        params: &[Param { name: "aCallback", ty: "*mut *const nsIPresentationSessionTransportCallback" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetCallback",
                        params: &[Param { name: "aCallback", ty: "*const nsIPresentationSessionTransportCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsINetAddr selfAddress; */
                    Method {
                        name: "GetSelfAddress",
                        params: &[Param { name: "aSelfAddress", ty: "*mut*const nsINetAddr" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void enableDataNotification (); */
                    Method {
                        name: "EnableDataNotification",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void send (in AString data); */
                    Method {
                        name: "Send",
                        params: &[Param { name: "data", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void sendBinaryMsg (in ACString data); */
                    Method {
                        name: "SendBinaryMsg",
                        params: &[Param { name: "data", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void sendBlob (in Blob blob); */
                    Method {
                        name: "SendBlob",
                        params: &[Param { name: "blob", ty: "*const libc::c_void" }],
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

        ]; D}

