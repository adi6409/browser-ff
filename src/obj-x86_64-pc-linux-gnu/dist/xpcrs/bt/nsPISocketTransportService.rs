//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsPISocketTransportService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsPISocketTransportService",
            base: Some("nsIRoutedSocketTransportService"),
            methods: Ok(&[
                    /* void init (); */
                    Method {
                        name: "Init",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void shutdown (in bool aXpcomShutdown); */
                    Method {
                        name: "Shutdown",
                        params: &[Param { name: "aXpcomShutdown", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long sendBufferSize; */
                    Method {
                        name: "GetSendBufferSize",
                        params: &[Param { name: "aSendBufferSize", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean offline; */
                    Method {
                        name: "GetOffline",
                        params: &[Param { name: "aOffline", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetOffline",
                        params: &[Param { name: "aOffline", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long keepaliveIdleTime; */
                    Method {
                        name: "GetKeepaliveIdleTime",
                        params: &[Param { name: "aKeepaliveIdleTime", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long keepaliveRetryInterval; */
                    Method {
                        name: "GetKeepaliveRetryInterval",
                        params: &[Param { name: "aKeepaliveRetryInterval", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long keepaliveProbeCount; */
                    Method {
                        name: "GetKeepaliveProbeCount",
                        params: &[Param { name: "aKeepaliveProbeCount", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

