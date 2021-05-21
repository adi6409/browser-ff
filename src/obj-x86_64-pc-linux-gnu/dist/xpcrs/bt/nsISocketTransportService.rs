//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsISocketTransportService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISTSShutdownObserver",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void observe (); */
                    Method {
                        name: "Observe",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISocketTransportService",
            base: Some("nsISupports"),
            methods: Err("native type PRFileDesc unsupported"),
        },

        Interface {
            name: "nsIRoutedSocketTransportService",
            base: Some("nsISocketTransportService"),
            methods: Ok(&[
                    /* nsISocketTransport createRoutedTransport (in Array<ACString> aSocketTypes, in AUTF8String aHost, in long aPort, in AUTF8String aHostRoute, in long aPortRoute, in nsIProxyInfo aProxyInfo, in nsIDNSRecord aDnsRecord); */
                    Method {
                        name: "CreateRoutedTransport",
                        params: &[Param { name: "aSocketTypes", ty: "*const thin_vec::ThinVec<::nsstring::nsCString>" }, Param { name: "aHost", ty: "*const ::nsstring::nsACString" }, Param { name: "aPort", ty: "i32" }, Param { name: "aHostRoute", ty: "*const ::nsstring::nsACString" }, Param { name: "aPortRoute", ty: "i32" }, Param { name: "aProxyInfo", ty: "*const nsIProxyInfo" }, Param { name: "aDnsRecord", ty: "*const nsIDNSRecord" }, Param { name: "_retval", ty: "*mut*const nsISocketTransport" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

