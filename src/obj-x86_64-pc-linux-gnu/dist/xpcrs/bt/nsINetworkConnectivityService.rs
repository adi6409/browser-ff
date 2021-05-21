//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsINetworkConnectivityService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsINetworkConnectivityService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [infallible] readonly attribute nsINetworkConnectivityService_ConnectivityState DNSv4; */
                    Method {
                        name: "GetDNSv4",
                        params: &[Param { name: "aDNSv4", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [infallible] readonly attribute nsINetworkConnectivityService_ConnectivityState DNSv6; */
                    Method {
                        name: "GetDNSv6",
                        params: &[Param { name: "aDNSv6", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [infallible] readonly attribute nsINetworkConnectivityService_ConnectivityState IPv4; */
                    Method {
                        name: "GetIPv4",
                        params: &[Param { name: "aIPv4", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [infallible] readonly attribute nsINetworkConnectivityService_ConnectivityState IPv6; */
                    Method {
                        name: "GetIPv6",
                        params: &[Param { name: "aIPv6", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [infallible] readonly attribute nsINetworkConnectivityService_ConnectivityState NAT64; */
                    Method {
                        name: "GetNAT64",
                        params: &[Param { name: "aNAT64", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void recheckDNS (); */
                    Method {
                        name: "RecheckDNS",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void recheckIPConnectivity (); */
                    Method {
                        name: "RecheckIPConnectivity",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

