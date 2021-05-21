//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/exthandler/nsIExternalHelperAppService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIExternalHelperAppService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIStreamListener doContent (in ACString aMimeContentType, in nsIRequest aRequest, in nsIInterfaceRequestor aContentContext, in boolean aForceSave, [optional] in nsIInterfaceRequestor aWindowContext); */
                    Method {
                        name: "DoContent",
                        params: &[Param { name: "aMimeContentType", ty: "*const ::nsstring::nsACString" }, Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aContentContext", ty: "*const nsIInterfaceRequestor" }, Param { name: "aForceSave", ty: "bool" }, Param { name: "aWindowContext", ty: "*const nsIInterfaceRequestor" }, Param { name: "_retval", ty: "*mut*const nsIStreamListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIStreamListener createListener (in ACString aMimeContentType, in nsIRequest aRequest, in BrowsingContext aContentContext, in boolean aForceSave, [optional] in nsIInterfaceRequestor aWindowContext); */
                    Method {
                        name: "CreateListener",
                        params: &[Param { name: "aMimeContentType", ty: "*const ::nsstring::nsACString" }, Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aContentContext", ty: "*const libc::c_void" }, Param { name: "aForceSave", ty: "bool" }, Param { name: "aWindowContext", ty: "*const nsIInterfaceRequestor" }, Param { name: "_retval", ty: "*mut*const nsIStreamListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean applyDecodingForExtension (in AUTF8String aExtension, in ACString aEncodingType); */
                    Method {
                        name: "ApplyDecodingForExtension",
                        params: &[Param { name: "aExtension", ty: "*const ::nsstring::nsACString" }, Param { name: "aEncodingType", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsPIExternalAppLauncher",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void deleteTemporaryFileOnExit (in nsIFile aTemporaryFile); */
                    Method {
                        name: "DeleteTemporaryFileOnExit",
                        params: &[Param { name: "aTemporaryFile", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void deleteTemporaryPrivateFileWhenPossible (in nsIFile aTemporaryFile); */
                    Method {
                        name: "DeleteTemporaryPrivateFileWhenPossible",
                        params: &[Param { name: "aTemporaryFile", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIHelperAppLauncher",
            base: Some("nsICancelable"),
            methods: Ok(&[
                    /* readonly attribute nsIMIMEInfo MIMEInfo; */
                    Method {
                        name: "GetMIMEInfo",
                        params: &[Param { name: "aMIMEInfo", ty: "*mut*const nsIMIMEInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIURI source; */
                    Method {
                        name: "GetSource",
                        params: &[Param { name: "aSource", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString suggestedFileName; */
                    Method {
                        name: "GetSuggestedFileName",
                        params: &[Param { name: "aSuggestedFileName", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void promptForSaveDestination (); */
                    Method {
                        name: "PromptForSaveDestination",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void launchWithApplication (in boolean aHandleInternally); */
                    Method {
                        name: "LaunchWithApplication",
                        params: &[Param { name: "aHandleInternally", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void saveDestinationAvailable (in nsIFile aFile); */
                    Method {
                        name: "SaveDestinationAvailable",
                        params: &[Param { name: "aFile", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setWebProgressListener (in nsIWebProgressListener2 aWebProgressListener); */
                    Method {
                        name: "SetWebProgressListener",
                        params: &[Param { name: "aWebProgressListener", ty: "*const nsIWebProgressListener2" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIFile targetFile; */
                    Method {
                        name: "GetTargetFile",
                        params: &[Param { name: "aTargetFile", ty: "*mut*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean targetFileIsExecutable; */
                    Method {
                        name: "GetTargetFileIsExecutable",
                        params: &[Param { name: "aTargetFileIsExecutable", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute PRTime timeDownloadStarted; */
                    Method {
                        name: "GetTimeDownloadStarted",
                        params: &[Param { name: "aTimeDownloadStarted", ty: "*mut PRTime" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute int64_t contentLength; */
                    Method {
                        name: "GetContentLength",
                        params: &[Param { name: "aContentLength", ty: "*mut int64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint64_t browsingContextId; */
                    Method {
                        name: "GetBrowsingContextId",
                        params: &[Param { name: "aBrowsingContextId", ty: "*mut uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

