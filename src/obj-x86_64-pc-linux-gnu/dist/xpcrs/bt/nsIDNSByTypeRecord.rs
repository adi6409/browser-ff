//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/dns/nsIDNSByTypeRecord.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDNSByTypeRecord",
            base: Some("nsIDNSRecord"),
            methods: Err("Rust only supports [ref] / [ptr] native types"),
        },

        Interface {
            name: "nsIDNSTXTRecord",
            base: Some("nsISupports"),
            methods: Err("native type CopyableTArray<nsCString> unsupported"),
        },

        Interface {
            name: "nsISVCParam",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute uint16_t type; */
                    Method {
                        name: "GetType",
                        params: &[Param { name: "aType", ty: "*mut uint16_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISVCParamAlpn",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute Array<ACString> alpn; */
                    Method {
                        name: "GetAlpn",
                        params: &[Param { name: "aAlpn", ty: "*mut thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISVCParamNoDefaultAlpn",
            base: Some("nsISupports"),
            methods: Ok(&[
                    ]),
        },

        Interface {
            name: "nsISVCParamPort",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute uint16_t port; */
                    Method {
                        name: "GetPort",
                        params: &[Param { name: "aPort", ty: "*mut uint16_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISVCParamIPv4Hint",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute Array<nsINetAddr> ipv4Hint; */
                    Method {
                        name: "GetIpv4Hint",
                        params: &[Param { name: "aIpv4Hint", ty: "*mut thin_vec::ThinVec<RefPtr<nsINetAddr>>" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISVCParamEchConfig",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute ACString echconfig; */
                    Method {
                        name: "GetEchconfig",
                        params: &[Param { name: "aEchconfig", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISVCParamIPv6Hint",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute Array<nsINetAddr> ipv6Hint; */
                    Method {
                        name: "GetIpv6Hint",
                        params: &[Param { name: "aIpv6Hint", ty: "*mut thin_vec::ThinVec<RefPtr<nsINetAddr>>" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISVCParamODoHConfig",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute ACString ODoHConfig; */
                    Method {
                        name: "GetODoHConfig",
                        params: &[Param { name: "aODoHConfig", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISVCBRecord",
            base: Some("nsISupports"),
            methods: Err("nostdcall is unsupported"),
        },

        Interface {
            name: "nsIDNSHTTPSSVCRecord",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute Array<nsISVCBRecord> records; */
                    Method {
                        name: "GetRecords",
                        params: &[Param { name: "aRecords", ty: "*mut thin_vec::ThinVec<RefPtr<nsISVCBRecord>>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISVCBRecord GetServiceModeRecord (in boolean aNoHttp2, in boolean aNoHttp3); */
                    Method {
                        name: "GetServiceModeRecord",
                        params: &[Param { name: "aNoHttp2", ty: "bool" }, Param { name: "aNoHttp3", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsISVCBRecord" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean hasIPAddresses; */
                    Method {
                        name: "GetHasIPAddresses",
                        params: &[Param { name: "aHasIPAddresses", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean allRecordsExcluded; */
                    Method {
                        name: "GetAllRecordsExcluded",
                        params: &[Param { name: "aAllRecordsExcluded", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<nsISVCBRecord> GetAllRecordsWithEchConfig (in boolean aNoHttp2, in boolean aNoHttp3, out boolean aAllRecordsHaveEchConfig, out boolean aAllRecordsInH3ExcludedList); */
                    Method {
                        name: "GetAllRecordsWithEchConfig",
                        params: &[Param { name: "aNoHttp2", ty: "bool" }, Param { name: "aNoHttp3", ty: "bool" }, Param { name: "aAllRecordsHaveEchConfig", ty: "*mut bool" }, Param { name: "aAllRecordsInH3ExcludedList", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut thin_vec::ThinVec<RefPtr<nsISVCBRecord>>" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

