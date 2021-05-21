//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/system/nsIGIOService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIGIOMimeApp",
            base: Some("nsIHandlerApp"),
            methods: Ok(&[
                    /* readonly attribute AUTF8String id; */
                    Method {
                        name: "GetId",
                        params: &[Param { name: "aId", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String command; */
                    Method {
                        name: "GetCommand",
                        params: &[Param { name: "aCommand", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long expectsURIs; */
                    Method {
                        name: "GetExpectsURIs",
                        params: &[Param { name: "aExpectsURIs", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIUTF8StringEnumerator supportedURISchemes; */
                    Method {
                        name: "GetSupportedURISchemes",
                        params: &[Param { name: "aSupportedURISchemes", ty: "*mut*const nsIUTF8StringEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setAsDefaultForMimeType (in AUTF8String mimeType); */
                    Method {
                        name: "SetAsDefaultForMimeType",
                        params: &[Param { name: "mimeType", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setAsDefaultForFileExtensions (in AUTF8String extensions); */
                    Method {
                        name: "SetAsDefaultForFileExtensions",
                        params: &[Param { name: "extensions", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setAsDefaultForURIScheme (in AUTF8String uriScheme); */
                    Method {
                        name: "SetAsDefaultForURIScheme",
                        params: &[Param { name: "uriScheme", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIGIOService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* AUTF8String getMimeTypeFromExtension (in AUTF8String extension); */
                    Method {
                        name: "GetMimeTypeFromExtension",
                        params: &[Param { name: "extension", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIHandlerApp getAppForURIScheme (in AUTF8String aURIScheme); */
                    Method {
                        name: "GetAppForURIScheme",
                        params: &[Param { name: "aURIScheme", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut *const nsIHandlerApp" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIMutableArray getAppsForURIScheme (in AUTF8String aURIScheme); */
                    Method {
                        name: "GetAppsForURIScheme",
                        params: &[Param { name: "aURIScheme", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const nsIMutableArray" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIHandlerApp getAppForMimeType (in AUTF8String mimeType); */
                    Method {
                        name: "GetAppForMimeType",
                        params: &[Param { name: "mimeType", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut *const nsIHandlerApp" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIGIOMimeApp createAppFromCommand (in AUTF8String cmd, in AUTF8String appName); */
                    Method {
                        name: "CreateAppFromCommand",
                        params: &[Param { name: "cmd", ty: "*const ::nsstring::nsACString" }, Param { name: "appName", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut *const nsIGIOMimeApp" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIGIOMimeApp findAppFromCommand (in AUTF8String cmd); */
                    Method {
                        name: "FindAppFromCommand",
                        params: &[Param { name: "cmd", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut *const nsIGIOMimeApp" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AUTF8String getDescriptionForMimeType (in AUTF8String mimeType); */
                    Method {
                        name: "GetDescriptionForMimeType",
                        params: &[Param { name: "mimeType", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void showURI (in nsIURI uri); */
                    Method {
                        name: "ShowURI",
                        params: &[Param { name: "uri", ty: "*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] void showURIForInput (in ACString uri); */
                    Method {
                        name: "ShowURIForInput",
                        params: &[Param { name: "uri", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] void orgFreedesktopFileManager1ShowItems (in ACString path); */
                    Method {
                        name: "OrgFreedesktopFileManager1ShowItems",
                        params: &[Param { name: "path", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] bool shouldUseFlatpakPortal (); */
                    Method {
                        name: "ShouldUseFlatpakPortal",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

