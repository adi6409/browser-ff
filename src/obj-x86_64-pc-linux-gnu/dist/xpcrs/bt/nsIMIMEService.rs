//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/mime/nsIMIMEService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMIMEService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIMIMEInfo getFromTypeAndExtension (in ACString aMIMEType, in AUTF8String aFileExt); */
                    Method {
                        name: "GetFromTypeAndExtension",
                        params: &[Param { name: "aMIMEType", ty: "*const ::nsstring::nsACString" }, Param { name: "aFileExt", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const nsIMIMEInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString getTypeFromExtension (in AUTF8String aFileExt); */
                    Method {
                        name: "GetTypeFromExtension",
                        params: &[Param { name: "aFileExt", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString getTypeFromURI (in nsIURI aURI); */
                    Method {
                        name: "GetTypeFromURI",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString getTypeFromFile (in nsIFile aFile); */
                    Method {
                        name: "GetTypeFromFile",
                        params: &[Param { name: "aFile", ty: "*const nsIFile" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AUTF8String getPrimaryExtension (in ACString aMIMEType, in AUTF8String aFileExt); */
                    Method {
                        name: "GetPrimaryExtension",
                        params: &[Param { name: "aMIMEType", ty: "*const ::nsstring::nsACString" }, Param { name: "aFileExt", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIMIMEInfo getMIMEInfoFromOS (in ACString aType, in ACString aFileExtension, out boolean aFound); */
                    Method {
                        name: "GetMIMEInfoFromOS",
                        params: &[Param { name: "aType", ty: "*const ::nsstring::nsACString" }, Param { name: "aFileExtension", ty: "*const ::nsstring::nsACString" }, Param { name: "aFound", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut*const nsIMIMEInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

