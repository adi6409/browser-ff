//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIURIFixup.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIURIFixupInfo",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute BrowsingContext consumer; */
                    Method {
                        name: "GetConsumer",
                        params: &[Param { name: "aConsumer", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetConsumer",
                        params: &[Param { name: "aConsumer", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIURI preferredURI; */
                    Method {
                        name: "GetPreferredURI",
                        params: &[Param { name: "aPreferredURI", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetPreferredURI",
                        params: &[Param { name: "aPreferredURI", ty: "*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIURI fixedURI; */
                    Method {
                        name: "GetFixedURI",
                        params: &[Param { name: "aFixedURI", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetFixedURI",
                        params: &[Param { name: "aFixedURI", ty: "*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString keywordProviderName; */
                    Method {
                        name: "GetKeywordProviderName",
                        params: &[Param { name: "aKeywordProviderName", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetKeywordProviderName",
                        params: &[Param { name: "aKeywordProviderName", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString keywordAsSent; */
                    Method {
                        name: "GetKeywordAsSent",
                        params: &[Param { name: "aKeywordAsSent", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetKeywordAsSent",
                        params: &[Param { name: "aKeywordAsSent", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean fixupChangedProtocol; */
                    Method {
                        name: "GetFixupChangedProtocol",
                        params: &[Param { name: "aFixupChangedProtocol", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetFixupChangedProtocol",
                        params: &[Param { name: "aFixupChangedProtocol", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean fixupCreatedAlternateURI; */
                    Method {
                        name: "GetFixupCreatedAlternateURI",
                        params: &[Param { name: "aFixupCreatedAlternateURI", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetFixupCreatedAlternateURI",
                        params: &[Param { name: "aFixupCreatedAlternateURI", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AUTF8String originalInput; */
                    Method {
                        name: "GetOriginalInput",
                        params: &[Param { name: "aOriginalInput", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetOriginalInput",
                        params: &[Param { name: "aOriginalInput", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIInputStream postData; */
                    Method {
                        name: "GetPostData",
                        params: &[Param { name: "aPostData", ty: "*mut*const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetPostData",
                        params: &[Param { name: "aPostData", ty: "*const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIURIFixup",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIURIFixupInfo getFixupURIInfo (in AUTF8String aURIText, [optional] in unsigned long aFixupFlags); */
                    Method {
                        name: "GetFixupURIInfo",
                        params: &[Param { name: "aURIText", ty: "*const ::nsstring::nsACString" }, Param { name: "aFixupFlags", ty: "u32" }, Param { name: "_retval", ty: "*mut *const nsIURIFixupInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    /* unsigned long webNavigationFlagsToFixupFlags (in AUTF8String aURIText, in unsigned long aDocShellFlags); */
                    Method {
                        name: "WebNavigationFlagsToFixupFlags",
                        params: &[Param { name: "aURIText", ty: "*const ::nsstring::nsACString" }, Param { name: "aDocShellFlags", ty: "u32" }, Param { name: "_retval", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIURIFixupInfo keywordToURI (in AUTF8String aKeyword, [optional] in boolean aIsPrivateContext); */
                    Method {
                        name: "KeywordToURI",
                        params: &[Param { name: "aKeyword", ty: "*const ::nsstring::nsACString" }, Param { name: "aIsPrivateContext", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsIURIFixupInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    /* bool isDomainKnown (in AUTF8String aDomain); */
                    Method {
                        name: "IsDomainKnown",
                        params: &[Param { name: "aDomain", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

