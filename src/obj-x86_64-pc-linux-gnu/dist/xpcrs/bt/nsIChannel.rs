//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIChannel",
            base: Some("nsIRequest"),
            methods: Ok(&[
                    /* attribute nsIURI originalURI; */
                    Method {
                        name: "GetOriginalURI",
                        params: &[Param { name: "aOriginalURI", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetOriginalURI",
                        params: &[Param { name: "aOriginalURI", ty: "*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIURI URI; */
                    Method {
                        name: "GetURI",
                        params: &[Param { name: "aURI", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsISupports owner; */
                    Method {
                        name: "GetOwner",
                        params: &[Param { name: "aOwner", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetOwner",
                        params: &[Param { name: "aOwner", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIInterfaceRequestor notificationCallbacks; */
                    Method {
                        name: "GetNotificationCallbacks",
                        params: &[Param { name: "aNotificationCallbacks", ty: "*mut*const nsIInterfaceRequestor" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetNotificationCallbacks",
                        params: &[Param { name: "aNotificationCallbacks", ty: "*const nsIInterfaceRequestor" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsISupports securityInfo; */
                    Method {
                        name: "GetSecurityInfo",
                        params: &[Param { name: "aSecurityInfo", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute ACString contentType; */
                    Method {
                        name: "GetContentType",
                        params: &[Param { name: "aContentType", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetContentType",
                        params: &[Param { name: "aContentType", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute ACString contentCharset; */
                    Method {
                        name: "GetContentCharset",
                        params: &[Param { name: "aContentCharset", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetContentCharset",
                        params: &[Param { name: "aContentCharset", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute int64_t contentLength; */
                    Method {
                        name: "GetContentLength",
                        params: &[Param { name: "aContentLength", ty: "*mut int64_t" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetContentLength",
                        params: &[Param { name: "aContentLength", ty: "int64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIInputStream open (); */
                    Method {
                        name: "Open",
                        params: &[Param { name: "_retval", ty: "*mut*const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void asyncOpen (in nsIStreamListener aListener); */
                    Method {
                        name: "AsyncOpen",
                        params: &[Param { name: "aListener", ty: "*const nsIStreamListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute boolean canceled; */
                    Method {
                        name: "GetCanceled",
                        params: &[Param { name: "aCanceled", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute unsigned long contentDisposition; */
                    Method {
                        name: "GetContentDisposition",
                        params: &[Param { name: "aContentDisposition", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetContentDisposition",
                        params: &[Param { name: "aContentDisposition", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString contentDispositionFilename; */
                    Method {
                        name: "GetContentDispositionFilename",
                        params: &[Param { name: "aContentDispositionFilename", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetContentDispositionFilename",
                        params: &[Param { name: "aContentDispositionFilename", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString contentDispositionHeader; */
                    Method {
                        name: "GetContentDispositionHeader",
                        params: &[Param { name: "aContentDispositionHeader", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsILoadInfo loadInfo; */
                    Method {
                        name: "GetLoadInfo",
                        params: &[Param { name: "aLoadInfo", ty: "*mut *const nsILoadInfo" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetLoadInfo",
                        params: &[Param { name: "aLoadInfo", ty: "*const nsILoadInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute bool isDocument; */
                    Method {
                        name: "GetIsDocument",
                        params: &[Param { name: "aIsDocument", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIIdentChannel",
            base: Some("nsIChannel"),
            methods: Ok(&[
                    /* [must_use] attribute uint64_t channelId; */
                    Method {
                        name: "GetChannelId",
                        params: &[Param { name: "aChannelId", ty: "*mut uint64_t" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetChannelId",
                        params: &[Param { name: "aChannelId", ty: "uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

