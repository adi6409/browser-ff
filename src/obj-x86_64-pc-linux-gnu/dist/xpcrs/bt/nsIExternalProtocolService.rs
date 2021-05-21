//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/exthandler/nsIExternalProtocolService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIExternalProtocolService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* boolean externalProtocolHandlerExists (in string aProtocolScheme); */
                    Method {
                        name: "ExternalProtocolHandlerExists",
                        params: &[Param { name: "aProtocolScheme", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isExposedProtocol (in string aProtocolScheme); */
                    Method {
                        name: "IsExposedProtocol",
                        params: &[Param { name: "aProtocolScheme", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIHandlerInfo getProtocolHandlerInfo (in ACString aProtocolScheme); */
                    Method {
                        name: "GetProtocolHandlerInfo",
                        params: &[Param { name: "aProtocolScheme", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const nsIHandlerInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIHandlerInfo getProtocolHandlerInfoFromOS (in ACString aProtocolScheme, out boolean aFound); */
                    Method {
                        name: "GetProtocolHandlerInfoFromOS",
                        params: &[Param { name: "aProtocolScheme", ty: "*const ::nsstring::nsACString" }, Param { name: "aFound", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut*const nsIHandlerInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setProtocolHandlerDefaults (in nsIHandlerInfo aHandlerInfo, in boolean aOSHandlerExists); */
                    Method {
                        name: "SetProtocolHandlerDefaults",
                        params: &[Param { name: "aHandlerInfo", ty: "*const nsIHandlerInfo" }, Param { name: "aOSHandlerExists", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void loadURI (in nsIURI aURI, [optional] in nsIPrincipal aTriggeringPrincipal, [optional] in BrowsingContext aBrowsingContext); */
                    Method {
                        name: "LoadURI",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aTriggeringPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aBrowsingContext", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getApplicationDescription (in AUTF8String aScheme); */
                    Method {
                        name: "GetApplicationDescription",
                        params: &[Param { name: "aScheme", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* bool isCurrentAppOSDefaultForProtocol (in AUTF8String aScheme); */
                    Method {
                        name: "IsCurrentAppOSDefaultForProtocol",
                        params: &[Param { name: "aScheme", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

