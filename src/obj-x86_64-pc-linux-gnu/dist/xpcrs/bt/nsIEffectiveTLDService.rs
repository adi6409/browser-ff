//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/dns/nsIEffectiveTLDService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIEffectiveTLDService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* ACString getPublicSuffix (in nsIURI aURI); */
                    Method {
                        name: "GetPublicSuffix",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString getKnownPublicSuffix (in nsIURI aURI); */
                    Method {
                        name: "GetKnownPublicSuffix",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString getBaseDomain (in nsIURI aURI, [optional] in uint32_t aAdditionalParts); */
                    Method {
                        name: "GetBaseDomain",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aAdditionalParts", ty: "uint32_t" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString getPublicSuffixFromHost (in AUTF8String aHost); */
                    Method {
                        name: "GetPublicSuffixFromHost",
                        params: &[Param { name: "aHost", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString getKnownPublicSuffixFromHost (in AUTF8String aHost); */
                    Method {
                        name: "GetKnownPublicSuffixFromHost",
                        params: &[Param { name: "aHost", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString getBaseDomainFromHost (in AUTF8String aHost, [optional] in uint32_t aAdditionalParts); */
                    Method {
                        name: "GetBaseDomainFromHost",
                        params: &[Param { name: "aHost", ty: "*const ::nsstring::nsACString" }, Param { name: "aAdditionalParts", ty: "uint32_t" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString getNextSubDomain (in AUTF8String aHost); */
                    Method {
                        name: "GetNextSubDomain",
                        params: &[Param { name: "aHost", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* bool hasRootDomain (in AUTF8String aInput, in AUTF8String aHost); */
                    Method {
                        name: "HasRootDomain",
                        params: &[Param { name: "aInput", ty: "*const ::nsstring::nsACString" }, Param { name: "aHost", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

