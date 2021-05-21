//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/file/nsIFileProtocolHandler.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFileProtocolHandler",
            base: Some("nsIProtocolHandler"),
            methods: Ok(&[
                    /* nsIURI newFileURI (in nsIFile aFile); */
                    Method {
                        name: "NewFileURI",
                        params: &[Param { name: "aFile", ty: "*const nsIFile" }, Param { name: "_retval", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIURIMutator newFileURIMutator (in nsIFile file); */
                    Method {
                        name: "NewFileURIMutator",
                        params: &[Param { name: "file", ty: "*const nsIFile" }, Param { name: "_retval", ty: "*mut*const nsIURIMutator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AUTF8String getURLSpecFromFile (in nsIFile file); */
                    Method {
                        name: "GetURLSpecFromFile",
                        params: &[Param { name: "file", ty: "*const nsIFile" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AUTF8String getURLSpecFromActualFile (in nsIFile file); */
                    Method {
                        name: "GetURLSpecFromActualFile",
                        params: &[Param { name: "file", ty: "*const nsIFile" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AUTF8String getURLSpecFromDir (in nsIFile file); */
                    Method {
                        name: "GetURLSpecFromDir",
                        params: &[Param { name: "file", ty: "*const nsIFile" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIFile getFileFromURLSpec (in AUTF8String url); */
                    Method {
                        name: "GetFileFromURLSpec",
                        params: &[Param { name: "url", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIURI readURLFile (in nsIFile file); */
                    Method {
                        name: "ReadURLFile",
                        params: &[Param { name: "file", ty: "*const nsIFile" }, Param { name: "_retval", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIURI readShellLink (in nsIFile file); */
                    Method {
                        name: "ReadShellLink",
                        params: &[Param { name: "file", ty: "*const nsIFile" }, Param { name: "_retval", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

