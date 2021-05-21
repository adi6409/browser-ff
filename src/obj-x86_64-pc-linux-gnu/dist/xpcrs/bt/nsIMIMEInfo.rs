//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/mime/nsIMIMEInfo.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHandlerInfo",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute ACString type; */
                    Method {
                        name: "GetType",
                        params: &[Param { name: "aType", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString description; */
                    Method {
                        name: "GetDescription",
                        params: &[Param { name: "aDescription", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetDescription",
                        params: &[Param { name: "aDescription", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIHandlerApp preferredApplicationHandler; */
                    Method {
                        name: "GetPreferredApplicationHandler",
                        params: &[Param { name: "aPreferredApplicationHandler", ty: "*mut*const nsIHandlerApp" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetPreferredApplicationHandler",
                        params: &[Param { name: "aPreferredApplicationHandler", ty: "*const nsIHandlerApp" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIMutableArray possibleApplicationHandlers; */
                    Method {
                        name: "GetPossibleApplicationHandlers",
                        params: &[Param { name: "aPossibleApplicationHandlers", ty: "*mut*const nsIMutableArray" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean hasDefaultHandler; */
                    Method {
                        name: "GetHasDefaultHandler",
                        params: &[Param { name: "aHasDefaultHandler", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString defaultDescription; */
                    Method {
                        name: "GetDefaultDescription",
                        params: &[Param { name: "aDefaultDescription", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void launchWithURI (in nsIURI aURI, [optional] in BrowsingContext aBrowsingContext); */
                    Method {
                        name: "LaunchWithURI",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aBrowsingContext", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsHandlerInfoAction preferredAction; */
                    Method {
                        name: "GetPreferredAction",
                        params: &[Param { name: "aPreferredAction", ty: "*mut nsHandlerInfoAction" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetPreferredAction",
                        params: &[Param { name: "aPreferredAction", ty: "nsHandlerInfoAction" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean alwaysAskBeforeHandling; */
                    Method {
                        name: "GetAlwaysAskBeforeHandling",
                        params: &[Param { name: "aAlwaysAskBeforeHandling", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetAlwaysAskBeforeHandling",
                        params: &[Param { name: "aAlwaysAskBeforeHandling", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIMIMEInfo",
            base: Some("nsIHandlerInfo"),
            methods: Ok(&[
                    /* nsIUTF8StringEnumerator getFileExtensions (); */
                    Method {
                        name: "GetFileExtensions",
                        params: &[Param { name: "_retval", ty: "*mut*const nsIUTF8StringEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setFileExtensions (in AUTF8String aExtensions); */
                    Method {
                        name: "SetFileExtensions",
                        params: &[Param { name: "aExtensions", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean extensionExists (in AUTF8String aExtension); */
                    Method {
                        name: "ExtensionExists",
                        params: &[Param { name: "aExtension", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void appendExtension (in AUTF8String aExtension); */
                    Method {
                        name: "AppendExtension",
                        params: &[Param { name: "aExtension", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AUTF8String primaryExtension; */
                    Method {
                        name: "GetPrimaryExtension",
                        params: &[Param { name: "aPrimaryExtension", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetPrimaryExtension",
                        params: &[Param { name: "aPrimaryExtension", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString MIMEType; */
                    Method {
                        name: "GetMIMEType",
                        params: &[Param { name: "aMIMEType", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean equals (in nsIMIMEInfo aMIMEInfo); */
                    Method {
                        name: "Equals",
                        params: &[Param { name: "aMIMEInfo", ty: "*const nsIMIMEInfo" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIArray possibleLocalHandlers; */
                    Method {
                        name: "GetPossibleLocalHandlers",
                        params: &[Param { name: "aPossibleLocalHandlers", ty: "*mut*const nsIArray" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void launchWithFile (in nsIFile aFile); */
                    Method {
                        name: "LaunchWithFile",
                        params: &[Param { name: "aFile", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isCurrentAppOSDefault (); */
                    Method {
                        name: "IsCurrentAppOSDefault",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIHandlerApp",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute AString name; */
                    Method {
                        name: "GetName",
                        params: &[Param { name: "aName", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetName",
                        params: &[Param { name: "aName", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString detailedDescription; */
                    Method {
                        name: "GetDetailedDescription",
                        params: &[Param { name: "aDetailedDescription", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetDetailedDescription",
                        params: &[Param { name: "aDetailedDescription", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean equals (in nsIHandlerApp aHandlerApp); */
                    Method {
                        name: "Equals",
                        params: &[Param { name: "aHandlerApp", ty: "*const nsIHandlerApp" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void launchWithURI (in nsIURI aURI, [optional] in BrowsingContext aBrowsingContext); */
                    Method {
                        name: "LaunchWithURI",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aBrowsingContext", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsILocalHandlerApp",
            base: Some("nsIHandlerApp"),
            methods: Ok(&[
                    /* attribute nsIFile executable; */
                    Method {
                        name: "GetExecutable",
                        params: &[Param { name: "aExecutable", ty: "*mut*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetExecutable",
                        params: &[Param { name: "aExecutable", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long parameterCount; */
                    Method {
                        name: "GetParameterCount",
                        params: &[Param { name: "aParameterCount", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void clearParameters (); */
                    Method {
                        name: "ClearParameters",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void appendParameter (in AString param); */
                    Method {
                        name: "AppendParameter",
                        params: &[Param { name: "param", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getParameter (in unsigned long parameterIndex); */
                    Method {
                        name: "GetParameter",
                        params: &[Param { name: "parameterIndex", ty: "u32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean parameterExists (in AString param); */
                    Method {
                        name: "ParameterExists",
                        params: &[Param { name: "param", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIWebHandlerApp",
            base: Some("nsIHandlerApp"),
            methods: Ok(&[
                    /* attribute AUTF8String uriTemplate; */
                    Method {
                        name: "GetUriTemplate",
                        params: &[Param { name: "aUriTemplate", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetUriTemplate",
                        params: &[Param { name: "aUriTemplate", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIDBusHandlerApp",
            base: Some("nsIHandlerApp"),
            methods: Ok(&[
                    /* attribute AUTF8String service; */
                    Method {
                        name: "GetService",
                        params: &[Param { name: "aService", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetService",
                        params: &[Param { name: "aService", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AUTF8String objectPath; */
                    Method {
                        name: "GetObjectPath",
                        params: &[Param { name: "aObjectPath", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetObjectPath",
                        params: &[Param { name: "aObjectPath", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AUTF8String dBusInterface; */
                    Method {
                        name: "GetDBusInterface",
                        params: &[Param { name: "aDBusInterface", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetDBusInterface",
                        params: &[Param { name: "aDBusInterface", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AUTF8String method; */
                    Method {
                        name: "GetMethod",
                        params: &[Param { name: "aMethod", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetMethod",
                        params: &[Param { name: "aMethod", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

