//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsINetworkLinkService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsINetworkLinkService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute boolean isLinkUp; */
                    Method {
                        name: "GetIsLinkUp",
                        params: &[Param { name: "aIsLinkUp", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean linkStatusKnown; */
                    Method {
                        name: "GetLinkStatusKnown",
                        params: &[Param { name: "aLinkStatusKnown", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long linkType; */
                    Method {
                        name: "GetLinkType",
                        params: &[Param { name: "aLinkType", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString networkID; */
                    Method {
                        name: "GetNetworkID",
                        params: &[Param { name: "aNetworkID", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute Array<ACString> dnsSuffixList; */
                    Method {
                        name: "GetDnsSuffixList",
                        params: &[Param { name: "aDnsSuffixList", ty: "*mut thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long platformDNSIndications; */
                    Method {
                        name: "GetPlatformDNSIndications",
                        params: &[Param { name: "aPlatformDNSIndications", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

