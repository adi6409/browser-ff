//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/websocket/nsIWebSocketEventService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebSocketFrame",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] readonly attribute DOMHighResTimeStamp timeStamp; */
                    Method {
                        name: "GetTimeStamp",
                        params: &[Param { name: "aTimeStamp", ty: "*mut DOMHighResTimeStamp" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute boolean finBit; */
                    Method {
                        name: "GetFinBit",
                        params: &[Param { name: "aFinBit", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute boolean rsvBit1; */
                    Method {
                        name: "GetRsvBit1",
                        params: &[Param { name: "aRsvBit1", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute boolean rsvBit2; */
                    Method {
                        name: "GetRsvBit2",
                        params: &[Param { name: "aRsvBit2", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute boolean rsvBit3; */
                    Method {
                        name: "GetRsvBit3",
                        params: &[Param { name: "aRsvBit3", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute unsigned short opCode; */
                    Method {
                        name: "GetOpCode",
                        params: &[Param { name: "aOpCode", ty: "*mut u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute boolean maskBit; */
                    Method {
                        name: "GetMaskBit",
                        params: &[Param { name: "aMaskBit", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute unsigned long mask; */
                    Method {
                        name: "GetMask",
                        params: &[Param { name: "aMask", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute ACString payload; */
                    Method {
                        name: "GetPayload",
                        params: &[Param { name: "aPayload", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIWebSocketEventListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] void webSocketCreated (in unsigned long aWebSocketSerialID, in AString aURI, in ACString aProtocols); */
                    Method {
                        name: "WebSocketCreated",
                        params: &[Param { name: "aWebSocketSerialID", ty: "u32" }, Param { name: "aURI", ty: "*const ::nsstring::nsAString" }, Param { name: "aProtocols", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void webSocketOpened (in unsigned long aWebSocketSerialID, in AString aEffectiveURI, in ACString aProtocols, in ACString aExtensions, in uint64_t aHttpChannelId); */
                    Method {
                        name: "WebSocketOpened",
                        params: &[Param { name: "aWebSocketSerialID", ty: "u32" }, Param { name: "aEffectiveURI", ty: "*const ::nsstring::nsAString" }, Param { name: "aProtocols", ty: "*const ::nsstring::nsACString" }, Param { name: "aExtensions", ty: "*const ::nsstring::nsACString" }, Param { name: "aHttpChannelId", ty: "uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void webSocketMessageAvailable (in unsigned long aWebSocketSerialID, in ACString aMessage, in unsigned short aType); */
                    Method {
                        name: "WebSocketMessageAvailable",
                        params: &[Param { name: "aWebSocketSerialID", ty: "u32" }, Param { name: "aMessage", ty: "*const ::nsstring::nsACString" }, Param { name: "aType", ty: "u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void webSocketClosed (in unsigned long aWebSocketSerialID, in boolean aWasClean, in unsigned short aCode, in AString aReason); */
                    Method {
                        name: "WebSocketClosed",
                        params: &[Param { name: "aWebSocketSerialID", ty: "u32" }, Param { name: "aWasClean", ty: "bool" }, Param { name: "aCode", ty: "u16" }, Param { name: "aReason", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void frameReceived (in unsigned long aWebSocketSerialID, in nsIWebSocketFrame aFrame); */
                    Method {
                        name: "FrameReceived",
                        params: &[Param { name: "aWebSocketSerialID", ty: "u32" }, Param { name: "aFrame", ty: "*const nsIWebSocketFrame" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void frameSent (in unsigned long aWebSocketSerialID, in nsIWebSocketFrame aFrame); */
                    Method {
                        name: "FrameSent",
                        params: &[Param { name: "aWebSocketSerialID", ty: "u32" }, Param { name: "aFrame", ty: "*const nsIWebSocketFrame" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIWebSocketEventService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] void sendMessage (in unsigned long aWebSocketSerialID, in AString aMessage); */
                    Method {
                        name: "SendMessage",
                        params: &[Param { name: "aWebSocketSerialID", ty: "u32" }, Param { name: "aMessage", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void addListener (in unsigned long long aInnerWindowID, in nsIWebSocketEventListener aListener); */
                    Method {
                        name: "AddListener",
                        params: &[Param { name: "aInnerWindowID", ty: "u64" }, Param { name: "aListener", ty: "*const nsIWebSocketEventListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void removeListener (in unsigned long long aInnerWindowID, in nsIWebSocketEventListener aListener); */
                    Method {
                        name: "RemoveListener",
                        params: &[Param { name: "aInnerWindowID", ty: "u64" }, Param { name: "aListener", ty: "*const nsIWebSocketEventListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] bool hasListenerFor (in unsigned long long aInnerWindowID); */
                    Method {
                        name: "HasListenerFor",
                        params: &[Param { name: "aInnerWindowID", ty: "u64" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

