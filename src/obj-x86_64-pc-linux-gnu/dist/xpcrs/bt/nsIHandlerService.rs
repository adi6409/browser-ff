//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/exthandler/nsIHandlerService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHandlerService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void asyncInit (); */
                    Method {
                        name: "AsyncInit",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISimpleEnumerator enumerate (); */
                    Method {
                        name: "Enumerate",
                        params: &[Param { name: "_retval", ty: "*mut*const nsISimpleEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void fillHandlerInfo (in nsIHandlerInfo aHandlerInfo, in ACString aOverrideType); */
                    Method {
                        name: "FillHandlerInfo",
                        params: &[Param { name: "aHandlerInfo", ty: "*const nsIHandlerInfo" }, Param { name: "aOverrideType", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void store (in nsIHandlerInfo aHandlerInfo); */
                    Method {
                        name: "Store",
                        params: &[Param { name: "aHandlerInfo", ty: "*const nsIHandlerInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean exists (in nsIHandlerInfo aHandlerInfo); */
                    Method {
                        name: "Exists",
                        params: &[Param { name: "aHandlerInfo", ty: "*const nsIHandlerInfo" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void remove (in nsIHandlerInfo aHandlerInfo); */
                    Method {
                        name: "Remove",
                        params: &[Param { name: "aHandlerInfo", ty: "*const nsIHandlerInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString getTypeFromExtension (in ACString aFileExtension); */
                    Method {
                        name: "GetTypeFromExtension",
                        params: &[Param { name: "aFileExtension", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean existsForProtocolOS (in ACString aProtocolScheme); */
                    Method {
                        name: "ExistsForProtocolOS",
                        params: &[Param { name: "aProtocolScheme", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean existsForProtocol (in ACString aProtocolScheme); */
                    Method {
                        name: "ExistsForProtocol",
                        params: &[Param { name: "aProtocolScheme", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getMIMEInfoFromOS (in nsIHandlerInfo aHandlerInfo, in ACString aMIMEType, in ACString aExtension, out bool aFound); */
                    Method {
                        name: "GetMIMEInfoFromOS",
                        params: &[Param { name: "aHandlerInfo", ty: "*const nsIHandlerInfo" }, Param { name: "aMIMEType", ty: "*const ::nsstring::nsACString" }, Param { name: "aExtension", ty: "*const ::nsstring::nsACString" }, Param { name: "aFound", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getApplicationDescription (in ACString aProtocolScheme); */
                    Method {
                        name: "GetApplicationDescription",
                        params: &[Param { name: "aProtocolScheme", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

