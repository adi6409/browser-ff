//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIDashboard.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsINetDashboardCallback",
            base: Some("nsISupports"),
            methods: Err("specialtype jsval unsupported"),
        },

        Interface {
            name: "nsIDashboard",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void requestSockets (in nsINetDashboardCallback cb); */
                    Method {
                        name: "RequestSockets",
                        params: &[Param { name: "cb", ty: "*const nsINetDashboardCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void requestHttpConnections (in nsINetDashboardCallback cb); */
                    Method {
                        name: "RequestHttpConnections",
                        params: &[Param { name: "cb", ty: "*const nsINetDashboardCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void requestWebsocketConnections (in nsINetDashboardCallback cb); */
                    Method {
                        name: "RequestWebsocketConnections",
                        params: &[Param { name: "cb", ty: "*const nsINetDashboardCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void requestDNSInfo (in nsINetDashboardCallback cb); */
                    Method {
                        name: "RequestDNSInfo",
                        params: &[Param { name: "cb", ty: "*const nsINetDashboardCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void requestConnection (in ACString aHost, in unsigned long aPort, in string aProtocol, in unsigned long aTimeout, in nsINetDashboardCallback cb); */
                    Method {
                        name: "RequestConnection",
                        params: &[Param { name: "aHost", ty: "*const ::nsstring::nsACString" }, Param { name: "aPort", ty: "u32" }, Param { name: "aProtocol", ty: "*const libc::c_char" }, Param { name: "aTimeout", ty: "u32" }, Param { name: "cb", ty: "*const nsINetDashboardCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean enableLogging; */
                    Method {
                        name: "GetEnableLogging",
                        params: &[Param { name: "aEnableLogging", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetEnableLogging",
                        params: &[Param { name: "aEnableLogging", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void requestDNSLookup (in ACString aHost, in nsINetDashboardCallback cb); */
                    Method {
                        name: "RequestDNSLookup",
                        params: &[Param { name: "aHost", ty: "*const ::nsstring::nsACString" }, Param { name: "cb", ty: "*const nsINetDashboardCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void requestDNSHTTPSRRLookup (in ACString aHost, in nsINetDashboardCallback cb); */
                    Method {
                        name: "RequestDNSHTTPSRRLookup",
                        params: &[Param { name: "aHost", ty: "*const ::nsstring::nsACString" }, Param { name: "cb", ty: "*const nsINetDashboardCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void requestRcwnStats (in nsINetDashboardCallback cb); */
                    Method {
                        name: "RequestRcwnStats",
                        params: &[Param { name: "cb", ty: "*const nsINetDashboardCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AUTF8String getLogPath (); */
                    Method {
                        name: "GetLogPath",
                        params: &[Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

