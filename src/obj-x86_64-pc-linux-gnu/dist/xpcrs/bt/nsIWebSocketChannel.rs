//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/websocket/nsIWebSocketChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebSocketChannel",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] readonly attribute nsIURI originalURI; */
                    Method {
                        name: "GetOriginalURI",
                        params: &[Param { name: "aOriginalURI", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute nsIURI URI; */
                    Method {
                        name: "GetURI",
                        params: &[Param { name: "aURI", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] attribute nsIInterfaceRequestor notificationCallbacks; */
                    Method {
                        name: "GetNotificationCallbacks",
                        params: &[Param { name: "aNotificationCallbacks", ty: "*mut*const nsIInterfaceRequestor" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetNotificationCallbacks",
                        params: &[Param { name: "aNotificationCallbacks", ty: "*const nsIInterfaceRequestor" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute nsISupports securityInfo; */
                    Method {
                        name: "GetSecurityInfo",
                        params: &[Param { name: "aSecurityInfo", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] attribute nsILoadGroup loadGroup; */
                    Method {
                        name: "GetLoadGroup",
                        params: &[Param { name: "aLoadGroup", ty: "*mut*const nsILoadGroup" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetLoadGroup",
                        params: &[Param { name: "aLoadGroup", ty: "*const nsILoadGroup" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] attribute nsILoadInfo loadInfo; */
                    Method {
                        name: "GetLoadInfo",
                        params: &[Param { name: "aLoadInfo", ty: "*mut*const nsILoadInfo" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetLoadInfo",
                        params: &[Param { name: "aLoadInfo", ty: "*const nsILoadInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] attribute ACString protocol; */
                    Method {
                        name: "GetProtocol",
                        params: &[Param { name: "aProtocol", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetProtocol",
                        params: &[Param { name: "aProtocol", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute ACString extensions; */
                    Method {
                        name: "GetExtensions",
                        params: &[Param { name: "aExtensions", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute uint64_t httpChannelId; */
                    Method {
                        name: "GetHttpChannelId",
                        params: &[Param { name: "aHttpChannelId", ty: "*mut uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [notxpcom] nsresult initLoadInfoNative (in Node aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in nsICookieJarSettings aCookieJarSettings, in unsigned long aSecurityFlags, in nsContentPolicyType aContentPolicyType, in unsigned long aSandboxFlags); */
                    Method {
                        name: "InitLoadInfoNative",
                        params: &[Param { name: "aLoadingNode", ty: "*const libc::c_void" }, Param { name: "aLoadingPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aTriggeringPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aCookieJarSettings", ty: "*const nsICookieJarSettings" }, Param { name: "aSecurityFlags", ty: "u32" }, Param { name: "aContentPolicyType", ty: "nsContentPolicyType" }, Param { name: "aSandboxFlags", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void initLoadInfo (in Node aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in unsigned long aSecurityFlags, in nsContentPolicyType aContentPolicyType); */
                    Method {
                        name: "InitLoadInfo",
                        params: &[Param { name: "aLoadingNode", ty: "*const libc::c_void" }, Param { name: "aLoadingPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aTriggeringPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aSecurityFlags", ty: "u32" }, Param { name: "aContentPolicyType", ty: "nsContentPolicyType" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void asyncOpen (in nsIURI aURI, in ACString aOrigin, in unsigned long long aInnerWindowID, in nsIWebSocketListener aListener, in nsISupports aContext); */
                    Method {
                        name: "AsyncOpen",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aOrigin", ty: "*const ::nsstring::nsACString" }, Param { name: "aInnerWindowID", ty: "u64" }, Param { name: "aListener", ty: "*const nsIWebSocketListener" }, Param { name: "aContext", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void close (in unsigned short aCode, in AUTF8String aReason); */
                    Method {
                        name: "Close",
                        params: &[Param { name: "aCode", ty: "u16" }, Param { name: "aReason", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void sendMsg (in AUTF8String aMsg); */
                    Method {
                        name: "SendMsg",
                        params: &[Param { name: "aMsg", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void sendBinaryMsg (in ACString aMsg); */
                    Method {
                        name: "SendBinaryMsg",
                        params: &[Param { name: "aMsg", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void sendBinaryStream (in nsIInputStream aStream, in unsigned long length); */
                    Method {
                        name: "SendBinaryStream",
                        params: &[Param { name: "aStream", ty: "*const nsIInputStream" }, Param { name: "length", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] attribute unsigned long pingInterval; */
                    Method {
                        name: "GetPingInterval",
                        params: &[Param { name: "aPingInterval", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetPingInterval",
                        params: &[Param { name: "aPingInterval", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] attribute unsigned long pingTimeout; */
                    Method {
                        name: "GetPingTimeout",
                        params: &[Param { name: "aPingTimeout", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetPingTimeout",
                        params: &[Param { name: "aPingTimeout", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] attribute unsigned long serial; */
                    Method {
                        name: "GetSerial",
                        params: &[Param { name: "aSerial", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetSerial",
                        params: &[Param { name: "aSerial", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void setServerParameters (in nsITransportProvider aProvider, in ACString aNegotiatedExtensions); */
                    Method {
                        name: "SetServerParameters",
                        params: &[Param { name: "aProvider", ty: "*const nsITransportProvider" }, Param { name: "aNegotiatedExtensions", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

